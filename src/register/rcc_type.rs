#![allow(dead_code)]

#[repr(C)]
pub struct RCCRegister{
    pub rcc_cr : u32,
    pub rcc_pllcfgr : u32,
    pub rcc_cfgr : u32,
    reserved1: [u32;9],
    pub rcc_ahb1enr : u32,
    pub rcc_ahb2enr : u32,
    reserved2: [u32;2],
    pub rcc_apb1enr : u32,
    pub rcc_apb2enr : u32,
}
const RCC: *mut RCCRegister = 0x4002_3800 as *mut RCCRegister;
pub fn get_rcc_register() -> &'static mut RCCRegister {
    unsafe { &mut *RCC }
}
#[repr(u32)]
pub enum CR {
    HSION = 0,
    HSIRDY = 1,
    HSITRIM = 3,
    HSICAL = 8,
    HSEON = 16,
    HSERDY = 17,
    HSEBYP = 18,
    CSSON = 19,
    PLLON = 24,
    PLLRDY = 25,
}


