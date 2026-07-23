#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::mcal::usart_type::{UsartReturnType, UsartRxStatus, UsartTxStatus, Usart_ChannelConfigType,
                                 UsartTxRxMode, UsartParityType};
use crate::register::nvic::{nvic_enable_irq};
use crate::register::usart::{
    usart_disable_rx_interrupt, usart_disable_tx_interrupt, usart_enable,
    usart_enable_periheral_clock, usart_enable_rx_interrupt, usart_enable_tx_interrupt,
    usart_enable_txrx, usart_read, usart_read_direct, usart_rx_buffer_is_empty,
    usart_set_baud_rate, usart_tx_buffer_is_full, usart_write, usart_write_direct, usart_tx_complete, usart_set_parity,
    usart_enable_tc_interrupt, usart_disable_tc_interrupt, usart_clear_tc_flag,
    usart_has_error, usart_clear_error_flags
};
use crate::register::usart_type::{get_usart_register, UsartNumber};
use core::sync::atomic::{AtomicBool, AtomicU8, AtomicUsize, Ordering};
use crate::mcal::cfg::usart_cfg::{USART_CHANNEL_CONFIG};
use crate::bsw::usartif::usartif_tx::{usartif_tx_confirmation_by_channel};


// Track initialization status for USART1, USART2, and USART6
pub static USART_CHANNELS_INITIALIZED: [AtomicBool; 3] = [const { AtomicBool::new(false) }; 3]; 
static USART1_TX_BUFFER: [AtomicU8; 128] = [const { AtomicU8::new(0) }; 128];
static USART1_RX_BUFFER: [AtomicU8; 128] = [const { AtomicU8::new(0) }; 128];
static USART2_TX_BUFFER: [AtomicU8; 128] = [const { AtomicU8::new(0) }; 128];
static USART2_RX_BUFFER: [AtomicU8; 128] = [const { AtomicU8::new(0) }; 128];
static USART6_TX_BUFFER: [AtomicU8; 128] = [const { AtomicU8::new(0) }; 128];
static USART6_RX_BUFFER: [AtomicU8; 128] = [const { AtomicU8::new(0) }; 128];

static USART_TX_LEN: [AtomicUsize; 3] = [const { AtomicUsize::new(0) }; 3];
static USART_TX_INDEX: [AtomicUsize; 3] = [const { AtomicUsize::new(0) }; 3];
static USART_TX_BUSY: [AtomicBool; 3] = [const { AtomicBool::new(false) }; 3];
static USART_RX_BUSY: [AtomicBool; 3] = [const { AtomicBool::new(false) }; 3];
static USART_TX_DONE: [AtomicBool; 3] = [const { AtomicBool::new(false) }; 3];
static USART_ERROR: [AtomicBool; 3] = [const { AtomicBool::new(false) }; 3];

// This array is used to track the head and tail indices for each USART channel's TX and RX buffers. It helps manage the circular buffer behavior for asynchronous communication.
static USART_HEAD : [AtomicUsize; 3] = [const { AtomicUsize::new(0) }; 3];
static USART_TAIL : [AtomicUsize; 3] = [const { AtomicUsize::new(0) }; 3];

/* Init */
fn usart_channel_to_index(usart_number: UsartNumber) -> usize {
    match usart_number {
        UsartNumber::USART1 => 0,
        UsartNumber::USART2 => 1,
        UsartNumber::USART6 => 2,
    }
}
pub fn usart_init() {
    for channel in USART_CHANNEL_CONFIG.channels {
        let index = usart_channel_to_index(channel.usart_number);
        if !USART_CHANNELS_INITIALIZED[index].load(Ordering::SeqCst) {
            usart_enable_periheral_clock(channel.usart_number);
            if channel.baud_rate != 0 {
                usart_set_baud_rate(channel.usart_number, channel.baud_rate);
            }
            else{
                //DET report error: baud rate is zero
            }
            usart_enable_txrx(channel.usart_number);
            usart_enable(channel.usart_number);
            if channel.parity != UsartParityType::None {
                usart_set_parity(channel.usart_number, channel.parity);
            }
            if channel.mode == UsartTxRxMode::INTERRUPT {
                usart_init_interrupt(channel);
            }
            usart_rx_ring_clear(channel.usart_number);
            usart_set_error(channel.usart_number, false);
            USART_CHANNELS_INITIALIZED[index].store(true, Ordering::SeqCst);
        }else{
            // Channel already initialized, do nothing
        }
    }
}

