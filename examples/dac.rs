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
    let mut dac = dp.DAC.dac(gpioa.pa4);

    let mut val: u16 = 0;
    let mut dir = Dir::Up;

    dac.enable();

    loop {
        dac.set_value(val);
        match val {
            0 => dir = Dir::Up,
            4080 => dir = Dir::Down,
            _ => (),
        };
        match dir {
            Dir::Up => val += 1,
            Dir::Down => val -= 1,
        }
    }
}