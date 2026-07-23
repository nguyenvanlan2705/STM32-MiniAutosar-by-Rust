#![allow(dead_code)]
use crate::mcal::mcu::mcu_get_system_tick_count;
use core::sync::atomic::{AtomicBool, AtomicU8, AtomicU16, Ordering};
use crate::bsw::ioif::ioif::ioif_init;
use crate::bsw::management::comm::comm::{comm_getcurrentcommode, comm_init, comm_mainfunction};
use crate::bsw::management::comm::comm_type::{ComM_NetWorkHandleType::GPIO, ComMMode};
use crate::app::{button_app::button_app_1ms, led_app::{ led_app_1ms, led_app_500ms}, temperature_app::temperature_measurement_app_1ms};
use crate::bsw::cfg::scheduler_cfg::{SCHEDULER_TASKS_TABLE, TASK_LAST_RUN_TICKS};
use crate::mcal::usart::{usart_init};
use crate::register::usart_type::UsartNumber;
use crate::bsw::usartif::usartif_tx::{usartif_transmit};
use crate::bsw::usartif::usartif_rx::{usartif_get_pdu_status, usartif_startofreception, usartif_rx_processing, usartif_rx_data_is_available, usartif_rx_timeout_processing};
use crate::bsw::usartif::usartif_type::{UsartIf_PduStatus, UsartIf_ReturnType};
use crate::bsw::common_type::PduInfoType;
use crate::bsw::iohwab::sensor:: iohwab_sensor_mainfunction;
use crate::mcal::adc::{adc_init}; 
use crate::mcal::spi::{spi_init};
use crate::mcal::mcu::{mcu_init, mcu_init_systick_1ms};
use crate::mcal::port::port_init;
use crate::mcal::exti::exti_init;
use crate::mcal::external::mcp2515::{mcp2515_init};

static USART_RX_TEST_BUFFER: [AtomicU8; 6] = [const { AtomicU8::new(0) }; 6];
static RX_STARTED: AtomicBool = AtomicBool::new(false);
static TEMPERATURE : AtomicU16 = AtomicU16::new(0);
static COUNT : AtomicU16 = AtomicU16::new(0);

fn scheduler_clean_rx_buffer() {

    for byte in USART_RX_TEST_BUFFER.iter() {
        byte.store(0, Ordering::SeqCst);
    }
}
pub fn scheduler_runnable_1ms() {
    let button_status = button_app_1ms();
    led_app_1ms((button_status & 0xf) as u8); // just use the lower 4 bits of button_status for led_app_1ms
    let temperature = temperature_measurement_app_1ms();
    TEMPERATURE.store(temperature, Ordering::SeqCst);
    // Verify lower layer works well by checking if temperature is greater than 30, if so, increment count, otherwise reset count to 0
    if temperature > 30 {
        COUNT.fetch_add(1, Ordering::SeqCst);
    } else {
        COUNT.store(0, Ordering::SeqCst);
    }
}
pub fn scheduler_runnable_5ms() {
    if scheduler_is_network_fullcom() {
        
        if !RX_STARTED.load(Ordering::SeqCst) {
            let pdu_info = PduInfoType {
                data: USART_RX_TEST_BUFFER.as_ptr() as *mut u8,
                length: 6,
            };
            scheduler_clean_rx_buffer();
            let status = usartif_startofreception(UsartNumber::USART2, &pdu_info);

            if status == UsartIf_ReturnType::USARTIF_OK {
                RX_STARTED.store(true, Ordering::SeqCst);
            }
        }
        if RX_STARTED.load(Ordering::SeqCst) {
            if usartif_rx_data_is_available(UsartNumber::USART2) {
                usartif_rx_processing(UsartNumber::USART2);
            }
        }
        usartif_rx_timeout_processing(UsartNumber::USART2);
        let rx_status = usartif_get_pdu_status(UsartNumber::USART2);
        if rx_status == Some(UsartIf_PduStatus::USARTIF_COMPLETED) {
            let rx_data = unsafe {
                core::slice::from_raw_parts(
                    USART_RX_TEST_BUFFER.as_ptr() as *const u8,
                    6,
                )
            };
            if &rx_data[0..3] == *b"111" {
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

    }
    else {
        // Nếu không ở chế độ FULL_COMMUNICATION, có thể thực hiện các hành động khác hoặc bỏ qua
    }
}
pub fn scheduler_runnable_10ms() {
    comm_mainfunction();
    iohwab_sensor_mainfunction();
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
    //System initialization
    mcu_init();
    mcu_init_systick_1ms();

    scheduler_init();

    port_init();
    exti_init();

    ioif_init();
    comm_init();
    usart_init();
    adc_init();
    spi_init();
    mcp2515_init();
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
