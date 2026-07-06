# SysTick Draft

## Purpose

SysTick is a Cortex-M core timer. It is not an STM32 peripheral timer like TIM2/TIM3, so it does not need an RCC peripheral clock enable.

Current goal:

```text
Use SysTick as the first system tick source.
Later use it to drive scheduler-style periodic main functions.
```

## Current Source Layout

```text
src/register/type/systick_type.rs  SysTick register block
src/register/src/systick.rs        low-level SysTick register init
src/startup/vector_table.rs        SysTick_Handler vector entry
```

Clock helper files:

```text
src/register/type/clock_type.rs
src/register/src/clock.rs
src/mcal/src/mcu.rs
src/mcal/type/mcu_type.rs
src/mcal/cfg/mcu_cfg.rs
```

## Current Register Block

SysTick base address:

```text
0xE000_E010
```

Current register struct:

```rust
#[repr(C)]
pub struct SysTickRegister {
    pub SYST_CSR: u32,
    pub SYST_RVR: u32,
    pub SYST_CVR: u32,
    pub SYST_CALIB: u32,
}
```

Important registers:

```text
SYST_CSR    Control and Status Register
SYST_RVR    Reload Value Register
SYST_CVR    Current Value Register
SYST_CALIB  Calibration Register
```

Important `SYST_CSR` bits:

```text
bit 0 ENABLE     enable SysTick counter
bit 1 TICKINT    enable SysTick exception
bit 2 CLKSOURCE  use processor clock when set
bit 16 COUNTFLAG counter reached zero since last read
```

## Current Init Draft

Current low-level init:

```rust
pub fn systick_init(core_clock_hz: u32, tick_hz: u32)
```

For 16 MHz HSI and 1 kHz tick:

```text
reload = 16_000_000 / 1_000 - 1
       = 15_999
```

The current low-level function writes:

```text
SYST_RVR = reload
SYST_CVR = 0
SYST_CSR = ENABLE | TICKINT | CLKSOURCE
```

## Current Limitation

`SysTick_Handler` is present in the vector table, but currently loops forever.

That means:

```text
Do not call systick_init() in main yet.
```

If SysTick interrupt is enabled while the handler still loops forever, the CPU will enter the handler on the first tick and stay there.

## Recommended Next Step

Add an MCAL SysTick wrapper and a tick counter:

```text
src/mcal/src/systick.rs
```

Suggested API:

```rust
pub fn systick_init_1ms();
pub fn systick_irq_handler();
pub fn systick_get_tick_ms() -> u32;
```

Suggested flow:

```text
SysTick exception
    |
    v
SysTick_Handler in startup/vector_table.rs
    |
    v
mcal::systick::systick_irq_handler()
    |
    v
AtomicU32 tick counter += 1
```

Then `main.rs` can call:

```text
systick_init_1ms()
```

and ComM can be run periodically instead of every loop iteration:

```text
every 10 ms -> comm_mainfunction()
```

## Design Rule

Keep layering clean:

```text
MCAL Mcu knows the current system clock.
Register SysTick only writes SysTick registers.
Startup only dispatches SysTick exception to a lower-layer handler.
```
