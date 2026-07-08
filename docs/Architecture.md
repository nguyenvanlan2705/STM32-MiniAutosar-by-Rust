# MiniRustAutosar Architecture

## Goal

MiniRustAutosar is a learning-oriented Rust embedded framework inspired by AUTOSAR Classic.

Main goals:

- Layered architecture
- Configuration-driven drivers
- MCAL-style APIs
- Register-level hardware access
- Notification/callback flow
- Future BSW/RTE/Application separation

## Current Architecture

```text
Application                  active for button/LED demo runnables
    |
    v
RTE                          scaffolded; not active in main.rs yet
    |
    v
BSW                          partially active
    |
    +-- IoHwAb              active for button/LED demo
    +-- IoIf                RX indication and TX confirmation draft active for GPIO demo
    +-- Management/ComM     draft requested/current mode manager
    +-- Services/Scheduler  cyclic scheduler driven by SysTick tick count
    +-- PduR / Com          future idea
    |
    v
MCAL                         active
    +-- Mcu
    +-- Port
    +-- Dio
    +-- Exti
    +-- Uart                draft polling init/write/read path
    +-- SysTick             active as 1 ms system tick source
    +-- Nvic
    |
    v
Register Layer
    +-- RCC
    +-- Clock
    +-- GPIO
    +-- SYSCFG
    +-- EXTI
    +-- NVIC
    +-- SysTick
    +-- USART
    |
    v
Startup / Vector Table
    |
    v
STM32F411 Hardware
```

Current source layout keeps stable public module names while grouping files physically:

```text
src/register/type    register data types
src/register/src     register access functions
src/register/cfg     register constants/common addresses

src/mcal/type        MCAL public data types
src/mcal/src         MCAL driver implementation
src/mcal/cfg         MCAL configuration objects

src/bsw/cfg          generated-style BSW config objects
src/bsw/management   management modules such as ComM
```

## Current Active Runtime Flow

The code path currently compiled and exercised by `main.rs` is:

```text
Custom Reset handler
    |
    v
main()
    |
    +-- initialize clock through MCAL Mcu
    +-- initialize SysTick 1 ms tick through MCAL Mcu
    +-- configure PA0 and PD12..PD15 through MCAL Port
    +-- configure PA0 -> EXTI0 through MCAL Exti
    +-- initialize scheduler runtime state
    +-- initialize IoIf and ComM through scheduler one-shot init
    +-- request APP_GPIO FULL_COMMUNICATION
    |
    v
button interrupt on PA0
    |
    v
EXTI0_IRQHandler in startup vector table
    |
    v
MCAL EXTI interrupt dispatcher
    |
    v
registered callback enters IoHwAb button module
    |
    v
IoHwAb increments BUTTON_COUNT
    |
    v
IoHwAb calls IoIf RxIndication for PDU 0x100
    |
    v
IoIf marks the RX PDU indication as active
    |
    v
1 ms scheduler runnable reads button value through IoIf RX
    |
    v
App LED runnable writes LED single/group state through IoIf TX PDU IDs
    |
    v
IoIf maps TX PDU IDs to IoHwAb LED or LED group requests
    |
    v
IoHwAb maps LED requests to MCAL Dio
    |
    v
IoIf records TX confirmation for the PDU
```

ComM now gates the GPIO demo from scheduler runnables:

```text
scheduler 10 ms runnable
    |
    +-- comm_mainfunction()

scheduler app runnables
    |
    +-- comm_getcurrentcommode(GPIO)
    |
    +-- if FULL_COMMUNICATION:
          run IoIf RX/TX LED demo logic
```

Current demo hardware mapping:

| Logical role | STM32 pin | Current layer |
|---|---:|---|
| User button | PA0 | Port + Exti + Dio mapping |
| Yellow LED | PD12 | IoIf TX -> IoHwAb LED -> Dio -> GPIO |
| Orange LED | PD13 | IoIf TX -> IoHwAb LED -> Dio -> GPIO |
| Red LED | PD14 | IoIf TX -> IoHwAb LED -> Dio -> GPIO |
| Blue LED | PD15 | IoIf TX -> IoHwAb LED -> Dio -> GPIO |
| USART2 TX | PA2 | Port AF7 -> MCAL Uart -> USART2 |
| USART2 RX | PA3 | Port AF7 -> MCAL Uart -> USART2 |

