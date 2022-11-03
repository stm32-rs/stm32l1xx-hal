#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32l1xx_hal as hal;

use cortex_m_semihosting::hprintln;
use hal::prelude::*;
use hal::rcc::Config;
use hal::stm32;
use rt::entry;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(Config::hsi());
    let mut delay = cp.SYST.delay(rcc.clocks);

    let gpioa = dp.GPIOA.split();
    let qei = dp.TIM2.qei((gpioa.pa0, gpioa.pa1), &mut rcc);

    loop {
        let before = qei.count();
        delay.delay_ms(500_u16);
        let after = qei.count();

        let elapsed = after.wrapping_sub(before) as i16;
        hprintln!("Δ: {}", elapsed);
    }
}
