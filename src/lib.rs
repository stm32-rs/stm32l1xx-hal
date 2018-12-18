#![no_std]
#![allow(non_camel_case_types)]

extern crate bare_metal;
extern crate cast;
extern crate cortex_m;
extern crate cortex_m_semihosting as sh;
extern crate void;

pub extern crate embedded_hal as hal;
pub extern crate nb;

pub use nb::block;

pub extern crate stm32l1;

#[cfg(feature = "stm32l100")]
pub use stm32l1::stm32l100 as stm32;

#[cfg(feature = "stm32l151")]
pub use stm32l1::stm32l151 as stm32;

#[cfg(feature = "stm32l152")]
pub use stm32l1::stm32l151 as stm32;

#[cfg(feature = "stm32l162")]
pub use stm32l1::stm32l162 as stm32;

#[cfg(feature = "rt")]
pub use stm32l1::interrupt;

pub mod delay;
pub mod gpio;
pub mod prelude;
pub mod rcc;
pub mod time;
pub mod timer;
pub mod pwm;
pub mod serial;
// pub mod i2c;
// pub mod spi;
