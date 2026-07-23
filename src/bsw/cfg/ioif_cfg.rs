#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::bsw::ioif::ioif_type::{IoIf_TxChannelType, IoIf_RxChannelType, 
    IoIf_PeripheralType, IoIf_RxPdu, IoIf_TxPdu, IoIf_RxMode, IoIf_TxPduGroup, IoIf_TxChannelGroupType, IoIf_PduRuntimeStatus, IoIf_PduStatusType};
use core::sync::atomic::{AtomicU8};

const IOIF_RX_PDU_CONFIG: &[IoIf_RxPdu] = &[
    IoIf_RxPdu {
        index: 0,
        id: 0x100,
        peripheral: IoIf_PeripheralType::DIO,
        channel: IoIf_RxChannelType::BUTTON_USER,
        mode: IoIf_RxMode::INTERRUPT,
    },
    IoIf_RxPdu {
        index: 1,
        id: 0x101,
        peripheral: IoIf_PeripheralType::ADC,
        channel: IoIf_RxChannelType::SENSOR_LM35,
        mode: IoIf_RxMode::POLLING,
    },
];
const IOIF_TX_PDU_CONFIG: &[IoIf_TxPdu] = &[
    IoIf_TxPdu {
        index: 0,
        id: 0x200,
        peripheral: IoIf_PeripheralType::DIO,
        channel: IoIf_TxChannelType::LED_RED,
    },
    IoIf_TxPdu {
        index : 1,
        id: 0x201,
        peripheral: IoIf_PeripheralType::DIO,
        channel: IoIf_TxChannelType::LED_ORANGE,
    },
    IoIf_TxPdu {
        index : 2,
        id: 0x202,
        peripheral: IoIf_PeripheralType::DIO,
        channel: IoIf_TxChannelType::LED_BLUE,
    },
    IoIf_TxPdu {
        index : 3,
        id: 0x203,
        peripheral: IoIf_PeripheralType::DIO,
        channel: IoIf_TxChannelType::LED_YELLOW,
    },
];
const IOIF_TX_PDU_GROUP_CONFIG: &[IoIf_TxPduGroup] = &[
    IoIf_TxPduGroup {
        index: 0,
        id: 0x300,
        peripheral: IoIf_PeripheralType::DIO,
        channel_group: IoIf_TxChannelGroupType::LED_GROUP_RED_BLUE,
    },
    IoIf_TxPduGroup {
        index: 1,
        id: 0x301,
        peripheral: IoIf_PeripheralType::DIO,
        channel_group: IoIf_TxChannelGroupType::LED_GROUP_ORANGE_YELLOW,
    },
];
pub const IOIF_RX_PDU_COUNT: usize = IOIF_RX_PDU_CONFIG.len();
pub const IOIF_TX_PDU_COUNT: usize = IOIF_TX_PDU_CONFIG.len();
pub const IOIF_TX_PDU_GROUP_COUNT: usize = IOIF_TX_PDU_GROUP_CONFIG.len();
pub fn ioif_get_rx_pdu_config() -> &'static [IoIf_RxPdu] {
    IOIF_RX_PDU_CONFIG
}
pub fn ioif_get_tx_pdu_config() -> &'static [IoIf_TxPdu] {
    IOIF_TX_PDU_CONFIG
}
pub fn ioif_get_tx_pdu_group_config() -> &'static [IoIf_TxPduGroup] {
    IOIF_TX_PDU_GROUP_CONFIG
}

pub static IOIF_RX_PDU_STATUS: [IoIf_PduRuntimeStatus; IOIF_RX_PDU_COUNT] = [
    IoIf_PduRuntimeStatus {
        pdu_id: 0x100,
        status: AtomicU8::new(IoIf_PduStatusType::IOIF_IDLE as u8),
    },
    IoIf_PduRuntimeStatus {
        pdu_id: 0x101,
        status: AtomicU8::new(IoIf_PduStatusType::IOIF_IDLE as u8),
    },
];
pub static IOIF_TX_PDU_STATUS: [IoIf_PduRuntimeStatus; IOIF_TX_PDU_COUNT] = [
    IoIf_PduRuntimeStatus {
        pdu_id: 0x200,
        status: AtomicU8::new(IoIf_PduStatusType::IOIF_IDLE as u8),
    },
    IoIf_PduRuntimeStatus {
        pdu_id: 0x201,
        status: AtomicU8::new(IoIf_PduStatusType::IOIF_IDLE as u8),
    },
    IoIf_PduRuntimeStatus {
        pdu_id: 0x202,
        status: AtomicU8::new(IoIf_PduStatusType::IOIF_IDLE as u8),
    },
    IoIf_PduRuntimeStatus {
        pdu_id: 0x203,
        status: AtomicU8::new(IoIf_PduStatusType::IOIF_IDLE as u8),
    },
];

pub static IOIF_TX_PDU_GROUP_STATUS: [IoIf_PduRuntimeStatus; IOIF_TX_PDU_GROUP_COUNT] = [
    IoIf_PduRuntimeStatus {
        pdu_id: 0x300,
        status: AtomicU8::new(IoIf_PduStatusType::IOIF_IDLE as u8),
    },
    IoIf_PduRuntimeStatus {
        pdu_id: 0x301,
        status: AtomicU8::new(IoIf_PduStatusType::IOIF_IDLE as u8),
    },
];
