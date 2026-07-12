# Activation Steps Cheat Sheet

## Current Demo Summary

The current demo configures:

- PA0 as user button input with pulldown.
- PD12, PD13, PD14, and PD15 as LED outputs.
- PA0 mapped to EXTI0 with rising-edge trigger.
- EXTI0 interrupt routed through the custom vector table into the MCAL EXTI callback path.
- EXTI group handlers for EXTI9_5 and EXTI15_10 are available.
- Button notification is handled in IoHwAb and reported upward through IoIf RX indication.
- Normal LED writes are routed through IoIf TX and confirmed through IoIf TxConfirmation.
- Shared IoIf status tables use `AtomicU8`.
- The EXTI callback table uses `AtomicUsize` because callback addresses are pointer-sized.

The main loop now calls the scheduler. Scheduler runnables read the interrupt-updated button count through IoIf RX and write normal LED states through IoIf TX.

## 1. Activate GPIO Output

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

## 2. Activate GPIO Input

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

## 3. Activate DIO Logical Channel

```text
1. Configure pin using Port_Init
2. Define Dio_ChannelType
3. Map channel to port/pin in Dio config
4. Call Dio_WriteChannel or Dio_ReadChannel
```

Current code flow for LED write:

```text
ioif_write_tx_state(0x200, STD_ON)
    |
    v
IoIf TX PDU 0x200 -> LED_RED
    |
    v
IoHwAb set_led_state(LedColor::Red, On)
    |
    v
dio_writechannel(Dio_ChannelType::LedRed, HIGH)
    |
    v
GPIOx_BSRR set bit
```

Current code flow for button read after interrupt:

```text
ioif_read_rx_value(0x100, &mut count)
    |
    v
IoIf RX PDU 0x100 -> BUTTON_USER
    |
    v
If indication active, read IoHwAb BUTTON_COUNT
```

## 4. Activate EXTI Interrupt on PA0

```text
1. Configure PA0 as input using Port_Init

2. Enable SYSCFG clock
   RCC_APB2ENR.SYSCFGEN = 1

3. Select EXTI source
   SYSCFG_EXTICR1.EXTI0 = PA

4. Configure trigger
   EXTI_RTSR.TR0 = 1 for rising
   EXTI_FTSR.TR0 = 1 for falling

5. Clear pending
   EXTI_PR = 1 << 0

6. Enable interrupt mask
   EXTI_IMR.IM0 = 1

7. Enable NVIC
   NVIC_ISER for EXTI0 IRQ number 6

8. Provide vector table entry
   IRQ6 -> EXTI0_IRQHandler

9. In IRQ handler
   - clear pending
   - call MCAL EXTI handler
   - call registered callback
```

Current implementation status:

```text
Implemented for PA0 -> EXTI0.
Vector handlers exist for EXTI1..4, EXTI9_5, and EXTI15_10.
Only PA0 -> EXTI0 is configured in EXTI_CONFIG right now.
```

Current complete interrupt flow:

```text
PA0 rising edge
    |
    v
EXTI0_IRQHandler
    |
    v
MCAL exti_irq_handler(LINE0)
    |
    v
IoHwAb button_exti_notification()
    |
    v
IoIf ioif_rxindication(0x100)
    |
    v
main loop ioif_read_rx_value(0x100, &mut count)
```

## 5. Activate NVIC IRQ

```text
NVIC base = 0xE000_E100
ISER[n] enables interrupts
ICER[n] disables interrupts
```

For IRQ number:

```rust
index = irq / 32
bit   = irq % 32
```

Enable:

```text
NVIC_ISER[index] = 1 << bit
```

EXTI IRQ numbers:

```text
EXTI0      = 6
EXTI1      = 7
EXTI2      = 8
EXTI3      = 9
EXTI4      = 10
EXTI9_5    = 23
EXTI15_10  = 40
```

## 6. Activate RTT Logging

If using `rtt-target`:

```rust
use rtt_target::{rprintln, rtt_init_print};

rtt_init_print!();
rprintln!("Hello");
```

If linker error appears:

```text
undefined symbol: _critical_section_1_0_acquire
undefined symbol: _critical_section_1_0_release
```

Fix in Cargo.toml:

```toml
cortex-m = { version = "0.7", features = ["critical-section-single-core"] }
```

## 7. Simple Delay

```rust
#[inline(never)]
pub fn delay(mut count: u32) {
    while count > 0 {
        cortex_m::asm::nop();
        count -= 1;
    }
}
```

Later replace with SysTick/GPT.

