# Development Log

## Phase 0 - Environment Setup

Done:

- Created Rust embedded project for STM32F411.
- Used target:

```bash
rustup target add thumbv7em-none-eabihf
```

- Configured `.cargo/config.toml`.
- Used `probe-rs`, OpenOCD, Cortex-Debug, SVD, Memory Viewer.
- Debugged:
  - ST-Link WinUSB driver
  - `probe-rs` flashing
  - OpenOCD access error
  - GDB path
  - VS Code memory/peripheral viewer
  - Panic handler missing
  - Interrupt vector missing

## Phase 1 - Basic Rust Embedded

Learned:

- `#![no_std]`
- `#![no_main]`
- `#[entry]`
- `panic-halt`
- `memory.x`
- `link.x`
- `cargo build`
- `cargo run`

Important issue:

```text
#[panic_handler] function required, but not found
```

Fix:

```rust
use panic_halt as _;
```

or define a custom `#[panic_handler]`.

## Phase 2 - HAL GPIO

Initially used `stm32f4xx-hal`.

Learned:

- GPIO split
- GPIO output
- Button input
- `into_push_pull_output`
- `into_pull_down_input`
- `is_high()`
- `set_high()`
- `set_low()`

Then moved away from HAL to build own MCAL from Reference Manual.

## Phase 3 - Register Layer

Started defining registers manually:

- `RCCRegister`
- `PortRegister`
- `SYSCFGRegister`
- `EXTIRegister`
- `NVICRegister`

Important lessons:

- Use `#[repr(C)]` for register structs.
- Use `read_volatile()` and `write_volatile()`.
- Do not read write-only registers like `GPIOx_BSRR`.
- Register layer should not contain high-level logic.

## Phase 4 - Port Driver

Implemented `Port_Init()` style driver.

Responsibilities:

- Enable GPIO clock
- Configure mode
- Configure output type
- Configure speed
- Configure pull-up/pull-down

Important idea:

```text
Port configures pins.
Dio reads/writes pins.
```

## Phase 5 - Dio Driver

Implemented DIO APIs inspired by AUTOSAR:

- `Dio_ReadChannel`
- `Dio_WriteChannel`
- `Dio_FlipChannel`
- `Dio_ReadPort`
- `Dio_WritePort`
- Channel config and logical channel mapping

Important design decision:

Application should use logical channels:

```rust
Dio_ChannelType::LedRed
Dio_ChannelType::UserButton
```

not physical pins:

```rust
PORT::D, PIN::P12
```

## Phase 6 - EXTI Driver

Implemented the first EXTI driver path:

- SYSCFG EXTI line mapping
- EXTI IMR enable/disable
- Rising/falling trigger config
- Pending clear
- NVIC enable
- Callback table for EXTI lines
- PA0 -> EXTI0 configuration
- EXTI0 button callback increments a counter used by the LED demo

Important rule:

```text
DIO does not know interrupts.
EXTI handles interrupt lines.
Port/SYSCFG configures pin-to-EXTI mapping.
```

## Phase 7 - Startup / Vector Table

Replaced the `cortex-m-rt` startup path with custom startup code:

- Vector table
- Reset handler
- Exception handlers
- Interrupt table
- FPU enable
- `.bss` zero
- `.data` copy
- EXTI0 vector mapping
- EXTI0 vector dispatches into the MCAL EXTI handler

Important design rule:

Startup should only dispatch IRQs to MCAL handlers.

Good flow:

```text
Vector Table
    |
    v
EXTI0_IRQHandler
    |
    v
mcal::interrupcallback::exti_irq_handler(LINE0)
    |
    v
Registered callback
```

## Phase 8 - Current LED/Button Demo

Current runnable flow:

```text
Reset
    |
    v
main()
    |
    +-- mcal::mcu::enable_hsi()
    +-- mcal::port::port_init()
    +-- mcal::exti::exti_init()
    |
    v
PA0 button rising edge
    |
    v
EXTI0_IRQHandler()
    |
    v
mcal::interrupcallback::exti_irq_handler(LINE0)
    |
    v
registered button callback increments COUNT
    |
    v
main loop writes LED pattern through Dio
```

Hardware mapping currently used by the demo:

- PA0: user button input, pulldown
- PD12: yellow LED
- PD13: orange LED
- PD14: red LED
- PD15: blue LED

Build status:

```text
cargo check --target thumbv7em-none-eabihf
```

passes. Current warnings are mostly naming-style warnings from AUTOSAR-like names and unused future APIs.

## Phase 9 - Future BSW/RTE Idea

Planned BSW modules:

- IoHwAb
- IoIf inspired by CanIf
- UartIf
- PduR
- Com

UART can be used to simulate PDU flow:

```text
SOF | ID | LEN | DATA | CRC
```

Then UartIf can call:

```text
PduR_RxIndication()
PduR_TxConfirmation()
```

## Current Status

Completed/mostly completed:

- Register layer for RCC/GPIO/SYSCFG/EXTI/NVIC
- Port MCAL
- Dio MCAL
- Basic MCU clock helper
- EXTI register functions
- EXTI0 callback table path
- Custom startup/vector table
- LED/button interrupt demo using MCAL APIs

Scaffolded but not yet active in the main flow:

- `src/app`
- `src/rte`
- `src/bsw`
- `src/config`
- MCAL placeholders for ADC/CAN/GPT/PWM/SPI/UART/WDG

Next recommended work:

1. Clean up EXTI pending handling and callback safety.
2. Add dispatchers for EXTI1..4, EXTI9_5, and EXTI15_10.
3. Move LED/button access behind IoHwAb and RTE.
4. Move Port/Dio/Exti config objects into `src/config`.
5. Add `.gitignore` coverage for `target/` build output.
6. Replace `static mut COUNT` with a safer interrupt-shared state pattern.
