//! Touch sense controller
use crate::gpio::gpiob::{PB4, PB5, PB6, PB7};
use crate::gpio::{OpenDrain, Output, PushPull};
use crate::rcc::Rcc;
use crate::stm32::TSC;

#[derive(Debug)]
pub enum Event {
    /// Max count error
    MaxCountError,
    /// End of acquisition
    EndOfAcquisition,
}

#[derive(Debug)]
pub enum Error {
    /// Max count error
    MaxCountError,
}

pub enum TscPrescaler {
    NotDivided = 0b000,
    Div2 = 0b001,
    Div4 = 0b010,
    Div8 = 0b011,
    Div16 = 0b100,
    Div32 = 0b101,
    Div64 = 0b110,
    Div128 = 0b111,
}

pub trait TscExt {
    fn tsc(self, rcc: &mut Rcc) -> Tsc;
}

impl TscExt for TSC {
    fn tsc(self, rcc: &mut Rcc) -> Tsc {
        Tsc::tsc(self, rcc)
    }
}

pub struct Tsc {
    rb: TSC,
}

impl Tsc {
    pub fn tsc(tsc: TSC, rcc: &mut Rcc) -> Self {
        rcc.rb.ahbenr.modify(|_, w| w.tscen().set_bit());
        rcc.rb.ahbrstr.modify(|_, w| w.tscrst().set_bit());
        rcc.rb.ahbrstr.modify(|_, w| w.tscrst().clear_bit());

        // clear interrupt & flags
        tsc.icr.write(|w| w.eoaic().set_bit().mceic().set_bit());

        Tsc { rb: tsc }
    }

    /// Enables an interrupt event
    pub fn listen(&mut self, event: Event) {
        match event {
            Event::EndOfAcquisition => {
                self.rb.ier.modify(|_, w| w.eoaie().set_bit());
            }
            Event::MaxCountError => {
                self.rb.ier.modify(|_, w| w.mceie().set_bit());
            }
        }
    }

    /// Disables an interrupt event
    pub fn unlisten(&self, event: Event) {
        match event {
            Event::EndOfAcquisition => {
                self.rb.ier.modify(|_, w| w.eoaie().clear_bit());
            }
            Event::MaxCountError => {
                self.rb.ier.modify(|_, w| w.mceie().clear_bit());
            }
        }
    }

    /// Releases the TSC peripheral and associated pins
    pub fn free(self) -> TSC {
        self.rb
    }
}
