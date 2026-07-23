#![allow(dead_code)]
use crate::mcal::dio::dio_writechannel;
use crate::mcal::external::mcp2515_type::{MCP2515ConfigType,Mcp2515DeviceId, MCP2515Mode, 
    MCP2515BaudRate, MCP2515Instruction, MCP2515Register, MCP2515ReturnType, CTRL_MODE_MASK };
use crate::register::gpio_type::Dio_LevelType;
use crate::mcal::cfg::mcp2515_cfg::MCP2515_CONFIG_SET;
use crate::mcal::spi::spi_master_transfer_byte;

pub fn mcp2515_get_config(device_id: Mcp2515DeviceId) -> Option<&'static MCP2515ConfigType> {
    for config in MCP2515_CONFIG_SET.configs.iter() {
        if config.device_id == device_id {
            return Some(config);
        }
    }
    None
}
pub fn mcp2515_reset(device_id: Mcp2515DeviceId) -> MCP2515ReturnType{
    let config = mcp2515_get_config(device_id);
    
    if let Some(config) = config {
        let mut dummy = 0xff;
        dio_writechannel(config.cs_channel, Dio_LevelType::LOW); // Set CS low
        spi_master_transfer_byte(config.spi_channel, MCP2515Instruction::RESET as u8, &mut dummy); // Send RESET instruction (0xC0) to MCP2515
        dio_writechannel(config.cs_channel, Dio_LevelType::HIGH); // Set CS high
        MCP2515ReturnType::MCP2515_E_OK
    } else {
        MCP2515ReturnType::MCP2515_E_NOT_OK
    }
}
pub fn mcp2515_set_mode(device_id: Mcp2515DeviceId, mode: MCP2515Mode) ->MCP2515ReturnType{
    let config = mcp2515_get_config(device_id);
    
    if let Some(config) = config {
        let mut dummy = 0xff;
        dio_writechannel(config.cs_channel, Dio_LevelType::LOW); // Set CS low
        //BIT_MODIFY instruction (0x05) to MCP2515
        spi_master_transfer_byte(config.spi_channel, MCP2515Instruction::BIT_MODIFY as u8, &mut dummy); 
        spi_master_transfer_byte(config.spi_channel, MCP2515Register::CANCTRL as u8, &mut dummy); // Send address of CANCTRL register (0x0F)
        spi_master_transfer_byte(config.spi_channel, CTRL_MODE_MASK, &mut dummy); // Send mask (0xE0)
        spi_master_transfer_byte(config.spi_channel, mode as u8, &mut dummy); // Send desired mode value
        dio_writechannel(config.cs_channel, Dio_LevelType::HIGH); // Set CS high
        MCP2515ReturnType::MCP2515_E_OK
    } else {
        MCP2515ReturnType::MCP2515_E_NOT_OK
    }
}
// not verified
fn mcp2515_set_baudrate_1mbps(device_id: Mcp2515DeviceId) {
    let config = mcp2515_get_config(device_id);
    
    if let Some(config) = config {
        let mut dummy = 0xff;
        dio_writechannel(config.cs_channel, Dio_LevelType::LOW); // Set CS low
        spi_master_transfer_byte(config.spi_channel, MCP2515Instruction::WRITE as u8, &mut dummy); // Send WRITE instruction (0x02) to MCP2515
        spi_master_transfer_byte(config.spi_channel, 0x2A, &mut dummy); // Send address of CNF1 register (0x2A)
        spi_master_transfer_byte(config.spi_channel, 0x00, &mut dummy); // Set CNF1 for 1Mbps
        spi_master_transfer_byte(config.spi_channel, 0x80, &mut dummy); // Set CNF2 for 1Mbps
        spi_master_transfer_byte(config.spi_channel, 0x00, &mut dummy); // Set CNF3 for 1Mbps
        dio_writechannel(config.cs_channel, Dio_LevelType::HIGH); // Set CS high
    }
}
fn mcp2515_set_baudrate_500kbps(device_id: Mcp2515DeviceId) {
    let config = mcp2515_get_config(device_id);
    
    if let Some(config) = config {
        let mut dummy = 0xff;
        dio_writechannel(config.cs_channel, Dio_LevelType::LOW); // Set CS low
        spi_master_transfer_byte(config.spi_channel, MCP2515Instruction::WRITE as u8, &mut dummy); // Send WRITE instruction (0x02) to MCP2515
        spi_master_transfer_byte(config.spi_channel, MCP2515Register::CNF1 as u8, &mut dummy); // Send address of CNF1 register (0x2A)
        spi_master_transfer_byte(config.spi_channel, 0x00, &mut dummy); // Set CNF1 for 500kbps
        spi_master_transfer_byte(config.spi_channel, 0x90, &mut dummy); // Set CNF2 for 500kbps
        spi_master_transfer_byte(config.spi_channel, 0x02, &mut dummy); // Set CNF3 for 500kbps
        dio_writechannel(config.cs_channel, Dio_LevelType::HIGH); // Set CS high
    }
}
// not verified
fn mcp2515_set_baudrate_250kbps(device_id: Mcp2515DeviceId) {
    let config = mcp2515_get_config(device_id);
    
    if let Some(config) = config {
        let mut dummy = 0xff;
        dio_writechannel(config.cs_channel, Dio_LevelType::LOW); // Set CS low
        spi_master_transfer_byte(config.spi_channel, MCP2515Instruction::WRITE as u8, &mut dummy); // Send WRITE instruction (0x02) to MCP2515
        spi_master_transfer_byte(config.spi_channel, MCP2515Register::CNF1 as u8, &mut dummy); // Send address of CNF1 register (0x2A)
        spi_master_transfer_byte(config.spi_channel, 0x01, &mut dummy); // Set CNF1 for 250kbps
        spi_master_transfer_byte(config.spi_channel, 0x90, &mut dummy); // Set CNF2 for 250kbps
        spi_master_transfer_byte(config.spi_channel, 0x02, &mut dummy); // Set CNF3 for 250kbps
        dio_writechannel(config.cs_channel, Dio_LevelType::HIGH); // Set CS high
    }
}
// not verified
fn mcp2515_set_baudrate_125kbps(device_id: Mcp2515DeviceId) {
    let config = mcp2515_get_config(device_id);
    
    if let Some(config) = config {
        let mut dummy = 0xff;
        dio_writechannel(config.cs_channel, Dio_LevelType::LOW); // Set CS low
        spi_master_transfer_byte(config.spi_channel, MCP2515Instruction::WRITE as u8, &mut dummy); // Send WRITE instruction (0x02) to MCP2515
        spi_master_transfer_byte(config.spi_channel, MCP2515Register::CNF1 as u8, &mut dummy); // Send address of CNF1 register (0x2A)
        spi_master_transfer_byte(config.spi_channel, 0x03, &mut dummy); // Set CNF1 for 125kbps
        spi_master_transfer_byte(config.spi_channel, 0x90, &mut dummy); // Set CNF2 for 125kbps
        spi_master_transfer_byte(config.spi_channel, 0x02, &mut dummy); // Set CNF3 for 125kbps
        dio_writechannel(config.cs_channel, Dio_LevelType::HIGH); // Set CS high
    }
}
pub fn mcp2515_set_baudrate(device_id: Mcp2515DeviceId, baudrate: MCP2515BaudRate) {
    match baudrate {
        MCP2515BaudRate::BAUD_500KBPS => mcp2515_set_baudrate_500kbps(device_id),
        MCP2515BaudRate::BAUD_250KBPS => mcp2515_set_baudrate_250kbps(device_id),
        MCP2515BaudRate::BAUD_125KBPS => mcp2515_set_baudrate_125kbps(device_id),
        MCP2515BaudRate::BAUD_1MBPS => mcp2515_set_baudrate_1mbps(device_id),
    }
}

