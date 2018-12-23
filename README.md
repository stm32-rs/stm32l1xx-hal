stm32l1xx-hal
=============

🚧 Work in progress.

_stm32l1xx-hal_ contains a multi device hardware abstraction on top of the
peripheral access API for the STMicro STM32L1 series microcontrollers. The
selection of the MCU is done by feature gates, typically specified by board
support crates. Currently supported configurations are:

* stm32l100
* stm32l151
* stm32l152
* stm32l162

The idea behind this crate is to gloss over the slight differences in the
various peripherals available on those MCUs so a HAL can be written for all
chips in that same family without having to cut and paste crates for every
single model.

Collaboration on this crate is highly welcome as are pull requests!

This crate relies on Adam Greigs fantastic [stm32l1][] crate to provide
appropriate register definitions and implements a partial set of the
[embedded-hal][] traits.

Based on [stm32f4xx-hal][] crate by Daniel Egger.

[stm32l1]: https://crates.io/crates/stm32l1
[stm32f4xx-hal]: https://github.com/stm32-rs/stm32f4xx-hal
[embedded-hal]: https://github.com/japaric/embedded-hal.git