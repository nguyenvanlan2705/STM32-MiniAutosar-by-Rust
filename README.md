# STM32-MiniAutosar-by-Rust

A learning-oriented mini AUTOSAR-style embedded framework written in Rust for STM32F411.

## Current Status

The active demo currently covers:

- Custom `no_std` / `no_main` startup and vector table.
- Register-level RCC, GPIO, SYSCFG, EXTI, and NVIC access.
- MCAL-style `Mcu`, `Port`, `Dio`, and `Exti` modules.
- Register-level clock and SysTick draft.
- Register files are grouped into `src/register/type`, `src/register/src`, and `src/register/cfg`.
- MCAL files are grouped into `src/mcal/type`, `src/mcal/src`, and `src/mcal/cfg`.
- PA0 user button mapped to EXTI0.
- EXTI0 interrupt dispatched through the custom vector table into the MCAL callback path.
- EXTI1..4 and grouped EXTI9_5 / EXTI15_10 interrupt dispatch scaffolding.
- IoHwAb button notification state.
- IoHwAb LED APIs for PD12..PD15.
- IoIf RX indication draft for the PA0 button event.
- IoIf TX write/toggle/confirmation draft for LED output PD12..PD15.
- IoIf TX group PDU draft for grouped LED writes.
- ComM draft for requested/current communication mode management.
- Interrupt/shared state currently uses atomics where practical:
  - `AtomicU8` for small status/count tables.
  - `AtomicUsize` for stored callback addresses in the EXTI callback table.
- `main.rs` now reads the button-driven value through IoIf RX and writes LED single/group states through IoIf TX.
- `target/` build output ignored through `.gitignore`.

The current check command passes:

```text
cargo check --target thumbv7em-none-eabihf
```

Warnings are currently expected for AUTOSAR-style names and future APIs that are scaffolded but not used yet.

Current GPIO event flow:

```text
PA0 EXTI0 interrupt
-> MCAL EXTI dispatcher
-> IoHwAb button_exti_notification()
-> IoIf ioif_rxindication(0x100)
-> IoIf indication table marks the RX PDU active
-> main.rs reads the value through ioif_read_rx_value()
-> main.rs writes LED output through ioif_write_tx_state()
-> main.rs toggles LED output through IoIf_OutputType::TOGGLE
-> main.rs writes grouped LED output through ioif_write_tx_group_state()
-> IoIf TxConfirmation records the result
```

Current ComM flow:

```text
main.rs requests APP_GPIO FULL_COMMUNICATION once
-> comm_mainfunction() processes the request in the loop
-> LED/button demo runs when GPIO current mode is FULL_COMMUNICATION
```

## Documentation

- `docs/Architecture.md`: current architecture and active runtime flow.
- `docs/DevelopmentLog.md`: phase-by-phase progress and next recommended work.
- `docs/Startup.md`: custom startup/vector table notes.
- `docs/Port.md`: Port MCAL status.
- `docs/Dio.md`: Dio MCAL status.
- `docs/Exti.md`: EXTI MCAL status and current PA0 -> EXTI0 flow.
- `docs/ActivationSteps.md`: GPIO, DIO, EXTI, NVIC, RTT, and delay checklist.
- `docs/GlobalData.md`: notes for choosing data types for global/static variables.
- `docs/ComM.md`: current ComM draft, requested/current mode flow, and limitations.
- `docs/SysTick.md`: current SysTick register draft and next tick-counter step.
