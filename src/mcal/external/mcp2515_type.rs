#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::register::spi_type::{SPINumberType};
use crate::mcal::dio_type::{Dio_ChannelType};

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCP2515Register {
    BFPCTRL = 0x0C,
    TXRTSCTRL = 0x0D,
    CANSTAT = 0x0E,
    CANCTRL = 0x0F,
    CNF1 = 0x2A,
    CNF2 = 0x29,
    CNF3 = 0x28,
    /*TX BUFFER 0 */
    TXB0CTRL = 0x30,
    TXB0SIDH = 0x31,
    TXB0SIDL = 0x32,
    TXB0DLC = 0x35,
    TXB0D0 = 0x36,

    /*TX BUFFER 1 */
    TXB1CTRL = 0x40,
    TXB1SIDH = 0x41,
    TXB1SIDL = 0x42,
    TXB1DLC = 0x45,
    TXB1D0 = 0x46,
    /*TX BUFFER 2 */
    TXB2CTRL = 0x50,
    TXB2SIDH = 0x51,
    TXB2SIDL = 0x52,
    TXB2DLC = 0x55,
    TXB2D0 = 0x56,
    /*RX BUFFER 0 */
    RXB0CTRL = 0x60,
    RXB0SIDH = 0x61,
    RXB0SIDL = 0x62,
    RXB0DLC = 0x65,
    RXB0D0 = 0x66,
    /*RX BUFFER 1 */
    RXB1CTRL = 0x70,
    RXB1SIDH = 0x71,
    RXB1SIDL = 0x72,
    RXB1DLC = 0x75,
    RXB1D0 = 0x76,
    /*INTERRUPT */
    CANINTE = 0x2B,
    CANINTF = 0x2C,
    /*TEC/RCE */
    TEC = 0x1C,
    REC = 0x1D,
    /*EFLG */
    EFLG = 0x2D,
}
pub const CTRL_MODE_MASK: u8 = 0xE0; // Mask for the mode bits in the CANCTRL register

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCP2515Instruction{
    RESET = 0xC0,
    READ = 0x03,
    WRITE = 0x02,
    BIT_MODIFY = 0x05,
    READ_STATUS = 0xA0,
    RX_STATUS = 0xB0,
    RTS_TXB0 = 0x81,
    RTS_TXB1 = 0x82,
    RTS_TXB2 = 0x84,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCP2515Mode {
    NORMAL = 0x00,
    SLEEP = 0x20,
    LOOPBACK = 0x40,
    LISTEN_ONLY = 0x60,
    CONFIGURATION = 0x80,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mcp2515DeviceId {
    MCP2515_1 = 0,
    MCP2515_2 = 1,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MCP2515ConfigType {
    pub device_id: Mcp2515DeviceId, // Unique identifier for the MCP2515 device
    pub spi_channel: SPINumberType, // SPI channel used for communication with MCP2515
    pub cs_channel: Dio_ChannelType, // Chip Select channel for MCP2515
    pub int_channel: Option<Dio_ChannelType>, // Optional interrupt channel for MCP2515
    pub baudrate: MCP2515BaudRate, // Baud rate for CAN communication
    pub oscillator_hz: u32, // Oscillator frequency in Hz for MCP2515
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MCP2515ConfigSetType {
    pub configs: &'static [MCP2515ConfigType], // Array of MCP2515 configurations
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCP2515BaudRate {
    BAUD_125KBPS,
    BAUD_250KBPS,
    BAUD_500KBPS,
    BAUD_1MBPS,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCP2515ReturnType {
    MCP2515_E_OK,
    MCP2515_E_NOT_OK,
}
