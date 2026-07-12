#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

use crate ::bsw::management::comm::comm_type::{ComM_NetWorkHandleType};


#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComMUser{
    APP_GPIO,
    DIAG_USART,
    MANAGEMENT_CAN,
    APP_SPI,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComM_NetWorkHandleConfig{
    pub network_handle: ComM_NetWorkHandleType,
    pub user: ComMUser,
}

const COMM_NETWORK_HANDLE_CONFIG: &[ComM_NetWorkHandleConfig] = 
    &[
    ComM_NetWorkHandleConfig {
        network_handle: ComM_NetWorkHandleType::GPIO,
        user: ComMUser::APP_GPIO,
    },
    ComM_NetWorkHandleConfig {
        network_handle: ComM_NetWorkHandleType::USART,
        user: ComMUser::DIAG_USART,
    },
    ComM_NetWorkHandleConfig {
        network_handle: ComM_NetWorkHandleType::CAN,
        user: ComMUser::MANAGEMENT_CAN,
    },
];
pub const COMM_NETWORK_HANDLE_COUNT: usize = 4;
pub fn comm_get_network_handle_config() -> &'static [ComM_NetWorkHandleConfig] {
    &COMM_NETWORK_HANDLE_CONFIG
}