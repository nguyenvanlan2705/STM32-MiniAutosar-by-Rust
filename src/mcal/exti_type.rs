
use crate::register::{exti_type::Exti_TriggerType, syscfg_type::EXTILINE};
use crate::register::gpio_type::PORT;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Exti_Config {
    pub port :PORT,
    pub line: EXTILINE,
    pub trigger: Exti_TriggerType,
    pub enabled: bool,
}
