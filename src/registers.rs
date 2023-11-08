//! ADS122x04 registers and commands
#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
/// Commands to send to the device
pub enum Commands {
    /// Reset the device
    Reset = 0b110,
    /// Send START/SYNC command to start a measurement/series
    StartSync = 0b1000,
    /// Put the device in PowerDown mode
    PowerDown = 0b10,
    /// Request the data
    RData = 0b10000,
    /// Command to read a register
    RReg = 0b0100000,
    /// Command to write to a register
    WReg = 0b1000000,
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code, missing_docs)]
pub enum Mux {
    Ain0Ain1 = 0b0000,
    Ain0Ain2 = 0b0001,
    Ain0Ain3 = 0b0010,
    Ain1Ain0 = 0b0011,
    Ain1Ain2 = 0b0100,
    Ain1Ain3 = 0b0101,
    Ain2Ain3 = 0b0110,
    Ain3Ain2 = 0b0111,
    Ain0Avss = 0b1000,
    Ain1Avss = 0b1001,
    Ain2Avss = 0b1010,
    Ain3Avss = 0b1011,
    VrefMonitor = 0b1100,
    AvddMonitor = 0b1101,
    Shorted = 0b1110,
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code, missing_docs)]
pub enum DataRate {
    Sps20Normal = 0b0000,
    Sps45Normal = 0b0010,
    Sps90Normal = 0b0100,
    Sps175Normal = 0b0110,
    Sps330Normal = 0b1000,
    Sps600Normal = 0b1010,
    Sps1000Normal = 0b1100,
    Sps40Turbo = 0b0001,
    Sps90Turbo = 0b0011,
    Sps180Turbo = 0b0101,
    Sps350Turbo = 0b0111,
    Sps660Turbo = 0b1001,
    Sps1200Turbo = 0b1011,
    Sps2000Turbo = 0b1101,
}

#[allow(dead_code, missing_docs)]
impl DataRate {
    pub fn from(val: u8) -> Self {
        match val {
            0b0000 => Self::Sps20Normal,
            0b0010 => Self::Sps45Normal,
            0b0100 => Self::Sps90Normal,
            0b0110 => Self::Sps175Normal,
            0b1000 => Self::Sps330Normal,
            0b1010 => Self::Sps600Normal,
            0b1100 => Self::Sps1000Normal,
            0b0001 => Self::Sps40Turbo,
            0b0011 => Self::Sps90Turbo,
            0b0101 => Self::Sps180Turbo,
            0b0111 => Self::Sps350Turbo,
            0b1001 => Self::Sps660Turbo,
            0b1011 => Self::Sps1200Turbo,
            0b1101 => Self::Sps2000Turbo,
            _ => Self::Sps20Normal,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code, missing_docs)]
pub enum Gain {
    Gain1 = 0b000,
    Gain2 = 0b001,
    Gain4 = 0b010,
    Gain8 = 0b011,
    Gain16 = 0b100,
    Gain32 = 0b101,
    Gain64 = 0b110,
    Gain128 = 0b111,
}

#[allow(dead_code, missing_docs)]
impl Gain {
    pub fn from(val: u8) -> Self {
        match val {
            0b000 => Self::Gain1,
            0b001 => Self::Gain2,
            0b010 => Self::Gain4,
            0b011 => Self::Gain8,
            0b100 => Self::Gain16,
            0b101 => Self::Gain32,
            0b110 => Self::Gain64,
            0b111 => Self::Gain128,
            _ => Self::Gain1,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code, missing_docs)]
pub enum CurrentSource {
    Off = 0b000,
    I10uA = 0b001,
    I50uA = 0b010,
    I100uA = 0b011,
    I250uA = 0b100,
    I500uA = 0b101,
    I1000uA = 0b110,
    I1500uA = 0b111,
}

#[allow(dead_code, missing_docs)]
impl CurrentSource {
    pub fn from(val: u8) -> Self {
        match val {
            0b001 => Self::I10uA,
            0b010 => Self::I50uA,
            0b011 => Self::I100uA,
            0b100 => Self::I250uA,
            0b101 => Self::I500uA,
            0b110 => Self::I1000uA,
            0b111 => Self::I1500uA,
            _ => Self::Off,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code, missing_docs)]
pub enum CurrentRoute {
    Off = 0b000,
    Ain0 = 0b001,
    Ain1 = 0b010,
    Ain2 = 0b011,
    Ain3 = 0b100,
    RefP = 0b101,
    RefN = 0b110,
}

#[allow(dead_code, missing_docs)]
impl CurrentRoute {
    pub fn from(val: u8) -> Self {
        match val {
            0b001 => Self::Ain0,
            0b010 => Self::Ain1,
            0b011 => Self::Ain2,
            0b100 => Self::Ain3,
            0b101 => Self::RefP,
            0b110 => Self::RefN,
            _ => Self::Off,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code, missing_docs)]
pub enum ConversionMode {
    SingleShot = 0,
    Continuous = 1,
}

#[allow(dead_code, missing_docs)]
impl ConversionMode {
    pub fn from(val: u8) -> Self {
        match val {
            0 => Self::SingleShot,
            _ => Self::Continuous,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code, missing_docs)]
pub enum Crc {
    Disabled = 0b00,
    Inverted = 0b01,
    Crc16 = 0b10,
}

#[allow(dead_code, missing_docs)]
impl Crc {
    pub fn from(val: u8) -> Self {
        match val {
            0b01 => Self::Inverted,
            0b10 => Self::Crc16,
            _ => Self::Disabled,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
/// Voltage reference
pub enum VRef {
    /// use internal Vref of 2.048 volts
    Internal,
    /// use external reference on RefP and RefN pins
    External(f32),
    /// use AVDD-AVSS as a reference
    AnalogSupply(f32),
}

#[allow(dead_code, missing_docs)]
impl VRef {
    pub fn to_val(&self) -> u8 {
        match self {
            VRef::Internal => 0b00,
            VRef::External(_) => 0b01,
            VRef::AnalogSupply(_) => 0b10,
        }
    }

    pub fn to_voltage(&self) -> f32 {
        match self {
            VRef::Internal => 2.048,
            VRef::External(v) => *v,
            VRef::AnalogSupply(v) => *v,
        }
    }

    pub fn from(val: u8, voltage: f32) -> Self {
        match val {
            0b00 => VRef::Internal,
            0b01 => VRef::External(voltage),
            0b10 => VRef::AnalogSupply(voltage),
            _ => VRef::Internal,
        }
    }
}