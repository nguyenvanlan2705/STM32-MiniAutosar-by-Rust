# USART Draft

## Purpose

USART is the first serial communication peripheral in this project after the GPIO/EXTI demo.

Current goal:

```text
Use USART2 as the first USART path.
Later build UsartIf and use USART as a virtual bus for PDU experiments.
```

## Current Source Layout

```text
src/register/type/usart_type.rs  USART register block and USART instance type
src/register/src/usart.rs        low-level USART register access
src/mcal/type/usart_type.rs      MCAL USART return/status/config types
src/mcal/cfg/usart_cfg.rs        generated-style USART channel config
src/mcal/src/usart.rs            MCAL USART init, polling helpers, async TX/RX state and RX ring buffer
src/bsw/usartif/usartif_tx.rs    UsartIf TX PDU wrapper and confirmation draft
src/bsw/usartif/usartif_rx.rs    UsartIf RX StartOfReception and RX processing draft
src/bsw/cfg/usartif_cfg.rs       UsartIf generated-style TX/RX PDU config
src/startup/vector_table.rs      USART2 IRQ dispatch
```

## USART2 Pin Mapping

```text
PA2 -> USART2_TX -> AF7
PA3 -> USART2_RX -> AF7
```

Port activation requires both steps:

```text
GPIO mode = alternate function
GPIO AFR  = AF7
```

## Current Init Flow

```text
port_init()
    |
    v
configure PA2/PA3 as alternate function AF7

usart_init()
    |
    v
iterate USART_CHANNEL_CONFIG.channels
    |
    +-- enable USART peripheral clock
    +-- set USART_BRR when baud_rate != 0
    +-- enable transmitter and receiver
    +-- enable USART
    +-- configure parity when needed
    +-- enable NVIC IRQ when mode == INTERRUPT
```

## Current Runtime Flow

TX path:

```text
scheduler_runnable_1000ms()
    |
    v
bsw::usartif::usartif_transmit(TxPduId, PduInfoType)
    |
    v
find lower USART channel from UsartIf TX PDU config
    |
    v
mcal::usart::usart_start_send_async(lower_channel, data)
    |
    v
copy data into USART channel TX buffer
enable TXEIE
    |
    v
USART2_IRQHandler -> usart_irq_handler(USART2)
    |
    v
TXE feeds bytes into USART_DR
last byte disables TXEIE and enables TCIE
    |
    v
TC confirms transmission complete
    |
    v
MCAL calls usartif_tx_confirmation_by_channel()
    |
    v
UsartIf marks active TxPdu as confirmed
```

RX path:

```text
scheduler_runnable_5ms()
    |
    v
bsw::usartif::usartif_startofreception(USART2, PduInfoType)
    |
    v
UsartIf saves upper RX buffer pointer/length and resets its write index
    |
    v
mcal::usart::usart_start_receive_async(USART2, len)
    |
    v
enable RXNEIE once; USART_BUSY can mean the stream is already active
    |
    v
USART2_IRQHandler -> usart_irq_handler(USART2)
    |
    v
RXNE reads USART_DR and pushes the byte into the MCAL RX ring buffer
    |
    v
scheduler_runnable_5ms() calls usartif_rx_processing(USART2)
    |
    v
UsartIf pops bytes from the MCAL ring buffer into the saved upper buffer
    |
    v
UsartIf validates the frame on CR/LF and sets COMPLETED only when the frame is valid
```

## Current Status

- USART2 polling TX path exists.
- USART2 interrupt TX path exists.
- USART2 interrupt RX draft exists.
- MCAL USART now has generated-style config in `src/mcal/cfg/usart_cfg.rs`.
- Source/module/function naming is now aligned to `usart` instead of mixed `uart/usart`.
- UsartIf TX transmit draft exists and the 1000 ms scheduler runnable now transmits through `usartif_transmit()`.
- UsartIf TX confirmation is connected from MCAL TC interrupt through `usartif_tx_confirmation_by_channel()`.
- UsartIf RX draft saves an upper buffer, starts or reuses the MCAL RX stream, then processes bytes from the MCAL RX ring buffer.
- MCAL USART RX interrupt now reads `DR` on `RXNE` and pushes bytes into a per-channel ring buffer instead of waiting for one fixed-length interrupt transaction to complete.
- UsartIf RX now supports a simple CRC8-protected text frame when CRC is enabled in config.
- MCAL USART detects frame error `FE` and overrun error `ORE`, clears the hardware flags, and exposes an error state.

## Notes

- `usart_*` function names are used for Rust module/function identifiers.
- `USART_*` uppercase names are used for constants, return variants, and hardware-facing labels.
- TX confirmation now calls upward into UsartIf when TC completes.
- Direct TX helpers are still useful for small tests, but production flow should prefer async TX ownership.
- USART is a byte stream. The current RX test uses CR/LF as a frame delimiter. With CRC enabled, the test frame format is:

```text
payload + 2 ASCII hex CRC8 characters + CR/LF
```

Example:

```text
111F1\n
```

Here `F1` is the CRC8 of ASCII payload `111` using polynomial `0x07` and initial value `0x00`. The CRC characters are removed after validation, so the upper test buffer contains `111`.

The MCAL ring buffer reduces overrun risk because the ISR drains `DR` quickly. UsartIf still needs to process the ring regularly from the scheduler so the ring does not fill up.
