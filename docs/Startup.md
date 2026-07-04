# Startup and Vector Table

## Purpose

Startup code replaces the default embedded runtime.

Responsibilities:

- Vector table
- Reset handler
- Clear `.bss`
- Copy `.data`
- Enable FPU
- Call `main`
- Provide exception handlers
- Provide interrupt handlers

## Current Implementation Status

Implemented:

- Custom reset handler named `Reset`.
- `.bss` zero initialization.
- `.data` copy from Flash to RAM.
- FPU enable for Cortex-M4F.
- Exception vector table.
- STM32F411 interrupt vector table with 86 IRQ slots.
- EXTI0 vector entry wired to `EXTI0_IRQHandler`.
- `EXTI0_IRQHandler` dispatches to the MCAL EXTI interrupt handler.

The project keeps `use stm32f4 as _;` so the PAC crate is linked, but the PAC runtime vector table is not used.

## Boot Flow

```text
Reset
    |
    v
CPU reads initial stack pointer from vector table
    |
    v
CPU jumps to Reset handler
    |
    v
Reset handler clears .bss
    |
    v
Reset handler copies .data from Flash to RAM
    |
    v
Enable FPU
    |
    v
Call main()
```

## Linker Symbols

The linker script should provide:

```text
_sbss
_ebss
_sdata
_edata
_sidata
_stack_start
```

## Vector Table Structure

```text
Word 0  Initial MSP
Word 1  Reset Handler
Word 2  NMI
Word 3  HardFault
...
Word 15 SysTick
Word 16 IRQ0
Word 17 IRQ1
...
```

For STM32F411:

```text
IRQ6  = EXTI0
IRQ7  = EXTI1
IRQ8  = EXTI2
IRQ9  = EXTI3
IRQ10 = EXTI4
IRQ23 = EXTI9_5
IRQ40 = EXTI15_10
```

## Recommended Layering

Startup should only dispatch interrupts.

Example:

```rust
pub extern "C" fn EXTI0_IRQHandler() {
    crate::mcal::interrupcallback::exti_irq_handler(EXTILINE::LINE0);
}
```

Do not call application callback directly from the vector table.

## Common Error

```text
ERROR(cortex-m-rt): The interrupt vectors are missing.
```

Cause:

- Device crate not linked
- `rt` feature missing
- No custom vector table

Temporary fix if using PAC only for vector table:

```rust
use stm32f4 as _;
```

Long-term fix:

- Provide your own vector table and startup code.

Current status:

```text
The custom vector table and startup code are already present.
The active interrupt path currently covers EXTI0.
```
