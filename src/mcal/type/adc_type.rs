#![allow(dead_code)]
#![allow(non_camel_case_types)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)] 
pub enum ADCReturnType{
    ADC_E_OK,
    ADC_E_NOT_OK,
    ADC_E_TIMEOUT,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)] 
pub enum ADCResolutionType {
    ADC_RESOLUTION_12BIT = 0,
    ADC_RESOLUTION_10BIT = 1,
    ADC_RESOLUTION_8BIT = 2,
    ADC_RESOLUTION_6BIT = 3,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]  
pub enum ADCAlignmentType {
    ADC_ALIGNMENT_RIGHT = 0,
    ADC_ALIGNMENT_LEFT = 1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]  
pub enum ADCModeType {
    ADC_MODE_SINGLE = 0,
    ADC_MODE_CONTINUOUS = 1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)] 
pub enum ADCTriggerSourceType {
    ADC_TRIGGER_SOFTWARE = 0,
    ADC_TRIGGER_TIMER1_CC1 = 1,
    ADC_TRIGGER_TIMER1_CC2 = 2,
    ADC_TRIGGER_TIMER1_CC3 = 3,
    ADC_TRIGGER_TIMER2_CC2 = 4,
    ADC_TRIGGER_TIMER3_TRGO = 5,
    ADC_TRIGGER_TIMER4_CC4 = 6,
    ADC_TRIGGER_EXTI_LINE11 = 7,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]  
pub enum ADCTriggerEdgeType {
    ADC_TRIGGER_EDGE_RISING = 0,
    ADC_TRIGGER_EDGE_FALLING = 1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]  
pub enum ADCSequenceType {
    ADC_SEQUENCE_1 = 0,
    ADC_SEQUENCE_2 = 1,
    ADC_SEQUENCE_3 = 2,
    ADC_SEQUENCE_4 = 3,
    ADC_SEQUENCE_5 = 4,
    ADC_SEQUENCE_6 = 5,
    ADC_SEQUENCE_7 = 6,
    ADC_SEQUENCE_8 = 7,
    ADC_SEQUENCE_9 = 8,
    ADC_SEQUENCE_10 = 9,
    ADC_SEQUENCE_11 = 10,
    ADC_SEQUENCE_12 = 11,
    ADC_SEQUENCE_13 = 12,
    ADC_SEQUENCE_14 = 13,
    ADC_SEQUENCE_15 = 14,
    ADC_SEQUENCE_16 = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]  
pub enum ADCChannelType {
    ADC_CHANNEL_0 = 0,
    ADC_CHANNEL_1 = 1,
    ADC_CHANNEL_2 = 2,
    ADC_CHANNEL_3 = 3,
    ADC_CHANNEL_4 = 4,
    ADC_CHANNEL_5 = 5,
    ADC_CHANNEL_6 = 6,
    ADC_CHANNEL_7 = 7,
    ADC_CHANNEL_8 = 8,
    ADC_CHANNEL_9 = 9,
    ADC_CHANNEL_10 = 10,
    ADC_CHANNEL_11 = 11,
    ADC_CHANNEL_12 = 12,
    ADC_CHANNEL_13 = 13,
    ADC_CHANNEL_14 = 14,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]  
pub enum ADCSampleTimeType {
    ADC_SAMPLE_TIME_3CYCLES = 0,
    ADC_SAMPLE_TIME_15CYCLES = 1,
    ADC_SAMPLE_TIME_28CYCLES = 2,
    ADC_SAMPLE_TIME_56CYCLES = 3,
    ADC_SAMPLE_TIME_84CYCLES = 4,
    ADC_SAMPLE_TIME_112CYCLES = 5,
    ADC_SAMPLE_TIME_144CYCLES = 6,
    ADC_SAMPLE_TIME_480CYCLES = 7,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]  
pub enum ADCStatusType {
    ADC_STATUS_IDLE = 0,
    ADC_STATUS_BUSY = 1,
    ADC_STATUS_ERROR = 2,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]  
pub enum ADCConversionType{
    ADC_CONVERSION_REGULAR = 0,
    ADC_CONVERSION_INJECTED = 1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]  
pub enum ADCMethodType {
    POLLING,
    INTERRUPT,
    DMA,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]  
pub struct ADC_ChannelConfig {
    pub channel: ADCChannelType, // ADC channel number
    pub mode: ADCModeType, // ADC mode (single or continuous)
    pub sample_time: ADCSampleTimeType, // Sample time configuration
    pub resolution: ADCResolutionType, // Resolution in bits (e.g., 12, 10, 8)
    pub alignment: ADCAlignmentType, // Data alignment (right or left)
    pub sequence: ADCSequenceType, // Sequence order for regular conversions
    pub conversion: ADCConversionType, // Whether the channel is used for injected conversions
    pub trigger_source: ADCTriggerSourceType, // Trigger source for injected conversions
    pub trigger_edge: ADCTriggerEdgeType, // Trigger edge (rising or falling) for injected conversions
    pub method: ADCMethodType, // Whether to enable interrupt for this channel
}

pub struct ADC_ConfigType {
    pub channels: &'static [ADC_ChannelConfig],
}