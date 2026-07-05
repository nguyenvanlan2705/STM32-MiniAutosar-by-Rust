#![allow(non_snake_case)]
#![allow(dead_code)]

use crate::mcal::dio::{dio_flipchannel, dio_readchannel_output, dio_writechannel, dio_writechannelgroup, get_channelgroup_cfg};
use crate::register::gpio_type::{Dio_LevelType};
use crate::bsw::iohwab::iohwab_type::{LedColor, LedState, LedGroup};
use crate::mcal::dio_type::{Dio_ChannelType, Dio_ChannelGroupType};


fn led_color_to_channel(color: LedColor) -> Dio_ChannelType {
    match color {
        LedColor::Yellow => Dio_ChannelType::LedYellow,
        LedColor::Orange => Dio_ChannelType::LedOrange,
        LedColor::Red => Dio_ChannelType::LedRed,
        LedColor::Blue => Dio_ChannelType::LedBlue,
    }
}
fn led_group_to_channel_group(group: LedGroup) -> Dio_ChannelGroupType {
    let channel_groups = get_channelgroup_cfg().groups;
    match group {
        LedGroup::RedBlue => channel_groups[0],
        LedGroup::OrangeYellow => channel_groups[1],
    }
}
pub fn led_state_to_level(state: LedState) -> Dio_LevelType {
    match state {
        LedState::On => Dio_LevelType::HIGH,
        LedState::Off => Dio_LevelType::LOW,
        LedState::Toggle => Dio_LevelType::LOW, // Mặc định là LOW nếu không xác định
    }
}
pub fn led_toggle(color: LedColor){
    let channel = led_color_to_channel(color);
    let _ = dio_flipchannel(channel);
}
pub fn set_led_state(color: LedColor, state: LedState) {
    let channel = led_color_to_channel(color);
    let level = led_state_to_level(state);
    dio_writechannel(channel, level);
}
pub fn get_led_state(color: LedColor) -> Dio_LevelType {
    let channel = led_color_to_channel(color);
    let level = dio_readchannel_output(channel);
    level
}
pub fn led_set_state_group(group: LedGroup, value: u32) {
    let channel_group = led_group_to_channel_group(group);
    dio_writechannelgroup(channel_group, value);
}