# Engine Integration

Rust CUS should implement the existing engine unit-script abstraction rather
than introduce a parallel concept.

## `CUnitScript` remains the engine contract

`rts/Sim/Units/Scripts/UnitScript.h` already defines:

- animation commands and completion tracking;
- standard unit-script virtual methods;
- piece mapping;
- weapon hooks;
- transport/build/movement hooks;
- lifecycle behavior such as `Killed`.

A Rust-backed subclass/adapter should satisfy that contract and route work to a
Rust CUS instance.

## Per-instance routing

Normal Wasm callins are module-wide events. Unit scripts are different:

```text
engine invocation
    + unit ID / script instance
              |
              v
owning synced module
              |
              v
specific Rust CUS instance
```

This should remain a distinct dispatch domain rather than being added to the
ordinary `Callins.def` subscriber fan-out.

## Reuse current Wasm runtime infrastructure

WasmCUS should reuse the current `rust-wip` infrastructure for:

- module ownership;
- environment selection;
- load/unload and traps;
- Core-Wasm validation;
- execution budgeting;
- module dispatch/runtime lifetime.

It should not create another Wasmtime runtime beside `WasmInterfaceSystem`.

## Animation

Piece interpolation remains in `CUnitScript`/the engine. Rust initiates an
animation and waits for completion:

```rust
ctx.turn(piece, Axis::Y, target, speed);
ctx.wait_for_turn(piece, Axis::Y).await;
```

The existing single-threaded animation-completion phase is a natural place to
wake waiting CUS tasks after engine animation state is updated.

## Custom named entry points

`NativeCallUnitScript` in `SyncedCtrl.cpp` currently assumes Lua by casting to
`CLuaUnitScript`. Rust CUS requires that functionality to become backend-neutral
so game logic can invoke custom unit-script entry points without depending on a
Lua script environment.

A typed same-module Rust path is preferable when both sides are Rust. The
generic engine path remains useful for cross-language or externally initiated
calls.

## Attachment and ownership

A unit definition should select its unit-script backend deterministically.
Migration should not rely on several backends probing the unit and racing to
claim it.

The exact unit-def syntax is an implementation question.
