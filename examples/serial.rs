#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate nb;
extern crate panic_semihosting;
extern crate stm32l1xx_hal as hal;

use core::fmt::Write;
use hal::prelude::*;
use hal::rcc::Config;
use hal::serial;
use hal::stm32;
use nb::block;
use rt::entry;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(Config::hsi());
    
    let gpiob = dp.GPIOB.split();
    let tx = gpiob.pb10;
    let rx = gpiob.pb11;

    let serial = dp
        .USART3
        .usart((tx, rx), serial::Config::default(), &mut rcc)
        .unwrap();

    let (mut tx, mut rx) = serial.split();

    loop {
        let received = block!(rx.read()).unwrap();
        tx.write_str("\r\n").unwrap();
        block!(tx.write(received)).ok();
    }
}
