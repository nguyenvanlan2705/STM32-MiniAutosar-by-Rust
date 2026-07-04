# STM32-MiniAutosar-by-Rust

A learning-oriented mini AUTOSAR-style embedded framework written in Rust for STM32F411.

## Current Status

The active demo currently covers:

- Custom `no_std` / `no_main` startup and vector table.
- Register-level RCC, GPIO, SYSCFG, EXTI, and NVIC access.
- MCAL-style `Mcu`, `Port`, `Dio`, and `Exti` modules.
- PA0 user button mapped to EXTI0.
- EXTI0 interrupt dispatched through the custom vector table into the MCAL callback path.
- PD12..PD15 LEDs driven through Dio logical channels.

The current check command passes:

```text
cargo check --target thumbv7em-none-eabihf
```

Warnings are currently expected for AUTOSAR-style names and future APIs that are scaffolded but not used yet.

## Documentation

- `docs/Architecture.md`: current architecture and active runtime flow.
- `docs/DevelopmentLog.md`: phase-by-phase progress and next recommended work.
- `docs/Startup.md`: custom startup/vector table notes.
- `docs/Port.md`: Port MCAL status.
- `docs/Dio.md`: Dio MCAL status.
- `docs/Exti.md`: EXTI MCAL status and current PA0 -> EXTI0 flow.
- `docs/ActivationSteps.md`: GPIO, DIO, EXTI, NVIC, RTT, and delay checklist.
