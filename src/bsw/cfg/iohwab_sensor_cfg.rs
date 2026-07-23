#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
use crate::mcal::adc_type::{ADCChannelType};
use crate::bsw::iohwab::iohwab_type::{IoHwAb_SensorConfig, SensorType};

pub const SENSOR_CONFIG: &[IoHwAb_SensorConfig] = &[
    IoHwAb_SensorConfig {
        sensor_id: SensorType::LM35,
        adc_channel: ADCChannelType::ADC_CHANNEL_8,
        index: 0,
    },
];
pub const SENSOR_COUNT: usize = SENSOR_CONFIG.len();