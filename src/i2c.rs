//! I2C
use hal::blocking::i2c::{Read, Write, WriteRead};

use crate::gpio::gpiob::{PB10, PB11, PB6, PB7, PB8, PB9};
use crate::gpio::{AltMode, OpenDrain, Output};
use crate::prelude::*;
use crate::rcc::Rcc;
use crate::stm32::{I2C1, I2C2};
use crate::time::Hertz;
use cortex_m::peripheral::DWT;

/// I2C abstraction
pub struct I2c<I2C, PINS> {
    i2c: I2C,
    pins: PINS,
    speed: Hertz,
    clock: u32,
}

pub trait Pins<I2c> {
    fn setup(&self);
}

impl Pins<I2C1> for (PB6<Output<OpenDrain>>, PB7<Output<OpenDrain>>) {
    fn setup(&self) {
        self.0.set_alt_mode(AltMode::I2C);
        self.1.set_alt_mode(AltMode::I2C);
    }
}

impl Pins<I2C1> for (PB8<Output<OpenDrain>>, PB9<Output<OpenDrain>>) {
    fn setup(&self) {
        self.0.set_alt_mode(AltMode::I2C);
        self.1.set_alt_mode(AltMode::I2C);
    }
}

impl Pins<I2C2> for (PB10<Output<OpenDrain>>, PB11<Output<OpenDrain>>) {
    fn setup(&self) {
        self.0.set_alt_mode(AltMode::I2C);
        self.1.set_alt_mode(AltMode::I2C);
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    Bus,
    Arbitration,
    Acknowledge,
    Overrun,
    Timeout,
}

macro_rules! wait_for_flag {
    ($i2c:expr, $flag:ident) => {{
        let sr1 = $i2c.sr1.read();

        if sr1.berr().bit_is_set() {
            $i2c.sr1.write(|w| w.berr().clear_bit());
            Err(Error::Bus)
        } else if sr1.arlo().bit_is_set() {
            $i2c.sr1.write(|w| w.arlo().clear_bit());
            Err(Error::Arbitration)
        } else if sr1.af().bit_is_set() {
            $i2c.sr1.write(|w| w.af().clear_bit());
            Err(Error::Acknowledge)
        } else if sr1.ovr().bit_is_set() {
            $i2c.sr1.write(|w| w.ovr().clear_bit());
            Err(Error::Overrun)
        } else {
            Ok(sr1.$flag().bit_is_set())
        }
    }};
}

macro_rules! busy_wait {
    ($nb_expr:expr, $exit_cond:expr) => {{
        loop {
            match $nb_expr {
                Err(e) => break Err(e),
                Ok(true) => break Ok(()),
                Ok(false) if $exit_cond => break Err(Error::Timeout),
                Ok(false) => {}
            }
        }
    }};
}

macro_rules! busy_wait_cycles {
    ($nb_expr:expr, $cycles:expr) => {{
        let started = DWT::cycle_count();
        let cycles = $cycles;
        busy_wait!($nb_expr, DWT::cycle_count().wrapping_sub(started) >= cycles)
    }};
}

macro_rules! i2c {
    ($I2CX:ident, $i2cx:ident, $i2cxen:ident, $i2crst:ident) => {
        impl<PINS> I2c<$I2CX, PINS> {
            pub fn $i2cx(i2c: $I2CX, pins: PINS, speed: Hertz, rcc: &mut Rcc) -> Self
            where
                PINS: Pins<$I2CX>,
            {
                pins.setup();
                let speed: Hertz = speed.into();

                // Enable clock for I2C
                rcc.rb.apb1enr.modify(|_, w| w.$i2cxen().set_bit());

                // Reset I2C
                rcc.rb.apb1rstr.modify(|_, w| w.$i2crst().set_bit());
                rcc.rb.apb1rstr.modify(|_, w| w.$i2crst().clear_bit());

                // Calculate settings for I2C speed modes
                let clock = rcc.clocks.apb1_clk().0;
                let freq = clock / 1_000_000;
                assert!(freq >= 2 && freq <= 50);

                let mut i2c = I2c {
                    i2c,
                    pins,
                    speed,
                    clock,
                };
                i2c.init();
                i2c
            }

            fn init(&mut self) {
                let i2c = &mut self.i2c;
                let speed = self.speed;
                let clock = self.clock;
                let freq = clock / 1_000_000;

                // Make sure the I2C unit is disabled so we can configure it
                i2c.cr1.modify(|_, w| w.pe().clear_bit());

                // Configure bus frequency into I2C peripheral
                i2c.cr2.write(|w| unsafe { w.freq().bits(freq as u8) });

                let trise = if speed <= 100_u32.khz().into() {
                    freq + 1
                } else {
                    (freq * 300) / 1000 + 1
                };

                // Configure correct rise times
                i2c.trise.write(|w| w.trise().bits(trise as u8));

                // I2C clock control calculation
                if speed <= 100_u32.khz().into() {
                    let ccr = {
                        let ccr = clock / (speed.0 * 2);
                        if ccr < 4 {
                            4
                        } else {
                            ccr
                        }
                    };

                    // Set clock to standard mode with appropriate parameters for selected speed
                    i2c.ccr.write(|w| unsafe {
                        w.f_s()
                            .clear_bit()
                            .duty()
                            .clear_bit()
                            .ccr()
                            .bits(ccr as u16)
                    });
                } else {
                    const DUTYCYCLE: u8 = 0;
                    if DUTYCYCLE == 0 {
                        let ccr = clock / (speed.0 * 3);
                        let ccr = if ccr < 1 { 1 } else { ccr };

                        // Set clock to fast mode with appropriate parameters for selected speed (2:1 duty cycle)
                        i2c.ccr.write(|w| unsafe {
                            w.f_s().set_bit().duty().clear_bit().ccr().bits(ccr as u16)
                        });
                    } else {
                        let ccr = clock / (speed.0 * 25);
                        let ccr = if ccr < 1 { 1 } else { ccr };

                        // Set clock to fast mode with appropriate parameters for selected speed (16:9 duty cycle)
                        i2c.ccr.write(|w| unsafe {
                            w.f_s().set_bit().duty().set_bit().ccr().bits(ccr as u16)
                        });
                    }
                }

                // Enable the I2C processing
                i2c.cr1.modify(|_, w| w.pe().set_bit());
            }

            pub fn release(self) -> ($I2CX, PINS) {
                (self.i2c, self.pins)
            }

            fn send_start(&mut self) -> Result<(), Error> {
                let mut retries_left = 10;
                let mut last_err = Err(Error::Timeout);
                while retries_left > 0 {
                    // Send a START condition
                    self.i2c.cr1.modify(|_, w| w.start().set_bit());

                    // Wait until START condition was generated
                    last_err = busy_wait_cycles!(wait_for_flag!(self.i2c, sb), 24_0000);

                    if last_err.is_err() {
                        self.i2c.cr1.write(|w| w.pe().set_bit().swrst().set_bit());
                        self.i2c.cr1.reset();
                        self.init();
                    } else {
                        break;
                    }

                    retries_left -= 1;
                }
                last_err
            }

            fn write_bytes(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Error> {
                self.send_start()?;

                // Also wait until signalled we're master and everything is waiting for us
                busy_wait_cycles!(
                    {
                        let sr2 = self.i2c.sr2.read();
                        Ok(sr2.msl().bit_is_set() && sr2.busy().bit_is_set())
                    },
                    24_0000
                )?;

                // Set up current address, we're trying to talk to
                self.i2c
                    .dr
                    .write(|w| unsafe { w.bits(u32::from(addr) << 1) });

                // Wait until address was sent
                busy_wait_cycles!(wait_for_flag!(self.i2c, addr), 24_0000)?;

                // Clear condition by reading SR2
                self.i2c.sr2.read();

                // Send bytes
                for c in bytes {
                    self.send_byte(*c)?;
                }

                // Fallthrough is success
                Ok(())
            }

            fn send_byte(&self, byte: u8) -> Result<(), Error> {
                // Wait until we're ready for sending
                busy_wait_cycles!(wait_for_flag!(self.i2c, tx_e), 24_0000)?;

                // Push out a byte of data
                self.i2c.dr.write(|w| unsafe { w.bits(u32::from(byte)) });

                // Wait until byte is transferred
                busy_wait_cycles!(wait_for_flag!(self.i2c, btf), 24_0000)?;

                Ok(())
            }

            fn recv_byte(&self) -> Result<u8, Error> {
                busy_wait_cycles!(wait_for_flag!(self.i2c, rx_ne), 24_0000)?;
                let value = self.i2c.dr.read().bits() as u8;
                Ok(value)
            }
        }

        impl<PINS> WriteRead for I2c<$I2CX, PINS> {
            type Error = Error;

            fn write_read(
                &mut self,
                addr: u8,
                bytes: &[u8],
                buffer: &mut [u8],
            ) -> Result<(), Self::Error> {
                if !bytes.is_empty() {
                    self.write_bytes(addr, bytes)?;
                }

                if !buffer.is_empty() {
                    self.read(addr, buffer)?;
                } else if !bytes.is_empty() {
                    self.i2c.cr1.modify(|_, w| w.stop().set_bit());
                }

                Ok(())
            }
        }

        impl<PINS> Write for I2c<$I2CX, PINS> {
            type Error = Error;

            fn write(&mut self, addr: u8, bytes: &[u8]) -> Result<(), Self::Error> {
                self.write_bytes(addr, bytes)?;

                // Send a STOP condition
                self.i2c.cr1.modify(|_, w| w.stop().set_bit());

                // Fallthrough is success
                Ok(())
            }
        }

        impl<PINS> Read for I2c<$I2CX, PINS> {
            type Error = Error;

            fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Self::Error> {
                if let Some((last, buffer)) = buffer.split_last_mut() {
                    self.send_start()?;

                    // Also wait until signalled we're master and everything is waiting for us
                    busy_wait_cycles!(
                        {
                            let sr2 = self.i2c.sr2.read();
                            Ok(sr2.msl().bit_is_set() && sr2.busy().bit_is_set())
                        },
                        24_0000
                    )?;

                    // Set up current address, we're trying to talk to
                    self.i2c
                        .dr
                        .write(|w| unsafe { w.bits((u32::from(addr) << 1) + 1) });

                    // Wait until address was sent
                    busy_wait_cycles!(wait_for_flag!(self.i2c, addr), 24_0000)?;

                    // Clear condition by reading SR2
                    self.i2c.sr2.read();

                    // Receive bytes into buffer
                    for c in buffer {
                        *c = self.recv_byte()?;
                    }

                    // Prepare to send NACK then STOP after next byte
                    self.i2c
                        .cr1
                        .modify(|_, w| w.ack().clear_bit().stop().set_bit());

                    // Receive last byte
                    *last = self.recv_byte()?;

                    // Fallthrough is success
                    Ok(())
                } else {
                    Err(Error::Overrun)
                }
            }
        }

        impl I2cExt<$I2CX> for $I2CX {
            fn i2c<PINS, T>(self, pins: PINS, speed: T, rcc: &mut Rcc) -> I2c<$I2CX, PINS>
            where
                PINS: Pins<$I2CX>,
                T: Into<Hertz>,
            {
                I2c::$i2cx(self, pins, speed.into(), rcc)
            }
        }
    };
}

pub trait I2cExt<I2C> {
    fn i2c<PINS, T>(self, pins: PINS, speed: T, rcc: &mut Rcc) -> I2c<I2C, PINS>
    where
        PINS: Pins<I2C>,
        T: Into<Hertz>;
}

i2c!(I2C1, i2c1, i2c1en, i2c1rst);
i2c!(I2C2, i2c2, i2c2en, i2c2rst);
