#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::{bsw::{
    cfg::ioif_cfg::{
        IOIF_TX_PDU_COUNT, IOIF_TX_PDU_GROUP_COUNT, IOIF_TX_PDU_GROUP_STATUS, IOIF_TX_PDU_STATUS, ioif_get_tx_pdu_config, ioif_get_tx_pdu_group_config
    }, ioif::ioif_type::{
        IoIf_ConfirmationType, IoIf_OutputType, IoIf_PduStatusType, IoIf_PeripheralType, IoIf_ReturnType, IoIf_TxChannelGroupType, IoIf_TxChannelType, IoIf_TxPdu, IoIf_TxPduGroup
    },
}};
use crate::bsw::iohwab::{led::{set_led_state, led_set_state_group, led_toggle, get_led_state}, iohwab_type::{LedColor, LedState, LedGroup}};
use core::sync::atomic::{AtomicU8, AtomicU32, Ordering};
use crate::bsw::common_type::PduIdType;


// Bảng xác nhận gửi dữ liệu cho từng PDU
pub static IOIF_TX_CONFIRMATION_TABLE: [AtomicU8; IOIF_TX_PDU_COUNT] =
    [const { AtomicU8::new(0) }; IOIF_TX_PDU_COUNT];
// Bảng xác nhận gửi dữ liệu cho từng PDU Group
pub static IOIF_TX_GROUP_CONFIRMATION_TABLE: [AtomicU8; IOIF_TX_PDU_GROUP_COUNT] =
    [const { AtomicU8::new(0) }; IOIF_TX_PDU_GROUP_COUNT];
// Bảng trạng thái output cho từng PDU
pub static IOIF_TX_OUTPUT_STATUS : [AtomicU8; IOIF_TX_PDU_COUNT] =
    [const { AtomicU8::new(0) }; IOIF_TX_PDU_COUNT];
// Bảng trạng thái output cho từng PDU Group
pub static IOIF_TX_GROUP_OUTPUT_STATUS : [AtomicU32; IOIF_TX_PDU_GROUP_COUNT] =
    [const { AtomicU32::new(0) }; IOIF_TX_PDU_GROUP_COUNT];


pub fn ioif_clearall_tx_runtime_status() {
    // clear confirmation table
    for i in 0..IOIF_TX_PDU_COUNT {
        IOIF_TX_CONFIRMATION_TABLE[i].store(IoIf_ConfirmationType::CONFIRMED_NOT_OK as u8, Ordering::Relaxed);
    }
    for i in 0..IOIF_TX_PDU_GROUP_COUNT {
        IOIF_TX_GROUP_CONFIRMATION_TABLE[i].store(IoIf_ConfirmationType::CONFIRMED_NOT_OK as u8, Ordering::Relaxed);
    }
    // clear output status table
    for i in 0..IOIF_TX_PDU_COUNT {
        IOIF_TX_OUTPUT_STATUS[i].store(0, Ordering::Relaxed);
    }
    for i in 0..IOIF_TX_PDU_GROUP_COUNT {
        IOIF_TX_GROUP_OUTPUT_STATUS[i].store(0, Ordering::Relaxed);
    }
    // clear PDU status to IDLE
    for i in 0..IOIF_TX_PDU_COUNT {
        IOIF_TX_PDU_STATUS[i].status.store(IoIf_PduStatusType::IOIF_IDLE as u8, Ordering::Relaxed);
    }
    for i in 0..IOIF_TX_PDU_GROUP_COUNT {
        IOIF_TX_PDU_GROUP_STATUS[i].status.store(IoIf_PduStatusType::IOIF_IDLE as u8, Ordering::Relaxed);
    }
}

