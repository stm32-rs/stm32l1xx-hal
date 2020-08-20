#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate nb;
extern crate panic_semihosting;
extern crate stm32l1xx_hal as hal;

use embedded_hal::digital::v2::ToggleableOutputPin;
use hal::prelude::*;
use hal::rcc::Config;
use hal::stm32;
use nb::block;
use rt::entry;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(Config::hsi());

    let gpiob = dp.GPIOB.split();
    let mut led = gpiob.pb6.into_push_pull_output();

    let mut timer = dp.TIM2.timer(2.hz(), &mut rcc);

    loop {
        led.toggle().unwrap();
        block!(timer.wait()).unwrap();
    }
}
