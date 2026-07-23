#![allow(dead_code)]

use crate::bsw::ioif::{ioif_rx::ioif_read_rx_value};
use crate::bsw::ioif::ioif_type::{IoIf_ReturnType};

pub fn temperature_measurement_app_1ms() -> u16 {
    let mut adc_raw_value: u16 = 0;
    let temperature: f32;
    if ioif_read_rx_value(0x101, &mut adc_raw_value) == IoIf_ReturnType::IOIF_E_OK {
        temperature = (adc_raw_value as f32) * 3.3 / 4095.0 * 100.0; // Convert ADC value to temperature in Celsius
        temperature as u16 // Return the temperature as u16
    } else {
        0 // Return 0 if reading fails
    }
}