## 8. Activate IoIf RX PDU

For a GPIO input event such as the user button:

```text
1. Define RX channel type
   IoIf_RxChannelType::BUTTON_USER

2. Define RX PDU config
   id         = 0x100
   peripheral = DIO
   channel    = BUTTON_USER
   mode       = INTERRUPT

3. In IoHwAb callback
   call ioif_rxindication(0x100)

4. In main loop
   call ioif_read_rx_value(0x100, &mut data)
```

Important:

```text
ioif_rxindication() should only accept INTERRUPT mode PDUs.
POLLING mode PDUs should be read directly by ioif_read_rx_value().
```

## 9. Activate IoIf TX PDU

For GPIO output such as an LED:

```text
1. Define TX channel type
   IoIf_TxChannelType::LED_RED

2. Define TX PDU config
   id         = 0x200
   peripheral = DIO
   channel    = LED_RED

3. Application/main calls
   ioif_write_tx_state(0x200, IoIf_OutputType::STD_ON)

   For toggle:
   ioif_write_tx_state(0x200, IoIf_OutputType::TOGGLE)

4. IoIf maps PDU to IoHwAb LED

5. IoHwAb maps LED to MCAL Dio channel

6. Dio writes GPIO output

7. IoIf records tx confirmation
```

Current LED PDU IDs:

| PDU ID | LED |
|---:|---|
| `0x200` | Red |
| `0x201` | Orange |
| `0x202` | Blue |
| `0x203` | Yellow |

## 10. Activate IoIf TX Group PDU

For grouped GPIO output such as LED pairs:

```text
1. Define TX group channel type
   IoIf_TxChannelGroupType::LED_GROUP_RED_BLUE

2. Define TX group PDU config
   id            = 0x300
   peripheral    = DIO
   channel_group = LED_GROUP_RED_BLUE

3. Application/main calls
   ioif_write_tx_group_state(0x300, value)

4. IoIf maps group PDU to IoHwAb LedGroup

5. IoHwAb maps LedGroup to MCAL Dio_ChannelGroupType

6. Dio writes the selected GPIO group bits

7. IoIf records tx confirmation through the shared ioif_txconfirmation()
```

Current LED group PDU IDs:

| PDU ID | LED group |
|---:|---|
| `0x300` | Red + Blue |
| `0x301` | Orange + Yellow |

## 11. Things to Check When GPIO Does Not Work

```text
1. Is the GPIO clock enabled in RCC_AHB1ENR?
2. Is the pin mode correct in MODER?
3. Is pull-up/pull-down correct for the input?
4. Are you reading IDR for input?
5. Are you writing BSRR for single pin output?
6. Are you reading ODR for output latch state?
7. For EXTI, is SYSCFG clock enabled?
8. For EXTI, is EXTICR mapping the correct port?
9. Is the trigger set in RTSR/FTSR?
10. Is EXTI_IMR unmasked?
11. Is NVIC enabled for the correct IRQ?
12. Is the vector table entry wired to the handler?
13. Is the callback registered in EXTI config?
14. Is IoIf PDU ID matching the single or group config?
15. For group writes, do the Dio mask and offset describe the intended bit field?
16. For interrupt-shared globals, is the datatype atomic or otherwise protected?
17. For callback/global pointer storage, are you using a pointer-sized type such as `usize`/`AtomicUsize`?
```

## 12. Activate ComM Minimal Flow

Current ComM is a mode manager draft. It does not start real CAN/USART communication yet.

```text
1. Define a generated-style ComM user in comm_cfg.rs
   Example: ComMUser::APP_GPIO

2. Map the user to a network handle
   APP_GPIO -> GPIO

3. Initialize ComM
   comm_init()

4. Request a mode
   comm_requestcommode(APP_GPIO, FULL_COMMUNICATION)

5. Run the periodic processor
   comm_mainfunction()

6. Read the current mode
   comm_getcurrentcommode(GPIO)
```

Important:

```text
Requested mode and current mode are separate.
SILENT_COMMUNICATION is a current/internal mode, not a normal user request.
```

## 13. Activate SysTick

Current SysTick is active as a 1 ms system tick source.

Low-level register flow:

```text
1. Get system clock from MCAL Mcu
   HSI currently means 16 MHz

2. Choose tick rate
   Example: 1000 Hz for 1 ms

3. Calculate reload
   reload = core_clock_hz / tick_hz - 1

4. Write SysTick registers
   SYST_RVR = reload
   SYST_CVR = 0
   SYST_CSR = ENABLE | TICKINT | CLKSOURCE
```

