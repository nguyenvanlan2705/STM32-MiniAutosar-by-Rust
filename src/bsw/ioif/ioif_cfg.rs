#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::bsw::ioif::ioif_type::{IoIf_TxChannelType, IoIf_RxChannelType, 
    IoIf_PeripheralType, IoIf_RxPdu, IoIf_TxPdu, IoIf_RxMode, IoIf_TxPduGroup, IoIf_TxChannelGroupType};

const IOIF_RX_PDU_CONFIG: &[IoIf_RxPdu] = &[
    IoIf_RxPdu {
        index: 0,
        id: 0x100,
        peripheral: IoIf_PeripheralType::DIO,
        channel: IoIf_RxChannelType::BUTTON_USER,
        mode: IoIf_RxMode::INTERRUPT,
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
        channel_group: IoIf_TxChannelGroupType::LED_GROUP_RED_YELLOW,
    },
    IoIf_TxPduGroup {
        index: 1,
        id: 0x301,
        peripheral: IoIf_PeripheralType::DIO,
        channel_group: IoIf_TxChannelGroupType::LED_GROUP_BLUE_ORANGE,
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
