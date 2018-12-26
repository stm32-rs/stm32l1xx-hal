#![deny(warnings)]
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

    let sck = gpiob.pb3;
    let miso = gpiob.pb4;
    let mosi = gpiob.pb5;

    let mut spi = dp
        .SPI3
        .spi((sck, miso, mosi), spi::MODE_0, 100.khz(), &mut rcc);

    loop {
        spi.write(&[0, 1]).unwrap();
    }
}
