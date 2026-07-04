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

More detailed register flow for one pin:

```text
PortPinConfig
    |
    +-- port + pin select GPIO register block
    +-- enable clock in RCC_AHB1ENR
    +-- update two MODER bits for the pin
    +-- update one OTYPER bit for the pin
    +-- update two OSPEEDR bits for the pin
    +-- update two PUPDR bits for the pin
```

Example: configure PD12 as LED output.

```text
Pin number = 12

MODER shift   = 12 * 2 = 24
OSPEEDR shift = 12 * 2 = 24
PUPDR shift   = 12 * 2 = 24
OTYPER shift  = 12
```

Registers touched:

```text
RCC_AHB1ENR.GPIODEN = 1
GPIOD_MODER[25:24]  = 01 output
GPIOD_OTYPER[12]    = 0 push-pull
GPIOD_OSPEEDR[25:24]= selected speed
GPIOD_PUPDR[25:24]  = 00 no pull
```

Example: configure PA0 as button input.

```text
RCC_AHB1ENR.GPIOAEN = 1
GPIOA_MODER[1:0]    = 00 input
GPIOA_PUPDR[1:0]    = 10 pulldown
```

After this, Dio can read:

```text
GPIOA_IDR bit 0
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

### Read-modify-write

Most GPIO config registers contain multiple pins in the same register.

When configuring one pin, do not overwrite the whole register blindly.

Correct idea:

```text
1. read current register value
2. clear only the target pin field
3. OR in the new field value
4. write the updated value back
```

Example for a two-bit field:

```text
clear_mask = !(0b11 << shift)
new_value  = (old_value & clear_mask) | (mode << shift)
```

## Common Mistakes

### 1. Forgetting the GPIO clock

If `RCC_AHB1ENR.GPIOxEN` is not enabled, GPIO register writes may not take effect.

### 2. Configuring output in Dio

Dio should not configure pin mode. Keep mode setup in Port.

### 3. Wrong pull direction for button

The user button is expected as:

```text
Not pressed -> LOW
Pressed     -> HIGH
```

So PA0 currently uses pulldown.

### 4. Reusing output settings for input pins

For input pins, `OTYPER` and `OSPEEDR` are usually not meaningful for the signal read path, but the config struct may still carry values for consistency.
