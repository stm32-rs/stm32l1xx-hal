#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting;
extern crate panic_semihosting;
extern crate rtic;
extern crate stm32l1xx_hal as hal;

#[rtic::app(device = hal::stm32, peripherals = true)]
mod app {
    use cortex_m_semihosting::hprintln;
    use embedded_hal::digital::v2::OutputPin;
    use embedded_hal::digital::v2::ToggleableOutputPin;
    use hal::exti::TriggerEdge;
    use hal::gpio::gpiob::{PB6, PB7};
    use hal::gpio::{Output, PushPull};
    use hal::prelude::*;
    use hal::rcc::Config;
    use hal::stm32;
    use hal::timer::Timer;

    #[shared]
    struct Shared {
        delta: u32,
    }

    #[local]
    struct Local {
        timer: Timer<stm32::TIM2>,
        ticks_led: PB6<Output<PushPull>>,
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

        let ticks_led = gpiob.pb6.into_push_pull_output();
        let busy_led = gpiob.pb7.into_push_pull_output();
        let timer = timer;
        let exti = cx.device.EXTI;

        (
            Shared { delta: 0 },
            Local {
                timer,
                ticks_led,
                busy_led,
                exti,
            },
            init::Monotonics(),
        )
    }

    #[task(binds = TIM2, shared = [delta], local = [timer, ticks_led])]
    fn tim2_handler(mut cx: tim2_handler::Context) {
        cx.shared.delta.lock(|d| *d += 1);

        cx.local.ticks_led.toggle().unwrap();
        cx.local.timer.clear_irq();
    }

    #[task(binds = EXTI0, shared = [delta], local = [exti, busy_led])]
    fn exti0_handler(mut cx: exti0_handler::Context) {
        cx.local.busy_led.set_high().unwrap();
        hprintln!("Δ: {}", cx.shared.delta.lock(|d| *d)).unwrap();
        cx.local.busy_led.set_low().unwrap();

        cx.shared.delta.lock(|d| *d = 0);
        cx.local.exti.clear_irq(0);
    }
}
