# SysTick

## Purpose

SysTick is a Cortex-M core timer. It is not an STM32 peripheral timer like TIM2/TIM3, so it does not need an RCC peripheral clock enable.

Current goal:

```text
Use SysTick as the first system tick source.
Use it to drive scheduler-style periodic runnables.
```

## Current Source Layout

```text
src/register/type/systick_type.rs  SysTick register block
src/register/src/systick.rs        low-level SysTick register init
src/startup/vector_table.rs        SysTick_Handler vector entry
src/mcal/src/mcu.rs                MCAL tick counter and SysTick wrapper
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

## Current Runtime Flow

SysTick is now active as a 1 ms tick source.

Current flow:

```text
main()
    |
    v
mcu_init_systick_1ms()
    |
    v
register::systick::systick_init(system_clock_hz, 1000)
    |
    v
SysTick exception every 1 ms
    |
    v
SysTick_Handler
    |
    v
mcu::systick_1ms_handler()
    |
    v
SYSTEM_TICK_COUNT += 1
```

The scheduler reads time through:

```rust
mcu_get_system_tick_count()
```

## Important Rules

SysTick interrupt code must stay short:

```text
Good:
SysTick_Handler -> increment tick -> return

Bad:
SysTick_Handler -> loop/delay/log/application logic
```

`wrapping_sub()` should be used when comparing tick values:

```rust
let elapsed = now.wrapping_sub(last);
```

This keeps time comparison correct after the `u32` tick counter wraps around.

## Current Users

```text
Scheduler 1 ms/10 ms/100 ms/500 ms runnables use the system tick.
ComM is called from the scheduler 10 ms runnable.
```

## Design Rule

Keep layering clean:

```text
MCAL Mcu knows the current system clock.
Register SysTick only writes SysTick registers.
Startup only dispatches SysTick exception to a lower-layer handler.
```
