#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use crate::register::usart_type::UsartNumber;
use crate::bsw::common_type::PduIdType;
use core::sync::atomic::{AtomicU8};
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsartIf_ReturnType {
    USARTIF_OK,
    USARTIF_NOT_OK,
}



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsartIf_TxPduType{
    pub tx_pdu_id: PduIdType,
    pub tx_pdu_length: u8,
    pub lower_channel: UsartNumber,
    pub confirmation_index: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsartIf_TxPduConfig {
    pub tx_pdu: &'static [UsartIf_TxPduType],
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsartIf_TxConfirmationStatus {
    USARTIF_TX_CONFIRMATION_OK,
    USARTIF_TX_CONFIRMATION_NOT_OK,
}
pub const USARTIF_INVALID_PDU_ID: u16 = 0xFFFF;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsartIf_PduStatus {
    USARTIF_IDLE,
    USARTIF_PENDING,
    USARTIF_BUSY,
    USARTIF_COMPLETED,
    USARTIF_ERROR,
}
pub struct UsartIf_TxPduStatusType {
    pub pdu_id: PduIdType,
    pub status: AtomicU8,
}
pub struct UsartIf_RxPduStatusType {
    pub channel: UsartNumber,
    pub status: AtomicU8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsartIf_RxPduType{
    pub rx_pdu_id: PduIdType,
    pub rx_pdu_length: u8,
    pub lower_channel: UsartNumber,
    pub upper_id : PduIdType, 
    pub rx_timeout: u32,
    pub crc : bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsartIf_RxPduConfig {
    pub rx_pdu: &'static [UsartIf_RxPduType],
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsartIf_RxIndicationaStatus {
    USARTIF_RX_INDICATION_OK,
    USARTIF_RX_INDICATION_NOT_OK,
}