pub fn usart_init_interrupt(config: &Usart_ChannelConfigType) {
    nvic_enable_irq(config.irq_line);
}

/* Polling TX/RX */
pub fn usart_write_bytes(usart_number: UsartNumber, data: &[u8]) -> UsartReturnType {
    if data.is_empty() {
        return UsartReturnType::USART_NOT_OK;
    }
    for i in 0..data.len() {
        let byte = data[i];
        usart_write(usart_number, byte);
    }
    if let Some(usart) = get_usart_register(usart_number) {
        while !usart_tx_complete(usart) {}
        // Transmission complete
        //USARTIf_TxConfirmation(); // Call the confirmation callback
        UsartReturnType::USART_OK
    } else {
        UsartReturnType::USART_NOT_OK
    }
}
pub fn usart_write_string(usart_number: UsartNumber, data: &str) -> UsartReturnType {
    if data.is_empty() {
        return UsartReturnType::USART_NOT_OK;
    }
    for byte in data.bytes() {
        usart_write(usart_number, byte as u8);
    }
    if let Some(usart) = get_usart_register(usart_number) {
        while !usart_tx_complete(usart) {}
        //USARTIf_TxConfirmation(); // Call the confirmation callback
        UsartReturnType::USART_OK
    } else {
        UsartReturnType::USART_NOT_OK
    }
}
pub fn usart_read_bytes(usart_number: UsartNumber, data: &mut [u8]) -> UsartReturnType {
    if data.is_empty() {
        return UsartReturnType::USART_NOT_OK;
    }
    for i in 0..data.len() {
        data[i] = usart_read(usart_number);
    }
    UsartReturnType::USART_OK
}
pub fn usart_read_string(usart_number: UsartNumber, buffer: &mut [u8]) -> usize {
    let mut index = 0;
    while index < buffer.len() {
        let byte = usart_read(usart_number);
        if byte != b'\n' && byte != b'\r' {
            buffer[index] = byte;
            index += 1;
        } else {
            break; // Stop reading on newline or carriage return
        }
    }
    index // Return the number of bytes read
}

/* Direct TX/RX */
pub fn usart_write_bytes_direct(usart_number: UsartNumber, data: &[u8]) {
    for i in 0..data.len() {
        let byte = data[i];
        usart_write_direct(usart_number, byte);
    }
}
pub fn usart_write_string_direct(usart_number: UsartNumber, data: &str) {
    for byte in data.bytes() {
        usart_write_direct(usart_number, byte as u8);
    }
}
pub fn usart_read_bytes_direct(usart_number: UsartNumber, data: &mut [u8]) {
    for i in 0..data.len() {
        data[i] = usart_read_direct(usart_number);
    }
}
pub fn usart_read_string_direct(usart_number: UsartNumber, buffer: &mut [u8]) -> usize {
    let mut index = 0;
    while index < buffer.len() {
        let byte = usart_read_direct(usart_number);
        if byte != b'\n' && byte != b'\r' {
            buffer[index] = byte;
            index += 1;
        } else {
            break; // Stop reading on newline or carriage return
        }
    }
    index // Return the number of bytes read
}

/* Non-blocking single-byte TX/RX */
pub fn usart_read_byte_non_blocking(usart_number: UsartNumber) -> Option<u8> {
    let usart = get_usart_register(usart_number);
    if let Some(usart) = usart {
        if !usart_rx_buffer_is_empty(usart) {
            Some(usart_read_direct(usart_number))
        } else {
            None
        }
    } else {
        None
    }
}

pub fn usart_write_byte_non_blocking(usart_number: UsartNumber, byte: u8) -> UsartReturnType {
    let usart = get_usart_register(usart_number);
    if let Some(usart) = usart {
        if !usart_tx_buffer_is_full(usart) {
            usart_write_direct(usart_number, byte);
            UsartReturnType::USART_OK
        } else {
            UsartReturnType::USART_NOT_OK
        }
    } else {
        UsartReturnType::USART_NOT_OK
    }
}

