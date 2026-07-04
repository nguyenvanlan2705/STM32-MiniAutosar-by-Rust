#![no_std]
#![no_main]
use stm32f4 as _;
use panic_halt as _;
mod mcal;
mod register;
mod startup;
mod bsw;
use crate::bsw::iohwab::{button::get_button_count, iohwab_type::{LedColor, LedState, LedGroup}, led::{led_set_state_group, led_toggle, set_led_state}};
use rtt_target::{rprintln, rtt_init_print};

#[inline(never)]
pub fn delay(mut count1: u32) {
    while count1 > 0 {
        cortex_m::asm::nop();
        count1 -= 1;
    }
}

pub fn main() -> ! {
    rtt_init_print!();
    // Khởi tạo các module MCAL
    mcal::mcu::enable_hsi();
    mcal::port::port_init();
    mcal::exti::exti_init();
    loop {
        // let button_state = mcal::dio::dio_readchannel(mcal::std_type::Dio_ChannelType::UserButton);
        // if button_state == register::gpio_type::Dio_LevelType::HIGH {
        //     // Simple delay loop
        //     delay(160000);
        // }
        rprintln!("Counter: {}", get_button_count());
        match get_button_count() {
            0 => {
                set_led_state(LedColor::Yellow, LedState::Off);
                set_led_state(LedColor::Red, LedState::Off);
                set_led_state(LedColor::Orange, LedState::Off);
                set_led_state(LedColor::Blue, LedState::Off);
            }
            1 => {
                set_led_state(LedColor::Yellow, LedState::On);
                set_led_state(LedColor::Red, LedState::Off);
                set_led_state(LedColor::Orange, LedState::Off);
                set_led_state(LedColor::Blue, LedState::On);
            }
            2 => {
                set_led_state(LedColor::Yellow, LedState::Off);
                set_led_state(LedColor::Red, LedState::On);
                set_led_state(LedColor::Orange, LedState::On);
                set_led_state(LedColor::Blue, LedState::Off);
            }
            3 => {
                set_led_state(LedColor::Yellow, LedState::On);
                set_led_state(LedColor::Red, LedState::On);
                set_led_state(LedColor::Orange, LedState::Off);
                set_led_state(LedColor::Blue, LedState::Off);
            }
            4 => {
                set_led_state(LedColor::Yellow, LedState::Off);
                set_led_state(LedColor::Red, LedState::Off);
                set_led_state(LedColor::Orange, LedState::On);
                set_led_state(LedColor::Blue, LedState::On);
            }
            5 => {
                set_led_state(LedColor::Yellow, LedState::On);
                set_led_state(LedColor::Red, LedState::On);
                set_led_state(LedColor::Orange, LedState::On);
                set_led_state(LedColor::Blue, LedState::On);
            }
            6 => {
                set_led_state(LedColor::Yellow, LedState::Off);
                set_led_state(LedColor::Red, LedState::Off);
                set_led_state(LedColor::Orange, LedState::Off);
                set_led_state(LedColor::Blue, LedState::Off);
            }
            7 => {
                let redyellow = LedGroup::RedYellow;
                let value1 = 0b1010; // Example value, adjust as needed
                led_set_state_group(redyellow, value1);
            }
            8 => {
                let blueorange = LedGroup::BlueOrange;
                let value2 = 0b0101; // Example value, adjust as needed
                led_set_state_group(blueorange, value2);
            }
            _ => {
                let _ = led_toggle(LedColor::Yellow);
                let _ = led_toggle(LedColor::Red);
                let _ = led_toggle(LedColor::Orange);
                let _ = led_toggle(LedColor::Blue);
                // Simple delay loop
                delay(200000);
            }
        }
    }
}
