#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

use crate::register::uart_type::{get_usart_register, UsartNumber};
use crate::register::rcc_type::get_rcc_register;
use crate::mcal::mcu::mcu_get_system_clock_hz;

pub fn uart_enable_periheral_clock(usart_number: UsartNumber) {
    let rcc = get_rcc_register();
    match usart_number {
        UsartNumber::USART1 => rcc.rcc_apb2enr |= 1 << 4, // Enable USART1 clock
        UsartNumber::USART2 => rcc.rcc_apb1enr |= 1 << 17, // Enable USART2 clock
        UsartNumber::USART6 => rcc.rcc_apb2enr |= 1 << 5, // Enable USART6 clock
    }
}
pub fn uart_enable(usart_number: UsartNumber) {
    let usart = get_usart_register(usart_number);
    if let Some(usart) = usart {
        unsafe{
            let cr1 = core::ptr::read_volatile(&(*usart).CR1);
            core::ptr::write_volatile(&mut (*usart).CR1, cr1 | (1 << 13));
        }
    }
}

pub fn uart_set_baud_rate(usart_number: UsartNumber, baud_rate: u32) {
    let usart = get_usart_register(usart_number);
    let system_clock_hz = mcu_get_system_clock_hz();
    let brr_value = (system_clock_hz + baud_rate /2) / baud_rate; // Calculate the baud rate register
    if let Some(usart) = usart {
        unsafe{
            core::ptr::write_volatile(&mut (*usart).BRR, brr_value);
        }
        
    }
}

pub fn uart_enable_txrx(usart_number: UsartNumber) {
    let usart = get_usart_register(usart_number);
    if let Some(usart) = usart {
        unsafe{
            core::ptr::write_volatile(&mut (*usart).CR1, core::ptr::read_volatile(&(*usart).CR1) | (1 << 3) | (1 << 2)); // Enable TX and RX
        }
    }
}

pub fn uart_write(usart_number: UsartNumber, byte: u8) {
    let usart = get_usart_register(usart_number);
    if let Some(usart) = usart {
        unsafe{
            // Wait until TXE (Transmit Data Register Empty) flag is set
            while core::ptr::read_volatile(&(*usart).SR) & (1 << 7) == 0 {}
            core::ptr::write_volatile(&mut (*usart).DR, byte as u32);
        }
    }
}

pub fn uart_read(usart_number: UsartNumber) -> u8 {
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

pub fn uart_enable_interrupt(usart_number: UsartNumber) {
    let usart = get_usart_register(usart_number);
    if let Some(usart) = usart {
        unsafe{
            core::ptr::write_volatile(&mut (*usart).CR1, core::ptr::read_volatile(&(*usart).CR1) | (1 << 5)); // Enable RXNE interrupt
        }
    }
}


