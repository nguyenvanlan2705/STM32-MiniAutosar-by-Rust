#![allow(non_snake_case)]
#![allow(dead_code)]
// src/bsw/iohwab/iohwab_type.rs
// Định nghĩa các loại dữ liệu và cấu trúc cho I/O Hardware Abstraction Layer (IOHWAB)
/* LED */
pub enum LedColor {
    Yellow,
    Orange,
    Red,
    Blue,
}

pub enum LedState {
    On,
    Off
}

/* Button */
pub enum Button {
    UserButton,
}

pub enum LedGroup {
    RedYellow, // Ví dụ: Nhóm LED 1
    BlueOrange, // Ví dụ: Nhóm LED 2
}