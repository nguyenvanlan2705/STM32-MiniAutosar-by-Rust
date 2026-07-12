#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use crate::bsw::common_type::{PduInfoType};
use crate::bsw::cfg::usartif_cfg::{get_usartif_rxpdu_config, USARTIF_RX_PDUS_STATUS};
use crate::bsw::usartif::usartif_type::{UsartIf_ReturnType, UsartIf_RxPduType, UsartIf_RxPduConfig, 
    USARTIF_INVALID_PDU_ID, UsartIf_PduStatus, UsartIf_RxIndicationaStatus};
use crate::register::usart_type::UsartNumber;
use core::sync::atomic::{AtomicU16, AtomicUsize, AtomicU8, Ordering};
use crate::mcal::usart::{usart_clear_error_status, usart_get_rx_status, usart_start_receive_async, usart_read_received_async_data};
use crate::mcal::usart_type::{UsartRxStatus, UsartReturnType};


static USARTIF_RX_BUFFER_PTR: [AtomicUsize; 3] = [
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
];
static USARTIF_RX_BUFFER_LEN: [AtomicUsize; 3] = [
    AtomicUsize::new(0),
    AtomicUsize::new(0),
    AtomicUsize::new(0),
];

static USARTIF_INDICATION_TABLE: [AtomicU8; 3] = [
    //channel 1
    AtomicU8::new(UsartIf_RxIndicationaStatus::USARTIF_RX_INDICATION_NOT_OK as u8),
    //channel 2
    AtomicU8::new(UsartIf_RxIndicationaStatus::USARTIF_RX_INDICATION_NOT_OK as u8),
    //channel 6
    AtomicU8::new(UsartIf_RxIndicationaStatus::USARTIF_RX_INDICATION_NOT_OK as u8),
];
static USARTIF_ACTIVE_RX_CHANNEL: [AtomicU16; 3] = [
    //channel 1
    AtomicU16::new(USARTIF_INVALID_PDU_ID),
    //channel 2
    AtomicU16::new(USARTIF_INVALID_PDU_ID),
    //channel 6
    AtomicU16::new(USARTIF_INVALID_PDU_ID),
];

fn usart_save_rx_buffer(index: usize, buffer_ptr: *mut u8, buffer_len: usize) {
    if index < USARTIF_RX_BUFFER_PTR.len() {
        USARTIF_RX_BUFFER_PTR[index].store(buffer_ptr as usize, Ordering::SeqCst);
        USARTIF_RX_BUFFER_LEN[index].store(buffer_len as usize, Ordering::SeqCst);
    }
}
fn usart_clear_rx_buffer(index: usize) {
    if index < USARTIF_RX_BUFFER_PTR.len() {
        USARTIF_RX_BUFFER_PTR[index].store(0, Ordering::SeqCst);
        USARTIF_RX_BUFFER_LEN[index].store(0, Ordering::SeqCst);
    }
}

fn usartif_channel_to_index(channel: UsartNumber) -> Option<usize> {
    match channel {
        UsartNumber::USART1 => Some(0),
        UsartNumber::USART2 => Some(1),
        UsartNumber::USART6 => Some(2),
    }
}
fn usartif_set_channel_indication(channel: UsartNumber) {
    if let Some(index) = usartif_channel_to_index(channel) {
        USARTIF_INDICATION_TABLE[index].store(UsartIf_RxIndicationaStatus::USARTIF_RX_INDICATION_OK as u8, Ordering::SeqCst);
    }
}
fn usartif_clear_channel_indication(channel: UsartNumber) {
    if let Some(index) = usartif_channel_to_index(channel) {
        USARTIF_INDICATION_TABLE[index].store(UsartIf_RxIndicationaStatus::USARTIF_RX_INDICATION_NOT_OK as u8, Ordering::SeqCst);
    }
}
fn usartif_set_channel_active(channel: UsartNumber) {
    if let Some(index) = usartif_channel_to_index(channel) {
        USARTIF_ACTIVE_RX_CHANNEL[index].store(channel as u16, Ordering::SeqCst);
    }
}
fn usartif_get_channel_active(channel: UsartNumber) -> Option<UsartNumber> {
    if let Some(index) = usartif_channel_to_index(channel) {
        let active_channel = USARTIF_ACTIVE_RX_CHANNEL[index].load(Ordering::SeqCst);
        return match active_channel {
            x if x == UsartNumber::USART1 as u16 => Some(UsartNumber::USART1),
            x if x == UsartNumber::USART2 as u16 => Some(UsartNumber::USART2),
            x if x == UsartNumber::USART6 as u16 => Some(UsartNumber::USART6),
            _ => None, // Default case, you can handle it differently if needed
        };
    }
    None // Default case, you can handle it differently if needed
}
    
