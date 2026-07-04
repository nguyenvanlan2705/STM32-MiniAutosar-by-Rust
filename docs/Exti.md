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
- The EXTI0 callback currently increments `COUNT`, which the main loop uses to choose LED patterns.

Not implemented yet:

- Separate vector handlers for EXTI1, EXTI2, EXTI3, and EXTI4.
- Shared dispatch for EXTI9_5 and EXTI15_10.
- Bounds checking for callback registration.
- A safer replacement for `static mut` interrupt-shared state.

## EXTI Activation Steps

To make a GPIO pin generate interrupt:

### Step 1 - Configure GPIO pin using Port

Example PA0 input.

### Step 2 - Enable SYSCFG clock

```text
RCC_APB2ENR.SYSCFGEN = 1
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

### Step 4 - Configure trigger

EXTI_RTSR for rising.

EXTI_FTSR for falling.

### Step 5 - Enable line interrupt mask

```text
EXTI_IMR |= 1 << line
```

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

## Callback Table

```rust
type ExtiCallback = fn();

static mut EXTI_CALLBACKS: [Option<ExtiCallback>; 16] = [None; 16];
```

Register callback:

```rust
pub fn exti_register_callback(line: EXTILINE, cb: ExtiCallback) {
    unsafe {
        EXTI_CALLBACKS[line as usize] = Some(cb);
    }
}
```

Current code uses:

```text
register_exti_callback(LINE0, button_callback)
```

The callback is registered by `exti_init()`.

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
button_callback()
    |
    v
COUNT += 1
```