pub fn mcp2515_write(device_id: Mcp2515DeviceId, address: u8, data: &[u8]) -> MCP2515ReturnType {
    let config = mcp2515_get_config(device_id);
    
    if let Some(config) = config {
        let mut dummy = 0xff;
        dio_writechannel(config.cs_channel, Dio_LevelType::LOW); // Set CS low
        spi_master_transfer_byte(config.spi_channel, MCP2515Instruction::WRITE as u8, &mut dummy); // Send WRITE instruction (0x02) to MCP2515
        spi_master_transfer_byte(config.spi_channel, address, &mut dummy); // Send address to write to MCP2515
        for &byte in data {
            spi_master_transfer_byte(config.spi_channel, byte, &mut dummy); // Send data bytes
        }
        dio_writechannel(config.cs_channel, Dio_LevelType::HIGH); // Set CS high
        MCP2515ReturnType::MCP2515_E_OK
    } else {
        MCP2515ReturnType::MCP2515_E_NOT_OK
    }
}

pub fn mcp2515_read(device_id: Mcp2515DeviceId, address: u8, buffer: &mut [u8]) -> MCP2515ReturnType {
    let config = mcp2515_get_config(device_id);
    
    if let Some(config) = config {
        let mut dummy = 0xff;
        dio_writechannel(config.cs_channel, Dio_LevelType::LOW); // Set CS low
        spi_master_transfer_byte(config.spi_channel, MCP2515Instruction::READ as u8, &mut dummy); // Send READ instruction (0x03) to MCP2515
        spi_master_transfer_byte(config.spi_channel, address, &mut dummy); // Send address to read from MCP2515
        for byte in buffer.iter_mut() {
            spi_master_transfer_byte(config.spi_channel, 0x00, byte); // Read data bytes into buffer
        }
        dio_writechannel(config.cs_channel, Dio_LevelType::HIGH); // Set CS high
        MCP2515ReturnType::MCP2515_E_OK
    } else {
        MCP2515ReturnType::MCP2515_E_NOT_OK
    }
}
pub fn mcp2515_enable_interrupt_transmit(device_id: Mcp2515DeviceId) {
    let config = mcp2515_get_config(device_id);
    
    if let Some(config) = config {
        let mut dummy = 0xff;
        dio_writechannel(config.cs_channel, Dio_LevelType::LOW); // Set CS low
        spi_master_transfer_byte(config.spi_channel, MCP2515Instruction::WRITE as u8, &mut dummy); // Send WRITE instruction (0x02) to MCP2515
        spi_master_transfer_byte(config.spi_channel, MCP2515Register::CANINTE as u8, &mut dummy); // Send address of CANINTE register (0x2B)
        spi_master_transfer_byte(config.spi_channel, 0x01, &mut dummy); // Enable transmit interrupt (TX0IE)
        dio_writechannel(config.cs_channel, Dio_LevelType::HIGH); // Set CS high
    }
}
pub fn mcp2515_disable_interrupt_transmit(device_id: Mcp2515DeviceId) {
    let config = mcp2515_get_config(device_id);
    
    if let Some(config) = config {
        let mut dummy = 0xff;
        dio_writechannel(config.cs_channel, Dio_LevelType::LOW); // Set CS low
        spi_master_transfer_byte(config.spi_channel, MCP2515Instruction::WRITE as u8, &mut dummy); // Send WRITE instruction (0x02) to MCP2515
        spi_master_transfer_byte(config.spi_channel, MCP2515Register::CANINTE as u8, &mut dummy); // Send address of CANINTE register (0x2B)
        spi_master_transfer_byte(config.spi_channel, 0x00, &mut dummy); // Disable transmit interrupt (TX0IE)
        dio_writechannel(config.cs_channel, Dio_LevelType::HIGH); // Set CS high
    }
}

