#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::register::usart_type::{UsartNumber};
use crate::register::nvic_type::IRQn;
use crate::mcal::usart_type::{UsartTxRxMode, UsartParityType, Usart_ChannelConfigType, Usart_ConfigType};

pub const USART_CHANNEL_CONFIG : Usart_ConfigType = Usart_ConfigType{
    channels :&[
            Usart_ChannelConfigType {
            usart_number: UsartNumber::USART2,
            baud_rate: 9600,
            parity: UsartParityType::None,
            mode: UsartTxRxMode::INTERRUPT,
            irq_line: IRQn::USART2,
        },
    ],
};


