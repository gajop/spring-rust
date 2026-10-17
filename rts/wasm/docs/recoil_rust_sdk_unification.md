# One Rust SDK for native and Wasm

The optional `spring-sdk` crate lives on `sdk-unification`, separate from
`rust-wip`. Consumer ports live in dedicated worktrees. Original game,
SpringBoard and menu checkouts retain their current APIs. Native loading supports
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
API families take `&self`, so a handler can hold `spring.units()` and
`spring.game()` together, including across an immediate nested callin.

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

`commands::Queue<E>` holds owned closures taking Spring when executed. It
flushes FIFO; continuations run before the next command, and nested callins
can append work. Addon handlers drain delayed actions before queued commands,
after the outermost handler returns. Both receive fresh Spring references.
The queue holds no engine reference. `Spring::reborrow()` lets helpers share
a shorter scope.

Native panics are caught before crossing the C ABI; native builds require
`panic=unwind`. Failed handlers stop subsequent use of the module. Wasm failures
trap and follow the engine's module-fault handling.

## Background jobs

Workers are ordinary `fn(&[u8]) -> Vec<u8>` functions. Declare them once with
`export_jobs!`, submit with `spring.jobs().submit(WORKER, &input)`, and consume
the result in `Module::job_result`. Widget, gadget, and unsynced addon traits
also have `job_result`; `spring_addons::export_jobs!` declares their workers.
Addon results use the normal context and command queue.

Pass all inputs explicitly. Workers must not use mutable globals or engine
calls. Native uses up to four background threads per module, created on first
use. Wasm uses the existing engine job scheduler and a fresh private instance
for each job. `WORKER.in_wasm_module("navigation-worker")` selects a separately
loaded small Wasm module; native still calls the same linked Rust function.

Native Menu and All jobs submitted before or during Update are sealed after
that Update. Results arrive in submission order before the following Update,
even when workers finish in a different order. Synced Wasm jobs retain the
engine's global ordering and next-GameFrame delivery. Native synced module
loading is not supported. Jobs receive copied input and return owned output;
the result callback gets a fresh Spring reference and may submit more jobs.
Shutdown joins native workers before releasing module state or code.

Each input/output is limited to 1 MiB. Native queues allow 256 jobs and 16 MiB
of input per module per Update, matching local Wasm limits. Synced Wasm limits
are shared across synced modules, as before. Worker panics/traps and oversized
outputs become job errors; host resource failures stop the module. The live
fixture checks ordered output, an empty result, an oversized output, work
submitted from a result callback, and a separate Wasm worker module.

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

The consumer ports live in dedicated worktrees. The original game, menu, and
tool checkouts have not been edited. SDK changes remain on `sdk-unification`,
separate from `rust-wip`.

The SDK has adapters for all 1,425 callouts; this is not exhaustive behavioral
testing. All 38 engine suites and legacy native/Wasm parity checks pass,
including 429 selected synced Wasm probes. SDK tests also run under Wine.

The [menu fixture](../../../test/wasm_api/data/scoped_sdk_menu/README.md) uses
one Rust/RML source for native and Wasm. It tests typed events, reentry, two
game launches and failed-initialization cleanup. Both render identical pixels:
[results](data/scoped-sdk-2026-10-17/results.json),
[native](data/scoped-sdk-2026-10-17/native.png),
[Wasm](data/scoped-sdk-2026-10-17/wasm.png).

Both production menus have native/Wasm ports. The Cabal menu launches and
returns from Lua Gravitas on Linux and Wine. The other menu completes its
interaction tour and actual game launch/return on Linux and Wine, with
matching rendered screens.

SpringBoard uses `All` and keeps `ModuleRef<All>` in its existing models. Its
background I/O workers still receive data rather than Spring references.
Shared UI state permits overlapping reads and rejects conflicting writes
immediately. Shutdown closes UI models and joins owned workers.