## Current Build Status

The project currently passes:

```text
cargo check --target thumbv7em-none-eabihf
```

`target/` build output is ignored through the root `.gitignore`.

## Important Design Rules

### 1. Application must not touch registers

Application should not call:

```rust
GPIOA
GPIOD
BSRR
MODER
ODR
IDR
```

Application should use logical APIs such as:

```rust
Dio_WriteChannel(Dio_ChannelType::LedRed, Dio_LevelType::HIGH);
```

Later:

```rust
IoHwAb_SetLed(LedId::Red, LedState::On);
```

Current demo routes normal LED writes through IoIf TX:

```rust
ioif_write_tx_state(0x200, IoIf_OutputType::STD_ON);
ioif_write_tx_state(0x203, IoIf_OutputType::STD_OFF);
ioif_write_tx_state(0x200, IoIf_OutputType::TOGGLE);
```

Current demo also routes grouped LED writes through IoIf TX group PDUs:

```rust
ioif_write_tx_group_state(0x300, 0b1100);
ioif_write_tx_group_state(0x301, 0b0011);
```

The button count path is also routed through IoIf RX:

```text
main.rs -> IoIf read API -> IoHwAb button state
```

IoIf TX currently has separate config structs for single-channel TX PDUs and group TX PDUs. Both use the same `ioif_txconfirmation()` entry point.

TX confirmation currently records the command result (`IOIF_E_OK` or `IOIF_E_NOT_OK`). It is not an output-state table. If IoIf needs to report whether a LED is currently ON or OFF after a toggle, that should be tracked separately.

ComM currently models communication mode management only:

```text
comm_requestcommode(user, requested_mode)
    -> requested mode table
comm_mainfunction()
    -> current mode table
comm_getcurrentcommode(network)
    -> current mode
```

ComM is now active through scheduler runnables as a mode gate.

SysTick is active as the scheduler time base:

```text
register::systick::systick_init(core_clock_hz, tick_hz)
mcal::mcu::mcu_init_systick_1ms()
mcal::mcu::mcu_get_system_tick_count()
```

`SysTick_Handler` dispatches to the MCAL Mcu tick handler and must return quickly.

Shared state guideline:

```text
Small interrupt-shared flags/counters use atomics.
Read-only configuration uses const tables.
Pointer-sized callback storage uses AtomicUsize.
```

### 2. Register layer only maps hardware

The register layer contains:

- Base addresses
- Register block structs
- Low-level read/write helper functions
- Bit manipulation helpers

It should not contain application logic.

### 3. MCAL owns driver behavior

MCAL modules implement APIs such as:

```rust
Port_Init()
Dio_WriteChannel()
Dio_ReadChannel()
Exti_Init()
```

### 4. Configuration should drive initialization

Like AUTOSAR, hardware mapping should live in config objects:

```rust
PORT_CONFIG
DIO_CHANNEL_CONFIG
EXTI_CONFIG
```

Driver code should not hardcode board-specific pins.

## Why Not Use HAL/PAC for MCAL?

The project intentionally avoids HAL/PAC for driver logic to learn how MCAL works internally.

We may still use external crates only for CPU helpers:

```rust
cortex_m::asm::nop()
cortex_m::asm::dsb()
cortex_m::asm::isb()
```

GPIO/RCC/EXTI/NVIC access is defined from the STM32F411 Reference Manual.
USART access is also being added manually for the first UART draft.

## Future Direction

```text
GPIO + EXTI demo
    |
    v
IoHwAb button/LED adapter
    |
    v
IoIf RX indication for button event
    |
    v
IoIf TX confirmation for LED output
    |
    v
ComM requested/current mode management
    |
    v
SysTick-driven cyclic scheduler
    |
    v
RTE/App runnable split
    |
    v
UART as Virtual Bus
    |
    v
PduR / Com
    |
    v
CAN later
```

UART can be used as a virtual transport to simulate PDU flow before real CAN is implemented.
