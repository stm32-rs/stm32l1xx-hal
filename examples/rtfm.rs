#![deny(warnings)]
#![no_main]
#![no_std]
#![feature(custom_attribute)]

extern crate stm32l1xx_hal as hal;
extern crate panic_semihosting;
extern crate rtfm;
extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting;

use cortex_m_semihosting::hprintln;
use hal::stm32;
use rtfm::app;

#[app(device = hal::stm32)]
const APP: () = {
    static mut CLICKS: u32 = 0;
    static mut EXTI: stm32::EXTI = ();
    static mut SYSCFG: stm32::SYSCFG = ();

    #[init]
    fn init() {
        // Falling edge
        device.EXTI.ftsr.modify(|_, w| w.tr0().set_bit());
        // Interrupt mask
        device.EXTI.imr.modify(|_, w| w.mr0().set_bit());
        // Enable external interrupt on EXTI0
        device.SYSCFG.exticr1.modify(|_, w| unsafe { w.exti0().bits(0) });
        
        EXTI = device.EXTI;
        SYSCFG = device.SYSCFG;
    }

    #[interrupt(resources = [EXTI, SYSCFG])]
    fn EXTI0() {
        static mut COUNT: i32 = 0;
        *COUNT += 1;
        hprintln!("EXTI0 CLICKS: {}", COUNT).unwrap();
        // resources.EXTI.pr.modify(|_, w| w.pr0().set_bit());
        // resources.SYSCFG.exticr1.modify(|_, w| unsafe { w.exti0().bits(0) });
    }
};