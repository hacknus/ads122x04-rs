use std::fmt::Debug;

use embedded_hal::prelude::{_embedded_hal_blocking_i2c_Write, _embedded_hal_blocking_i2c_WriteRead};

use crate::registers::*;

mod registers;

#[derive(Debug)]
pub enum Error<I2cError>
{
    InvalidValue,
    I2CError(I2cError),
}

pub struct ADS122C04<I2C>
    where
        I2C: _embedded_hal_blocking_i2c_Write
        + _embedded_hal_blocking_i2c_WriteRead,
{
    i2c: I2C,
    v_ref: VRef,
    gain: u8,
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
    addr: u8,
}

impl<I2C, I2cError> ADS122C04<I2C>
    where
        I2C: _embedded_hal_blocking_i2c_Write<Error=I2cError>
        + _embedded_hal_blocking_i2c_WriteRead<Error=I2cError>,
{
    /// Create a new ADS122C04 device by supplying an I2C address and I2C handler
    pub fn new(addr: u8, i2c: I2C) -> Self {
        ADS122C04 {
            i2c,
            v_ref: VRef::Internal,
            gain: 1,
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
            addr,
        }
    }


    /// reads one byte from a specified register
    fn read(&mut self, mut reg: u8) -> Result<u8, Error<I2cError>> {
        reg = Commands::RReg as u8 | (reg << 1); // read command
        let mut buffer = [0];
        self.i2c
            .write_read(self.addr, &[reg], &mut buffer)
            .map(|_| buffer[0])
            .map_err(|err| Error::I2CError(err))
    }

    /// writes one byte into a specified register
    fn write(&mut self, mut reg: u8, val: u8) -> Result<(), Error<I2cError>> {
        reg = Commands::WReg as u8 | (reg << 1); // write command
        self.i2c
            .write(self.addr, &[reg, val])
            .map_err(|err| Error::I2CError(err))
    }

    /// updates a specified config register

    fn update_reg(&mut self, reg: u8) -> Result<(), Error<I2cError>> {
        match reg {
            0x00 => {
                let val = (self.pga_bypass as u8) | (self.gain << 1) | ((self.mux as u8) << 4);
                self.write(0x00, val)
            }
            0x01 => {
                let val = (self.temperature_sensor_mode as u8)
                    | (self.v_ref.to_val() << 1)
                    | ((self.conversion_mode as u8) << 3)
                    | ((self.data_rate as u8) << 4);
                self.write(0x01, val)
            }
            0x02 => {
                let val = (self.current_source as u8)
                    | ((self.burn_out_current_sources as u8) << 3)
                    | ((self.crc as u8) << 4)
                    | ((self.data_counter_enable as u8) << 6);
                self.write(0x02, val)
            }
            0x03 => {
                let val = ((self.current_route_2 as u8) << 3) | ((self.current_route_1 as u8) << 5);
                self.write(0x03, val)
            }
            _ => Err(Error::InvalidValue),
        }
    }

    /// reads a specified config register
    fn read_reg(&mut self, reg: u8) -> Result<u8, Error<I2cError>> {
        match reg {
            0x00 => self.read(0x00),
            0x01 => self.read(0x01),
            0x02 => self.read(0x02),
            0x03 => self.read(0x03),
            _ => Err(Error::InvalidValue),
        }
    }

    /// Enable or disable the programmable gain amplifier (PGA)
    pub fn set_pga_bypass(&mut self, state: bool) -> Result<(), Error<I2cError>> {
        self.pga_bypass = state;
        self.update_reg(0x00)
    }

    /// Read the status of the programmable gain amplifier (PGA)
    pub fn get_pga_bypass(&mut self) -> Result<bool, Error<I2cError>> {
        self.read_reg(0x00).map(|val| (val & 0b1) == 1)
    }

    /// Set the gain as either 1, 2, 4, 8, 16, 32, 64 or 128
    pub fn set_gain(&mut self, gain: u8) -> Result<(), Error<I2cError>> {
        match gain {
            1 | 2 | 4 | 8 | 16 | 32 | 64 | 128 => {
                self.gain = gain;
                self.update_reg(0x00)
            }
            _ => Err(Error::InvalidValue),
        }
    }

    /// Read the gain value
    pub fn get_gain(&mut self) -> Result<u8, Error<I2cError>> {
        self.read_reg(0x00).map(|val| (val >> 1) & 0b111)
    }

    /// Set the input multiplexer (MUX)
    pub fn set_input_mux(&mut self, mux: Mux) -> Result<(), Error<I2cError>> {
        self.mux = mux;
        self.update_reg(0x00)
    }

    /// Read the input multiplexer (MUX) setting
    pub fn get_input_mux(&mut self) -> Result<u8, Error<I2cError>> {
        self.read_reg(0x00).map(|val| val >> 4)
    }

    /// Enable or disable temperature sensor mode (TS)
    pub fn set_temperature_sensor_mode(&mut self, state: bool) -> Result<(), Error<I2cError>> {
        self.temperature_sensor_mode = state;
        self.update_reg(0x01)
    }

    /// Read the temperature sensor mode (TS)
    pub fn get_temperature_sensor_mode(&mut self) -> Result<bool, Error<I2cError>> {
        self.read_reg(0x01).map(|val| (val & 0b1) == 1)
    }

    /// Set the voltage reference (VREF)
    pub fn set_vref(&mut self, v_ref: VRef) -> Result<(), Error<I2cError>> {
        self.v_ref = v_ref;
        self.update_reg(0x01)
    }

    /// Read the voltage reference (VREF)
    pub fn get_vref(&mut self) -> Result<VRef, Error<I2cError>> {
        self.read_reg(0x01)
            .map(|val| VRef::from((val >> 1) & 0b11, self.v_ref.to_voltage()))
    }

    /// Set the conversion mode (CM)
    pub fn set_conversion_mode(&mut self, mode: ConversionMode) -> Result<(), Error<I2cError>> {
        self.conversion_mode = mode;
        self.update_reg(0x01)
    }

    /// Read the conversion mode (CM)
    pub fn get_conversion_mode(&mut self) -> Result<ConversionMode, Error<I2cError>> {
        self.read_reg(0x01)
            .map(|val| ConversionMode::from((val >> 3) & 0b1))
    }

    /// Read the operating mode
    pub fn get_operating_mode(&mut self) -> Result<bool, Error<I2cError>> {
        self.read_reg(0x01).map(|val| ((val >> 4) & 0b1) == 1)
    }

    /// Set the data rate
    pub fn set_data_rate(&mut self, rate: DataRate) -> Result<(), Error<I2cError>> {
        self.data_rate = rate;
        self.turbo_mode = (self.data_rate as u8 & 0b1) == 1;
        self.update_reg(0x01)
    }

    /// Read the data rate
    pub fn get_data_rate(&mut self) -> Result<DataRate, Error<I2cError>> {
        self.read_reg(0x01)
            .map(|val| DataRate::from((val >> 4) & 0b1111))
    }

    /// Set the current level of the internal excitation current sources
    pub fn set_current_level(&mut self, current: CurrentSource) -> Result<(), Error<I2cError>> {
        self.current_source = current;
        self.update_reg(0x02)
    }

    /// Read the current level of the internal excitation current sources
    pub fn get_current_level(&mut self) -> Result<CurrentSource, Error<I2cError>> {
        self.read_reg(0x02)
            .map(|val| CurrentSource::from(val & 0b111))
    }

    /// Enable or disable the 10 uA burnout current sources
    pub fn set_burnout_current_source(&mut self, state: bool) -> Result<(), Error<I2cError>> {
        self.burn_out_current_sources = state;
        self.update_reg(0x02)
    }

    /// Read the state of the 10 uA burnout current sources
    pub fn get_burnout_current_source(&mut self) -> Result<bool, Error<I2cError>> {
        self.read_reg(0x02).map(|val| ((val >> 3) & 0b1) == 1)
    }

    /// Set the CRC mode
    pub fn set_crc(&mut self, crc: Crc) -> Result<(), Error<I2cError>> {
        self.crc = crc;
        self.update_reg(0x02)
    }

    /// Read the CRC mode
    pub fn get_crc(&mut self) -> Result<Crc, Error<I2cError>> {
        self.read_reg(0x02).map(|val| Crc::from((val >> 4) & 0b11))
    }

    /// Enable or disable data counter
    pub fn set_data_counter(&mut self, state: bool) -> Result<(), Error<I2cError>> {
        self.data_counter_enable = state;
        self.update_reg(0x02)
    }

    /// Read the state of the data counter
    pub fn get_data_counter(&mut self) -> Result<bool, Error<I2cError>> {
        self.read_reg(0x02).map(|val| ((val >> 6) & 0b1) == 1)
    }

    /// Read the data ready (DRDY) register
    pub fn get_data_ready(&mut self) -> Result<bool, Error<I2cError>> {
        self.read_reg(0x02).map(|val| ((val >> 7) & 0b1) == 1)
    }

    /// Set the current routing of the excitation current source 1
    pub fn set_current_route_1(&mut self, route: CurrentRoute) -> Result<(), Error<I2cError>> {
        self.current_route_1 = route;
        self.update_reg(0x03)
    }

    /// Read the current routing of the excitation current source 1
    pub fn get_current_route_1(&mut self) -> Result<CurrentRoute, Error<I2cError>> {
        self.read_reg(0x03)
            .map(|val| CurrentRoute::from((val >> 5) & 0b111))
    }

    /// Set the current routing of the excitation current source 2
    pub fn set_current_route_2(&mut self, route: CurrentRoute) -> Result<(), Error<I2cError>> {
        self.current_route_2 = route;
        self.update_reg(0x03)
    }

    /// Read the current routing of the excitation current source 2
    pub fn get_current_route_2(&mut self) -> Result<CurrentRoute, Error<I2cError>> {
        self.read_reg(0x03)
            .map(|val| CurrentRoute::from((val >> 3) & 0b111))
    }

    /// Read the raw ADC value
    pub fn get_raw_adc(&mut self) -> Result<u32, Error<I2cError>> {
        let mut buffer = [0, 0, 0];
        self.i2c
            .write_read(self.addr, &[Commands::RData as u8], &mut buffer)
            .map(|_| {
                let msb = buffer[0];
                let csb = buffer[1];
                let lsb = buffer[2];
                (msb as u32) << 16 | (csb as u32) << 8 | (lsb as u32)
            })
            .map_err(|err| Error::I2CError(err))
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
    pub fn reset(&mut self) -> Result<(), Error<I2cError>> {
        self.i2c
            .write(self.addr, &[Commands::Reset as u8])
            .map_err(|err| Error::I2CError(err))
    }

    /// Start a measurement
    pub fn start(&mut self) -> Result<(), Error<I2cError>> {
        self.i2c
            .write(self.addr, &[Commands::StartSync as u8])
            .map_err(|err| Error::I2CError(err))
    }
}