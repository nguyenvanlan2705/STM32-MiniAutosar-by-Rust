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
- EXTI1..EXTI4 vector entries are wired.
- EXTI9_5 and EXTI15_10 grouped handlers are wired.

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

More detailed reset sequence:

```text
1. CPU loads MSP from vector table word 0
2. CPU jumps to vector table word 1, the Reset handler
3. Reset clears .bss so static zero variables start as 0
4. Reset copies .data initial values from Flash to RAM
5. Reset enables FPU for Cortex-M4F floating-point support
6. Reset calls main()
7. main() initializes MCAL/BSW modules
```

Why `.bss` matters:

```text
static BUTTON_COUNT: AtomicU8 = AtomicU8::new(0);
static IOIF_INDICATION_TABLE: [AtomicU8; ...] = [...];
static EXTI_CALLBACK: [AtomicUsize; 16] = [...];
```

These depend on startup zeroing/copying memory correctly.

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

Current EXTI vector flow:

```text
IRQ6  -> EXTI0_IRQHandler()      -> exti_irq_handler(LINE0)
IRQ7  -> EXTI1_IRQHandler()      -> exti_irq_handler(LINE1)
IRQ8  -> EXTI2_IRQHandler()      -> exti_irq_handler(LINE2)
IRQ9  -> EXTI3_IRQHandler()      -> exti_irq_handler(LINE3)
IRQ10 -> EXTI4_IRQHandler()      -> exti_irq_handler(LINE4)
IRQ23 -> EXTI9_5_IRQHandler()    -> exti_group_irq_handler(LINE5..LINE9)
IRQ40 -> EXTI15_10_IRQHandler()  -> exti_group_irq_handler(LINE10..LINE15)
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

Good interrupt layering:

```text
Vector table
    |
    v
Startup IRQ handler
    |
    v
MCAL interrupt dispatcher
    |
    v
Configured callback
    |
    v
IoHwAb / IoIf / App-level behavior
```

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
The active interrupt path currently covers PA0 -> EXTI0.
Vector handlers also exist for EXTI1..4, EXTI9_5, and EXTI15_10.
```

## Common Mistakes

### 1. Wrong vector table symbol placement

The vector table must be placed where the CPU expects it at boot.

### 2. Forgetting `#[unsafe(no_mangle)]`

Interrupt handler names must not be renamed by Rust name mangling.

### 3. Doing application logic in startup

Startup should initialize memory and dispatch interrupts. It should not know LED patterns, button counters, or application state.

### 4. Forgetting `.bss` and `.data`

If `.bss` is not cleared, zero-initialized globals may start with garbage.

If `.data` is not copied, initialized globals may have wrong values.

### 5. Choosing the wrong global datatype

For interrupt-shared state, prefer atomics for simple values:

```text
AtomicU8    small counters/status flags
AtomicU32   shared bit masks or 32-bit values
AtomicUsize pointer-sized callback storage
```

See `docs/GlobalData.md` for the current project guideline.
