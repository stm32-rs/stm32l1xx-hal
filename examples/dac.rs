#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32l1xx_hal as hal;

use hal::prelude::*;
use hal::stm32;
use rt::entry;

enum Dir { Up, Down }

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let gpioa = dp.GPIOA.split();
    let (mut dac1, mut dac2) = dp.DAC.dac((gpioa.pa4, gpioa.pa5));
    
    dac1.enable();
    dac2.enable();
    
    let mut val: u16 = 0;
    let mut dir = Dir::Up;
    
    loop {
        dac1.set(val);
        dac2.set(core::u16::MAX - val);
        match val {
            0 => dir = Dir::Up,
            core::u16::MAX => dir = Dir::Down,
            _ => (),
        };
        match dir {
            Dir::Up => val += 85,
            Dir::Down => val -= 85,
        }
    }
}