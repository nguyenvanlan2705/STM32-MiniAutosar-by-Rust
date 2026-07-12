#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::bsw::common_type::{PduIdType, PduInfoType};
use crate::bsw::usartif::usartif_type::{UsartIf_ReturnType, UsartIf_TxPduType, UsartIf_TxConfirmationStatus, USARTIF_INVALID_PDU_ID,UsartIf_PduStatus};
use crate::bsw::cfg::usartif_cfg:: {get_usartif_txpdu_config, USART_TXPDUCONFIG_SIZE, USARTIF_TX_PDUS_STATUS};
use crate::mcal::usart::{usart_get_tx_status, usart_start_send_async};
use crate::mcal::usart_type::{UsartTxStatus, UsartReturnType};
use crate::register::usart_type::UsartNumber;
use core::sync::atomic::{ AtomicU8, AtomicU16, Ordering};

static USARTIF_TX_CONFIRMATION_TABLE: [AtomicU8; USART_TXPDUCONFIG_SIZE] =
    [const { AtomicU8::new(UsartIf_TxConfirmationStatus::USARTIF_TX_CONFIRMATION_NOT_OK as u8) }; USART_TXPDUCONFIG_SIZE];

static USARTIF_ACTIVE_TXPDU_ID: [AtomicU16; 3] = [
    //channel 1
    AtomicU16::new(USARTIF_INVALID_PDU_ID),
    //channel 2
    AtomicU16::new(USARTIF_INVALID_PDU_ID),
    //channel 6
    AtomicU16::new(USARTIF_INVALID_PDU_ID),
];

