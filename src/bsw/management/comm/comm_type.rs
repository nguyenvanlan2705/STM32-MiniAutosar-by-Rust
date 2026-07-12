#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComMMode{
    NO_COMMUNICATION,
    SILENT_COMMUNICATION,
    FULL_COMMUNICATION,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComMReturnType{
    COMM_E_OK = 0,
    COMM_E_NOT_OK = 1,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComM_NetWorkHandleType{
    GPIO,
    USART,
    CAN,
    SPI
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComM_StateType{
  COMM_NO_COM_NO_PENDING_REQUEST ,
  COMM_NO_COM_REQUEST_PENDING,
  COMM_FULL_COM_NETWORK_REQUESTED,
  COMM_FULL_COM_READY_SLEEP,
  COMM_SILENT_COM,
}
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComMRequestedMode {
    NO_COMMUNICATION,
    FULL_COMMUNICATION,
}