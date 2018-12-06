#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate panic_semihosting;
extern crate stm32l1xx_hal as hal;

use core::fmt::Write;
use rt::{entry, exception, ExceptionFrame};

#[entry]
fn main() -> ! {
    let mut hstdout = sh::hio::hstdout().unwrap();
    writeln!(hstdout, "Hello, world!").unwrap();
    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

#[exception]
fn DefaultHandler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}

