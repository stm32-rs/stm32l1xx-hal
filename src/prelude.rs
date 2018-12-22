pub use hal::prelude::*;

pub use hal::watchdog::Watchdog as _stm32f4xx_hal_watchdog_Watchdog;
pub use hal::watchdog::WatchdogEnable as _stm32f4xx_hal_watchdog_WatchdogEnable;

pub use crate::dac::DacExt as _stm32f4xx_hal_analog_DacExt;
pub use crate::dac::DacOut as _stm32f4xx_hal_analog_DacOut;
pub use crate::dac::DacPin as _stm32f4xx_hal_analog_DacPin;
pub use crate::delay::DelayExt as _stm32f4xx_hal_delay_DelayExt;
pub use crate::exti::ExtiExt as _stm32f4xx_hal_exti_ExtiExt;
pub use crate::gpio::GpioExt as _stm32f4xx_hal_gpio_GpioExt;
pub use crate::pwm::PwmExt as _stm32f4xx_hal_pwm_PwmExt;
pub use crate::rcc::RccExt as _stm32f4xx_hal_rcc_RccExt;
pub use crate::serial::Serial1Ext as _stm32f4xx_hal_serial_Serial1Ext;
pub use crate::serial::Serial2Ext as _stm32f4xx_hal_serial_Serial2Ext;
pub use crate::serial::Serial3Ext as _stm32f4xx_hal_serial_Serial3Ext;
pub use crate::time::U32Ext as _stm32f4xx_hal_time_U32Ext;
pub use crate::timer::TimerExt as _stm32f4xx_hal_timer_TimerExt;
pub use crate::watchdog::IndependedWatchdogExt as _stm32f4xx_hal_watchdog_IndependedWatchdogExt;
pub use crate::watchdog::WindowWatchdogExt as _stm32f4xx_hal_watchdog_WindowWatchdogExt;
// pub use i2c::Pins as _stm32f4xx_hal_i2c_Pins;
