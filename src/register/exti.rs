use crate::register::exti_type::{Exti_TriggerType};
use crate::register::exti_type::get_exti_register;
use crate::register::syscfg_type::{EXTILINE};
use crate::register::nvic_type::{IRQn};

pub fn enable_exti_line(exti_line: EXTILINE) {
    unsafe {
        let exti = get_exti_register();
        let current_value = core::ptr::read_volatile(&(*exti).imr);
        let new_value = current_value | (1 << exti_line as u32);
        core::ptr::write_volatile(&mut (*exti).imr, new_value);
    }
}

pub fn disable_exti_line(exti_line: EXTILINE) {
    unsafe {
        let exti = get_exti_register();
        let current_value = core::ptr::read_volatile(&(*exti).imr);
        let new_value = current_value & !(1 << exti_line as u32);
        core::ptr::write_volatile(&mut (*exti).imr, new_value);
    }
}
pub fn set_exti_pending(exti_line: EXTILINE) {
    unsafe {
        let exti = get_exti_register();
        let current_value = core::ptr::read_volatile(&(*exti).swier);
        let new_value = current_value | (1 << exti_line as u32);
        core::ptr::write_volatile(&mut (*exti).swier, new_value);
    }
}
pub fn clear_exti_pending(exti_line: EXTILINE) {
    unsafe {
        let exti = get_exti_register();
        let current_value = core::ptr::read_volatile(&(*exti).pr);
        let new_value = current_value | (1 << exti_line as u32);
        core::ptr::write_volatile(&mut (*exti).pr, new_value);
    }
}
pub fn set_exti_swier(exti_line: EXTILINE) {
    unsafe {
        let exti = get_exti_register();
        let current_value = core::ptr::read_volatile(&(*exti).swier);
        let new_value = current_value | (1 << exti_line as u32);
        core::ptr::write_volatile(&mut (*exti).swier, new_value);
    }
}
pub fn clear_exti_swier(exti_line: EXTILINE) {
    unsafe {
        let exti = get_exti_register();
        let current_value = core::ptr::read_volatile(&(*exti).swier);
        let new_value = current_value & !(1 << exti_line as u32);
        core::ptr::write_volatile(&mut (*exti).swier, new_value);
    }
}
fn set_rtsr(exti_line: EXTILINE) {
    unsafe {
        let exti = get_exti_register();
        let current_value = core::ptr::read_volatile(&(*exti).rtsr);
        let new_value = current_value | (1 << exti_line as u32);
        core::ptr::write_volatile(&mut (*exti).rtsr, new_value);
    }
}
fn clear_rtsr(exti_line: EXTILINE) {
    unsafe {
        let exti = get_exti_register();
        let current_value = core::ptr::read_volatile(&(*exti).rtsr);
        let new_value = current_value & !(1 << exti_line as u32);
        core::ptr::write_volatile(&mut (*exti).rtsr, new_value);
    }
}
fn set_ftsr(exti_line: EXTILINE) {
    unsafe {
        let exti = get_exti_register();
        let current_value = core::ptr::read_volatile(&(*exti).ftsr);
        let new_value = current_value | (1 << exti_line as u32);
        core::ptr::write_volatile(&mut (*exti).ftsr, new_value);
    }
}
fn clear_ftsr(exti_line: EXTILINE) {
    unsafe {
        let exti = get_exti_register();
        let current_value = core::ptr::read_volatile(&(*exti).ftsr);
        let new_value = current_value & !(1 << exti_line as u32);
        core::ptr::write_volatile(&mut (*exti).ftsr, new_value);
    }
}
pub fn set_exti_trigger(exti_line: EXTILINE, trigger_type: Exti_TriggerType) {
    match trigger_type {
        Exti_TriggerType::RISING => {
            set_rtsr(exti_line);
            clear_ftsr(exti_line);
        }
        Exti_TriggerType::FALLING => {
            set_ftsr(exti_line);
            clear_rtsr(exti_line);
        }
        Exti_TriggerType::RISING_FALLING => {
            set_rtsr(exti_line);
            set_ftsr(exti_line);
        }
    }
}
pub fn get_exti_get_irq(line: EXTILINE) -> IRQn {
    match line {
        EXTILINE::LINE0 => IRQn::EXTI0,
        EXTILINE::LINE1 => IRQn::EXTI1,
        EXTILINE::LINE2 => IRQn::EXTI2,
        EXTILINE::LINE3 => IRQn::EXTI3,
        EXTILINE::LINE4 => IRQn::EXTI4,
        EXTILINE::LINE5 => IRQn::EXTI9_5,
        EXTILINE::LINE6 => IRQn::EXTI9_5,
        EXTILINE::LINE7 => IRQn::EXTI9_5,
        EXTILINE::LINE8 => IRQn::EXTI9_5,
        EXTILINE::LINE9 => IRQn::EXTI9_5,
        EXTILINE::LINE10 => IRQn::EXTI15_10,
        EXTILINE::LINE11 => IRQn::EXTI15_10,
        EXTILINE::LINE12 => IRQn::EXTI15_10,
        EXTILINE::LINE13 => IRQn::EXTI15_10,
        EXTILINE::LINE14 => IRQn::EXTI15_10,
        EXTILINE::LINE15 => IRQn::EXTI15_10,
    }
}