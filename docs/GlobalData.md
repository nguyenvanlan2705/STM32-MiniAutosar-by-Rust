# Global and Static Data Notes

## Purpose

This project uses global/static variables for embedded-style shared state:

- interrupt callback tables
- indication flags
- confirmation flags
- small counters
- generated or static configuration objects

Because some of these variables are accessed from both normal code and interrupt context, the data type matters.

## Current Global State

| Global data | Current type | Why |
|---|---|---|
| `BUTTON_COUNT` | `AtomicU8` | Small button counter shared between EXTI callback and main loop |
| `IOIF_INDICATION_TABLE` | `[AtomicU8; IOIF_RX_PDU_COUNT]` | One small flag/result value per RX PDU |
| `IOIF_TX_CONFIRMATION_TABLE` | `[AtomicU8; IOIF_TX_PDU_COUNT]` | One small confirmation result per TX PDU |
| `IOIF_TX_GROUP_CONFIRMATION_TABLE` | `[AtomicU8; IOIF_TX_PDU_GROUP_COUNT]` | One small confirmation result per TX group PDU |
| `COMM_CURRENTCOMMODE` | `[AtomicU8; COMM_NETWORK_HANDLE_COUNT]` | Current communication mode per network handle |
| `COMM_REQUESTEDCOMMODE` | `[AtomicU8; COMM_NETWORK_HANDLE_COUNT]` | Requested communication mode per network handle |
| `EXTI_CALLBACK` | `[AtomicUsize; 16]` | Stores function pointer addresses for EXTI lines |
| `PORT_CONFIG`, `DIO_CHANNEL_CONFIG`, `EXTI_CONFIG` | `const` config objects/slices | Read-only configuration |

## Rule 1 - Prefer `const` for Configuration

Configuration tables should be immutable:

```rust
pub const IOIF_RX_PDU_COUNT: usize = IOIF_RX_PDU_CONFIG.len();
```

Good examples:

```rust
PORT_CONFIG
DIO_CHANNEL_CONFIG
EXTI_CONFIG
IOIF_RX_PDU_CONFIG
IOIF_TX_PDU_CONFIG
```

Use `const` or immutable `static` when the value never changes after build/startup.

## Rule 2 - Avoid `static mut` for Interrupt-Shared State

`static mut` is risky because Rust cannot protect you from simultaneous access:

```rust
static mut FLAG: u8 = 0;
```

If an interrupt writes `FLAG` while the main loop reads it, the code needs a clear synchronization strategy.

For simple integer flags/counters, prefer atomics:

```rust
use core::sync::atomic::{AtomicU8, Ordering};

static FLAG: AtomicU8 = AtomicU8::new(0);
```

## Rule 3 - Choose the Smallest Clear Atomic Type

For simple flags:

```rust
AtomicBool
```

For small enum-like status values:

```rust
AtomicU8
```

For counters that can fit in 0..255:

```rust
AtomicU8
```

For register-sized values or bit masks:

```rust
AtomicU32
```

For addresses or pointer-sized values:

```rust
AtomicUsize
```

In this project, TX/RX status tables use `AtomicU8` because the stored values are small return/status values.

## Rule 4 - Use `AtomicUsize` for Function Pointer Storage

Function pointers are pointer-sized, so do not store them in `AtomicU32` unless the architecture decision is very explicit.

Current EXTI callback idea:

```rust
static EXTI_CALLBACK: [AtomicUsize; 16] =
    [const { AtomicUsize::new(0) }; 16];
```

Store:

```rust
EXTI_CALLBACK[line as usize].store(callback as usize, Ordering::Release);
```

Load:

```rust
let callback = EXTI_CALLBACK[line as usize].load(Ordering::Acquire);
```

Convert back:

```rust
Some(unsafe { core::mem::transmute(callback) })
```

The `unsafe` remains because Rust cannot prove that the integer is a valid function pointer. The safety rule is: only values produced from valid `fn()` pointers should be stored in this table.

## Rule 5 - Pick Ordering Based on What You Need

For simple counters/flags where no other data depends on the flag:

```rust
Ordering::Relaxed
```

This is currently enough for:

```rust
BUTTON_COUNT
IOIF_INDICATION_TABLE
IOIF_TX_CONFIRMATION_TABLE
```

For publishing a callback pointer before an interrupt may read it:

```rust
store(..., Ordering::Release)
load(Ordering::Acquire)
```

This is used for EXTI callback registration.

Use `SeqCst` only when you need the simplest strongest ordering or when debugging ordering-sensitive code. It is easy to reason about, but heavier than needed for many embedded flags.

## Rule 6 - Array Initialization Needs `const { ... }`

Atomic types are not `Copy` in the normal sense. This will not work for arrays in many cases:

```rust
static TABLE: [AtomicU8; 4] = [AtomicU8::new(0); 4];
```

Use an inline const block:

```rust
static TABLE: [AtomicU8; 4] = [const { AtomicU8::new(0) }; 4];
```

Meaning:

```text
For each array element, create a new AtomicU8 initialized to 0 at compile time.
```

## Rule 7 - Bounds Check Before Indexing Global Tables

Any global table indexed by config should validate the index:

```rust
if index >= IOIF_TX_PDU_COUNT {
    return IoIf_ReturnType::IOIF_E_NOT_OK;
}
```

This matters because config mistakes should not silently access the wrong table element.

## Current Recommendation

Keep this direction:

```text
read-only config       -> const tables
small shared flags     -> AtomicU8 or AtomicBool
small shared counters  -> AtomicU8 or AtomicU16
bit masks/registers    -> AtomicU32 when truly shared
function pointers      -> AtomicUsize plus a narrow unsafe conversion point
larger shared structs  -> avoid for now; later use critical sections
```

For this project stage, atomics are a good learning step before moving to more formal critical-section or scheduler-based state ownership.
