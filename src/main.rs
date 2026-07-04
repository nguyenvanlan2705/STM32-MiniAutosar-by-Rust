#![no_std]
#![no_main]
use stm32f4 as _;
use panic_halt as _;
mod mcal;
mod register;
mod startup;
mod bsw;
use crate::bsw::{iohwab::{
    iohwab_type::{LedColor},
    led::{led_toggle},
},
    ioif::ioif_rx::ioif_read_rx_value,
    ioif::ioif::ioif_init,
    ioif::ioif_tx::{ioif_write_tx_state, ioif_write_tx_group_state},
    ioif::ioif_type::IoIf_OutputType,};
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
    ioif_init();

    let mut count :u8 =0;
    loop {
        // let button_state = mcal::dio::dio_readchannel(mcal::std_type::Dio_ChannelType::UserButton);
        // if button_state == register::gpio_type::Dio_LevelType::HIGH {
        //     // Simple delay loop
        //     delay(160000);
        // }

        let _ = ioif_read_rx_value(0x100, &mut count);
        rprintln!("Counter1: {}", count);
        match count {
            0 => {
                ioif_write_tx_state(0x203, IoIf_OutputType::STD_OFF);
                ioif_write_tx_state(0x200, IoIf_OutputType::STD_OFF);
                ioif_write_tx_state(0x201, IoIf_OutputType::STD_OFF);
                ioif_write_tx_state(0x202, IoIf_OutputType::STD_OFF);
            }
            1 => {
                ioif_write_tx_state(0x203, IoIf_OutputType::STD_ON);
                ioif_write_tx_state(0x200, IoIf_OutputType::STD_OFF);
                ioif_write_tx_state(0x202, IoIf_OutputType::STD_OFF);
                ioif_write_tx_state(0x201, IoIf_OutputType::STD_ON);
            }
            2 => {
                ioif_write_tx_state(0x203, IoIf_OutputType::STD_OFF);
                ioif_write_tx_state(0x200, IoIf_OutputType::STD_ON);
                ioif_write_tx_state(0x202, IoIf_OutputType::STD_ON);
                ioif_write_tx_state(0x201, IoIf_OutputType::STD_OFF);
            }
            3 => {
                ioif_write_tx_state(0x203, IoIf_OutputType::STD_ON);
                ioif_write_tx_state(0x200, IoIf_OutputType::STD_ON);
                ioif_write_tx_state(0x202, IoIf_OutputType::STD_OFF);
                ioif_write_tx_state(0x201, IoIf_OutputType::STD_OFF);
            }
            4 => {
                ioif_write_tx_state(0x203, IoIf_OutputType::STD_OFF);
                ioif_write_tx_state(0x200, IoIf_OutputType::STD_OFF);
                ioif_write_tx_state(0x202, IoIf_OutputType::STD_ON);
                ioif_write_tx_state(0x201, IoIf_OutputType::STD_ON);
            }
            5 => {
                ioif_write_tx_state(0x203, IoIf_OutputType::STD_ON);
                ioif_write_tx_state(0x200, IoIf_OutputType::STD_ON);
                ioif_write_tx_state(0x202, IoIf_OutputType::STD_ON);
                ioif_write_tx_state(0x201, IoIf_OutputType::STD_ON);
            }
            6 => {
                ioif_write_tx_state(0x203, IoIf_OutputType::STD_OFF);
                ioif_write_tx_state(0x200, IoIf_OutputType::STD_OFF);
                ioif_write_tx_state(0x202, IoIf_OutputType::STD_OFF);
                ioif_write_tx_state(0x201, IoIf_OutputType::STD_OFF);
            }
            7 => {
                let redyellow = 0b1100; // Example value, adjust as needed
                ioif_write_tx_group_state(0x300, redyellow);
            }
            8 => {
                let blueorange = 0b0011; // Example value, adjust as needed
                ioif_write_tx_group_state(0x301, blueorange);
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
