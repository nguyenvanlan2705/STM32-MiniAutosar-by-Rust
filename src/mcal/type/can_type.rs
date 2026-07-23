#![allow(dead_code)]
#![allow(non_camel_case_types)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanControllerIdType {
    CAN0,
    CAN1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanModeType {
    CAN_MODE_NORMAL = 0,
    CAN_MODE_LOOPBACK = 1,
    CAN_MODE_SILENT = 2,
    CAN_MODE_SILENT_LOOPBACK = 3,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanHardwareControllerType {
    MCP2515_0,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CanBaudrateType {
    CAN_BAUDRATE_125K = 0,
    CAN_BAUDRATE_250K = 1,
    CAN_BAUDRATE_500K = 2,
    CAN_BAUDRATE_1M = 3,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CanChannelConfigType{
    pub controller_id: CanControllerIdType,
    pub mode: CanModeType,
    pub baudrate: CanBaudrateType,
    pub interrupt_enable: bool,
    pub hw_controler: CanHardwareControllerType,
}