SpringBoard's full lint chain, 110 Rust unit tests, 85 in-engine integration
tests (including slow tests), 8 smoke checks, 19 E2E-driver regression tests,
and Windows cross-check pass. Its 53-case E2E run has 38 complete cases and 15
screenshot failures; all functional assertions pass. The pre-port snapshot,
tested with the same renderer and driver, reproduces those visual failures
and additionally fails the brush, project-info, and numeric-drag assertions
fixed in the port. Checked paired screenshots match within the existing
tolerance. Expected screenshots and tolerances were not changed. The visual
suite is still failing. The Windows editor DLL also passes all 85 integration
tests and eight smoke checks under Wine, including compilation and export.

The addon handlers and shared Cabal libraries now use the SDK on native and
Wasm. The shared library's 44 unit tests pass. Widgets and helpers receive
Spring explicitly; definition caches accept a resolver closure. Unit scripts
use the same scheduler source on both backends, and native task captures must
be `Send`. Their host belongs to the addon module and permits callbacks during
attachment without borrowing the entire host. Engine operations resolve a
fresh scope through the owning module. Missing or rejected CUS operations
produce errors. The same-source native/Wasm fixture also tests a real unit,
piece animation, a suspended task and a named engine-to-CUS call on Linux and
Wine. Addon tests cover native CUS reentry during
attachment, named results, task wakeups, unit-ID reuse, addon job results and
shutdown. All 12 addon tests pass on Linux and Wine. The SDK
has 51 unit tests, an export test, and 8 compile-fail checks. These checks do
not replace actual game execution.

`UnitCtx::with_spring` gives a unit-script task a fresh synced Spring reference
for one operation. It rejects workers and detached scripts. Suspended tasks
retain only the checked module identifier. The SDK also provides scoped frame,
numeric rules-param, and screen-picking helpers used by the game ports. Default
unit creation requests automatic IDs on both backends.

`spring.unsynced_random().float()`/`int()` preserve the former module-local
PCG stream, seeded once from the engine. The generated `next_float()`/`next_int()`
methods draw directly from the engine's unsynced RNG. Game ports must retain
that distinction so rendering cannot change a module's random sequence.

MyCube's Wasm game code now uses the SDK. Its existing 32-check integration
suite passes on Linux and Wine, including unit scripts, UI messages and restart.
All workspace features pass strict Clippy. Its five-shot visual scenario
completes; manually measured pixels meet all six existing tolerances. The
visual suite still rejects the installed Mesa Ubuntu package revision `.4`
versus the recorded `.2`. No golden images or tolerances changed.

Actual game loading caught two callback defects: UI addons must not export
`DrawUnit`, which the engine only supports in unsynced gadgets, and the Wasm
`DrawWorld` callback must have zero arguments. Both are fixed in the source
generator/addon exports, with a real cdylib example and ABI compile check.
Unit-script callbacks also participate in the addon's command queue, so queued
work waits until the callback returns. Tasks started by `Create` now update
the scheduler's next deadline immediately.

Game teardown now shuts down native/Wasm modules before destroying GUI and
sound services, allowing addons to restore UI state and stop music cleanly.
The existing 38 engine suites, native/Wasm launch-and-return fixtures on Linux
and Wine, and SpringBoard's full test command pass with that change.

Flove's Wasm port passes all 84 existing integration assertions on Linux and
Wine, and its Lua/Rust unit animations agree within the existing tolerance.
Its visual run caught a real SDK bug: fixed-capacity graphics arrays rejected
scalar, vec2 and vec3 inputs. Both backends now accept and zero-fill those
prefixes, matching the old Wasm adapter. A native ABI regression test verifies
the exact values passed to the engine and rejects oversized arrays. Black
outlines and mouse-over highlights are restored. All nine captures complete;
measured differences against the pre-port code on the same renderer are
0–2.08%. Four later images exceed the old golden limit in both versions, and
the renderer revision gate remains red. No expected images or limits changed.

Hunted's Wasm port passes strict default/all-feature Clippy and 30 common unit
tests. All 35 integration assertions pass on Linux and Wine. The nearby-shot
fear test sometimes killed its subject before checking fear; the same failure
reproduced in the pre-port build. The fixture now chooses among the same nearby
aim offsets using the actual muzzle position. All fear, ammunition and damage
assertions remain unchanged. It passes at three different Linux run speeds and
under Wine. The visual scenario and HUD/restart checks complete, with all eight
measured images/crops within their existing limits. The Mesa revision gate
remains red.

