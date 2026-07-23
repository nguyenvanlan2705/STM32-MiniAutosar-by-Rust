# SPI Draft

SPI is the next peripheral draft after USART and ADC.

The current hardware test uses two SPI instances on the same STM32F411 Discovery board:

```text
SPI1 = master
SPI2 = slave
```

## Files

```text
src/register/type/spi_type.rs  SPI register block and SPI instance type
src/register/src/spi.rs        low-level SPI register access
src/mcal/type/spi_type.rs      MCAL SPI config and enum types
src/mcal/cfg/spi_cfg.rs        generated-style SPI channel config
src/mcal/src/spi.rs            MCAL SPI init and basic polling APIs
```

## AUTOSAR Alignment Note

The current SPI implementation is intentionally still a direct byte-level MCAL bring-up driver.
It is good enough for:

```text
SPI1/SPI2 loopback testing
MCP2515 register read/write bring-up
logic analyzer validation
```

It is not yet an AUTOSAR-style SPI MCAL implementation.

AUTOSAR SPI introduces these main concepts:

```text
Channel  -> data unit/buffer configuration
Job      -> one chip-select controlled transfer using one or more channels
Sequence -> ordered group of jobs submitted to the SPI driver
```

The current project still uses instance-level APIs such as:

```rust
spi_master_transfer_byte()
spi_master_start_to_transfer()
spi_master_receive_byte()
spi_slave_start_to_preload()
spi_slave_receive_byte()
```

Those APIs are useful for learning and board bring-up, but later they should become lower-level building blocks under an AUTOSAR-like API surface:

```text
Spi_Init()
Spi_SetupEB()
Spi_SyncTransmit(sequence)
Spi_AsyncTransmit(sequence)
Spi_MainFunction_Handling()
Spi_GetJobResult()
Spi_GetSequenceResult()
```

For now, MCP2515 uses the byte-level SPI helper directly. This keeps the CAN controller bring-up simple before adding the full SPI job/sequence abstraction.

## Current Pin Mapping

SPI1 master:

```text
PA4 -> SPI1 software NSS GPIO placeholder
PA5 -> SPI1_SCK  AF5
PA6 -> SPI1_MISO AF5
PA7 -> SPI1_MOSI AF5
```

SPI2 slave:

```text
PA12 -> SPI2 software NSS GPIO placeholder
PB13 -> SPI2_SCK  AF5
PB14 -> SPI2_MISO AF5
PB15 -> SPI2_MOSI AF5
```

Current loopback wiring:

```text
SPI1 PA5 SCK   -> SPI2 PB13 SCK
SPI1 PA7 MOSI  -> SPI2 PB15 MOSI
SPI1 PA6 MISO  <- SPI2 PB14 MISO
GND            -> common ground
```

## Board-Specific SPI1 Note

On STM32F411 Discovery, the onboard SPI sensor shares the SPI1 bus pins.
Its chip-select pin is `PE3`.

If `PE3` is not kept high, the onboard sensor can drive `SPI1_MISO` and corrupt the loopback test, especially after a full power cycle.

The project therefore defines:

```rust
Dio_ChannelType::OnboardSpiSensorCs
```

and maps it to:

```text
PE3
```

`spi_init()` now sets this channel high before enabling the configured SPI channels:

```rust
dio_writechannel(Dio_ChannelType::OnboardSpiSensorCs, Dio_LevelType::HIGH);
```

This is not SPI hardware NSS. It is a board-level chip-select line used to deselect the onboard sensor.

## Current Init Flow

```text
main()
    |
    +-- port_init()
    |
    +-- spi_init()
            |
            +-- set PE3 high to deselect onboard SPI sensor
            +-- iterate SPI_CONFIG.channels
            +-- enable SPI peripheral clock
            +-- configure master/slave mode
            +-- configure baud rate prescaler
            +-- configure 8-bit/16-bit frame format
            +-- configure MSB/LSB first
            +-- configure Motorola/TI frame mode
            +-- configure software/hardware NSS mode
            +-- enable or disable SPI interrupt
            +-- enable SPI peripheral
```

