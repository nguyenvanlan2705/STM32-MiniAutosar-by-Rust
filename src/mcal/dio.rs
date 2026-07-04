#![allow(dead_code)]

use crate::mcal::dio_type::{Dio_ChannelType, Dio_ConfigType, Dio_ChannelConfig, Dio_GroupConfigType, Dio_ChannelGroupType};
use crate::register::gpio_type::{PORT, PIN, Dio_LevelType};
use crate::register::dio::{dio_read, dio_toggle, dio_write, dio_write_port, dio_read_output, dio_read_output_port};
const DIO_CHANNEL_CONFIG : Dio_ConfigType = Dio_ConfigType{
    channels: &[
        Dio_ChannelConfig{
            channel : Dio_ChannelType::LedYellow,
            port : PORT::D,
            pin : PIN::P12,
        },
        Dio_ChannelConfig{
            channel : Dio_ChannelType::LedOrange,
            port : PORT::D,
            pin : PIN::P13,
        },
        Dio_ChannelConfig{
            channel : Dio_ChannelType::LedRed,
            port : PORT::D,
            pin : PIN::P14,
        },
        Dio_ChannelConfig{
            channel : Dio_ChannelType::LedBlue,
            port : PORT::D,
            pin : PIN::P15,
        },
        Dio_ChannelConfig{
            channel : Dio_ChannelType::UserButton,
            port : PORT::A,
            pin : PIN::P0,
        },
    ],
};
const DIO_CHANNELGROUP_CFG : Dio_GroupConfigType = Dio_GroupConfigType{
    groups: &[
        Dio_ChannelGroupType{
            port : PORT::D,
            mask : 0b1100_0000_0000_0000,
            offset : 12,
        },
        Dio_ChannelGroupType{
            port : PORT::D,
            mask : 0b0011_0000_0000_0000,
            offset : 12,
        },
    ],
};
pub fn get_channelgroup_cfg() -> &'static Dio_GroupConfigType {
    &DIO_CHANNELGROUP_CFG
}
fn get_channel_cfg_index(channel: Dio_ChannelType) -> usize {
    for (index, channel_cfg) in DIO_CHANNEL_CONFIG.channels.iter().enumerate() {
        if channel_cfg.channel == channel { 
            return index;
        }
    }
    0xffff // Return an invalid index if the channel is not found
}
pub fn dio_readchannel(channel: Dio_ChannelType) -> Dio_LevelType{
    let index = get_channel_cfg_index(channel);
    let channel_cfg = &DIO_CHANNEL_CONFIG.channels[index];
    let level = dio_read(channel_cfg.port, channel_cfg.pin);
    level
}

pub fn dio_writechannel(channel: Dio_ChannelType, level: Dio_LevelType){
    let index = get_channel_cfg_index(channel);
    let channel_cfg = &DIO_CHANNEL_CONFIG.channels[index];
    dio_write(channel_cfg.port, channel_cfg.pin, level);
}
pub fn dio_flipchannel(channel: Dio_ChannelType) -> Dio_LevelType{
    let index = get_channel_cfg_index(channel);
    let channel_cfg = &DIO_CHANNEL_CONFIG.channels[index];
    let level = dio_toggle(channel_cfg.port, channel_cfg.pin);
    level
}
pub fn dio_readchannel_output(channel: Dio_ChannelType) -> Dio_LevelType{
    let index = get_channel_cfg_index(channel);
    let channel_cfg = &DIO_CHANNEL_CONFIG.channels[index];
    let level = dio_read_output(channel_cfg.port, channel_cfg.pin);
    level
}
pub fn dio_writechannelgroup(group: Dio_ChannelGroupType, value: u32){
    let current_value = dio_read_output_port(group.port);
    let cleared_value = current_value & !(group.mask as u32);
    let shifted_value = ((value as u32) << group.offset) & (group.mask as u32);
    let new_value = cleared_value | shifted_value;
    dio_write_port(group.port, new_value);
}
pub fn dio_readchannelgroup(group: Dio_ChannelGroupType) -> u32{
    let port_value = dio_read_output_port(group.port);
    let masked_value = (port_value & (group.mask as u32)) >> group.offset;
    masked_value
}