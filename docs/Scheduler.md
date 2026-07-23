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
1 ms    button app + LED pattern + temperature app latest-value read
5 ms    USART RX command draft when GPIO is FULL_COMMUNICATION
10 ms   comm_mainfunction() + IoHwAb Sensor mainfunction
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
    if RX is not started:
        pass static RX buffer through PduInfoType
        call UsartIf StartOfReception
        mark RX as started when OK
    if RX is started:
        if RX data is available:
            call UsartIf RX processing so it can pop bytes from the MCAL ring buffer
        call UsartIf RX timeout processing
    check UsartIf RX PDU status
    if COMPLETED:
        inspect static RX test buffer payload
        optionally transmit a response through UsartIf TX
        mark RX as not started so the next request can be armed
    if ERROR:
        mark RX as not started so a new request can recover/re-arm
```

Keep this runnable non-blocking. A blocking USART read inside a cyclic scheduler would hold the main loop and delay every other runnable.

The RX test buffer must be static because UsartIf keeps its pointer after `scheduler_runnable_5ms()` returns. A local stack buffer inside the 5 ms runnable would no longer be valid after the function returns.

The current RX test is delimiter and CRC based:

```text
USART ISR         -> read DR and push byte into MCAL ring buffer
Scheduler 5 ms    -> call UsartIf RX processing
UsartIf RX        -> pop ring bytes into the saved upper buffer until CR/LF
CRC enabled       -> validate payload + ASCII hex CRC
Completion        -> valid CRC frame
Error             -> too short, CRC mismatch, timeout, or buffer full before delimiter
```

The current terminal test frame for payload `111` is:

```text
111F1\n
```

After UsartIf validates CRC, it removes the last two CRC characters from the upper buffer. The scheduler test then compares the payload bytes, for example `rx_data[0..3] == b"111"`.

The MCAL ring buffer reduces `ORE` risk because the ISR drains `DR` quickly. The scheduler still needs to call RX processing regularly; otherwise the ring can fill.

## ADC Sensor Runnable

The ADC sensor draft is handled through IoHwAb Sensor from the 10 ms runnable:

```text
scheduler_runnable_10ms()
    |
    +-- comm_mainfunction()
    |
    +-- iohwab_sensor_mainfunction()
```

The sensor mainfunction is a small non-blocking state machine:

```text
SENSOR_IDLE
    -> start ADC conversion
    -> SENSOR_CONVERTING

SENSOR_CONVERTING
    -> check ADC EOC once
    -> if not complete, return immediately
    -> if complete, read ADC_DR and cache the value
    -> SENSOR_COMPLETE

SENSOR_COMPLETE
    -> wait for IoIf/App to read latest value
```

The 1 ms runnable reads the latest cached value through the application path:

```text
scheduler_runnable_1ms()
    |
    v
temperature_measurement_app_1ms()
    |
    v
ioif_read_rx_value(0x101, &mut raw)
    |
    v
IoHwAb Sensor latest cached ADC value
```

Important:

```text
ADC conversion wait loops do not belong inside scheduler runnables.
The scheduler is cooperative, so a blocking ADC wait delays all other runnables.
```

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
