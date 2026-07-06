use crate::register::clock_type::{
    ClockResourceType,
};
use crate::register::rcc_type;

pub fn enable_hsi(){
    unsafe {
        let rcc = rcc_type::get_rcc_register();
        let shift_value = core::ptr::read_volatile(&(*rcc).rcc_cr) | (1 << rcc_type::CR::HSION as u32);
        core::ptr::write_volatile(&mut (*rcc).rcc_cr, shift_value);
        while (core::ptr::read_volatile(&(*rcc).rcc_cr)
            & (1 << rcc_type::CR::HSIRDY as u32))
            == 0
        {}
    }
}
pub fn get_clock_resource() -> ClockResourceType {
    unsafe {
        let rcc = rcc_type::get_rcc_register();
        let clock_source = core::ptr::read_volatile(&(*rcc).rcc_cfgr) & 0b11;
        match clock_source {
            0b00 => ClockResourceType::HSI,
            0b01 => ClockResourceType::HSE,
            0b10 => ClockResourceType::PLL,
            _ => ClockResourceType::HSI, // Default case
        }
    }
}