# BSW ComM Draft

## Purpose

ComM is the Communication Manager layer.

In this project stage, ComM does not send PDUs and does not call CanIf/UartIf yet. Its current job is to model a small AUTOSAR-like communication mode manager:

```text
User request
    |
    v
requested communication mode table
    |
    v
comm_mainfunction()
    |
    v
current communication mode table
```

The current draft also has an internal state table so ComM can evolve toward a real state machine instead of directly mapping requested mode to current mode.

## Current Source Layout

```text
src/bsw/management/comm/comm.rs       ComM behavior
src/bsw/management/comm/comm_type.rs  stable ComM types
src/bsw/cfg/comm_cfg.rs               generated-style ComM users/config
```

`ComMUser` lives in `comm_cfg.rs` because concrete users are configuration items that a future generator can create.

## Current Types

Current communication mode:

```rust
pub enum ComMMode {
    NO_COMMUNICATION,
    SILENT_COMMUNICATION,
    FULL_COMMUNICATION,
}
```

Requested communication mode:

```rust
pub enum ComMRequestedMode {
    NO_COMMUNICATION,
    FULL_COMMUNICATION,
}
```

Important distinction:

```text
Requested mode is what a user is allowed to request.
Current mode is what ComM currently reports for a network.
```

`SILENT_COMMUNICATION` is intentionally not part of `ComMRequestedMode`. It is a current/internal communication mode that can be introduced later by ComM state handling.

## Current Config

Current configured users:

```text
APP_GPIO        -> GPIO
DIAG_UART       -> UART
MANAGEMENT_CAN  -> CAN
```

The enum also contains `SPI`, so the communication mode tables are sized for all network handles:

```text
COMM_NETWORK_HANDLE_COUNT = 4
```

The config table length is different from the network handle count:

```text
network handle count = number of possible network handles
config entry count   = number of configured user-network mappings
```

## Current Flow

Request flow:

```text
comm_requestcommode(user, requested_mode)
    |
    v
find user in ComM config
    |
    v
store requested mode for that network handle
```

Main function flow:

```text
comm_mainfunction()
    |
    v
for each configured network
    |
    v
read requested mode
    |
    v
read internal state
    |
    v
transition internal state
    |
    v
map internal state to current mode
    |
    v
update internal state and current mode tables
```

Read flow:

```text
comm_getcurrentcommode(network_handle)
    |
    v
read current mode table
    |
    v
return ComMMode
```

## Why `comm_mainfunction()` Exists

AUTOSAR Classic modules commonly run state machines from periodic main functions.

For ComM:

```text
comm_requestcommode()
    receives and stores a request

comm_mainfunction()
    processes requests and updates current mode

comm_getcurrentcommode()
    reports current mode
```

This separation matters because later ComM may need to wait for lower layers such as CanSM, UartSM, Nm, or bus wakeup/sleep logic.

## Current Limitations

- `ComM_StateType` is used as a simple internal state table.
- `SILENT_COMMUNICATION` has no trigger yet because there is no BusSM/Nm/timer flow.
- ComM is called from the scheduler 10 ms runnable.
- GPIO application runnables check `comm_getcurrentcommode(GPIO)` before running DIO/IoIf logic.

## Recommended Next Step

Keep ComM minimal and refine the internal state machine only when a lower layer such as UartSM/CanSM/Nm exists:

```text
requested_mode
    |
    v
current internal state
    |
    v
comm_transition_state()
    |
    v
new internal state
    |
    v
new current mode
```

Current demo call shape:

```text
main startup:
comm_init() through scheduler_oneshot_task()
comm_requestcommode(APP_GPIO, FULL_COMMUNICATION)

scheduler 10 ms runnable:
comm_mainfunction()

GPIO app runnables:
comm_getcurrentcommode(GPIO)
```
