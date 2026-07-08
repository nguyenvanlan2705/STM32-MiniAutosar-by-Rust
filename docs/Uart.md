# UART Draft

## Purpose

UART is the next communication building block after the GPIO/EXTI demo.

Current goal:

```text
Use USART2 as the first polling UART path.
Later build UartIf and use UART as a virtual bus for PDU experiments.
```

## Current Source Layout

```text
src/register/type/uart_type.rs  USART register block and USART instance type
src/register/src/uart.rs        low-level USART register access
src/mcal/src/uart.rs            MCAL UART init wrapper
src/mcal/cfg/port_cfg.rs        PA2/PA3 alternate function pin config
```

## USART2 Pin Mapping

Current draft uses USART2:

```text
PA2 -> USART2_TX -> AF7
PA3 -> USART2_RX -> AF7
```

Port activation requires both steps:

```text
GPIO mode = alternate function
GPIO AFR  = AF7
```

Setting only `MODE::ALTERNATE` is not enough. The AFR register tells the pin which peripheral function to connect to.

## Current Init Flow

```text
port_init()
    |
    v
configure PA2/PA3 as alternate function AF7

uart_init(USART2, baudrate)
    |
    +-- enable USART2 peripheral clock on APB1
    +-- set USART_BRR
    +-- enable transmitter and receiver
    +-- enable USART
```

## Important Registers

```text
USART_SR   status flags such as TXE and RXNE
USART_DR   transmit/receive data register
USART_BRR  baud rate register
USART_CR1  USART enable, TX enable, RX enable, interrupts
USART_CR2  stop bits and clock settings
USART_CR3  flow control and DMA settings
```

## Baud Rate Note

`USART_BRR` stores the clock divider used to generate the baud rate.

Current draft uses:

```rust
brr = (system_clock_hz + baud_rate / 2) / baud_rate
```

This is acceptable while the clock tree is simple and APB prescalers are not modeled.

Future improvement:

```text
USART2 uses PCLK1.
USART1/USART6 use PCLK2.
```

So later the MCU clock helper should expose:

```rust
mcu_get_pclk1_hz()
mcu_get_pclk2_hz()
```

## Current Runtime Status

- USART2 init is called from `scheduler_oneshot_task()`.
- Current configured baud rate is `9600`.
- PA2/PA3 are configured by `port_init()` before the scheduler one-shot init.

## Current Limitations

- UART has init but no periodic TX/RX demo yet.
- TX/RX are polling helpers only.
- RX interrupt and NVIC wiring are not implemented yet.
- There is no UartIf/PduR/Com flow yet.

Recommended first hardware test:

```text
1. Initialize Port.
2. Initialize USART2 at 115200 baud.
3. Send one byte or a short string periodically.
4. Observe it on the ST-Link virtual COM port or external UART adapter.
```
