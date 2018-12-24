#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

#[macro_use]
extern crate nb;
extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32l1xx_hal as hal;

use core::fmt::Write;
use hal::prelude::*;
use hal::rcc::ClockSrc;
use hal::serial::Config;
use hal::stm32;
use rt::entry;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.clock_src(ClockSrc::HSI).freeze();

    let gpiob = dp.GPIOB.split();
    let tx = gpiob.pb10;
    let rx = gpiob.pb11;

    let serial = dp
        .USART3
        .usart((tx, rx), Config::default(), clocks)
        .unwrap();

    let (mut tx, mut rx) = serial.split();

    loop {
        let received = block!(rx.read()).unwrap();
        tx.write_str("\r\n").unwrap();
        block!(tx.write(received)).ok();
    }
}
