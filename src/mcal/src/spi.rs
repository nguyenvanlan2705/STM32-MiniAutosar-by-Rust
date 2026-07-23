#![allow(dead_code)]
use core::sync::atomic::{Ordering};
use crate::mcal::dio::dio_writechannel;
use crate::mcal::dio_type::Dio_ChannelType;
use crate::mcal::spi_type::{SPIClockPolarityType, SPIMethodeType, SPIReturnType, SPIModeType, SPISlaveStatus, SPIMasterStatus};
use crate::mcal::cfg::spi_cfg::{SPI_CONFIG, SPI_SLAVE_STATUS, SPI_MASTER_STATUS, SPI_TX_DATA, SPI_RX_DATA, 
    SPI_MASTER_STATUS_CONFIG, SPI_SLAVE_STATUS_CONFIG};
use crate::register::gpio_type::Dio_LevelType;
use crate::register::spi_type::SPINumberType;
use crate::register::spi::{r_spi_disable_interrupt, r_spi_enable, r_spi_enable_interrupt, 
    r_spi_enable_peripheral_clock, r_spi_is_busy, r_spi_is_receive_complete, 
    r_spi_is_transmit_complete, r_spi_read_data_8bit_non_blocking, 
    r_spi_set_baud_rate, r_spi_set_chip_select, r_spi_set_clock_polarity, r_spi_set_data_frame_format, 
    r_spi_set_frame_format, r_spi_set_frame_mode, r_spi_set_master_or_slave, 
    r_spi_write_data_8bit_non_blocking, r_spi_clear_overrun_flag, r_spi_is_overrun};


fn spi_get_status_index(spi_number: SPINumberType, ismaster: bool) -> Option<usize> {
    if ismaster {
        for channel in SPI_MASTER_STATUS_CONFIG.channels.iter() {
            if channel.spi_number == spi_number {
                return Some(channel.status_index);
            }
        }
    } else {
        for channel in SPI_SLAVE_STATUS_CONFIG.channels.iter() {
            if channel.spi_number == spi_number {
                return Some(channel.status_index);
            }
        }
    }
    None
}
fn spi_get_master_status(spi_number: SPINumberType) -> SPIMasterStatus {
    if let Some(index) = spi_get_status_index(spi_number, true) {
        match SPI_MASTER_STATUS[index].load(Ordering::SeqCst) {
            0 => SPIMasterStatus::SPI_MASTER_IDLE,
            1 => SPIMasterStatus::SPI_MASTER_REQUESTED,
            2 => SPIMasterStatus::SPI_MASTER_BUSY,
            3 => SPIMasterStatus::SPI_MASTER_COMPLETE,
            4 => SPIMasterStatus::SPI_MASTER_TIMEOUT,
            5 => SPIMasterStatus::SPI_MASTER_OVERRUN,
            6 => SPIMasterStatus::SPI_MASTER_ERROR,
            _ => SPIMasterStatus::SPI_MASTER_ERROR, // Default case for unexpected values
        }
    } else {
        SPIMasterStatus::SPI_MASTER_ERROR // Default if index not found
    }
}

fn spi_set_master_status(spi_number: SPINumberType, status: SPIMasterStatus) {
    if let Some(index) = spi_get_status_index(spi_number, true) {
        SPI_MASTER_STATUS[index].store(status as u8, Ordering::SeqCst);
    }
}

fn spi_get_slave_status(spi_number: SPINumberType) -> SPISlaveStatus {
    if let Some(index) = spi_get_status_index(spi_number, false) {
        match SPI_SLAVE_STATUS[index].load(Ordering::SeqCst) {
            0 => SPISlaveStatus::SPI_SLAVE_IDLE,
            1 => SPISlaveStatus::SPI_SLAVE_PRELOADED,
            2 => SPISlaveStatus::SPI_SLAVE_BUSY,
            3 => SPISlaveStatus::SPI_SLAVE_RECEIVED,
            4 => SPISlaveStatus::SPI_SLAVE_OVERRUN,
            5 => SPISlaveStatus::SPI_SLAVE_ERROR,
            _ => SPISlaveStatus::SPI_SLAVE_ERROR, // Default case for unexpected values
        }
    } else {
        SPISlaveStatus::SPI_SLAVE_ERROR // Default if index not found
    }
}

