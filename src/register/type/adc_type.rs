#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ADCRegister {
    pub SR: u32, // Status Register
    pub CR1: u32, // Control Register 1
    pub CR2: u32, // Control Register 2
    pub SMPR1: u32, // Sample Time Register 1
    pub SMPR2: u32, // Sample Time Register 2
    pub JOFR1: u32, // Injected Channel Data Offset Register 1
    pub JOFR2: u32, // Injected Channel Data Offset Register 2
    pub JOFR3: u32, // Injected Channel Data Offset Register 3
    pub JOFR4: u32, // Injected Channel Data Offset Register 4
    pub HTR: u32, // Watchdog Higher Threshold Register
    pub LTR: u32, // Watchdog Lower Threshold Register
    pub SQR1: u32, // Regular Sequence Register 1
    pub SQR2: u32, // Regular Sequence Register 2
    pub SQR3: u32, // Regular Sequence Register 3
    pub JSQR: u32, // Injected Sequence Register
    pub JDR1: u32, // Injected Data Register 1
    pub JDR2: u32, // Injected Data Register 2
    pub JDR3: u32, // Injected Data Register 3
    pub JDR4: u32, // Injected Data Register 4
    pub DR: u32,   // Regular Data Register
}
const ADC1: *mut ADCRegister = 0x4001_2000 as *mut ADCRegister;

pub fn get_r_adc_register() -> &'static mut ADCRegister {
    unsafe { &mut *ADC1 }
}