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
mod app {
    use super::*;

    #[shared]
    struct Shared {
        #[lock_free]
        delta: u32,
    }

    #[local]
    struct Local {
        timer: Timer<stm32::TIM2>,
        tick_led: PB6<Output<PushPull>>,
        busy_led: PB7<Output<PushPull>>,
        exti: stm32::EXTI,
    }

    #[init]
    fn init(cx: init::Context) -> (Shared, Local, init::Monotonics) {
        let mut rcc = cx.device.RCC.freeze(Config::hsi());

        let gpiob = cx.device.GPIOB.split();
        let mut timer = cx.device.TIM2.timer(1.hz(), &mut rcc);

        timer.listen();
        cx.device.EXTI.listen(0, TriggerEdge::Rising);

        let tick_led = gpiob.pb6.into_push_pull_output();
        let busy_led = gpiob.pb7.into_push_pull_output();
        let exti = cx.device.EXTI;
        let delta = 0;
        (
            Shared { delta },
            Local {
                timer,
                tick_led,
                busy_led,
                exti,
            },
            init::Monotonics(),
        )
    }

    #[task(binds = TIM2, local = [timer, tick_led], shared = [delta])]
    fn tim2_handler(cx: tim2_handler::Context) {
        *cx.shared.delta += 1;
        cx.local.tick_led.toggle().unwrap();
        cx.local.timer.clear_irq();
    }

    #[task(binds = EXTI0, local = [exti, busy_led], shared = [delta])]
    fn exti0_handler(cx: exti0_handler::Context) {
        cx.local.busy_led.set_high().unwrap();
        hprintln!("Δ: {}", *cx.shared.delta);
        cx.local.busy_led.set_low().unwrap();
        *cx.shared.delta = 0;
        cx.local.exti.clear_irq(0);
    }

    #[idle]
    fn idle(_: idle::Context) -> ! {
        loop {}
    }
}