/* Helper functions for USART state management */
fn usart_tx_set_len(usart_number: UsartNumber, len: usize) {
    let index = usart_channel_to_index(usart_number);
    USART_TX_LEN[index].store(len, Ordering::SeqCst);
}
fn usart_tx_set_index(usart_number: UsartNumber, index: usize) {
    let channel_index = usart_channel_to_index(usart_number);
    USART_TX_INDEX[channel_index].store(index, Ordering::SeqCst);
}
fn usart_set_busy(usart_number: UsartNumber, busy: bool, is_tx: bool) {
    let channel_index = usart_channel_to_index(usart_number);
    if is_tx {
        USART_TX_BUSY[channel_index].store(busy, Ordering::SeqCst);
    } else {
        USART_RX_BUSY[channel_index].store(busy, Ordering::SeqCst);
    }
}
fn usart_tx_set_done(usart_number: UsartNumber, done: bool) {
    let channel_index = usart_channel_to_index(usart_number);
    USART_TX_DONE[channel_index].store(done, Ordering::SeqCst);
}

fn usart_tx_get_len(usart_number: UsartNumber) -> usize {
    let channel_index = usart_channel_to_index(usart_number);
    USART_TX_LEN[channel_index].load(Ordering::SeqCst)
}

fn usart_tx_get_index(usart_number: UsartNumber) -> usize {
    let channel_index = usart_channel_to_index(usart_number);
    USART_TX_INDEX[channel_index].load(Ordering::SeqCst)
}
fn usart_get_busy(usart_number: UsartNumber, is_tx: bool) -> bool {
    let channel_index = usart_channel_to_index(usart_number);
    if is_tx {
        USART_TX_BUSY[channel_index].load(Ordering::SeqCst)
    } else {
        USART_RX_BUSY[channel_index].load(Ordering::SeqCst)
    }
}
fn usart_tx_get_done(usart_number: UsartNumber) -> bool {
    let channel_index = usart_channel_to_index(usart_number);
    USART_TX_DONE[channel_index].load(Ordering::SeqCst)
}

fn usart_set_error(usart_number: UsartNumber, error: bool) {
    let channel_index = usart_channel_to_index(usart_number);
    USART_ERROR[channel_index].store(error, Ordering::SeqCst);
}
fn usart_get_error(usart_number: UsartNumber) -> bool {
    let channel_index = usart_channel_to_index(usart_number);
    USART_ERROR[channel_index].load(Ordering::SeqCst)
}
fn usart_tx_set_data_to_channel(usart_number: UsartNumber, index: usize, byte: u8) {
    let channel_index = usart_channel_to_index(usart_number);
    if channel_index == 0 {
        USART1_TX_BUFFER[index].store(byte, Ordering::SeqCst);
    } else if channel_index == 1 {
        USART2_TX_BUFFER[index].store(byte, Ordering::SeqCst)
    } else {
        USART6_TX_BUFFER[index].store(byte, Ordering::SeqCst)
    }
}
fn usart_get_rx_data_from_channel(usart_number: UsartNumber, index: usize) -> u8 {
    let channel_index = usart_channel_to_index(usart_number);
    if channel_index == 0 {
        USART1_RX_BUFFER[index].load(Ordering::SeqCst)
    } else if channel_index == 1 {
        USART2_RX_BUFFER[index].load(Ordering::SeqCst)
    } else {
        USART6_RX_BUFFER[index].load(Ordering::SeqCst)
    }
}

fn usart_set_rx_data_to_channel(usart_number: UsartNumber, index: usize, byte: u8) {
    let channel_index = usart_channel_to_index(usart_number);
    if channel_index == 0 {
        USART1_RX_BUFFER[index].store(byte, Ordering::SeqCst);
    } else if channel_index == 1 {
        USART2_RX_BUFFER[index].store(byte, Ordering::SeqCst)
    } else {
        USART6_RX_BUFFER[index].store(byte, Ordering::SeqCst)
    }
}
fn usart_get_tx_data_from_channel(usart_number: UsartNumber, index: usize) -> u8 {
    let channel_index = usart_channel_to_index(usart_number);
    if channel_index == 0 {
        USART1_TX_BUFFER[index].load(Ordering::SeqCst)
    } else if channel_index == 1 {
        USART2_TX_BUFFER[index].load(Ordering::SeqCst)
    } else {
        USART6_TX_BUFFER[index].load(Ordering::SeqCst)
    }
}
/* Async TX */
fn usart_start_transmit_async(usart_number: UsartNumber, len: usize) -> UsartReturnType {
    let channel_index = usart_channel_to_index(usart_number);
    if len == 0 {
        return UsartReturnType::USART_NOT_OK;
    }
    if USART_TX_BUSY[channel_index].load(Ordering::SeqCst) {
        return UsartReturnType::USART_BUSY;
    }

    usart_tx_set_index(usart_number, 0);
    usart_tx_set_len(usart_number, len);
    usart_set_busy(usart_number, true, true);
    usart_tx_set_done(usart_number, false);
    UsartReturnType::USART_OK
}

