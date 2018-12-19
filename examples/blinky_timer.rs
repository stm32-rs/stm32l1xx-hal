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

use hal::prelude::*;
use hal::stm32;
use hal::timer::Timer;
use hal::rcc::SysClkSource;
use rt::entry;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sys_clk_src(SysClkSource::HSI).freeze();
    
    let gpiob = dp.GPIOB.split();
    let mut led = gpiob.pb6.into_push_pull_output();

    let mut timer = Timer::tim2(dp.TIM2, 2.hz(), clocks);
 
    loop {
        led.toggle();
        block!(timer.wait()).unwrap();
    }
}