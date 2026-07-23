
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]


use crate::mcal::adc_type::{ADC_ConfigType, ADC_ChannelConfig, ADCChannelType, ADCModeType, ADCSampleTimeType, ADCMethodType,
    ADCResolutionType, ADCAlignmentType, ADCSequenceType, ADCConversionType, ADCTriggerSourceType, ADCTriggerEdgeType};

pub const ADC_CONFIG: ADC_ConfigType = ADC_ConfigType {
    channels: &[
        ADC_ChannelConfig {
            channel: ADCChannelType::ADC_CHANNEL_8,
            mode: ADCModeType::ADC_MODE_SINGLE,
            sample_time: ADCSampleTimeType::ADC_SAMPLE_TIME_84CYCLES,
            resolution: ADCResolutionType::ADC_RESOLUTION_12BIT,
            alignment: ADCAlignmentType::ADC_ALIGNMENT_RIGHT,
            sequence: ADCSequenceType::ADC_SEQUENCE_1,
            conversion: ADCConversionType::ADC_CONVERSION_REGULAR,
            trigger_source: ADCTriggerSourceType::ADC_TRIGGER_SOFTWARE,
            trigger_edge: ADCTriggerEdgeType::ADC_TRIGGER_EDGE_RISING,
            method: ADCMethodType::POLLING,
        },
    ]
};
