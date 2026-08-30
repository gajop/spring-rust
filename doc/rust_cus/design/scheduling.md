# Scheduling and Async

## V1: ordinary Rust futures

V1 should use stable Rust `async`/`Future`s rather than a custom source-to-state
compiler.

```text
#[cus] async fn
      |
      v
rustc-generated Future
      |
      v
CUS executor/scheduler
```

The CUS scheduler polls tasks when the relevant engine condition becomes ready.
It does not need a general-purpose async runtime such as Tokio.

## CUS suspension primitives

CUS-owned engine waits include:

```rust
ctx.sleep(duration).await;
ctx.wait_for_turn(piece, axis).await;
ctx.wait_for_move(piece, axis).await;
ctx.next_frame().await;
```

These correspond closely to LUS/COB operations and are sufficient for the
common animation/state-machine workload. They are also the suspension operations
whose engine meaning CUS owns and which a future durable backend can reasonably
lower into explicit saved state.

Other ordinary Rust futures are not prohibited in V1. They simply do not gain a
promise that an eventual persistent-task backend can serialize or reconstruct
them automatically.

## Sleep conversion

For LUS-compatible millisecond sleeps, preserve the current framework's exact
conversion:

```text
frames = max(1, floor(milliseconds / 33))
```

The divisor is the literal `33`, not `30` and not a value derived from
`GAME_SPEED`. In particular, `Sleep(0)` resumes on the next frame. Rust
`ctx.sleep(Duration)` should use this conversion when expressing LUS-style
millisecond sleeps so ports do not silently change animation timing.

Frame-native operations such as `ctx.next_frame()` remain explicit and do not
need to round-trip through milliseconds.

## Engine animation remains engine-side

`CUnitScript` already owns animation interpolation and completion tracking.
Rust scripts should issue engine animation commands and suspend until the
engine reports completion rather than reimplement interpolation in Rust.

This preserves existing behavior and avoids unnecessary per-frame crossings.
The current engine already separates multithreaded animation interpolation from
a deterministic single-threaded animation-completion pass; script wakeups belong
on the script side of that deterministic boundary, not in the MT interpolation
work.

## Concurrent tasks

CUS needs an equivalent of LUS/COB script threads for patterns such as:

- walking loops;
- delayed weapon restoration;
- periodic heat/cooling logic;
- independent weapon aiming tasks;
- generated animation playback.

Spawned tasks should be known/named CUS tasks rather than arbitrary opaque
future values supplied to `spawn`. Conceptually:

```rust
ctx.spawn(Self::restore_after_delay, 5.seconds());
```

This is a public-API decision, not a V1 storage decision. A named task may be
stored as a boxed rustc future initially; later codegen may represent the same
task inline or as an explicit generated enum/state machine without changing the
game source.

For LUS-compatible semantics, `spawn` starts the child immediately: the child
runs until its first suspension or completion before the spawning task
continues. This ordering is observable and should not be replaced by an
unspecified later-frame enqueue.

## Signals and cancellation

V1 signal behavior should match LUS for ported scripts:

- a numeric signal cancels suspended tasks whose numeric signal mask intersects
  it (`task_mask & signal != 0`);
- if non-bitmask signal identifiers are supported, they match by equality;
- a spawned task inherits the spawning task's signal mask; a top-level task
  starts with mask `0`;
- signalling removes matching tasks that are currently sleeping/waiting;
- the currently running task is not killed by its own `signal()` call.

With ordinary futures, cancelling a suspended task can mean dropping the owned
future, which may run Rust destructors. That is suitable for guest-internal
cleanup.

Engine-visible settlement must not depend on Rust `Drop`: a Wasm trap provides
no reliable guest unwind point. Lifecycle obligations such as completing
`Killed` therefore need an engine-side fallback/guarantee.

The exact Rust types used for signal masks/identities and task handles remain API
details; the ordering and cancellation behavior above are compatibility
semantics.

## Frame scheduling

The synced game module already receives `GameFrame` and already has a synced
Wasm execution budget in the current branch. CUS should reuse that module-level
scheduling/budget path rather than introduce a per-unit tick.

Due timers, queued task wakeups, and other batchable scheduler work should be
drained for the module as a whole. Immediate synchronous unit-script requests
remain direct per-instance calls; sleeping tasks must not cause one guest
crossing per unit per frame.
