#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::mcal::dio::{dio_readchannel};
use crate::register::gpio_type::{Dio_LevelType};
use crate::mcal::dio_type::{Dio_ChannelType};
use crate::bsw::{iohwab::iohwab_type::{Button},
                ioif::ioif_rx::ioif_rxindication};
use core::sync::atomic::{AtomicU8, Ordering};

static BUTTON_COUNT : AtomicU8 = AtomicU8::new(0);
pub fn button_exti_notification() {
    // Xử lý ngắt từ nút nhấn
    // Ví dụ: tăng biến đếm, thay đổi trạng thái LED, v.v.
    // Ở đây, chúng ta chỉ in ra thông báo để minh họa.
    BUTTON_COUNT.fetch_add(1, Ordering::Relaxed);
    if BUTTON_COUNT.load(Ordering::Relaxed) > 10 {
        BUTTON_COUNT.store(0, Ordering::Relaxed);
    }
    let _ = ioif_rxindication(0x100); // Gọi hàm ioif_rxindication với pdu_id là 0
}
pub fn get_button_count() -> u8 {
    BUTTON_COUNT.load(Ordering::Relaxed)
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