fn spi_set_slave_status(spi_number: SPINumberType, status: SPISlaveStatus) {
    if let Some(index) = spi_get_status_index(spi_number, false) {
        SPI_SLAVE_STATUS[index].store(status as u8, Ordering::SeqCst);
    }
}
fn spi_save_tx_data(spi_number: SPINumberType, data: u8) {
    let index = match spi_number {
        SPINumberType::SPI1 => 0,
        SPINumberType::SPI2 => 1,
        SPINumberType::SPI3 => 2,
        SPINumberType::SPI4 => 3,
    };
    SPI_TX_DATA[index].store(data, Ordering::SeqCst);
}

fn spi_get_tx_data(spi_number: SPINumberType) -> u8 {
    let index = match spi_number {
        SPINumberType::SPI1 => 0,
        SPINumberType::SPI2 => 1,
        SPINumberType::SPI3 => 2,
        SPINumberType::SPI4 => 3,
    };
    SPI_TX_DATA[index].load(Ordering::SeqCst)
}

fn spi_save_rx_data(spi_number: SPINumberType, data: u8) {
    let index = match spi_number {
        SPINumberType::SPI1 => 0,
        SPINumberType::SPI2 => 1,
        SPINumberType::SPI3 => 2,
        SPINumberType::SPI4 => 3,
    };
    SPI_RX_DATA[index].store(data, Ordering::SeqCst);
}

fn spi_get_rx_data(spi_number: SPINumberType) -> u8 {
    let index = match spi_number {
        SPINumberType::SPI1 => 0,
        SPINumberType::SPI2 => 1,
        SPINumberType::SPI3 => 2,
        SPINumberType::SPI4 => 3,
    };
    SPI_RX_DATA[index].load(Ordering::SeqCst)
}

pub fn spi_init(){
    // Disable the chip select pin for the onboard SPI sensor to avoid any accidental communication during initialization
    dio_writechannel(Dio_ChannelType::OnboardSpiSensorCs, Dio_LevelType::HIGH);
    for channel_config in SPI_CONFIG.channels.iter() {
        r_spi_enable_peripheral_clock(channel_config.spi_number);
        r_spi_set_master_or_slave(channel_config.spi_number, channel_config.mode);
        r_spi_set_baud_rate(channel_config.spi_number, channel_config.baud_rate_prescaler as u8);
        r_spi_set_data_frame_format(channel_config.spi_number, channel_config.data_frame_format);
        r_spi_set_frame_format(channel_config.spi_number, channel_config.frame_format);
        r_spi_set_frame_mode(channel_config.spi_number, channel_config.frame_mode as u8);
        r_spi_set_chip_select(channel_config.spi_number, channel_config.nss_control, channel_config.mode);
        if channel_config.clock_polarity == SPIClockPolarityType::SPI_CLOCK_POLARITY_HIGH {
            r_spi_set_clock_polarity(channel_config.spi_number, 1 , channel_config.clock_phase as u8);
        } else {
            r_spi_set_clock_polarity(channel_config.spi_number, 0, channel_config.clock_phase as u8);
        }
        if channel_config.methode == SPIMethodeType::SPI_INTERRUPT {
            r_spi_enable_interrupt(channel_config.spi_number);
        } else {
            r_spi_disable_interrupt(channel_config.spi_number);
        }
        r_spi_enable(channel_config.spi_number);
    }
}

fn spi_error_recovery_procedure(spi_number: SPINumberType, is_master: bool) {
    // Clear the global status to IDLE
    if is_master {
        spi_set_master_status(spi_number, SPIMasterStatus::SPI_MASTER_IDLE);
    } else {
        spi_set_slave_status(spi_number, SPISlaveStatus::SPI_SLAVE_IDLE);
    }
}

/* Main non-blocking SPI state-machine APIs */
pub fn spi_slave_receive_byte(spi_number: SPINumberType, rx_data: &mut u8) -> SPIReturnType {
    if spi_get_slave_status(spi_number) != SPISlaveStatus::SPI_SLAVE_RECEIVED {
        return SPIReturnType::SPI_E_NOT_OK;
    }
    *rx_data = spi_get_rx_data(spi_number);
    spi_set_slave_status(spi_number, SPISlaveStatus::SPI_SLAVE_IDLE);
    SPIReturnType::SPI_E_OK
}

