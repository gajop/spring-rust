# Remove `UnsupportedHostTarget`

Date: 2026-08-22
Standalone task. Independent of `next_agent_handoff_2.md`; do this next.

## Status

Completed in the current follow-up.

- `ErrorCode::UnsupportedHostTarget` removed from the Rust API and generated
  SDKs.
- Owned generation omits missing lowerings instead of emitting runtime-failing
  stubs.
- Probe generation excludes absent owned symbols with an explicit manifest
  reason.
- `spring-wasm-core` import modules are `wasm32`-only and excluded from the
  native Rust workspace; three pure packing tests remain host-runnable.
- `verify_codegen.py`, workspace tests, direct host codec tests, and the Core
  wasm guest check are green.
- The old 574 fallback count is no longer emitted; 331 synced cases are
  selected.

## The problem

The former `ErrorCode::UnsupportedHostTarget = -1`
(`rust/crates/spring-wasm-core/src/lib.rs`) is an anti-pattern. It turns
"nobody generated this wrapper" into a runtime failure in a modder's game.

**Nothing it reports is actually unsupported.** All 1354 callouts have a
working host binding and a working raw guest entry point. Example:
`GetTeamUnitsByDefs` is live in `WasmCoreUnitsQueryBorrowedBindings.cpp` and
callable through `units_query_borrowed.rs::get_team_units_by_defs_into`, while
the owned façade returns this error instead of calling it. The gap is generator
coverage, not capability.

Count across `rts/wasm/generated/sdk/*.rs` and
`rust/crates/spring-wasm-core/src/*.rs`:

| | count | cause |
| --- | ---: | --- |
| `#[cfg(not(target_arch = "wasm32"))]` arms | 1,470 | crate is compiled for the host, where wasm imports do not exist |
| unguarded stubs | 2,435 | wrapper was never generated |
| total | 3,905 | |

## Goal (met)

The code and generated artifacts contain no `UnsupportedHostTarget` reference.
This task document retains the historical name so the removal decision remains
searchable. Wasm imports are target-gated; host builds retain pure packing and
codec helpers and do not expose import wrappers.

## Part A — generate the missing wrappers

The generic owned renderer now emits no placeholder for a shape it cannot
lower. The generated coverage report records the remaining shape and reason;
those symbols are absent by design until a real adapter is added. The
environment-scoped renderer follows the same omission rule.

Groups, per `core_parity_handoff.md` §4: dynamic/recursive output decoders,
variable-input descriptors, command and piece record lists, rules and
configuration string/list APIs, mutating control callouts. Smallest group
first; rerun both synced parity contexts after each.

**Rule: never emit a stub that fails at runtime.** If a wrapper genuinely
cannot be generated, emit nothing and record the reason in the generation
report. A missing symbol is a compile error the user sees immediately; a
runtime error is a bug shipped to a modder.

Generator work only — never hand-edit generated files.

## Part B — stop compiling guest bindings for the host

Implemented: `spring-wasm-core` is excluded from the native Rust workspace;
its import modules and generated bindings are gated to `wasm32`. The host
package still runs the packing/layout tests in `src/lib.rs`.

Root cause of the 1,470 guarded arms: `spring-wasm-core` is a workspace member,
so `cargo build --workspace` compiles it for the host, where the wasm imports
are absent. A runtime error was chosen instead of not compiling.

Split the crate:

- **pure codec/helper logic** (`decode_fill`, packing/unpacking, descriptor
  layout math) stays host-compilable and keeps its unit tests — that is real
  test value, do not lose it;
- **import wrappers** become `wasm32`-only: gate the modules, drop the crate
  from the host workspace build, build it for `wasm32-unknown-unknown` only.

Every `#[cfg(not(target_arch = "wasm32"))]` arm then disappears.

## Part C — delete the code

`ErrorCode::UnsupportedHostTarget` and every code/generated reference are
removed. If any case
still needs an error afterwards, it is a real error and gets its own named code
with a real meaning — not this one.

Also drop the corresponding section from the generated API reference
(`rts/wasm/docs/generated/core_api_reference.md`); there will be nothing to
list.

## The in-flight environment SDK

Commit `6abf6e1e9b` produced `rts/wasm/generated/sdk/core_environments.rs`; its
renderer now applies the same omission rule and the file contains no sentinel
sites. It remains a generated-sharding follow-up from `refactor_plan.md`.

That file is also larger than everything it replaced. Check it against the
sharding work in `refactor_plan.md` before it settles.

## Done when

- `rg UnsupportedHostTarget rust rts/wasm/generated test/wasm_api` is empty
- every callout is callable through the owned façade, or is absent by design
  with a recorded reason
- both synced parity contexts still pass, with selected-case counts higher than
  before
- `verify_codegen.py` green, workspace tests green, engine builds
