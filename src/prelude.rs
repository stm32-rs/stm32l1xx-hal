pub use hal::prelude::*;

pub use hal::watchdog::Watchdog as _stm32f4xx_hal_watchdog_Watchdog;
pub use hal::watchdog::WatchdogEnable as _stm32f4xx_hal_watchdog_WatchdogEnable;

pub use dac::DacExt as _stm32f4xx_hal_analog_DacExt;
pub use dac::DacOut as _stm32f4xx_hal_analog_DacOut;
pub use dac::DacPin as _stm32f4xx_hal_analog_DacPin;
pub use delay::DelayExt as _stm32f4xx_hal_delay_DelayExt;
pub use exti::ExtiExt as _stm32f4xx_hal_exti_ExtiExt;
pub use gpio::GpioExt as _stm32f4xx_hal_gpio_GpioExt;
pub use pwm::PwmExt as _stm32f4xx_hal_pwm_PwmExt;
pub use rcc::RccExt as _stm32f4xx_hal_rcc_RccExt;
pub use serial::Serial1Ext as _stm32f4xx_hal_serial_Serial1Ext;
pub use serial::Serial2Ext as _stm32f4xx_hal_serial_Serial2Ext;
pub use serial::Serial3Ext as _stm32f4xx_hal_serial_Serial3Ext;
pub use time::U32Ext as _stm32f4xx_hal_time_U32Ext;
pub use timer::TimerExt as _stm32f4xx_hal_timer_TimerExt;
pub use watchdog::IndependedWatchdogExt as _stm32f4xx_hal_watchdog_IndependedWatchdogExt;
pub use watchdog::WindowWatchdogExt as _stm32f4xx_hal_watchdog_WindowWatchdogExt;
// pub use i2c::Pins as _stm32f4xx_hal_i2c_Pins;
