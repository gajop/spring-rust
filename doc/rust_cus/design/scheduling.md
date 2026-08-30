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

The supported semantic suspension points should be CUS-owned operations:

```rust
ctx.sleep(duration).await;
ctx.wait_for_turn(piece, axis).await;
ctx.wait_for_move(piece, axis).await;
ctx.next_frame().await;
```

These correspond closely to LUS/COB operations and are sufficient for the
common animation/state-machine workload.

Arbitrary futures may be technically possible in V1, but games should not rely
on arbitrary future graphs being persistable later. CUS-owned suspension points
form the forward-compatible subset.

## Engine animation remains engine-side

`CUnitScript` already owns animation interpolation and completion tracking.
Rust scripts should issue engine animation commands and suspend until the
engine reports completion rather than reimplement interpolation in Rust.

This preserves existing behavior and avoids unnecessary per-frame crossings.

## Concurrent tasks

CUS needs an equivalent of LUS/COB script threads for patterns such as:

- walking loops;
- delayed weapon restoration;
- periodic heat/cooling logic;
- independent weapon aiming tasks;
- generated animation playback.

The exact spawn API is not fixed. A likely constraint is that spawned tasks are
CUS-owned and identifiable, rather than accepting every possible external
executor/future composition pattern.

## Signals and cancellation

Signals should cancel matching CUS tasks. With ordinary futures, cancellation
naturally means dropping the owned future.

Exact compatibility details — signal-mask inheritance, destructor behavior,
thread identity, and interaction with synchronous engine requests — should be
verified during implementation against COB/LUS behavior. They are not blocking
the architecture.

## Frame scheduling

The synced game module already receives `GameFrame` and already has a synced
Wasm execution budget in the current branch. CUS scheduling should reuse that
frame/budget model rather than add a second module-level frame call.
