#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
use core::sync::atomic::{AtomicU8};
use crate::bsw::common_type::PduIdType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]   
pub enum IoIf_ReturnType {
    IOIF_E_OK = 0,
    IOIF_E_NOT_OK = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]   
pub enum IoIf_TxChannelType {
    LED_RED,
    LED_ORANGE,
    LED_BLUE,
    LED_YELLOW,
    RELAY,
    FAN_ENABLE,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)] 
pub enum IoIf_TxChannelGroupType{
    LED_GROUP_RED_BLUE,
    LED_GROUP_ORANGE_YELLOW,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)] 
pub enum IoIf_RxChannelType {
    BUTTON_USER,
    SENSOR_LM35,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]   
pub enum IoIf_PeripheralType{
    DIO,
    ADC,
    PWM
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]   
pub enum IoIf_RxMode{
    POLLING,
    INTERRUPT,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]   
pub struct IoIf_RxPdu{
    pub index : usize,
    pub id : PduIdType,
    pub peripheral: IoIf_PeripheralType,
    pub channel: IoIf_RxChannelType,
    pub mode: IoIf_RxMode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]   
pub struct IoIf_TxPdu{
    pub index : usize,
    pub id : PduIdType,
    pub peripheral: IoIf_PeripheralType,
    pub channel: IoIf_TxChannelType,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]   
pub struct IoIf_TxPduGroup{
    pub index : usize,
    pub id : PduIdType,
    pub peripheral: IoIf_PeripheralType,
    pub channel_group: IoIf_TxChannelGroupType,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]   
pub struct IoIf_ConfigRXType{
    pub rx_pdus: &'static [IoIf_RxPdu],
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]   
pub struct IoIf_ConfigTXType{
    pub tx_pdus: &'static [IoIf_TxPdu],
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IoIf_ConfigTXGroupType{
    pub tx_pdu_groups: &'static [IoIf_TxPduGroup],
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)] 
pub enum IoIf_OutputType {
    STD_ON,
    STD_OFF,
    TOGGLE
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoIf_ConfirmationType {
    CONFIRMED_OK,
    CONFIRMED_NOT_OK,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoIf_PduStatusType{
    IOIF_IDLE,
    IOIF_PENDING,
    IOIF_BUSY,
    IOIF_COMPLETED,
    IOIF_ERROR,
}

pub struct IoIf_PduRuntimeStatus {
    pub pdu_id: PduIdType,
    pub status: AtomicU8,
}
pub const IOIF_INVALID_PDU_ID: PduIdType = 0xFFFF;