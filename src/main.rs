#![no_std]
#![no_main]
use stm32f4 as _;
use panic_halt as _;
mod mcal;
mod register;
mod startup;
mod bsw;
mod app;
use crate::bsw::{
    management::comm::{comm::{comm_requestcommode},
                       comm_type::{ComMRequestedMode},},
    cfg::comm_cfg::ComMUser,
};
use crate::bsw::services::scheduler::{scheduler_oneshot_task, scheduler_mainfunction, scheduler_init};
use crate::mcal::mcu::{mcu_init, mcu_init_systick_1ms, };

pub fn main() -> ! {
    // Khởi tạo các module MCAL
    mcu_init();
    mcu_init_systick_1ms();
    mcal::port::port_init();
    mcal::exti::exti_init();
    scheduler_init();
    scheduler_oneshot_task();
    comm_requestcommode(ComMUser::APP_GPIO, ComMRequestedMode::FULL_COMMUNICATION);
    //let mut count: u32 = 0;
    loop {
        scheduler_mainfunction();
    }
}