## Current Test Flow

```text
SPI2 slave preload byte 0xAA
    |
    v
SPI1 master transfers byte 0x55
    |
    +-- SPI1 receives 0xAA from SPI2 MISO
    |
    +-- SPI2 receives 0x55 from SPI1 MOSI
```

Current APIs:

```rust
spi_master_transfer_byte(SPI1, data)
spi_slave_preload_byte(SPI2, data)
spi_slave_ready_to_receive(SPI2)
spi_slave_receive_byte(SPI2)
```

`spi_master_transfer_byte()` is named as a transfer because SPI is full-duplex.
Every master clock sends one bit and receives one bit at the same time.

Slave APIs are kept separate for readability during the loopback test:

```text
preload slave TX data
wait until slave RXNE is set
read slave RX data
```

## Current MCP2515 Bring-Up Flow

MCP2515 is modeled as an external MCAL component and currently uses SPI1 plus a GPIO-controlled chip select:

```text
MCP2515 CS  -> Dio_ChannelType::Mcp2515Cs
MCP2515 INT -> optional Dio_ChannelType::mcp2515Int
MCP2515 SPI -> SPINumberType::SPI1
```

Current MCP2515 init flow:

```text
mcp2515_init()
    |
    +-- set CS high
    +-- send RESET instruction
    +-- write CNF1/CNF2/CNF3 for configured baudrate
    +-- set MCP2515 mode to NORMAL
```

Because init ends in NORMAL mode, reading `CANCTRL` after `mcp2515_init()` can return a value such as `0x07`.
That is valid because:

```text
0x07 & 0xE0 = 0x00 -> NORMAL mode bits
```

To verify reset/configuration mode, read `CANCTRL` or `CANSTAT` immediately after reset and before switching to NORMAL:

```text
CANCTRL & 0xE0 should be 0x80 after reset/config mode
CANSTAT & 0xE0 should be 0x80 after reset/config mode
```

`READ_STATUS` instruction `0xA0` is not the same as reading the `CANSTAT` register.
`READ_STATUS` returns quick RX/TX buffer flags. A value of `0x00` means no quick RX/TX event is pending.
To inspect the MCP2515 operation mode, use normal register read on `CANSTAT = 0x0E` or `CANCTRL = 0x0F`.

## Important Lessons

SPI is not like UART.

UART RX can receive data whenever the remote side sends bytes.
SPI slave cannot send by itself. The master must generate SCK.

For a slave to return meaningful data on MISO, the slave transmit register must be loaded before the master clocks the frame.

Software NSS only changes internal SPI behavior. It does not create a physical chip-select frame on the wire.
If a test requires frame boundaries, use a real GPIO CS or hardware NSS.

For loopback debugging:

```text
Check CPOL/CPHA on both sides.
Check bit order on both sides.
Check that the logic analyzer uses the same SPI mode.
Check that MISO is not shared with another active device.
Do not trust the first frame after reset until the slave has been preloaded and stale RX/OVR state is cleared.
```

## Current Status

```text
SPI1 master polling transfer works in hardware testing.
SPI2 slave polling preload/receive works in hardware testing.
MOSI and MISO are visible on the logic analyzer.
Power-cycle instability was improved by deselecting the onboard SPI sensor through PE3.
MCP2515 SPI register access is in bring-up and can read CANCTRL/READ_STATUS.
```

## Next Work

```text
Keep byte-level polling helpers as bring-up/test APIs.
Add normal register-read helper for MCP2515 CANSTAT.
Add a small delay after MCP2515 reset before writing CNF registers.
Decide whether MCP2515 init should stop in CONFIGURATION mode during bring-up or switch to NORMAL for runtime.
Later refactor SPI MCAL toward AUTOSAR Channel/Job/Sequence concepts.
```
