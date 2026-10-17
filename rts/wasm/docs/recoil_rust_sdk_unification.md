# One Rust SDK for native and Wasm

The optional `spring-sdk` crate lives on `sdk-unification`, separate from
`rust-wip`. Existing games, SpringBoard and both existing menus keep their
current APIs until ported in their dedicated worktrees. Native loading supports
menus and in-game `All` modules, including initialization on the loading thread.

## Using it

```rust
fn update(
    app: &App,
    spring: &mut Spring<'_, Menu>,
    delta_seconds: f32,
) -> Result<()> {
    app.update(spring, delta_seconds)
}
spring_sdk::export_update!(APP, update);
```

Handlers take ordinary arguments: `&str` for strings, slices for lists,
`Float3` for positions and a command struct for commands. They return
`Result<()>`, a value such as `Result<bool>`/`Result<String>`, or a tuple for
multiple answers. ABI query/result structs, dummy arguments and redundant
lengths stay inside the adapters.

`spring` gives the handler access to engine calls, such as
`spring.units().get_all_units()`. Native code calls the C API tables;
Wasm code uses Core-Wasm imports. Both expose the same Rust-facing API.

## Compile time or runtime?

| Mistake or condition | Check |
| --- | --- |
| Synced code tries to draw | Compile time |
| UI code tries to change synced unit state | Compile time |
| Handler exported for the wrong environment | Compile time |
| Borrowed Spring handle saved past its callin or sent to a worker | Compile time |
| Wrong native interface version | Runtime, during initialization |
| Native callback on the wrong thread | Runtime, before user code runs |
| Use after shutdown or typed Rml model close | Runtime |
| Team permissions, LOS and engine state | Existing engine runtime checks |

Rust types distinguish rules synced/unsynced, gaia synced/unsynced, UI, menu
and intro. `All` is available on native and Wasm: it combines these APIs and
callins for editors, tools, single-player games and games with their own
synchronization. It does not make arbitrary simulation changes deterministic.
The engine still validates a compiled module's declared environment
when loading it; a loaded file need not have been written using this SDK.

The native adapter checks for null API tables/function pointers. This covers
incomplete test interfaces and invalid integrations. No adapter is intentionally
unimplemented. A matching, fully initialized engine should provide the required
entries. A missing entry returns `NotAvailable` and should be investigated.

## Reentry and threads

Engine calls can immediately call back into the module. The SDK preserves
that behavior, without queuing calls or returning invented default answers.

Handlers receive `&App` rather than one `&mut App` covering the entire module.
Otherwise a nested callin could create a second mutable reference to the same
state. Use separate state fields with `Cell`, `RefCell` or existing resource
helpers, and release conflicting borrows before reentrant engine calls.

Arguments and engine results are copied before user code runs or another call
can overwrite engine scratch storage. No SDK module/callback-registry borrow
is held across user code. The same Rml listener can reenter if its application
state allows it; a synchronous `FnMut` callback rejects recursion into itself.

Native module state and retained callback captures must be `Send`: the engine
can initialize on its loading thread and invoke later callins on another thread.
It does not require the application state to be `Sync`. Native entries are
serialized per module, and same-thread reentry runs immediately. Typed Rml
bindings are transferable identifiers and still need a fresh Spring reference
for every engine operation. Use `Arc` and synchronized shared state for captures
that need sharing; Wasm-only captures can still use `Rc`.

Spring references stay on the engine thread. Workers receive copied data and
send results back. This does not enable arbitrary worker-thread engine calls
or change SpringBoard's existing threading assumptions. Drawing still belongs
in the appropriate draw callin.

Models that previously stored engine references can keep a `ModuleRef<E>`.
`with_spring` supplies a fresh scoped reference, and succeeds only while this
same module is already in a live callin. On native it also checks the thread;
workers can carry an identifier but cannot open an engine scope. No engine calls
run when the identifier is dropped.

`commands::Queue<E>` holds owned closures taking Spring when executed. Its
outer scope flushes FIFO after delayed addon actions; continuations run before
the next command, and reentrant callins can append work. The queue holds no
engine reference. `Spring::reborrow()` lets helpers share a shorter scope.

Native panics are caught before crossing the C ABI; native builds require
`panic=unwind`. Failed handlers stop subsequent use of the module. Wasm failures
trap and follow the engine's module-fault handling.

## Rml and loading

Typed Rml variables, rows and events share one implementation. Their operations
take the current Spring reference. Closing a model invalidates its bindings.
Use explicit `close`/`unbind`; Rust `Drop` makes no engine calls. Unloading a
module also removes its remaining UI resources.

New Wasm modules initialize before normal callins, with the engine's Rml
ownership and execution limits set. Initialization retains the existing
recursive loader lock to prevent concurrent unload; same-thread callbacks can
reenter. Failure rejects the module and removes its UI. Existing Wasm modules
keep their startup path.

## Compatibility and remaining work

All seven Cabal games, both existing menus, the other game workspace and
SpringBoard compile against the branch's existing crates. SpringBoard's native
plugin builds too. Its god-mode compile fix is separate. These are build
checks, not full gameplay testing of every game.

The SDK has adapters for all 1,425 callouts; this is not exhaustive behavioral
testing. All 38 engine suites and legacy native/Wasm parity checks pass,
including 429 selected synced Wasm probes. SDK tests also run under Wine.

The [menu fixture](../../../test/wasm_api/data/scoped_sdk_menu/README.md) uses
one Rust/RML source for native and Wasm. It tests typed events, reentry, two
game launches and failed-initialization cleanup. Both render identical pixels:
[results](data/scoped-sdk-2026-10-17/results.json),
[native](data/scoped-sdk-2026-10-17/native.png),
[Wasm](data/scoped-sdk-2026-10-17/wasm.png).

Both production menus have native/Wasm ports in dedicated consumer worktrees.
The Cabal menu launches and returns from Lua Gravitas on both backends. The
other menu completes its interaction tour, with matching rendered screens.
Game-code ports, SpringBoard, discovery/rooms/reconnect UI and the remaining
shared helpers are still in progress.
A live native/Wasm `All` fixture also tests terrain mutation and drawing in one
module, two launches, shutdown, and native loading-thread handoff
([results](data/scoped-sdk-2026-10-17/all-results.json)). Consumer ports
are still in progress; the fixture is not a claim that they have been migrated.
The Windows engine and same-source menu/game fixture pass under Wine,
including `All` and loading-thread handoff. This is not native Windows or
multiplayer gameplay verification. Packaging keeps using independently
built engine releases.
