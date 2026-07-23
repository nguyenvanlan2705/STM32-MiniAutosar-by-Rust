# MCAL Port Driver

## Purpose

The Port module configures pins.

It is responsible for:

- GPIO clock enable
- Pin mode
- Output type
- Output speed
- Pull-up / pull-down
- Alternate function selection

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
| PA2 | USART2 TX | Alternate AF7 | None |
| PA3 | USART2 RX | Alternate AF7 | None |
| PB0 | ADC1_IN8 / LM35 draft | Analog | None |
| PE3 | Onboard SPI sensor CS | Output | None |
| PA4 | SPI1 software NSS placeholder | Output | None |
| PA5 | SPI1 SCK | Alternate AF5 | None |
| PA6 | SPI1 MISO | Alternate AF5 | None |
| PA7 | SPI1 MOSI | Alternate AF5 | None |
| PA12 | SPI2 software NSS placeholder | Output | None |
| PB13 | SPI2 SCK | Alternate AF5 | None |
| PB14 | SPI2 MISO | Alternate AF5 | None |
| PB15 | SPI2 MOSI | Alternate AF5 | None |

The configuration currently lives in `src/mcal/cfg/port_cfg.rs`.

`PE3` is board-specific for STM32F411 Discovery. It is kept high by `spi_init()` so the onboard SPI sensor does not drive SPI1 MISO during loopback tests.

Port configuration is read-only runtime data. Keep it as `const` configuration, not mutable global state. For broader global/static datatype rules, see `docs/GlobalData.md`.

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
            alternate_function: Dio_AlternateFunctionType::NONE,
        },
        PortPinConfig {
            port: PORT::A,
            pin: PIN::P0,
            mode: MODE::INPUT,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::LOW,
            pull: PULL::PULLDOWN,
            alternate_function: Dio_AlternateFunctionType::NONE,
        },
        PortPinConfig {
            port: PORT::A,
            pin: PIN::P2,
            mode: MODE::ALTERNATE,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::VERYHIGH,
            pull: PULL::NONE,
            alternate_function: Dio_AlternateFunctionType::AF7,
        },
        PortPinConfig {
            port: PORT::B,
            pin: PIN::P0,
            mode: MODE::ANALOG,
            output_type: OUTPUTTYPE::PUSHPULL,
            output_speed: OUTPUTSPEED::VERYHIGH,
            pull: PULL::NONE,
            alternate_function: Dio_AlternateFunctionType::NONE,
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
          +-- if mode is ALTERNATE, configure AFRL/AFRH
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
    +-- update four AFR bits when alternate function is selected
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

Example: configure PA2 as USART2_TX.

```text
RCC_AHB1ENR.GPIOAEN = 1
GPIOA_MODER[5:4]    = 10 alternate function
GPIOA_AFRL[11:8]    = 0111 AF7
```

Example: configure PB0 as ADC1_IN8 analog input.

```text
RCC_AHB1ENR.GPIOBEN = 1
GPIOB_MODER[1:0]    = 11 analog
GPIOB_PUPDR[1:0]    = 00 no pull
```

After this, MCAL ADC can sample ADC channel 8.

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

### 5. Forgetting AFR for alternate pins

For peripherals such as USART, SPI, or PWM, setting `MODE::ALTERNATE` is only half of the configuration.

You must also select the correct alternate function:

```text
PA2 USART2_TX -> AF7
PA3 USART2_RX -> AF7
```

### 6. Confusing ADC channel number with GPIO pin number

For the current ADC draft:

```text
ADC_CHANNEL_8 -> PB0
```

It is not PA8. The GPIO pin must be configured as analog for the real ADC input pin.
