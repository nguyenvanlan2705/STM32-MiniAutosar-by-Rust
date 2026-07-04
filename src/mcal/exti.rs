#![allow(dead_code)]

use crate::mcal::exti_type::{Exti_Config, Exti_ConfigType};
use crate::register::exti::{set_exti_trigger, enable_exti_line, get_exti_get_irq, disable_exti_line, clear_exti_pending};
use crate::register::nvic::{nvic_enable_irq, nvic_disable_irq};
use crate::register::syscfg_type::{EXTILINE};
use crate::register::syscfg::{enable_syscfg_clock, configure_exti_line};
use crate::register::gpio_type::{PORT};
use crate::bsw::iohwab::button::button_callback;

// Định nghĩa mảng callback cho các dòng EXTI
pub static mut EXTI_CALLBACK: [Option<fn()>; 16] = [None; 16];
// Hàm đăng ký callback cho một dòng EXTI cụ thể
pub fn register_exti_callback(line: EXTILINE, callback: fn()) {
    unsafe {
        EXTI_CALLBACK[line as usize] = Some(callback);
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
// Cấu hình EXTI cho nút nhấn (ví dụ: nút nhấn kết nối với PA0)
const EXTI_CONFIG: Exti_ConfigType = Exti_ConfigType {
    exti: &[
        Exti_Config {
            port: PORT::A,
            line: EXTILINE::LINE0,
            trigger: crate::register::exti_type::Exti_TriggerType::RISING,
            enabled: true,
            callbackfn: Some(button_callback),
        }
    ]
}; 
// Hàm khởi tạo EXTI
pub fn exti_init() {
    for exti_cfg in EXTI_CONFIG.exti.iter(){
        if exti_cfg.enabled {
            // Kích hoạt clock cho SYSCFG (nếu cần)
            enable_syscfg_clock();
            // Cấu hình dòng EXTI
            configure_exti_line(exti_cfg.line, exti_cfg.port);
            // Cấu hình trigger
            set_exti_trigger(exti_cfg.line, exti_cfg.trigger);
            // Đăng ký callback và kích hoạt ngắt trong NVIC
            if let Some(callback) = exti_cfg.callbackfn {
                register_exti_callback(exti_cfg.line, callback);
            }
            // Kích hoạt thông báo ngắt
            exti_enable_notification(exti_cfg.line);
        }
    }
}
