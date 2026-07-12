//! Startup / vector table cho STM32F411 (Cortex-M4F).
//!
//! Module này tự viết phần khởi động thay cho `cortex-m-rt`:
//!  - Bảng vector (initial stack pointer + exception + interrupt).
//!  - Reset handler: khởi tạo `.data`, xoá `.bss`, bật FPU rồi gọi `main`.
//!  - Các default handler cho exception và ngoại vi.
//!
//! Các ký hiệu `_sbss`, `_ebss`, `_sdata`, `_edata`, `_sidata`, `_stack_start`
//! được cấp bởi linker script `link.x`.

#![allow(non_snake_case)]
use crate::register::syscfg_type::EXTILINE;
use crate::mcal::mcu::systick_1ms_handler;
use crate::mcal::interrupcallback::{exti_irq_handler, exti_group_irq_handler};
use crate::mcal::usart::{usart_irq_handler};
use crate::register::usart_type::{UsartNumber};
/// Một phần tử trong bảng vector: hoặc là con trỏ hàm handler,
/// hoặc là ô "reserved" (giá trị 0).
#[repr(C)]
pub union Vector {
    pub handler: unsafe extern "C" fn(),
    pub reserved: usize,
}

// Các ký hiệu (địa chỉ) do linker script cung cấp.
unsafe extern "C" {
    static mut _sbss: u32; // đầu vùng .bss
    static mut _ebss: u32; // cuối vùng .bss
    static mut _sdata: u32; // đầu vùng .data trong RAM
    static mut _edata: u32; // cuối vùng .data trong RAM
    static _sidata: u32; // ảnh khởi tạo của .data nằm trong FLASH
}

/// Reset handler - điểm vào của chip sau khi reset.
///
/// # Safety
/// Chỉ được gọi bởi phần cứng thông qua bảng vector.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn Reset() -> ! {
    unsafe {
        // 1) Xoá vùng .bss về 0.
        let bss_start = &raw mut _sbss;
        let bss_end = &raw const _ebss;
        let count = (bss_end as usize - bss_start as usize) / core::mem::size_of::<u32>();
        for i in 0..count {
            core::ptr::write_volatile(bss_start.add(i), 0);
        }

        // 2) Sao chép giá trị khởi tạo của .data từ FLASH vào RAM.
        let data_start = &raw mut _sdata;
        let data_end = &raw const _edata;
        let data_src = &raw const _sidata;
        let count = (data_end as usize - data_start as usize) / core::mem::size_of::<u32>();
        for i in 0..count {
            core::ptr::write_volatile(data_start.add(i), core::ptr::read(data_src.add(i)));
        }

        // 3) Bật FPU (Cortex-M4F): cho phép truy cập đầy đủ CP10 & CP11.
        const SCB_CPACR: *mut u32 = 0xE000_ED88 as *mut u32;
        let cpacr = core::ptr::read_volatile(SCB_CPACR);
        core::ptr::write_volatile(SCB_CPACR, cpacr | (0b1111 << 20));
        cortex_m::asm::dsb();
        cortex_m::asm::isb();
    }

    // 4) Chuyển quyền điều khiển cho ứng dụng.
    crate::main()
}

