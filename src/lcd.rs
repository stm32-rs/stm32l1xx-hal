//! LCD
use crate::gpio::gpioa::*;
use crate::gpio::gpiob::*;
use crate::gpio::gpioc::*;
use crate::gpio::gpiod::*;
use crate::gpio::{AltMode, Floating, Input};
use crate::rcc::Rcc;
use crate::stm32::LCD;

pub trait CommonPin {
    const MODE: u8;

    fn setup(&self);
}

impl CommonPin for PA8<Input<Floating>> {
    const MODE: u8 = 0b000;

    fn setup(&self) {
        self.set_alt_mode(AltMode::LCD);
    }
}

impl CommonPin for (PA8<Input<Floating>>, PA9<Input<Floating>>) {
    const MODE: u8 = 0b001;

    fn setup(&self) {
        self.0.set_alt_mode(AltMode::LCD);
        self.1.set_alt_mode(AltMode::LCD);
    }
}

impl CommonPin
    for (
        PA8<Input<Floating>>,
        PA9<Input<Floating>>,
        PA10<Input<Floating>>,
    )
{
    const MODE: u8 = 0b010;

    fn setup(&self) {
        self.0.set_alt_mode(AltMode::LCD);
        self.1.set_alt_mode(AltMode::LCD);
        self.2.set_alt_mode(AltMode::LCD);
    }
}

impl CommonPin
    for (
        PA8<Input<Floating>>,
        PA9<Input<Floating>>,
        PA10<Input<Floating>>,
        PB9<Input<Floating>>,
    )
{
    const MODE: u8 = 0b011;

    fn setup(&self) {
        self.0.set_alt_mode(AltMode::LCD);
        self.1.set_alt_mode(AltMode::LCD);
        self.2.set_alt_mode(AltMode::LCD);
        self.3.set_alt_mode(AltMode::LCD);
    }
}

impl CommonPin
    for (
        PA8<Input<Floating>>,
        PA9<Input<Floating>>,
        PA10<Input<Floating>>,
        PB9<Input<Floating>>,
        PC10<Input<Floating>>,
        PC11<Input<Floating>>,
        PC12<Input<Floating>>,
        PD2<Input<Floating>>,
    )
{
    const MODE: u8 = 0b100;

    fn setup(&self) {
        self.0.set_alt_mode(AltMode::LCD);
        self.1.set_alt_mode(AltMode::LCD);
        self.2.set_alt_mode(AltMode::LCD);
        self.3.set_alt_mode(AltMode::LCD);
        self.4.set_alt_mode(AltMode::LCD);
        self.5.set_alt_mode(AltMode::LCD);
        self.6.set_alt_mode(AltMode::LCD);
        self.7.set_alt_mode(AltMode::LCD);
    }
}

pub trait SegmentPin {
    fn setup(&self);
}

impl SegmentPin for PA1<Input<Floating>> {
    fn setup(&self) {
        self.set_alt_mode(AltMode::LCD);
    }
}

impl SegmentPin for (PA1<Input<Floating>>, PA2<Input<Floating>>) {
    fn setup(&self) {
        self.0.set_alt_mode(AltMode::LCD);
        self.1.set_alt_mode(AltMode::LCD);
    }
}

impl SegmentPin
    for (
        PA1<Input<Floating>>,
        PA2<Input<Floating>>,
        PA3<Input<Floating>>,
        PA6<Input<Floating>>,
    )
{
    fn setup(&self) {
        self.0.set_alt_mode(AltMode::LCD);
        self.1.set_alt_mode(AltMode::LCD);
        self.2.set_alt_mode(AltMode::LCD);
        self.3.set_alt_mode(AltMode::LCD);
    }
}

impl SegmentPin
    for (
        PA1<Input<Floating>>,
        PA2<Input<Floating>>,
        PA3<Input<Floating>>,
        PA6<Input<Floating>>,
        PA7<Input<Floating>>,
        PB0<Input<Floating>>,
        PB1<Input<Floating>>,
        PB3<Input<Floating>>,
    )
{
    fn setup(&self) {
        self.0.set_alt_mode(AltMode::LCD);
        self.1.set_alt_mode(AltMode::LCD);
        self.2.set_alt_mode(AltMode::LCD);
        self.3.set_alt_mode(AltMode::LCD);
        self.4.set_alt_mode(AltMode::LCD);
        self.5.set_alt_mode(AltMode::LCD);
        self.6.set_alt_mode(AltMode::LCD);
        self.7.set_alt_mode(AltMode::LCD);
    }
}