pub fn mcp2515_enable_interrupt_receive(device_id: Mcp2515DeviceId) {
    let config = mcp2515_get_config(device_id);
    
    if let Some(config) = config {
        let mut dummy = 0xff;
        dio_writechannel(config.cs_channel, Dio_LevelType::LOW); // Set CS low
        spi_master_transfer_byte(config.spi_channel, MCP2515Instruction::WRITE as u8, &mut dummy); // Send WRITE instruction (0x02) to MCP2515
        spi_master_transfer_byte(config.spi_channel, MCP2515Register::CANINTE as u8, &mut dummy); // Send address of CANINTE register (0x2B)
        spi_master_transfer_byte(config.spi_channel, 0x03, &mut dummy); // Enable receive interrupt (RX0IE and RX1IE)
        dio_writechannel(config.cs_channel, Dio_LevelType::HIGH); // Set CS high
    }
}
pub fn mcp2515_disable_interrupt_receive(device_id: Mcp2515DeviceId) {
    let config = mcp2515_get_config(device_id);
    
    if let Some(config) = config {
        let mut dummy = 0xff;
        dio_writechannel(config.cs_channel, Dio_LevelType::LOW); // Set CS low
        spi_master_transfer_byte(config.spi_channel, MCP2515Instruction::WRITE as u8, &mut dummy); // Send WRITE instruction (0x02) to MCP2515
        spi_master_transfer_byte(config.spi_channel, MCP2515Register::CANINTE as u8, &mut dummy); // Send address of CANINTE register (0x2B)
        spi_master_transfer_byte(config.spi_channel, 0x00, &mut dummy); // Disable receive interrupt (RX0IE and RX1IE)
        dio_writechannel(config.cs_channel, Dio_LevelType::HIGH); // Set CS high
    }
}

