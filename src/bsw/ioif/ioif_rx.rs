#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused)]
use crate::{bsw::{
    cfg::ioif_cfg::{IOIF_RX_PDU_COUNT, IOIF_RX_PDU_STATUS, ioif_get_rx_pdu_config}, ioif::ioif_type::{
        IoIf_PeripheralType, IoIf_ReturnType, IoIf_RxChannelType, IoIf_RxMode, IoIf_RxPdu, IoIf_PduStatusType
    },
}, mcal::dio_type::StdReturnType::E_NOT_OK};
use crate::mcal::adc_type::ADCReturnType;
use crate::bsw::iohwab::{button::{get_button_count, read_button_state},
                        iohwab_type::{Button, SensorStatusType, IoHwAb_ReturnType, SensorType},
                        sensor::{iohwab_sensor_read_latest_value}};
use core::sync::atomic::{AtomicU8, Ordering};
use crate::bsw::common_type::PduIdType;
use crate::bsw::ioif::ioif_type::IOIF_INVALID_PDU_ID;

static IOIF_INDICATION_TABLE: [AtomicU8; IOIF_RX_PDU_COUNT] =
    [const { AtomicU8::new(0) }; IOIF_RX_PDU_COUNT];


pub fn ioif_clear_all_rx_runtime_status() {
    for i in 0..IOIF_RX_PDU_COUNT {
        IOIF_RX_PDU_STATUS[i].status.store(IoIf_PduStatusType::IOIF_IDLE as u8, Ordering::Relaxed);
    }
    ioif_clear_all_rx_indications();
}
fn ioif_get_rx_index_by_id(pdu_id: PduIdType) -> Option<usize> {
    let rx_pdus = ioif_get_rx_pdu_config();
    for pdu in rx_pdus {
        if pdu.id == pdu_id {
            return Some(pdu.index);
        }
    }
    None
}
pub fn ioif_clear_all_rx_indications() {
    for i in 0..IOIF_RX_PDU_COUNT {
        let _ = ioif_clear_rx_indication_by_index(i);
    }
}

fn ioif_get_rx_pdu_status_by_id(id: PduIdType) -> Option<u8> {
    let index = ioif_get_rx_index_by_id(id);
    if let Some(idx) = index {
        Some(IOIF_RX_PDU_STATUS[idx].status.load(Ordering::Relaxed))
    } else {
        None
    }
}
fn ioif_set_rx_pdu_status_by_id(id: PduIdType, status: IoIf_PduStatusType) {
    if let Some(index) = ioif_get_rx_index_by_id(id) {
        if index < IOIF_RX_PDU_COUNT {
            IOIF_RX_PDU_STATUS[index].status.store(status as u8, Ordering::Relaxed);
        }
    }
}

fn get_rx_pdu_by_id(pdu_id: PduIdType) -> Option<&'static IoIf_RxPdu> {
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
        //Add more mappings here if you have more channels
        _ => None,
    }
}
pub fn ioif_rxindication(pdu_id: PduIdType) -> IoIf_ReturnType{
    if pdu_id == IOIF_INVALID_PDU_ID {
        return IoIf_ReturnType::IOIF_E_NOT_OK;
    }
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
                        ioif_set_rx_pdu_status_by_id(pdu_id, IoIf_PduStatusType::IOIF_PENDING);
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

fn ioif_rx_channel_to_sensor(channel: IoIf_RxChannelType) -> Option<SensorType> {
    match channel {
        IoIf_RxChannelType::SENSOR_LM35 => Some(SensorType::LM35),
        //Add more mappings here if you have more channels
        _ => None,
    }
}
pub fn ioif_read_rx_value(pdu_id: PduIdType, data: &mut u16) -> IoIf_ReturnType {
    if pdu_id == IOIF_INVALID_PDU_ID {
        return IoIf_ReturnType::IOIF_E_NOT_OK;
    }
    if let Some(pdu_cfg) = get_rx_pdu_by_id(pdu_id) {
        match pdu_cfg.peripheral {
            IoIf_PeripheralType::DIO => {
                match pdu_cfg.mode{
                    IoIf_RxMode::INTERRUPT => {
                        // Kiểm tra xem dữ liệu đã được nhận chưa
                        if ioif_is_rx_indication_active(pdu_cfg.index) == IoIf_ReturnType::IOIF_E_OK {
                            // Nếu dữ liệu đã được nhận, đọc giá trị từ phần cứng (ví dụ: từ GPIO)
                            *data = get_button_count() as u16; // Giả sử chúng ta đang đọc giá trị từ nút nhấn
                            // Sau khi đọc xong, xóa trạng thái nhận dữ liệu
                            let _ = ioif_clear_rx_indication_by_index(pdu_cfg.index);
                            ioif_set_rx_pdu_status_by_id(pdu_id, IoIf_PduStatusType::IOIF_IDLE);
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
                            *data = _state as u16;
                            ioif_set_rx_pdu_status_by_id(pdu_id, IoIf_PduStatusType::IOIF_IDLE);
                            IoIf_ReturnType::IOIF_E_OK
                        } else {
                            ioif_set_rx_pdu_status_by_id(pdu_id, IoIf_PduStatusType::IOIF_ERROR);
                            IoIf_ReturnType::IOIF_E_NOT_OK
                        }
                    }
                }
            }
            // Xử lý các kênh khác (ADC) nếu cần
            IoIf_PeripheralType::ADC => {
                match pdu_cfg.mode{
                    IoIf_RxMode::INTERRUPT => {
                        // Interrupt for ADC has not been implemented, so we return an error
                        ioif_set_rx_pdu_status_by_id(pdu_id, IoIf_PduStatusType::IOIF_ERROR);
                        let _ = ioif_clear_rx_indication_by_index(pdu_cfg.index);
                        ioif_set_rx_pdu_status_by_id(pdu_id, IoIf_PduStatusType::IOIF_IDLE);
                        IoIf_ReturnType::IOIF_E_NOT_OK
                    }
                    IoIf_RxMode::POLLING => {
                        let sensor = ioif_rx_channel_to_sensor(pdu_cfg.channel);
                        if let Some(sensor) = sensor {
                            let result = iohwab_sensor_read_latest_value(sensor, data);
                            match result {
                                IoHwAb_ReturnType::IOHWAB_E_OK => {
                                    ioif_set_rx_pdu_status_by_id(pdu_id, IoIf_PduStatusType::IOIF_IDLE);
                                    IoIf_ReturnType::IOIF_E_OK
                                }
                                IoHwAb_ReturnType::IOHWAB_E_NOT_OK => {
                                    ioif_set_rx_pdu_status_by_id(pdu_id, IoIf_PduStatusType::IOIF_ERROR);
                                    IoIf_ReturnType::IOIF_E_NOT_OK
                                }
                            }
                        } else {
                            ioif_set_rx_pdu_status_by_id(pdu_id, IoIf_PduStatusType::IOIF_ERROR);
                            IoIf_ReturnType::IOIF_E_NOT_OK
                        }
                    }
                } 
            }
            _ => {
                ioif_set_rx_pdu_status_by_id(pdu_id, IoIf_PduStatusType::IOIF_ERROR);
                IoIf_ReturnType::IOIF_E_NOT_OK
            }
        } 
    } else {
        IoIf_ReturnType::IOIF_E_NOT_OK
    }
}
