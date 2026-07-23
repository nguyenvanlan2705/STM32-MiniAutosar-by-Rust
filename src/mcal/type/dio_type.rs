#![allow(dead_code)]
#![allow(non_camel_case_types)]

use crate::register::gpio_type::{PORT, PIN};
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StdReturnType {
    E_OK = 0,
    E_NOT_OK = 1,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dio_ChannelType {
    LedRed,
    LedOrange,
    LedBlue,
    LedYellow,
    UserButton,
    Relay,
    FanEnable,
    OnboardSpiSensorCs,
    Mcp2515Cs,
    mcp2515Int,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]    
pub struct Dio_ChannelGroupType{
    pub port : PORT,
    pub mask : u16,
    pub offset : u8,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dio_ChannelConfig {
    pub channel: Dio_ChannelType,
    pub port: PORT,
    pub pin: PIN,
}

pub struct Dio_ConfigType {
    pub channels: &'static [Dio_ChannelConfig],
}

pub struct Dio_GroupConfigType{
    pub groups: &'static [Dio_ChannelGroupType],
}
