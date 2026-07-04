
#![allow(non_camel_case_types)]

use crate::register::{exti_type::Exti_TriggerType, syscfg_type::EXTILINE};
use crate::register::gpio_type::PORT;

#[derive(Clone, Copy, Debug)]
pub struct Exti_Config {
    pub port :PORT,
    pub line: EXTILINE,
    pub trigger: Exti_TriggerType,
    pub enabled: bool,
    pub callbackfn : Option<fn()>,
}
pub struct Exti_ConfigType {
    pub exti: &'static [Exti_Config],
}