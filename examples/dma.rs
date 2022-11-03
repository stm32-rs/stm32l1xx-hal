#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate panic_semihosting;
extern crate stm32l1xx_hal as hal;

use hal::prelude::*;
use hal::rcc::Config;
use hal::stm32;
use rt::entry;
use sh::hprintln;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.freeze(Config::hsi());

    let dma1 = dp.DMA1.dma(&mut rcc);
    let dma2 = dp.DMA2.dma(&mut rcc);

    hprintln!("DMA1: {:?}, DMA2: {:?}", dma1, dma2);
    loop {}
}
