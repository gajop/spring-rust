# Why Async Rust

Unit scripts are naturally sequential programs that frequently suspend:

```text
turn gun
wait for turn
sleep
restore gun
```

Writing that as explicit hand-maintained states is possible but poor UX:

```rust
match self.state {
    State::Turning => ...,
    State::Sleeping => ...,
    State::Restoring => ...,
}
```

COB and LUS already provide a better sequential model. Rust CUS should not move
backwards.

## Async gives the right source shape

```rust
ctx.turn(self.gun, Axis::X, pitch, speed);
ctx.wait_for_turn(self.gun, Axis::X).await;
ctx.sleep(500.ms()).await;
```

Rustc already handles:

- locals across suspension;
- branches;
- loops;
- early returns;
- nested normal Rust expressions.

That makes ordinary stable futures the simplest V1 implementation.

## Why not nightly coroutines

Nightly Rust coroutines map elegantly to a CUS scheduler because `yield` can
represent `Sleep` or `WaitForTurn`, and rustc still generates the state machine.

They do not solve the important future persistence problem: their generated
state is also opaque/compiler-owned. They additionally require unstable Rust.

They are useful as a reference or experiment, not a reason to make V1 depend on
nightly.

## Why a proc macro may still matter later

A future save/load implementation may need a stable explicit task layout. The
same async-looking CUS source can then be lowered by `#[cus]` into a generated
serializable state machine.

That future possibility is why the proposal distinguishes the **authoring UX**
(async Rust) from the **lowering implementation** (ordinary Future in V1,
potential durable generated state later).
