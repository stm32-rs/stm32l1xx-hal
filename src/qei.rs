//! Quadrature Encoder Interface
use crate::gpio::gpioa::{PA0, PA1, PA6, PA7};
use crate::gpio::gpiob::{PB6, PB7};
use crate::gpio::{AltMode, Floating, Input};
use crate::hal::{self, Direction};
use crate::rcc::Rcc;
use crate::stm32::{TIM2, TIM3, TIM4, TIM5};
use core::u16;

pub trait Pins<TIM> {
    fn setup(&self);
}

impl Pins<TIM2> for (PA0<Input<Floating>>, PA1<Input<Floating>>) {
    fn setup(&self) {
        self.0.set_alt_mode(AltMode::TIM2);
        self.1.set_alt_mode(AltMode::TIM2);
    }
}

impl Pins<TIM3> for (PA6<Input<Floating>>, PA7<Input<Floating>>) {
    fn setup(&self) {
        self.0.set_alt_mode(AltMode::TIM3_5);
        self.1.set_alt_mode(AltMode::TIM3_5);
    }
}

impl Pins<TIM4> for (PB6<Input<Floating>>, PB7<Input<Floating>>) {
    fn setup(&self) {
        self.0.set_alt_mode(AltMode::TIM3_5);
        self.1.set_alt_mode(AltMode::TIM3_5);
    }
}

impl Pins<TIM5> for (PA0<Input<Floating>>, PA1<Input<Floating>>) {
    fn setup(&self) {
        self.0.set_alt_mode(AltMode::TIM3_5);
        self.1.set_alt_mode(AltMode::TIM3_5);
    }
}

pub struct Qei<TIM, PINS> {
    tim: TIM,
    pins: PINS,
}

pub trait QeiExt<TIM, PINS>
where
    PINS: Pins<TIM>,
{
    fn qei(self, pins: PINS, rcc: &mut Rcc) -> Qei<TIM, PINS>;
}

macro_rules! hal {
    ($($TIMX:ident: ($timX:ident, $timXen:ident, $timXrst:ident),)+) => {
        $(
            impl<PINS> Qei<$TIMX, PINS> where PINS: Pins<$TIMX> {
                fn $timX(tim: $TIMX, pins: PINS, rcc: &mut Rcc) -> Self {
                    pins.setup();
                    // enable and reset peripheral to a clean slate state
                    rcc.rb.apb1enr.modify(|_, w| w.$timXen().set_bit());
                    rcc.rb.apb1rstr.modify(|_, w| w.$timXrst().set_bit());
                    rcc.rb.apb1rstr.modify(|_, w| w.$timXrst().clear_bit());

                    // Configure TxC1 and TxC2 as captures
                    tim.ccmr1_output.write(|w| unsafe {
                        w.cc1s().bits(0b01).cc2s().set_bit()
                    });

                    // Enable and configure to capture on rising edge
                    tim.ccer.write(|w| {
                        w.cc1e()
                            .set_bit()
                            .cc2e()
                            .set_bit()
                            .cc1p()
                            .clear_bit()
                            .cc2p()
                            .clear_bit()
                            .cc1np()
                            .clear_bit()
                            .cc2np()
                            .clear_bit()
                    });

                    // Encoder mode, count up/down on both TI1FP1 and TI2FP2
                    tim.smcr.write(|w| unsafe { w.sms().bits(0b011) });

                    tim.arr.write(|w| w.arr().bits(u16::MAX));
                    tim.cr1.write(|w| w.cen().enabled());

                    Qei { tim, pins }
                }

                pub fn release(self) -> ($TIMX, PINS) {
                    (self.tim, self.pins)
                }
            }

            impl<PINS> hal::Qei for Qei<$TIMX, PINS> {
                type Count = u16;

                fn count(&self) -> u16 {
                    self.tim.cnt.read().cnt().bits()
                }

                fn direction(&self) -> Direction {
                    if self.tim.cr1.read().dir().bit_is_clear() {
                        hal::Direction::Upcounting
                    } else {
                        hal::Direction::Downcounting
                    }
                }
            }

            impl<PINS> QeiExt<$TIMX, PINS> for $TIMX where PINS: Pins<$TIMX> {
                fn qei(self, pins: PINS, rcc: &mut Rcc) -> Qei<$TIMX, PINS> {
                    Qei::$timX(self, pins, rcc)
                }
            }
        )+
    }
}

hal! {
    TIM2: (tim2, tim2en, tim2rst),
    TIM3: (tim3, tim3en, tim3rst),
    TIM4: (tim4, tim4en, tim4rst),
    TIM5: (tim5, tim5en, tim5rst),
}