fn usart_send_async(usart_number: UsartNumber, data: &[u8]) -> UsartReturnType {
    let usart_reg = get_usart_register(usart_number);
    if let Some(usart) = usart_reg {
        if data.len() > USART2_TX_BUFFER.len() {
            return UsartReturnType::USART_NOT_OK; // Data length exceeds buffer size
        }
        for index in 0..data.len() {
            usart_tx_set_data_to_channel(usart_number, index, data[index]);
        }
        usart_enable_tx_interrupt(usart);
        UsartReturnType::USART_OK
    } else {
        UsartReturnType::USART_NOT_OK
    }
}

pub fn usart_start_send_async(usart_number: UsartNumber, data: &[u8]) -> UsartReturnType {
    if data.is_empty() || data.len() > USART2_TX_BUFFER.len() {
        return UsartReturnType::USART_NOT_OK;
    }
    if usart_get_error(usart_number) {
        return UsartReturnType::USART_NOT_OK;
    }
    let usart = get_usart_register(usart_number);
    if let Some(usart) = usart {
        usart_clear_tc_flag(usart); // Clear the TC flag to avoid false triggers
    }
    let st = usart_start_transmit_async(usart_number, data.len());
    if st == UsartReturnType::USART_OK {
        usart_send_async(usart_number, data)
    } else {
        st
    }
}

pub fn usart_get_tx_status(usart_number: UsartNumber) -> UsartTxStatus {
    if usart_get_error(usart_number) {
        UsartTxStatus::Error
    } else if usart_get_busy(usart_number, true) {
        UsartTxStatus::Busy
    } else if usart_tx_get_done(usart_number) {
        UsartTxStatus::Completed
    } else {
        UsartTxStatus::Idle
    }
}

/* Async RX */
pub fn usart_start_receive_async(usart_number: UsartNumber, len: usize) -> UsartReturnType {
    let usart = get_usart_register(usart_number);
    if let Some(usart) = usart {
        if usart_get_error(usart_number) {
            return UsartReturnType::USART_NOT_OK;
        }
        if usart_get_busy(usart_number, false) {
            return UsartReturnType::USART_BUSY;
        }
        if len == 0 || len > USART2_RX_BUFFER.len() {
            return UsartReturnType::USART_NOT_OK;
        }
        usart_set_busy(usart_number, true, false);
        usart_enable_rx_interrupt(usart);
        UsartReturnType::USART_OK
    } else {
        UsartReturnType::USART_NOT_OK
    }
}

pub fn usart_get_rx_status(usart_number: UsartNumber) -> UsartRxStatus {
    if usart_get_error(usart_number) {
        UsartRxStatus::Error
    } else if usart_get_busy(usart_number, false) {
        UsartRxStatus::Busy
    } else {
        UsartRxStatus::Idle
    }
}

pub fn usart_get_error_status(usart_number: UsartNumber) -> bool {
    usart_get_error(usart_number)
}

pub fn usart_clear_error_status(usart_number: UsartNumber) {
    if let Some(usart) = get_usart_register(usart_number) {
        usart_clear_error_flags(usart);
    }
    usart_set_error(usart_number, false);
}

fn usart_get_rx_buffer_length(usart_number: UsartNumber) -> usize {
    match usart_number {
        UsartNumber::USART1 => USART1_RX_BUFFER.len(),
        UsartNumber::USART2 => USART2_RX_BUFFER.len(),
        UsartNumber::USART6 => USART6_RX_BUFFER.len(),
    }
}

