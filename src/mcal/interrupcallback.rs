use crate::register::exti::{clear_exti_pending};
use crate::mcal::exti::{EXTI_CALLBACK};
use crate::register::syscfg_type::{EXTILINE};
/// Hàm xử lý ngắt EXTI, được gọi từ vector table.
pub fn exti_irq_handler(line: EXTILINE) {
    clear_exti_pending(line);
    unsafe {
        if let Some(cb) = EXTI_CALLBACK[line as usize] {
            cb();
        }
    }
}