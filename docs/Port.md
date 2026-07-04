# MCAL Port Driver

## Purpose

The Port module configures pins.

It is responsible for:

- GPIO clock enable
- Pin mode
- Output type
- Output speed
- Pull-up / pull-down
- Future alternate function

It must not read or write pin levels. That is Dio's responsibility.

## Current Implementation Status

The current Port configuration initializes:

| Pin | Role | Mode | Pull |
|---|---|---|---|
| PD12 | Yellow LED | Output | None |
| PD13 | Orange LED | Output | None |
| PD14 | Red LED | Output | None |
| PD15 | Blue LED | Output | None |
| PA0 | User button | Input | Pulldown |

The configuration currently lives inside `src/mcal/port.rs`. A future cleanup should move it to `src/config/port_cfg.rs`.

## AUTOSAR-like Rule

```text
Port = configuration
Dio  = read/write
```

## Steps to Activate GPIO Output

Example: PD12 LED output.

```text
1. Enable GPIOD clock
   RCC_AHB1ENR.GPIODEN = 1

2. Configure PD12 mode
   GPIOD_MODER.MODER12 = 01 output

3. Configure output type
   GPIOD_OTYPER.OT12 = 0 push-pull

4. Configure output speed
   GPIOD_OSPEEDR.OSPEEDR12 = desired speed

5. Configure pull
   GPIOD_PUPDR.PUPDR12 = 00 no pull

6. Write output high
   GPIOD_BSRR = 1 << 12

7. Write output low
   GPIOD_BSRR = 1 << (12 + 16)
```

## Steps to Activate GPIO Input

Example: PA0 user button.

```text
1. Enable GPIOA clock
   RCC_AHB1ENR.GPIOAEN = 1

2. Configure PA0 mode
   GPIOA_MODER.MODER0 = 00 input

3. Configure pull
   GPIOA_PUPDR.PUPDR0 = pull-up or pull-down depending on board

4. Read input
   GPIOA_IDR bit 0
```

For STM32F4 Discovery user button:

```text
Not pressed -> usually LOW
Pressed     -> usually HIGH
```

## Port Config Example

```rust
pub const PORT_CONFIG: PortConfig = PortConfig {
    pins: &[
        PortPinConfig {
            port: PORT::D,
            pin: PIN::P12,
            mode: MODE::OUTPUT,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::HIGH,
            pull: PULL::NONE,
        },
        PortPinConfig {
            port: PORT::A,
            pin: PIN::P0,
            mode: MODE::INPUT,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::LOW,
            pull: PULL::PULLDOWN,
        },
    ],
};
```

## Port_Init Flow

```text
Port_Init()
    |
    +-- for each pin config
          |
          +-- enable GPIO clock
          +-- configure MODER
          +-- configure OTYPER
          +-- configure OSPEEDR
          +-- configure PUPDR
```

## Important Lessons

### `#[repr(C)]`

All register structs must use:

```rust
#[repr(C)]
```

Otherwise Rust may reorder fields.

### Volatile access

Use:

```rust
core::ptr::read_volatile()
core::ptr::write_volatile()
```

for hardware registers.
