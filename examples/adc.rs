#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32l1xx_hal as hal;

use hal::adc::Precision;
use hal::adc::VRef;
use hal::prelude::*;
use hal::rcc::Config;
use hal::stm32;
use rt::entry;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(Config::hsi());
    let gpioa = dp.GPIOA.split();

    let mut adc = dp.ADC.adc(&mut rcc);
    adc.set_precision(Precision::B_12);

    let mut chan = gpioa.pa0.into_analog();
    let mut vref = VRef::new();

    let vref_cal: u16 = VRef::get_vrefcal();
    /* scale for 12-bit range */
    let scale = 4095;

    vref.enable(&mut adc);

    loop {
        let chan_val: u16 = adc.read(&mut chan).unwrap();
        let vref_val: u16 = adc.read(&mut vref).unwrap();

        let _absolute_voltage = 3 * vref_cal * chan_val / vref_val / scale;
    }
}
