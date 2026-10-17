# Scoped Rust SDK: native and Wasm

Status: opt-in implementation on `sdk-unification`, separate from `rust-wip`.
Production games and menus have not been migrated. The shared API is the new
`spring-sdk` crate; existing `spring`, `spring-addons` and `spring-native` users
keep their APIs.

## What is unified

The generator emits one public callout surface from the normalized API model:
**1,425 methods**, with real native and Wasm adapters. Strict generation fails
if a method needs an adapter. It also generates shared owned query/result
handlers for every canonical callin and its aliases. Initialization and shutdown
are provided by the module entry wrapper. This coverage describes executable
adapters, not exhaustive behavioural verification of 1,425 engine functions.

```rust
fn update(
    app: &App,
    spring: &mut Spring<'_, Menu>,
    query: &UpdateQuery,
) -> Result<UpdateResult> {
    app.update(spring, query.delta_seconds)?;
    Ok(UpdateResult::default())
}
```

Native code calls the actual `NativeInterface` tables with native C query/result
structures. It does not emulate Wasm memory, pointers or import trampolines.
Wasm uses the existing Core imports and reviewed typed projections. Missing
native tables/functions return `NotAvailable`; adapters do not fabricate
successful engine results.

Public values use semantic Rust records, strings and vectors on both targets.
Native output strings/lists are copied before another host call can reuse its
scratch storage. Native input arenas keep nested strings, records and buffers
alive through the complete synchronous call. Callin queries and retained event
arguments are copied before user code runs, including before reentry.

The flat generated surface retains existing semantic conventions, including
code-only `ApiError` and some redundant count fields in callin records. It is
not a claim that every historical engine API now has ideal ergonomics.

## Spring references and admission

`Spring<'call, E>` is an explicit scoped engine reference. API families are
obtained from it, for example `spring.units_query()` and `spring.rml_ui()`.
Neither it nor an API-family handle can escape its callin or be sent/shared
with a worker thread. A higher-ranked scope lifetime and invariant lifetime
marker enforce this; compile-fail tests cover both escape and worker access.

Environment markers are sealed: rules synced/unsynced, gaia synced/unsynced,
UI, menu and intro. Generated capability bounds follow the audited loader
matrix. A synced handle cannot draw; a UI handle cannot call synced unit
mutation functions. Capability groups represent sets of environments, rather
than a guarantee that every method in a group performs the same kind of work.
Callin entry additionally checks the handler's environment mask.

The module owns admission authority and passes a fresh Spring reference into
each handler and retained callback. Native authority records the initialization
thread, interface pointer and liveness. Each operation checks them. A foreign
retained callback executes no user handler and reports an error back to the
originating module's next scope boundary.

`ModuleContext::from_native` / `from_wasm` are unsafe admission boundaries for
integration code, not an ambient escape hatch for applications. Their caller
must establish the actual host environment and interface lifetime.

The native module factory currently admits **Menu only**. The standalone loader
checks `NativeModuleEnvironment`; the legacy in-game loader rejects marked SDK
modules. Unmarked legacy modules retain their loading contract. Native rules/UI
and a mixed tools environment require an explicit host dispatch and visibility
contract before admission is enabled. Native synced gameplay is not being
introduced or claimed deterministic by this change.

## Lifecycle and reentry

The factory exports native initialization/shutdown, the environment marker,
Wasm callback dispatch and shared callin wrappers. New Wasm modules opt into
`spring:module/initialize()`. The host invokes it once after validation, binding
and dispatch-plan construction, before the first callin. It applies the normal
execution budget, UI visibility and Rml owner scope. Initialization retains the
engine's existing recursive registry lock so another loading/frontend thread
cannot destroy the host during entry; same-thread callbacks reenter immediately.
Normal cached callin dispatch does not acquire that startup lock.
Legacy modules without this export keep their
lifecycle. Initialization failure clears that host's Rml contexts and rejects
the module.

Module handlers take `&Module`, not a module-wide `&mut Module`. The runtime
clones an `Rc` before entry and holds no slot/registry borrow across user code
or a callout. Nested callins therefore enter immediately, with their own Spring
scope. The SDK does not automatically queue mutations or replace immediate
answers with neutral defaults.

Application state still needs ordinary Rust borrowing discipline. Use separate
`Resource`s, `RefCell`s, `Cell`s or immutable snapshots, and end conflicting
borrows before callouts which can reenter that state. Passing Spring does not
make overlapping mutable game-state borrows valid. The retained callback API
uses `Fn`; the same listener can reenter itself when its explicitly borrowed
state permits it. A conflicting borrow fails visibly. Synchronous batch APIs
accept `FnMut`; recursively invoking that same active closure is rejected,
while nested independent callbacks remain supported.

Native panics are caught before crossing C ABI boundaries. A panic in a
synchronous host callback is resumed only after host cleanup returns, then
contained by the module boundary. Retained callback errors/panics are surfaced
at the next API/scope boundary. An unhandled callin failure poisons the module
and rejects subsequent handlers. Native builds require `panic=unwind`;
`panic=abort` is rejected. Wasm entry failures trap and the engine's normal
fault handling removes the module.

## Threads and rendering

