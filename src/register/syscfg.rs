use crate::register::syscfg_type::{get_syscfg_register, EXTILINE};
use crate::register::rcc_type::{ get_rcc_register};
use crate::register::gpio_type::{PORT};

pub fn enable_syscfg_clock() {
    unsafe {
        let rcc = get_rcc_register();
        let shift_value = core::ptr::read_volatile(&(*rcc).rcc_apb2enr) | (1 << 14);
        core::ptr::write_volatile(&mut (*rcc).rcc_apb2enr, shift_value);
    }
}
pub fn configure_exti_line(exti_line: EXTILINE, port_value: PORT) {
    unsafe {
        let syscfg = get_syscfg_register();
        let index = (exti_line as usize) >> 2;
        let shift = ((exti_line as usize) & 0x03) << 2;
        let current_value = core::ptr::read_volatile(&(*syscfg).exticr[index]);
        let cleared_value = current_value & !(0xF << shift);  
        let new_value = cleared_value | ((port_value as u32) << shift);
        core::ptr::write_volatile(&mut (*syscfg).exticr[index], new_value);
    }
}