impl SegmentPin
    for (
        PA1<Input<Floating>>,
        PA2<Input<Floating>>,
        PA3<Input<Floating>>,
        PA6<Input<Floating>>,
        PA7<Input<Floating>>,
        PB0<Input<Floating>>,
        PB1<Input<Floating>>,
        PB3<Input<Floating>>,
        PB4<Input<Floating>>,
        PB5<Input<Floating>>,
        PB10<Input<Floating>>,
        PB11<Input<Floating>>,
        PB12<Input<Floating>>,
        PB13<Input<Floating>>,
        PB14<Input<Floating>>,
        PB15<Input<Floating>>,
        PB8<Input<Floating>>,
        PA15<Input<Floating>>,
        PC0<Input<Floating>>,
        PC1<Input<Floating>>,
        PC2<Input<Floating>>,
        PC3<Input<Floating>>,
        PC4<Input<Floating>>,
        PC5<Input<Floating>>,
        PC6<Input<Floating>>,
        PC7<Input<Floating>>,
        PC8<Input<Floating>>,
        PC9<Input<Floating>>,
        PC10<Input<Floating>>,
        PC11<Input<Floating>>,
        PC12<Input<Floating>>,
        PD2<Input<Floating>>,
    )
{
    fn setup(&self) {
        self.0.set_alt_mode(AltMode::LCD);
        self.1.set_alt_mode(AltMode::LCD);
        self.2.set_alt_mode(AltMode::LCD);
        self.3.set_alt_mode(AltMode::LCD);
        self.4.set_alt_mode(AltMode::LCD);
        self.5.set_alt_mode(AltMode::LCD);
        self.6.set_alt_mode(AltMode::LCD);
        self.7.set_alt_mode(AltMode::LCD);
        self.8.set_alt_mode(AltMode::LCD);
        self.9.set_alt_mode(AltMode::LCD);
        self.10.set_alt_mode(AltMode::LCD);
        self.11.set_alt_mode(AltMode::LCD);
        self.12.set_alt_mode(AltMode::LCD);
        self.13.set_alt_mode(AltMode::LCD);
        self.14.set_alt_mode(AltMode::LCD);
        self.15.set_alt_mode(AltMode::LCD);
        self.16.set_alt_mode(AltMode::LCD);
        self.17.set_alt_mode(AltMode::LCD);
        self.18.set_alt_mode(AltMode::LCD);
        self.19.set_alt_mode(AltMode::LCD);
        self.20.set_alt_mode(AltMode::LCD);
        self.21.set_alt_mode(AltMode::LCD);
        self.22.set_alt_mode(AltMode::LCD);
        self.23.set_alt_mode(AltMode::LCD);
        self.24.set_alt_mode(AltMode::LCD);
        self.25.set_alt_mode(AltMode::LCD);
        self.26.set_alt_mode(AltMode::LCD);
        self.27.set_alt_mode(AltMode::LCD);
        self.28.set_alt_mode(AltMode::LCD);
        self.29.set_alt_mode(AltMode::LCD);
        self.30.set_alt_mode(AltMode::LCD);
        self.31.set_alt_mode(AltMode::LCD);
    }
}

#[derive(Default)]
pub struct LcdConfig {
    clk_pre: u8,
    clk_div: u8,
    bias: u8,
}

impl LcdConfig {}

pub struct Lcd {
    rb: LCD,
}

impl Lcd {
    pub fn new<CPINS, SPINS>(
        lcd: LCD,
        comm_pins: CPINS,
        seg_pins: SPINS,
        cfg: LcdConfig,
        rcc: &mut Rcc,
    ) -> Self
    where
        CPINS: CommonPin,
        SPINS: SegmentPin,
    {
        // Enable MSI
        rcc.rb.cr.write(|w| w.msion().set_bit());
        while rcc.rb.cr.read().msirdy().bit_is_clear() {}

        // Enable LCD clock
        rcc.rb.apb1enr.modify(|_, w| unsafe { w.lcden().set_bit() });
        rcc.rb.apb1rstr.modify(|_, w| w.lcdrst().set_bit());
        rcc.rb.apb1rstr.modify(|_, w| w.lcdrst().clear_bit());

        comm_pins.setup();
        seg_pins.setup();

        lcd.cr
            .modify(|_, w| unsafe { w.bias().bits(0b_10).duty().bits(CPINS::MODE) });
        lcd.fcr
            .modify(|_, w| unsafe { w.ps().bits(2).div().bits(4).cc().bits(3) });

        Lcd { rb: lcd }
    }

    pub fn disable(&mut self) {
        self.rb.cr.modify(|_, w| w.lcden().clear_bit());
    }

    pub fn enable(&mut self) {
        self.rb.cr.modify(|_, w| w.lcden().set_bit());
    }

    pub fn write(&mut self, common: u8, data: u32) {
        match common {
            0 => self.rb.ram_com0.modify(|_, w| unsafe { w.bits(data) }),
            1 => self.rb.ram_com1.modify(|_, w| unsafe { w.bits(data) }),
            2 => self.rb.ram_com2.modify(|_, w| unsafe { w.bits(data) }),
            3 => self.rb.ram_com3.modify(|_, w| unsafe { w.bits(data) }),
            4 => self.rb.ram_com4.modify(|_, w| unsafe { w.bits(data) }),
            5 => self.rb.ram_com5.modify(|_, w| unsafe { w.bits(data) }),
            6 => self.rb.ram_com6.modify(|_, w| unsafe { w.bits(data) }),
            _ => return,
        }

        self.rb.sr.modify(|_, w| w.udr().set_bit());
    }
}

pub trait LcdExt {
    fn lcd<CPINS, SPINS>(
        self,
        comm_pins: CPINS,
        seg_pins: SPINS,
        cfg: LcdConfig,
        rcc: &mut Rcc,
    ) -> Lcd
    where
        CPINS: CommonPin,
        SPINS: SegmentPin;
}

impl LcdExt for LCD {
    fn lcd<CPINS, SPINS>(
        self,
        comm_pins: CPINS,
        seg_pins: SPINS,
        cfg: LcdConfig,
        rcc: &mut Rcc,
    ) -> Lcd
    where
        CPINS: CommonPin,
        SPINS: SegmentPin,
    {
        Lcd::new(self, comm_pins, seg_pins, cfg, rcc)
    }
}
