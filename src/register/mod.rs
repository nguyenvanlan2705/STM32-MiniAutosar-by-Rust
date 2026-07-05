#[path = "type/gpio_type.rs"]
pub mod gpio_type;
#[path = "type/rcc_type.rs"]
pub mod rcc_type;
#[path = "type/syscfg_type.rs"]
pub mod syscfg_type;
#[path = "type/exti_type.rs"]
pub mod exti_type;
#[path = "type/nvic_type.rs"]
pub mod nvic_type;

#[path = "src/gpio.rs"]
pub mod gpio;
#[path = "src/dio.rs"]
pub mod dio;
#[path = "src/syscfg.rs"]
pub mod syscfg;
#[path = "src/exti.rs"]
pub mod exti;
#[path = "src/nvic.rs"]
pub mod nvic;

#[path = "cfg/common.rs"]
pub mod common;
