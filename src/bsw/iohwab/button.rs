#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::mcal::dio::{dio_readchannel};
use crate::register::gpio_type::{Dio_LevelType};
use crate::mcal::dio_type::{Dio_ChannelType};
use crate::bsw::{iohwab::iohwab_type::{Button},
                ioif::ioif_rx::ioif_rxindication};
use core::sync::atomic::{AtomicU8, AtomicU32, Ordering};
use crate::mcal::mcu::{mcu_get_system_tick_count};

const BUTTON_DEBOUNCE_MS : u32 = 80; // Thời gian chống rung 80ms
static LAST_TICK_BUTTON: AtomicU32 = AtomicU32::new(0);

static BUTTON_COUNT : AtomicU8 = AtomicU8::new(0);
pub fn button_exti_notification() {
    let now_tick = mcu_get_system_tick_count();
    let last_tick = LAST_TICK_BUTTON.load(Ordering::Relaxed);
    if now_tick.wrapping_sub(last_tick) < BUTTON_DEBOUNCE_MS {
        // Nếu thời gian kể từ lần nhấn trước đó chưa đủ 80ms, bỏ qua ngắt này
        return;
    }
    let count = BUTTON_COUNT.load(Ordering::Relaxed);
    let next = if count >= 10 { 0 } else { count + 1 };
    BUTTON_COUNT.store(next, Ordering::Relaxed);
    // Xử lý ngắt từ nút nhấn
    // Ví dụ: tăng biến đếm, thay đổi trạng thái LED, v.v.
    // Ở đây, chúng ta chỉ in ra thông báo để minh họa.
    let _ = ioif_rxindication(0x100); // Gọi hàm ioif_rxindication với pdu_id là 0
}
pub fn get_button_count() -> u8 {
    let count = BUTTON_COUNT.load(Ordering::Relaxed);
    count as u8
}
pub fn button_to_channel(button: Button) -> Dio_ChannelType {
    match button {
        Button::UserButton => Dio_ChannelType::UserButton,
    }
}
pub fn read_button_state(button: Button) -> Dio_LevelType {
    let channel = button_to_channel(button);
    dio_readchannel(channel)
}