pub fn spi_master_receive_byte(spi_number: SPINumberType, rx_data: &mut u8) -> SPIReturnType {
    if spi_get_master_status(spi_number) != SPIMasterStatus::SPI_MASTER_COMPLETE {
        return SPIReturnType::SPI_E_NOT_OK;
    }

    *rx_data = spi_get_rx_data(spi_number);
    spi_set_master_status(spi_number, SPIMasterStatus::SPI_MASTER_IDLE);

    SPIReturnType::SPI_E_OK
}
// Called by the application to request a SPI transfer in master mode.
pub fn spi_master_start_to_transfer(spi_number: SPINumberType, tx_data: u8) -> SPIReturnType {
    if r_spi_is_overrun(spi_number) {
        r_spi_clear_overrun_flag(spi_number);
        spi_set_master_status(spi_number, SPIMasterStatus::SPI_MASTER_OVERRUN);
        return SPIReturnType::SPI_E_NOT_OK;
    }
    if r_spi_is_busy(spi_number) {
        spi_set_master_status(spi_number, SPIMasterStatus::SPI_MASTER_ERROR);
        return SPIReturnType::SPI_E_NOT_OK;
    }
    if !r_spi_is_transmit_complete(spi_number) {
        spi_set_master_status(spi_number, SPIMasterStatus::SPI_MASTER_ERROR);
        return SPIReturnType::SPI_E_NOT_OK;
    }
    spi_set_master_status(spi_number, SPIMasterStatus::SPI_MASTER_REQUESTED);
    spi_save_tx_data(spi_number, tx_data);
    SPIReturnType::SPI_E_OK
}

// Called by the application to prepare the slave response data before the master clocks the bus.
pub fn spi_slave_start_to_preload(spi_number: SPINumberType, tx_data: u8) -> SPIReturnType {
    if r_spi_is_overrun(spi_number) {
        r_spi_clear_overrun_flag(spi_number);
        spi_set_slave_status(spi_number, SPISlaveStatus::SPI_SLAVE_OVERRUN);
        return SPIReturnType::SPI_E_NOT_OK;
    }
    spi_save_tx_data(spi_number, tx_data);
    spi_set_slave_status(spi_number, SPISlaveStatus::SPI_SLAVE_PRELOADED);
    SPIReturnType::SPI_E_OK
}

fn spi_mainfunction_master(spi_number: SPINumberType) {
    let master_status = spi_get_master_status(spi_number);
    match master_status {
        SPIMasterStatus::SPI_MASTER_IDLE => {
            // Do nothing, waiting for a transfer to be initiated
        }
        SPIMasterStatus::SPI_MASTER_REQUESTED => { 
            if r_spi_is_transmit_complete(spi_number) {
                let tx_data = spi_get_tx_data(spi_number);
                r_spi_write_data_8bit_non_blocking(spi_number, tx_data);
                spi_set_master_status(spi_number, SPIMasterStatus::SPI_MASTER_BUSY);
            }
        }
        SPIMasterStatus::SPI_MASTER_BUSY => {
            if r_spi_is_receive_complete(spi_number) {
                if let Some(rx_data) = r_spi_read_data_8bit_non_blocking(spi_number) {
                    spi_save_rx_data(spi_number, rx_data);
                    if !r_spi_is_busy(spi_number) {
                        spi_set_master_status(spi_number, SPIMasterStatus::SPI_MASTER_COMPLETE);
                    }
                }
            }
        }
        SPIMasterStatus::SPI_MASTER_COMPLETE => {
            // Transfer complete, waiting for application to read the data
        }
        SPIMasterStatus::SPI_MASTER_TIMEOUT => {
            // Handle timeout error
            spi_set_master_status(spi_number, SPIMasterStatus::SPI_MASTER_TIMEOUT);
        }
        SPIMasterStatus::SPI_MASTER_OVERRUN => {
            // Handle overrun error
            spi_set_master_status(spi_number, SPIMasterStatus::SPI_MASTER_ERROR);
        }
        SPIMasterStatus::SPI_MASTER_ERROR => {
            // Handle error recovery
            spi_error_recovery_procedure(spi_number, true);
        }
    }
}

