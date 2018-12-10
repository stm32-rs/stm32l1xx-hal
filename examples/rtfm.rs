//#![deny(unsafe_code)]
//#![deny(warnings)]
#![no_main]
#![no_std]
#![feature(custom_attribute)]

#[macro_use]
extern crate stm32l1xx_hal as hal;
extern crate panic_semihosting;
extern crate rtfm;
extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting;

use cortex_m_semihosting::{debug, hprintln};
use hal::stm32::Interrupt;
use rtfm::{app, Mutex};

#[app(device = hal::stm32)]
const APP: () = {
    static mut CLICKS: u32 = 0;

    #[init]
    fn init() {
        // Cortex-M peripherals
        let core: rtfm::Peripherals = core;
        // Device specific peripherals
        let device: hal::stm32::Peripherals = device;
               
        // Falling edge
        device.EXTI.ftsr.modify(|_, w| w.tr0().set_bit());
        // Interrupt mask
        device.EXTI.imr.modify(|_, w| w.mr0().set_bit());
        // Enable external interrupt on EXTI0
        device.SYSCFG.exticr1.modify(|_, w| unsafe { w.exti0().bits(0) });

        // Enable interrupt on EXTI0 line
        // core.NVIC.enable(Interrupt::EXTI0);
    }

    #[interrupt(resources = [CLICKS])]
    fn EXTI0() {
        *resources.CLICKS += 1;
        hprintln!("EXTI0(CLICKS = {})", resources.CLICKS).unwrap();      
    }
};