#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused)]

use crate::bsw::{ioif::{ioif_cfg::{IOIF_TX_PDU_COUNT, ioif_get_tx_pdu_config, ioif_get_tx_pdu_group_config, IOIF_TX_PDU_GROUP_COUNT}, 
                 ioif_type::{IoIf_TxChannelType, IoIf_PeripheralType, IoIf_ReturnType, 
                    IoIf_TxPdu, IoIf_TxPduGroup, IoIf_OutputType, IoIf_TxChannelGroupType}}};
use crate::bsw::iohwab::{led::{set_led_state, led_set_state_group}, iohwab_type::{LedColor, LedState, LedGroup}};

static mut IOIF_TX_CONFIRMATION_TABLE :[u8; IOIF_TX_PDU_COUNT] = [0; IOIF_TX_PDU_COUNT]; // Bảng trạng thái gửi dữ liệu cho các kênh
static mut IOIF_TX_GROUP_CONFIRMATION_TABLE :[u8; IOIF_TX_PDU_GROUP_COUNT] = [0; IOIF_TX_PDU_GROUP_COUNT]; // Bảng trạng thái gửi dữ liệu cho các nhóm kênh
fn get_tx_pdu_by_id(pdu_id: u32) -> Option<&'static IoIf_TxPdu> {
    let tx_pdus = ioif_get_tx_pdu_config();
    for pdu in tx_pdus {
        if pdu.id == pdu_id {
            return Some(pdu);
        }
    }
    None
}
fn get_tx_pdu_group_by_id(pdu_group_id: u32) -> Option<&'static IoIf_TxPduGroup> {
    let tx_pdu_groups = ioif_get_tx_pdu_group_config();
    for pdu_group in tx_pdu_groups {
        if pdu_group.id == pdu_group_id {
            return Some(pdu_group);
        }
    }
    None
}
fn ioif_txchannel_to_ledcolor(channel: IoIf_TxChannelType) -> LedColor {
    match channel {
        IoIf_TxChannelType::LED_RED => LedColor::Red,
        IoIf_TxChannelType::LED_ORANGE => LedColor::Orange,
        IoIf_TxChannelType::LED_BLUE => LedColor::Blue,
        IoIf_TxChannelType::LED_YELLOW => LedColor::Yellow,
        _ => panic!("Unsupported channel for LED mapping"),
    }
}
fn ioif_txchannelgroup_to_ledgroup(channel_group: IoIf_TxChannelGroupType) -> LedGroup {
    match channel_group {
        IoIf_TxChannelGroupType::LED_GROUP_RED_YELLOW => LedGroup::RedYellow,
        IoIf_TxChannelGroupType::LED_GROUP_BLUE_ORANGE => LedGroup::BlueOrange,
        _ => panic!("Unsupported channel group for LED mapping"),
    }
}
fn ioif_ledstate_to_ledstate(state: IoIf_OutputType) -> LedState {
    match state {
        IoIf_OutputType::STD_ON => LedState::On,
        IoIf_OutputType::STD_OFF => LedState::Off,
    }
}
fn ioif_switchtxchannel(pdu_cfg : &IoIf_TxPdu, state : IoIf_OutputType) -> IoIf_ReturnType{
    let led_state = ioif_ledstate_to_ledstate(state);
    let led_color = ioif_txchannel_to_ledcolor(pdu_cfg.channel);
    let index = pdu_cfg.index;
    match pdu_cfg.channel {
        IoIf_TxChannelType::LED_RED => {
             set_led_state(led_color, led_state); 
             IoIf_ReturnType::IOIF_E_OK 
            },
        IoIf_TxChannelType::LED_ORANGE => {
             set_led_state(led_color, led_state); 
             IoIf_ReturnType::IOIF_E_OK 
            },
        IoIf_TxChannelType::LED_BLUE => { 
            set_led_state(led_color, led_state); 
            IoIf_ReturnType::IOIF_E_OK 
            },
        IoIf_TxChannelType::LED_YELLOW => { 
            set_led_state(led_color, led_state); 
            IoIf_ReturnType::IOIF_E_OK 
            },
        _ => IoIf_ReturnType::IOIF_E_NOT_OK,
    }
}
fn ioif_switchtxchannel_group(pdu_group_cfg : &IoIf_TxPduGroup, value: u32) -> IoIf_ReturnType{
    let led_group = ioif_txchannelgroup_to_ledgroup(pdu_group_cfg.channel_group);
    match pdu_group_cfg.channel_group {
        IoIf_TxChannelGroupType::LED_GROUP_RED_YELLOW => {
            led_set_state_group(led_group, value);
            IoIf_ReturnType::IOIF_E_OK
        },
        IoIf_TxChannelGroupType::LED_GROUP_BLUE_ORANGE => {
            led_set_state_group(led_group, value);
            IoIf_ReturnType::IOIF_E_OK
        },
        _ => IoIf_ReturnType::IOIF_E_NOT_OK,
    }
}
pub fn ioif_txconfirmation(pdu_id: u32, result: IoIf_ReturnType) -> IoIf_ReturnType {
    // Tìm cấu hình PDU dựa trên pdu_id
    let pdu_cfg = get_tx_pdu_by_id(pdu_id);
    if let Some(_pdu) = pdu_cfg {
        // Xử lý xác nhận gửi dữ liệu dựa trên cấu hình PDU
        // Ở đây, chúng ta chỉ trả về IOIF_E_OK để minh họa
        unsafe {
            if _pdu.index >= IOIF_TX_PDU_COUNT {
                return IoIf_ReturnType::IOIF_E_NOT_OK;
            }
            IOIF_TX_CONFIRMATION_TABLE[_pdu.index] = result as u8;
        }
        return IoIf_ReturnType::IOIF_E_OK;
    } 
    if let Some(pdu_group) = get_tx_pdu_group_by_id(pdu_id) {
        // Xử lý xác nhận gửi dữ liệu dựa trên cấu hình PDU Group
        // Ở đây, chúng ta chỉ trả về IOIF_E_OK để minh họa
        unsafe {
            if pdu_group.index >= IOIF_TX_PDU_GROUP_COUNT {
                return IoIf_ReturnType::IOIF_E_NOT_OK;
            }
            IOIF_TX_GROUP_CONFIRMATION_TABLE[pdu_group.index] = result as u8;
        }
        return IoIf_ReturnType::IOIF_E_OK;
    }
    // Không tìm thấy cấu hình PDU hoặc PDU Group cho pdu_id này
    IoIf_ReturnType::IOIF_E_NOT_OK
}
pub fn ioif_write_tx_state(pdu_id: u32, state: IoIf_OutputType) -> IoIf_ReturnType {
    // Tìm cấu hình PDU dựa trên pdu_id
    let pdu_cfg = get_tx_pdu_by_id(pdu_id);
    if let Some(pdu) = pdu_cfg {
        // Xử lý dữ liệu gửi đi dựa trên cấu hình PDU
        match pdu.peripheral {
            IoIf_PeripheralType::DIO => {
                // Xử lý dữ liệu DIO
                let result =  ioif_switchtxchannel(pdu, state);
                if(result == IoIf_ReturnType::IOIF_E_OK){
                    return ioif_txconfirmation(pdu_id, IoIf_ReturnType::IOIF_E_OK);
                } else {
                    return ioif_txconfirmation(pdu_id, IoIf_ReturnType::IOIF_E_NOT_OK);
                }
            }
            // Xử lý các kênh khác (ADC, PWM) nếu cần
            _ => {
                return IoIf_ReturnType::IOIF_E_NOT_OK;
            }
        }
    } else {
        // Không tìm thấy cấu hình PDU cho pdu_id này
        return IoIf_ReturnType::IOIF_E_NOT_OK;
    }
}
pub fn ioif_write_tx_group_state(pdu_group_id: u32, value: u32) -> IoIf_ReturnType {
    let pdu_group_cfg = get_tx_pdu_group_by_id(pdu_group_id);
    if let Some(pdu_group) = pdu_group_cfg {
        match pdu_group.peripheral {
            IoIf_PeripheralType::DIO => {
                // Xử lý dữ liệu DIO cho nhóm kênh
                let result = ioif_switchtxchannel_group(pdu_group, value);
                if result == IoIf_ReturnType::IOIF_E_OK {
                    return ioif_txconfirmation(pdu_group_id, IoIf_ReturnType::IOIF_E_OK);
                } else {
                    return ioif_txconfirmation(pdu_group_id, IoIf_ReturnType::IOIF_E_NOT_OK);
                }
            }
            // Xử lý các nhóm kênh khác (ADC, PWM) nếu cần
            _ => {
                return IoIf_ReturnType::IOIF_E_NOT_OK;
            }
        }
    } else {
        return IoIf_ReturnType::IOIF_E_NOT_OK;
    }
}