fn spi_mainfunction_slave(spi_number: SPINumberType) {
    let slave_status = spi_get_slave_status(spi_number);
    match slave_status {
        SPISlaveStatus::SPI_SLAVE_IDLE => {
            // Do nothing, waiting for a transfer to be initiated
        }
        SPISlaveStatus::SPI_SLAVE_PRELOADED => {
            if r_spi_is_transmit_complete(spi_number) {
                let tx_data = spi_get_tx_data(spi_number);
                r_spi_write_data_8bit_non_blocking(spi_number, tx_data);
                spi_set_slave_status(spi_number, SPISlaveStatus::SPI_SLAVE_BUSY);
            }
        }
        SPISlaveStatus::SPI_SLAVE_BUSY => {
            if r_spi_is_receive_complete(spi_number) {
                if let Some(rx_data) = r_spi_read_data_8bit_non_blocking(spi_number) {
                    spi_save_rx_data(spi_number, rx_data);
                    spi_set_slave_status(spi_number, SPISlaveStatus::SPI_SLAVE_RECEIVED);
                }
            }
        }
        SPISlaveStatus::SPI_SLAVE_RECEIVED => {
            // Transfer complete, waiting for application to read the data
        }
        SPISlaveStatus::SPI_SLAVE_OVERRUN => {
            // Handle overrun error
            spi_set_slave_status(spi_number, SPISlaveStatus::SPI_SLAVE_ERROR);
        }
        SPISlaveStatus::SPI_SLAVE_ERROR => {
            // Handle error recovery
            spi_error_recovery_procedure(spi_number, false);
        }
    }
}
// main function to be called for changing the state of the SPI driver, to be called in the main loop of the application
pub fn spi_mainfunction() {
    for channel in SPI_CONFIG.channels.iter() {
        if channel.mode == SPIModeType::SPI_MODE_MASTER {
            spi_mainfunction_master(channel.spi_number);
        } else {
            spi_mainfunction_slave(channel.spi_number);
        }
    }
}

/* Bring-up/test-only polling helpers */

pub fn spi_wait_tx_ready(spi_number: SPINumberType, timeout: u32) -> SPIReturnType {
    let mut elapsed = 0;
    while !r_spi_is_transmit_complete(spi_number) {
        if elapsed >= timeout {
            return SPIReturnType::SPI_E_NOT_OK;
        }
        elapsed += 1;
    }
    SPIReturnType::SPI_E_OK
}

pub fn spi_wait_rx_ready(spi_number: SPINumberType, timeout: u32) -> SPIReturnType {
    let mut elapsed = 0;
    while !r_spi_is_receive_complete(spi_number) {
        if elapsed >= timeout {
            return SPIReturnType::SPI_E_NOT_OK;
        }
        elapsed += 1;
    }
    SPIReturnType::SPI_E_OK
}

pub fn spi_wait_tx_rx_not_busy(spi_number: SPINumberType, timeout: u32) -> SPIReturnType {
    let mut elapsed = 0;
    while r_spi_is_busy(spi_number) {
        if elapsed >= timeout {
            return SPIReturnType::SPI_E_NOT_OK;
        }
        elapsed += 1;
    }
    SPIReturnType::SPI_E_OK
}

pub fn spi_master_transfer_byte(spi_number: SPINumberType, tx_data: u8, rx_data: &mut u8) -> SPIReturnType {
    if spi_wait_tx_ready(spi_number, 1000) != SPIReturnType::SPI_E_OK {
        return SPIReturnType::SPI_E_NOT_OK;
    }
    r_spi_write_data_8bit_non_blocking(spi_number, tx_data);
    if spi_wait_rx_ready(spi_number, 1000) != SPIReturnType::SPI_E_OK {
        return SPIReturnType::SPI_E_NOT_OK;
    }
    let result = r_spi_read_data_8bit_non_blocking(spi_number);
    if let Some(value) = result {
        *rx_data = value;
        SPIReturnType::SPI_E_OK
    } else {
        SPIReturnType::SPI_E_NOT_OK
    }
}

pub fn spi_slave_preload_byte(spi_number: SPINumberType, data: u8) {
    if spi_wait_tx_ready(spi_number, 1000) != SPIReturnType::SPI_E_OK {
        return;
    }
    r_spi_write_data_8bit_non_blocking(spi_number, data);
}

pub fn spi_slave_ready_to_receive(spi_number: SPINumberType) -> bool {
    r_spi_is_receive_complete(spi_number)
}