fn usartif_reset_channel_active(channel: UsartNumber) {
    if let Some(index) = usartif_channel_to_index(channel) {
        USARTIF_ACTIVE_RX_CHANNEL[index].store(USARTIF_INVALID_PDU_ID, Ordering::SeqCst);
    }
}
fn usartif_set_pdu_status(channel: UsartNumber, status: UsartIf_PduStatus) {
    for pdu_status in USARTIF_RX_PDUS_STATUS.iter() {
        if pdu_status.channel == channel {
            pdu_status.status.store(status as u8, Ordering::SeqCst);
            break;
        }
    }
}
pub fn usartif_get_pdu_status(channel: UsartNumber) -> Option<UsartIf_PduStatus> {
    for pdu_status in USARTIF_RX_PDUS_STATUS.iter() {
        if pdu_status.channel == channel {
            return match pdu_status.status.load(Ordering::SeqCst) {
                x if x == UsartIf_PduStatus::USARTIF_IDLE as u8 => Some(UsartIf_PduStatus::USARTIF_IDLE),
                x if x == UsartIf_PduStatus::USARTIF_PENDING as u8 => Some(UsartIf_PduStatus::USARTIF_PENDING),
                x if x == UsartIf_PduStatus::USARTIF_BUSY as u8 => Some(UsartIf_PduStatus::USARTIF_BUSY),
                x if x == UsartIf_PduStatus::USARTIF_COMPLETED as u8 => Some(UsartIf_PduStatus::USARTIF_COMPLETED),
                x if x == UsartIf_PduStatus::USARTIF_ERROR as u8 => Some(UsartIf_PduStatus::USARTIF_ERROR),
                _ => None,
            };
        }
    }
    None
}

fn usartif_get_rxpducfg_from_id(channel: UsartNumber) -> Option<&'static UsartIf_RxPduType> {
    let rxpdu_config: &UsartIf_RxPduConfig = get_usartif_rxpdu_config();
    for rxpdu in rxpdu_config.rx_pdu.iter() {
        if rxpdu.lower_channel == channel {
            return Some(rxpdu)
        }
    }
    None
}

pub fn usartif_recover_rx_error(channel: UsartNumber) {
    if let Some(index) = usartif_channel_to_index(channel) {
        usart_clear_rx_buffer(index);
    }
    usartif_reset_channel_active(channel);
    usartif_clear_channel_indication(channel);
    usartif_set_pdu_status(channel, UsartIf_PduStatus::USARTIF_IDLE);
    usart_clear_error_status(channel);
}

fn usartif_det_check(channel: UsartNumber, pduinfo : *const PduInfoType) -> UsartIf_ReturnType {
    if pduinfo.is_null() {
        //DET reporting for null pointer
        return UsartIf_ReturnType::USARTIF_NOT_OK;
    }else {
        if unsafe { (*pduinfo).data.is_null() } {
            //DET reporting for null pointer
            return UsartIf_ReturnType::USARTIF_NOT_OK;
        }
        if unsafe { (*pduinfo).length == 0 } {
            //DET reporting for zero length
            return UsartIf_ReturnType::USARTIF_NOT_OK;
        }
        else {
            let rxpdu = usartif_get_rxpducfg_from_id(channel);
            if let Some(rxpdu) = rxpdu {
                if unsafe { (*pduinfo).length as u8 } > rxpdu.rx_pdu_length {
                    //DET reporting for length mismatch
                    return UsartIf_ReturnType::USARTIF_NOT_OK;
                    }
                    return UsartIf_ReturnType::USARTIF_OK;
                }
            }
            //DET reporting for invalid PDU ID
            return UsartIf_ReturnType::USARTIF_NOT_OK;
        }
}

