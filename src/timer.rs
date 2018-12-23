//! Timers
use cast::{u16, u32};
use cortex_m::peripheral::syst::SystClkSource;
use cortex_m::peripheral::SYST;
use hal::timer::{CountDown, Periodic};
use nb;
use void::Void;

use crate::rcc::Clocks;
use crate::stm32::{RCC, TIM2, TIM3, TIM4, TIM5, TIM6, TIM7, TIM9};
use crate::time::Hertz;

pub trait TimerExt<TIM> {
    fn timer<T>(self, timeout: T, clocks: Clocks) -> Timer<TIM>
    where
        T: Into<Hertz>;
}

/// Hardware timers
pub struct Timer<TIM> {
    clocks: Clocks,
    tim: TIM,
}

impl Timer<SYST> {
    /// Configures the SYST clock as a periodic count down timer
    pub fn syst<T>(mut syst: SYST, timeout: T, clocks: Clocks) -> Self
    where
        T: Into<Hertz>,
    {
        syst.set_clock_source(SystClkSource::Core);
        let mut timer = Timer { tim: syst, clocks };
        timer.start(timeout);
        timer
    }

    /// Starts listening
    pub fn listen(&mut self) {
        self.tim.enable_interrupt()
    }

    /// Stops listening
    pub fn unlisten(&mut self) {
        self.tim.disable_interrupt()
    }
}

impl CountDown for Timer<SYST> {
    type Time = Hertz;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Hertz>,
    {
        let rvr = self.clocks.sys_clk().0 / timeout.into().0 - 1;
        assert!(rvr < (1 << 24));

        self.tim.set_reload(rvr);
        self.tim.clear_current();
        self.tim.enable_counter();
    }

    fn wait(&mut self) -> nb::Result<(), Void> {
        if self.tim.has_wrapped() {
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl TimerExt<SYST> for SYST {
    fn timer<T>(self, timeout: T, clocks: Clocks) -> Timer<SYST>
    where
        T: Into<Hertz>,
    {
        Timer::syst(self, timeout, clocks)
    }
}

impl Periodic for Timer<SYST> {}

macro_rules! timers {
    ($($TIM:ident: ($tim:ident, $timXen:ident, $timXrst:ident, $apbenr:ident, $apbrstr:ident, $timclk:ident),)+) => {
        $(
            impl TimerExt<$TIM> for $TIM {
                fn timer<T>(self, timeout: T, clocks: Clocks) -> Timer<$TIM>
                    where
                        T: Into<Hertz>,
                {
                    Timer::$tim(self, timeout, clocks)
                }
            }

            impl Timer<$TIM> {
                /// Configures a TIM peripheral as a periodic count down timer
                pub fn $tim<T>(tim: $TIM, timeout: T, clocks: Clocks) -> Self
                where
                    T: Into<Hertz>,
                {
                    let rcc = unsafe { &(*RCC::ptr()) };
                    rcc.$apbenr.modify(|_, w| w.$timXen().set_bit());
                    rcc.$apbrstr.modify(|_, w| w.$timXrst().set_bit());
                    rcc.$apbrstr.modify(|_, w| w.$timXrst().clear_bit());

                    let mut timer = Timer {
                        clocks,
                        tim,
                    };
                    timer.start(timeout);
                    timer
                }

                /// Starts listening
                pub fn listen(&mut self) {
                    self.tim.dier.write(|w| w.uie().set_bit());
                }

                /// Stops listening
                pub fn unlisten(&mut self) {
                    self.tim.dier.write(|w| w.uie().clear_bit());
                }

                /// Clears interrupt flag
                pub fn clear_irq(&mut self) {
                    self.tim.sr.write(|w| w.uif().clear_bit());
                }

                /// Releases the TIM peripheral
                pub fn release(self) -> $TIM {
                    self.tim.cr1.modify(|_, w| w.cen().clear_bit());
                    self.tim
                }
            }

            impl CountDown for Timer<$TIM> {
                type Time = Hertz;

                fn start<T>(&mut self, timeout: T)
                where
                    T: Into<Hertz>,
                {
                    // pause
                    self.tim.cr1.modify(|_, w| w.cen().clear_bit());
                    // reset counter
                    self.tim.cnt.reset();

                    let freq = timeout.into().0;
                    let ticks = self.clocks.$timclk().0 / freq;
                    let psc = u16((ticks - 1) / (1 << 16)).unwrap();

                    self.tim.psc.write(|w| unsafe { w.psc().bits(psc) });
                    self.tim.arr.write(|w| unsafe { w.bits(ticks / u32(psc + 1)) });

                    self.tim.cr1.modify(|_, w| w.urs().set_bit());
                    self.tim.cr1.modify(|_, w| w.cen().set_bit());
                }

                fn wait(&mut self) -> nb::Result<(), Void> {
                    if self.tim.sr.read().uif().bit_is_clear() {
                        Err(nb::Error::WouldBlock)
                    } else {
                        self.tim.sr.modify(|_, w| w.uif().clear_bit());
                        Ok(())
                    }
                }
            }

            impl Periodic for Timer<$TIM> {}
        )+
    }
}

timers! {
    TIM2: (tim2, tim2en, tim2rst, apb1enr, apb1rstr, apb1_tim_clk),
    TIM3: (tim3, tim3en, tim3rst, apb1enr, apb1rstr, apb1_tim_clk),
    TIM4: (tim4, tim4en, tim4rst, apb1enr, apb1rstr, apb1_tim_clk),
    TIM5: (tim5, tim5en, tim5rst, apb1enr, apb1rstr, apb1_tim_clk),
    TIM6: (tim6, tim6en, tim6rst, apb1enr, apb1rstr, apb1_tim_clk),
    TIM7: (tim7, tim7en, tim7rst, apb1enr, apb1rstr, apb1_tim_clk),
    TIM9: (tim9, tim9en, tim9rst, apb2enr, apb2rstr, apb2_tim_clk),
}
