# Activation Steps Cheat Sheet

## Current Demo Summary

The current demo configures:

- PA0 as user button input with pulldown.
- PD12, PD13, PD14, and PD15 as LED outputs.
- PA0 mapped to EXTI0 with rising-edge trigger.
- EXTI0 interrupt routed through the custom vector table into the MCAL EXTI callback path.

The main loop reads the interrupt-updated `COUNT` value and writes LED patterns through Dio.

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
Not yet generalized for EXTI1..4, EXTI9_5, or EXTI15_10.
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
