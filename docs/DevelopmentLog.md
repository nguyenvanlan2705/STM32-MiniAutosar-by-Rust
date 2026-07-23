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
- UsartIf
- PduR
- Com

USART can be used to simulate PDU flow:

```text
SOF | ID | LEN | DATA | CRC
```

Then UsartIf can call:

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
  - `DIAG_USART`
  - `MANAGEMENT_CAN`
  - `APP_SPI`
- Added network handles:
  - `GPIO`
  - `USART`
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

## Phase 15 - Clock and SysTick

Started the first timer/clock direction and enabled the first system tick path:

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
- `SysTick_Handler` now dispatches into the MCAL Mcu tick handler.
- Added `SYSTEM_TICK_COUNT` as an `AtomicU32` tick counter.
- Added `mcu_get_system_tick_count()` for scheduler/runtime code.
- `main.rs` now initializes SysTick through `mcu_init_systick_1ms()`.

Important clock lesson:

```text
The value 16 MHz is not stored directly in an RCC register.
RCC registers store the selected clock source.
If SYSCLK source is HSI, software knows from the reference manual that HSI is 16 MHz.
```

Important SysTick lesson:

```text
SysTick_Handler must be short.
It should increment or dispatch a tick handler, then return.
Do not place delay loops, logging, or application logic inside SysTick_Handler.
```

## Phase 16 - Cyclic Scheduler Draft

Added the first cooperative/cyclic scheduler service:

- Added `src/bsw/services/scheduler.rs`.
- Added `src/bsw/services/scheduler_type.rs`.
- Added generated-style scheduler config in `src/bsw/cfg/scheduler_cfg.rs`.
- Added runnable config table with 1 ms, 10 ms, 100 ms, and 500 ms periodic runnables.
- `main.rs` now initializes the scheduler and calls one scheduler entry point:

```rust
loop {
    scheduler_mainfunction();
}
```

Current scheduler flow:

```text
SysTick_Handler
    |
    v
mcu::systick_1ms_handler()
    |
    v
SYSTEM_TICK_COUNT += 1
    |
    v
main loop
    |
    v
scheduler_mainfunction()
    |
    v
check each runnable period using mcu_get_system_tick_count()
```

Current runnable mapping:

```text
1 ms    button app + LED pattern when GPIO network is FULL_COMMUNICATION
10 ms   comm_mainfunction()
100 ms  reserved
500 ms  LED toggle demo when GPIO network is FULL_COMMUNICATION
```

Important scheduler lesson:

```text
Scheduler decides when a runnable is called.
ComM decides whether a network is allowed to run application logic.
SysTick only provides time; it does not run application tasks directly.
```

Important runtime-state lesson:

```text
Scheduler config is const.
Scheduler last-run tick state must be static because it changes at runtime.
```

## Phase 17 - ComM Internal State Draft

Added a simple internal state table to the ComM draft:

- Added `COMM_INTERNALSTATE` as an `AtomicU8` table.
- Added `comm_get_internal_state()`.
- Added `comm_set_internal_state()`.
- Added `comm_transition_state()`.
- Added `comm_internal_state_to_current_mode()`.
- `comm_mainfunction()` now transitions internal state first, then updates current communication mode.

Current simple transition idea:

```text
NO_COM_NO_PENDING_REQUEST + FULL request
    -> NO_COM_REQUEST_PENDING

NO_COM_REQUEST_PENDING + FULL request
    -> FULL_COM_NETWORK_REQUESTED

FULL_COM_NETWORK_REQUESTED + NO request
    -> FULL_COM_READY_SLEEP

FULL_COM_READY_SLEEP + NO request
    -> NO_COM_NO_PENDING_REQUEST
```

Important ComM state lesson:

```text
Requested mode is an input.
Internal state is ComM's private state machine.
Current mode is what ComM reports to other modules.
```

## Phase 18 - USART Register/MCAL Draft

Started the first USART direction using USART2:

- Added `src/register/type/usart_type.rs`.
- Added `src/register/src/usart.rs`.
- Added `src/mcal/src/usart.rs`.
- Added `src/mcal/cfg/usart_cfg.rs`.
- Added USART register block:
  - `SR`
  - `DR`
  - `BRR`
  - `CR1`
  - `CR2`
  - `CR3`
  - `GTPR`
