// cargo run --example rtfm --features=stm32l100
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
use hal::gpio::{Output, PushPull};
use hal::gpio::gpiob::{PB6, PB7};
use hal::timer::Timer;
use hal::rcc::SysClkSource;

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
        let clocks = rcc.cfgr
            .sys_clk_src(SysClkSource::HSI)
            .freeze();

        device.EXTI.ftsr.modify(|_, w| w.tr0().set_bit());
        device.EXTI.imr.modify(|_, w| w.mr0().set_bit());
        device.SYSCFG.exticr1.modify(|_, w| unsafe { w.exti0().bits(0) });

        let mut timer = Timer::tim2(device.TIM2, 1.hz(), clocks);
        timer.listen();

        let gpiob = device.GPIOB.split();

        GREEN_LED = gpiob.pb7.into_push_pull_output();
        BLUE_LED = gpiob.pb6.into_push_pull_output();
        TIMER = timer;
        EXTI = device.EXTI;
    }

    #[interrupt(resources = [TIMER, BLUE_LED, DELTA])]
    fn TIM2() {
        resources.TIMER.clear_interrupt();
        resources.BLUE_LED.toggle();
        *resources.DELTA += 1;
    }
    
    #[interrupt(resources = [EXTI, GREEN_LED, DELTA])]
    fn EXTI0() {
        resources.EXTI.pr.modify(|_, w| w.pr0().set_bit());
        
        resources.GREEN_LED.set_high();
        hprintln!("Δ: {}", resources.DELTA).unwrap();
        resources.GREEN_LED.set_low();
        
        *resources.DELTA = 0;
    }

    #[idle]
    fn idle() -> ! {
        loop {}
    }
};
