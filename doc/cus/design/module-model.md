# Synced Module Model

This is a core architectural requirement:

> Rust CUS scripts are part of the same synced Rust module as the rest of the
> game's synced Rust logic.

A game should be able to structure itself normally:

```text
my-game/
  src/
    lib.rs
    targeting.rs
    status.rs
    abilities.rs
    unit_scripts/
      mod.rs
      amphraid.rs
      commander.rs
```

Those are Rust modules/crates in one synced game program. They are not separate
engine plugins connected through an RPC layer.

## Game to CUS

Game logic must be able to reach a unit's typed script through a registry or
handle API, for example:

```rust
cus::with::<AmphRaid>(unit_id, |script, ctx| {
    script.set_range_multiplier(ctx, multiplier);
});
```

The exact registry API is provisional. The important property is that this can
be an ordinary same-module Rust call.

## CUS to game

CUS methods can call the game's own APIs directly:

```rust
fn block_shot(
    &mut self,
    ctx: &mut UnitCtx,
    weapon: WeaponId,
    target: UnitId,
) -> bool {
    crate::overkill::should_block(ctx.unit(), target, weapon)
}
```

Spring does not define `crate::overkill`. Zero-K, MCL, BAR, or another game can
have completely different APIs.

## Many scripts, one module

One synced module can register many CUS script types and hold many instances:

```text
RulesSynced module
  AmphRaid code       -> instances for units 4, 18, 27, ...
  Commander code      -> instances for units 1, 41, ...
  Factory code        -> instances for units 12, 13, ...
```

The engine routes a unit-script invocation to the owning game module plus the
specific unit-script instance. This is separate from ordinary module-wide
`Callins.def` fan-out.

## Why this matters

Keeping CUS inside the game module gives:

- typed game-specific APIs;
- no extra Wasm crossing for game-to-CUS calls;
- shared game data structures and libraries;
- simple native parity;
- freedom to factor reusable game crates normally;
- a clear distinction between CUS scheduling and module/event dispatch.

## Runtime source layout

The portable runtime is kept in `rust/crates/spring/src/cus/`. `mod.rs` is a
thin compatibility surface that re-exports the public API. Value types,
engine operations, scheduling, script behavior, instance ownership, registry
ownership, and the native/Core-Wasm transports live in their corresponding
focused modules. This keeps transport changes from obscuring scheduler
invariants while preserving the single `spring::cus` API used by game code.
