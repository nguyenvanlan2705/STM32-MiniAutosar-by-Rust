#![allow(dead_code)]

use crate::register::spi_type::{SPINumberType, get_spi_register};
use crate::register::rcc_type::{get_rcc_register};
use crate::mcal::spi_type::{SPIFrameFormatType, SPIDataFrameFormatType,SPIModeType, SPINSSControlType};

pub fn r_spi_enable_peripheral_clock(spi_number: SPINumberType) {
    let rcc =  get_rcc_register();
    unsafe{
        match spi_number {
            SPINumberType::SPI1 => {
                core::ptr::write_volatile(&mut rcc.rcc_apb2enr, core::ptr::read_volatile(&rcc.rcc_apb2enr) | (1 << 12)); // Enable SPI1 clock
            }
            SPINumberType::SPI2 => {
                core::ptr::write_volatile(&mut rcc.rcc_apb1enr, core::ptr::read_volatile(&rcc.rcc_apb1enr) | (1 << 14)); // Enable SPI2 clock
            }
            SPINumberType::SPI3 => {
                core::ptr::write_volatile(&mut rcc.rcc_apb1enr, core::ptr::read_volatile(&rcc.rcc_apb1enr) | (1 << 15)); // Enable SPI3 clock
            }
            SPINumberType::SPI4 => {
                core::ptr::write_volatile(&mut rcc.rcc_apb2enr, core::ptr::read_volatile(&rcc.rcc_apb2enr) | (1 << 13)); // Enable SPI4 clock
            }
        }
    }
}

pub fn r_spi_set_baud_rate(spi_number: SPINumberType, baud_rate: u8) {
    if let Some(spi) = get_spi_register(spi_number) {
        unsafe {
            let cr1 = core::ptr::read_volatile(&(*spi).cr1);
            let new_cr1 = (cr1 & !(0b111 << 3)) | ((baud_rate as u32 & 0b111) << 3);
            core::ptr::write_volatile(&mut (*spi).cr1, new_cr1);
        }
    }
}

pub fn r_spi_enable(spi_number: SPINumberType) {
    if let Some(spi) = get_spi_register(spi_number) {
        unsafe {
            let cr1 = core::ptr::read_volatile(&(*spi).cr1);
            core::ptr::write_volatile(&mut (*spi).cr1, cr1 | (1 << 6)); // Set SPE bit
        }
    }
}

pub fn r_spi_set_clock_polarity(spi_number: SPINumberType, cpol: u8, cpha: u8) {
    if let Some(spi) = get_spi_register(spi_number) {
        unsafe {
            let cr1 = core::ptr::read_volatile(&(*spi).cr1);
            let new_cr1 = (cr1 & !(1 << 1)) | ((cpol as u32) << 1); // Set CPOL bit
            let new_cr1 = (new_cr1 & !(1 << 0)) | ((cpha as u32) << 0); // Set CPHA bit
            core::ptr::write_volatile(&mut (*spi).cr1, new_cr1);
        }
    }
    
}

