//! I2C/UART interfaces

use embedded_hal::{
    blocking::i2c,
    blocking::serial,
    serial as serial_nb,
};
use nb::block;

use crate::{Error, private};
use crate::registers::*;

/// I2C interface
#[derive(Debug)]
pub struct I2cInterface<I2C> {
    pub(crate) i2c: I2C,
    pub(crate) address: u8,
}

/// UART interface
#[derive(Debug)]
pub struct SerialInterface<UART> {
    pub(crate) serial: UART,
}

/// Write data
pub trait WriteData: private::Sealed {
    /// Error type
    type Error;
    /// Write to an u8 register
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error>;
    /// Write data. The first element corresponds to the starting address.
    fn write_data(&mut self, payload: u8) -> Result<(), Self::Error>;
}

impl<I2C, E> WriteData for I2cInterface<I2C>
    where
        I2C: i2c::Write<Error=E>,
{
    type Error = Error<E>;
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error> {
        let register = Commands::WReg as u8 | (register << 1); // write command
        self.i2c
            .write(self.address, &[register, data])
            .map_err(Error::CommError)
    }

    fn write_data(&mut self, payload: u8) -> Result<(), Self::Error> {
        self.i2c.write(self.address, &[payload]).map_err(Error::CommError)
    }
}

impl<UART, E> WriteData for SerialInterface<UART>
    where
        UART: serial::Write<u8, Error=E> + serial_nb::Read<u8, Error=E>,
{
    type Error = Error<E>;
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error> {
        let register = Commands::WReg as u8 | (register << 1); // write command
        self.serial.bwrite_all(&[0x55, register, data]).map_err(Error::CommError)?;
        self.serial.bflush().map_err(Error::CommError)
    }

    fn write_data(&mut self, payload: u8) -> Result<(), Self::Error> {
        self.serial.bwrite_all(&[0x55, payload]).map_err(Error::CommError)?;
        self.serial.bflush().map_err(Error::CommError)
    }
}

/// Read data
pub trait ReadData: private::Sealed {
    /// Error type
    type Error;
    /// Read an u8 register
    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error>;
    /// Read some data. The first element corresponds to the starting address.
    fn read_data(&mut self) -> Result<u32, Self::Error>;
}

impl<I2C, E> ReadData for I2cInterface<I2C>
    where
        I2C: i2c::WriteRead<Error=E>,
{
    type Error = Error<E>;
    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error> {
        let register = Commands::RReg as u8 | (register << 1); // read command
        let mut buffer = [0];
        self.i2c
            .write_read(self.address, &[register], &mut buffer)
            .map(|_| buffer[0])
            .map_err(Error::CommError)
    }

    fn read_data(&mut self) -> Result<u32, Self::Error> {
        let mut buffer = [0, 0, 0];
        self.i2c
            .write_read(self.address, &[Commands::RData as u8], &mut buffer)
            .map(|_| {
                let msb = buffer[0];
                let csb = buffer[1];
                let lsb = buffer[2];
                (msb as u32) << 16 | (csb as u32) << 8 | (lsb as u32)
            })
            .map_err(Error::CommError)
    }
}

impl<UART, E> ReadData for SerialInterface<UART>
    where
        UART: serial::Write<u8, Error=E> + serial_nb::Read<u8, Error=E>,
{
    type Error = Error<E>;
    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error> {
        let register = Commands::RReg as u8 | (register << 1); // read command
        self.serial.bwrite_all(&[0x55, register]).map_err(Error::CommError)?;
        self.serial.bflush().map_err(Error::CommError)?;
        block!(self.serial.read()).map_err(Error::CommError)
    }

    fn read_data(&mut self) -> Result<u32, Self::Error> {
        self.serial.bwrite_all(&[0x55, Commands::RData as u8]).map_err(Error::CommError)?;
        self.serial.bflush().map_err(Error::CommError)?;
        let msb = block!(self.serial.read()).map_err(Error::CommError)?;
        let csb = block!(self.serial.read()).map_err(Error::CommError)?;
        let lsb = block!(self.serial.read()).map_err(Error::CommError)?;
        Ok((msb as u32) << 16 | (csb as u32) << 8 | (lsb as u32))
    }
}