use core::marker::PhantomData;
use core::mem;

use crate::gpio::gpioa::{PA0, PA1, PA2, PA3, PA6, PA7};
use crate::gpio::gpiob::{PB0, PB1, PB6, PB7, PB8, PB9};
use crate::gpio::{Alternate, AF1, AF2, AF3};
use crate::rcc::Clocks;
use crate::stm32::RCC;
use crate::stm32::{TIM10, TIM11, TIM2, TIM3, TIM4, TIM5};
use crate::time::Hertz;
use cast::{u16, u32};
use hal;

pub struct C1;
pub struct C2;
pub struct C3;
pub struct C4;

pub trait Pins<TIM> {
    type Channels;
}

pub trait PwmExt: Sized {
    fn pwm<PINS, T>(self, _: PINS, frequency: T, clocks: Clocks) -> PINS::Channels
    where
        PINS: Pins<Self>,
        T: Into<Hertz>;
}

pub struct Pwm<TIM, CHANNEL> {
    _channel: PhantomData<CHANNEL>,
    _tim: PhantomData<TIM>,
}

macro_rules! channels {
    ($TIMX:ident, $c1:ty) => {
        impl Pins<$TIMX> for $c1 {
            type Channels = Pwm<$TIMX, C1>;
        }

        impl hal::PwmPin for Pwm<$TIMX, C1> {
            type Duty = u16;

            fn disable(&mut self) {
                unsafe {
                    (*$TIMX::ptr()).ccer.modify(|_, w| w.cc1e().clear_bit());
                }
            }

            fn enable(&mut self) {
                unsafe {
                    let tim = &*$TIMX::ptr();
                    tim.ccmr1_output
                        .modify(|_, w| w.oc1pe().set_bit().oc1m().bits(6));
                    tim.ccer.modify(|_, w| w.cc1e().set_bit());
                }
            }

            fn get_duty(&self) -> u16 {
                unsafe { (*$TIMX::ptr()).ccr1.read().ccr1().bits() }
            }

            fn get_max_duty(&self) -> u16 {
                unsafe { (*$TIMX::ptr()).arr.read().arr().bits() }
            }

            fn set_duty(&mut self, duty: u16) {
                unsafe { (*$TIMX::ptr()).ccr1.write(|w| w.ccr1().bits(duty)) }
            }
        }
    };
    ($TIMX:ident, $c1:ty, $c2:ty, $c3:ty, $c4:ty) => {
        channels!($TIMX, $c1);

        impl Pins<$TIMX> for $c2 {
            type Channels = Pwm<$TIMX, C2>;
        }

        impl Pins<$TIMX> for $c3 {
            type Channels = Pwm<$TIMX, C3>;
        }

        impl Pins<$TIMX> for $c4 {
            type Channels = Pwm<$TIMX, C4>;
        }

        impl Pins<$TIMX> for ($c1, $c2) {
            type Channels = (Pwm<$TIMX, C1>, Pwm<$TIMX, C2>);
        }

        impl Pins<$TIMX> for ($c1, $c2, $c3, $c4) {
            type Channels = (
                Pwm<$TIMX, C1>,
                Pwm<$TIMX, C2>,
                Pwm<$TIMX, C3>,
                Pwm<$TIMX, C4>,
            );
        }

        impl hal::PwmPin for Pwm<$TIMX, C2> {
            type Duty = u16;

            fn disable(&mut self) {
                unsafe {
                    (*$TIMX::ptr()).ccer.modify(|_, w| w.cc2e().clear_bit());
                }
            }

            fn enable(&mut self) {
                unsafe {
                    let tim = &*$TIMX::ptr();
                    tim.ccmr1_output
                        .modify(|_, w| w.oc2pe().set_bit().oc2m().bits(6));
                    tim.ccer.modify(|_, w| w.cc2e().set_bit());
                }
            }

            fn get_duty(&self) -> u16 {
                unsafe { (*$TIMX::ptr()).ccr2.read().ccr2().bits() }
            }

            fn get_max_duty(&self) -> u16 {
                unsafe { (*$TIMX::ptr()).arr.read().arr().bits() }
            }

            fn set_duty(&mut self, duty: u16) {
                unsafe { (*$TIMX::ptr()).ccr2.write(|w| w.ccr2().bits(duty)) }
            }
        }

        impl hal::PwmPin for Pwm<$TIMX, C3> {
            type Duty = u16;

            fn disable(&mut self) {
                unsafe {
                    (*$TIMX::ptr()).ccer.modify(|_, w| w.cc3e().clear_bit());
                }
            }

            fn enable(&mut self) {
                unsafe {
                    let tim = &*$TIMX::ptr();
                    tim.ccmr2_output
                        .modify(|_, w| w.oc3pe().set_bit().oc3m().bits(6));
                    tim.ccer.modify(|_, w| w.cc3e().set_bit());
                }
            }

            fn get_duty(&self) -> u16 {
                unsafe { (*$TIMX::ptr()).ccr3.read().ccr3().bits() }
            }

            fn get_max_duty(&self) -> u16 {
                unsafe { (*$TIMX::ptr()).arr.read().arr().bits() }
            }

            fn set_duty(&mut self, duty: u16) {
                unsafe { (*$TIMX::ptr()).ccr3.write(|w| w.ccr3().bits(duty)) }
            }
        }

        impl hal::PwmPin for Pwm<$TIMX, C4> {
            type Duty = u16;

            fn disable(&mut self) {
                unsafe {
                    (*$TIMX::ptr()).ccer.modify(|_, w| w.cc4e().clear_bit());
                }
            }

            fn enable(&mut self) {
                unsafe {
                    let tim = &*$TIMX::ptr();
                    tim.ccmr2_output
                        .modify(|_, w| w.oc4pe().set_bit().oc4m().bits(6));
                    tim.ccer.modify(|_, w| w.cc4e().set_bit());
                }
            }

            fn get_duty(&self) -> u16 {
                unsafe { (*$TIMX::ptr()).ccr4.read().ccr4().bits() }
            }

            fn get_max_duty(&self) -> u16 {
                unsafe { (*$TIMX::ptr()).arr.read().arr().bits() }
            }

            fn set_duty(&mut self, duty: u16) {
                unsafe { (*$TIMX::ptr()).ccr4.write(|w| w.ccr4().bits(duty)) }
            }
        }
    };
}

