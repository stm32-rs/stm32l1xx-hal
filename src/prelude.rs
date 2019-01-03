pub use hal::prelude::*;

pub use hal::adc::OneShot as _hal_adc_OneShot;
pub use hal::watchdog::Watchdog as _hal_watchdog_Watchdog;
pub use hal::watchdog::WatchdogEnable as _hal_watchdog_WatchdogEnable;

pub use crate::adc::AdcExt as _stm32f4xx_hal_analog_AdcExt;
pub use crate::dac::DacExt as _stm32f4xx_hal_analog_DacExt;
pub use crate::dac::DacOut as _stm32f4xx_hal_analog_DacOut;
pub use crate::dac::DacPin as _stm32f4xx_hal_analog_DacPin;
pub use crate::delay::DelayExt as _stm32f4xx_hal_delay_DelayExt;
pub use crate::dma::DmaExt as _stm32f4xx_hal_dma_DmaExt;
pub use crate::exti::ExtiExt as _stm32f4xx_hal_exti_ExtiExt;
pub use crate::gpio::GpioExt as _stm32f4xx_hal_gpio_GpioExt;
pub use crate::i2c::I2c1Ext as _stm32f4xx_hal_i2c1_I2cExt;
pub use crate::i2c::I2c2Ext as _stm32f4xx_hal_i2c2_I2cExt;
pub use crate::lcd::LcdExt as _stm32f4xx_hal_lcd_LcdExt;
pub use crate::pwm::PwmExt as _stm32f4xx_hal_pwm_PwmExt;
pub use crate::qei::QeiExt as _stm32f4xx_hal_qei_QeiExt;
pub use crate::rcc::RccExt as _stm32f4xx_hal_rcc_RccExt;
pub use crate::serial::Serial1Ext as _stm32f4xx_hal_serial_Serial1Ext;
pub use crate::serial::Serial2Ext as _stm32f4xx_hal_serial_Serial2Ext;
pub use crate::serial::Serial3Ext as _stm32f4xx_hal_serial_Serial3Ext;
pub use crate::spi::SpiExt as _stm32f4xx_hal_spi_SpiExt;
pub use crate::time::MonoTimerExt as _stm32f4xx_hal_time_MonoTimerExt;
pub use crate::time::U32Ext as _stm32f4xx_hal_time_U32Ext;
pub use crate::timer::TimerExt as _stm32f4xx_hal_timer_TimerExt;
pub use crate::watchdog::IndependedWatchdogExt as _stm32f4xx_hal_watchdog_IndependedWatchdogExt;
pub use crate::watchdog::WindowWatchdogExt as _stm32f4xx_hal_watchdog_WindowWatchdogExt;