pub fn ioif_clear_tx_output_status_by_index(index: usize, isGroup: bool) {
    if isGroup {
        IOIF_TX_GROUP_OUTPUT_STATUS[index].store(0, Ordering::Relaxed);
    } else {
        IOIF_TX_OUTPUT_STATUS[index].store(0, Ordering::Relaxed);
    }
}
fn ioif_get_tx_pdu_status_by_index(index: usize, isGroup: bool) -> Option<IoIf_PduStatusType> {
    if isGroup {
        if index >= IOIF_TX_PDU_GROUP_COUNT {
            return None;
        }
        let value = IOIF_TX_PDU_GROUP_STATUS[index].status.load(Ordering::Relaxed);
        match value {
            0 => Some(IoIf_PduStatusType::IOIF_IDLE),
            1 => Some(IoIf_PduStatusType::IOIF_PENDING),
            2 => Some(IoIf_PduStatusType::IOIF_BUSY),
            3 => Some(IoIf_PduStatusType::IOIF_COMPLETED),
            4 => Some(IoIf_PduStatusType::IOIF_ERROR),
            _ => None,
        }
    } else {
        if index >= IOIF_TX_PDU_COUNT {
            return None;
        }
        let value = IOIF_TX_PDU_STATUS[index].status.load(Ordering::Relaxed);
        match value {
            0 => Some(IoIf_PduStatusType::IOIF_IDLE),
            1 => Some(IoIf_PduStatusType::IOIF_PENDING),
            2 => Some(IoIf_PduStatusType::IOIF_BUSY),
            3 => Some(IoIf_PduStatusType::IOIF_COMPLETED),
            4 => Some(IoIf_PduStatusType::IOIF_ERROR),
            _ => None,
        }
    }
}
fn ioif_get_index_by_id(pdu_id: PduIdType, isGroup: bool) -> Option<usize> {
    if isGroup {
        let tx_pdu_groups = ioif_get_tx_pdu_group_config();
        for (index, pdu_group) in tx_pdu_groups.iter().enumerate() {
            if pdu_group.id == pdu_id {
                return Some(index);
            }
        }
    } else {
        let tx_pdus = ioif_get_tx_pdu_config();
        for (index, pdu) in tx_pdus.iter().enumerate() {
            if pdu.id == pdu_id {
                return Some(index);
            }
        }
    }
    None
}

fn ioif_set_tx_pdu_status_by_id(id: PduIdType, status: IoIf_PduStatusType, isGroup: bool) -> IoIf_ReturnType {
    if isGroup {
        let index = ioif_get_index_by_id(id, true);
        if let Some(index) = index {
            if index >= IOIF_TX_PDU_GROUP_COUNT {
                return IoIf_ReturnType::IOIF_E_NOT_OK;
            }
            IOIF_TX_PDU_GROUP_STATUS[index].status.store(status as u8, Ordering::Relaxed);
        } else {
            return IoIf_ReturnType::IOIF_E_NOT_OK;
        }
    } else {
        if let Some(index) = ioif_get_index_by_id(id, false) {
            if index >= IOIF_TX_PDU_COUNT {
                return IoIf_ReturnType::IOIF_E_NOT_OK;
            }
            IOIF_TX_PDU_STATUS[index].status.store(status as u8, Ordering::Relaxed);
        } else {
            return IoIf_ReturnType::IOIF_E_NOT_OK;
        }
    }
    IoIf_ReturnType::IOIF_E_OK
}

pub fn ioif_get_tx_output_status_by_index(index: usize, isGroup: bool) -> Option<u32> {
    if isGroup {
        if index >= IOIF_TX_PDU_GROUP_COUNT {
            return None;
        }
        Some(IOIF_TX_GROUP_OUTPUT_STATUS[index].load(Ordering::Relaxed))
    } else {
        if index >= IOIF_TX_PDU_COUNT {
            return None;
        }
        Some(IOIF_TX_OUTPUT_STATUS[index].load(Ordering::Relaxed) as u32)
    }
}
// Hàm đặt trạng thái output dựa trên index
fn ioif_set_tx_output_status_table(pdu_cfg : &IoIf_TxPdu, state: IoIf_OutputType) -> IoIf_ReturnType {
    let index = pdu_cfg.index;
    if index >= IOIF_TX_PDU_COUNT {
        return IoIf_ReturnType::IOIF_E_NOT_OK;
    }
    if state == IoIf_OutputType::TOGGLE {
        let channel = ioif_txchannel_to_ledcolor(pdu_cfg.channel);
        if let Some(channel) = channel {
            let state = get_led_state(channel);
            IOIF_TX_OUTPUT_STATUS[index].store(state as u8, Ordering::Relaxed);
        } else {
            return IoIf_ReturnType::IOIF_E_NOT_OK;
        }
    } else {
        IOIF_TX_OUTPUT_STATUS[index].store(state as u8, Ordering::Relaxed);
    }
    IoIf_ReturnType::IOIF_E_OK
}
// Hàm đặt trạng thái output cho nhóm kênh dựa trên index
fn ioif_set_tx_group_output_status_table_by_index(pdu_group_cfg : &IoIf_TxPduGroup, value: u32) -> IoIf_ReturnType {
    let index = pdu_group_cfg.index;
    if index >= IOIF_TX_PDU_GROUP_COUNT {
        return IoIf_ReturnType::IOIF_E_NOT_OK;
    }
    IOIF_TX_GROUP_OUTPUT_STATUS[index].store(value, Ordering::Relaxed);
    IoIf_ReturnType::IOIF_E_OK
}
// Hàm tìm kiếm cấu hình PDU dựa trên pdu_id
fn get_tx_pdu_by_id(pdu_id: PduIdType) -> Option<&'static IoIf_TxPdu> {
    let tx_pdus = ioif_get_tx_pdu_config();
    for pdu in tx_pdus {
        if pdu.id == pdu_id {
            return Some(pdu);
        }
    }
    None
}