/// Ring buffer management for RX
fn usart_rx_ring_push(usart_number: UsartNumber, byte: u8) {
    let channel_index = usart_channel_to_index(usart_number);
    let head = USART_HEAD[channel_index].load(Ordering::SeqCst);
    let tail = USART_TAIL[channel_index].load(Ordering::SeqCst);
    let rx_buffer_length = usart_get_rx_buffer_length(usart_number);
    let next_head = (head + 1) % rx_buffer_length;
    if next_head != tail {
        // There is space in the buffer
        usart_set_rx_data_to_channel(usart_number, head, byte);
        USART_HEAD[channel_index].store(next_head, Ordering::SeqCst);
    } else {
        // Buffer overflow, handle error if needed
        usart_set_error(usart_number, true);
    }
}
pub fn usart_rx_ring_pop(usart_number: UsartNumber) -> Option<u8> {
    let channel_index = usart_channel_to_index(usart_number);
    let head = USART_HEAD[channel_index].load(Ordering::SeqCst);
    let tail = USART_TAIL[channel_index].load(Ordering::SeqCst);
    if head == tail {
        // Buffer is empty
        None
    } else {
        let byte = usart_get_rx_data_from_channel(usart_number, tail);
        let next_tail = (tail + 1) % usart_get_rx_buffer_length(usart_number);
        USART_TAIL[channel_index].store(next_tail, Ordering::SeqCst);
        Some(byte)
    }
}
pub fn usart_rx_ring_is_empty(usart_number: UsartNumber) -> bool {
    let channel_index = usart_channel_to_index(usart_number);
    let head = USART_HEAD[channel_index].load(Ordering::SeqCst);
    let tail = USART_TAIL[channel_index].load(Ordering::SeqCst);
    head == tail
}
pub fn usart_rx_ring_clear(usart_number: UsartNumber) {
    let channel_index = usart_channel_to_index(usart_number);
    USART_HEAD[channel_index].store(0, Ordering::SeqCst);
    USART_TAIL[channel_index].store(0, Ordering::SeqCst);
}
/* TX confirmation */
pub fn usart_get_tx_complete_status(usart_number: UsartNumber) -> UsartReturnType {
    let usart = get_usart_register(usart_number);
    if let Some(usart) = usart {
        let st = usart_tx_complete(usart);
        if st {
            UsartReturnType::USART_OK
        } else {
            UsartReturnType::USART_NOT_OK
        }
    } else {
        UsartReturnType::USART_NOT_OK
    }
}
/* Interrupt handler */
pub fn usart_irq_handler(usart_number: UsartNumber) {
    let usart = get_usart_register(usart_number);
    if let Some(usart) = usart {
        let iserror = usart_has_error(usart);
        if iserror {
            usart_set_error(usart_number, true);
            usart_disable_rx_interrupt(usart);
            usart_clear_error_flags(usart);
            return;
        }
        if !usart_rx_buffer_is_empty(usart) { // RXNE flag
            let byte = usart_read_direct(usart_number);
            usart_rx_ring_push(usart_number, byte);
        }
        // Handle TXE (Transmit Data Register Empty) interrupt
        if !usart_tx_buffer_is_full(usart) && usart_get_busy(usart_number, true) { // TXE flag
            // Handle the transmit event (e.g., send the next byte from a buffer)
            let index = usart_tx_get_index(usart_number);
            if index < usart_tx_get_len(usart_number) {
                let byte = usart_get_tx_data_from_channel(usart_number, index);
                usart_write_direct(usart_number, byte);
                let next_index = usart_tx_get_index(usart_number) + 1;
                usart_tx_set_index(usart_number, next_index);
                if next_index >= usart_tx_get_len(usart_number) {
                    // All bytes have been sent, disable TX interrupt
                    usart_disable_tx_interrupt(usart);
                    // Optionally, you can set a flag to indicate that transmission is complete
                    usart_enable_tc_interrupt(usart);  // enable TCIE
                }

            }
        }
        // Handle TC (Transmission Complete) interrupt
        if usart_tx_complete(usart) && usart_get_busy(usart_number, true) {
            usart_disable_tc_interrupt(usart); // Disable TC interrupt
            USART_TX_BUSY[usart_channel_to_index(usart_number)].store(false, Ordering::SeqCst); // Transmission complete
            USART_TX_DONE[usart_channel_to_index(usart_number)].store(true, Ordering::SeqCst);
            usartif_tx_confirmation_by_channel(usart_number); // Call the confirmation callback
        }
    }
}
