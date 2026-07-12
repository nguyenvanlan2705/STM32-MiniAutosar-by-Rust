# Cyclic Scheduler

## Purpose

The scheduler is the first simple time-based service in this project.

It is not a preemptive OS scheduler. It is a cooperative/cyclic scheduler:

```text
main loop
    |
    v
scheduler_mainfunction()
    |
    v
check tick elapsed time
    |
    v
call due runnable functions
```

Tasks are not interrupted and context-switched by this scheduler. A runnable must finish and return before the next runnable can run.

## Current Source Layout

```text
src/bsw/services/scheduler.rs       scheduler runtime and runnable functions
src/bsw/services/scheduler_type.rs  scheduler types
src/bsw/cfg/scheduler_cfg.rs        generated-style runnable config table
```

## Current Runtime Flow

```text
SysTick_Handler
    |
    v
mcu::systick_1ms_handler()
    |
    v
SYSTEM_TICK_COUNT += 1
    |
    v
main loop
    |
    v
scheduler_mainfunction()
    |
    v
for each configured runnable:
    now = mcu_get_system_tick_count()
    elapsed = now.wrapping_sub(last_run_tick[index])
    if elapsed >= period_ms:
        call runnable
        last_run_tick[index] = now
```

Each runnable has its own last-run tick entry. This is important because a 1 ms runnable must not reset the timing of a 10 ms, 100 ms, or 500 ms runnable.

## Current Runnable Mapping

```text
1 ms    button app + LED pattern when GPIO is FULL_COMMUNICATION
5 ms    USART RX command draft when GPIO is FULL_COMMUNICATION
10 ms   comm_mainfunction()
500 ms  LED toggle demo when GPIO is FULL_COMMUNICATION
1000 ms USART TX interrupt demo when GPIO is FULL_COMMUNICATION
```

## ComM Gating

The scheduler decides when a runnable is due.

ComM decides whether GPIO application logic is allowed to run:

```text
scheduler_runnable_10ms()
    |
    v
comm_mainfunction()

scheduler_runnable_1ms() / scheduler_runnable_500ms()
    |
    v
if comm_getcurrentcommode(GPIO) == FULL_COMMUNICATION:
    run GPIO app logic
else:
    skip GPIO app logic
```

Do not gate `comm_mainfunction()` behind `FULL_COMMUNICATION`. ComM needs to run so it can move from requested mode to current mode.

The USART TX demo follows the same gating idea:

```text
scheduler_runnable_1000ms()
    |
    v
if GPIO network is FULL_COMMUNICATION:
    build PduInfoType
    call usartif_transmit(TxPduId 0)
    UsartIf maps TxPduId to USART2
    MCAL USART owns async TX interrupt path
```

This keeps scheduler code above the MCAL USART channel details. MCAL TC interrupt now routes confirmation back into UsartIf by channel.

The USART RX draft is currently checked from the 5 ms runnable:

```text
scheduler_runnable_5ms()
    |
    v
if GPIO network is FULL_COMMUNICATION:
    check UsartIf RX PDU status
    if COMPLETED:
        inspect static RX test buffer
        optionally transmit a response through UsartIf TX
        mark RX as not started so the next request can be armed
    if not started:
        pass static RX buffer through PduInfoType
        call UsartIf RX start/indication draft API
```

Keep this runnable non-blocking. A blocking USART read inside a cyclic scheduler would hold the main loop and delay every other runnable.

The RX test buffer must be static because MCAL completes RX later in an interrupt. A local stack buffer inside the 5 ms runnable would no longer be valid after the function returns.

For fixed-length RX testing, the scheduler/config length must match the terminal payload exactly:

```text
111      -> length 3
111\n    -> length 4
111\r\n  -> length 5
```

If the terminal sends extra bytes after the configured RX length, those bytes can remain unread and later cause `ORE`.

## Toggle Note

`IoIf_OutputType::TOGGLE` is a command, not a stable state.

It is safe in a 500 ms runnable:

```text
every 500 ms -> toggle LED once
```

It is not safe to call repeatedly in a fast loop unless the intended behavior is rapid blinking.

Also avoid having two runnables control the same LED at the same time. For example, a 500 ms toggle can be overwritten by a 1 ms LED pattern runnable if both write the same LED PDU IDs.

## Runtime State Note

The runtime tick array is sized from the scheduler config table length:

```rust
const TASK_SIZE: usize = SCHEDULER_TASKS_TABLE.tasks.len();
```

This is important because the last-run tick table is runtime state and must be `static`, not `const`.

```text
Config table       -> const
Runtime tick state -> static
```

## Current Limitations

One-shot initialization is currently called explicitly:

```rust
scheduler_oneshot_task();
```

Later it can either stay as a startup API or become a real one-shot runnable with a completed flag.
