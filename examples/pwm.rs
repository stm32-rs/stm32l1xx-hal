#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32l1xx_hal as hal;

use cortex_m::asm;
use hal::prelude::*;
use hal::stm32;
use rt::entry;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();
    
    let gpioa = dp.GPIOA.split();
     
    let c1 = gpioa.pa0.into_alternate_af2();
    let mut pwm = dp.TIM5.pwm(c1, 10.khz(), clocks);

    let max = pwm.get_max_duty();

    pwm.enable();
    
    // full
    pwm.set_duty(max);
    asm::bkpt();

    // dim
    pwm.set_duty(max / 2);
    asm::bkpt();

    // zero
    pwm.set_duty(0);
    asm::bkpt();

    loop { }
}