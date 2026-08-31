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

## Engine pointers do not cross the portable Rust API

Several `CUnitScript` virtuals use engine pointers as C++ implementation details,
including transport methods taking `const CUnit*`, `AimShieldWeapon` taking a
`CPlasmaRepulser*`, and targeting methods taking `const CUnit*`.

These pointers must be normalized by the C++ adapter before dispatch. The public
Rust API should use stable values such as `UnitId`, `WeaponId`, or another typed
handle, identically for WasmCUS and NativeCUS. NativeCUS should not expose raw
engine pointers merely because it could.

Likewise, C++ in/out references become explicit values in the Rust ABI. For
example `HitByWeapon(..., float& inoutDamage)` should enter Rust with the current
damage and synchronously return the replacement damage (or an equivalent typed
result), rather than exposing a reference across the boundary.

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

## Keep the C++ adapter small and backend-neutral

`CUnit` stores its script object in the fixed inline `usMemBuffer`, sized around
the existing unit-script subclasses. A Rust-backed `CUnitScript` adapter should
therefore contain only small routing/ownership data such as module identity,
instance identity, and capability/provided-method masks. Real Rust script and
task state belongs in the owning Rust module.

Growing the adapter should not require growing per-unit storage for every game,
including games that do not use Rust CUS.

The adapter should be backend-neutral from its first implementation. WasmCUS may
be implemented first, but the C++ object should target a Rust-CUS backend
interface rather than depend directly on Wasmtime-specific state. NativeCUS then
implements the same adapter-facing contract instead of requiring a retrofit.

## Reuse current Wasm runtime infrastructure

WasmCUS should reuse the current `rust-wip` infrastructure for:

- module ownership;
- environment selection;
- load/unload and traps;
- Core-Wasm validation;
- execution budgeting;
- module dispatch/runtime lifetime.

It should not create another Wasmtime runtime beside `WasmInterfaceSystem`.

## Initialization and `Create`

LUS provides an important precedent for attachment ordering. The framework first
constructs the per-unit environment, registers the callin table with
`Spring.UnitScript.CreateScript`, registers its own unit bookkeeping, and only
then starts `script.Create` as a thread. `CLuaUnitScript::Create()` itself is a
no-op.

Rust CUS should make this ownership explicit rather than waiting for an engine
callin that may arrive too early or not at all:

1. synchronously construct/register the Rust script instance during attachment
   (for example the provisional `fn new(&mut InitCtx) -> Self`);
2. make the instance visible to routing/scheduling;
3. if the script defines suspendable startup work, start that as a normal named
   CUS task after attachment is complete.

The exact names are provisional; the ordering is not.

## Synchronous re-entrancy fallback

Core Wasm cannot recursively enter an already-running Wasmtime store. A
synchronous engine request that re-enters the same CUS runtime therefore cannot
be queued or suspended: the engine needs a result immediately.

The adapter should define neutral per-method fallback behavior from the first
slice and use the same logical policy for NativeCUS where practical, so backend
choice does not alter gameplay semantics. Baseline fallbacks matching existing
LUS failure/missing-handler behavior are:

| Method | Fallback |
| --- | --- |
| `QueryWeapon` | `-1` / no piece |
| `AimFromWeapon` | `-1` / no piece |
| `QueryTransport` | `-1` / no piece |
| `QueryNanoPiece` | `-1` / no piece |
| `QueryBuildInfo` | `-1` / no piece |
| `QueryLandingPads` | empty result |
| `BlockShot` | `false` |
| `TargetWeight` | `1.0` |
| `HitByWeapon` / `WorldHitByWeapon` | leave damage unchanged |

This table is a CUS adapter contract for an unavailable synchronous dispatch. It
should not be confused with LUS's `default_return_values` table, which is also
used when particular weapon handlers are absent.

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
