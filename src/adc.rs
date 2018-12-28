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
}

pub trait AdcChannel {
    fn setup(&mut self, adc: &mut Adc);
}

macro_rules! adc_pins {
    ($($Chan:ty: ($pin:ty, $bank_b:tt, $chan:expr, $smprx:ident)),+ $(,)*) => {
        $(
            impl Channel<Adc> for $pin {
                type ID = u8;

                fn channel() -> u8 { $chan }
            }

            impl AdcChannel for $pin {
                fn setup(&mut self, adc: &mut Adc) {
                    adc.rb.cr2.modify(|_, w| w.adc_cfg().bit($bank_b));
                    adc.rb.$smprx.modify(|r, w| unsafe {
                        const OFFSET: u8 = 3 * $chan % 10;
                        let mut bits = r.smp().bits() as u32;
                        bits &= !(0xfff << OFFSET);
                        bits |= (adc.sample_time as u32) << OFFSET;
                        w.bits(bits)
                    });
                    adc.rb.sqr5.write(|w| unsafe { w.sq1().bits($chan) });
                }
            }
        )+
    };
}

adc_pins! {
    Channel0: (gpioa::PA0<Analog>, false, 0_u8, smpr3),
    Channel1: (gpioa::PA1<Analog>, false, 1_u8, smpr3),
    Channel2: (gpioa::PA2<Analog>, false, 2_u8, smpr3),
    Channel3: (gpioa::PA3<Analog>, false, 3_u8, smpr3),
    Channel4: (gpioa::PA4<Analog>, false, 4_u8, smpr3),
    Channel5: (gpioa::PA5<Analog>, false, 5_u8, smpr3),
    Channel6: (gpioa::PA6<Analog>, false, 6_u8, smpr3),
    Channel7: (gpioa::PA7<Analog>, false, 7_u8, smpr3),
    Channel8: (gpiob::PB0<Analog>, false, 8_u8, smpr3),
    Channel9: (gpiob::PB1<Analog>, false, 9_u8, smpr3),
    Channel10: (gpioc::PC0<Analog>, false, 10_u8, smpr2),
    Channel11: (gpioc::PC1<Analog>, false, 11_u8, smpr2),
    Channel12: (gpioc::PC2<Analog>, false, 12_u8, smpr2),
    Channel13: (gpioc::PC3<Analog>, false, 13_u8, smpr2),
    Channel14: (gpioc::PC4<Analog>, false, 14_u8, smpr2),
    Channel15: (gpioc::PC5<Analog>, false, 15_u8, smpr2),
    Channel18: (gpiob::PB12<Analog>, false, 18_u8, smpr2),
    Channel19: (gpiob::PB13<Analog>, false, 19_u8, smpr2),
    Channel20: (gpiob::PB14<Analog>, false, 20_u8, smpr1),
    Channel21: (gpiob::PB15<Analog>, false, 21_u8, smpr1),
}

#[cfg(not(feature = "stm32l100"))]
adc_pins! {
    Channel22: (gpioe::PE7<Analog>, false, 22_u8, smpr1),
    Channel23: (gpioe::PE8<Analog>, false, 23_u8, smpr1),
    Channel24: (gpioe::PE9<Analog>, false, 24_u8, smpr1),
    Channel25: (gpioe::PE10<Analog>, false, 25_u8, smpr1),
    Channel27: (gpiof::PF6<Analog>, false, 27_u8, smpr1),
    Channel28: (gpiof::PF7<Analog>, false, 28_u8, smpr1),
    Channel29: (gpiof::PF8<Analog>, false, 29_u8, smpr1),
    Channel30: (gpiof::PF9<Analog>, false, 30_u8, smpr1),
    Channel31: (gpiof::PF10<Analog>, false, 31_u8, smpr1),
    Channel0b: (gpiob::PB2<Analog>, true, 0_u8, smpr3),
    Channel1b: (gpiof::PF11<Analog>, true, 1_u8, smpr3),
    Channel2b: (gpiof::PF12<Analog>, true, 2_u8, smpr3),
    Channel3b: (gpiof::PF13<Analog>, true, 3_u8, smpr3),
    Channel6b: (gpiof::PF14<Analog>, true, 6_u8, smpr3),
    Channel7b: (gpiof::PF15<Analog>, true, 7_u8, smpr3),
    Channel8b: (gpiog::PG0<Analog>, true, 8_u8, smpr3),
    Channel9b: (gpiog::PG1<Analog>, true, 9_u8, smpr3),
    Channel10b: (gpiog::PG2<Analog>, true, 10_u8, smpr2),
    Channel11b: (gpiog::PG3<Analog>, true, 11_u8, smpr2),
    Channel12b: (gpiog::PG4<Analog>, true, 12_u8, smpr2),
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

impl Channel<Adc> for VTemp {
    type ID = u8;

    fn channel() -> u8 {
        16
    }
}

impl Channel<Adc> for VRef {
    type ID = u8;

    fn channel() -> u8 {
        17
    }
}

impl<WORD, PIN> OneShot<Adc, WORD, PIN> for Adc
where
    WORD: From<u16>,
    PIN: AdcChannel + Channel<Adc, ID = u8>,
{
    type Error = ();

    fn read(&mut self, pin: &mut PIN) -> nb::Result<WORD, Self::Error> {
        self.power_up();
        pin.setup(self);

        self.rb
            .cr1
            .modify(|_, w| unsafe { w.res().bits(self.precision as u8) });
        self.rb
            .cr2
            .modify(|_, w| w.align().bit(self.align == Align::Left).swstart().set_bit());
        while self.rb.sr.read().eoc().bit_is_clear() {}

        let res = self.rb.dr.read().bits() as u16;
        let val = if self.align == Align::Left && self.precision == Precision::B_6 {
            res << 8
        } else {
            res
        };

        self.power_down();
        Ok(val.into())
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
