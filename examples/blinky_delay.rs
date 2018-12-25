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
use hal::stm32;
use rt::entry;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC.freeze(Config::default());
    let mut delay = cp.SYST.delay(rcc.clocks);

    let gpiob = dp.GPIOB.split();
    let mut led = gpiob.pb6.into_push_pull_output();

    loop {
        led.toggle();
        delay.delay(300.ms());
    }
}
