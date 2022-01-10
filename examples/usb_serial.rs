//! CDC-ACM serial port example using polling in a busy loop.
//!
//! Note:
//! When building this since this is a larger program,
//! one would need to build it using release profile
//! since debug profiles generates artifacts that
//! cause FLASH overflow errors due to their size
#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;
extern crate stm32l1xx_hal as hal;

use embedded_hal::digital::v2::OutputPin;
use hal::prelude::*;
use hal::rcc::{Config, PLLDiv, PLLMul, PLLSource};
use hal::stm32;
use hal::usb::{Peripheral, UsbBus};
use rt::entry;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    // dp.RCC.apb1enr.modify(|_, w| w.pwren().set_bit());
    // dp.PWR.cr.write(|w| unsafe { w.vos().bits(1) });
    // while dp.PWR.csr.read().vosf().bit_is_set() {}

    dp.FLASH.acr.write(|w| w.acc64().set_bit());
    dp.FLASH.acr.modify(|_, w| w.prften().set_bit());
    dp.FLASH.acr.modify(|_, w| w.latency().set_bit());

    let mut rcc = dp.RCC.freeze(Config::pll(
        PLLSource::HSE(16.mhz()),
        PLLMul::Mul6,
        PLLDiv::Div4,
    ));
    let mut delay = cp.SYST.delay(rcc.clocks);

    let mut gpioa = dp.GPIOA.split();

    let mut led = gpioa.pa1.into_push_pull_output();
    led.set_low().unwrap();

    // Pull the D+ pin down to send a RESET condition to the USB bus.
    // This forced reset is needed only for development, without it host
    // will not reset your device when you upload new firmware.
    let mut usb_dp = gpioa.pa12.into_push_pull_output();
    usb_dp.set_low().unwrap();
    delay.delay(10.ms());

    let usb = Peripheral {
        usb: dp.USB,
        pin_dm: gpioa.pa11,
        pin_dp: usb_dp.into_floating_input(),
    };
    let usb_bus = UsbBus::new(usb);

    let mut serial = SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x16c0, 0x27dd))
        .manufacturer("stm32-rs")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(USB_CLASS_CDC)
        .build();

    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 64];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                led.set_high().unwrap();

                // Echo back in upper case
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }

                let mut write_offset = 0;
                while write_offset < count {
                    match serial.write(&buf[write_offset..count]) {
                        Ok(len) if len > 0 => {
                            write_offset += len;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }

        led.set_low().unwrap();
    }
}
