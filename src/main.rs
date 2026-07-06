#![no_std]
#![no_main]
use stm32f4 as _;
use panic_halt as _;
mod mcal;
mod register;
mod startup;
mod bsw;
use crate::bsw::{
    ioif::ioif_rx::ioif_read_rx_value,
    ioif::ioif::ioif_init,
    ioif::ioif_tx::{ioif_write_tx_state, ioif_write_tx_group_state},
    ioif::ioif_type::IoIf_OutputType,
    management::comm::{comm::{comm_init, comm_mainfunction, comm_getcurrentcommode, comm_requestcommode},
                       comm_type::{ComMRequestedMode, ComM_NetWorkHandleType, ComMMode},},
    cfg::comm_cfg::ComMUser,
};

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
    mcal::mcu::mcu_init();
    mcal::port::port_init();
    mcal::exti::exti_init();
    ioif_init();
    comm_init();
    comm_requestcommode(ComMUser::APP_GPIO, ComMRequestedMode::FULL_COMMUNICATION);
    let mut count :u8 =0;
    loop {
        // let button_state = mcal::dio::dio_readchannel(mcal::std_type::Dio_ChannelType::UserButton);
        // if button_state == register::gpio_type::Dio_LevelType::HIGH {
        //     // Simple delay loop
        //     delay(160000);
        // }
        comm_mainfunction();
        let commode = comm_getcurrentcommode(ComM_NetWorkHandleType::GPIO);
        if let Some(commode) = commode {
            rprintln!("Current Communication Mode: {:?}", commode);
            if commode == ComMMode::FULL_COMMUNICATION {
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
                        let orangeyellow = 0b0011; // Example value, adjust as needed
                        ioif_write_tx_group_state(0x301, orangeyellow);
                    }
                    _ => {
                        ioif_write_tx_state(0x203, IoIf_OutputType::TOGGLE);
                        ioif_write_tx_state(0x200, IoIf_OutputType::TOGGLE);
                        ioif_write_tx_state(0x202, IoIf_OutputType::TOGGLE);
                        ioif_write_tx_state(0x201, IoIf_OutputType::TOGGLE);
                        // Simple delay loop
                        delay(200000);
                    }
                }
            }
        }
    }
}
