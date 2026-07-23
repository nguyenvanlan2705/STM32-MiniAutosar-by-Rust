#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
use crate::register::spi_type::{SPINumberType};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SPIReturnType {
    SPI_E_OK,
    SPI_E_NOT_OK,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SPIMasterStatus{
    SPI_MASTER_IDLE,
    SPI_MASTER_REQUESTED,
    SPI_MASTER_BUSY,
    SPI_MASTER_COMPLETE,
    SPI_MASTER_TIMEOUT,
    SPI_MASTER_OVERRUN,
    SPI_MASTER_ERROR,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SPISlaveStatus{
    SPI_SLAVE_IDLE,
    SPI_SLAVE_PRELOADED,
    SPI_SLAVE_BUSY,
    SPI_SLAVE_RECEIVED,
    SPI_SLAVE_OVERRUN,
    SPI_SLAVE_ERROR,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SPIModeType {
    SPI_MODE_SLAVE = 0,
    SPI_MODE_MASTER = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SPIClockPolarityType {
    SPI_CLOCK_POLARITY_LOW = 0,
    SPI_CLOCK_POLARITY_HIGH = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SPIClockPhaseType {
    SPI_CLOCK_PHASE_1EDGE = 0,
    SPI_CLOCK_PHASE_2EDGE = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SPIDataFrameFormatType {
    SPI_8BIT = 0,
    SPI_16BIT = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SPIBaudRatePrescalerType {
    SPI_BAUDRATEPRESCALER_2 = 0,
    SPI_BAUDRATEPRESCALER_4 = 1,
    SPI_BAUDRATEPRESCALER_8 = 2,
    SPI_BAUDRATEPRESCALER_16 = 3,
    SPI_BAUDRATEPRESCALER_32 = 4,
    SPI_BAUDRATEPRESCALER_64 = 5,
    SPI_BAUDRATEPRESCALER_128 = 6,
    SPI_BAUDRATEPRESCALER_256 = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SPIFrameFormatType{
    SPI_MSB_FIRST = 0,
    SPI_LSB_FIRST = 1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SPIFrameModeType{
    SPI_FRAME_MOTOROLA = 0,
    SPI_FRAME_TI = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SPIMethodeType{
    SPI_POLLING = 0,
    SPI_INTERRUPT = 1,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SPINSSControlType{
    SPI_NSS_HARDWARE = 0,
    SPI_NSS_SOFTWARE = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]  
pub struct SPI_ChannelConfigType {
    pub spi_number: SPINumberType,
    pub mode: SPIModeType,
    pub clock_polarity: SPIClockPolarityType,
    pub clock_phase: SPIClockPhaseType,
    pub data_frame_format: SPIDataFrameFormatType,
    pub baud_rate_prescaler: SPIBaudRatePrescalerType,
    pub frame_format: SPIFrameFormatType,
    pub frame_mode: SPIFrameModeType,
    pub crc_enabled: bool,
    pub nss_control: SPINSSControlType,
    pub methode: SPIMethodeType,
}

pub struct SPI_ConfigType {
    pub channels: &'static [SPI_ChannelConfigType],
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)] 
pub struct SPIMasterStatusChannelType{
    pub spi_number: SPINumberType,
    pub status_index: usize,
}
pub struct SPIMasterStatusChannelConfig {
    pub channels: &'static [SPIMasterStatusChannelType],
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SPISlaveStatusChannelType{
    pub spi_number: SPINumberType,
    pub status_index: usize,
}
pub struct SPISlaveStatusChannelConfig {
    pub channels: &'static [SPISlaveStatusChannelType],
}