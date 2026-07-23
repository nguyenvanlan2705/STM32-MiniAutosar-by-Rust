#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]


use crate::bsw::common_type::{PduInfoType};
use crate::bsw::cfg::usartif_cfg::{get_usartif_rxpdu_config, USARTIF_RX_PDUS_STATUS};
use crate::bsw::usartif::usartif_type::{UsartIf_ReturnType, UsartIf_RxPduType, UsartIf_RxPduConfig, 
    USARTIF_INVALID_PDU_ID, UsartIf_PduStatus, UsartIf_RxIndicationaStatus};
use crate::register::usart_type::UsartNumber;
use core::sync::atomic::{AtomicU16, AtomicU32, AtomicUsize, AtomicU8, Ordering};
use crate::mcal::usart::{usart_clear_error_status, usart_get_rx_status, usart_start_receive_async, usart_rx_ring_pop, usart_rx_ring_is_empty};
use crate::mcal::usart_type::{UsartRxStatus, UsartReturnType};
use crate::mcal::mcu::mcu_get_system_tick_count;

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

static USART_RX_INDEX: [AtomicUsize; 3] = [
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

static USARTIF_RX_LAST_BYTE_TICK: [AtomicU32; 3] = [
    AtomicU32::new(0),
    AtomicU32::new(0),
    AtomicU32::new(0),
];

fn usartif_update_last_byte_tick(channel: UsartNumber) {
    if let Some(index) = usartif_channel_to_index(channel) {
        let current_tick = mcu_get_system_tick_count();
        USARTIF_RX_LAST_BYTE_TICK[index].store(current_tick, Ordering::SeqCst);
    }
}

fn usartif_get_last_byte_tick(channel: UsartNumber) -> Option<u32> {
    if let Some(index) = usartif_channel_to_index(channel) {
        return Some(USARTIF_RX_LAST_BYTE_TICK[index].load(Ordering::SeqCst));
    }
    None
}

fn usartif_increase_rx_index(index: usize) {
    if index < USART_RX_INDEX.len() {
        let current_index = USART_RX_INDEX[index].load(Ordering::SeqCst);
        let new_index = current_index + 1; 
        USART_RX_INDEX[index].store(new_index, Ordering::SeqCst);
    }
}
fn usartif_get_rx_index(index: usize) -> usize {
    if index < USART_RX_INDEX.len() {
        return USART_RX_INDEX[index].load(Ordering::SeqCst);
    }
    0 // Default value if index is out of bounds
}
fn usartif_reset_rx_index(index: usize) {
    if index < USART_RX_INDEX.len() {
        USART_RX_INDEX[index].store(0, Ordering::SeqCst);
    }
}
fn usart_save_rx_buffer(index: usize, buffer_ptr: *mut u8, buffer_len: usize) {
    if index < USARTIF_RX_BUFFER_PTR.len() {
        USARTIF_RX_BUFFER_PTR[index].store(buffer_ptr as usize, Ordering::SeqCst);
        USARTIF_RX_BUFFER_LEN[index].store(buffer_len as usize, Ordering::SeqCst);
    }
}

fn usartif_clear_upper_buffer(buffer_ptr: *mut u8, buffer_len: usize) {
    unsafe {
        let buffer = core::slice::from_raw_parts_mut(buffer_ptr, buffer_len);
        for byte in buffer.iter_mut() {
            *byte = 0;
        }
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
pub fn usartif_rx_data_is_available(channel: UsartNumber) -> bool {
    !usart_rx_ring_is_empty(channel)
}
pub fn usartif_recover_rx_error(channel: UsartNumber) {
    if let Some(index) = usartif_channel_to_index(channel) {
        let buffer_ptr = USARTIF_RX_BUFFER_PTR[index].load(Ordering::SeqCst) as *mut u8;
        let buffer_len = USARTIF_RX_BUFFER_LEN[index].load(Ordering::SeqCst) as usize;
        if !buffer_ptr.is_null() && buffer_len > 0 {
            usartif_clear_upper_buffer(buffer_ptr, buffer_len);
        }
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
pub fn usartif_startofreception(channel: UsartNumber, pduinfo : *const PduInfoType) -> UsartIf_ReturnType {
    let det_check_result = usartif_det_check(channel, pduinfo);
    if det_check_result == UsartIf_ReturnType::USARTIF_OK {
        let rxpdu = usartif_get_rxpducfg_from_id(channel).unwrap();
        let lower_status = usart_get_rx_status(rxpdu.lower_channel);
        let current_status = usartif_get_pdu_status(rxpdu.lower_channel).unwrap();
        if lower_status == UsartRxStatus::Error || current_status == UsartIf_PduStatus::USARTIF_ERROR {
            usartif_recover_rx_error(rxpdu.lower_channel);
            return UsartIf_ReturnType::USARTIF_NOT_OK;
        }
        let buffer_ptr = unsafe { (*pduinfo).data as *mut u8 };
        let buffer_len = unsafe { (*pduinfo).length as usize };
        if  current_status == UsartIf_PduStatus::USARTIF_IDLE || current_status == UsartIf_PduStatus::USARTIF_COMPLETED {
            let index = usartif_channel_to_index(rxpdu.lower_channel).unwrap();
            usart_save_rx_buffer(index, buffer_ptr, buffer_len);
            usartif_clear_channel_indication(rxpdu.lower_channel);
            usartif_clear_upper_buffer(buffer_ptr, buffer_len);
            usartif_reset_rx_index(index);
            usartif_set_pdu_status(rxpdu.lower_channel, UsartIf_PduStatus::USARTIF_PENDING);
            usartif_set_channel_active(rxpdu.lower_channel);
            // Start the asynchronous reception
            let status = usart_start_receive_async(rxpdu.lower_channel, unsafe { (*pduinfo).length as usize });
            if status == UsartReturnType::USART_OK || status == UsartReturnType::USART_BUSY {
                usartif_set_pdu_status(rxpdu.lower_channel, UsartIf_PduStatus::USARTIF_BUSY);
                return UsartIf_ReturnType::USARTIF_OK;
            } else {
                usartif_set_pdu_status(rxpdu.lower_channel, UsartIf_PduStatus::USARTIF_ERROR);
                usartif_clear_upper_buffer(buffer_ptr, buffer_len);
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
        // Handle DET error reporting here
        return det_check_result;
    }
}

fn usartif_pdu_timeout_processing(channel: UsartNumber) {
    if let Some(index) = usartif_channel_to_index(channel) {
        let buffer_ptr = USARTIF_RX_BUFFER_PTR[index].load(Ordering::SeqCst) as *mut u8;
        let buffer_len = USARTIF_RX_BUFFER_LEN[index].load(Ordering::SeqCst) as usize;
        if !buffer_ptr.is_null() && buffer_len > 0 {
            usartif_clear_upper_buffer(buffer_ptr, buffer_len);
        }
    }
    if let Some(index) = usartif_channel_to_index(channel) {
        usartif_reset_rx_index(index);
    }
    usartif_reset_channel_active(channel);
    usartif_clear_channel_indication(channel);
    usartif_set_pdu_status(channel, UsartIf_PduStatus::USARTIF_ERROR);
}
fn usartif_rx_data_is_timeout(channel: UsartNumber, timeout_ms: u32) -> bool {
    if let Some(last_byte_tick) = usartif_get_last_byte_tick(channel) {
        let current_tick = mcu_get_system_tick_count();
        let elapsed_time = current_tick.wrapping_sub(last_byte_tick);
        return elapsed_time >= timeout_ms;
    }
    false
}
pub fn usartif_rx_timeout_processing(channel: UsartNumber) {
    let index = usartif_channel_to_index(channel).unwrap();
    let buffer_index = usartif_get_rx_index(index);
    let current_status = usartif_get_pdu_status(channel).unwrap();
    if buffer_index == 0 {
        // No data has been received yet, so no timeout processing is needed
        return;
    }
    if current_status != UsartIf_PduStatus::USARTIF_BUSY {
        // If the status is not BUSY, we should not process further
        return;
    }
    if let Some(rxpdu) = usartif_get_rxpducfg_from_id(channel) {
        if usartif_rx_data_is_timeout(channel, rxpdu.rx_timeout) {
            usartif_pdu_timeout_processing(channel);
        }
    }
}
fn usartif_crc8_calc(data : &[u8]) -> u8 {
    let mut crc: u8 = 0x00;
    for &byte in data {
        crc ^= byte;
        for _ in 0..8 {
            if (crc & 0x80) != 0 {
                crc = (crc << 1) ^ 0x07; // Polynomial x^8 + x^2 + x + 1
            } else {
                crc <<= 1;
            }
        }
    }
    crc
}
fn usartif_ascii_to_hex(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'A'..=b'F' => byte - b'A' + 10,
        b'a'..=b'f' => byte - b'a' + 10,
        _ => 0,
    }
}

fn usartif_pair_ascii_to_hex(high: u8, low: u8) -> u8 {
    (usartif_ascii_to_hex(high) << 4) | usartif_ascii_to_hex(low)
}
fn usartif_remove_crc_from_buffer(channel: UsartNumber) {
    let index = usartif_channel_to_index(channel).unwrap();
    let current_index = usartif_get_rx_index(index);
    if current_index >= 2 {
        unsafe {
            let buffer_ptr = USARTIF_RX_BUFFER_PTR[index].load(Ordering::SeqCst) as *mut u8;
            let buffer_slice = core::slice::from_raw_parts_mut(buffer_ptr, current_index);
            // Remove the last two bytes (CRC) from the buffer
            buffer_slice[current_index - 2] = 0;
            buffer_slice[current_index - 1] = 0;
        }
        // Update the index to reflect the removal of CRC bytes
        USART_RX_INDEX[index].store(current_index - 2, Ordering::SeqCst);
    }
    
}
pub fn usartif_rx_processing(channel: UsartNumber){
    let index = usartif_channel_to_index(channel).unwrap();
    // Retrieve the buffer pointer and length for the given channel
    let buffer_ptr = USARTIF_RX_BUFFER_PTR[index].load(Ordering::SeqCst);
    let buffer_len = USARTIF_RX_BUFFER_LEN[index].load(Ordering::SeqCst);
    // Check if the buffer pointer and length are valid
    if buffer_ptr == 0 || buffer_len == 0 {
        usartif_set_pdu_status(channel, UsartIf_PduStatus::USARTIF_ERROR);
        return;
    }
    let current_status = usartif_get_pdu_status(channel).unwrap();
    if current_status != UsartIf_PduStatus::USARTIF_BUSY {
        // If the status is not BUSY, we should not process further
        return;
    }

    while let Some(byte) = usart_rx_ring_pop(channel) {
        unsafe {
            let buffer_slice = core::slice::from_raw_parts_mut(buffer_ptr as *mut u8, buffer_len);
            let current_index = usartif_get_rx_index(index);
            if current_index >= buffer_len {
                // Buffer overflow, set the status to error and return
                usartif_set_pdu_status(channel, UsartIf_PduStatus::USARTIF_ERROR);
                return;
            }
    
            if byte == b'\r' || byte == b'\n'{
                let pducfg = usartif_get_rxpducfg_from_id(channel).unwrap();
                if pducfg.crc {
                    if current_index < 3 {
                        // Not enough data to check CRC, set the status to error and return
                        usartif_set_pdu_status(channel, UsartIf_PduStatus::USARTIF_ERROR);
                        return;
                    }
                    let crc_calc = usartif_crc8_calc(&buffer_slice[0..current_index- 2]); // Calculate CRC for the received data excluding the last byte
                    let crc_received = usartif_pair_ascii_to_hex(buffer_slice[current_index -2], buffer_slice[current_index -1]); // Assuming the last byte is the CRC byte
                    if crc_calc != crc_received {
                        // CRC mismatch, set the status to error and return
                        usartif_set_pdu_status(channel, UsartIf_PduStatus::USARTIF_ERROR);
                        return;
                    }
                }
                usartif_remove_crc_from_buffer(channel);
                // End of reception, set the status to completed
                usartif_set_pdu_status(channel, UsartIf_PduStatus::USARTIF_COMPLETED);
                usartif_reset_rx_index(index);
                usartif_set_channel_indication(channel);
                usartif_reset_channel_active(channel);
                return;
            }
            buffer_slice[current_index] = byte;
            if usartif_get_last_byte_tick(channel).is_some() {
                usartif_update_last_byte_tick(channel);
            }
            usartif_increase_rx_index(index);
            let new_index = usartif_get_rx_index(index);
            if new_index >= buffer_len {
                // Buffer overflow, set the status to error and return
                usartif_set_pdu_status(channel, UsartIf_PduStatus::USARTIF_ERROR);
                return;
            }
        }
    }
}