- Added USART instances:
  - `USART1`
  - `USART2`
  - `USART6`
- Added low-level helpers:
  - enable USART peripheral clock
  - set baud rate
  - enable TX/RX
  - enable USART
  - polling write
  - polling read
  - RXNE interrupt enable draft
- Added PA2/PA3 alternate-function config for USART2:
  - PA2: USART2_TX, AF7
  - PA3: USART2_RX, AF7
- Port config now carries an `alternate_function` field.
- Port init writes GPIO AFR when a pin is configured as alternate function.
- `scheduler_oneshot_task()` now initializes USART2 at 9600 baud.
- Added MCAL USART polling wrappers:
  - `usart_write_bytes()`
  - `usart_write_string()`
  - `usart_read_bytes()`
  - `usart_read_string()`
- Added a scheduler 1000 ms USART TX demo runnable.
- USART TX through the MCAL polling path has been hardware-tested successfully.
- Added USART2 interrupt dispatch through `USART2_IRQHandler`.
- Added NVIC enable helper usage for USART2 IRQ.
- Added MCAL USART async TX state:
  - static TX buffer
  - TX length
  - TX index
  - TX busy flag
- Added `usart_start_send_async()` to request interrupt-driven TX.
- USART TX through the MCAL interrupt path has been hardware-tested successfully.
- Added the first USART RX scheduler/interrupt draft:
  - `scheduler_runnable_5ms()` checks USART RX only when GPIO network is `FULL_COMMUNICATION`
  - MCAL USART has per-channel RX buffer, RX length, RX index, and RX busy state
  - `USART2_IRQHandler` dispatches RX/TX handling through `usart_irq_handler()`
  - RX testing with scheduler and interrupt is now behaving better than the earlier direct-read approach
- Refined the MCAL USART async concept toward explicit state-machine APIs:
  - `usart_start_send_async()`
  - `usart_get_tx_status()`
  - `usart_start_receive_async()`
  - `usart_get_rx_status()`
  - `usart_read_received_async_data()`
- Cleaned up key USART async state details:
  - TX rejects empty data and data larger than the 128-byte TX buffer before starting
  - TX done state is reset when a new TX request starts
  - RX done state is reset when a new RX request starts
  - RX completion is set immediately after the expected byte count is received

Important USART lesson:

```text
GPIO alternate mode is not enough.
The GPIO AFR register must select the peripheral function, such as AF7 for USART2 PA2/PA3.
```

Current USART limitation:

```text
USART TX interrupt is active through the scheduler 1000 ms runnable.
USART RX is drafted and has started working better through the scheduler/interrupt test path.
TX/RX now use a clearer start/status/read-or-complete concept, similar to a small asynchronous state machine.
Current scheduler USART usage is now a basic UsartIf TX/RX test on top of MCAL USART.
Baud calculation currently assumes the USART peripheral clock equals the simple system clock.
Later USART2 should use PCLK1, while USART1/USART6 should use PCLK2.
```

## Phase 19 - ADC and IoHwAb Sensor Draft

Started the first analog input direction using ADC1:

- Added ADC register layer for ADC1.
- Added MCAL ADC config under `src/mcal/cfg/adc_cfg.rs`.
- Added MCAL ADC type definitions under `src/mcal/type/adc_type.rs`.
- Added MCAL ADC implementation under `src/mcal/src/adc.rs`.
- Corrected ADC1 base address to `0x4001_2000`.
- Configured PB0 as analog input for `ADC_CHANNEL_8`.
- Added ADC channel config:
  - channel: `ADC_CHANNEL_8`
  - mode: single conversion
  - sample time: 84 cycles
  - resolution: 12-bit
  - alignment: right
  - method: polling
- Added IoHwAb Sensor draft:
  - `src/bsw/iohwab/sensor.rs`
  - `src/bsw/cfg/iohwab_sensor_cfg.rs`
  - `SensorType::LM35`
- Added IoIf RX PDU for LM35:
  - PDU ID: `0x101`
  - peripheral: `ADC`
  - channel: `SENSOR_LM35`
  - mode: `POLLING`
- Added temperature application draft:
  - `src/app/temperature_app.rs`
