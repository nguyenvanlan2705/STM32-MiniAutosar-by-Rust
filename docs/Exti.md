# MCAL EXTI Driver

## Purpose

EXTI handles external interrupt lines.

Important distinction:

```text
DIO reads/writes GPIO level.
EXTI handles interrupt line events.
```

DIO should not know about interrupts.

## Current Implementation Status

Implemented and active:

- PA0 is mapped to EXTI line 0.
- EXTI0 uses rising-edge trigger.
- SYSCFG clock enable is implemented.
- EXTI line mask enable is implemented.
- NVIC enable for EXTI0 is implemented.
- A global callback table exists for 16 EXTI lines.
- `EXTI0_IRQHandler()` is installed in the custom vector table.
- `EXTI1_IRQHandler()` through `EXTI4_IRQHandler()` are installed.
- `EXTI9_5_IRQHandler()` dispatches pending lines 5..9.
- `EXTI15_10_IRQHandler()` dispatches pending lines 10..15.
- EXTI pending status can be checked with `is_exti_pending`.
- EXTI notification APIs exist:
  - `exti_enable_notification`
  - `exti_disable_notification`
- The EXTI0 callback currently enters IoHwAb button logic, increments `BUTTON_COUNT`, then reports PDU `0x100` through IoIf RX indication.
- The EXTI callback table currently uses `[AtomicUsize; 16]` instead of `static mut`.
- EXTI config lives in `src/mcal/cfg/exti_cfg.rs`.

Not implemented yet:

- Bounds checking for callback registration.
- Debounce handling for the PA0 button interrupt.

## EXTI Activation Steps

To make a GPIO pin generate interrupt:

### Step 1 - Configure GPIO pin using Port

Example PA0 input.

### Step 2 - Enable SYSCFG clock

```text
RCC_APB2ENR.SYSCFGEN = 1
```

Why:

```text
SYSCFG owns the EXTI source selection registers.
Without SYSCFG clock, PA0/PB0/PC0/... cannot be mapped to EXTI0 reliably.
```

### Step 3 - Map GPIO port to EXTI line

Example:

```text
PA0 -> EXTI0
```

EXTICR layout:

```text
EXTICR1: EXTI0  EXTI1  EXTI2  EXTI3
EXTICR2: EXTI4  EXTI5  EXTI6  EXTI7
EXTICR3: EXTI8  EXTI9  EXTI10 EXTI11
EXTICR4: EXTI12 EXTI13 EXTI14 EXTI15
```

Recommended enum:

```rust
#[repr(u8)]
pub enum EXTILINE {
    LINE0 = 0,
    LINE1 = 1,
    ...
    LINE15 = 15,
}
```

Calculate:

```rust
let index = (line as usize) / 4;
let shift = ((line as usize) % 4) * 4;
```

For PA0:

```text
line  = 0
index = 0 / 4 = 0        -> EXTICR1
shift = (0 % 4) * 4 = 0
value = PORTA = 0

SYSCFG_EXTICR1[3:0] = 0000
```

For PB0, the same EXTI0 line would use:

```text
SYSCFG_EXTICR1[3:0] = 0001
```

Only one port can own one EXTI line at a time.

### Step 4 - Configure trigger

EXTI_RTSR for rising.

EXTI_FTSR for falling.

For the current PA0 button:

```text
EXTI_RTSR.TR0 = 1
EXTI_FTSR.TR0 = 0
```

Meaning:

```text
LOW -> HIGH causes interrupt
HIGH -> LOW does not
```

### Step 5 - Enable line interrupt mask

```text
EXTI_IMR |= 1 << line
```

If the IMR bit is 0, the pending bit may exist but the interrupt is masked.

### Step 6 - Clear pending before enabling NVIC

```text
EXTI_PR = 1 << line
```

Clear pending by writing 1.

Current note:

```text
The IRQ handler clears pending before calling the registered callback.
Clearing pending during initialization is still a recommended cleanup step.
```

Current `exti_enable_notification()` clears pending before enabling the line and NVIC IRQ.

### Step 7 - Enable NVIC IRQ

