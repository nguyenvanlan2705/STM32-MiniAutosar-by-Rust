#![allow(dead_code)]
use crate::mcal::mcu::mcu_get_system_tick_count;
use core::sync::atomic::{Ordering};
use crate::bsw::ioif::ioif::ioif_init;
use crate::bsw::management::comm::comm::{comm_getcurrentcommode, comm_init, comm_mainfunction};
use crate::bsw::management::comm::comm_type::{ComM_NetWorkHandleType::GPIO, ComMMode};
use crate::app::{button_app::button_app_1ms, led_app::{ led_app_1ms, led_app_500ms}};
use crate::bsw::cfg::scheduler_cfg::{SCHEDULER_TASKS_TABLE, TASK_LAST_RUN_TICKS};
use crate::mcal::uart::uart_init;
use crate::register::uart_type::UsartNumber;


pub fn scheduler_runnable_1ms() {
    if scheduler_is_network_fullcom() {
        let button_status = button_app_1ms();
        led_app_1ms(button_status);
    }
    else {
        // Nếu không ở chế độ FULL_COMMUNICATION, có thể thực hiện các hành động khác hoặc bỏ qua
    }
}
pub fn scheduler_runnable_10ms() {
    comm_mainfunction();
}
pub fn scheduler_runnable_100ms() {

}
fn scheduler_is_network_fullcom() -> bool {
    let mode = comm_getcurrentcommode(GPIO);
    if let Some(mode) = mode {
        mode == ComMMode::FULL_COMMUNICATION
    } else {
        false
    }
}
pub fn scheduler_runnable_500ms() {
    if scheduler_is_network_fullcom() {
        led_app_500ms();
    }
    else {
        // Nếu không ở chế độ FULL_COMMUNICATION, có thể thực hiện các hành động khác hoặc bỏ qua
    }
}


pub fn scheduler_init() {
    let now = mcu_get_system_tick_count();
    for index in 0..TASK_LAST_RUN_TICKS.len() {
        TASK_LAST_RUN_TICKS[index].store(now, Ordering::SeqCst);
    }
}

pub fn scheduler_oneshot_task() {
    ioif_init();
    comm_init();
    uart_init(UsartNumber::USART2, 9600);
}
pub fn scheduler_mainfunction(){
    for index in 0..SCHEDULER_TASKS_TABLE.tasks.len() {
        let task = &SCHEDULER_TASKS_TABLE.tasks[index];

        let now = mcu_get_system_tick_count();
        let last = TASK_LAST_RUN_TICKS[index].load(Ordering::Relaxed);
        let elapsed = now.wrapping_sub(last);

        if elapsed >= task.period_ms {
            (task.task_fn)();
            TASK_LAST_RUN_TICKS[index].store(now, Ordering::Relaxed);
        }
    }
}
