use crate::bsw::ioif::{ioif_rx::ioif_read_rx_value};
use crate::bsw::ioif::ioif_type::{IoIf_ReturnType};
use core::sync::atomic::{AtomicU16, Ordering};

static LAST_BUTTON_STATUS: AtomicU16 = AtomicU16::new(0);

pub fn button_app_1ms() -> u16 {
    let mut count: u16 = LAST_BUTTON_STATUS.load(Ordering::Relaxed);

    if ioif_read_rx_value(0x100, &mut count) == IoIf_ReturnType::IOIF_E_OK {
        LAST_BUTTON_STATUS.store(count, Ordering::Relaxed);
    }
    count
}