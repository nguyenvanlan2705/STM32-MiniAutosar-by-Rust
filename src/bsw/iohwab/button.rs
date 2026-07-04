#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::mcal::dio::{dio_readchannel};
use crate::register::gpio_type::{Dio_LevelType};
use crate::mcal::dio_type::{Dio_ChannelType};
use crate::bsw::iohwab::iohwab_type::{Button};


static mut BUTTON_COUNT : u8 = 0;
pub fn button_callback() {
    // Xử lý ngắt từ nút nhấn
    // Ví dụ: tăng biến đếm, thay đổi trạng thái LED, v.v.
    // Ở đây, chúng ta chỉ in ra thông báo để minh họa.
    unsafe {
        BUTTON_COUNT = BUTTON_COUNT + 1;
        if BUTTON_COUNT > 8 {
            BUTTON_COUNT = 0;
        }
    }
}
pub fn get_button_count() -> u8 {
    unsafe { BUTTON_COUNT }
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