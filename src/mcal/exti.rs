use crate::mcal::exti_type::Exti_Config;
use crate::register::exti::{set_exti_trigger, enable_exti_line, get_exti_get_irq};
use crate::register::nvic::{nvic_enable_irq};
use crate::register::syscfg_type::{EXTILINE};
use crate::register::syscfg::{enable_syscfg_clock, configure_exti_line};
use crate::register::gpio_type::{PORT};
pub static mut COUNT : u8 = 0;

fn button_callback() {
    // Xử lý ngắt từ nút nhấn
    // Ví dụ: tăng biến đếm, thay đổi trạng thái LED, v.v.
    // Ở đây, chúng ta chỉ in ra thông báo để minh họa.
    unsafe {
        COUNT = COUNT + 1; 
    }
}
// Định nghĩa mảng callback cho các dòng EXTI
pub static mut EXTI_CALLBACK: [Option<fn()>; 16] = [None; 16];
// Hàm đăng ký callback cho một dòng EXTI cụ thể
pub fn register_exti_callback(line: usize, callback: fn()) {
    unsafe {
        EXTI_CALLBACK[line] = Some(callback);
    }
}
// Cấu hình EXTI cho nút nhấn (ví dụ: nút nhấn kết nối với PA0)
const EXTI_CONFIG: Exti_Config = Exti_Config {
    port : PORT::A,
    line: EXTILINE::LINE0,
    trigger: crate::register::exti_type::Exti_TriggerType::RISING,
    enabled: true,
}; 
// Hàm trả về cấu hình EXTI
pub fn get_exti_config() -> &'static Exti_Config {
    &EXTI_CONFIG
}
// Hàm khởi tạo EXTI
pub fn exti_init() {
    if EXTI_CONFIG.enabled {
        // Lấy cấu hình EXTI
        let exti_cfg = get_exti_config();
        // Kích hoạt clock cho SYSCFG (nếu cần)
        enable_syscfg_clock();
        // Cấu hình dòng EXTI
        configure_exti_line(exti_cfg. line, exti_cfg. port);
        // Cấu hình trigger
        set_exti_trigger(exti_cfg. line, exti_cfg. trigger);
        // Kích hoạt dòng EXTI
        enable_exti_line(exti_cfg. line);
        // Đăng ký callback và kích hoạt ngắt trong NVIC
        let exti_line = get_exti_get_irq(exti_cfg.line);
        register_exti_callback(exti_cfg.line as usize, button_callback);
        nvic_enable_irq(exti_line);
    }
}