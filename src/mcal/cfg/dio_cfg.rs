use crate::mcal::dio_type::{
    Dio_ChannelConfig, Dio_ChannelGroupType, Dio_ChannelType, Dio_ConfigType, Dio_GroupConfigType,
};
use crate::register::gpio_type::{PIN, PORT};

pub const DIO_CHANNEL_CONFIG: Dio_ConfigType = Dio_ConfigType {
    channels: &[
        Dio_ChannelConfig {
            channel: Dio_ChannelType::LedYellow,
            port: PORT::D,
            pin: PIN::P12,
        },
        Dio_ChannelConfig {
            channel: Dio_ChannelType::LedOrange,
            port: PORT::D,
            pin: PIN::P13,
        },
        Dio_ChannelConfig {
            channel: Dio_ChannelType::LedRed,
            port: PORT::D,
            pin: PIN::P14,
        },
        Dio_ChannelConfig {
            channel: Dio_ChannelType::LedBlue,
            port: PORT::D,
            pin: PIN::P15,
        },
        Dio_ChannelConfig {
            channel: Dio_ChannelType::UserButton,
            port: PORT::A,
            pin: PIN::P0,
        },
    ],
};

pub const DIO_CHANNELGROUP_CFG: Dio_GroupConfigType = Dio_GroupConfigType {
    groups: &[
        Dio_ChannelGroupType {
            port: PORT::D,
            mask: 0b1100_0000_0000_0000,
            offset: 12,
        },
        Dio_ChannelGroupType {
            port: PORT::D,
            mask: 0b0011_0000_0000_0000,
            offset: 12,
        },
    ],
};
