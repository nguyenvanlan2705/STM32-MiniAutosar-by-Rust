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
        // Configure PB0 as analog input for ADC channel 8
        PortPinConfig {
            port: PORT::B,
            pin: PIN::P0,
            mode: MODE::ANALOG,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::VERYHIGH,
            pull: PULL::NONE,
            alternate_function: Dio_AlternateFunctionType::NONE,
        },
        // PE3 is the onboard SPI sensor CS on STM32F411 Discovery.
        // Keep it high so the onboard sensor does not drive SPI1 MISO.
        PortPinConfig {
            port: PORT::E,
            pin: PIN::P3,
            mode: MODE::OUTPUT,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::VERYHIGH,
            pull: PULL::NONE,
            alternate_function: Dio_AlternateFunctionType::NONE,
        },
        // Pin Port config for SPI1 (PA4, PA5, PA6, PA7) can be added here if needed
        PortPinConfig {
            port: PORT::A,
            pin: PIN::P4,
            mode: MODE::OUTPUT,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::VERYHIGH,
            pull: PULL::NONE,
            alternate_function: Dio_AlternateFunctionType::NONE, // SPI1_NSS Software controlled
        },

        PortPinConfig {
            port: PORT::A,
            pin: PIN::P5,
            mode: MODE::ALTERNATE,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::VERYHIGH,
            pull: PULL::NONE,
            alternate_function: Dio_AlternateFunctionType::AF5, // SPI1_SCK
        },
        PortPinConfig {
            port: PORT::A,
            pin: PIN::P6,
            mode: MODE::ALTERNATE,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::VERYHIGH,
            pull: PULL::NONE,
            alternate_function: Dio_AlternateFunctionType::AF5, // SPI1_MISO
        },
        PortPinConfig {
            port: PORT::A,
            pin: PIN::P7,
            mode: MODE::ALTERNATE,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::VERYHIGH,
            pull: PULL::NONE,
            alternate_function: Dio_AlternateFunctionType::AF5, // SPI1_MOSI
        },
        PortPinConfig {
            port: PORT::A,
            pin: PIN::P8,
            mode: MODE::INPUT,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::LOW,
            pull: PULL::PULLUP,
            alternate_function: Dio_AlternateFunctionType::NONE, // MCP2515 INT pin
        },
        // Configure PA12, PB13, PB14, PB15 for SPI2
        PortPinConfig {
            port: PORT::A,
            pin: PIN::P12,
            mode: MODE::OUTPUT,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::VERYHIGH,
            pull: PULL::NONE,
            alternate_function: Dio_AlternateFunctionType::NONE, // SPI2_NSS Software controlled
        },
        PortPinConfig {
            port: PORT::B,
            pin: PIN::P13,
            mode: MODE::ALTERNATE,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::VERYHIGH,
            pull: PULL::NONE,
            alternate_function: Dio_AlternateFunctionType::AF5, // SPI2_SCK
        },
        PortPinConfig {
            port: PORT::B,
            pin: PIN::P14,
            mode: MODE::ALTERNATE,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::VERYHIGH,
            pull: PULL::NONE,
            alternate_function: Dio_AlternateFunctionType::AF5, // SPI2_MISO
        },
        PortPinConfig {
            port: PORT::B,
            pin: PIN::P15,
            mode: MODE::ALTERNATE,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::VERYHIGH,
            pull: PULL::NONE,
            alternate_function: Dio_AlternateFunctionType::AF5, // SPI2_MOSI
        },
    ],
};
