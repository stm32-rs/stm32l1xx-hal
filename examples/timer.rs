#![deny(warnings)]
#![no_main]
#![no_std]

#[macro_use]
extern crate cortex_m_semihosting as sh;
extern crate stm32l1xx_hal as hal;
extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;

use hal::prelude::*;
use hal::timer::Timer;
use hal::rcc::SysClkSource;
use hal::stm32::{self, interrupt};
use rt::entry;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sys_clk_src(SysClkSource::HSI).freeze();

    let mut timer = Timer::tim2(dp.TIM2, 2.hz(), clocks);
    timer.listen(&mut cp.NVIC);
    timer.start(2.hz());

    loop {}
}

#[interrupt]
fn TIM2() {
    static mut COUNT: i32 = 0;
    *COUNT += 1;
    hprintln!("TICK # {}", COUNT).unwrap();
}
