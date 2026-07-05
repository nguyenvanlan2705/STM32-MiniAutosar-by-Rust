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
#[derive(Clone, Copy)]
pub enum IRQn {
    EXTI0 = 6,
    EXTI1 = 7,
    EXTI2 = 8,
    EXTI3 = 9,
    EXTI4 = 10,
    EXTI9_5 = 23,
    EXTI15_10 = 40,
}