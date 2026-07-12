#![allow(dead_code)]
pub struct PduInfoType{
    pub data: *const u8,
    pub length: u32,
}
pub type PduIdType = u16;