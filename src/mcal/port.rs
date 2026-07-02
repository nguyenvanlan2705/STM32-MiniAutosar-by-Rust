// GPIO/src/mcal/port.rs
// This file is part of the GPIO project.
use crate::register::gpio_type::{PORT, PIN, MODE, OUTPUTTYPE, OUTPUTSPEED, PULL, PortPinConfig, PortConfig};
use crate::register::gpio::{port_write_mode, port_write_outputtype, port_write_outputspeed, 
                            port_write_pull,enable_portx_clock, get_port_register};

// The port_init function initializes the GPIO ports based on the configuration defined in PORT_CONFIG.
const PORT_CONFIG: PortConfig = PortConfig {
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

pub fn port_init() {
    for pin_config in PORT_CONFIG.pins {
        // Enable the clock for the specified port
        enable_portx_clock(pin_config.port);
        // Get the base address of the port register
        let port_register = get_port_register(pin_config.port);
        // Configure the pin mode
        port_write_mode(port_register, pin_config.pin, pin_config.mode);
        // Configure the output type
        port_write_outputtype(port_register, pin_config.pin, pin_config.output_type);
        // Configure the output speed
        port_write_outputspeed(port_register, pin_config.pin, pin_config.output_speed);
        // Configure the pull-up/pull-down resistors
        port_write_pull(port_register, pin_config.pin, pin_config.pull);
    }
}