#![allow(dead_code)]

use crate::register::{{clock_type::ClockResourceType},
                      {clock::{get_clock_resource, enable_hsi}},
                      {systick::systick_init}};
use core::sync::atomic::{AtomicU32, Ordering};

// This module provides functions to initialize and manage the MCU, including clock configuration and SysTick timer setup.
static SYSTEM_TICK_COUNT: AtomicU32 = AtomicU32::new(0);
// Initializes the SysTick timer to generate an interrupt every 1 millisecond.
pub fn systick_1ms_handler() {
    // This function will be called every 1ms by the SysTick interrupt
    // Implement your 1ms tick handling logic here
    SYSTEM_TICK_COUNT.fetch_add(1, Ordering::Relaxed);
}
// Function to get the current system tick count ms
pub fn mcu_get_system_tick_count() -> u32 {
    SYSTEM_TICK_COUNT.load(Ordering::Relaxed)
}


// Function to get the current clock resource type
pub fn mcu_get_clock_resource() -> ClockResourceType {
    get_clock_resource()
}

// Function to get the current system clock frequency in Hz
pub fn mcu_get_system_clock_hz() -> u32 {
    match get_clock_resource() {
        ClockResourceType::HSI => 16_000_000,
        ClockResourceType::HSE => 0, // Placeholder value, replace with actual HSE frequency if known
        ClockResourceType::PLL => 0, // Placeholder value, replace with actual PLL frequency if known
    }
}

// Initializes the SysTick timer to generate an interrupt every 1 millisecond.
pub fn mcu_init_systick_1ms(){
    let system_clock_hz = mcu_get_system_clock_hz();
    systick_init(system_clock_hz, 1000);
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
