#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPINumberType {
    SPI1,
    SPI2,
    SPI3,
    SPI4,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SPI_Register {
    pub cr1: u32,
    pub cr2: u32,
    pub sr: u32,
    pub dr: u32,
    pub crcpr: u32,
    pub rxcrcr: u32,
    pub txcrcr: u32,
    pub i2scfgr: u32,
    pub i2spr: u32,
}

const SPI1: *mut SPI_Register = 0x4001_3000 as *mut SPI_Register;
const SPI2: *mut SPI_Register = 0x4000_3800 as *mut SPI_Register;
const SPI3: *mut SPI_Register = 0x4000_3C00 as *mut SPI_Register;
const SPI4: *mut SPI_Register = 0x4001_3400 as *mut SPI_Register;

pub fn get_spi_register(spi_number: SPINumberType) -> Option<*mut SPI_Register> {
    match spi_number {
        SPINumberType::SPI1 => Some(SPI1),
        SPINumberType::SPI2 => Some(SPI2),
        SPINumberType::SPI3 => Some(SPI3),
        SPINumberType::SPI4 => Some(SPI4),
    }
}

