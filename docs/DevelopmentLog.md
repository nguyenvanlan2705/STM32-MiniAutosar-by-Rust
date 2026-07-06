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
- `Dio_ReadOutputChannel`
- `Dio_ReadChannelGroup`
- `Dio_WriteChannelGroup` draft
- Channel config and logical channel mapping
- Channel group config for LED groups on port D

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
- Notification-style APIs:
  - `exti_enable_notification`
  - `exti_disable_notification`
- Config-based callback registration through `callbackfn`
- Pending check helper for grouped interrupt dispatch
- EXTI0 button callback is now routed to IoHwAb button logic
- EXTI callback table now stores callback addresses with `AtomicUsize`

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
- EXTI1..EXTI4 vector handlers
- EXTI9_5 and EXTI15_10 group handlers
- Group handlers dispatch only pending EXTI lines

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
    +-- mcal::mcu::mcu_init()
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
registered callback enters IoHwAb button logic
    |
    v
IoHwAb increments BUTTON_COUNT
    |
    v
main loop reads IoHwAb button count and writes LED pattern through IoHwAb LED APIs
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

passes.

## Phase 9 - IoHwAb Introduction

Started moving demo behavior above MCAL:

- Added `src/bsw/iohwab/button.rs`.
- Added `src/bsw/iohwab/led.rs`.
- Added `src/bsw/iohwab/iohwab_type.rs`.
- Button EXTI callback now increments `BUTTON_COUNT` in IoHwAb.
- Initially, `main.rs` read the button count through `get_button_count()`.
- This direct read has now been superseded by the IoIf RX read path.
- LED operations in `main.rs` now go through IoHwAb APIs:
  - `set_led_state`
  - `led_toggle`
  - `led_set_state_group`
- IoHwAb maps friendly types such as `LedColor`, `LedState`, `Button`, and `LedGroup` to MCAL Dio channels/groups.

## Phase 10 - IoIf RX Indication Draft

Started moving the button event flow above IoHwAb:

- Added `src/bsw/ioif/ioif_type.rs`.
- Added IoIf config. It now lives under `src/bsw/cfg/ioif_cfg.rs`.
- Added `src/bsw/ioif/ioif_rx.rs`.
- Added `src/bsw/ioif/ioif.rs`.
- Added RX PDU config for the user button:
  - PDU ID: `0x100`
  - Peripheral: `DIO`
  - Channel: `BUTTON_USER`
  - Mode: `INTERRUPT`
- Added `IOIF_INDICATION_TABLE` sized from `IOIF_RX_PDU_COUNT`.
- `button_exti_notification()` now calls `ioif_rxindication(0x100)` after updating the button count.
- `ioif_rxindication()` marks the configured RX PDU as active.
- `ioif_init()` clears all RX indications during startup.
- `main.rs` calls `ioif_read_rx_value(0x100, &mut count)` and uses that value for the LED pattern.

Current intended flow:

```text
PA0 rising edge
    |
    v
EXTI0 interrupt
    |
    v
MCAL EXTI dispatcher
    |
    v
IoHwAb button_exti_notification()
    |
    v
IoIf ioif_rxindication(0x100)
    |
    v
IoIf indication table marks RX PDU active
    |
    v
main.rs reads through ioif_read_rx_value()
```

## Phase 11 - IoIf TX Confirmation Draft

Started routing LED output through IoIf TX:

- Added `src/bsw/ioif/ioif_tx.rs`.
- Added TX PDU config for LED outputs:
  - `0x200`: `LED_RED`
  - `0x201`: `LED_ORANGE`
  - `0x202`: `LED_BLUE`
  - `0x203`: `LED_YELLOW`
- Added TX group PDU config for LED groups:
  - `0x300`: `LED_GROUP_RED_BLUE`
  - `0x301`: `LED_GROUP_ORANGE_YELLOW`
- Added `IoIf_TxChannelType` and `IoIf_OutputType`.
- Added `IoIf_TxChannelGroupType` and `IoIf_TxPduGroup`.
- `main.rs` now uses `ioif_write_tx_state()` for normal LED on/off writes.
- `main.rs` now uses `ioif_write_tx_group_state()` for grouped LED writes.
- `IoIf_OutputType::TOGGLE` is supported for single LED TX PDUs.
- `ioif_write_tx_state()` maps TX PDU IDs to IoHwAb LED operations.
- `ioif_write_tx_group_state()` maps TX group PDU IDs to IoHwAb LED group operations.
- `ioif_txconfirmation()` records the write result in `IOIF_TX_CONFIRMATION_TABLE`.
- `ioif_txconfirmation()` is shared by single TX PDU and group TX PDU paths.
- Single TX confirmations are stored in `IOIF_TX_CONFIRMATION_TABLE`.
- Group TX confirmations are stored in `IOIF_TX_GROUP_CONFIRMATION_TABLE`.
- `BUTTON_COUNT`, `IOIF_INDICATION_TABLE`, `IOIF_TX_CONFIRMATION_TABLE`, and `IOIF_TX_GROUP_CONFIRMATION_TABLE` now use `AtomicU8`.
- TX confirmation records command result only. It does not represent the current physical LED ON/OFF state after a toggle.

## Phase 12 - Repository Hygiene

Done:

- Added root `.gitignore`.
- Ignored `target/` build output.
- Removed previously tracked `target/` files from Git index.
- Build artifacts should no longer pollute `git status` after this point.

## Phase 13 - Future BSW/RTE Idea

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

## Phase 14 - ComM Draft

Started BSW management layer with ComM:

