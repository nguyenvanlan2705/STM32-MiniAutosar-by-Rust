#![no_std]
#![no_main]
use stm32f4 as _;
use panic_halt as _;
mod mcal;
mod register;
mod startup;
use rtt_target::{rprintln, rtt_init_print};
use crate::mcal::exti::COUNT;

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
        rprintln!("Counter: {}", unsafe{COUNT});
        unsafe{
            match COUNT {
            0 => {
                mcal::dio::dio_writechannel(mcal::std_type::Dio_ChannelType::LedYellow, register::gpio_type::Dio_LevelType::LOW);
                mcal::dio::dio_writechannel(mcal::std_type::Dio_ChannelType::LedRed, register::gpio_type::Dio_LevelType::LOW);
                mcal::dio::dio_writechannel(mcal::std_type::Dio_ChannelType::LedOrange, register::gpio_type::Dio_LevelType::LOW);
                mcal::dio::dio_writechannel(mcal::std_type::Dio_ChannelType::LedBlue, register::gpio_type::Dio_LevelType::LOW);
            }
            1 => {
                mcal::dio::dio_writechannel(mcal::std_type::Dio_ChannelType::LedYellow, register::gpio_type::Dio_LevelType::HIGH);
                mcal::dio::dio_writechannel(mcal::std_type::Dio_ChannelType::LedRed, register::gpio_type::Dio_LevelType::LOW);
                mcal::dio::dio_writechannel(mcal::std_type::Dio_ChannelType::LedOrange, register::gpio_type::Dio_LevelType::LOW);
                mcal::dio::dio_writechannel(mcal::std_type::Dio_ChannelType::LedBlue, register::gpio_type::Dio_LevelType::HIGH);
            }
            2 => {
                mcal::dio::dio_writechannel(mcal::std_type::Dio_ChannelType::LedYellow, register::gpio_type::Dio_LevelType::LOW);
                mcal::dio::dio_writechannel(mcal::std_type::Dio_ChannelType::LedRed, register::gpio_type::Dio_LevelType::HIGH);
                mcal::dio::dio_writechannel(mcal::std_type::Dio_ChannelType::LedOrange, register::gpio_type::Dio_LevelType::HIGH);
                mcal::dio::dio_writechannel(mcal::std_type::Dio_ChannelType::LedBlue, register::gpio_type::Dio_LevelType::LOW);
            }
            3 => {
                mcal::dio::dio_writechannel(mcal::std_type::Dio_ChannelType::LedYellow, register::gpio_type::Dio_LevelType::HIGH);
                mcal::dio::dio_writechannel(mcal::std_type::Dio_ChannelType::LedRed, register::gpio_type::Dio_LevelType::HIGH);
                mcal::dio::dio_writechannel(mcal::std_type::Dio_ChannelType::LedOrange, register::gpio_type::Dio_LevelType::LOW);
                mcal::dio::dio_writechannel(mcal::std_type::Dio_ChannelType::LedBlue, register::gpio_type::Dio_LevelType::LOW);
            }
            4 => {
                mcal::dio::dio_writechannel(mcal::std_type::Dio_ChannelType::LedYellow, register::gpio_type::Dio_LevelType::LOW);
                mcal::dio::dio_writechannel(mcal::std_type::Dio_ChannelType::LedRed, register::gpio_type::Dio_LevelType::LOW);
                mcal::dio::dio_writechannel(mcal::std_type::Dio_ChannelType::LedOrange, register::gpio_type::Dio_LevelType::HIGH);
                mcal::dio::dio_writechannel(mcal::std_type::Dio_ChannelType::LedBlue, register::gpio_type::Dio_LevelType::HIGH);
            }
            5 => {
                mcal::dio::dio_writechannel(mcal::std_type::Dio_ChannelType::LedYellow, register::gpio_type::Dio_LevelType::HIGH);
                mcal::dio::dio_writechannel(mcal::std_type::Dio_ChannelType::LedRed, register::gpio_type::Dio_LevelType::HIGH);
                mcal::dio::dio_writechannel(mcal::std_type::Dio_ChannelType::LedOrange, register::gpio_type::Dio_LevelType::HIGH);
                mcal::dio::dio_writechannel(mcal::std_type::Dio_ChannelType::LedBlue, register::gpio_type::Dio_LevelType::HIGH);
            }
            _ => {
                let _ = mcal::dio::dio_flipchannel(mcal::std_type::Dio_ChannelType::LedYellow);
                let _ = mcal::dio::dio_flipchannel(mcal::std_type::Dio_ChannelType::LedRed);
                let _ = mcal::dio::dio_flipchannel(mcal::std_type::Dio_ChannelType::LedOrange);
                let _ = mcal::dio::dio_flipchannel(mcal::std_type::Dio_ChannelType::LedBlue);
                // Simple delay loop
                delay(1600000);
            }
        } 
        }
    }
}