// ---------------------------------------------------------------------------
// Exception handlers (Cortex-M4). Mặc định treo trong vòng lặp vô hạn.
// ---------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub extern "C" fn NMI() {
    loop {
        cortex_m::asm::nop();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn HardFault() {
    loop {
        cortex_m::asm::nop();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn MemManage() {
    loop {
        cortex_m::asm::nop();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn BusFault() {
    loop {
        cortex_m::asm::nop();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn UsageFault() {
    loop {
        cortex_m::asm::nop();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn SVCall() {
    loop {
        cortex_m::asm::nop();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn DebugMonitor() {
    loop {
        cortex_m::asm::nop();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn PendSV() {
    loop {
        cortex_m::asm::nop();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn SysTick_Handler() {
    systick_1ms_handler();
}

/// Handler mặc định cho mọi ngắt ngoại vi chưa được nối riêng.
#[unsafe(no_mangle)]
pub extern "C" fn DefaultHandler() {
    loop {
        cortex_m::asm::nop();
    }
}
/*USARTx Interrupt Handlers */
#[unsafe(no_mangle)]
pub extern "C" fn USART2_IRQHandler() {
    // Handle USART2 interrupt
    usart_irq_handler(UsartNumber::USART2);
}
/*EXTIx Interrupt Handlers */
#[unsafe(no_mangle)]
pub extern "C" fn EXTI0_IRQHandler() {
    exti_irq_handler(EXTILINE::LINE0);
}
#[unsafe(no_mangle)]
pub extern "C" fn EXTI1_IRQHandler() {
    exti_irq_handler(EXTILINE::LINE1);
}
#[unsafe(no_mangle)]
pub extern "C" fn EXTI2_IRQHandler() {
    exti_irq_handler(EXTILINE::LINE2);
}
#[unsafe(no_mangle)]
pub extern "C" fn EXTI3_IRQHandler() {
    exti_irq_handler(EXTILINE::LINE3);
}
#[unsafe(no_mangle)]
pub extern "C" fn EXTI4_IRQHandler() {
    exti_irq_handler(EXTILINE::LINE4);
}
#[unsafe(no_mangle)]
pub extern "C" fn EXTI9_5_IRQHandler() {
    exti_group_irq_handler(&[
        EXTILINE::LINE5,
        EXTILINE::LINE6,
        EXTILINE::LINE7,
        EXTILINE::LINE8,
        EXTILINE::LINE9,
    ]);
}
#[unsafe(no_mangle)]
pub extern "C" fn EXTI15_10_IRQHandler() {
    exti_group_irq_handler(&[
        EXTILINE::LINE10,
        EXTILINE::LINE11,
        EXTILINE::LINE12,
        EXTILINE::LINE13,
        EXTILINE::LINE14,
        EXTILINE::LINE15,
    ]);
}

// ---------------------------------------------------------------------------
// Bảng vector đặt tại đầu FLASH.
// Word đầu tiên (initial stack pointer) do linker script tự phát ra.
// ---------------------------------------------------------------------------

/// Vector Reset (word thứ 2 của bảng vector).
#[unsafe(link_section = ".vector_table.reset_vector")]
#[unsafe(no_mangle)]
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = Reset;

/// 14 exception vector (vị trí 2..15).
#[unsafe(link_section = ".vector_table.exceptions")]
#[unsafe(no_mangle)]
pub static __EXCEPTIONS: [Vector; 14] = [
    Vector { handler: NMI },          // 2  NMI
    Vector { handler: HardFault },    // 3  HardFault
    Vector { handler: MemManage },    // 4  MemManage
    Vector { handler: BusFault },     // 5  BusFault
    Vector { handler: UsageFault },   // 6  UsageFault
    Vector { reserved: 0 },           // 7  Reserved
    Vector { reserved: 0 },           // 8  Reserved
    Vector { reserved: 0 },           // 9  Reserved
    Vector { reserved: 0 },           // 10 Reserved
    Vector { handler: SVCall },       // 11 SVCall
    Vector { handler: DebugMonitor }, // 12 Debug Monitor
    Vector { reserved: 0 },           // 13 Reserved
    Vector { handler: PendSV },       // 14 PendSV
    Vector { handler: SysTick_Handler },      // 15 SysTick
];

/// 86 vector ngắt ngoại vi (IRQ 0..85) của STM32F411.
#[unsafe(link_section = ".vector_table.interrupts")]
#[unsafe(no_mangle)]
pub static __INTERRUPTS: [Vector; 86] = [
    Vector { handler: DefaultHandler },          // 0  WWDG
    Vector { handler: DefaultHandler },          // 1  PVD
    Vector { handler: DefaultHandler },          // 2  TAMP_STAMP
    Vector { handler: DefaultHandler },          // 3  RTC_WKUP
    Vector { handler: DefaultHandler },          // 4  FLASH
    Vector { handler: DefaultHandler },          // 5  RCC
    Vector { handler: EXTI0_IRQHandler },        // 6  EXTI0
    Vector { handler: EXTI1_IRQHandler },          // 7  EXTI1
    Vector { handler: EXTI2_IRQHandler },          // 8  EXTI2
    Vector { handler: EXTI3_IRQHandler },          // 9  EXTI3
    Vector { handler: EXTI4_IRQHandler },          // 10 EXTI4
    Vector { handler: DefaultHandler },          // 11 DMA1_Stream0
    Vector { handler: DefaultHandler },          // 12 DMA1_Stream1
    Vector { handler: DefaultHandler },          // 13 DMA1_Stream2
    Vector { handler: DefaultHandler },          // 14 DMA1_Stream3
    Vector { handler: DefaultHandler },          // 15 DMA1_Stream4
    Vector { handler: DefaultHandler },          // 16 DMA1_Stream5
    Vector { handler: DefaultHandler },          // 17 DMA1_Stream6
    Vector { handler: DefaultHandler },          // 18 ADC
    Vector { reserved: 0 },                      // 19 Reserved
    Vector { reserved: 0 },                      // 20 Reserved
    Vector { reserved: 0 },                      // 21 Reserved
    Vector { reserved: 0 },                      // 22 Reserved
    Vector { handler: EXTI9_5_IRQHandler },      // 23 EXTI9_5
    Vector { handler: DefaultHandler },          // 24 TIM1_BRK_TIM9
    Vector { handler: DefaultHandler },          // 25 TIM1_UP_TIM10
    Vector { handler: DefaultHandler },          // 26 TIM1_TRG_COM_TIM11
    Vector { handler: DefaultHandler },          // 27 TIM1_CC
    Vector { handler: DefaultHandler },          // 28 TIM2
    Vector { handler: DefaultHandler },          // 29 TIM3
    Vector { handler: DefaultHandler },          // 30 TIM4
    Vector { handler: DefaultHandler },          // 31 I2C1_EV
    Vector { handler: DefaultHandler },          // 32 I2C1_ER
    Vector { handler: DefaultHandler },          // 33 I2C2_EV
    Vector { handler: DefaultHandler },          // 34 I2C2_ER
    Vector { handler: DefaultHandler },          // 35 SPI1
    Vector { handler: DefaultHandler },          // 36 SPI2
    Vector { handler: DefaultHandler },          // 37 USART1
    Vector { handler: USART2_IRQHandler },       // 38 USART2
    Vector { reserved: 0 },                      // 39 Reserved
    Vector { handler: EXTI15_10_IRQHandler },    // 40 EXTI15_10
    Vector { handler: DefaultHandler },          // 41 RTC_Alarm
    Vector { handler: DefaultHandler },          // 42 OTG_FS_WKUP
    Vector { reserved: 0 },                      // 43 Reserved
    Vector { reserved: 0 },                      // 44 Reserved
    Vector { reserved: 0 },                      // 45 Reserved
    Vector { reserved: 0 },                      // 46 Reserved
    Vector { handler: DefaultHandler },          // 47 DMA1_Stream7
    Vector { reserved: 0 },                      // 48 Reserved
    Vector { handler: DefaultHandler },          // 49 SDIO
    Vector { handler: DefaultHandler },          // 50 TIM5
    Vector { handler: DefaultHandler },          // 51 SPI3
    Vector { reserved: 0 },                      // 52 Reserved
    Vector { reserved: 0 },                      // 53 Reserved
    Vector { reserved: 0 },                      // 54 Reserved
    Vector { reserved: 0 },                      // 55 Reserved
    Vector { handler: DefaultHandler },          // 56 DMA2_Stream0
    Vector { handler: DefaultHandler },          // 57 DMA2_Stream1
    Vector { handler: DefaultHandler },          // 58 DMA2_Stream2
    Vector { handler: DefaultHandler },          // 59 DMA2_Stream3
    Vector { handler: DefaultHandler },          // 60 DMA2_Stream4
    Vector { reserved: 0 },                      // 61 Reserved
    Vector { reserved: 0 },                      // 62 Reserved
    Vector { reserved: 0 },                      // 63 Reserved
    Vector { reserved: 0 },                      // 64 Reserved
    Vector { reserved: 0 },                      // 65 Reserved
    Vector { reserved: 0 },                      // 66 Reserved
    Vector { handler: DefaultHandler },          // 67 OTG_FS
    Vector { handler: DefaultHandler },          // 68 DMA2_Stream5
    Vector { handler: DefaultHandler },          // 69 DMA2_Stream6
    Vector { handler: DefaultHandler },          // 70 DMA2_Stream7
    Vector { handler: DefaultHandler },          // 71 USART6
    Vector { handler: DefaultHandler },          // 72 I2C3_EV
    Vector { handler: DefaultHandler },          // 73 I2C3_ER
    Vector { reserved: 0 },                      // 74 Reserved
    Vector { reserved: 0 },                      // 75 Reserved
    Vector { reserved: 0 },                      // 76 Reserved
    Vector { reserved: 0 },                      // 77 Reserved
    Vector { reserved: 0 },                      // 78 Reserved
    Vector { reserved: 0 },                      // 79 Reserved
    Vector { reserved: 0 },                      // 80 Reserved
    Vector { handler: DefaultHandler },          // 81 FPU
    Vector { reserved: 0 },                      // 82 Reserved
    Vector { reserved: 0 },                      // 83 Reserved
    Vector { handler: DefaultHandler },          // 84 SPI4
    Vector { handler: DefaultHandler },          // 85 SPI5
];
