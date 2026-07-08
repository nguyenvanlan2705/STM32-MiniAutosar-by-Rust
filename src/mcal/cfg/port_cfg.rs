use crate::register::gpio_type::{
    MODE, OUTPUTSPEED, OUTPUTTYPE, PIN, PORT, PULL, PortConfig, PortPinConfig, Dio_AlternateFunctionType
};

pub const PORT_CONFIG: PortConfig = PortConfig {
    ports: &[
        PortPinConfig {
            port: PORT::D,
            pin: PIN::P12,
            mode: MODE::OUTPUT,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::HIGH,
            pull: PULL::NONE,
            alternate_function: Dio_AlternateFunctionType::NONE,
        },
        PortPinConfig {
            port: PORT::D,
            pin: PIN::P13,
            mode: MODE::OUTPUT,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::HIGH,
            pull: PULL::NONE,
            alternate_function: Dio_AlternateFunctionType::NONE,
        },
        PortPinConfig {
            port: PORT::D,
            pin: PIN::P14,
            mode: MODE::OUTPUT,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::HIGH,
            pull: PULL::NONE,
            alternate_function: Dio_AlternateFunctionType::NONE,
        },
        PortPinConfig {
            port: PORT::D,
            pin: PIN::P15,
            mode: MODE::OUTPUT,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::HIGH,
            pull: PULL::NONE,
            alternate_function: Dio_AlternateFunctionType::NONE,
        },
        PortPinConfig {
            port: PORT::A,
            pin: PIN::P0,
            mode: MODE::INPUT,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::LOW,
            pull: PULL::PULLDOWN,
            alternate_function: Dio_AlternateFunctionType::NONE,
        },
        PortPinConfig {
            port: PORT::A,
            pin: PIN::P2,
            mode: MODE::ALTERNATE,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::VERYHIGH,
            pull: PULL::NONE,
            alternate_function: Dio_AlternateFunctionType::AF7,
        },
        PortPinConfig {
            port: PORT::A,
            pin: PIN::P3,
            mode: MODE::ALTERNATE,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::VERYHIGH,
            pull: PULL::NONE,
            alternate_function: Dio_AlternateFunctionType::AF7,
        },
    ],
};
