#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
use crate::mcal::adc_type::{ADCChannelType};

// src/bsw/iohwab/iohwab_type.rs
// Định nghĩa các loại dữ liệu và cấu trúc cho I/O Hardware Abstraction Layer (IOHWAB)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/* LED */
pub enum LedColor {
    Yellow,
    Orange,
    Red,
    Blue,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedState {
    On,
    Off,
    Toggle
}

/* Button */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    UserButton,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedGroup {
    RedBlue    , // Ví dụ: Nhóm LED 1
    OrangeYellow, // Ví dụ: Nhóm LED 2
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorType{
    LM35,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SensorStatusType{
    SENSOR_IDLE,
    SENSOR_CONVERTING,
    SENSOR_COMPLETE,
    SENSOR_ERROR,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IoHwAb_SensorConfig {
    pub sensor_id: SensorType,
    pub adc_channel: ADCChannelType,
    pub index: usize,
}
pub struct IoHwAb_SensorConfigType {
    pub sensors: &'static [IoHwAb_SensorConfig],
}

pub enum IoHwAb_ReturnType {
    IOHWAB_E_OK = 0,
    IOHWAB_E_NOT_OK = 1,
}