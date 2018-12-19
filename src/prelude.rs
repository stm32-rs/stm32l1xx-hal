pub use hal::prelude::*;

// pub use i2c::Pins as _stm32f4xx_hal_i2c_Pins;
pub use dac::DacExt as _stm32f4xx_hal_analog_DacExt;
pub use dac::DacOut as _stm32f4xx_hal_analog_DacOut;
pub use dac::DacPin as _stm32f4xx_hal_analog_DacPin;
pub use gpio::GpioExt as _stm32f4xx_hal_gpio_GpioExt;
pub use pwm::PwmExt as _stm32f4xx_hal_pwm_PwmExt;
pub use rcc::RccExt as _stm32f4xx_hal_rcc_RccExt;
pub use time::U32Ext as _stm32f4xx_hal_time_U32Ext;
