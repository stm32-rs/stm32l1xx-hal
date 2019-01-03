// #![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32l1xx_hal as hal;

use cortex_m_semihosting::hprintln;
use hal::lcd::LcdConfig;
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
    let gpiob = dp.GPIOB.split();

    let cfg = LcdConfig::default();
    let mut lcd = dp.LCD.lcd(
        (gpioa.pa8, gpioa.pa9, gpioa.pa10, gpiob.pb9),
        (gpioa.pa1, gpioa.pa2, gpioa.pa3, gpioa.pa6, gpioa.pa7, gpiob.pb0, gpiob.pb1, gpiob.pb3),
        cfg,
        &mut rcc,
    );

    lcd.enable();
    let mut cnt = 0;

    loop {
        lcd.write(0, cnt);
        cnt = cnt.wrapping_add(1);
        delay.delay(100.ms());
    }
}
