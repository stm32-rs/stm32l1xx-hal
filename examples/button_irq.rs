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
use hal::stm32::{self, interrupt, Interrupt, EXTI};
use rt::entry;

static INT: Mutex<RefCell<Option<EXTI>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let mut cp = cortex_m::Peripherals::take().unwrap();

    let exti = dp.EXTI;
    // Falling edge
    exti.ftsr.modify(|_, w| w.tr0().set_bit());
    // Interrupt mask
    exti.imr.modify(|_, w| w.mr0().set_bit());
    
    // Enable external interrupt on EXTI0
    dp.SYSCFG.exticr1.modify(|_, w| unsafe { w.exti0().bits(0) });
    
    // Enable interrupt on EXTI0 line
    cp.NVIC.enable(Interrupt::EXTI0);

    cortex_m::interrupt::free(move |cs| {
        *INT.borrow(cs).borrow_mut() = Some(exti);
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
            // Clear pending bit
            exti.pr.modify(|_, w| w.pr0().set_bit());
        }
    });
}
