# Save/Load Path

Exact persistence of suspended Rust execution is **not required in V1**.

V1 should use ordinary Rust futures because they give the desired UX and let us
validate the much more important runtime, API, routing, native/Wasm, and
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

The public source should remain the same:

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
same source -> #[cus] durable lowering -> explicit serializable CUS task state
```

A future proc macro can treat CUS `.await` points as suspension boundaries and
lower control flow into explicit program-counter states.

## Cross-suspension locals

The clean forward-compatible rule is likely:

> A local that must survive a CUS suspension needs an explicit type and, for a
> persistent task, that type must satisfy a CUS save-state trait.

For example:

```rust
let target: Vec3 = ctx.position_of(enemy);
ctx.sleep(500.ms()).await;
ctx.move_to(target);
```

could later lower to task state containing `target: Vec3`.

This is a reasonable authoring restriction and can be introduced without
changing the overall async style.

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

To keep migration practical:

- use CUS-owned wait/sleep primitives for suspension;
- avoid making arbitrary third-party future graphs part of the persistence
  contract;
- keep durable game state in normal script/game structs where possible;
- do not expose compiler-generated future layout as API.
