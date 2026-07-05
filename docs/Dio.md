# MCAL Dio Driver

## Purpose

The Dio module reads and writes digital logic levels.

It does not configure pin mode. Pin configuration belongs to Port.

## Current Implementation Status

Implemented active APIs:

- `dio_readchannel`
- `dio_writechannel`
- `dio_flipchannel`
- `dio_readchannel_output`
- `dio_readchannelgroup`
- `dio_writechannelgroup`

Configured logical channels:

| Logical channel | Pin |
|---|---|
| `LedYellow` | PD12 |
| `LedOrange` | PD13 |
| `LedRed` | PD14 |
| `LedBlue` | PD15 |
| `UserButton` | PA0 |

The channel configuration currently lives in `src/mcal/cfg/dio_cfg.rs`.

Channel group configuration is also present in `src/mcal/cfg/dio_cfg.rs`.

Current groups:

| Logical group in IoHwAb | Port | Mask | Offset |
|---|---|---:|---:|
| `LedGroup::RedBlue` | D | `0b1100_0000_0000_0000` | 12 |
| `LedGroup::OrangeYellow` | D | `0b0011_0000_0000_0000` | 12 |

Current note:

```text
Dio_WriteChannelGroup now preserves bits outside the group mask.
It reads ODR, clears only group bits, inserts the shifted value, then writes the new port value through ODR.
```

Current channel group write flow:

```text
ioif_write_tx_group_state(0x300, value)
    |
    +-- IoIf group PDU 0x300 -> LED_GROUP_RED_BLUE
    +-- IoHwAb LedGroup::RedBlue -> Dio_ChannelGroupType
    |
    v
dio_writechannelgroup(group, value)
    |
    +-- read current output latch from GPIOx_ODR
    +-- clear only the bits selected by group.mask
    +-- shift value by group.offset
    +-- mask shifted value so it cannot leak outside the group
    +-- OR cleared value and shifted value
    +-- write final full-port value to GPIOx_ODR
```

## Main AUTOSAR-like APIs

Recommended API set:

```rust
Dio_ReadChannel()
Dio_WriteChannel()
Dio_FlipChannel()
Dio_ReadPort()
Dio_WritePort()
Dio_ReadChannelGroup()
Dio_WriteChannelGroup()
Dio_ReadOutputChannel()
Dio_ReadOutputPort()
```

## Important Types

```rust
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dio_LevelType {
    LOW = 0,
    HIGH = 1,
}
```

```rust
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dio_ChannelType {
    LedYellow,
    LedOrange,
    LedRed,
    LedBlue,
    UserButton,
}
```

Channel IDs should represent logical channels, not physical pins.

Good:

```rust
Dio_ChannelType::LedRed
```

Avoid in application:

```rust
PORT::D, PIN::P12
```

## Dio Channel Config

```rust
pub struct Dio_ChannelConfig {
    pub channel: Dio_ChannelType,
    pub port: PORT,
    pub pin: PIN,
}
```

## Steps to Use DIO

### Step 1 - Configure pin using Port_Init

DIO assumes pin mode is already configured.

### Step 2 - Read input

Read GPIOx_IDR:

```text
GPIOx_IDR bit n
```

Code flow:

```text
dio_readchannel(Dio_ChannelType::UserButton)
    |
    +-- find channel config
    +-- UserButton -> PORTA, PIN0
    +-- register::dio::dio_read(PORTA, PIN0)
    +-- read GPIOA_IDR bit 0
    +-- return HIGH or LOW
```

### Step 3 - Write output

Write GPIOx_BSRR:

```text
Set pin n:   BSRR = 1 << n
Reset pin n: BSRR = 1 << (n + 16)
```

Important:

```text
GPIOx_BSRR is write-only.
Do not read BSRR.
```

Code flow:

```text
dio_writechannel(Dio_ChannelType::LedRed, HIGH)
    |
    +-- find channel config
    +-- LedRed -> PORTD, PIN14
    +-- register::dio::dio_write(PORTD, PIN14, HIGH)
    +-- write GPIOx_BSRR bit 14
```

For LOW:

```text
dio_writechannel(Dio_ChannelType::LedRed, LOW)
    |
    +-- write GPIOx_BSRR bit 14 + 16
```

## Toggle Logic

Use ODR to know output latch state:

```text
Read ODR
If high -> reset via BSRR
If low  -> set via BSRR
```

## ReadPort vs ReadOutputPort

| API | Register |
|---|---|
| Dio_ReadPort | IDR |
| Dio_ReadOutputPort | ODR |

Use `IDR` when you want the physical pin level.

Use `ODR` when you want the output latch value.

This distinction matters because an output pin may be driven by the latch, but the physical level can still be affected by board wiring or external circuitry.

## Channel Group

For channel group APIs, use:

```rust
pub struct Dio_ChannelGroupType {
    pub port: PORT,
    pub mask: u16,
    pub offset: u8,
}
```

Use `u16` because STM32 GPIO has 16 pins.

For the STM32F4 Discovery LEDs:

```text
PD12 Yellow
PD13 Orange
PD14 Red
PD15 Blue
```

A full LED group would be:

```rust
Dio_ChannelGroupType {
    port: PORT::D,
    mask: 0xF000,
    offset: 12,
}
```

The current code experiments with smaller LED groups.

Example with current `LedGroup::RedBlue`:

```text
mask   = 0b1100_0000_0000_0000
offset = 12
value  = 0b0011
```

The value is shifted:

```text
value << offset = 0b0011_0000_0000_0000
```

Then masked:

```text
(value << offset) & mask = 0b0000_0000_0000_0000
```

So for this group, only the bits included by `mask` can change.

Important note:

```text
mask and offset must describe the same bit field.
If the mask starts at PD14 but offset is 12, low value bits may not land where you expect.
```

For future generator work, the group config should make this relationship obvious. A generated config should either:

```text
1. create masks and offsets from selected pins automatically, or
2. validate that mask + offset + group width are consistent.
```

## BSRR, ODR, and IDR

Important GPIO behavior:

```text
Writing GPIOx_BSRR bit 0..15 sets the output latch.
Writing GPIOx_BSRR bit 16..31 resets the output latch.
The output latch is reflected in ODR.
IDR reads the physical pin level.
```

So:

```text
BSRR set   -> ODR bit becomes 1
BSRR reset -> ODR bit becomes 0
```

`BSRR` is a write-only command register. It does not need to be cleared after writing.

## Common Mistakes

### 1. Using Dio before Port_Init

Dio assumes the pin is already configured. If PD12 is still input mode, writing DIO will not behave like an LED output.

Correct order:

```text
Port_Init()
then Dio_WriteChannel()
```

### 2. Reading BSRR

`BSRR` is a command register. Read `ODR` if you want output state.

### 3. Writing a group without preserving other pins

Avoid writing a raw full-port value unless that is really intended. Channel group writes should update only selected mask bits.

### 4. Mixing IDR and ODR meaning

For input buttons, read `IDR`.

For output LEDs, read `ODR` when checking the latch state.
