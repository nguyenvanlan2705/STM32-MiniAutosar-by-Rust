#![allow(dead_code)]

use crate::register::adc_type::{get_r_adc_register};
use crate::register::rcc_type::{get_rcc_register};
use crate::mcal::adc_type::{ ADCAlignmentType};

pub fn r_adc_enable_peripheral_clock() {
    unsafe {
        let rcc = get_rcc_register();
        // Enable ADC clock by setting the ADCEN bit in RCC_APB2ENR
        core::ptr::write_volatile(&mut (*rcc).rcc_apb2enr, core::ptr::read_volatile(&(*rcc).rcc_apb2enr) | (1 << 8));}
}
pub fn r_adc_enable() {
    unsafe {
        let adc = get_r_adc_register();
        // Enable ADC by setting the ADON bit in CR2
        core::ptr::write_volatile(&mut (*adc).CR2, core::ptr::read_volatile(&(*adc).CR2) | 1);
    }
}

pub fn r_adc_single_conversion_start() {
    unsafe {
        let adc = get_r_adc_register();
        // Start single conversion by setting the SWSTART bit in CR2
        core::ptr::write_volatile(&mut (*adc).CR2, core::ptr::read_volatile(&(*adc).CR2) | (1 << 30));
    }
}
pub fn r_adc_continous_conversion_start() {
    unsafe {
        let adc = get_r_adc_register();
        // Start continuous conversion by setting the CONT bit in CR2
        core::ptr::write_volatile(&mut (*adc).CR2, core::ptr::read_volatile(&(*adc).CR2) | (1 << 1));
    }
}

pub fn r_adc_is_conversion_complete() -> bool {
    unsafe {
        let adc = get_r_adc_register();
        // Check the EOC (End of Conversion) bit in SR (Status Register)
        (core::ptr::read_volatile(&(*adc).SR) & (1 << 1)) != 0
    }
}

pub fn r_adc_set_channel(channel: u8) {
    unsafe {
        let adc = get_r_adc_register();
        // Set the channel in SQR3 (Regular Sequence Register 3)
        core::ptr::write_volatile(&mut (*adc).SQR3, channel as u32);
    }
}
pub fn r_adc_set_sample_time(channel: u8, sample_time: u8) {
    unsafe {
        let adc = get_r_adc_register();
        if channel <= 9 {
            // Set sample time in SMPR2 for channels 0-9
            let smpr2_value = core::ptr::read_volatile(&(*adc).SMPR2);
            let new_smpr2_value = (smpr2_value & !(0b111 << (channel * 3))) | ((sample_time as u32) << (channel * 3));
            core::ptr::write_volatile(&mut (*adc).SMPR2, new_smpr2_value);
        } else if channel <= 18 {
            // Set sample time in SMPR1 for channels 10-18
            let smpr1_value = core::ptr::read_volatile(&(*adc).SMPR1);
            let new_smpr1_value = (smpr1_value & !(0b111 << ((channel - 10) * 3))) | ((sample_time as u32) << ((channel - 10) * 3));
            core::ptr::write_volatile(&mut (*adc).SMPR1, new_smpr1_value);
        }
    }
}

pub fn r_adc_set_sequence(sequence: &[u8]) {
    unsafe {
        let adc = get_r_adc_register();
        // Set the sequence in SQR1, SQR2, and SQR3
        let mut sqr1_value = 0u32;
        let mut sqr2_value = 0u32;
        let mut sqr3_value = 0u32;

        for (i, &channel) in sequence.iter().enumerate() {
            if i < 6 {
                sqr3_value |= (channel as u32) << (i * 5);
            } else if i < 12 {
                sqr2_value |= (channel as u32) << ((i - 6) * 5);
            } else if i < 16 {
                sqr1_value |= (channel as u32) << ((i - 12) * 5);
            }
        }

        core::ptr::write_volatile(&mut (*adc).SQR3, sqr3_value);
        core::ptr::write_volatile(&mut (*adc).SQR2, sqr2_value);
        core::ptr::write_volatile(&mut (*adc).SQR1, sqr1_value | (((sequence.len() as u32 - 1) & 0xF) << 20));
    }
}
pub fn r_adc_set_alignment(right_aligned: ADCAlignmentType ) {
    unsafe {
        let adc = get_r_adc_register();
        if right_aligned == ADCAlignmentType::ADC_ALIGNMENT_RIGHT {
            // Set right alignment by clearing the ALIGN bit in CR2
            core::ptr::write_volatile(&mut (*adc).CR2, core::ptr::read_volatile(&(*adc).CR2) & !(1 << 11));
        } else {
            // Set left alignment by setting the ALIGN bit in CR2
            core::ptr::write_volatile(&mut (*adc).CR2, core::ptr::read_volatile(&(*adc).CR2) | (1 << 11));
        }
    }
}

pub fn r_adc_set_resolution(resolution: u8) {
    unsafe {
        let adc = get_r_adc_register();
        // Set the resolution in CR1 (RES bits)
        let cr1_value = core::ptr::read_volatile(&(*adc).CR1) & !(0b11 << 24);
        let new_cr1_value = cr1_value | ((resolution as u32 & 0b11) << 24);
        core::ptr::write_volatile(&mut (*adc).CR1, new_cr1_value);
    }
}
pub fn r_adc_set_trigger_source(trigger_source: u8) {
    unsafe {
        let adc = get_r_adc_register();
        // Set the trigger source in CR2 (EXTSEL bits)
        let cr2_value = core::ptr::read_volatile(&(*adc).CR2) & !(0b1111 << 17);
        let new_cr2_value = cr2_value | ((trigger_source as u32 & 0b1111) << 17);
        core::ptr::write_volatile(&mut (*adc).CR2, new_cr2_value);
    }
}
pub fn r_adc_set_trigger_edge(trigger_edge: u8) {
    unsafe {
        let adc = get_r_adc_register();
        // Set the trigger edge in CR2 (EXTEN bits)
        let cr2_value = core::ptr::read_volatile(&(*adc).CR2) & !(0b11 << 20);
        let new_cr2_value = cr2_value | ((trigger_edge as u32 & 0b11) << 20);
        core::ptr::write_volatile(&mut (*adc).CR2, new_cr2_value);
    }
}

pub fn r_adc_read_data() -> u16 {
    unsafe {
        let adc = get_r_adc_register();
        // Read the converted data from DR (Data Register)
        core::ptr::read_volatile(&(*adc).DR) as u16
    }
}

pub fn r_adc_disable() {
    unsafe {
        let adc = get_r_adc_register();
        // Disable ADC by clearing the ADON bit in CR2
        core::ptr::write_volatile(&mut (*adc).CR2, core::ptr::read_volatile(&(*adc).CR2) & !1);
    }
}
pub fn r_adc_enable_interrupt() {
    unsafe {
        let adc = get_r_adc_register();
        // Enable ADC interrupt by setting the EOCIE bit in CR1
        core::ptr::write_volatile(&mut (*adc).CR1, core::ptr::read_volatile(&(*adc).CR1) | (1 << 5));
    }
}

pub fn r_adc_disable_interrupt() {
    unsafe {
        let adc = get_r_adc_register();
        // Disable ADC interrupt by clearing the EOCIE bit in CR1
        core::ptr::write_volatile(&mut (*adc).CR1, core::ptr::read_volatile(&(*adc).CR1) & !(1 << 5));
    }
}