// Hàm tìm kiếm cấu hình PDU Group dựa trên pdu_group_id
fn get_tx_pdu_group_by_id(pdu_group_id: PduIdType) -> Option<&'static IoIf_TxPduGroup> {
    let tx_pdu_groups = ioif_get_tx_pdu_group_config();
    for pdu_group in tx_pdu_groups {
        if pdu_group.id == pdu_group_id {
            return Some(pdu_group);
        }
    }
    None
}

// Hàm chuyển đổi IoIf_TxChannelType sang LedColor
fn ioif_txchannel_to_ledcolor(channel: IoIf_TxChannelType) -> Option<LedColor> {
    match channel {
        IoIf_TxChannelType::LED_RED => Some(LedColor::Red),
        IoIf_TxChannelType::LED_ORANGE => Some(LedColor::Orange),
        IoIf_TxChannelType::LED_BLUE => Some(LedColor::Blue),
        IoIf_TxChannelType::LED_YELLOW => Some(LedColor::Yellow),
        _ => None,
    }
}

// Hàm chuyển đổi IoIf_TxChannelGroupType sang LedGroup
fn ioif_txchannelgroup_to_ledgroup(channel_group: IoIf_TxChannelGroupType) -> Option<LedGroup> {
    match channel_group {
        IoIf_TxChannelGroupType::LED_GROUP_RED_BLUE => Some(LedGroup::RedBlue),
        IoIf_TxChannelGroupType::LED_GROUP_ORANGE_YELLOW => Some(LedGroup::OrangeYellow),
    }
}

// Hàm chuyển đổi IoIf_OutputType sang LedState
fn ioif_ledstate_to_ledstate(state: IoIf_OutputType) -> LedState {
    match state {
        IoIf_OutputType::STD_ON => LedState::On,
        IoIf_OutputType::STD_OFF => LedState::Off,
        IoIf_OutputType::TOGGLE => LedState::Toggle,
    }
}

// Hàm đặt trạng thái xác nhận gửi dữ liệu dựa trên index
fn ioif_set_tx_confirmation_table_by_index(index: usize, result: IoIf_ConfirmationType, isGroup : bool) -> IoIf_ReturnType {
    if isGroup {
        if index >= IOIF_TX_PDU_GROUP_COUNT {
                return IoIf_ReturnType::IOIF_E_NOT_OK;
        }
        IOIF_TX_GROUP_CONFIRMATION_TABLE[index].store(result as u8, Ordering::Relaxed);
    } else {
        if index >= IOIF_TX_PDU_COUNT {
            return IoIf_ReturnType::IOIF_E_NOT_OK;
        }
        IOIF_TX_CONFIRMATION_TABLE[index].store(result as u8, Ordering::Relaxed);
    }
    IoIf_ReturnType::IOIF_E_OK
}