pub fn r_spi_set_chip_select(spi_number: SPINumberType, select: SPINSSControlType, mastermode: SPIModeType) {
    if let Some(spi) = get_spi_register(spi_number) {
        unsafe {
            let cr1 = core::ptr::read_volatile(&(*spi).cr1);
            let new_cr1 = if select == SPINSSControlType::SPI_NSS_SOFTWARE { 
                cr1 | (1 << 9) // Set SSM bit to select chip
            } else {
                cr1 & !(1 << 9) // Clear SSM bit to deselect chip
            };
            core::ptr::write_volatile(&mut (*spi).cr1, new_cr1);
            // set SSI bit based on master/slave mode
            let cr1 = core::ptr::read_volatile(&(*spi).cr1);
            let new_cr1 = if mastermode == SPIModeType::SPI_MODE_MASTER {
                cr1 | (1 << 8) // Set SSI bit for master mode
            } else {
                cr1 & !(1 << 8) // Clear SSI bit for slave mode
            };
            core::ptr::write_volatile(&mut (*spi).cr1, new_cr1);
        }
    }
}
pub fn r_spi_set_nss_output(spi_number: SPINumberType, nss_output: bool) {
    if let Some(spi) = get_spi_register(spi_number) {
        unsafe {
            let cr2 = core::ptr::read_volatile(&(*spi).cr2);
            let new_cr2 = if nss_output {
                cr2 | (1 << 2) // Set SSOE bit to enable NSS output
            } else {
                cr2 & !(1 << 2) // Clear SSOE bit to disable NSS output
            };
            core::ptr::write_volatile(&mut (*spi).cr2, new_cr2);
        }
    }
}
pub fn r_spi_set_master_or_slave(spi_number: SPINumberType, master: SPIModeType) {
    if let Some(spi) = get_spi_register(spi_number) {
        unsafe {
            let cr1 = core::ptr::read_volatile(&(*spi).cr1);
            let new_cr1 = if master == SPIModeType::SPI_MODE_MASTER {
                cr1 | (1 << 2) // Set MSTR bit for master mode
            } else {
                cr1 & !(1 << 2) // Clear MSTR bit for slave mode
            };
            core::ptr::write_volatile(&mut (*spi).cr1, new_cr1);
        }
    }
}
pub fn r_spi_set_frame_format(spi_number: SPINumberType, lsb_first: SPIFrameFormatType) {
    if let Some(spi) = get_spi_register(spi_number) {
        unsafe {
            let cr1 = core::ptr::read_volatile(&(*spi).cr1);
            let new_cr1 = if lsb_first == SPIFrameFormatType::SPI_LSB_FIRST {
                cr1 | (1 << 7) // Set LSBFIRST bit for LSB first
            } else {
                cr1 & !(1 << 7) // Clear LSBFIRST bit for MSB first
            };
            core::ptr::write_volatile(&mut (*spi).cr1, new_cr1);
        }
    }
}

pub fn r_spi_set_data_frame_format(spi_number: SPINumberType, data_frame_16bit: SPIDataFrameFormatType) {
    if let Some(spi) = get_spi_register(spi_number) {
        unsafe {
            let cr1 = core::ptr::read_volatile(&(*spi).cr1);
            let new_cr1 = if data_frame_16bit == SPIDataFrameFormatType::SPI_16BIT {
                cr1 | (1 << 11) // Set DFF bit for 16-bit data frame
            } else {
                cr1 & !(1 << 11) // Clear DFF bit for 8-bit data frame
            };
            core::ptr::write_volatile(&mut (*spi).cr1, new_cr1);
        }
    }
}
pub fn r_spi_is_busy(spi_number: SPINumberType) -> bool {
    if let Some(spi) = get_spi_register(spi_number) {
        unsafe {
            return core::ptr::read_volatile(&(*spi).sr) & (1 << 7) != 0; // Check BSY bit
        }
    }
    false
}

pub fn r_spi_clear_overrun_flag(spi_number: SPINumberType) {
    if let Some(spi) = get_spi_register(spi_number) {
        unsafe {
            // Read DR and SR to clear OVR flag
            let _ = core::ptr::read_volatile(&(*spi).dr);
            let _ = core::ptr::read_volatile(&(*spi).sr);
        }
    }
}

pub fn r_spi_enable_interrupt(spi_number: SPINumberType) {
    if let Some(spi) = get_spi_register(spi_number) {
        unsafe {
            let cr2 = core::ptr::read_volatile(&(*spi).cr2);
            core::ptr::write_volatile(&mut (*spi).cr2, cr2 | (1 << 7)); // Set TXEIE bit
            core::ptr::write_volatile(&mut (*spi).cr2, cr2 | (1 << 6)); // Set RXNEIE bit
        }
    }
}

pub fn r_spi_disable_interrupt(spi_number: SPINumberType) {
    if let Some(spi) = get_spi_register(spi_number) {
        unsafe {
            let cr2 = core::ptr::read_volatile(&(*spi).cr2);
            core::ptr::write_volatile(&mut (*spi).cr2, cr2 & !(1 << 7)); // Clear TXEIE bit
            core::ptr::write_volatile(&mut (*spi).cr2, cr2 & !(1 << 6)); // Clear RXNEIE bit
        }
    }
}