- Scheduler now calls `iohwab_sensor_mainfunction()` from the 10 ms runnable.
- The 1 ms runnable reads the latest sensor value through IoIf and converts raw ADC data to a temperature estimate.

Important ADC lesson:

```text
ADC channel number is not always the same as GPIO pin number.
ADC_CHANNEL_8 maps to PB0 on STM32F411, not PA8.
```

Important scheduler lesson:

```text
Do not block inside scheduler runnables waiting for ADC EOC.
Start conversion and poll completion as separate state-machine steps.
```

Current ADC sensor state flow:

```text
SENSOR_IDLE
    -> start ADC conversion
    -> SENSOR_CONVERTING

SENSOR_CONVERTING
    -> check EOC once
    -> if not complete, return and keep CONVERTING
    -> if complete, read DR and save cached value
    -> SENSOR_COMPLETE

SENSOR_COMPLETE
    -> IoIf/App reads latest cached value
    -> SENSOR_IDLE
```

Hardware validation checkpoint:

```text
PB0 connected to GND   -> raw near 0
PB0 connected to 3.3 V -> raw near 4095
```

This indicates the ADC driver, ADC1 base address, and channel/pin mapping are basically correct.

LM35 validation remains pending until the sensor module and wiring can be checked with a multimeter or a replacement module.

## Phase 20 - SPI Register/MCAL Loopback Draft

Started the first SPI direction using SPI1 and SPI2 on the same STM32F411 Discovery board:

- Added SPI register/type/config/MCAL draft files.
- Configured SPI1 as polling master.
- Configured SPI2 as polling slave.
- Added SPI1 pin config:
  - PA4: software NSS GPIO placeholder
  - PA5: SPI1_SCK AF5
  - PA6: SPI1_MISO AF5
  - PA7: SPI1_MOSI AF5
- Added SPI2 pin config:
  - PA12: software NSS GPIO placeholder
  - PB13: SPI2_SCK AF5
  - PB14: SPI2_MISO AF5
  - PB15: SPI2_MOSI AF5
- Hardware loopback wiring is:
  - PA5 -> PB13
  - PA7 -> PB15
  - PA6 <- PB14
- Added master/slave-oriented MCAL APIs:
  - `spi_master_transfer_byte()`
  - `spi_slave_preload_byte()`
  - `spi_slave_ready_to_receive()`
  - `spi_slave_receive_byte()`
- `spi_slave_ready_to_receive()` now checks RXNE instead of using BSY as a read-ready condition.
- Added `Dio_ChannelType::OnboardSpiSensorCs` mapped to PE3.
- `spi_init()` now forces PE3 HIGH before configuring SPI channels so the onboard SPI sensor does not drive SPI1 MISO.

Important SPI lesson:

SPI MISO stability depends on the whole bus, not only the selected test wires.
On STM32F411 Discovery, the onboard SPI sensor shares SPI1 pins.
After a full power cycle, PE3 must be driven HIGH to keep that sensor deselected before SPI1 loopback testing.

Current SPI status:

```text
SPI1 master polling transfer is hardware-tested.
SPI2 slave polling preload/receive is hardware-tested.
Logic analyzer can observe SPI clock/data after wiring and mode settings are corrected.
Power-cycle MISO instability is resolved by moving PE3 deselect handling into spi_init().
```

## Phase 21 - MCP2515 Bring-Up over SPI

Started MCP2515 external CAN controller bring-up on top of the current MCAL SPI byte-level helper.

Added/drafted:

- `src/mcal/external/mcp2515_type.rs`
- `src/mcal/external/mcp2515.rs`
- `src/mcal/cfg/mcp2515_cfg.rs`
- MCP2515 config maps:
  - device id: `MCP2515_1`
  - SPI channel: `SPI1`
  - CS channel: `Dio_ChannelType::Mcp2515Cs`
  - optional INT channel: `Dio_ChannelType::mcp2515Int`
  - oscillator: 8 MHz
  - CAN baudrate: 500 kbps
- Basic MCP2515 commands:
  - reset
  - read register
  - write register
  - bit modify
  - read quick status
  - set mode
  - write CNF1/CNF2/CNF3 for baudrate