// Hàm set trạng thái output dựa trên index
fn ioif_switchtxchannel(pdu_cfg : &IoIf_TxPdu, state : IoIf_OutputType) -> IoIf_ReturnType{
    let led_state = ioif_ledstate_to_ledstate(state);
    let led_color = ioif_txchannel_to_ledcolor(pdu_cfg.channel);
    match pdu_cfg.channel {
        IoIf_TxChannelType::LED_RED => {
            if let Some(led_color) = led_color {
                if led_state == LedState::Toggle {
                    led_toggle(led_color);
                } else {
                    set_led_state(led_color, led_state);
                }
                IoIf_ReturnType::IOIF_E_OK
            } else {
                IoIf_ReturnType::IOIF_E_NOT_OK
            }
        },
        IoIf_TxChannelType::LED_ORANGE => {
            if let Some(led_color) = led_color {
                if led_state == LedState::Toggle {
                    led_toggle(led_color);
                } else {
                    set_led_state(led_color, led_state);
                }
                IoIf_ReturnType::IOIF_E_OK
            } else {
                IoIf_ReturnType::IOIF_E_NOT_OK
            }
        },
        IoIf_TxChannelType::LED_BLUE => {
            if let Some(led_color) = led_color {
                if led_state == LedState::Toggle {
                    led_toggle(led_color);
                } else {
                    set_led_state(led_color, led_state);
                }
                IoIf_ReturnType::IOIF_E_OK
            } else {
                IoIf_ReturnType::IOIF_E_NOT_OK
            }
        },
        IoIf_TxChannelType::LED_YELLOW => {
            if let Some(led_color) = led_color {
                if led_state == LedState::Toggle {
                    led_toggle(led_color);
                } else {
                    set_led_state(led_color, led_state);
                }
                IoIf_ReturnType::IOIF_E_OK
            } else {
                IoIf_ReturnType::IOIF_E_NOT_OK
            }
            },
        _ => IoIf_ReturnType::IOIF_E_NOT_OK,
    }
}

// Hàm set trạng thái output cho nhóm kênh dựa trên cấu hình PDU Group
fn ioif_switchtxchannel_group(pdu_group_cfg : &IoIf_TxPduGroup, value: u32) -> IoIf_ReturnType{
    let led_group = ioif_txchannelgroup_to_ledgroup(pdu_group_cfg.channel_group);
    match pdu_group_cfg.channel_group {
        IoIf_TxChannelGroupType::LED_GROUP_RED_BLUE => {
            if let Some(led_group) = led_group {
                led_set_state_group(led_group, value);
                IoIf_ReturnType::IOIF_E_OK
            } else {
                IoIf_ReturnType::IOIF_E_NOT_OK
            }
        },
        IoIf_TxChannelGroupType::LED_GROUP_ORANGE_YELLOW => {
            if let Some(led_group) = led_group {
                led_set_state_group(led_group, value);
                IoIf_ReturnType::IOIF_E_OK
            } else {
                IoIf_ReturnType::IOIF_E_NOT_OK
            }
        },
    }
}

