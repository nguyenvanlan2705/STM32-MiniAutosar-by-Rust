#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use crate::register::usart_type::UsartNumber;
use crate::register::nvic_type::IRQn;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]    
pub enum UsartRxStatus {
    Idle,
    Busy,
    Error,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]    
pub enum UsartTxStatus {
    Idle,
    Busy,
    Completed,
    Error,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]    
pub enum UsartReturnType {
    USART_OK,
    USART_BUSY,
    USART_NOT_OK,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]    
pub enum UsartParityType {
    None,
    Even,
    Odd,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]  
pub enum UsartTxRxMode {
    POLLING,
    INTERRUPT,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Usart_ChannelConfigType  {
    pub usart_number: UsartNumber,
    pub baud_rate: u32,
    pub parity: UsartParityType,
    pub mode : UsartTxRxMode,
    pub irq_line: IRQn,
}
pub struct Usart_ConfigType {
    pub channels: &'static [Usart_ChannelConfigType],
}