/*Helpers Session */
fn usartif_set_pdu_status(txpduid: PduIdType, status: UsartIf_PduStatus) {
    for pdu_status in USARTIF_TX_PDUS_STATUS.iter() {
        if pdu_status.pdu_id == txpduid {
            pdu_status.status.store(status as u8, Ordering::SeqCst);
            break;
        }
    }
}
fn usartif_get_pdu_status(txpduid: PduIdType) -> Option<UsartIf_PduStatus> {
    for pdu_status in USARTIF_TX_PDUS_STATUS.iter() {
        if pdu_status.pdu_id == txpduid {
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
fn usartif_channel_to_index(channel: UsartNumber) -> Option<usize> {
    match channel {
        UsartNumber::USART1 => Some(0),
        UsartNumber::USART2 => Some(1),
        UsartNumber::USART6 => Some(2),
    }
}
fn usart_set_pdu_active(channel: UsartNumber, txpduid: PduIdType) {
    if let Some(index) = usartif_channel_to_index(channel) {
        USARTIF_ACTIVE_TXPDU_ID[index].store(txpduid, Ordering::SeqCst);
    }
}
fn usartif_reset_pdu_active(channel: UsartNumber) {
    if let Some(index) = usartif_channel_to_index(channel) {
        USARTIF_ACTIVE_TXPDU_ID[index].store(USARTIF_INVALID_PDU_ID, Ordering::SeqCst);
    }
}
fn usartif_get_txpducfg_from_id(txpduid: PduIdType) -> Option<&'static UsartIf_TxPduType> {
    let txpducfgs = get_usartif_txpdu_config();
    for txpducfg in txpducfgs.tx_pdu.iter() {
        if txpducfg.tx_pdu_id == txpduid {
            return Some(txpducfg);
        }
    }
    None
}
fn usartif_det_check(txpduid: PduIdType, pduinfo : *const PduInfoType) -> UsartIf_ReturnType {
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
            let txpducfg = usartif_get_txpducfg_from_id(txpduid);
            if let Some(txpducfg) = txpducfg {
                if unsafe { (*pduinfo).length as u8 } > txpducfg.tx_pdu_length {
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
//Function to transmit a PDU to the lower layer driver
pub fn usartif_transmit(txpduid: PduIdType, txpduinfo: *const PduInfoType) -> UsartIf_ReturnType {
    let det_check_result = usartif_det_check(txpduid, txpduinfo);
    if det_check_result == UsartIf_ReturnType::USARTIF_NOT_OK {
        // Handle DET error reporting here
        return UsartIf_ReturnType::USARTIF_NOT_OK;
    }
    let txpducfg = usartif_get_txpducfg_from_id(txpduid).unwrap();
    let tx_status = usart_get_tx_status(txpducfg.lower_channel);
    if tx_status == UsartTxStatus::Busy || tx_status == UsartTxStatus::Error {
        //DET report error: lower layer driver busy or error
        return UsartIf_ReturnType::USARTIF_NOT_OK;
    }
    // Start the asynchronous transmission
    let data = unsafe {
        // Create a slice from the raw pointer and length
        core::slice::from_raw_parts(txpduinfo.as_ref().unwrap().data, txpduinfo.as_ref().unwrap().length as usize)
    };

    
    let current_state = usartif_get_pdu_status(txpduid);
    if current_state == Some(UsartIf_PduStatus::USARTIF_BUSY) || current_state == Some(UsartIf_PduStatus::USARTIF_PENDING) {
        //DET report error: PDU is already in a busy or pending state
        return UsartIf_ReturnType::USARTIF_NOT_OK;
    }
    // Set the PDU as active for the channel and update the status to PENDING
    usartif_set_pdu_status(txpduid, UsartIf_PduStatus::USARTIF_PENDING);
    // Set the confirmation status to NOT_OK initially, as the transmission has not yet completed
    usartif_set_tx_confirmation_status(txpducfg, UsartIf_TxConfirmationStatus::USARTIF_TX_CONFIRMATION_NOT_OK);
    usart_set_pdu_active(txpducfg.lower_channel, txpduid);

    let result = usart_start_send_async(txpducfg.lower_channel, data);
    if result == UsartReturnType::USART_OK {
        usartif_set_pdu_status(txpduid, UsartIf_PduStatus::USARTIF_BUSY);
        UsartIf_ReturnType::USARTIF_OK
    } else {
        //DET report error: lower layer driver failed to start transmission
        usartif_reset_pdu_active(txpducfg.lower_channel);
        // Update the PDU status to ERROR
        usartif_set_pdu_status(txpduid, UsartIf_PduStatus::USARTIF_ERROR);
        UsartIf_ReturnType::USARTIF_NOT_OK
    }
}

/* Confirmation session */
fn usartif_set_tx_confirmation_status(txpdu: &UsartIf_TxPduType, status: UsartIf_TxConfirmationStatus) {
    let index = txpdu.confirmation_index as usize;
    if index < USARTIF_TX_CONFIRMATION_TABLE.len() {
        USARTIF_TX_CONFIRMATION_TABLE[index].store(status as u8, Ordering::SeqCst);
    }
}
fn usartif_tx_confirmation(txpduid: PduIdType){
    // This function is called by the lower layer driver when the transmission is complete
    // You can implement any necessary actions here, such as notifying the upper layer
    // For example, you can log the confirmation or trigger a callback
    // In this example, we simply print a message for demonstration
    let txpdu = usartif_get_txpducfg_from_id(txpduid);
    if let Some(txpdu) = txpdu {
        // Update the confirmation status in the table
        usartif_set_tx_confirmation_status(txpdu, UsartIf_TxConfirmationStatus::USARTIF_TX_CONFIRMATION_OK);  
        // Update the PDU status to COMPLETED 
        usartif_set_pdu_status(txpduid, UsartIf_PduStatus::USARTIF_COMPLETED);
    }
}
pub fn usartif_tx_confirmation_by_channel(channel: UsartNumber) {
    if let Some(index) = usartif_channel_to_index(channel) {
        let txpduid = USARTIF_ACTIVE_TXPDU_ID[index].load(Ordering::SeqCst);

        if txpduid != USARTIF_INVALID_PDU_ID {
            usartif_tx_confirmation(txpduid);
            // Reset the active PDU ID for the channel
            usartif_reset_pdu_active(channel);
        }
    }
}
pub fn usartif_get_tx_confirmation_status(txpduid: PduIdType) -> UsartIf_TxConfirmationStatus {
    let txpdu = usartif_get_txpducfg_from_id(txpduid);
    if let Some(txpdu) = txpdu {
        let index = txpdu.confirmation_index as usize;
        if index < USARTIF_TX_CONFIRMATION_TABLE.len() {
            let status = USARTIF_TX_CONFIRMATION_TABLE[index].load(Ordering::SeqCst);
            if status == UsartIf_TxConfirmationStatus::USARTIF_TX_CONFIRMATION_OK as u8 {
                return UsartIf_TxConfirmationStatus::USARTIF_TX_CONFIRMATION_OK;
            }
        }
    }
    UsartIf_TxConfirmationStatus::USARTIF_TX_CONFIRMATION_NOT_OK    
}
