#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

use crate::register::usart_type::{get_usart_register, UsartNumber};
use crate::register::rcc_type::get_rcc_register;
use crate::mcal::mcu::mcu_get_system_clock_hz;
use crate::mcal::usart_type::{UsartParityType};
use crate::register::usart_type::UsartRegister;


pub fn usart_enable_periheral_clock(usart_number: UsartNumber) {
    let rcc = get_rcc_register();
    match usart_number {
        UsartNumber::USART1 => rcc.rcc_apb2enr |= 1 << 4, // Enable USART1 clock
        UsartNumber::USART2 => rcc.rcc_apb1enr |= 1 << 17, // Enable USART2 clock
        UsartNumber::USART6 => rcc.rcc_apb2enr |= 1 << 5, // Enable USART6 clock
    }
}
pub fn usart_enable(usart_number: UsartNumber) {
    let usart = get_usart_register(usart_number);
    if let Some(usart) = usart {
        unsafe{
            let cr1 = core::ptr::read_volatile(&(*usart).CR1);
            core::ptr::write_volatile(&mut (*usart).CR1, cr1 | (1 << 13));
        }
    }
}

pub fn usart_set_baud_rate(usart_number: UsartNumber, baud_rate: u32) {
    let usart = get_usart_register(usart_number);
    let system_clock_hz = mcu_get_system_clock_hz();
    let brr_value = (system_clock_hz + baud_rate /2) / baud_rate; // Calculate the baud rate register
    if let Some(usart) = usart {
        unsafe{
            core::ptr::write_volatile(&mut (*usart).BRR, brr_value);
        }
        
    }
}

pub fn usart_enable_txrx(usart_number: UsartNumber) {
    let usart = get_usart_register(usart_number);
    if let Some(usart) = usart {
        unsafe{
            core::ptr::write_volatile(&mut (*usart).CR1, core::ptr::read_volatile(&(*usart).CR1) | (1 << 3) | (1 << 2)); // Enable TX and RX
        }
    }
}

pub fn usart_write(usart_number: UsartNumber, byte: u8) {
    let usart = get_usart_register(usart_number);
    if let Some(usart) = usart {
        unsafe{
            // Wait until TXE (Transmit Data Register Empty) flag is set
            while core::ptr::read_volatile(&(*usart).SR) & (1 << 7) == 0 {}
            core::ptr::write_volatile(&mut (*usart).DR, byte as u32);
        }
    }
}

pub fn usart_write_direct(usart_number: UsartNumber, byte: u8) {
    let usart = get_usart_register(usart_number);
    if let Some(usart) = usart {
        unsafe{
            core::ptr::write_volatile(&mut (*usart).DR, byte as u32);
        }
    }
}

pub fn usart_read(usart_number: UsartNumber) -> u8 {
    let usart = get_usart_register(usart_number);
    if let Some(usart) = usart {
        unsafe{
            // Wait until RXNE (Read Data Register Not Empty) flag is set
            while core::ptr::read_volatile(&(*usart).SR) & (1 << 5) == 0 {}
            return core::ptr::read_volatile(&(*usart).DR) as u8;
        }
    }
    0
}
pub fn usart_read_direct(usart_number: UsartNumber) -> u8 {
    let usart = get_usart_register(usart_number);
    if let Some(usart) = usart {
        unsafe{
            return core::ptr::read_volatile(&(*usart).DR) as u8;
        }
    }
    0
}
pub fn usart_enable_rx_interrupt(usart_reg: *mut UsartRegister) {
    unsafe{
        core::ptr::write_volatile(&mut (*usart_reg).CR1, core::ptr::read_volatile(&(*usart_reg).CR1) | (1 << 5)); // Enable RXNE interrupt
    }
}

pub fn usart_enable_tx_interrupt(usart_reg: *mut UsartRegister) {
    unsafe{
        core::ptr::write_volatile(&mut (*usart_reg).CR1, core::ptr::read_volatile(&(*usart_reg).CR1) | (1 << 7)); // Enable TXE interrupt
    }
}

