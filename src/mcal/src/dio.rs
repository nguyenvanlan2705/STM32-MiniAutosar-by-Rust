#![allow(dead_code)]

use crate::mcal::cfg::dio_cfg::{DIO_CHANNEL_CONFIG, DIO_CHANNELGROUP_CFG};
use crate::mcal::dio_type::{Dio_ChannelGroupType, Dio_ChannelType, Dio_GroupConfigType};
use crate::register::gpio_type::Dio_LevelType;
use crate::register::dio::{dio_read, dio_toggle, dio_write, dio_write_port, dio_read_output, dio_read_output_port};

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
