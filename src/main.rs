#![no_std]
#![no_main]
use stm32f4 as _;
use panic_halt as _;
mod mcal;
mod register;
mod startup;
mod bsw;
mod app;
use crate::{bsw::{
    cfg::comm_cfg::ComMUser, management::comm::{comm::comm_requestcommode, comm_type::ComMRequestedMode,},
}};
use crate::bsw::services::scheduler::{scheduler_oneshot_task};
use crate::mcal::external::mcp2515::{mcp2515_read_register_ctrl, mcp2515_read_register_status};
use crate::mcal::external::mcp2515_type::{Mcp2515DeviceId};

fn delay(count: u32) {
    for _ in 0..count {
        cortex_m::asm::nop();
    }
}

static mut MCP2515_CTRL_REG: u8 = 0;
static mut MCP2515_STATUS_REG: u8 = 0;
pub fn main() -> ! {
    scheduler_oneshot_task();
    comm_requestcommode(ComMUser::APP_GPIO, ComMRequestedMode::FULL_COMMUNICATION);
    loop {
        let mcp2515_read_register_ctrl = mcp2515_read_register_ctrl(Mcp2515DeviceId::MCP2515_1);
        let mcp2515_read_register_status = mcp2515_read_register_status(Mcp2515DeviceId::MCP2515_1);
        unsafe {
            MCP2515_CTRL_REG = mcp2515_read_register_ctrl;
            MCP2515_STATUS_REG = mcp2515_read_register_status;
        }
        delay(1000000);
    }
}
