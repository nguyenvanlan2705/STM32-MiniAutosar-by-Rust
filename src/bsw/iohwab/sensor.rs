#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::mcal::adc::{adc_read_conversion_result, adc_is_conversion_complete, adc_start_conversion};
use crate::mcal::adc_type::{ADCReturnType};
use crate::bsw::iohwab::iohwab_type::{SensorType, SensorStatusType, IoHwAb_SensorConfig, IoHwAb_ReturnType};
use core::sync::atomic::{AtomicU8, AtomicU16, Ordering};
use crate::bsw::cfg::iohwab_sensor_cfg::{SENSOR_CONFIG, SENSOR_COUNT};


static SENSOR_VALUE_TABLE: [AtomicU16; SENSOR_COUNT] =
    [const { AtomicU16::new(0) }; SENSOR_COUNT];

static SENSOR_STATUS_TABLE: [AtomicU8; SENSOR_COUNT] =
    [const { AtomicU8::new(SensorStatusType::SENSOR_IDLE as u8) }; SENSOR_COUNT];

fn iohwab_save_sensor_value_by_index(index: usize, value: u16) {
    SENSOR_VALUE_TABLE[index].store(value, Ordering::Relaxed);
}
fn iohwab_get_sensor_value_by_index(index: usize) -> u16 {
    SENSOR_VALUE_TABLE[index].load(Ordering::Relaxed)
}
fn iohwab_get_sensor_cfg(sensor: SensorType) -> Option<&'static IoHwAb_SensorConfig> {
    for config in SENSOR_CONFIG.iter() {
        if config.sensor_id == sensor {
            return Some(config);
        }
    }
    None
}
fn iohwab_get_sensor_index(sensor: SensorType) -> Option<usize> {
    for (index, config) in SENSOR_CONFIG.iter().enumerate() {
        if config.sensor_id == sensor {
            return Some(index);
        }
    }
    None
}
fn sensor_set_status_by_index(index: usize, status: SensorStatusType) {
    SENSOR_STATUS_TABLE[index].store(status as u8, Ordering::Relaxed);
}

fn sensor_get_status_by_index(index: usize) -> SensorStatusType {
    match SENSOR_STATUS_TABLE[index].load(Ordering::Relaxed) {
        0 => SensorStatusType::SENSOR_IDLE,
        1 => SensorStatusType::SENSOR_CONVERTING,
        2 => SensorStatusType::SENSOR_COMPLETE,
        3 => SensorStatusType::SENSOR_ERROR,
        _ => SensorStatusType::SENSOR_ERROR, // Mặc định là lỗi nếu giá trị không hợp lệ
    }
}

fn iohwab_adc_error_recovery_procedure(index: usize) {
    sensor_set_status_by_index(index, SensorStatusType::SENSOR_IDLE);
}
fn iohwab_sensor_start_measurement(sensor: SensorType) {
    let cfg = iohwab_get_sensor_cfg(sensor);
    if let Some(cfg) = cfg {
        let status = sensor_get_status_by_index(cfg.index);
        if status == SensorStatusType::SENSOR_CONVERTING {
            // if the sensor is already converting, we can choose to ignore or handle it differently
            return;
        }
        if status == SensorStatusType::SENSOR_ERROR {
            // If the sensor is in error state, we can choose to reset it or handle it differently
            iohwab_adc_error_recovery_procedure(cfg.index);
        }
        adc_start_conversion(cfg.adc_channel as u8);
        sensor_set_status_by_index(cfg.index, SensorStatusType::SENSOR_CONVERTING);
    }
}

// Main function to be called periodically to handle sensor state transitions
fn iohwab_sensor_poll_measurement(sensor : SensorType) {
    if let Some(cfg) = iohwab_get_sensor_cfg(sensor) {
        let index = cfg.index;
        let status = sensor_get_status_by_index(index);
        if status == SensorStatusType::SENSOR_CONVERTING {
            let st_complete = adc_is_conversion_complete();
            if st_complete == ADCReturnType::ADC_E_OK {
                let mut data: u16 = 0;
                let st_read = adc_read_conversion_result(&mut data);
                if st_read == ADCReturnType::ADC_E_OK {
                    iohwab_save_sensor_value_by_index(index, data);
                    sensor_set_status_by_index(index, SensorStatusType::SENSOR_COMPLETE);
                } else {
                    sensor_set_status_by_index(index, SensorStatusType::SENSOR_ERROR);
                }
            } else {
                return; // Still converting, do nothing
            }
        }
    }
}

pub fn iohwab_sensor_mainfunction() {
    for cfg in SENSOR_CONFIG.iter() {
            let status = sensor_get_status_by_index(cfg.index);
            match status {
                SensorStatusType::SENSOR_IDLE => {
                    iohwab_sensor_start_measurement(cfg.sensor_id);
                }
                SensorStatusType::SENSOR_CONVERTING => {
                    iohwab_sensor_poll_measurement(cfg.sensor_id);
                }
                SensorStatusType::SENSOR_COMPLETE => {
                    // Do nothing.
                    // App/IoIf will read latest value.
                }
                SensorStatusType::SENSOR_ERROR => {
                    iohwab_adc_error_recovery_procedure(cfg.index);
                }
        }
    }
}
pub fn iohwab_sensor_read_latest_value(sensor: SensorType, data: &mut u16) -> IoHwAb_ReturnType {
    let index = iohwab_get_sensor_index(sensor);
    if let Some(index) = index {
        let status = sensor_get_status_by_index(index);
        if status == SensorStatusType::SENSOR_COMPLETE {
            *data = iohwab_get_sensor_value_by_index(index);
            sensor_set_status_by_index(index, SensorStatusType::SENSOR_IDLE);
            return IoHwAb_ReturnType::IOHWAB_E_OK;
        } else {
            return IoHwAb_ReturnType::IOHWAB_E_NOT_OK;
        }
    } else {
        return IoHwAb_ReturnType::IOHWAB_E_NOT_OK;
    }
}


