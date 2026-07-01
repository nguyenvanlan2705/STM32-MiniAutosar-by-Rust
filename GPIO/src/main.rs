#![no_std]
#![no_main]
use stm32f4 as _;
use panic_halt as _;
use cortex_m_rt::entry;
mod mcal;
mod register;

#[entry]
fn main() -> ! {
    mcal::port::port_init();
    loop {
        let button_state = register::dio::dio_read(register::gpio_type::PORT::A, register::gpio_type::PIN::P0);
        if button_state == register::gpio_type::Level::High {
            register::dio::dio_write(register::gpio_type::PORT::D, register::gpio_type::PIN::P12, register::gpio_type::Level::High);
        } else {
            register::dio::dio_write(register::gpio_type::PORT::D, register::gpio_type::PIN::P12, register::gpio_type::Level::Low);
        }
    }
}