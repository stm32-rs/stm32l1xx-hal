//! # Analog to Digital converter
use crate::gpio::*;
use crate::rcc::Rcc;
use crate::stm32::ADC;
use hal::adc::{Channel, OneShot};

/// Analog to Digital converter interface
pub struct Adc {
    rb: ADC,
    sample_time: SampleTime,
    align: Align,
    precision: Precision,
}

/// Internal temperature sensor (ADC Channel 16)
pub struct VTemp;

/// Internal voltage reference (ADC Channel 17)
pub struct VRef;

/// ADC Result Alignment
#[derive(PartialEq)]
pub enum Align {
    /// Right aligned results (least significant bits)
    ///
    /// Results in all precisions returning values from 0-(2^bits-1) in
    /// steps of 1.
    Right,
    /// Left aligned results (most significant bits)
    ///
    /// Results in all precisions returning a value in the range 0-65535.
    /// Depending on the precision the result will step by larger or smaller
    /// amounts.
    Left,
}

/// ADC Sampling Precision
#[derive(Copy, Clone, PartialEq)]
pub enum Precision {
    /// 12 bit precision
    B_12 = 0b00,
    /// 10 bit precision
    B_10 = 0b01,
    /// 8 bit precision
    B_8 = 0b10,
    /// 6 bit precision
    B_6 = 0b11,
}

/// ADC Sampling time
#[derive(Copy, Clone, PartialEq)]
pub enum SampleTime {
    T_4 = 0b000,
    T_9 = 0b001,
    T_16 = 0b010,
    T_24 = 0b011,
    T_48 = 0b100,
    T_96 = 0b101,
    T_192 = 0b110,
    T_384 = 0b111,
}

impl Adc {
    pub fn new(adc: ADC, rcc: &mut Rcc) -> Self {
        // Enable HSI
        rcc.rb.cr.write(|w| w.hsion().set_bit());
        while rcc.rb.cr.read().hsirdy().bit_is_clear() {}

        // Enable ADC clocks
        rcc.rb.apb2enr.modify(|_, w| w.adc1en().set_bit());

        Self {
            rb: adc,
            sample_time: SampleTime::T_4,
            align: Align::Right,
            precision: Precision::B_12,
        }
    }

    /// Set the Adc sampling time
    pub fn set_sample_time(&mut self, t_samp: SampleTime) {
        self.sample_time = t_samp;
    }

    /// Set the Adc result alignment
    pub fn set_align(&mut self, align: Align) {
        self.align = align;
    }

    /// Set the Adc precision
    pub fn set_precision(&mut self, precision: Precision) {
        self.precision = precision;
    }

    fn power_up(&mut self) {
        if self.rb.sr.read().adons().bit_is_set() {
            self.power_down();
        }
        self.rb.cr2.modify(|_, w| w.adon().set_bit());
        while self.rb.sr.read().adons().bit_is_clear() {}
    }

    fn power_down(&mut self) {
        self.rb.cr2.modify(|_, w| w.adon().clear_bit());
    }

    fn convert(&mut self, chan: u8) -> u16 {
        // self.rb
        //     .smprx
        //     .write(|w| unsafe { w.bits(self.sample_time as u8) });
        self.rb
            .cr1
            .modify(|_, w| unsafe { w.res().bits(self.precision as u8) });
        self.rb.sqr5.write(|w| unsafe { w.sq1().bits(chan) });
        self.rb
            .cr2
            .modify(|_, w| w.align().bit(self.align == Align::Left).swstart().set_bit());
        while self.rb.sr.read().eoc().bit_is_clear() {}

        let res = self.rb.dr.read().bits() as u16;
        if self.align == Align::Left && self.precision == Precision::B_6 {
            res << 8
        } else {
            res
        }
    }
}

macro_rules! adc_pins {
    ($($pin:ty => $chan:expr),+ $(,)*) => {
        $(
            impl Channel<Adc> for $pin {
                type ID = u8;

                fn channel() -> u8 { $chan }
            }
        )+
    };
}

impl VTemp {
    /// Init a new VTemp
    pub fn new() -> Self {
        VTemp {}
    }

    /// Enable the internal temperature sense
    pub fn enable(&mut self, adc: &mut Adc) {
        adc.rb.ccr.modify(|_, w| w.tsvrefe().set_bit());
    }

    /// Disable the internal temperature sense.
    pub fn disable(&mut self, adc: &mut Adc) {
        adc.rb.ccr.modify(|_, w| w.tsvrefe().clear_bit());
    }
}

impl VRef {
    /// Init a new VRef
    pub fn new() -> Self {
        VRef {}
    }

    /// Enable the internal voltage reference, remember to disable when not in use.
    pub fn enable(&mut self, adc: &mut Adc) {
        adc.rb.ccr.modify(|_, w| w.tsvrefe().set_bit());
    }

    /// Disable the internal reference voltage.
    pub fn disable(&mut self, adc: &mut Adc) {
        adc.rb.ccr.modify(|_, w| w.tsvrefe().clear_bit());
    }
}

adc_pins!(
    gpioa::PA0<Analog> => 0_u8,
    gpioa::PA1<Analog> => 1_u8,
    VTemp => 16_u8,
    VRef  => 17_u8,
);

impl<WORD, PIN> OneShot<Adc, WORD, PIN> for Adc
where
    WORD: From<u16>,
    PIN: Channel<Adc, ID = u8>,
{
    type Error = ();

    fn read(&mut self, _pin: &mut PIN) -> nb::Result<WORD, Self::Error> {
        self.power_up();
        let res = self.convert(PIN::channel());
        self.power_down();
        Ok(res.into())
    }
}

pub trait AdcExt {
    fn adc(self, rcc: &mut Rcc) -> Adc;
}

impl AdcExt for ADC {
    fn adc(self, rcc: &mut Rcc) -> Adc {
        Adc::new(self, rcc)
    }
}
