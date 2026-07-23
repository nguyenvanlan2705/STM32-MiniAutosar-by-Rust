#![allow(dead_code)]

use crate::bsw::ioif::ioif_tx::{ioif_write_tx_state};
use crate::bsw::ioif::ioif_type::IoIf_OutputType;

pub fn led_app_1ms(status: u8) {
    match status {
        1 => {
            ioif_write_tx_state(0x200, IoIf_OutputType::STD_OFF);
            ioif_write_tx_state(0x201, IoIf_OutputType::STD_OFF);
        }
        2 => {
            ioif_write_tx_state(0x200, IoIf_OutputType::STD_OFF);
            ioif_write_tx_state(0x201, IoIf_OutputType::STD_ON);
        }
        3 => {
            ioif_write_tx_state(0x200, IoIf_OutputType::STD_ON);
            ioif_write_tx_state(0x201, IoIf_OutputType::STD_OFF);
        }
        4 => {
            ioif_write_tx_state(0x200, IoIf_OutputType::STD_ON);
            ioif_write_tx_state(0x201, IoIf_OutputType::STD_ON);
        }
        5 => {
            ioif_write_tx_state(0x200, IoIf_OutputType::STD_OFF);
            ioif_write_tx_state(0x201, IoIf_OutputType::STD_ON);
        }
        6 => {
            ioif_write_tx_state(0x200, IoIf_OutputType::STD_ON);
            ioif_write_tx_state(0x201, IoIf_OutputType::STD_OFF);
        }
        7 => {
            ioif_write_tx_state(0x200, IoIf_OutputType::STD_OFF);
            ioif_write_tx_state(0x201, IoIf_OutputType::STD_OFF);
        }
        _ =>{
            // Handle other cases or do nothing
        }
    }
}
pub fn led_app_200ms() {
}
pub fn led_app_500ms() {
    // Toggle the outputs every 500ms
    ioif_write_tx_state(0x203, IoIf_OutputType::TOGGLE);
    ioif_write_tx_state(0x202, IoIf_OutputType::TOGGLE);
}
