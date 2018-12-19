//! DAC
use cast::u32;
use core::mem;

use stm32::{RCC, DAC};
use gpio::gpioa::{PA4, PA5};
use gpio::{Input, Floating};

pub trait DacExt {
    fn dac<PINS>(self, pins: PINS) -> PINS::Output
    where
        PINS: Pins<DAC>;
}

impl DacExt for DAC {
    fn dac<PINS>(self, pins: PINS) -> PINS::Output
    where
        PINS: Pins<DAC>,
    {
        dac(self, pins)
    }
}

pub struct C1;
pub struct C2;

pub trait DacOut {
    fn enable(&mut self);
    fn set(&mut self, val: u16);
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

pub fn dac<PINS>(_dac: DAC, _pins: PINS) -> PINS::Output
where
    PINS: Pins<DAC>,
{
    // NOTE(unsafe) This executes only during initialisation
    let rcc = unsafe { &(*RCC::ptr()) };

    // Enable DAC clocks
    rcc.apb1enr.modify(|_, w| w.dacen().set_bit());

    // Reset DAC
    rcc.apb1rstr.modify(|_, w| w.dacrst().set_bit());
    rcc.apb1rstr.modify(|_, w| w.dacrst().clear_bit());

    unsafe { mem::uninitialized() }
}

macro_rules! dac {
    ($CX:ident, $en:ident, $dhrx:ident) => {
        impl DacOut for $CX {
            fn enable(&mut self) {
                unsafe {
                    (*DAC::ptr()).cr.modify(|_, w| w.$en().set_bit());
                }
            }
            
            fn set(&mut self, val: u16) {
                unsafe {
                    (*DAC::ptr()).$dhrx.modify(|_, w| w.bits(u32(val)));
                }
            }
        }
    };
}

dac!(C1, en1, dhr12l1);
dac!(C2, en2, dhr12l2);