pub fn mcp2515_read_status(device_id: Mcp2515DeviceId) -> u8 {
    let config = mcp2515_get_config(device_id);
    
    if let Some(config) = config {
        let mut dummy = 0xff;
        let mut status = 0xff;
        dio_writechannel(config.cs_channel, Dio_LevelType::LOW); // Set CS low
        spi_master_transfer_byte(config.spi_channel, MCP2515Instruction::READ_STATUS as u8, &mut dummy); // Send READ_STATUS instruction (0xA0) to MCP2515
        spi_master_transfer_byte(config.spi_channel, 0x00, &mut status); // Read status byte
        dio_writechannel(config.cs_channel, Dio_LevelType::HIGH); // Set CS high
        return status; // Return the status byte
    }
    0 // Return 0 if config is not found
}

pub fn mcp2515_read_register_status(device_id: Mcp2515DeviceId) -> u8 {
    let config = mcp2515_get_config(device_id);
    
    if let Some(config) = config {
        let mut dummy = 0xff;
        let mut status = 0xff;
        dio_writechannel(config.cs_channel, Dio_LevelType::LOW); // Set CS low
        spi_master_transfer_byte(config.spi_channel, MCP2515Instruction::READ as u8, &mut dummy); // Send READ instruction (0x03) to MCP2515
        spi_master_transfer_byte(config.spi_channel, MCP2515Register::CANSTAT as u8, &mut dummy); // Send address of CANSTAT register (0x0E)
        spi_master_transfer_byte(config.spi_channel, 0x00, &mut status); // Read status byte
        dio_writechannel(config.cs_channel, Dio_LevelType::HIGH); // Set CS high
        return status; // Return the status byte
    }
    0 // Return 0 if config is not found
}
pub fn mcp2515_read_register_ctrl(device_id: Mcp2515DeviceId) -> u8 {
    let config = mcp2515_get_config(device_id);
    
    if let Some(config) = config {
        let mut dummy = 0xff;
        let mut ctrl = 0xff;
        dio_writechannel(config.cs_channel, Dio_LevelType::LOW); // Set CS low
        spi_master_transfer_byte(config.spi_channel, MCP2515Instruction::READ as u8, &mut dummy); // Send READ instruction (0x03) to MCP2515
        spi_master_transfer_byte(config.spi_channel, MCP2515Register::CANCTRL as u8, &mut dummy); // Send address of CANCTRL register (0x0F)
        spi_master_transfer_byte(config.spi_channel, 0x00, &mut ctrl); // Read control byte
        dio_writechannel(config.cs_channel, Dio_LevelType::HIGH); // Set CS high
        return ctrl; // Return the control byte
    }
    0 // Return 0 if config is not found
}

pub fn mcp2515_init(){
    for config in MCP2515_CONFIG_SET.configs.iter() {
        dio_writechannel(config.cs_channel, Dio_LevelType::HIGH);
        mcp2515_reset(config.device_id);
        mcp2515_set_baudrate(config.device_id, config.baudrate);
        mcp2515_set_mode(config.device_id, MCP2515Mode::NORMAL);
    }
}