use crate::register::rcc_type;

pub fn enable_hsi(){
    unsafe {
        let rcc = rcc_type::get_rcc_register();
        let shift_value = core::ptr::read_volatile(&(*rcc).rcc_cr) | (1 << rcc_type::CR::HSION as u32);
        core::ptr::write_volatile(&mut (*rcc).rcc_cr, shift_value);
        while (core::ptr::read_volatile(&(*rcc).rcc_cr)
            & (1 << rcc_type::CR::HSIRDY as u32))
            == 0
        {}
    }
}