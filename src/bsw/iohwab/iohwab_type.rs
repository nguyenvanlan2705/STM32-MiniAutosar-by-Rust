#![allow(non_snake_case)]
#![allow(dead_code)]
// src/bsw/iohwab/iohwab_type.rs
// Định nghĩa các loại dữ liệu và cấu trúc cho I/O Hardware Abstraction Layer (IOHWAB)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/* LED */
pub enum LedColor {
    Yellow,
    Orange,
    Red,
    Blue,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedState {
    On,
    Off
}

/* Button */
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    UserButton,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedGroup {
    RedYellow, // Ví dụ: Nhóm LED 1
    BlueOrange, // Ví dụ: Nhóm LED 2
}