// Implement the usartif_startofreception function
pub fn usartif_rxindication(channel: UsartNumber, pduinfo : *const PduInfoType) -> UsartIf_ReturnType {
    let det_check_result = usartif_det_check(channel, pduinfo);
    if det_check_result == UsartIf_ReturnType::USARTIF_OK {
        let rxpdu = usartif_get_rxpducfg_from_id(channel).unwrap();
        let lower_status = usart_get_rx_status(rxpdu.lower_channel);
        if lower_status == UsartRxStatus::Error {
            usartif_recover_rx_error(rxpdu.lower_channel);
            return UsartIf_ReturnType::USARTIF_NOT_OK;
        }
        if lower_status != UsartRxStatus::Busy {
            let current_status = usartif_get_pdu_status(rxpdu.lower_channel).unwrap();
            if current_status == UsartIf_PduStatus::USARTIF_IDLE || current_status == UsartIf_PduStatus::USARTIF_COMPLETED {
                let buffer_ptr = unsafe { (*pduinfo).data as *mut u8 };
                let buffer_len = unsafe { (*pduinfo).length as usize };

                let index = usartif_channel_to_index(rxpdu.lower_channel).unwrap();
                usart_save_rx_buffer(index, buffer_ptr, buffer_len);
                usartif_set_pdu_status(rxpdu.lower_channel, UsartIf_PduStatus::USARTIF_PENDING);
                usartif_set_channel_active(rxpdu.lower_channel);
                // Start the asynchronous reception
                let status = usart_start_receive_async(rxpdu.lower_channel, unsafe { (*pduinfo).length as usize });
                if status == UsartReturnType::USART_OK {
                    usartif_set_pdu_status(rxpdu.lower_channel, UsartIf_PduStatus::USARTIF_BUSY);
                    return UsartIf_ReturnType::USARTIF_OK;
                } else {
                    usartif_set_pdu_status(rxpdu.lower_channel, UsartIf_PduStatus::USARTIF_ERROR);
                    usart_clear_rx_buffer(index);
                    usartif_reset_channel_active(rxpdu.lower_channel);
                    return UsartIf_ReturnType::USARTIF_NOT_OK;
                }
            } else {
                // Handle the case where the PDU is not in an appropriate state for processing
                usartif_set_pdu_status(rxpdu.lower_channel, UsartIf_PduStatus::USARTIF_ERROR);
                usartif_reset_channel_active(rxpdu.lower_channel);
                return UsartIf_ReturnType::USARTIF_NOT_OK;
            }
        } else {
            // Handle the case where the lower channel is busy or in error state
            return UsartIf_ReturnType::USARTIF_NOT_OK;
        }
    } else {
        // Handle DET error reporting here
        return det_check_result;
    }
}

pub fn usartif_rxindication_by_channel(channel: UsartNumber) {
    let index = usartif_channel_to_index(channel).unwrap();
    // Retrieve the buffer pointer and length for the given channel
    let buffer_ptr = USARTIF_RX_BUFFER_PTR[index].load(Ordering::SeqCst);
    let buffer_len = USARTIF_RX_BUFFER_LEN[index].load(Ordering::SeqCst);
    // Check if the buffer pointer and length are valid
    if buffer_ptr == 0 || buffer_len == 0 {
        usartif_set_pdu_status(channel, UsartIf_PduStatus::USARTIF_ERROR);
        return;
    }
    let buffer = unsafe {
        core::slice::from_raw_parts_mut(buffer_ptr as *mut u8, buffer_len)
    };
    let size = usart_read_received_async_data(channel, buffer);
    // Update the PDU status based on the received data size
    if size > 0 {
        usartif_set_pdu_status(channel, UsartIf_PduStatus::USARTIF_COMPLETED);
        usartif_set_channel_indication(channel);
    } else {
        usartif_set_pdu_status(channel, UsartIf_PduStatus::USARTIF_ERROR);
    }
    usart_clear_rx_buffer(index);
    usartif_reset_channel_active(channel);
}