Important MCP2515 validation notes:

```text
READ_STATUS instruction 0xA0 returns quick RX/TX buffer flags.
It does not return the CANSTAT register.
```

To check MCP2515 mode:

```text
Read CANCTRL register 0x0F, then mask with 0xE0.
Read CANSTAT register 0x0E, then mask with 0xE0.
```

Current init ends with:

```text
mcp2515_set_mode(..., NORMAL)
```

Therefore reading `CANCTRL = 0x07` after init is acceptable because:

```text
0x07 & 0xE0 = 0x00 -> NORMAL mode bits
```

The current SPI/MCP2515 path is still a bring-up path, not a full AUTOSAR SPI Channel/Job/Sequence implementation.
The byte-level SPI helper is intentionally kept for hardware validation before refactoring SPI toward AUTOSAR concepts.

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
- ComM internal state table draft exists
- ComM is now used by the scheduler to gate GPIO app runnables behind `FULL_COMMUNICATION`
- Register clock and SysTick files exist and SysTick is active as the 1 ms scheduler time base
- Cyclic scheduler draft exists under `src/bsw/services`
- Scheduler config table exists under `src/bsw/cfg/scheduler_cfg.rs`
- Scheduler runtime tick state is sized from the scheduler config table length
- USART register and MCAL init draft exists for USART2 polling
- USART MCAL TX polling path is working
- USART MCAL TX interrupt path is working
- USART MCAL RX scheduler/interrupt draft is connected and working in hardware testing with the current ring-buffer test flow
- USART MCAL async APIs now expose explicit TX/RX status concepts
- USART TX/RX async state cleanup is mostly complete for the current basic test level
- UsartIf TX draft exists on top of MCAL USART:
  - `usartif_transmit()` accepts `PduIdType` and `PduInfoType`
  - UsartIf TX config maps TxPduId 0 to `USART2`
  - scheduler 1000 ms runnable now transmits through UsartIf instead of directly calling MCAL TX
  - UsartIf TX confirmation table exists
- MCAL USART TC interrupt now calls UsartIf TX confirmation by channel.
- UsartIf RX draft exists on top of MCAL USART:
  - scheduler 5 ms runnable uses a static RX test buffer
  - UsartIf `StartOfReception` saves the upper RX buffer pointer/length and resets the RX write index
  - MCAL USART RX interrupt reads `DR` on `RXNE` and pushes bytes into a per-channel RX ring buffer
  - scheduler 5 ms runnable calls UsartIf RX processing so it can pop ring-buffer bytes into the saved upper buffer
  - UsartIf validates a simple delimiter-based frame on CR/LF
  - When CRC is enabled, the current test frame format is `payload + two ASCII hex CRC8 characters + CR/LF`, for example `111F1\n`
  - UsartIf removes the CRC characters from the upper buffer after successful validation and marks RX completed
  - UsartIf marks RX error for too-short frames, CRC mismatch, timeout, or buffer full before delimiter
- MCAL USART detects `FE` and `ORE`, clears hardware error flags, and exposes an error state.
- USART RX moved from the earlier fixed-length interrupt-complete model toward a ring-buffer stream model, reducing overrun risk during terminal testing.
- ADC register and MCAL draft exists for ADC1.
- ADC1 channel 8 is configured through MCAL config and PB0 is configured as analog input through Port config.
- ADC basic hardware sanity checks passed:
  - PB0 tied to GND gives raw value near 0
  - PB0 tied to 3.3 V gives raw value near full scale
- IoHwAb Sensor draft exists for LM35 over ADC channel 8.
- IoHwAb Sensor uses a non-blocking state machine:
  - `SENSOR_IDLE`
  - `SENSOR_CONVERTING`
  - `SENSOR_COMPLETE`
  - `SENSOR_ERROR`