Area 17's Wasm port passes 22 Rust and six Lua gameplay checks on Linux and
Wine, its technology UI fixture, strict default/all-feature Clippy, six shared
unit tests and installed-module/static verification. The visual scenario
checks the Rml callbacks, four weapons/abilities and crater, and all three
measured images meet the existing 8% limit. Its renderer revision gate is
still red. Diagnostic builds now explicitly enable their INFO sections so
verification markers remain visible with the engine's default NOTICE filter.

Parts Needed's Wasm port passes 70 gameplay assertions and its UI handshake
on Linux and Wine, strict default/all-feature Clippy and four shared unit
tests. Its live Lua/Wasm unit-script comparison passes across 3,498 samples
(maximum transform difference 0.00003384, existing limit 0.0001), alongside
the source/keyframe comparison. All six visual captures complete, including
the non-black and settled-camera checks; every manually measured image/crop
meets its existing limit. The Mesa package revision gate remains red.

Gravitas's Wasm port passes all 19 in-game cases on Linux and Wine, nine shared
unit tests and strict default/all-feature Clippy. Its normal build completes
the real level with no test cases or runtime warnings. Real input verifies
movement, four jump controls, push, pull, shooting just beyond weapon range,
and return to the menu on both platforms. The last case exposed a pre-port
Rust aiming bug: clamping a distant target dropped its height and used only a
horizontal range check. The fix restores the Lua gun's 3D aim and weapon range
query. The existing test is unchanged. Manually measured arena/HUD images meet
their limits; the Mesa revision gate remains red. Wine control testing passes
with software OpenGL; Zink crashes before gameplay and remains unresolved.

To the Last Drop's Wasm port passes all 54 gameplay assertions on Linux and
Wine, ten shared unit tests and strict default/all-feature Clippy. Live Lua/Rust
animation comparison is exact across 105 samples. All three visual captures
complete, and both boss models render. The first two images are pixel-identical
to the pre-port build on the same renderer; both versions slightly exceed the
old 1% limit. The third image meets its 3% limit. No goldens or limits changed;
the renderer gate and visual suite remain red.

Custom CUS tick and detach callbacks receive Spring explicitly, just like
invoke and named calls. Addons can optionally use `create: make_state` when
state construction needs engine resources. It receives Spring and returns
`Result<State>`; existing addons can keep `Default` construction.

All seven Cabal game-code ports are locally committed. The other game-code
port passes Linux gameplay, rendering and full HUD controls, plus actual
menu launch/return on both platforms. Its Linux/Wine match passes 901 frames
with movement, production, formations and ownership checks, comparing engine
and full Wasm memory checksums on every frame. A second 901-frame run passes
the actual HUD pause/speed controls and host exit confirmation, with the same
sync checks. Discovery, rooms, and reconnect UI remain separate work.
No private game content has been uploaded or released.
A live native/Wasm `All` fixture also tests terrain mutation and drawing in one
module, two launches, shutdown, and native loading-thread handoff
([results](data/scoped-sdk-2026-10-17/all-results.json)). Consumer ports
are still in progress; the fixture is not a claim that every game has migrated.
The Windows engine and same-source menu/game fixture pass under Wine,
including `All`, loading-thread handoff, live CUS, ordered job results and a
separate Wasm worker module. Native Windows hardware remains untested.
Packaging keeps using independently built engine releases.

The ordinary engine fixes are also prepared on `ship-game-multiplayer`, without
the new SDK: worktree version tracking, optional Windows storage diagnostics,
rules-param visibility outside UI, shutdown before host services disappear,
strict-Clippy fixes in the legacy Wasm adapters and command helper, and valid
path-cache ZIP checksums when movement data is empty. The live fixture checks
those archives with an independent ZIP reader and rejects warnings/errors.
Its 38 engine suites, strict generation and legacy Wasm Clippy pass.

An isolated copy of the recorded Mesa `.2` renderer removes the Cabal renderer
mismatch without changing system packages or expectations. Flove still fails
four of nine comparisons (5.1–5.7%, limit 5%), consistent with the pre-port
results. The matching-renderer experiment is not a visual-suite pass.
