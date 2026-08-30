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

## Which operations are synchronous is already an engine constraint

Methods whose result is consumed inline cannot suspend. In the current
`CUnitScript` contract these include:

- `QueryLandingPads`;
- `QueryTransport`;
- `QueryNanoPiece`;
- `QueryBuildInfo`;
- `QueryWeapon`;
- `AimFromWeapon`;
- `BlockShot`;
- `TargetWeight`;
- `HitByWeapon` and `WorldHitByWeapon`, which synchronously mutate damage
  through an in/out reference.

`QueryLandingPads` is a list result rather than a scalar, so its Wasm/native
transport needs an explicit list/capacity contract rather than only a scalar
return convention.

`AimWeapon` and `AimShieldWeapon` are different: the C++ virtuals return `void`,
but existing unit-script semantics allow asynchronous aiming. A Rust
`async fn aim_weapon(...) -> bool` can therefore use the `bool` as completion
state that the adapter publishes to the engine; it is not returned synchronously
through `CUnitScript::AimWeapon`.

`Killed` is also a multi-step protocol. Regardless of how Rust code is authored,
the engine adapter must guarantee death-script settlement even if the Rust
handler is missing or traps. Engine-visible completion cannot rely on guest
unwinding or Rust `Drop`.

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

The attachment/create-script ABI should include at least stable instance
identity and a generated mask describing which standard entry points the script
provides. Unimplemented entry points should be rejected/skipped engine-side
without entering the guest.

## Keep the C++ adapter small

`CUnit` stores its script object in the fixed inline `usMemBuffer`, sized around
the existing unit-script subclasses. A Rust-backed `CUnitScript` adapter should
therefore contain only small routing/ownership data such as module identity,
instance identity, and capability/provided-method masks. Real Rust script and
task state belongs in the owning Rust module.

Growing the adapter should not require growing per-unit storage for every game,
including games that do not use Rust CUS.

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

The engine already performs animation interpolation separately from the
single-threaded completion pass. CUS should preserve that model: script code is
not run from the multithreaded interpolation work, and completion/wakeup ordering
stays deterministic.

## Custom named entry points

`NativeCallUnitScript` in `SyncedCtrl.cpp` currently assumes Lua by casting to
`CLuaUnitScript`. Rust CUS requires that functionality to become backend-neutral
so game logic can invoke custom unit-script entry points without depending on a
Lua script environment.

A typed same-module Rust path is preferable when both sides are Rust. The
generic engine path remains useful for cross-language or externally initiated
calls.

## Attachment and ownership

The engine already supports replacing/attaching the per-unit script object for
LUS through the unit-script creation path. Rust CUS can use the same structural
model: the owning synced game module explicitly attaches a Rust script instance
to a unit, rather than requiring a new unit-def file format or backend probing.

Exact registration/callout spelling is an implementation detail. The invariant
is that a unit has one current `CUnitScript` owner and attachment is explicit and
deterministic.
