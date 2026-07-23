#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::register::usart_type::UsartNumber;
use crate::bsw::usartif::usartif_type::{UsartIf_TxPduType, UsartIf_TxPduConfig, UsartIf_TxPduStatusType,
     UsartIf_RxPduStatusType, UsartIf_PduStatus, UsartIf_RxPduType, UsartIf_RxPduConfig};
use core::sync::atomic::{AtomicU8};
/*TX Session */
const USART_TXPDUCONFIG : UsartIf_TxPduConfig = UsartIf_TxPduConfig {
    tx_pdu: &[
        UsartIf_TxPduType {
            tx_pdu_id: 0x00,
            tx_pdu_length: 16,
            lower_channel: UsartNumber::USART2,
            confirmation_index: 0,
        },
    ],
};
pub const USART_TXPDUCONFIG_SIZE: usize = USART_TXPDUCONFIG.tx_pdu.len();
pub fn get_usartif_txpdu_config() -> &'static UsartIf_TxPduConfig {
    &USART_TXPDUCONFIG
}

pub static USARTIF_TX_PDUS_STATUS: [UsartIf_TxPduStatusType; USART_TXPDUCONFIG_SIZE] = [
    // Initialize all PDU statuses to IDLE
    UsartIf_TxPduStatusType {
        pdu_id: 0,
        status: AtomicU8::new(UsartIf_PduStatus::USARTIF_IDLE as u8),
    },
];

/*RX Session */
const USART_RXPDUCONFIG : UsartIf_RxPduConfig = UsartIf_RxPduConfig {
    rx_pdu: &[
        UsartIf_RxPduType {
            rx_pdu_id: 0x00,
            rx_pdu_length: 16,
            lower_channel: UsartNumber::USART2,
            upper_id: 0x00,
            rx_timeout: 1000, // Timeout in milliseconds
            crc: true, // Enable CRC check for this PDU
        },
    ],
};
pub fn get_usartif_rxpdu_config() -> &'static UsartIf_RxPduConfig {
    &USART_RXPDUCONFIG
}
pub const USART_RXPDUCONFIG_SIZE: usize = USART_RXPDUCONFIG.rx_pdu.len();
pub static USARTIF_RX_PDUS_STATUS: [UsartIf_RxPduStatusType; USART_RXPDUCONFIG_SIZE] = [
    // Initialize all PDU statuses to IDLE
    UsartIf_RxPduStatusType {
        channel: UsartNumber::USART2,
        status: AtomicU8::new(UsartIf_PduStatus::USARTIF_IDLE as u8),
    },
];
