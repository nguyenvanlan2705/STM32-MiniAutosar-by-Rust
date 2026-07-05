use crate::bsw::iohwab::button::button_exti_notification;
use crate::mcal::exti_type::{Exti_Config, Exti_ConfigType};
use crate::register::exti_type::Exti_TriggerType;
use crate::register::gpio_type::PORT;
use crate::register::syscfg_type::EXTILINE;

pub const EXTI_CONFIG: Exti_ConfigType = Exti_ConfigType {
    exti: &[Exti_Config {
        port: PORT::A,
        line: EXTILINE::LINE0,
        trigger: Exti_TriggerType::RISING,
        enabled: true,
        callbackfn: Some(button_exti_notification),
    }],
};
