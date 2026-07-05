use crate::bsw::ioif::ioif_rx::{ioif_clear_all_rx_indications};
use crate::bsw::ioif::ioif_tx::{ioif_clearall_tx_confirmation_and_outputstatus};
pub fn ioif_init() {
    ioif_clear_all_rx_indications();
    ioif_clearall_tx_confirmation_and_outputstatus();
}