#![allow(dead_code)]

use crate::register::adc::{ r_adc_enable, r_adc_is_conversion_complete, r_adc_read_data,
    r_adc_enable_interrupt, r_adc_set_trigger_source, r_adc_set_alignment, r_adc_set_channel, r_adc_set_trigger_edge,
    r_adc_set_resolution, r_adc_set_sample_time, r_adc_set_sequence, r_adc_single_conversion_start, r_adc_enable_peripheral_clock};
use crate::mcal::adc_type::{ADCMethodType, ADCConversionType, ADCReturnType };
use crate::mcal::cfg::adc_cfg::ADC_CONFIG;

pub fn adc_init(){
    for channel_config in ADC_CONFIG.channels.iter() {
        r_adc_enable_peripheral_clock();
        r_adc_set_channel(channel_config.channel as u8);
        r_adc_set_sample_time(channel_config.channel as u8, channel_config.sample_time as u8);
        r_adc_set_sequence(&[channel_config.channel as u8]);
        r_adc_set_resolution(channel_config.resolution as u8);
        r_adc_set_alignment(channel_config.alignment);
        r_adc_set_trigger_source(channel_config.trigger_source as u8);
        r_adc_set_trigger_edge(channel_config.trigger_edge as u8);

        if channel_config.conversion == ADCConversionType::ADC_CONVERSION_INJECTED {
            // Configure for injected conversion
        } else {
            // Configure for regular conversion
        }
        if channel_config.method == ADCMethodType::INTERRUPT {
            // Configure for interrupt method
            r_adc_enable_interrupt();
        } else if channel_config.method == ADCMethodType::DMA {
            // Configure for DMA method
        }else{
            // polling method
        }
        r_adc_enable();
    }
}

//Function to avoid stuck in while loop if ADC is not ready
pub fn adc_wait_for_conversion_complete(timeout: u32) -> bool {
    let mut count = 0;
    while !r_adc_is_conversion_complete() {
        if count >= timeout {
            return false;
        }
        count += 1;
        
    }
    true
}
pub fn adc_start_conversion(channel: u8) {
    r_adc_set_channel(channel);
    r_adc_single_conversion_start();
}

pub fn adc_convert_complete() -> ADCReturnType {
    if !adc_wait_for_conversion_complete(1000) {
        return ADCReturnType::ADC_E_TIMEOUT;
    }
    ADCReturnType::ADC_E_OK
}
// Function to read ADC value for a specific channel
pub fn adc_read_conversion_result(data: &mut u16) -> ADCReturnType {
    *data = r_adc_read_data();
    ADCReturnType::ADC_E_OK
}

// Non-blocking function to check if ADC conversion is complete
pub fn adc_is_conversion_complete() -> ADCReturnType {
    if r_adc_is_conversion_complete() {
        ADCReturnType::ADC_E_OK
    } else {
        ADCReturnType::ADC_E_NOT_OK
    }
}