macro_rules! timers {
    ($($TIMX:ident: ($apb_clk:ident, $apbXenr:ident, $apbXrstr:ident, $timX:ident, $timXen:ident, $timXrst:ident),)+) => {
        $(
            impl PwmExt for $TIMX {
                fn pwm<PINS, T>(
                    self,
                    _pins: PINS,
                    freq: T,
                    clocks: Clocks,
                ) -> PINS::Channels
                where
                    PINS: Pins<Self>,
                    T: Into<Hertz>,
                {
                    //TODO: pin remap
                    $timX(self, _pins, freq.into(), clocks)
                }
            }

            fn $timX<PINS>(
                tim: $TIMX,
                _pins: PINS,
                freq: Hertz,
                clocks: Clocks,
            ) -> PINS::Channels
            where
                PINS: Pins<$TIMX>,
            {
                let rcc = unsafe { &(*RCC::ptr()) };

                rcc.$apbXenr.modify(|_, w| w.$timXen().set_bit());
                rcc.$apbXrstr.modify(|_, w| w.$timXrst().set_bit());
                rcc.$apbXrstr.modify(|_, w| w.$timXrst().clear_bit());

                let clk = clocks.$apb_clk().0;
                let freq = freq.0;
                let ticks = clk / freq;
                let psc = u16((ticks - 1) / (1 << 16)).unwrap();
                let arr = u16(ticks / u32(psc + 1)).unwrap();

                tim.psc.write(|w| unsafe { w.psc().bits(psc) });

                tim.arr.write(|w| unsafe { w.arr().bits(arr) });

                tim.cr1.write(|w| w.cen().set_bit());

                unsafe { mem::uninitialized() }
            }
        )+
    }
}

channels!(
    TIM2,
    PA0<Alternate<AF1>>,
    PA1<Alternate<AF1>>,
    PA2<Alternate<AF1>>,
    PA3<Alternate<AF1>>
);
channels!(
    TIM3,
    PA6<Alternate<AF2>>,
    PA7<Alternate<AF2>>,
    PB0<Alternate<AF2>>,
    PB1<Alternate<AF2>>
);
channels!(
    TIM4,
    PB6<Alternate<AF2>>,
    PB7<Alternate<AF2>>,
    PB8<Alternate<AF2>>,
    PB9<Alternate<AF2>>
);
channels!(
    TIM5,
    PA0<Alternate<AF2>>,
    PA1<Alternate<AF2>>,
    PA2<Alternate<AF2>>,
    PA3<Alternate<AF2>>
);
channels!(TIM10, PA6<Alternate<AF3>>);
channels!(TIM11, PA7<Alternate<AF3>>);

timers! {
    TIM2: (apb1_clk, apb1enr, apb1rstr, tim2, tim2en, tim2rst),
    TIM3: (apb1_clk, apb1enr, apb1rstr, tim3, tim3en, tim3rst),
    TIM4: (apb1_clk, apb1enr, apb1rstr, tim4, tim4en, tim4rst),
    TIM5: (apb1_clk, apb1enr, apb1rstr, tim5, tim5en, tim5rst),
    TIM10: (apb2_clk, apb2enr, apb2rstr, tim10, tim10en, tm10rst),
    TIM11: (apb2_clk, apb2enr, apb2rstr, tim11, tim11en, tm11rst),
}
