# Game API Integration

Rust CUS must be able to participate in arbitrary game logic. This is one of the
main differences from targeting COB as an execution format.

## Same-module calls

When game logic and CUS live in the same synced module, game-specific APIs are
ordinary Rust:

```rust
pub mod overkill {
    pub fn should_block(
        unit: UnitId,
        target: UnitId,
        damage: f32,
    ) -> bool {
        // Zero-K-specific logic
    }
}
```

A script can call it directly:

```rust
fn block_shot(
    &mut self,
    ctx: &mut UnitCtx,
    _weapon: WeaponId,
    target: UnitId,
) -> bool {
    crate::overkill::should_block(ctx.unit(), target, 130.0)
}
```

There is no CUS ABI crossing here.

## Every game owns its API

Spring should provide engine-facing crates and CUS infrastructure, not a
universal gameplay API.

A game may have crates such as:

```text
zk-game-api
mcl-combat
bar-animation
my-status-effects
```

These can be shared by the game's unit scripts and its other synced logic.

## When direct calls are not possible

Some operations may genuinely target another subsystem, module, or language
runtime. A game-facing wrapper can hide the transport:

```rust
crate::abilities::start_jump(unit, destination);
```

Internally that may be implemented as:

- a same-module function call;
- an engine host API call;
- an engine-mediated message;
- a queued command/event drained later;
- another deliberately asynchronous operation.

CUS should not force one transport for all game APIs.

## Queries versus commands

Deferred events are a good fallback for mutations that do not require an
immediate result:

```rust
abilities::spawn_effect(...);
status::apply(...);
```

They are not a universal replacement for synchronous queries:

```rust
if targeting::can_fire_at(target) {
    ...
}
```

If a caller needs the result immediately, the implementation must provide a
direct/synchronous path or explicitly expose an asynchronous API. Pretending a
next-frame event is synchronous would change game semantics.

## Reentrancy

Core Wasm cannot recursively enter an already-running Wasmtime store. Same-Rust
calls inside the active module do not create that problem; cross-runtime calls
may. Any API that crosses such a boundary must choose direct host data, queued
commands, or another non-recursive design deliberately.
