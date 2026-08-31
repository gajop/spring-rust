# Rust CUS implementation

This document records the concrete V1 slice currently implemented in the
engine and Rust crates. The surrounding design documents deliberately leave
some names open; the interfaces below are the stable implementation boundary
for this slice.

## Shared Rust API

`rust::crates::spring::cus` is available when the `alloc` feature is enabled.
It is target-independent and contains:

- typed `UnitId`, `WeaponId`, `WeaponDefId`, `Piece`, `Axis`, `Angle`, and
  `AngularSpeed` values;
- LUS-compatible `Duration` conversion (`max(1, floor(milliseconds / 33))`);
- immediate piece/visibility/effect/attachment operations through
  `UnitEngine`;
- deterministic `CusScheduler`, `TaskDefinition`, task handles, frame waits,
  animation waits, and signal-mask cancellation;
- `UnitScript`'s synchronous query and in/out call-ins with neutral defaults;
- `ScriptCapabilities`, with one stable bit for each standard call entry point;
- `CusInstance` construction/attachment and generation-safe `CusRegistry`;
- `prelude` imports for game-owned script modules.

The source is organized as a module directory rather than a monolith:
`types.rs` contains value types, `engine.rs` the engine-operation traits,
`scheduler.rs` task state and wake indexes, `script.rs` the script contract,
`instance.rs` and `registry.rs` ownership, and `core_module.rs`/`wasm.rs` the
two transports. `mod.rs` only declares these modules and preserves the
`spring::cus` public re-exports.

`TaskDefinition::new` accepts a known function entry point. `with_state` and
`CusInstance::spawn_with_state` provide the same named-task model while giving
the task an `Rc<RefCell<S>>` handle to the owning script state. Tasks are
started synchronously and run through their first suspension before the caller
continues. Suspended tasks are drained in stable insertion order at each
strictly newer simulation frame.

`CusInstance` exposes the synchronous `UnitScript` methods as typed dispatch
helpers. `CusRegistry::tick` drains all attached instances in ascending unit-ID
order, giving a module backend one deterministic frame call while preserving
per-instance task isolation.

The shared API intentionally does not require Tokio, nightly Rust, or a
proc-macro. It uses ordinary stable Rust futures and keeps task storage
private so generated task storage can replace boxed futures later.

## Engine adapter

`CNativeUnitScript` implements the existing `CUnitScript` contract. It stores only
the unit pointer, owning backend pointer, instance ID, and provided-call mask in
the existing inline unit-script buffer. Rust-owned script state and suspended
futures stay in the owning module. The mask uses one bit per
`NativeUnitScriptCall`, so absent standard entry points are skipped before a
backend crossing.

`NativeUnitScriptBackend` is the backend-neutral dispatch boundary. The engine
contains two concrete hosts:

- NativeCUS calls the optional `RustCUSInvoke`, `RustCUSCallNamed`,
  `RustCUSTick`, and `RustCUSDetach` exports supplied by a native module.
- Core-Wasm resolves the corresponding `SPRING_CUS_*` exports from a Wasmtime
  module and marshals calls through its validated module-owned scratch buffer.

Both hosts implement:

```text
Attach(instance, adapter)
Invoke(instance, standard_call, float_arguments, integer_arguments, result)
CallNamed(instance, name, float_arguments, return_buffer, ...)
Detach(instance)
Tick(game_frame)
```

The adapter normalizes engine pointers to IDs before dispatch. It represents
damage as an explicit synchronous replacement value and represents landing
pads as a vector result. If dispatch is unavailable or reentrant, the adapter
uses the documented neutral values: no piece (`-1`), an empty landing-pad
result, `false` for `BlockShot`, `1.0` for `TargetWeight`, and unchanged damage.

`AimWeapon` and `AimShieldWeapon` remain `void` engine call-ins. A backend may
return an immediate completion result or leave the request pending; a pending
Rust task publishes completion through the engine-owned completion hooks. The
adapter is supplied to the backend at attachment time for this weak, non-owning
completion association.
`Killed` always settles through the delayed wreck level when no handler,
backend, or completion result is available.

Destroying the C++ adapter calls `Detach(instance)` before its storage is
released, allowing a backend to drop the corresponding Rust registry entry and
cancel any remaining tasks. If a death task is still pending at that point,
the adapter settles it with the delayed wreck level before releasing the
script, so death completion does not depend on Rust future destruction.

The Core-Wasm transport uses a 16 KiB buffer. Float arguments start at byte 0,
integer arguments at 1024, named-call text at 2048, and result storage at
4096. A module exports `SPRING_CUS_BUFFER`, `SPRING_CUS_BUFFER_SIZE`,
`SPRING_CUS_INIT`, `SPRING_CUS_INVOKE`, `SPRING_CUS_CALL_NAMED`,
`SPRING_CUS_TICK`, and `SPRING_CUS_DETACH`; the export macro in
`spring::cus::core_module` supplies this ABI for a `CoreCusModule`.

## Attachment and scheduling

`CUnitScriptFactory::AttachCusScript` explicitly replaces the current unit
script and returns the newly constructed adapter. The owning module must make
its Rust instance visible to routing before starting its startup task, then
invoke the adapter's `Create` call-in when attachment occurs after normal unit
initialization. No unit-definition extension or automatic backend probing is
needed.

`CUnitScriptEngine` tracks every loaded NativeCUS/Core-Wasm backend. It calls
each backend's `Tick` once per simulation frame after the normal deferred COB
call-ins, preserving module-level scheduling rather than ticking idle units
individually. Removing a backend first detaches all of its adapters, so module
unload cannot leave dangling script references. Core-Wasm `Create` calls are
queued while an attach import is executing and flushed after the guest call
returns, because Wasmtime stores cannot be re-entered. Engine animation
interpolation remains in `CUnitScript`; Rust waits query completion state from
the backend.

Named `CallUnitScript` dispatch is now virtual on `CUnitScript`, so Lua keeps
its existing implementation and Rust/backends can implement the same generic
engine path without a Lua cast.

## Deliberate V1 boundary

The transport, attachment, lifecycle, and scheduling paths are implemented for
both supported module hosts. Suspended-future save/load is not implemented, as
permitted by the V1 design. A future durable backend may replace the task
storage without changing the engine-facing CUS contract.
