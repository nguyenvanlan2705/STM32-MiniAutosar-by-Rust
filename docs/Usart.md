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
src/mcal/src/usart.rs            MCAL USART init, polling helpers, async TX/RX state
src/bsw/usartif/usartif_tx.rs    UsartIf TX PDU wrapper and confirmation draft
src/bsw/usartif/usartif_rx.rs    UsartIf RX start/indication draft
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
bsw::usartif::usartif_rxindication(USART2, PduInfoType)
    |
    v
UsartIf saves upper RX buffer pointer and length
    |
    v
mcal::usart::usart_start_receive_async(USART2, len)
    |
    v
enable RXNEIE
    |
    v
USART2_IRQHandler -> usart_irq_handler(USART2)
    |
    v
RXNE stores bytes into USART channel RX buffer
expected length reached -> RX completed
    |
    v
MCAL calls usartif_rxindication_by_channel(USART2)
    |
    v
UsartIf copies MCAL RX buffer into saved upper buffer
```

## Current Status

- USART2 polling TX path exists.
- USART2 interrupt TX path exists.
- USART2 interrupt RX draft exists.
- MCAL USART now has generated-style config in `src/mcal/cfg/usart_cfg.rs`.
- Source/module/function naming is now aligned to `usart` instead of mixed `uart/usart`.
- UsartIf TX transmit draft exists and the 1000 ms scheduler runnable now transmits through `usartif_transmit()`.
- UsartIf TX confirmation is connected from MCAL TC interrupt through `usartif_tx_confirmation_by_channel()`.
- UsartIf RX draft saves an upper buffer before starting async RX, then copies data after MCAL RX complete.
- MCAL USART detects frame error `FE` and overrun error `ORE`, clears the hardware flags, and exposes an error state.

## Notes

- `usart_*` function names are used for Rust module/function identifiers.
- `USART_*` uppercase names are used for constants, return variants, and hardware-facing labels.
- TX confirmation now calls upward into UsartIf when TC completes.
- Direct TX helpers are still useful for small tests, but production flow should prefer async TX ownership.
- USART is a byte stream. Fixed-length RX must match the actual terminal payload, including line endings:

```text
No line ending -> 111      -> length 3
LF             -> 111\n    -> length 4
CRLF           -> 111\r\n  -> length 5
```

If the configured RX length is shorter than the bytes sent by the terminal, extra bytes can remain in `DR`; the next byte can then set `ORE`.
