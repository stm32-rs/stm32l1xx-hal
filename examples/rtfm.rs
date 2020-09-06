#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting;
extern crate panic_semihosting;
extern crate rtic;
extern crate stm32l1xx_hal as hal;

use cortex_m_semihosting::hprintln;
use rtic::app;

use embedded_hal::digital::v2::OutputPin;
use embedded_hal::digital::v2::ToggleableOutputPin;
use hal::exti::TriggerEdge;
use hal::gpio::gpiob::{PB6, PB7};
use hal::gpio::{Output, PushPull};
use hal::prelude::*;
use hal::rcc::Config;
use hal::stm32;
use hal::timer::Timer;

#[app(device = hal::stm32, peripherals = true)]
const APP: () = {
    struct Resources {
        // resources
        #[init(0)]
        DELTA: u32,

        // late resources
        TIMER: Timer<stm32::TIM2>,
        TICKS_LED: PB6<Output<PushPull>>,
        BUSY_LED: PB7<Output<PushPull>>,
        EXTI: stm32::EXTI,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        let mut rcc = cx.device.RCC.freeze(Config::hsi());

        let gpiob = cx.device.GPIOB.split();
        let mut timer = cx.device.TIM2.timer(1.hz(), &mut rcc);

        timer.listen();
        cx.device.EXTI.listen(0, TriggerEdge::Rising);

        let TICKS_LED = gpiob.pb6.into_push_pull_output();
        let BUSY_LED = gpiob.pb7.into_push_pull_output();
        let TIMER = timer;
        let EXTI = cx.device.EXTI;

        init::LateResources {
            TIMER,
            TICKS_LED,
            BUSY_LED,
            EXTI,
        }
    }

    #[task(binds = TIM2, resources = [TIMER, TICKS_LED, DELTA])]
    fn tim2_handler(cx: tim2_handler::Context) {
        *cx.resources.DELTA += 1;

        cx.resources.TICKS_LED.toggle().unwrap();
        cx.resources.TIMER.clear_irq();
    }

    #[task(binds = EXTI0, resources = [EXTI, BUSY_LED, DELTA])]
    fn exti0_handler(cx: exti0_handler::Context) {
        cx.resources.BUSY_LED.set_high().unwrap();
        hprintln!("Δ: {}", cx.resources.DELTA).unwrap();
        cx.resources.BUSY_LED.set_low().unwrap();

        *cx.resources.DELTA = 0;
        cx.resources.EXTI.clear_irq(0);
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {}
    }
};
