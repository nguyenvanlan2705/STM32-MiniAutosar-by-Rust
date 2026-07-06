#![allow(dead_code)]

use crate::register::systick_type::get_systick_register;

pub fn systick_init(core_clock_hz: u32, tick_hz: u32) {
    // Get a mutable reference to the SysTick register
    let systick = get_systick_register();
    // Set the reload value for the specified tick rate
    unsafe {
        if tick_hz != 0 && tick_hz <= core_clock_hz {
            core::ptr::write_volatile(&mut systick.SYST_RVR, (core_clock_hz / tick_hz) - 1);
        }
        // Clear the current value
        core::ptr::write_volatile(&mut systick.SYST_CVR, 0);
        // Enable SysTick, use processor clock, enable interrupt
        core::ptr::write_volatile(&mut systick.SYST_CSR, 0x07);
    }
    
}
