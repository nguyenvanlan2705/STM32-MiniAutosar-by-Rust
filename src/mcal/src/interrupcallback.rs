use crate::register::exti::{clear_exti_pending, is_exti_pending};
use crate::mcal::exti::{register_get_exti_callback};
use crate::register::syscfg_type::{EXTILINE};
/// Hàm xử lý ngắt EXTI, được gọi từ vector table.
pub fn exti_irq_handler(line: EXTILINE) {
    clear_exti_pending(line);
    let cbkfn = register_get_exti_callback(line);
    if let Some(cb) = cbkfn {
        cb();
    }
}
/// Hàm xử lý ngắt EXTI cho nhiều dòng, được gọi từ vector table.
pub fn exti_group_irq_handler(lines: &[EXTILINE]) {
    for &line in lines {
        if is_exti_pending(line) {
            exti_irq_handler(line);
        }
    }
}