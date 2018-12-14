// #![deny(warnings)]
#![no_main]
#![no_std]

#[macro_use]
extern crate stm32l1xx_hal as hal;
extern crate cortex_m;
extern crate cortex_m_rt as rt;
#[macro_use]
extern crate cortex_m_semihosting as sh;
extern crate panic_semihosting;

use core::fmt::Write;

use hal::prelude::*;
use hal::timer::Timer;
use hal::rcc::SysClockSource;
use hal::stm32;
use rt::entry;

interrupt!(TIM2, tim2_tick, state: u8 = 0);

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let mut cp = cortex_m::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sys_clk_src(SysClockSource::HSI).freeze();
   
    let mut timer = Timer::tim2(dp.TIM2, 2.hz(), clocks);
    timer.listen(&mut cp.NVIC);
    timer.start(2.hz());

    loop {}
}

fn tim2_tick(state: &mut u8) {
    *state += 1;
    hprintln!("TICK # {}", state).unwrap();
}
