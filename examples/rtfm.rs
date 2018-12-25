#![deny(warnings)]
#![no_main]
#![no_std]
#![feature(custom_attribute)]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting;
extern crate panic_semihosting;
extern crate rtfm;
extern crate stm32l1xx_hal as hal;

use cortex_m_semihosting::hprintln;
use rtfm::app;

use hal::exti::TriggerEdge;
use hal::gpio::gpiob::{PB6, PB7};
use hal::gpio::{Output, PushPull};
use hal::prelude::*;
use hal::rcc::Config;
use hal::stm32;
use hal::timer::Timer;

#[app(device = hal::stm32)]
const APP: () = {
    static mut DELTA: u32 = 0;
    static mut TIMER: Timer<stm32::TIM2> = ();
    static mut TICKS_LED: PB6<Output<PushPull>> = ();
    static mut BUSY_LED: PB7<Output<PushPull>> = ();
    static EXTI: stm32::EXTI = ();

    #[init]
    fn init() {
        let mut rcc = device.RCC.freeze(Config::hsi());

        let gpiob = device.GPIOB.split();
        let mut timer = device.TIM2.timer(1.hz(), &mut rcc);

        timer.listen();
        device.EXTI.listen(0, TriggerEdge::Rising);

        TICKS_LED = gpiob.pb6.into_push_pull_output();
        BUSY_LED = gpiob.pb7.into_push_pull_output();
        TIMER = timer;
        EXTI = device.EXTI;
    }

    #[interrupt(resources = [TIMER, TICKS_LED, DELTA])]
    fn TIM2() {
        *resources.DELTA += 1;

        resources.TICKS_LED.toggle();
        resources.TIMER.clear_irq();
    }

    #[interrupt(resources = [EXTI, BUSY_LED, DELTA])]
    fn EXTI0() {
        resources.BUSY_LED.set_high();
        hprintln!("Δ: {}", resources.DELTA).unwrap();
        resources.BUSY_LED.set_low();

        *resources.DELTA = 0;
        resources.EXTI.clear_irq(0);
    }

    #[idle]
    fn idle() -> ! {
        loop {}
    }
};