pub fn r_spi_is_transmit_complete(spi_number: SPINumberType) -> bool {
    if let Some(spi) = get_spi_register(spi_number) {
        unsafe {
            return core::ptr::read_volatile(&(*spi).sr) & (1 << 1) != 0; // Check TXE bit
        }
    }
    false
}

pub fn r_spi_is_receive_complete(spi_number: SPINumberType) -> bool {
    if let Some(spi) = get_spi_register(spi_number) {
        unsafe {
            return core::ptr::read_volatile(&(*spi).sr) & (1 << 0) != 0; // Check RXNE bit
        }
    }
    false
}
pub fn r_spi_is_overrun(spi_number: SPINumberType) -> bool {
    if let Some(spi) = get_spi_register(spi_number) {
        unsafe {
            return core::ptr::read_volatile(&(*spi).sr) & (1 << 6) != 0; // Check OVR bit
        }
    }
    false
}
// Function to set the SPI protocol mode (Motorola or TI)
pub fn r_spi_set_frame_mode(spi_number: SPINumberType, mode: u8) {
    if let Some(spi) = get_spi_register(spi_number) {
        unsafe {
            let cr2 = core::ptr::read_volatile(&(*spi).cr2);
            let new_cr2 = (cr2 & !(1 << 4)) | ((mode as u32 & 0b1) << 4); // Set FRXTH bit for TI mode
            core::ptr::write_volatile(&mut (*spi).cr2, new_cr2);
        }
    }
}

pub fn r_spi_write_data_8bit(spi_number: SPINumberType, data: u8) {
    if let Some(spi) = get_spi_register(spi_number) {
        unsafe {
            while core::ptr::read_volatile(&(*spi).sr) & (1 << 1) == 0 {}

            let dr = &mut (*spi).dr as *mut u32 as *mut u8;
            core::ptr::write_volatile(dr, data);
        }
    }
}

pub fn r_spi_read_data_8bit(spi_number: SPINumberType) -> u8 {
    if let Some(spi) = get_spi_register(spi_number) {
        unsafe {
            while core::ptr::read_volatile(&(*spi).sr) & (1 << 0) == 0 {}

            let dr = &(*spi).dr as *const u32 as *const u8;
            return core::ptr::read_volatile(dr);
        }
    }
    0
}

pub fn r_spi_write_data_16bit(spi_number: SPINumberType, data: u16) {
    if let Some(spi) = get_spi_register(spi_number) {
        unsafe {
            while core::ptr::read_volatile(&(*spi).sr) & (1 << 1) == 0 {}

            let dr = &mut (*spi).dr as *mut u32 as *mut u16;
            core::ptr::write_volatile(dr, data);
        }
    }
}

pub fn r_spi_read_data_16bit(spi_number: SPINumberType) -> u16 {
    if let Some(spi) = get_spi_register(spi_number) {
        unsafe {
            while core::ptr::read_volatile(&(*spi).sr) & (1 << 0) == 0 {}

            let dr = &(*spi).dr as *const u32 as *const u16;
            return core::ptr::read_volatile(dr);
        }
    }
    0
}

pub fn r_spi_read_data_8bit_non_blocking(spi_number: SPINumberType) -> Option<u8> {
    if let Some(spi) = get_spi_register(spi_number) {
        unsafe {
            if core::ptr::read_volatile(&(*spi).sr) & (1 << 0) != 0 {
                let dr = &(*spi).dr as *const u32 as *const u8;
                return Some(core::ptr::read_volatile(dr));
            }
        }
    }
    None
}

pub fn r_spi_write_data_8bit_non_blocking(spi_number: SPINumberType, data: u8) -> bool {
    if let Some(spi) = get_spi_register(spi_number) {
        unsafe {
            if core::ptr::read_volatile(&(*spi).sr) & (1 << 1) != 0 {
                let dr = &mut (*spi).dr as *mut u32 as *mut u8;
                core::ptr::write_volatile(dr, data);
                return true;
            }
        }
    }
    false
}