A mutable Spring reference controls use of that token; it is not an exclusive
Rust borrow of all engine state. A Spring reference is admission authority, **not a mutex and not permission to
call the engine from arbitrary threads**. Current native SDK access stays on
the admitting engine thread. Workers can receive owned snapshots and return
owned results through ordinary Rust channels. Engine references, API-family
handles and Rml binding identities stay on the engine thread.

Rendering still has an engine draw-phase contract. Environment bounds do not
prove that a particular callin is inside the appropriate GL pass. Use the
render callins for drawing/readback. The fixture captures its image from
`DrawScreenPost`, after Rml rendering.

Legacy `NativeInterfaceRef` threading assumptions are unchanged. This SDK does
not retroactively make existing native modules or SpringBoard thread-safe.
A future tools/native-game admission design must specify thread ownership,
visibility, per-thread host state and which callouts can run concurrently.
In particular, native game loading and later simulation/render entries may use
different engine threads; merely remembering the initialization thread would
be insufficient for that future factory.

## Shared Rml layer

`spring_sdk::rml` provides one implementation of `DataModel`, `Variable<T>`,
`Rows`, `EventBinding`, `Color`, `Pixels` and `Percent`. Scalars include bool,
i32, f32 and String. Bindings own identities and values rather than Spring
references; every operation requires a fresh scoped reference from the same
module. Rows validate schema and complete row width before mutation.

Closing a model invalidates its typed bindings before a host call can reenter
one of them. Cleanup requiring engine access is explicit (`close` / `unbind`);
Rust `Drop` does not call the engine through a stale reference. Module teardown
cancels Rust callbacks, and the engine clears owner contexts/listeners before
unloading the native library or Wasm instance. Low-level generated handle APIs
remain available; callers using them directly must manage their own logical
binding lifetime.

Rml numeric expression literals arrive as floating-point values. Event schemas
must match actual argument types; the fixture checks a UTF-8 string and f32
argument rather than silently coercing a mismatched integer schema.

## Compatibility and scope

| Consumer | Current impact |
| --- | --- |
| Seven SpringCabal Wasm games | Keep `spring` / `spring-addons`; no source migration |
| SpringCabal menu | Keeps its existing SDK and RML; no native conversion |
| Other downstream game and menu | Keep existing SDK and RML; no source migration |
| SpringBoard | Keeps `spring-native`; its god-mode tuple compile repair is independent |
| New SDK menu fixture | One Rust/RML source builds as native and Wasm |

The loader audit also fixes missing menu/intro registrations when a function
already matched an in-game provider. Registrations are merged by provider and
function, so unrelated functions do not gain standalone access. For example,
menu config reads become available, while `GetGameFrame` stays excluded from
Menu. Existing wire routes remain unchanged. Eight previously unregistered
borrowed transports are added; a regression test protects the existing
length-prefixed string route from replacement by the NUL-terminated route.
The legacy generated string-result macros also receive a Rust 2024 unsafe
pointer-expression fix.

This is the shared **core SDK**, not automatic migration of every convenience
layer. Existing addons, command queues, CUS and job-extension helpers remain
on their legacy integrations. A native tools/game factory and migration of
those helpers need follow-up design/work. The existing menus are not converted,
and room discovery, readiness/reconnect UX and production native-menu packaging
are separate work.

## Verification and trying it

The crate tests cover scratch ownership, thread rejection, immediate reentry,
borrow conflicts, panic cleanup, callback cancellation and typed Rml lifetime/
schema checks. A generated fixture expands every callin export on native and
Wasm, including strings, records and lists. Native tests also run as Windows
GNU executables under Wine; this does not replace a native Windows test.

The repository-owned [menu fixture](../../../test/wasm_api/data/scoped_sdk_menu/README.md)
checks real Rml rendering, typed values/events, same-listener reentry,
two generated-map game launches/returns, and teardown on both transports.
Its native and Wasm screenshots must have visible panel/title colors and
identical pixels. A separate RulesSynced guest verifies initialization, a real
game-frame handler and shutdown during those launches. Callback readers are
linked only when registered, so the generic dispatcher does not introduce
frontend Rml imports into a synced module. A negative run deliberately fails
initialization; an independent probe checks that the failed owner's context
is gone and its name can be reused. Both transports must reject the module
before activation and exit normally. The fixtures contain no downstream game
assets or source.

Recorded fixture results and rendered screenshots are retained in
[data/scoped-sdk-2026-10-17](data/scoped-sdk-2026-10-17/results.json)
([native](data/scoped-sdk-2026-10-17/native.png),
[Wasm](data/scoped-sdk-2026-10-17/wasm.png)). Both positive and failed-initialization
runs exit zero. Legacy compatibility checks pass for all seven Cabal games,
both existing menus, the downstream game workspace and SpringBoard; its native
plugin also builds. These are compile/build checks, not full gameplay tests for
every downstream game. The SDK engine passes all 38 CTest suites. Legacy
native parity and all 429 selected synced Wasm probes match their Lua reference;
render-only and other excluded probes are not counted. Workspace tests, the
all-crate native/Wasm build check, SDK Clippy on both targets and generated
artifact reproducibility checks pass.

Use a locally tested SDK engine for this fixture. Packaging downstream games
continues to consume independently built engine releases.
