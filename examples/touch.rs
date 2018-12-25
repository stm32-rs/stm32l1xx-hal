// #![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32l1xx_hal as hal;

use hal::prelude::*;
use hal::rcc::Config;
use hal::{spi, stm32};
use rt::entry;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.freeze(Config::hsi());
    let gpiob = dp.GPIOB.split();

    let mut sample = gpiob.pb4;
    let mut c1 = gpiob.pb5;
    let mut c2 = gpiob.pb6;
    let mut c3 = gpiob.pb7;

    loop {}
}
