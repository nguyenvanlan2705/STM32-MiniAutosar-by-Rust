use crate::register::gpio_type::{PIN, MODE, OUTPUTTYPE, OUTPUTSPEED, PULL, PORT, PortRegister};
use crate::register::rcc_type;

pub fn enable_portx_clock(port: PORT){
    unsafe {
        let rcc = rcc_type::get_rcc_register();
        let shift_value = core::ptr::read_volatile(&(*rcc).rcc_ahb1enr) | (1 << port as u32);
        core::ptr::write_volatile(&mut (*rcc).rcc_ahb1enr, shift_value);
    }
}
pub fn get_port_register(port: PORT) -> *mut PortRegister {
    match port {
        PORT::A => 0x4002_0000 as *mut PortRegister,
        PORT::B => 0x4002_0400 as *mut PortRegister,
        PORT::C => 0x4002_0800 as *mut PortRegister,
        PORT::D => 0x4002_0C00 as *mut PortRegister,
        PORT::E => 0x4002_1000 as *mut PortRegister,
        PORT::F => 0x4002_1400 as *mut PortRegister,
        PORT::G => 0x4002_1800 as *mut PortRegister,
        PORT::H => 0x4002_1C00 as *mut PortRegister,
    }
}
pub fn port_write_mode(port_register: *mut crate::register::gpio_type::PortRegister, pin: PIN, mode: MODE) {
    unsafe {
        let moder_shift = (pin as u32) * 2;
        let moder_value = core::ptr::read_volatile(&(*port_register).moder) & !(0b11 << moder_shift);
        let moder_value = moder_value | ((mode as u32) << moder_shift);
        core::ptr::write_volatile(&mut (*port_register).moder, moder_value);
    }
}
pub fn port_write_outputtype(port_register: *mut crate::register::gpio_type::PortRegister, pin: PIN, output_type: OUTPUTTYPE) {
    unsafe {
        let otyper_shift = pin as u32;
        let otyper_value = core::ptr::read_volatile(&(*port_register).otyper) & !(0b1 << otyper_shift);
        let otyper_value = otyper_value | ((output_type as u32) << otyper_shift);
        core::ptr::write_volatile(&mut (*port_register).otyper, otyper_value);
    }
}
pub fn port_write_outputspeed(port_register: *mut crate::register::gpio_type::PortRegister, pin: PIN, output_speed: OUTPUTSPEED) {
    unsafe {
        let ospeedr_shift = (pin as u32) * 2;
        let ospeedr_value = core::ptr::read_volatile(&(*port_register).ospeedr) & !(0b11 << ospeedr_shift);
        let ospeedr_value = ospeedr_value | ((output_speed as u32) << ospeedr_shift);
        core::ptr::write_volatile(&mut (*port_register).ospeedr, ospeedr_value);
    }
}
pub fn port_write_pull(port_register: *mut crate::register::gpio_type::PortRegister, pin: PIN, pull: PULL) {
    unsafe {
        let pupdr_shift = (pin as u32) * 2;
        let pupdr_value = core::ptr::read_volatile(&(*port_register).pupdr) & !(0b11 << pupdr_shift);
        let pupdr_value = pupdr_value | ((pull as u32) << pupdr_shift);
        core::ptr::write_volatile(&mut (*port_register).pupdr, pupdr_value);
    }
}
 pub fn port_write_alternate_function(port_register: *mut crate::register::gpio_type::PortRegister, pin: PIN, af: u8) {
    unsafe {
        if pin as u32 <= 7 {
            let shift = (pin as u32) * 4;
            let afr_value = core::ptr::read_volatile(&(*port_register).afrl);
            let new_afr_value = (afr_value & !(0b1111 << shift)) | ((af as u32 & 0b1111) << shift);
            core::ptr::write_volatile(&mut (*port_register).afrl, new_afr_value);
        } else {
            let shift = ((pin as u32) - 8) * 4;
            let afr_value = core::ptr::read_volatile(&(*port_register).afrh);
            let new_afr_value = (afr_value & !(0b1111 << shift)) | ((af as u32 & 0b1111) << shift);
            core::ptr::write_volatile(&mut (*port_register).afrh, new_afr_value);
        }
    }
}