- Added `src/bsw/management/comm/comm.rs`.
- Added `src/bsw/management/comm/comm_type.rs`.
- Added generated-style ComM config in `src/bsw/cfg/comm_cfg.rs`.
- Added `ComMUser` as a config-level type:
  - `APP_GPIO`
  - `DIAG_UART`
  - `MANAGEMENT_CAN`
  - `APP_SPI`
- Added network handles:
  - `GPIO`
  - `UART`
  - `CAN`
  - `SPI`
- Added current mode table and requested mode table using `AtomicU8`.
- Added `comm_requestcommode()` to store requested mode by configured user.
- Added `comm_mainfunction()` to process requested mode into current mode.
- Added `comm_getcurrentcommode()` to read the current network mode.

Important ComM lesson:

```text
comm_requestcommode() receives a request.
comm_mainfunction() processes the request.
comm_getcurrentcommode() reports current mode.
```

Current limitation:

```text
SILENT_COMMUNICATION has no real transition trigger yet because BusSM/Nm/timer flow is not implemented.
```

## Phase 15 - Clock and SysTick Draft

Started the first timer/clock direction:

- Added register clock helper layer:
  - `src/register/type/clock_type.rs`
  - `src/register/src/clock.rs`
- Moved HSI enable flow behind MCAL Mcu init:
  - `mcal::mcu::mcu_init()`
  - `mcal::mcu::mcu_get_system_clock_hz()`
- Added SysTick register block:
  - `src/register/type/systick_type.rs`
- Added low-level SysTick init draft:
  - `src/register/src/systick.rs`
  - `systick_init(core_clock_hz, tick_hz)`
- Added SysTick vector entry in startup.
- Kept SysTick inactive in `main.rs`/`mcu_init()` until the handler can dispatch to a real tick counter.

Important clock lesson:

```text
The value 16 MHz is not stored directly in an RCC register.
RCC registers store the selected clock source.
If SYSCLK source is HSI, software knows from the reference manual that HSI is 16 MHz.
```

Current SysTick limitation:

```text
SysTick register init exists, but it is not active in main.
SysTick_Handler still loops forever, so enabling SysTick interrupt now would trap the CPU in the handler.
```

## Current Status

Completed/mostly completed:

- Register layer for RCC/GPIO/SYSCFG/EXTI/NVIC
- Port MCAL
- Dio MCAL
- Basic MCU clock helper
- EXTI register functions
- EXTI callback table path
- EXTI notification APIs
- EXTI grouped interrupt dispatch
- Custom startup/vector table
- IoHwAb button/LED adapter draft
- IoIf RX indication draft for the PA0 button event
- IoIf TX confirmation draft for LED outputs
- IoIf TX group PDU draft for grouped LED outputs
- LED/button interrupt demo now uses IoIf RX/TX APIs in `main.rs`
- `Dio_WriteChannelGroup` now applies mask/offset logic and preserves bits outside the channel group
- `dio_write_port()` now writes full port values through `ODR`, matching channel-group write semantics
- `main.rs` no longer reads `get_button_count()` directly
- `ioif_read_rx_value()` now returns `IoIf_ReturnType`
- `ioif_read_rx_value()` handles RX mode selection between `INTERRUPT` and `POLLING`
- `ioif_rxindication()` rejects non-`INTERRUPT` RX PDUs
- `ioif_rxindication()` validates the RX PDU index before setting the indication table
- `ioif_txconfirmation()` validates the TX PDU index before setting the confirmation table
- `ioif_txconfirmation()` now handles both single TX PDUs and group TX PDUs
- LED group demo cases now route through `ioif_write_tx_group_state()`
- Single LED TX now supports `IoIf_OutputType::TOGGLE`
- IoHwAb button count and IoIf RX/TX state tables now use `AtomicU8`
- ComM draft exists under `src/bsw/management/comm`
- ComM is now lightly used in `main.rs` to gate the GPIO demo behind `FULL_COMMUNICATION`
- Register clock and SysTick draft files exist
- `main.rs` now initializes Mcu through `mcu_init()` instead of calling HSI enable directly
- BSW config files now live under `src/bsw/cfg`
- Register layer files are now grouped under `src/register/type`, `src/register/src`, and `src/register/cfg` while keeping the old public module paths through `src/register/mod.rs`.
- MCAL Port/Dio/Exti configuration objects now live under `src/mcal/cfg`.
- MCAL type files now live under `src/mcal/type` and MCAL driver implementation files now live under `src/mcal/src` while keeping old public module paths through `src/mcal/mod.rs`.
- EXTI callback table no longer uses `static mut`; it uses `AtomicUsize` and a getter API.
- Added global/static datatype notes in `docs/GlobalData.md`.
- Root `.gitignore` and `target/` untracking

End-of-day checkpoint:

```text
The GPIO demo flow is stable enough to continue tomorrow from the IoIf/RTE boundary.
Current main flow:
PA0 EXTI interrupt -> IoHwAb button -> IoIf RX PDU 0x100 -> main LED pattern
Normal LED writes -> IoIf TX PDU 0x200..0x203
Grouped LED writes -> IoIf TX group PDU 0x300..0x301
```

Scaffolded but not yet active in the main flow:

- `src/app`
- `src/rte`
- MCAL placeholders for ADC/CAN/GPT/PWM/SPI/UART/WDG

Next recommended work:

1. Add an MCAL SysTick wrapper and tick counter.
2. Change `SysTick_Handler` to dispatch to the MCAL SysTick handler instead of looping forever.
3. Use SysTick tick count to call `comm_mainfunction()` periodically instead of every loop.
4. Move LED pattern logic out of `main.rs` into App/RTE layer.
5. Add a simple ComM internal state table if continuing toward AUTOSAR-like state machine behavior.