| EXTI Line | IRQ |
|---|---|
| LINE0 | EXTI0 |
| LINE1 | EXTI1 |
| LINE2 | EXTI2 |
| LINE3 | EXTI3 |
| LINE4 | EXTI4 |
| LINE5..LINE9 | EXTI9_5 |
| LINE10..LINE15 | EXTI15_10 |

### Step 8 - ISR clears pending and calls callback

```text
IRQ vector
    |
    v
EXTI0_IRQHandler()
    |
    v
mcal::interrupcallback::exti_irq_handler(LINE0)
    |
    +-- clear pending
    +-- call callback
```

For grouped lines:

```text
EXTI9_5_IRQHandler()
    |
    v
exti_group_irq_handler([LINE5..LINE9])
    |
    +-- check pending bit
    +-- dispatch pending line to exti_irq_handler(line)
```

## Code Flow for Current Button Interrupt

```text
PA0 rising edge
    |
    +-- EXTI_PR.PR0 becomes pending
    +-- NVIC sees EXTI0 IRQ enabled
    |
    v
EXTI0_IRQHandler()
    |
    v
exti_irq_handler(LINE0)
    |
    +-- clear_exti_pending(LINE0)
    +-- read EXTI_CALLBACK[0]
    +-- call button_exti_notification()
    |
    v
button_exti_notification()
    |
    +-- BUTTON_COUNT += 1
    +-- ioif_rxindication(0x100)
    |
    v
IoIf marks RX PDU 0x100 active
```

## Callback Table

```rust
type ExtiCallback = fn();

static EXTI_CALLBACK: [AtomicUsize; 16] =
    [const { AtomicUsize::new(0) }; 16];
```

Register callback:

```rust
pub fn register_exti_callback(line: EXTILINE, callback: fn()) {
    EXTI_CALLBACK[line as usize].store(callback as usize, Ordering::Release);
}
```

Current code uses:

```text
register_exti_callback(LINE0, IoHwAb button callback)
```

The callback is registered by `exti_init()`.

Read callback:

```rust
pub fn register_get_exti_callback(line: EXTILINE) -> Option<fn()> {
    let callback = EXTI_CALLBACK[line as usize].load(Ordering::Acquire);
    if callback == 0 {
        None
    } else {
        Some(unsafe { core::mem::transmute(callback) })
    }
}
```

Why `AtomicUsize`:

```text
A function pointer is pointer-sized.
usize is the Rust integer type intended to hold pointer-sized values.
AtomicUsize lets interrupt code read the callback address without static mut.
```

Remaining improvement:

```text
Add line/index bounds checks before indexing EXTI_CALLBACK.
```

## Important Design Rule

Startup should not know application callbacks.

Bad:

```text
Vector Table -> Application callback
```

Good:

```text
Vector Table -> IRQHandler -> MCAL EXTI -> callback
```

## Current Demo Flow

```text
PA0 rising edge
    |
    v
EXTI0 pending bit
    |
    v
NVIC EXTI0 IRQ
    |
    v
EXTI0_IRQHandler()
    |
    v
exti_irq_handler(LINE0)
    |
    +-- clear EXTI pending bit
    +-- call EXTI_CALLBACK[0]
    |
    v
button_exti_notification()
    |
    v
IoHwAb BUTTON_COUNT += 1
    |
    v
ioif_rxindication(0x100)
    |
    v
IOIF_INDICATION_TABLE[0] = 1
```

## Common Mistakes

### 1. Forgetting SYSCFG clock

The GPIO pin can be configured correctly, but EXTI source mapping still will not work if SYSCFG clock is off.

### 2. Clearing pending incorrectly

EXTI pending bits are cleared by writing 1, not writing 0.

```text
Correct: EXTI_PR = 1 << line
```

### 3. Calling application logic directly from vector table

Keep the vector table thin:

```text
Vector Table -> MCAL IRQ handler -> registered callback
```

### 4. Doing long work in interrupt context

The current callback is simple for learning. Later, debounce and heavier application logic should move to a periodic task/main loop.

### 5. Sharing EXTI grouped IRQs

Lines 5..9 share one IRQ and lines 10..15 share one IRQ.

The grouped handler must check which pending bit is active before dispatching.
