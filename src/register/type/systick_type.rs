#![allow(dead_code)]
#![allow(non_snake_case)]

#[repr(C)]
pub struct SysTickRegister {
    pub SYST_CSR : u32, // SysTick Control and Status Register
    pub SYST_RVR : u32, // SysTick Reload Value Register
    pub SYST_CVR : u32, // SysTick Current Value Register
    pub SYST_CALIB : u32, // SysTick Calibration Value Register
}
const SYSTICK : *mut SysTickRegister = 0xE000E010 as *mut SysTickRegister;

pub fn get_systick_register() -> &'static mut SysTickRegister {
    unsafe { &mut *SYSTICK }
}


