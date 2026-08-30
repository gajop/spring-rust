# Save/Load Path

Exact persistence of suspended Rust execution is **not required in V1**.

That is an informed compatibility choice rather than an unknown:

- COB persists interpreter/thread execution state, so a sleeping or waiting COB
  thread can resume from the same point after load.
- LUS does not serialize Lua coroutine/thread execution state. Engine-owned
  `CUnitScript` animation state is persisted, but the Lua coroutine waiting on
  it is not.

Therefore ordinary Rust futures in V1 are broadly aligned with current LUS
persistence behavior, but migrating a COB script with active serialized threads
to Rust CUS can lose a capability it has today. That tradeoff should be explicit.

V1 should still use ordinary Rust futures because they give the desired UX and
let us validate the more important runtime, API, routing, native/Wasm, and
performance questions first.

## The problem with normal futures

Rust already lowers:

```rust
async fn walk(...) {
    ...
    ctx.sleep(500.ms()).await;
    ...
}
```

into a state machine containing locals that live across the suspension point.
That generated future type and layout are compiler implementation details, not
a stable savegame format.

The same problem applies to current nightly coroutines: rustc owns an opaque
generated state representation.

## Future durable lowering

The public authoring style should remain the same:

```rust
#[cus]
async fn walk(...) {
    ...
}
```

V1:

```text
source -> thin #[cus] metadata/wrappers -> rustc Future
```

Possible later implementation:

```text
same async style -> #[cus] durable lowering -> explicit serializable CUS task state
```

A future proc macro can treat CUS suspension points as boundaries and lower
control flow into explicit program-counter states.

This is a path to preserving the authoring model, not a guarantee that every V1
async body can be transformed byte-for-byte without cleanup. Code that keeps
borrows, iterators, opaque futures, or otherwise non-persistable state across a
suspension may need local changes when opting into durable tasks.

## Cross-suspension locals

For code intended to become persistable, the useful discipline is:

> A local that must survive a CUS suspension should have an explicit owned type;
> a future persistent-task mode may additionally require that type to satisfy a
> CUS save-state trait.

For example:

```rust
let target: Vec3 = ctx.position_of(enemy);
ctx.sleep(500.ms()).await;
ctx.move_to(target);
```

could later lower to task state containing `target: Vec3`.

V1 does not need to reject other normal Rust async code or require save-state
traits before durable tasks exist.

## Why the macro is non-trivial

Finding `.await` is easy. Correctly lowering normal Rust around suspension
points requires handling:

- `if` / `else`;
- `match`;
- loops;
- `break` / `continue`;
- early `return`;
- locals live across several suspension points.

That is compiler-like work and should only be built if durable suspended state
is actually required.

## V1 compatibility guidance

- CUS-owned wait/sleep primitives have stable engine semantics and are the
  suspension points a durable backend is expected to understand.
- Other ordinary futures may be used in V1, but carry no automatic persistence
  promise.
- Spawned script threads/tasks use named CUS tasks so their identity remains
  under CUS control even if V1 stores their futures opaquely.
- Keep durable game state in normal script/game structs where practical.
- Do not expose compiler-generated future layout as API.
