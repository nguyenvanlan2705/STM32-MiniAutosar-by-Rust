#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused)]

use crate::bsw::{
    cfg::ioif_cfg::{ioif_get_rx_pdu_config, IOIF_RX_PDU_COUNT},
    ioif::ioif_type::{
        IoIf_PeripheralType, IoIf_ReturnType, IoIf_RxChannelType, IoIf_RxMode, IoIf_RxPdu,
    },
};
use crate::bsw::iohwab::{button::{get_button_count, read_button_state},
                        iohwab_type::{Button}};
use core::sync::atomic::{AtomicU8, Ordering};


static IOIF_INDICATION_TABLE: [AtomicU8; IOIF_RX_PDU_COUNT] =
    [const { AtomicU8::new(0) }; IOIF_RX_PDU_COUNT];

fn get_rx_pdu_by_id(pdu_id: u32) -> Option<&'static IoIf_RxPdu> {
    let rx_pdus = ioif_get_rx_pdu_config();
    for pdu in rx_pdus {
        if pdu.id == pdu_id {
            return Some(pdu);
        }
    }
    None
}

fn ioif_get_rx_indication(index : usize) -> Option<u8> {
    if index >= IOIF_RX_PDU_COUNT {
        return None;
    }
    if IOIF_INDICATION_TABLE[index].load(Ordering::Relaxed) != 0 {
        Some(IOIF_INDICATION_TABLE[index].load(Ordering::Relaxed))
    } else {
        None
    }
}
fn ioif_is_rx_indication_active(index : usize) -> IoIf_ReturnType {
    if index >= IOIF_RX_PDU_COUNT {
        return IoIf_ReturnType::IOIF_E_NOT_OK;
    }
    if let Some(value) = ioif_get_rx_indication(index) {
            IoIf_ReturnType::IOIF_E_OK
        } else {
            IoIf_ReturnType::IOIF_E_NOT_OK
        }
}
fn ioif_set_rx_indication_by_index(index : usize) -> IoIf_ReturnType {
    if index >= IOIF_RX_PDU_COUNT {
        return IoIf_ReturnType::IOIF_E_NOT_OK;
        }
        IOIF_INDICATION_TABLE[index].store(1, Ordering::Relaxed);
        IoIf_ReturnType::IOIF_E_OK
}

fn ioif_clear_rx_indication_by_index(index : usize) -> IoIf_ReturnType {
    if index >= IOIF_RX_PDU_COUNT {
        return IoIf_ReturnType::IOIF_E_NOT_OK;
    }
        IOIF_INDICATION_TABLE[index].store(0, Ordering::Relaxed);
        IoIf_ReturnType::IOIF_E_OK
}

fn ioif_rxchannel_to_button(channel: IoIf_RxChannelType) -> Option<Button> {
    match channel {
        IoIf_RxChannelType::BUTTON_USER => Some(Button::UserButton),
        _ => None,
    }
}
pub fn ioif_rxindication(pdu_id: u32) -> IoIf_ReturnType{
    // Tìm cấu hình PDU dựa trên pdu_id
    let pdu_cfg = get_rx_pdu_by_id(pdu_id);
    if let Some(pdu) = pdu_cfg {
        // Kiểm tra xem PDU có ở chế độ INTERRUPT không
        if pdu.mode != IoIf_RxMode::INTERRUPT {
            return IoIf_ReturnType::IOIF_E_NOT_OK;
        }
        // Xử lý dữ liệu nhận được dựa trên cấu hình PDU
        match pdu.peripheral {
            IoIf_PeripheralType::DIO => {
                // Xử lý dữ liệu DIO
                match pdu.channel {
                    IoIf_RxChannelType::BUTTON_USER => {
                        if pdu.index >= IOIF_RX_PDU_COUNT {
                            return IoIf_ReturnType::IOIF_E_NOT_OK;
                        }
                        // Đánh dấu rằng dữ liệu đã được nhận cho kênh BUTTON_USER
                        let _ = ioif_set_rx_indication_by_index(pdu.index);
                        return IoIf_ReturnType::IOIF_E_OK;
                    }
                    // Xử lý các kênh DIO khác nếu cần
                    _ => {
                        return IoIf_ReturnType::IOIF_E_NOT_OK;
                    }
                }
            }
            // Xử lý các kênh khác (ADC) nếu cần
            _ => {
                return IoIf_ReturnType::IOIF_E_NOT_OK;
            }
        }
    } else {
        // Không tìm thấy cấu hình PDU cho pdu_id này
        // Có thể ghi log hoặc xử lý lỗi ở đây
        return IoIf_ReturnType::IOIF_E_NOT_OK;
    }
}
pub fn ioif_read_rx_value(pdu_id: u32, data: &mut u8) -> IoIf_ReturnType {
    if let Some(pdu_cfg) = get_rx_pdu_by_id(pdu_id) {
        match pdu_cfg.mode{
            IoIf_RxMode::INTERRUPT => {
                // Kiểm tra xem dữ liệu đã được nhận chưa
                if ioif_is_rx_indication_active(pdu_cfg.index) == IoIf_ReturnType::IOIF_E_OK {
                    // Nếu dữ liệu đã được nhận, đọc giá trị từ phần cứng (ví dụ: từ GPIO)
                    *data = get_button_count(); // Giả sử chúng ta đang đọc giá trị từ nút nhấn
                    // Sau khi đọc xong, xóa trạng thái nhận dữ liệu
                    let _ = ioif_clear_rx_indication_by_index(pdu_cfg.index);
                    IoIf_ReturnType::IOIF_E_OK
                } else {
                    // Dữ liệu chưa được nhận
                    IoIf_ReturnType::IOIF_E_NOT_OK
                }
            }
            IoIf_RxMode::POLLING => {
                if let Some(_channel) = ioif_rxchannel_to_button(pdu_cfg.channel) {
                    // Đọc giá trị từ phần cứng (ví dụ: từ GPIO) mà không cần chờ ngắt
                    let _state = read_button_state(_channel); // Giả sử chúng ta đang đọc giá trị từ nút nhấn
                    *data = _state as u8;
                    IoIf_ReturnType::IOIF_E_OK
                } else {
                    IoIf_ReturnType::IOIF_E_NOT_OK
                }
            }
        } 
        
    } else {
        IoIf_ReturnType::IOIF_E_NOT_OK
    }
}
pub fn ioif_clear_all_rx_indications() {
    for i in 0..IOIF_RX_PDU_COUNT {
        let _ = ioif_clear_rx_indication_by_index(i);
    }
}