- IoIf RX PDU `0x101` routes ADC sensor latest-value reads to IoHwAb Sensor.
- Temperature app draft reads ADC raw data through IoIf and converts it using `raw * 3.3 / 4095.0 * 100.0`.
- SPI register/MCAL polling draft exists for SPI1 master and SPI2 slave loopback testing.
- SPI loopback is stable after forcing the onboard SPI sensor CS `PE3` high from `spi_init()`.
- MCP2515 external CAN controller bring-up exists under MCAL external and currently uses SPI1 byte-level polling helpers.
- MCP2515 can be reset, configured for 500 kbps with an 8 MHz oscillator, switched to NORMAL mode, and read through SPI register commands.
- Port config supports alternate-function selection for PA2/PA3 USART2 AF7
- `main.rs` now runs the app through `scheduler_mainfunction()` instead of directly executing the LED/button demo
- `main.rs` now initializes Mcu through `mcu_init()` instead of calling HSI enable directly
- BSW config files now live under `src/bsw/cfg`
- Register layer files are now grouped under `src/register/type`, `src/register/src`, and `src/register/cfg` while keeping the old public module paths through `src/register/mod.rs`.
- MCAL Port/Dio/Exti configuration objects now live under `src/mcal/cfg`.
- MCAL type files now live under `src/mcal/type` and MCAL driver implementation files now live under `src/mcal/src` while keeping old public module paths through `src/mcal/mod.rs`.
- EXTI callback table no longer uses `static mut`; it uses `AtomicUsize` and a getter API.
- Added global/static datatype notes in `docs/GlobalData.md`.
- Root `.gitignore` and `target/` untracking

Current runtime checkpoint:

```text
Current main flow:
Reset -> main init -> scheduler_init -> scheduler_oneshot_task -> scheduler_mainfunction loop
PA0 EXTI interrupt -> IoHwAb button -> IoIf RX PDU 0x100 -> scheduler 1 ms runnable -> app LED pattern
Scheduler 10 ms runnable -> comm_mainfunction()
Scheduler 10 ms runnable -> IoHwAb Sensor mainfunction for ADC/LM35 draft
Scheduler 500 ms runnable -> LED toggle demo
Scheduler 1000 ms runnable -> UsartIf TX PDU 0 -> USART2 TX interrupt demo
Scheduler 5 ms runnable -> UsartIf RX StartOfReception + RX processing over MCAL USART RX ring buffer + CRC frame validation
Temperature app -> IoIf RX PDU 0x101 -> IoHwAb Sensor latest cached ADC value
SPI test loop -> SPI2 preload 0xAA -> SPI1 transfers 0x55 -> SPI1 receives MISO, SPI2 receives MOSI
MCP2515 init -> SPI1 byte-level commands -> MCP2515 reset/CNF/mode/read-status/register read
Normal LED writes -> IoIf TX PDU 0x200..0x203
Grouped LED writes -> IoIf TX group PDU 0x300..0x301
```

Scaffolded or drafted but not yet active in the main flow:

- `src/app`
- `src/rte`
- UsartIf queue/buffer refinement and future PduR connection
- MCAL placeholders for CAN/GPT/PWM/WDG
- SPI is now active as a direct MCAL polling loopback draft; full AUTOSAR SPI Channel/Job/Sequence support is not implemented yet.
- MCP2515 is started as an MCAL external component, but CAN frame transmit/receive APIs are not implemented yet.
- LM35 hardware validation is pending stable sensor hardware or multimeter confirmation

Next recommended work:

1. Keep direct TX writes as test-only while async TX owns production USART transmission.
2. Add MCU PCLK1/PCLK2 helpers before supporting USART1/USART6 baud rate robustly.
3. Decide whether 1 ms LED pattern logic and 500 ms LED toggle should control the same LEDs or be separated.
4. Refine UsartIf RX naming so StartOfReception, RX processing, and future RxIndication responsibilities are clearer.
5. Refine USART RX framing beyond the current delimiter + CRC8 text test, for example SOF, length field, escape handling, or future PduR routing.
6. Rename old blocking ADC helpers to make them clearly test-only, or remove them after the non-blocking ADC sensor path is stable.
7. Add an ADC ownership rule before configuring multiple ADC sensors.
8. Add a normal MCP2515 CANSTAT register read helper; do not confuse it with READ_STATUS quick status.
9. Add a small MCP2515 post-reset delay before CNF writes.
10. Keep current SPI byte helpers for bring-up, but plan the AUTOSAR-style SPI refactor around Channel, Job, and Sequence.
11. Decide the next MCP2515 milestone: loopback CAN frame transmit first, then RX buffer/interrupt handling.
