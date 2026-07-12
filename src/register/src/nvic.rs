#![allow(dead_code)]

use crate::register::nvic_type::{get_nvic_register, IRQn};

pub fn nvic_enable_irq(irqn: IRQn) {
    unsafe {
        let nvic = get_nvic_register();
        let index = irqn as usize / 32;
        let bit_position = irqn as usize % 32;
        core::ptr::write_volatile(&mut (*nvic).iser[index], 1 << bit_position);
    }
}
pub fn nvic_disable_irq(irqn: IRQn) {
    unsafe {
        let nvic = get_nvic_register();
        let index = irqn as usize / 32;
        let bit_position = irqn as usize % 32;
        core::ptr::write_volatile(&mut (*nvic).icer[index], 1 << bit_position);
    }
}
