#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExtiRegister{
    pub imr: u32,
    pub emr: u32,
    pub rtsr: u32,
    pub ftsr: u32,
    pub swier: u32,
    pub pr: u32,
}
const EXTI : *mut ExtiRegister = 0x4001_3C00 as *mut ExtiRegister;
pub fn get_exti_register() -> &'static mut ExtiRegister {
    unsafe { &mut *EXTI }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Exti_TriggerType {
    RISING,
    FALLING,
    RISING_FALLING,
}

