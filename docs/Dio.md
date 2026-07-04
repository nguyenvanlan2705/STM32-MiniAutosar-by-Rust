# MCAL Dio Driver

## Purpose

The Dio module reads and writes digital logic levels.

It does not configure pin mode. Pin configuration belongs to Port.

## Current Implementation Status

Implemented active APIs:

- `dio_readchannel`
- `dio_writechannel`
- `dio_flipchannel`

Configured logical channels:

| Logical channel | Pin |
|---|---|
| `LedYellow` | PD12 |
| `LedOrange` | PD13 |
| `LedRed` | PD14 |
| `LedBlue` | PD15 |
| `UserButton` | PA0 |

The current LED demo writes only through these Dio logical channels. The channel configuration currently lives inside `src/mcal/dio.rs`; a future cleanup should move it to `src/config/dio_cfg.rs`.

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

## Channel Group

For channel group APIs, use:

```rust
pub struct Dio_ChannelGroupType {
    pub port: Dio_PortType,
    pub mask: u16,
    pub offset: u8,
}
```

Use `u16` because STM32 GPIO has 16 pins.
