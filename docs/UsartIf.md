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
MCAL USART async RX stream
    |
    v
USART2 RXNE interrupt
    |
    v
MCAL RX ring buffer
    |
    v
UsartIf RX processing from scheduler
```

## Current Source Layout

```text
src/bsw/usartif/usartif_type.rs   UsartIf return/config/status types
src/bsw/usartif/usartif_tx.rs     UsartIf TX transmit and confirmation draft
src/bsw/usartif/usartif_rx.rs     UsartIf RX StartOfReception and RX processing draft
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
usartif_startofreception(channel, pduinfo)
    |
    +-- validate channel and PduInfoType
    +-- save upper buffer pointer and length by channel
    +-- clear the upper buffer
    +-- reset the UsartIf RX write index
    +-- set RX PDU status to PENDING/BUSY
    +-- call usart_start_receive_async(channel, len)
```

With the RX ring-buffer model, `USART_BUSY` from MCAL can be acceptable during `StartOfReception`. It means the lower USART RX interrupt stream is already enabled.

The interrupt path is:

```text
USART RXNE interrupt
    |
    v
MCAL usart_irq_handler()
    |
    +-- read USART_DR
    +-- push byte into the MCAL RX ring buffer
```

The scheduler-side processing path is:

```text
scheduler_runnable_5ms()
    |
    v
usartif_rx_processing(channel)
    |
    +-- get saved upper buffer pointer and length
    +-- pop bytes from the MCAL RX ring buffer
    +-- write bytes into the saved upper buffer using the UsartIf RX write index
    +-- when CR/LF is received, validate the frame
    +-- set RX PDU status COMPLETED when the frame is valid
    +-- set RX PDU status ERROR when CRC fails, the frame is too short, or the upper buffer is full
    +-- set RX indication OK
```

Current RX is a simple delimiter-based frame test on top of the MCAL ring buffer. When CRC is enabled in the RX PDU config, the current frame format is:

```text
payload + CRC8_HEX_HIGH + CRC8_HEX_LOW + CR/LF
```

Example:

```text
111F1\n
```

For this example:

```text
payload        = "111"
received CRC   = "F1" as ASCII hex -> 0xF1
calculated CRC = CRC8(payload), poly 0x07, init 0x00
```

The two CRC characters are removed from the saved upper buffer after a valid frame, so the scheduler/app sees only the payload bytes. This is still a learning-level frame format; it is not yet a full transport protocol with SOF, length, escape handling, or routing through PduR.

## Error Handling

```text
MCAL detects FE/ORE
    |
    v
UsartIf recovery clears saved RX state and MCAL error state
```

Overrun can happen during testing when the terminal sends more bytes than the configured RX length. For example, `111\n` is 4 bytes, not 3.

The ring buffer reduces this risk by draining `USART_DR` inside the ISR. It does not remove the need for periodic upper-layer processing; if UsartIf does not pop bytes often enough, the ring can still fill and set the software error state.

Invalid terminal frames such as empty lines, too-short CRC frames, non-hex CRC text, or wrong CRC should put the RX PDU into `USARTIF_ERROR`. The scheduler then re-arms reception through `StartOfReception`, which performs the recovery path.

## Queue Note

Current UsartIf does not queue multiple pending TX messages. If the lower USART channel is busy, `usartif_transmit()` returns `USARTIF_NOT_OK`.

For embedded/no_std, prefer a fixed-size static ring buffer over a linked list:

```text
fixed memory
no heap allocation
predictable timing
```
