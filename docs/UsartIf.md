# UsartIf Draft

## Purpose

UsartIf is the interface layer above MCAL USART.

Its job is not to touch USART registers directly. It maps logical PDU IDs to a configured lower USART channel and forwards data to the MCAL USART async API.

Current scope:

```text
Scheduler/App test code
    |
    v
UsartIf_Transmit(TxPduId, PduInfoType)
    |
    v
MCAL USART async TX
    |
    v
USART2 interrupt TXE/TC handling
```

RX is also drafted:

```text
Scheduler/App test code
    |
    v
UsartIf StartOfReception-style request
    |
    v
MCAL USART async RX
    |
    v
USART2 RXNE interrupt
    |
    v
UsartIf RxIndication by channel
```

## Current Source Layout

```text
src/bsw/usartif/usartif_type.rs   UsartIf return/config/status types
src/bsw/usartif/usartif_tx.rs     UsartIf TX transmit and confirmation draft
src/bsw/usartif/usartif_rx.rs     UsartIf RX start/indication draft
src/bsw/cfg/usartif_cfg.rs        generated-style UsartIf TX/RX PDU config
src/bsw/common_type.rs            PduIdType and PduInfoType
```

## TX PDU Config

Current config maps one logical TX PDU to USART2:

```text
TxPduId 0x00
    |
    v
lower_channel = USART2
tx_pdu_length = 16
confirmation_index = 0
```

This keeps the scheduler and future upper layers from needing to know the concrete USART channel.

## Transmit Flow

```text
usartif_transmit(txpduid, txpduinfo)
    |
    +-- find TxPdu config from txpduid
    +-- reject null PduInfoType pointer
    +-- reject null data pointer
    +-- reject data length larger than configured PDU length
    +-- check lower MCAL USART TX status
    +-- convert PduInfoType data pointer + length into &[u8]
    +-- call usart_start_send_async(lower_channel, data)
```

The conversion to `&[u8]` is safe only because MCAL USART copies the bytes into its internal TX buffer during the call.

## Confirmation Flow

Current `usartif_tx_confirmation_by_channel(channel)` is called by MCAL when TC completes.

Current flow:

```text
USART TC interrupt
    |
    v
MCAL usart_irq_handler()
    |
    v
UsartIf TxConfirmation
    |
    v
mark TxPdu as confirmed
```

Because USART has no PDU ID in the byte stream, UsartIf stores the active TxPduId per channel before starting MCAL async TX.

## RX Flow

```text
usartif_rxindication(channel, pduinfo)
    |
    +-- validate channel and PduInfoType
    +-- save upper buffer pointer and length by channel
    +-- set RX PDU status to PENDING/BUSY
    +-- call usart_start_receive_async(channel, len)
```

When MCAL receives the configured number of bytes:

```text
USART RX complete interrupt
    |
    v
MCAL usart_irq_handler()
    |
    v
usartif_rxindication_by_channel(channel)
    |
    +-- get saved upper buffer pointer and length
    +-- copy MCAL RX buffer into upper buffer
    +-- set RX indication OK
    +-- set RX PDU status COMPLETED
    +-- clear saved buffer state
```

Current RX is fixed-length. It does not yet parse delimiters or frames from a continuous stream.

## Error Handling

```text
MCAL detects FE/ORE
    |
    v
UsartIf recovery clears saved RX state and MCAL error state
```

Overrun can happen during testing when the terminal sends more bytes than the configured RX length. For example, `111\n` is 4 bytes, not 3.

## Queue Note

Current UsartIf does not queue multiple pending TX messages. If the lower USART channel is busy, `usartif_transmit()` returns `USARTIF_NOT_OK`.

For embedded/no_std, prefer a fixed-size static ring buffer over a linked list:

```text
fixed memory
no heap allocation
predictable timing
```