// Hàm xác nhận gửi dữ liệu dựa trên pdu_id và kết quả
pub fn ioif_txconfirmation(pdu_id: PduIdType, result: IoIf_ConfirmationType) -> IoIf_ReturnType {
    // Tìm cấu hình PDU dựa trên pdu_id
    let pdu_cfg = get_tx_pdu_by_id(pdu_id);
    if let Some(_pdu) = pdu_cfg {
        // Xử lý xác nhận gửi dữ liệu dựa trên cấu hình PDU
        // Ở đây, chúng ta chỉ trả về IOIF_E_OK để minh họa
        if _pdu.index >= IOIF_TX_PDU_COUNT {
            return IoIf_ReturnType::IOIF_E_NOT_OK;
        }
        ioif_set_tx_confirmation_table_by_index(_pdu.index, result, false);
        if result == IoIf_ConfirmationType::CONFIRMED_OK {
            ioif_set_tx_pdu_status_by_id(pdu_id, IoIf_PduStatusType::IOIF_COMPLETED, false);
        } else {
            ioif_set_tx_pdu_status_by_id(pdu_id, IoIf_PduStatusType::IOIF_ERROR, false);
        }
        return IoIf_ReturnType::IOIF_E_OK;
    } 
    if let Some(pdu_group) = get_tx_pdu_group_by_id(pdu_id) {
        // Xử lý xác nhận gửi dữ liệu dựa trên cấu hình PDU Group
        // Ở đây, chúng ta chỉ trả về IOIF_E_OK để minh họa
        if pdu_group.index >= IOIF_TX_PDU_GROUP_COUNT {
            return IoIf_ReturnType::IOIF_E_NOT_OK;
        }
        ioif_set_tx_confirmation_table_by_index(pdu_group.index, result, true);
        if result == IoIf_ConfirmationType::CONFIRMED_OK {
            ioif_set_tx_pdu_status_by_id(pdu_id, IoIf_PduStatusType::IOIF_COMPLETED, true);
        } else {
            ioif_set_tx_pdu_status_by_id(pdu_id, IoIf_PduStatusType::IOIF_ERROR, true);
        }
        return IoIf_ReturnType::IOIF_E_OK;
    }
    // Không tìm thấy cấu hình PDU hoặc PDU Group cho pdu_id này
    IoIf_ReturnType::IOIF_E_NOT_OK
}
fn ioif_error_recovery(pdu_id: PduIdType) -> IoIf_ReturnType {
    // Tìm cấu hình PDU dựa trên pdu_id
    let pdu_cfg = get_tx_pdu_by_id(pdu_id);
    if let Some(_pdu) = pdu_cfg {
        // Xử lý khôi phục lỗi dựa trên cấu hình PDU
        // Ở đây, chúng ta chỉ trả về IOIF_E_OK để minh họa
        if _pdu.index >= IOIF_TX_PDU_COUNT {
            return IoIf_ReturnType::IOIF_E_NOT_OK;
        }
        ioif_set_tx_pdu_status_by_id(pdu_id, IoIf_PduStatusType::IOIF_IDLE, false);
        ioif_set_tx_confirmation_table_by_index(_pdu.index, IoIf_ConfirmationType::CONFIRMED_NOT_OK, false);
        return IoIf_ReturnType::IOIF_E_OK;
    } 
    if let Some(pdu_group) = get_tx_pdu_group_by_id(pdu_id) {
        // Xử lý khôi phục lỗi dựa trên cấu hình PDU Group
        // Ở đây, chúng ta chỉ trả về IOIF_E_OK để minh họa
        if pdu_group.index >= IOIF_TX_PDU_GROUP_COUNT {
            return IoIf_ReturnType::IOIF_E_NOT_OK;
        }
        ioif_set_tx_pdu_status_by_id(pdu_id, IoIf_PduStatusType::IOIF_IDLE, true);
        ioif_set_tx_confirmation_table_by_index(pdu_group.index, IoIf_ConfirmationType::CONFIRMED_NOT_OK, true);
        return IoIf_ReturnType::IOIF_E_OK;
    }
    // Không tìm thấy cấu hình PDU hoặc PDU Group cho pdu_id này
    IoIf_ReturnType::IOIF_E_NOT_OK
}
// Hàm gửi dữ liệu dựa trên pdu_id và trạng thái output
pub fn ioif_write_tx_state(pdu_id: PduIdType, state: IoIf_OutputType) -> IoIf_ReturnType {
    // Tìm cấu hình PDU dựa trên pdu_id
    let pdu_cfg = get_tx_pdu_by_id(pdu_id);
    if let Some(pdu) = pdu_cfg {
        // Kiểm tra xem index có hợp lệ không
        if pdu.index >= IOIF_TX_PDU_COUNT {
                    return IoIf_ReturnType::IOIF_E_NOT_OK;
        }
        let current_status = ioif_get_tx_pdu_status_by_index(pdu.index, false).unwrap();
        if current_status == IoIf_PduStatusType::IOIF_ERROR {
            ioif_error_recovery(pdu_id);
            return IoIf_ReturnType::IOIF_E_NOT_OK;
        }
        if current_status != IoIf_PduStatusType::IOIF_IDLE && current_status != IoIf_PduStatusType::IOIF_COMPLETED {
            return IoIf_ReturnType::IOIF_E_NOT_OK;
        }
        // Đặt trạng thái xác nhận gửi dữ liệu là PENDING trước khi thực hiện gửi
        ioif_set_tx_confirmation_table_by_index(pdu.index, IoIf_ConfirmationType::CONFIRMED_NOT_OK, false);
        ioif_set_tx_pdu_status_by_id(pdu_id, IoIf_PduStatusType::IOIF_PENDING, false);
        ioif_clear_tx_output_status_by_index(pdu.index, false);
        // Xử lý dữ liệu gửi dựa trên cấu hình PDU
        match pdu.peripheral {
            IoIf_PeripheralType::DIO => {
                ioif_set_tx_pdu_status_by_id(pdu_id, IoIf_PduStatusType::IOIF_BUSY, false);
                let result =  ioif_switchtxchannel(pdu, state);
                if result == IoIf_ReturnType::IOIF_E_OK{
                    let st_set = ioif_set_tx_output_status_table(pdu, state);
                    if  st_set == IoIf_ReturnType::IOIF_E_OK{
                        return ioif_txconfirmation(pdu_id, IoIf_ConfirmationType::CONFIRMED_OK);
                    } else {
                        return ioif_txconfirmation(pdu_id, IoIf_ConfirmationType::CONFIRMED_NOT_OK);
                    }
                } else {
                    return ioif_txconfirmation(pdu_id, IoIf_ConfirmationType::CONFIRMED_NOT_OK);
                }
            }
            // Xử lý các kênh khác (PWM) nếu cần
            _ => {
                let _ = ioif_txconfirmation(pdu_id, IoIf_ConfirmationType::CONFIRMED_NOT_OK);
                return IoIf_ReturnType::IOIF_E_NOT_OK;
            }
        }
    } else {
        // Không tìm thấy cấu hình PDU cho pdu_id này
        return IoIf_ReturnType::IOIF_E_NOT_OK;
    }
}

