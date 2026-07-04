#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused)]

use crate::bsw::{ioif::{ioif_cfg::{IOIF_RX_PDU_COUNT, ioif_get_rx_pdu_config}, 
                 ioif_type::{IoIf_RxChannelType, IoIf_PeripheralType, IoIf_ReturnType, IoIf_RxMode, IoIf_RxPdu}}};
use crate::bsw::iohwab::{button::{get_button_count, read_button_state},
                        iohwab_type::{Button}};



static mut IOIF_INDICATION_TABLE :[u8; IOIF_RX_PDU_COUNT] = [0; IOIF_RX_PDU_COUNT]; // Bảng trạng thái nhận dữ liệu cho các kênh

fn get_rx_pdu_by_id(pdu_id: u32) -> Option<&'static IoIf_RxPdu> {
    let rx_pdus = ioif_get_rx_pdu_config();
    for pdu in rx_pdus {
        if pdu.id == pdu_id {
            return Some(pdu);
        }
    }
    None
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
                        unsafe {
                            if pdu.index >= IOIF_RX_PDU_COUNT {
                                return IoIf_ReturnType::IOIF_E_NOT_OK;
                            }
                            // Đánh dấu rằng dữ liệu đã được nhận cho kênh BUTTON_USER
                            IOIF_INDICATION_TABLE[pdu.index] = 1;
                        }
                        return IoIf_ReturnType::IOIF_E_OK;
                    }
                    // Xử lý các kênh DIO khác nếu cần
                    _ => {
                        return IoIf_ReturnType::IOIF_E_NOT_OK;
                    }
                }
            }
            // Xử lý các kênh khác (ADC, PWM) nếu cần
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

pub fn ioif_is_rx_indication_active(pdu_cfg : &IoIf_RxPdu) -> bool {
    unsafe {
        if pdu_cfg.index >= IOIF_RX_PDU_COUNT {
            return false;
        }
        IOIF_INDICATION_TABLE[pdu_cfg.index] != 0
    }
}
pub fn ioif_clear_rx_indication(pdu_cfg: &IoIf_RxPdu) {
    unsafe {
        if pdu_cfg.index >= IOIF_RX_PDU_COUNT {
            return;
        }
        IOIF_INDICATION_TABLE[pdu_cfg.index] = 0;
    }
}
fn ioif_rxchannel_to_button(channel: IoIf_RxChannelType) -> Button {
    match channel {
        IoIf_RxChannelType::BUTTON_USER => Button::UserButton,
        _ => panic!("Unsupported channel for button mapping"),
    }
}
pub fn ioif_read_rx_value(pdu_id: u32, data: &mut u8) -> IoIf_ReturnType {
    if let Some(pdu_cfg) = get_rx_pdu_by_id(pdu_id) {
        match pdu_cfg.mode{
            IoIf_RxMode::INTERRUPT => {
                // Kiểm tra xem dữ liệu đã được nhận chưa
                if ioif_is_rx_indication_active(pdu_cfg) {
                    // Nếu dữ liệu đã được nhận, đọc giá trị từ phần cứng (ví dụ: từ GPIO)
                    *data = get_button_count(); // Giả sử chúng ta đang đọc giá trị từ nút nhấn
                    // Sau khi đọc xong, xóa trạng thái nhận dữ liệu
                    ioif_clear_rx_indication(pdu_cfg);
                    IoIf_ReturnType::IOIF_E_OK
                } else {
                    // Dữ liệu chưa được nhận
                    IoIf_ReturnType::IOIF_E_NOT_OK
                }
            }
            IoIf_RxMode::POLLING => {
                let _channel = ioif_rxchannel_to_button(pdu_cfg.channel);
                // Đọc giá trị từ phần cứng (ví dụ: từ GPIO) mà không cần chờ ngắt
                let _state = read_button_state(_channel); // Giả sử chúng ta đang đọc giá trị từ nút nhấn
                *data = _state as u8;
                IoIf_ReturnType::IOIF_E_OK
            }
        } 
        
    } else {
        IoIf_ReturnType::IOIF_E_NOT_OK
    }
}
pub fn ioif_clear_all_rx_indications() {
    unsafe {
        for i in 0..IOIF_RX_PDU_COUNT {
            IOIF_INDICATION_TABLE[i] = 0;
        }
    }
}