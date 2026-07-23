#![allow(dead_code)]

use core::sync::atomic::{AtomicU8};
use crate::register::spi_type::{SPINumberType};
use crate::mcal::spi_type::{
    SPI_ConfigType,
    SPI_ChannelConfigType,
    SPIModeType,
    SPIClockPolarityType,
    SPIClockPhaseType,
    SPIDataFrameFormatType,
    SPIBaudRatePrescalerType,
    SPIFrameFormatType,
    SPIFrameModeType,
    SPIMethodeType,
    SPINSSControlType,
    SPIMasterStatus,
    SPISlaveStatus,
    SPISlaveStatusChannelConfig,
    SPISlaveStatusChannelType,
    SPIMasterStatusChannelConfig,
    SPIMasterStatusChannelType,
};

pub const SPI_CONFIG: SPI_ConfigType = SPI_ConfigType {
    channels: &[
        SPI_ChannelConfigType {
            spi_number: SPINumberType::SPI1,
            mode: SPIModeType::SPI_MODE_MASTER,
            clock_polarity: SPIClockPolarityType::SPI_CLOCK_POLARITY_LOW,
            clock_phase: SPIClockPhaseType::SPI_CLOCK_PHASE_1EDGE,
            data_frame_format: SPIDataFrameFormatType::SPI_8BIT,
            baud_rate_prescaler: SPIBaudRatePrescalerType::SPI_BAUDRATEPRESCALER_256,
            frame_format: SPIFrameFormatType::SPI_MSB_FIRST,
            frame_mode: SPIFrameModeType::SPI_FRAME_MOTOROLA,
            crc_enabled: false,
            nss_control: SPINSSControlType::SPI_NSS_SOFTWARE,
            methode: SPIMethodeType::SPI_POLLING,
        },
        SPI_ChannelConfigType {
            spi_number: SPINumberType::SPI2,
            mode: SPIModeType::SPI_MODE_SLAVE,
            clock_polarity: SPIClockPolarityType::SPI_CLOCK_POLARITY_LOW,
            clock_phase: SPIClockPhaseType::SPI_CLOCK_PHASE_1EDGE,
            data_frame_format: SPIDataFrameFormatType::SPI_8BIT,
            baud_rate_prescaler: SPIBaudRatePrescalerType::SPI_BAUDRATEPRESCALER_256,
            frame_format: SPIFrameFormatType::SPI_MSB_FIRST,
            frame_mode: SPIFrameModeType::SPI_FRAME_MOTOROLA,
            crc_enabled: false,
            nss_control: SPINSSControlType::SPI_NSS_SOFTWARE,
            methode: SPIMethodeType::SPI_POLLING,
        },
    ]
};

pub const SPI_CHANNEL_COUNT: usize = SPI_CONFIG.channels.len();
pub const SPI_CHANNEL_MASTER_COUNT: usize = 1;  
pub const SPI_CHANNEL_SLAVE_COUNT: usize = 1;

// SPI TX Data Array
pub static SPI_TX_DATA : [AtomicU8; SPI_CHANNEL_COUNT] = [
    AtomicU8::new(0),
    AtomicU8::new(0),
];
pub static SPI_RX_DATA : [AtomicU8; SPI_CHANNEL_COUNT] = [
    AtomicU8::new(0),
    AtomicU8::new(0),
];
pub static SPI_MASTER_STATUS: [AtomicU8; SPI_CHANNEL_MASTER_COUNT] = [
    AtomicU8::new(SPIMasterStatus::SPI_MASTER_IDLE as u8),
];
pub static SPI_SLAVE_STATUS: [AtomicU8; SPI_CHANNEL_SLAVE_COUNT] = [
    AtomicU8::new(SPISlaveStatus::SPI_SLAVE_IDLE as u8),
];

pub static SPI_MASTER_STATUS_CONFIG: SPIMasterStatusChannelConfig = SPIMasterStatusChannelConfig {
    channels: &[
        SPIMasterStatusChannelType {
            spi_number: SPINumberType::SPI1,
            status_index: 0,
        },
    ],
};

pub static SPI_SLAVE_STATUS_CONFIG: SPISlaveStatusChannelConfig = SPISlaveStatusChannelConfig {
    channels: &[
        SPISlaveStatusChannelType {
            spi_number: SPINumberType::SPI2,
            status_index: 0,
        },
    ],
};