// Hàm gửi dữ liệu cho nhóm kênh dựa trên pdu_group_id và giá trị
pub fn ioif_write_tx_group_state(pdu_group_id: PduIdType, value: u32) -> IoIf_ReturnType {
    let pdu_group_cfg = get_tx_pdu_group_by_id(pdu_group_id);
    if let Some(pdu_group) = pdu_group_cfg {
        // Kiểm tra xem index có hợp lệ không
        if pdu_group.index >= IOIF_TX_PDU_GROUP_COUNT {
            return IoIf_ReturnType::IOIF_E_NOT_OK;
        }
        let current_status = ioif_get_tx_pdu_status_by_index(pdu_group.index, true).unwrap();
        if current_status == IoIf_PduStatusType::IOIF_ERROR {
            ioif_error_recovery(pdu_group_id);
            return IoIf_ReturnType::IOIF_E_NOT_OK;
        }
        if current_status != IoIf_PduStatusType::IOIF_IDLE && current_status != IoIf_PduStatusType::IOIF_COMPLETED {
            return IoIf_ReturnType::IOIF_E_NOT_OK;
        }
        // Đặt trạng thái xác nhận gửi dữ liệu là PENDING trước khi thực hiện gửi
        ioif_set_tx_confirmation_table_by_index(pdu_group.index, IoIf_ConfirmationType::CONFIRMED_NOT_OK, true);
        ioif_set_tx_pdu_status_by_id(pdu_group_id, IoIf_PduStatusType::IOIF_PENDING, true);
        ioif_clear_tx_output_status_by_index(pdu_group.index, true);
        // Xử lý dữ liệu gửi dựa trên cấu hình PDU Group
        match pdu_group.peripheral {
            IoIf_PeripheralType::DIO => {
                ioif_set_tx_pdu_status_by_id(pdu_group_id, IoIf_PduStatusType::IOIF_BUSY, true);
                // Xử lý dữ liệu DIO cho nhóm kênh
                let result = ioif_switchtxchannel_group(pdu_group, value);

                if result == IoIf_ReturnType::IOIF_E_OK {
                    let st_set  = ioif_set_tx_group_output_status_table_by_index(pdu_group, value);
                    if st_set == IoIf_ReturnType::IOIF_E_OK {
                        ioif_set_tx_pdu_status_by_id(pdu_group_id, IoIf_PduStatusType::IOIF_COMPLETED, true);
                        return ioif_txconfirmation(pdu_group_id, IoIf_ConfirmationType::CONFIRMED_OK);
                    } else {
                        ioif_set_tx_pdu_status_by_id(pdu_group_id, IoIf_PduStatusType::IOIF_ERROR, true);
                        return ioif_txconfirmation(pdu_group_id, IoIf_ConfirmationType::CONFIRMED_NOT_OK);
                    }
                } else {
                    ioif_set_tx_pdu_status_by_id(pdu_group_id, IoIf_PduStatusType::IOIF_ERROR, true);
                    return ioif_txconfirmation(pdu_group_id, IoIf_ConfirmationType::CONFIRMED_NOT_OK);
                }
            }
            // Xử lý các nhóm kênh khác (ADC, PWM) nếu cần
            _ => {
                let _ = ioif_txconfirmation(pdu_group_id, IoIf_ConfirmationType::CONFIRMED_NOT_OK);
                return IoIf_ReturnType::IOIF_E_NOT_OK;
            }
        }
    } else {
        return IoIf_ReturnType::IOIF_E_NOT_OK;
    }
}
