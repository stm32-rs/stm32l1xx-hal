#![deny(warnings)]
#![no_main]
#![no_std]
#![feature(custom_attribute)]

extern crate stm32l1xx_hal as hal;
extern crate panic_semihosting;
extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting;
extern crate rtfm;

use cortex_m_semihosting::hprintln;
use rtfm::app;

use hal::prelude::*;
use hal::stm32;
use hal::exti::TriggerEdge;
use hal::gpio::{Output, PushPull};
use hal::gpio::gpiob::{PB6, PB7};
use hal::timer::Timer;
use hal::rcc::ClockSrc;

#[app(device = hal::stm32)]
const APP: () = {
    static mut DELTA: u32 = 0;
    static mut TIMER: Timer<stm32::TIM2> = ();
    static mut BLUE_LED: PB6<Output<PushPull>> = ();
    static mut GREEN_LED: PB7<Output<PushPull>> = ();
    static EXTI: stm32::EXTI = ();

    #[init]
    fn init() {
        let rcc = device.RCC.constrain();
        let clocks = rcc.cfgr.clock_src(ClockSrc::HSI).freeze();

        let gpiob = device.GPIOB.split();
        let mut timer = Timer::tim2(device.TIM2, 1.hz(), clocks);

        timer.listen();
        device.EXTI.listen(0, TriggerEdge::Rising);

        GREEN_LED = gpiob.pb7.into_push_pull_output();
        BLUE_LED = gpiob.pb6.into_push_pull_output();
        TIMER = timer;
        EXTI = device.EXTI;
    }

    #[interrupt(resources = [TIMER, BLUE_LED, DELTA])]
    fn TIM2() {
        *resources.DELTA += 1;

        resources.BLUE_LED.toggle();
        resources.TIMER.clear_interrupt();
    }
    
    #[interrupt(resources = [EXTI, GREEN_LED, DELTA])]
    fn EXTI0() {
        resources.GREEN_LED.set_high();
        hprintln!("Δ: {}", resources.DELTA).unwrap();
        resources.GREEN_LED.set_low();

        *resources.DELTA = 0;
        resources.EXTI.clear_interrupt(0);
    }

    #[idle]
    fn idle() -> ! {
        loop {}
    }
};
