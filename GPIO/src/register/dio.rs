use crate::register::gpio_type::{PORT, PIN, Level};
use crate::register::gpio::{get_port_register};

pub fn dio_write(port: PORT, pin: PIN, value: Level) {
    unsafe {
        let gpio = get_port_register(port);

        let value = if value == Level::High {
            1u32 << (pin as u32)
        } else {
            1u32 << ((pin as u32) + 16)
        };

        core::ptr::write_volatile(&mut (*gpio).bsrr, value);
    }
}
pub fn dio_read(port: PORT, pin: PIN) -> Level {
    unsafe {
        let port_register = get_port_register(port);
        let idr_shift = pin as u32;
        let idr_value = core::ptr::read_volatile(&(*port_register).idr) & (0b1 << idr_shift);
        if idr_value != 0 {
            Level::High
        } else {
            Level::Low
        }
    }
}
pub fn dio_toggle(port: PORT, pin: PIN) {
    unsafe {
        let port_register = get_port_register(port);
        let odr_shift = pin as u32;
        let odr_value = core::ptr::read_volatile(&(*port_register).odr) & (0b1 << odr_shift);
        if odr_value != 0 {
            // Pin is currently high, set it low
            core::ptr::write_volatile(&mut (*port_register).bsrr, 1u32 << (odr_shift + 16));
        } else {
            // Pin is currently low, set it high
            core::ptr::write_volatile(&mut (*port_register).bsrr, 1u32 << odr_shift);
        }
    }
}
pub fn dio_read_output(port: PORT, pin: PIN) -> Level {
    unsafe {
        let port_register = get_port_register(port);
        let odr_shift = pin as u32;
        let odr_value = core::ptr::read_volatile(&(*port_register).odr) & (0b1 << odr_shift);
        if odr_value != 0 {
            Level::High
        } else {
            Level::Low
        }
    }
}
pub fn dio_write_port(port: PORT, value: u32) {
    unsafe {
        let port_register = get_port_register(port);
        core::ptr::write_volatile(&mut (*port_register).odr, value);
    }
}
pub fn dio_read_port(port: PORT) -> u32 {
    unsafe {
        let port_register = get_port_register(port);
        core::ptr::read_volatile(&(*port_register).idr)
    }

}
pub fn dio_read_output_port(port: PORT) -> u32 {
    unsafe {
        let port_register = get_port_register(port);
        core::ptr::read_volatile(&(*port_register).odr)
    }
}