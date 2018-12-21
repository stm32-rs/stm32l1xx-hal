// ![deny(warnings)]
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
use hal::timer::Timer;
use hal::rcc::ClockSrc;
use hal::stm32::{self, interrupt, Interrupt};
use rt::entry;

static TIMER: Mutex<RefCell<Option<Timer<stm32::TIM2>>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.clock_src(ClockSrc::HSI).freeze();

    let mut timer = Timer::tim2(dp.TIM2, 1.hz(), clocks);
    timer.listen();

    cp.NVIC.enable(Interrupt::TIM2);
    
    cortex_m::interrupt::free(move |cs| {
        *TIMER.borrow(cs).borrow_mut() = Some(timer);
    });

    loop {}
}

#[interrupt]
fn TIM2() {
    static mut COUNTER: u32 = 0;
    *COUNTER += 1;
    hprintln!("{}", COUNTER).unwrap();

    cortex_m::interrupt::free(|cs| {
        if let &mut Some(ref mut timer) = TIMER.borrow(cs).borrow_mut().deref_mut() {
           timer.clear_interrupt();
        }
    });
}
