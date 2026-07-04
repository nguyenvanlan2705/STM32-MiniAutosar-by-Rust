#![allow(dead_code)]

use crate::register::nvic_type::{get_nvic_register, IRQn};

pub fn nvic_enable_irq(irqn: IRQn) {
    unsafe {
        let nvic = get_nvic_register();
        let index = irqn as usize / 32;
        let bit_position = irqn as usize % 32;
        let currentvalue = core::ptr::read_volatile(&(*nvic).icer[index]);
        let new_value = currentvalue | (1 << bit_position);
        core::ptr::write_volatile(&mut (*nvic).iser[index], new_value);
    }
}
pub fn nvic_disable_irq(irqn: IRQn) {
    unsafe {
        let nvic = get_nvic_register();
        let index = irqn as usize / 32;
        let currentvalue = core::ptr::read_volatile(&(*nvic).icer[index]);
        let bit_position = irqn as usize % 32;
        let new_value = currentvalue & !(1 << bit_position);
        core::ptr::write_volatile(&mut (*nvic).icer[index], new_value);
    }
}
