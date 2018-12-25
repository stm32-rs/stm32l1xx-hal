#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32l1xx_hal as hal;

use hal::hal::Direction;
use hal::prelude::*;
use hal::stm32;
use hal::rcc::Config;
use rt::entry;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let mut rcc = dp.RCC.freeze(Config::hsi());

    let gpioa = dp.GPIOA.split();
    let mut dac = dp.DAC.dac(gpioa.pa4, &mut rcc);
    
    let mut dir = Direction::Upcounting;
    let mut val: u16 = 0;

    dac.enable();

    loop {
        dac.set_value(val);
        match val {
            0 => dir = Direction::Upcounting,
            4080 => dir = Direction::Downcounting,
            _ => (),
        };

        match dir {
            Direction::Upcounting => val += 1,
            Direction::Downcounting => val -= 1,
        }
    }
}
