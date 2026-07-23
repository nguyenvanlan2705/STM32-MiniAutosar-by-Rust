use crate::mcal::external::mcp2515_type::{MCP2515ConfigType,MCP2515ConfigSetType,Mcp2515DeviceId, MCP2515BaudRate};
use crate::register::spi_type::{SPINumberType};
use crate::mcal::dio_type::{Dio_ChannelType};

pub const MCP2515_CONFIG_SET: MCP2515ConfigSetType = MCP2515ConfigSetType {
    configs: &[
        MCP2515ConfigType {
            device_id: Mcp2515DeviceId::MCP2515_1,
            spi_channel: SPINumberType::SPI1,
            cs_channel: Dio_ChannelType::Mcp2515Cs,
            int_channel: Some(Dio_ChannelType::mcp2515Int),
            baudrate: MCP2515BaudRate::BAUD_500KBPS,
            oscillator_hz: 8_000_000, // 8 MHz
        },
    ],
};

