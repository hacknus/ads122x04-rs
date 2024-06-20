//! I2C/UART interfaces

use embedded_hal::i2c::{I2c};
use embedded_io::{Read, ReadExactError, Write};

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

impl<I2C> WriteData for I2cInterface<I2C>
where
    I2C: I2c,
{
    type Error = Error<I2C::Error>;
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error> {
        let register = Commands::WReg as u8 | (register << 2); // write command
        self.i2c
            .write(self.address, &[register, data])
            .map_err(Error::CommError)
    }

    fn write_data(&mut self, payload: u8) -> Result<(), Self::Error> {
        self.i2c.write(self.address, &[payload]).map_err(Error::CommError)
    }
}

impl<UART> WriteData for SerialInterface<UART>
where
    UART: Write + Read,
{
    type Error = Error<UART::Error>;
    fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error> {
        let register = Commands::WReg as u8 | (register << 2); // write command
        self.serial.write_all(&[0x55, register, data]).map_err(Error::CommError)?;
        self.serial.flush().map_err(Error::CommError)
    }

    fn write_data(&mut self, payload: u8) -> Result<(), Self::Error> {
        self.serial.write_all(&[0x55, payload]).map_err(Error::CommError)?;
        self.serial.flush().map_err(Error::CommError)
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

impl<I2C> ReadData for I2cInterface<I2C>
where
    I2C: I2c,
{
    type Error = Error<I2C::Error>;
    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error> {
        let register = Commands::RReg as u8 | (register << 2); // read command
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

impl<UART> ReadData for SerialInterface<UART>
where
    UART: Write + Read,
    Error<<UART as embedded_io::ErrorType>::Error>: From<Error<ReadExactError<<UART as embedded_io::ErrorType>::Error>>>,
{
    type Error = Error<UART::Error>;
    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error> {
        let register = Commands::RReg as u8 | (register << 2); // read command
        self.serial.write_all(&[0x55, register]).map_err(Error::CommError)?;
        self.serial.flush().map_err(Error::CommError)?;
        let mut buf = [0; 1];
        self.serial.read_exact(&mut buf).map_err(Error::CommError)?;
        Ok(buf[0])
    }

    fn read_data(&mut self) -> Result<u32, Self::Error> {
        self.serial.write_all(&[0x55, Commands::RData as u8]).map_err(Error::CommError)?;
        self.serial.flush().map_err(Error::CommError)?;
        let mut buf = [0; 3];
        self.serial.read_exact(&mut buf).map_err(Error::CommError)?;
        Ok((buf[0] as u32) << 16 | (buf[1] as u32) << 8 | (buf[2] as u32))
    }
}