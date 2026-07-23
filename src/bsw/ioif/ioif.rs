use crate::bsw::ioif::ioif_rx::{ioif_clear_all_rx_runtime_status};
use crate::bsw::ioif::ioif_tx::{ioif_clearall_tx_runtime_status};
pub fn ioif_init() {
    ioif_clear_all_rx_runtime_status();
    ioif_clearall_tx_runtime_status();
}