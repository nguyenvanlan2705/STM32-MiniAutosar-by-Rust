use crate::register::uart::{uart_enable_periheral_clock,
    uart_enable, uart_set_baud_rate, uart_enable_txrx};
use crate::register::uart_type::{UsartNumber};

pub fn uart_init(usart_number: UsartNumber, baud_rate: u32) {
    uart_enable_periheral_clock(usart_number);
    uart_set_baud_rate(usart_number, baud_rate);
    uart_enable_txrx(usart_number);
    uart_enable(usart_number);
}
