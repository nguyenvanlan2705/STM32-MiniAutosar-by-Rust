#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]   
pub enum IoIf_ReturnType {
    IOIF_E_OK = 0,
    IOIF_E_NOT_OK = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]   
pub enum IoIf_TxChannelType {
    LED_RED,
    LED_ORANGE,
    LED_BLUE,
    LED_YELLOW,
    RELAY,
    FAN_ENABLE,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)] 
pub enum IoIf_TxChannelGroupType{
    LED_GROUP_RED_YELLOW,
    LED_GROUP_BLUE_ORANGE,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)] 
pub enum IoIf_RxChannelType {
    BUTTON_USER,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]   
pub enum IoIf_PeripheralType{
    DIO,
    ADC,
    PWM
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]   
pub enum IoIf_RxMode{
    POLLING,
    INTERRUPT,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]   
pub struct IoIf_RxPdu{
    pub index : usize,
    pub id : u32,
    pub peripheral: IoIf_PeripheralType,
    pub channel: IoIf_RxChannelType,
    pub mode: IoIf_RxMode,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]   
pub struct IoIf_TxPdu{
    pub index : usize,
    pub id : u32,
    pub peripheral: IoIf_PeripheralType,
    pub channel: IoIf_TxChannelType,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]   
pub struct IoIf_TxPduGroup{
    pub index : usize,
    pub id : u32,
    pub peripheral: IoIf_PeripheralType,
    pub channel_group: IoIf_TxChannelGroupType,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]   
pub struct IoIf_ConfigRXType{
    pub rx_pdus: &'static [IoIf_RxPdu],
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]   
pub struct IoIf_ConfigTXType{
    pub tx_pdus: &'static [IoIf_TxPdu],
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IoIf_ConfigTXGroupType{
    pub tx_pdu_groups: &'static [IoIf_TxPduGroup],
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)] 
pub enum IoIf_OutputType {
    STD_ON,
    STD_OFF
}
