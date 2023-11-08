//! A platform agnostic rust driver for the ADS122U04 (UART) and ADS122C04 (I2C) ADC from Texas Instruments.
#![no_std]
#![allow(dead_code)]
#![deny(missing_docs)]
#![deny(warnings)]

use core::fmt::Debug;
use core::option::Option;
use core::result::Result;
use core::result::Result::Err;

use embedded_hal::{
    blocking::i2c,
    blocking::serial,
    serial as serial_nb,
};

use crate::{interface::{I2cInterface, ReadData, SerialInterface, WriteData}};
use crate::registers::*;

pub mod registers;
pub mod interface;


mod private {
    use super::interface;

    pub trait Sealed {}

    impl<UART> Sealed for interface::SerialInterface<UART> {}

    impl<I2C, > Sealed for interface::I2cInterface<I2C> {}
}

#[derive(Debug)]
/// Error enum for ADS122x04
pub enum Error<E>
{
    /// An invalid value has been entered
    InvalidValue,
    /// A communcation error has occured
    CommError(E),
}

/// Device handler for ADS122x04
pub struct ADS122x04<BUS>
{
    bus: BUS,
    v_ref: VRef,
    gain: Gain,
    mux: Mux,
    current_source: CurrentSource,
    current_route_1: CurrentRoute,
    current_route_2: CurrentRoute,
    data_rate: DataRate,
    pga_bypass: bool,
    turbo_mode: bool,
    conversion_mode: ConversionMode,
    temperature_sensor_mode: bool,
    data_counter_enable: bool,
    crc: Crc,
    burn_out_current_sources: bool,
}

impl<I2C, E> ADS122x04<I2cInterface<I2C>>
    where
        I2C: i2c::Write<Error=E> + i2c::WriteRead<Error=E>,
{
    /// Create a new ADS122C04 device by supplying an I2C address and I2C handler
    pub fn new_i2c(address: u8, i2c: I2C) -> Self
    {
        ADS122x04 {
            bus: I2cInterface { i2c, address },
            v_ref: VRef::Internal,
            gain: Gain::Gain1,
            mux: Mux::Ain0Ain1,
            current_source: CurrentSource::Off,
            current_route_1: CurrentRoute::Off,
            current_route_2: CurrentRoute::Off,
            data_rate: DataRate::Sps20Normal,
            pga_bypass: false,
            turbo_mode: false,
            conversion_mode: ConversionMode::SingleShot,
            temperature_sensor_mode: false,
            data_counter_enable: false,
            crc: Crc::Disabled,
            burn_out_current_sources: false,
        }
    }
}

impl<UART, E> ADS122x04<SerialInterface<UART>>
    where
        UART: serial::Write<u8, Error=E> + serial_nb::Read<u8, Error=E>,
{
    /// Create a new ADS122C04 device by supplying a serial handler (UART)
    pub fn new_serial(serial: UART) -> Self {
        ADS122x04 {
            bus: SerialInterface { serial },
            v_ref: VRef::Internal,
            gain: Gain::Gain1,
            mux: Mux::Ain0Ain1,
            current_source: CurrentSource::Off,
            current_route_1: CurrentRoute::Off,
            current_route_2: CurrentRoute::Off,
            data_rate: DataRate::Sps20Normal,
            pga_bypass: false,
            turbo_mode: false,
            conversion_mode: ConversionMode::SingleShot,
            temperature_sensor_mode: false,
            data_counter_enable: false,
            crc: Crc::Disabled,
            burn_out_current_sources: false,
        }
    }
}

