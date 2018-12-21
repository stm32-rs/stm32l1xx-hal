#![no_std]
#![allow(non_camel_case_types)]

extern crate bare_metal;
extern crate cast;
extern crate cortex_m;
extern crate void;

pub extern crate embedded_hal as hal;
pub extern crate nb;
pub extern crate stm32l1;

pub use nb::block;

#[cfg(feature = "stm32l100")]
pub use stm32l1::stm32l100 as stm32;

#[cfg(any(feature = "stm32l151", feature = "stm32l152"))]
pub use stm32l1::stm32l151 as stm32;

#[cfg(feature = "stm32l162")]
pub use stm32l1::stm32l162 as stm32;

#[cfg(feature = "rt")]
pub use stm32::interrupt;

mod bb;

pub mod dac;
pub mod delay;
pub mod exti;
pub mod gpio;
pub mod prelude;
pub mod pwm;
pub mod rcc;
pub mod serial;
pub mod time;
pub mod timer;
pub mod watchdog;
// pub mod i2c;
// pub mod spi;
