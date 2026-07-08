#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UsartRegister{
    pub SR: u32, // Status Register
    pub DR: u32, // Data Register
    pub BRR: u32, // Baud Rate Register
    pub CR1: u32, // Control Register 1
    pub CR2: u32, // Control Register 2
    pub CR3: u32, // Control Register 3
    pub GTPR: u32, // Guard time and prescaler register
}
const USART1: *mut UsartRegister = 0x4001_1000 as *mut UsartRegister;
const USART2: *mut UsartRegister = 0x4000_4400 as *mut UsartRegister;
const USART6: *mut UsartRegister = 0x4001_1400 as *mut UsartRegister;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UsartNumber {
    USART1 = 1,
    USART2 = 2,
    USART6 = 6,
}
pub fn get_usart_register(usart_number: UsartNumber) -> Option<*mut UsartRegister> {
    match usart_number {
        UsartNumber::USART1 => Some(USART1),
        UsartNumber::USART2 => Some(USART2),
        UsartNumber::USART6  => Some(USART6),
    }
}