impl<BUS, E> ADS122x04<BUS>
    where
        BUS: ReadData<Error=Error<E>> + WriteData<Error=Error<E>>,
{
    /// updates a specified config register
    fn update_reg(&mut self, reg: u8) -> Result<(), Error<E>> {
        match reg {
            0x00 => {
                let val = (self.pga_bypass as u8) | ((self.gain as u8) << 1) | ((self.mux as u8) << 4);
                self.bus.write_register(0x00, val)
            }
            0x01 => {
                let val = (self.temperature_sensor_mode as u8)
                    | (self.v_ref.to_val() << 1)
                    | ((self.conversion_mode as u8) << 3)
                    | ((self.turbo_mode as u8) << 4)
                    | ((self.data_rate as u8) << 5);
                self.bus.write_register(0x01, val)
            }
            0x02 => {
                let val = (self.current_source as u8)
                    | ((self.burn_out_current_sources as u8) << 3)
                    | ((self.crc as u8) << 4)
                    | ((self.data_counter_enable as u8) << 6);
                self.bus.write_register(0x02, val)
            }
            0x03 => {
                let val = ((self.current_route_2 as u8) << 2) | ((self.current_route_1 as u8) << 5);
                self.bus.write_register(0x03, val)
            }
            _ => Err(Error::InvalidValue),
        }
    }

    /// reads a specified config register
    fn read_reg(&mut self, reg: u8) -> Result<u8, Error<E>> {
        match reg {
            0x00 => self.bus.read_register(0x00),
            0x01 => self.bus.read_register(0x01),
            0x02 => self.bus.read_register(0x02),
            0x03 => self.bus.read_register(0x03),
            _ => Err(Error::InvalidValue),
        }
    }

    /// Enable or disable the programmable gain amplifier (PGA)
    pub fn set_pga_bypass(&mut self, state: bool) -> Result<(), Error<E>> {
        self.pga_bypass = state;
        self.update_reg(0x00)
    }

    /// Read the status of the programmable gain amplifier (PGA)
    pub fn get_pga_bypass(&mut self) -> Result<bool, Error<E>> {
        self.read_reg(0x00).map(|val| (val & 0b1) == 1)
    }

    /// Set the gain as either 0, 1, 2, 4, 8, 16, 32, 64 or 128
    pub fn set_gain(&mut self, gain: Gain) -> Result<(), Error<E>> {
        self.gain = gain;
        self.update_reg(0x00)
    }

    /// Read the gain value
    pub fn get_gain(&mut self) -> Result<Gain, Error<E>> {
        self.read_reg(0x00).map(|val| Gain::from((val >> 1) & 0b111))
    }

    /// Set the input multiplexer (MUX)
    pub fn set_input_mux(&mut self, mux: Mux) -> Result<(), Error<E>> {
        self.mux = mux;
        self.update_reg(0x00)
    }

    /// Read the input multiplexer (MUX) setting
    pub fn get_input_mux(&mut self) -> Result<u8, Error<E>> {
        self.read_reg(0x00).map(|val| val >> 4)
    }

    /// Enable or disable temperature sensor mode (TS)
    pub fn set_temperature_sensor_mode(&mut self, state: bool) -> Result<(), Error<E>> {
        self.temperature_sensor_mode = state;
        self.update_reg(0x01)
    }

    /// Read the temperature sensor mode (TS)
    pub fn get_temperature_sensor_mode(&mut self) -> Result<bool, Error<E>> {
        self.read_reg(0x01).map(|val| (val & 0b1) == 1)
    }

    /// Set the voltage reference (VREF)
    pub fn set_vref(&mut self, v_ref: VRef) -> Result<(), Error<E>> {
        self.v_ref = v_ref;
        self.update_reg(0x01)
    }

    /// Read the voltage reference (VREF)
    pub fn get_vref(&mut self) -> Result<VRef, Error<E>> {
        self.read_reg(0x01)
            .map(|val| VRef::from((val >> 1) & 0b11, self.v_ref.to_voltage()))
    }

    /// Set the conversion mode (CM)
    pub fn set_conversion_mode(&mut self, mode: ConversionMode) -> Result<(), Error<E>> {
        self.conversion_mode = mode;
        self.update_reg(0x01)
    }

    /// Read the conversion mode (CM)
    pub fn get_conversion_mode(&mut self) -> Result<ConversionMode, Error<E>> {
        self.read_reg(0x01)
            .map(|val| ConversionMode::from((val >> 3) & 0b1))
    }

    /// Read the operating mode
    pub fn get_operating_mode(&mut self) -> Result<bool, Error<E>> {
        self.read_reg(0x01).map(|val| ((val >> 4) & 0b1) == 1)
    }

    /// Set the data rate
    pub fn set_data_rate(&mut self, rate: DataRate) -> Result<(), Error<E>> {
        self.data_rate = rate;
        self.turbo_mode = (self.data_rate as u8 & 0b1) == 1;
        self.update_reg(0x01)
    }

    /// Read the data rate
    pub fn get_data_rate(&mut self) -> Result<DataRate, Error<E>> {
        self.read_reg(0x01)
            .map(|val| DataRate::from((val >> 4) & 0b1111))
    }

    /// Set the current level of the internal excitation current sources
    pub fn set_current_level(&mut self, current: CurrentSource) -> Result<(), Error<E>> {
        self.current_source = current;
        self.update_reg(0x02)
    }

    /// Read the current level of the internal excitation current sources
    pub fn get_current_level(&mut self) -> Result<CurrentSource, Error<E>> {
        self.read_reg(0x02)
            .map(|val| CurrentSource::from(val & 0b111))
    }

    /// Enable or disable the 10 uA burnout current sources
    pub fn set_burnout_current_source(&mut self, state: bool) -> Result<(), Error<E>> {
        self.burn_out_current_sources = state;
        self.update_reg(0x02)
    }

    /// Read the state of the 10 uA burnout current sources
    pub fn get_burnout_current_source(&mut self) -> Result<bool, Error<E>> {
        self.read_reg(0x02).map(|val| ((val >> 3) & 0b1) == 1)
    }

    /// Set the CRC mode
    pub fn set_crc(&mut self, crc: Crc) -> Result<(), Error<E>> {
        self.crc = crc;
        self.update_reg(0x02)
    }

    /// Read the CRC mode
    pub fn get_crc(&mut self) -> Result<Crc, Error<E>> {
        self.read_reg(0x02).map(|val| Crc::from((val >> 4) & 0b11))
    }

    /// Enable or disable data counter
    pub fn set_data_counter(&mut self, state: bool) -> Result<(), Error<E>> {
        self.data_counter_enable = state;
        self.update_reg(0x02)
    }

    /// Read the state of the data counter
    pub fn get_data_counter(&mut self) -> Result<bool, Error<E>> {
        self.read_reg(0x02).map(|val| ((val >> 6) & 0b1) == 1)
    }

    /// Read the data ready (DRDY) register
    pub fn get_data_ready(&mut self) -> Result<bool, Error<E>> {
        self.read_reg(0x02).map(|val| ((val >> 7) & 0b1) == 1)
    }

    /// Set the current routing of the excitation current source 1
    pub fn set_current_route_1(&mut self, route: CurrentRoute) -> Result<(), Error<E>> {
        self.current_route_1 = route;
        self.update_reg(0x03)
    }

    /// Read the current routing of the excitation current source 1
    pub fn get_current_route_1(&mut self) -> Result<CurrentRoute, Error<E>> {
        self.read_reg(0x03)
            .map(|val| CurrentRoute::from((val >> 5) & 0b111))
    }

    /// Set the current routing of the excitation current source 2
    pub fn set_current_route_2(&mut self, route: CurrentRoute) -> Result<(), Error<E>> {
        self.current_route_2 = route;
        self.update_reg(0x03)
    }

    /// Read the current routing of the excitation current source 2
    pub fn get_current_route_2(&mut self) -> Result<CurrentRoute, Error<E>> {
        self.read_reg(0x03)
            .map(|val| CurrentRoute::from((val >> 3) & 0b111))
    }

    /// transform the raw u32 value to signed i32 value according to datasheet
    fn raw_to_signed(&self, x: u32) -> i32 {
        if x >> 23 == 1 {
            -((x & 0b11111111111111111111111) as i32)
        } else {
            x as i32
        }
    }

    /// Read the raw ADC value
    pub fn get_raw_adc(&mut self) -> Result<i32, Error<E>> {
        self.bus.read_data().map(|val| self.raw_to_signed(val))
    }

    /// Read the voltage of the ADC
    pub fn get_voltage(&mut self) -> Option<f32> {
        // returns voltage in V
        let raw = self.get_raw_adc().ok();
        let v_ref = self.v_ref.to_voltage();
        raw.map(|raw| (v_ref as f64 / ((1 << 23) as f64) * (raw as f64)) as f32)
    }

    /// Convert the raw ADC value to voltage
    pub fn convert_raw_to_voltage(&mut self, raw: Option<u32>) -> Option<f32> {
        // returns voltage in V
        let v_ref = self.v_ref.to_voltage();
        raw.map(|raw| (v_ref as f64 / ((1 << 23) as f64) * (raw as f64)) as f32)
    }

    /// Reset the device
    pub fn reset(&mut self) -> Result<(), Error<E>> {
        self.bus.write_data(Commands::Reset as u8)
    }

    /// Start a measurement
    pub fn start(&mut self) -> Result<(), Error<E>> {
        self.bus.write_data(Commands::StartSync as u8)
    }
}
