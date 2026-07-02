use crate::mcal::std_type::{Dio_ChannelType, Dio_ConfigType, Dio_ChannelConfig};
use crate::register::gpio_type::{PORT, PIN, Dio_LevelType};
use crate::register::dio::{dio_read, dio_toggle, dio_write};
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