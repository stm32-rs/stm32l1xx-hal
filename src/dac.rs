//! DAC
use cast::u32;
use core::mem;

use crate::gpio::gpioa::{PA4, PA5};
use crate::gpio::{Floating, Input};
use crate::rcc::Rcc;
use crate::stm32::DAC;

pub struct C1;
pub struct C2;

pub trait DacOut<V> {
    fn set_value(&mut self, val: V);
    fn get_value(&mut self) -> V;
}

pub trait DacPin {
    fn enable(&mut self);
}

pub trait Pins<DAC> {
    type Output;
}

impl Pins<DAC> for PA4<Input<Floating>> {
    type Output = C1;
}

impl Pins<DAC> for PA5<Input<Floating>> {
    type Output = C2;
}

impl Pins<DAC> for (PA4<Input<Floating>>, PA5<Input<Floating>>) {
    type Output = (C1, C2);
}

pub fn dac<PINS>(_dac: DAC, _pins: PINS, rcc: &mut Rcc) -> PINS::Output
where
    PINS: Pins<DAC>,
{
    // Enable DAC clocks
    rcc.rb.apb1enr.modify(|_, w| w.dacen().set_bit());

    // Reset DAC
    rcc.rb.apb1rstr.modify(|_, w| w.dacrst().set_bit());
    rcc.rb.apb1rstr.modify(|_, w| w.dacrst().clear_bit());

    unsafe { mem::MaybeUninit::uninit().assume_init() }
}

macro_rules! dac {
    ($CX:ident, $en:ident, $dhrx:ident, $daccxdhr:ident) => {
        impl DacPin for $CX {
            fn enable(&mut self) {
                unsafe {
                    (*DAC::ptr()).cr.modify(|_, w| w.$en().set_bit());
                }
            }
        }

        impl DacOut<u16> for $CX {
            fn set_value(&mut self, val: u16) {
                unsafe {
                    (*DAC::ptr()).$dhrx.modify(|_, w| w.bits(u32(val)));
                }
            }

            fn get_value(&mut self) -> u16 {
                unsafe { (*DAC::ptr()).$dhrx.read().$daccxdhr().bits() }
            }
        }

        impl DacOut<u8> for $CX {
            fn set_value(&mut self, val: u8) {
                unsafe {
                    (*DAC::ptr()).$dhrx.modify(|_, w| w.bits(u32(val)));
                }
            }

            fn get_value(&mut self) -> u8 {
                unsafe { (*DAC::ptr()).$dhrx.read().$daccxdhr().bits() as u8 }
            }
        }
    };
}

pub trait DacExt {
    fn dac<PINS>(self, pins: PINS, rcc: &mut Rcc) -> PINS::Output
    where
        PINS: Pins<DAC>;
}

impl DacExt for DAC {
    fn dac<PINS>(self, pins: PINS, rcc: &mut Rcc) -> PINS::Output
    where
        PINS: Pins<DAC>,
    {
        dac(self, pins, rcc)
    }
}

dac!(C1, en1, dhr12r1, dacc1dhr);
dac!(C2, en2, dhr12r2, dacc2dhr);
