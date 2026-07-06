// GPIO/src/mcal/port.rs
// This file is part of the GPIO project.
use crate::mcal::cfg::port_cfg::PORT_CONFIG;
use crate::register::gpio::{port_write_mode, port_write_outputtype, port_write_outputspeed, 
                            port_write_pull,enable_portx_clock, get_port_register};

pub fn port_init() {
    for port_config in PORT_CONFIG.ports {
        // Enable the clock for the specified port
        enable_portx_clock(port_config.port);
        // Get the base address of the port register
        let port_register = get_port_register(port_config.port);
        // Configure the pin mode
        port_write_mode(port_register, port_config.pin, port_config.mode);
        // Configure the output type
        port_write_outputtype(port_register, port_config.pin, port_config.output_type);
        // Configure the output speed
        port_write_outputspeed(port_register, port_config.pin, port_config.output_speed);
        // Configure the pull-up/pull-down resistors
        port_write_pull(port_register, port_config.pin, port_config.pull);
    }
}
