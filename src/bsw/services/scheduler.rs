#![allow(dead_code)]
use crate::mcal::mcu::mcu_get_system_tick_count;
use core::sync::atomic::{AtomicBool, Ordering};
use crate::bsw::ioif::ioif::ioif_init;
use crate::bsw::management::comm::comm::{comm_getcurrentcommode, comm_init, comm_mainfunction};
use crate::bsw::management::comm::comm_type::{ComM_NetWorkHandleType::GPIO, ComMMode};
use crate::app::{button_app::button_app_1ms, led_app::{ led_app_1ms, led_app_500ms}};
use crate::bsw::cfg::scheduler_cfg::{SCHEDULER_TASKS_TABLE, TASK_LAST_RUN_TICKS};
use crate::mcal::usart::{usart_init};
use crate::register::usart_type::UsartNumber;
use crate::bsw::usartif::usartif_tx::{usartif_transmit};
use crate::bsw::usartif::usartif_rx::{usartif_get_pdu_status, usartif_rxindication};
use crate::bsw::usartif::usartif_type::{UsartIf_PduStatus, UsartIf_ReturnType};
use crate::bsw::common_type::PduInfoType;

static mut USART_RX_TEST_BUFFER: [u8; 4] = [0; 4];
static RX_STARTED: AtomicBool = AtomicBool::new(false);


pub fn scheduler_runnable_1ms() {
    let button_status = button_app_1ms();
    led_app_1ms(button_status);
}
pub fn scheduler_runnable_5ms() {
    if scheduler_is_network_fullcom() {
        let rx_status = usartif_get_pdu_status(UsartNumber::USART2);

        if rx_status == Some(UsartIf_PduStatus::USARTIF_COMPLETED) {
            let rx_data = unsafe {
                core::slice::from_raw_parts(
                    core::ptr::addr_of!(USART_RX_TEST_BUFFER) as *const u8,
                    4,
                )
            };
            if rx_data[0] == 49 && rx_data[1] == 49 && rx_data[2] == 49 && rx_data[3] == 13 {
                let pdu_info1 = PduInfoType {
                    data: b"successfully!\n".as_ptr(),
                    length: b"successfully!\n".len() as u32,
                };
                usartif_transmit(0, &pdu_info1);
            }
            RX_STARTED.store(false, Ordering::SeqCst);
        } else if rx_status == Some(UsartIf_PduStatus::USARTIF_ERROR) {
            RX_STARTED.store(false, Ordering::SeqCst);
        }

        if !RX_STARTED.load(Ordering::SeqCst) {
            let pdu_info = PduInfoType {
                data: core::ptr::addr_of_mut!(USART_RX_TEST_BUFFER) as *mut u8,
                length: 4,
            };
            let status = usartif_rxindication(UsartNumber::USART2, &pdu_info);
            if status == UsartIf_ReturnType::USARTIF_OK {
                RX_STARTED.store(true, Ordering::SeqCst);
            }
        }
    }
    else {
        // Nếu không ở chế độ FULL_COMMUNICATION, có thể thực hiện các hành động khác hoặc bỏ qua
    }
}
pub fn scheduler_runnable_10ms() {
    comm_mainfunction();
}
pub fn scheduler_runnable_1000ms(){
    if scheduler_is_network_fullcom() {
        let pdu_info = PduInfoType {
            data: b"Hello, USART!\n".as_ptr(),
            length: b"Hello, USART!\n".len() as u32,
        };
        usartif_transmit(0, &pdu_info);
    }
    
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
    usart_init();
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
