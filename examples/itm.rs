#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32l1xx_hal as hal;

use cortex_m::iprintln;
use rt::{entry, exception, ExceptionFrame};
#[entry]
fn main() -> ! {
    let p = cortex_m::Peripherals::take().unwrap();
    let mut itm = p.ITM;
    iprintln!(&mut itm.stim[0], "Hello, world!");
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
