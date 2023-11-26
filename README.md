# Texas Instruments ADS122x04 Rust Driver

[![crates.io](https://img.shields.io/crates/v/ads122x04.svg)](https://crates.io/crates/ads122x04)
[![Docs](https://docs.rs/ads122x04/badge.svg)](https://docs.rs/ads122x04)
[![Rust](https://github.com/hacknus/ads122x04-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/hacknus/ads122x04-rs/actions/workflows/rust.yml)

A platform agnostic rust driver for the [ADS122U04](https://www.ti.com/lit/ds/symlink/ads122u04.pdf) (UART)
and [ADS122C04](https://www.ti.com/lit/ds/symlink/ads122c04.pdf) (I2C) ADC from Texas Instruments.

To use this driver, consult the I2C example below:

```rust
use ads122x04::{interface::*, registers::*, ADS122x04, Error as ADS122x04Error};

{
    let mut adc = ADS122x04::new_i2c(address, i2c);
    adc.reset();
    adc.calibrate_offset();
    adc.set_input_mux(Mux::Ain1Ain0);
    adc.set_conversion_mode(ConversionMode::Continuous);
    adc.set_current_level(CurrentSource::Off);
    adc.set_current_route_1(CurrentRoute::Ain3);
    adc.start();
    let measurement = adc.get_voltage();
}

```

TODO:
- [ ] test UART
- [ ] implement CRC

### Products That Use This Library

- [Qwiic PT100 - ADS122C04 (SPX-16770)](https://www.sparkfun.com/products/16770)  
  Sparkfun sensor development board. Can be used to read a PT100 RTD.