#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#[repr(C)]
pub struct NVICRegister {
    pub iser: [u32; 8], // Interrupt Set-Enable Registers
    pub icer: [u32; 8], // Interrupt Clear-Enable Registers
    pub ispr: [u32; 8], // Interrupt Set-Pending Registers
    pub icpr: [u32; 8], // Interrupt Clear-Pending Registers
    pub iabr: [u32; 8], // Interrupt Active Bit Registers
    pub ipr: [u32; 60], // Interrupt Priority Registers
}
const NVIC: *mut NVICRegister = 0xE000_E100 as *mut NVICRegister;
pub fn get_nvic_register() -> &'static mut NVICRegister {
    unsafe { &mut *NVIC }
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQn{
    WWDG = 0, // Window WatchDog Interrupt
    PVD = 1, // PVD through EXTI Line detection Interrupt
    TAMP_STAMP = 2, // Tamper and TimeStamp interrupts through the EXTI line
    RTC_WKUP = 3,   // RTC Wakeup interrupt through the EXTI line
    FLASH = 4,  // FLASH global Interrupt
    RCC = 5,    // RCC global Interrupt
    EXTI0 = 6, // EXTI Line0 Interrupt
    EXTI1 = 7, // EXTI Line1 Interrupt
    EXTI2 = 8, // EXTI Line2 Interrupt
    EXTI3 = 9, // EXTI Line3 Interrupt
    EXTI4 = 10, // EXTI Line4 Interrupt
    DMA1_Stream0 = 11, // DMA1 Stream0 Interrupt
    DMA1_Stream1 = 12, // DMA1 Stream1 Interrupt
    DMA1_Stream2 = 13, // DMA1 Stream2 Interrupt
    DMA1_Stream3 = 14, // DMA1 Stream3 Interrupt
    DMA1_Stream4 = 15, // DMA1 Stream4 Interrupt
    DMA1_Stream5 = 16, // DMA1 Stream5 Interrupt
    DMA1_Stream6 = 17, // DMA1 Stream6 Interrupt
    ADC = 18, // ADC1, ADC2 and ADC3 global Interrupts
    EXTI9_5 = 23, // External Line[9:5] Interrupts
    TIM1_BRK_TIM9 = 24, // TIM1 Break interrupt and TIM9
    TIM1_UP_TIM10 = 25, // TIM1 Update Interrupt and TIM10
    TIM1_TRG_COM_TIM11 = 26, // TIM1 Trigger and Commutation Interrupt and TIM11
    TIM1_CC = 27, // TIM1 Capture Compare Interrupt
    TIM2 = 28, // TIM2 global Interrupt
    TIM3 = 29, // TIM3 global Interrupt
    TIM4 = 30, // TIM4 global Interrupt
    I2C1_EV = 31, // I2C1 Event Interrupt
    I2C1_ER = 32, // I2C1 Error Interrupt
    I2C2_EV = 33, // I2C2 Event Interrupt
    I2C2_ER = 34, // I2C2 Error Interrupt
    SPI1 = 35, // SPI1 global Interrupt
    SPI2 = 36, // SPI2 global Interrupt
    USART1 = 37, // USART1 global Interrupt
    USART2 = 38, // USART2 global Interrupt
    Reserved = 39, // USART6 global Interrupt
    EXTI15_10 = 40, // External Line[15:10] Interrupts
    RTC_Alarm = 41, // RTC Alarm (A and B) through EXTI
    OTG_FS_WKUP = 42, // USB On-The-Go FS Wakeup through EXTI line interrupt
    USART6 = 71, // USART6 global Interrupt
}