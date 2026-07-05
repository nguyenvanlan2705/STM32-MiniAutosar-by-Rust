use crate::register::gpio_type::{
    MODE, OUTPUTSPEED, OUTPUTTYPE, PIN, PORT, PULL, PortConfig, PortPinConfig,
};

pub const PORT_CONFIG: PortConfig = PortConfig {
    pins: &[
        PortPinConfig {
            port: PORT::D,
            pin: PIN::P12,
            mode: MODE::OUTPUT,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::HIGH,
            pull: PULL::NONE,
        },
        PortPinConfig {
            port: PORT::D,
            pin: PIN::P13,
            mode: MODE::OUTPUT,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::HIGH,
            pull: PULL::NONE,
        },
        PortPinConfig {
            port: PORT::D,
            pin: PIN::P14,
            mode: MODE::OUTPUT,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::HIGH,
            pull: PULL::NONE,
        },
        PortPinConfig {
            port: PORT::D,
            pin: PIN::P15,
            mode: MODE::OUTPUT,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::HIGH,
            pull: PULL::NONE,
        },
        PortPinConfig {
            port: PORT::A,
            pin: PIN::P0,
            mode: MODE::INPUT,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::LOW,
            pull: PULL::PULLDOWN,
        },
    ],
};
