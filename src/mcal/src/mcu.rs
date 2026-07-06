#![allow(dead_code)]

use crate::register::clock_type::ClockResourceType;
use crate::register::clock::{get_clock_resource, enable_hsi};

pub fn mcu_get_clock_resource() -> ClockResourceType {
    get_clock_resource()
}

pub fn mcu_get_system_clock_hz() -> u32 {
    match get_clock_resource() {
        ClockResourceType::HSI => 16_000_000,
        ClockResourceType::HSE => 0, // Placeholder value, replace with actual HSE frequency if known
        ClockResourceType::PLL => 0, // Placeholder value, replace with actual PLL frequency if known
    }
}

pub fn mcu_init(){
    let clock_resource = mcu_get_clock_resource();
    match clock_resource {
        ClockResourceType::HSI => {
            // HSI is already enabled by default, no action needed
            enable_hsi();
        }
        ClockResourceType::HSE => {
            // Enable HSE if needed (not implemented in this example)
        }
        ClockResourceType::PLL => {
            // Enable PLL if needed (not implemented in this example)
        }
    }
}