pub fn usart_disable_rx_interrupt(usart_reg: *mut UsartRegister) {
    unsafe{
        core::ptr::write_volatile(&mut (*usart_reg).CR1, core::ptr::read_volatile(&(*usart_reg).CR1) & !(1 << 5)); // Disable RXNE interrupt
    }
}

pub fn usart_disable_tx_interrupt(usart_reg: *mut UsartRegister) {
    unsafe{
        core::ptr::write_volatile(&mut (*usart_reg).CR1, core::ptr::read_volatile(&(*usart_reg).CR1) & !(1 << 7)); // Disable TXE interrupt
    }
}
pub fn usart_tx_buffer_is_full(usart_reg: *mut UsartRegister) -> bool {
    let sr = unsafe { core::ptr::read_volatile(&(*usart_reg).SR) };
    (sr & (1 << 7)) == 0 // TXE flag is set
}

pub fn usart_rx_buffer_is_empty(usart_reg: *mut UsartRegister) -> bool {
    let sr = unsafe { core::ptr::read_volatile(&(*usart_reg).SR) };
    (sr & (1 << 5)) == 0 // RXNE flag is not set
}

pub fn usart_tx_complete(usart_reg: *mut UsartRegister) -> bool {
    let sr = unsafe { core::ptr::read_volatile(&(*usart_reg).SR) };
    (sr & (1 << 6)) != 0 // TC flag is set
}

pub fn usart_frame_error(usart_reg: *mut UsartRegister) -> bool {
    let sr = unsafe { core::ptr::read_volatile(&(*usart_reg).SR) };
    (sr & (1 << 1)) != 0 // FE flag
}

pub fn usart_overrun_error(usart_reg: *mut UsartRegister) -> bool {
    let sr = unsafe { core::ptr::read_volatile(&(*usart_reg).SR) };
    (sr & (1 << 3)) != 0 // ORE flag
}

pub fn usart_has_error(usart_reg: *mut UsartRegister) -> bool {
    usart_frame_error(usart_reg) || usart_overrun_error(usart_reg)
}

pub fn usart_clear_error_flags(usart_reg: *mut UsartRegister) {
    unsafe {
        let _ = core::ptr::read_volatile(&(*usart_reg).SR);
        let _ = core::ptr::read_volatile(&(*usart_reg).DR);
    }
}
pub fn usart_set_parity(usart_number: UsartNumber, parity: UsartParityType) {
    let usart = get_usart_register(usart_number);
    if let Some(usart) = usart {
        unsafe{
            let mut cr1 = core::ptr::read_volatile(&(*usart).CR1);
            cr1 &= !((1 << 10) | (1 << 9)); // Clear PCE and PS bits
            match parity {
                UsartParityType::None => { /* No parity, do nothing */ }
                UsartParityType::Even => { cr1 |= 1 << 10; } // Even parity PCE = 1, PS = 0
                UsartParityType::Odd => { cr1 |= (1 << 10) | (1 << 9); } // Odd parity PCE = 1, PS = 1
            }
            core::ptr::write_volatile(&mut (*usart).CR1, cr1);
        }
    }
}

pub fn usart_enable_tc_interrupt(usart_reg: *mut UsartRegister) {
    unsafe {
        let cr1 = core::ptr::read_volatile(&(*usart_reg).CR1);
        core::ptr::write_volatile(&mut (*usart_reg).CR1, cr1 | (1 << 6)); // TCIE
    }
}

pub fn usart_disable_tc_interrupt(usart_reg: *mut UsartRegister) {
    unsafe {
        let cr1 = core::ptr::read_volatile(&(*usart_reg).CR1);
        core::ptr::write_volatile(&mut (*usart_reg).CR1, cr1 & !(1 << 6)); // TCIE
    }
}
pub fn usart_clear_tc_flag(usart_reg: *mut UsartRegister) {
    unsafe {
        let sr = core::ptr::read_volatile(&(*usart_reg).SR);
        core::ptr::write_volatile(&mut (*usart_reg).SR, sr & !(1 << 6)); // Clear TC flag
    }
}
