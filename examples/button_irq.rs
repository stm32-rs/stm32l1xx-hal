#![deny(warnings)]
#![no_main]
#![no_std]

#[macro_use]
extern crate cortex_m_semihosting as sh;
extern crate stm32l1xx_hal as hal;
extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;

use core::cell::RefCell;
use core::ops::DerefMut;

use cortex_m::interrupt::Mutex;
use hal::prelude::*;
use hal::exti::TriggerEdge;
use hal::stm32::{self, interrupt, Interrupt, EXTI};
use rt::entry;

static INT: Mutex<RefCell<Option<EXTI>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let mut cp = cortex_m::Peripherals::take().unwrap();
    
    cp.NVIC.enable(Interrupt::EXTI0);
    dp.EXTI.listen(0, TriggerEdge::Falling);
    
    cortex_m::interrupt::free(move |cs| {
        *INT.borrow(cs).borrow_mut() = Some(dp.EXTI);
    });

    loop {}
}

#[interrupt]
fn EXTI0() {
    static mut COUNT: i32 = 0;
    
    *COUNT += 1;
    hprintln!("CLICK # {}", COUNT).unwrap();

    cortex_m::interrupt::free(|cs| {
        if let &mut Some(ref mut exti) = INT.borrow(cs).borrow_mut().deref_mut() {
            exti.clear_interrupt(0);
        }
    });
}
