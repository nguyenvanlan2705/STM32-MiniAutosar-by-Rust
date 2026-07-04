use crate::bsw::ioif::ioif_rx::{ioif_clear_all_rx_indications};
pub fn ioif_init() {
    ioif_clear_all_rx_indications();
}