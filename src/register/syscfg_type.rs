#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Syscfg_Register {
    pub memrmp: u32,
    pub pmc: u32,
    pub exticr: [u32; 4],
    pub cmpcr: u32,
}
const SYSCFG : *mut Syscfg_Register = 0x4001_3800 as *mut Syscfg_Register;
pub fn get_syscfg_register() -> &'static mut Syscfg_Register {
    unsafe { &mut *SYSCFG }
}
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTILINE {
    LINE0 = 0,
    LINE1 = 1,
    LINE2 = 2,
    LINE3 = 3,
    LINE4 = 4,
    LINE5 = 5,
    LINE6 = 6,
    LINE7 = 7,
    LINE8 = 8,
    LINE9 = 9,
    LINE10 = 10,
    LINE11 = 11,
    LINE12 = 12,
    LINE13 = 13,
    LINE14 = 14,
    LINE15 = 15,
}