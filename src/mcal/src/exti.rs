#![allow(dead_code)]

use crate::mcal::cfg::exti_cfg::EXTI_CONFIG;
use crate::register::exti::{
    clear_exti_pending, disable_exti_line, enable_exti_line, get_exti_get_irq, set_exti_trigger,
};
use crate::register::nvic::{nvic_disable_irq, nvic_enable_irq};
use crate::register::syscfg::{configure_exti_line, enable_syscfg_clock};
use crate::register::syscfg_type::EXTILINE;
use core::sync::atomic::{AtomicUsize, Ordering};

static EXTI_CALLBACK: [AtomicUsize; 16] = [const{AtomicUsize::new(0)}; 16];
pub fn register_exti_callback(line: EXTILINE, callback: fn()) {
        EXTI_CALLBACK[line as usize].store(callback as usize, Ordering::Release);
}
pub fn register_get_exti_callback(line: EXTILINE) -> Option<fn()> {
    let callback = EXTI_CALLBACK[line as usize].load(Ordering::Acquire);
    if callback == 0 {
        None
    } else {
        Some(unsafe { core::mem::transmute(callback) })
    }
}

pub fn exti_enable_notification(line: EXTILINE) {
    clear_exti_pending(line);
    enable_exti_line(line);

    let irq = get_exti_get_irq(line);
    nvic_enable_irq(irq);
}

pub fn exti_disable_notification(line: EXTILINE) {
    disable_exti_line(line);

    let irq = get_exti_get_irq(line);
    nvic_disable_irq(irq);
}

pub fn exti_init() {
    for exti_cfg in EXTI_CONFIG.exti.iter() {
        if exti_cfg.enabled {
            enable_syscfg_clock();
            configure_exti_line(exti_cfg.line, exti_cfg.port);
            set_exti_trigger(exti_cfg.line, exti_cfg.trigger);

            if let Some(callback) = exti_cfg.callbackfn {
                register_exti_callback(exti_cfg.line, callback);
            }

            exti_enable_notification(exti_cfg.line);
        }
    }
}