Important:

```text
SysTick_Handler must dispatch to the MCAL tick handler and return quickly.
Do not put application logic, delay loops, or logging in SysTick_Handler.
```

Current flow:

```text
SysTick_Handler
    |
    v
mcal::mcu::systick_1ms_handler()
    |
    v
SYSTEM_TICK_MS += 1
```

## 14. Activate Scheduler

The scheduler is a cooperative/cyclic scheduler driven by the SysTick tick count.

Startup flow:

```text
1. Initialize MCU clock
   mcu_init()

2. Initialize SysTick 1 ms
   mcu_init_systick_1ms()

3. Initialize Port and EXTI
   port_init()
   exti_init()

4. Initialize scheduler runtime state
   scheduler_init()

5. Run one-shot service init
   scheduler_oneshot_task()

6. Request ComM mode
   comm_requestcommode(APP_GPIO, FULL_COMMUNICATION)

7. Enter main loop
   loop {
       scheduler_mainfunction();
   }
```

Current runnable periods:

```text
1 ms    button/LED app logic when GPIO is FULL_COMMUNICATION
5 ms    USART RX command draft when GPIO is FULL_COMMUNICATION
10 ms   comm_mainfunction()
500 ms  LED toggle demo when GPIO is FULL_COMMUNICATION
1000 ms USART TX interrupt demo when GPIO is FULL_COMMUNICATION
```

Important:

```text
Scheduler decides when a runnable is due.
ComM decides whether GPIO app logic is allowed to run.
SysTick only provides time.
```

## 15. Activate USART2 Draft

Current USART draft targets USART2:

```text
PA2 -> USART2_TX -> AF7
PA3 -> USART2_RX -> AF7
```

Activation flow:

```text
1. Configure PA2 and PA3 through Port config
   mode = ALTERNATE
   alternate_function = AF7

2. Run Port init
   port_init()

3. Enable USART2 peripheral clock
   RCC_APB1ENR.USART2EN = 1

4. Configure baud rate
   USART_BRR = peripheral_clock / baudrate

5. Enable TX and RX
   USART_CR1.TE = 1
   USART_CR1.RE = 1

6. Enable USART
   USART_CR1.UE = 1

7. For polling TX
   wait until USART_SR.TXE = 1
   write byte to USART_DR

8. For polling RX
   wait until USART_SR.RXNE = 1
   read byte from USART_DR

9. For interrupt TX
   enable NVIC USART2 IRQ
   copy data into MCAL TX buffer
   enable USART_CR1.TXEIE
   write the next byte from USART2_IRQHandler

10. For interrupt RX draft
    enable NVIC USART2 IRQ
    prepare MCAL RX buffer/length/index state
    enable USART_CR1.RXNEIE
    read USART_DR from USART2_IRQHandler when RXNE is set
```

Important:

```text
Alternate mode alone is not enough.
The GPIO AFR register must also select AF7 for PA2/PA3.
```

Current MCAL TX interrupt flow:

```text
scheduler_runnable_1000ms()
    |
    v
UsartIf usartif_transmit()
    |
    v
validate length, copy bytes into static TX buffer, mark TX busy, enable TXEIE
    |
    v
USART2_IRQHandler -> usart_irq_handler()
    |
    v
write next byte when TXE is set
    |
    v
TC interrupt -> MCAL calls UsartIf TxConfirmation by channel
```

Current UsartIf/MCAL RX draft flow:

```text
scheduler_runnable_5ms()
    |
    v
UsartIf saves a static upper RX buffer pointer and length
    |
    v
mcal::usart::usart_start_receive_async()
    |
    v
prepare fixed-length RX request and enable RXNEIE
    |
    v
USART2_IRQHandler -> usart_irq_handler()
    |
    v
read byte from USART_DR when RXNE is set
    |
    v
expected length reached
    |
    v
MCAL calls UsartIf RxIndication by channel
    |
    v
UsartIf copies MCAL RX buffer into saved upper buffer
```

Current status:

```text
USART2 TX through MCAL polling path is working.
USART2 TX through MCAL interrupt path is working.
USART2 RX with scheduler/interrupt has started working better in hardware testing.
TX/RX now use a clearer start/status/read-or-complete concept.
Current scheduler USART usage now goes through the UsartIf TX/RX draft for the basic test path.
USART RX fixed-length testing must match terminal line endings. For example, `111\n` is 4 bytes and `111\r\n` is 5 bytes. If the configured length is shorter than the actual sent bytes, extra bytes can cause `ORE`.
```
