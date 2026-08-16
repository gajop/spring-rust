Recoil Wasm implementation ledger
=================================

Date: 2026-08-17
Repository: spring-bar
Runtime target: Wasmtime 42.0.1 / WIT Component Model
Requested build: ./docker-build-v2/build.sh linux -DUSE_ASAN=ON

Status vocabulary
-----------------

DONE
    Implemented and backed by the evidence listed here.

VERIFY
    Code is present, but the final test or runtime check has not been repeated
    after the latest changes.

DEFERRED
    Intentionally excluded by the original implementation plan.

TODO-ENGINE
    Separate engine or third-party teardown work, outside Wasm scope.

Current snapshot
----------------

The non-deferred implementation and the Linux validation path are complete.
The required ASAN Docker build, full CTest suite, Rust/codegen checks, native
and Wasm parity runs, and rendering/callin checks all pass. VERIFY below is
reserved for platform execution that this x86_64 host cannot perform; it does
not identify unfinished Wasm code.

Phase 0 — baseline generator/tests       DONE
-----------------------------------------

* The native snapshot and semantic generator are reproducible.
* Generator tests cover scalar, string, list/count, optional, nested record,
  enum, fixed-array, callback, handle, annotation, and duplicate/unknown
  declaration cases.
* The parity fixture contains two teams/two ally teams and LOS/radar cases.

Phase 1 — shared semantic API model       DONE
------------------------------------------

* The canonical model generates WIT, Rust SDK, C++ adapter, callin/callout
  registries, signatures, and machine-readable reports.
* Current generated inventory: 55 modules, 1,353 functions, zero unsupported
  functions.
* Signature comparison checks functions, records, enums, environments,
  mutability, visibility, lowering status, and callins. Annotation typos and
  duplicates fail generation instead of being silently ignored.

Phase 2 — Wasmtime/Component proof       DONE
-----------------------------------------

* Wasmtime 42.0.1 is pinned and checksum-controlled.
* Scalar, string, bytes, list, record, tuple, option, result, enum, flags,
  variant, and resource lowering code is implemented and covered by fixtures.
* Runtime limits cover module bytes, memory, tables, imports, exports, value
  nodes, Component nesting, resources, result bytes, host work, and fuel.
* Core imports fail closed; only spring.add-i32 is accepted. WASI is denied.
  NaN canonicalization is enabled and included in runtime identity.
* Production Component callable imports are checked against generated callout
  inventory. Named Component type exports are accepted as non-callable types.
* The ASAN CTest run passes all 713 `test_WasmInterface` assertions and the
  isolated allocator re-entry, trap, and fuel tests.
* Component resource transfer validates all pending fields before committing
  ownership; the focused fixture covers valid, invalid, stale, and
  post-failure resource use.

Phase 3 — WIT/callouts/worlds          DONE
--------------------------------------

* Rules/Gaia synced and unsynced worlds are generated from the canonical
  model. The UI world is described but disabled by the original design.
* All generated and reviewed manual callouts are represented in the registry;
  manual entries reach explicit native fallback code.
* Environment capability checks run before instantiation and at registration.
  Raw pointer-management surfaces are not exposed to Wasm.

Phase 4 — shared Rust module API       DONE
--------------------------------------

* rust/crates/spring-wasm provides owned semantic values, error categories,
  budgets, callback/resource state, and dispatch contracts.
* Generated Rust bindings and the representative guest façade are present.
* Workspace tests and formatting pass after the final generated ABI changes.

Phase 5 — engine runtime/multiple       DONE
instances
----------------------------------------

* WasmRuntime, WasmModule, WasmInterfaceSystem, WasmEnvironment, WasmDispatch,
  WasmResources, and WasmModuleManifest are integrated.
* Module IDs, ordering, hashes, lifecycle state, callback/resource cleanup,
  archive identity, atomic manifest loading, and synced configuration identity
  are implemented.
* Synced dispatch faults are handled separately from unsynced module faults.
* The full ASAN CTest suite passes after the final security/lifecycle changes.

Phase 6 — callbacks/RmlUi/Gfx/shapes   DONE
--------------------------------------

* Callback IDs, ownership, controlled re-entry, adapter thunks, RmlUi
  listener/data-model cleanup, Gfx paths, and special/manual lowering paths
  are implemented.
* New RmlUi removal ABI operations are appended to preserve existing field
  offsets. Module cleanup disables callback lifetime first and unregisters
  subscriptions in reverse order.
* The focused RmlUi callback-lifetime test covers context listeners, element
  listeners, and data-model event bindings during module cleanup.

Phase 7 — sandbox/sync                  DONE
---------------------------------------

* WASI, untrusted AOT, unsupported core imports, and non-deterministic host
  authority are denied by policy.
* Memory/table/import/export/resource/value/fuel/host-work limits, recursive
  nesting, handle ownership, callback policy, import re-entry, canonical
  result fuel, hostile allocator paths, and cleanup are implemented.
* The Linux post-change direct-test and synced Wasm evidence pass. The
  arm64/Windows executions remain CI-only because of the host/image limits
  recorded below.

Phase 8 — Lua/native/Wasm parity       DONE
--------------------------------------

* Native/Wasm normalized signatures, the archive-loaded Wasm fixture, the
  observation stream, and the parity harness are implemented.
* Final native/Lua headless parity passes with streams 636/636, 180/180, and
  71/71. Rendering parity passes with streams 638/638, 285/285, and 101/101.
* The rendering run covers all 149 deterministic callbacks on both sides with
  matching arguments and returns, plus all Gfx/Rml checks.
* Final Wasm parity passes synced 314/314, unsynced 81/81, and rendering
  unsynced 81/81 typed probes. All runs report zero vacuous rows and pass
  fixture discovery and deterministic Component observations.
* The final runs use only the scripted fixture and automatic quit path; no
  mouse, keyboard, or ad-hoc process control is part of the gate.

Phase 9 — LuaUI environment           DEFERRED
-------------------------------------

* LuaUI Wasm runtime loading remains disabled by the original design.
* The design requires LuaUI local-player visibility, LOS/radar,
  degraded-value semantics, UI callins, and LuaUI↔Wasm RmlUi parity before
  enabling this environment. This is an intentional future phase, not an
  unfinished Phase 0–8 implementation item.

Phase 10 — CI/build/release            VERIFY
---------------------------------------

* CMake wiring, Wasmtime acquisition, generator/snapshot checks, Rust guests,
  allocator fixtures, ASAN test wiring, parity harness wiring, benchmark gates,
  and cross-platform workflow files are present.
* The amd64 Windows MinGW path selects/packages the Wasmtime DLL/import library
  for the engine. Local Windows runtime/ASAN tests are not available in the
  supplied image.
* The x86_64 host cannot execute the arm64 Docker image; native arm64 CI is the
  appropriate execution gate.
* The Linux ASAN CTest and static verification pass. The committed workflow
  files provide the remaining arm64/Windows synced parity matrix; those jobs
  are CI-only validation, not missing local implementation.

Verification record
-------------------

Observed in the current pass:

* `./docker-build-v2/build.sh linux -DUSE_ASAN=ON` — PASS.
* `./docker-build-v2/build.sh --compile linux -t check` — 38/38 CTest entries
  PASS.
* `cargo fmt --manifest-path rust/Cargo.toml --all --check` and
  `cargo test --manifest-path rust/Cargo.toml --workspace` — PASS.
* `python3 rts/wasm/verify_codegen.py` — PASS; 55 modules, 1,353 functions,
  zero unsupported functions, and reproducible generated probe output.
* `python3 rust/crates/spring-native-codegen/snapshot_native.py --check` and
  Python syntax checks — PASS.
* Native/Lua and Wasm parity reports listed under Phase 8 — PASS.
* Windows engine-headless cross-build/package with tests disabled — PASS.
* arm64 runtime and Windows runtime/ASAN CTest — not executable on this host;
  CI-only.

Explicit design deferrals
-------------------------

* LuaUI runtime loading and its role-specific parity work.
* WASI/general operating-system imports.
* Content-supplied AOT deserialization.
* Native opaque archive pointers in Wasm payloads.
* Native-module trust policy, LuaMenu, LuaIntro, LuaParser/definition parsing,
  and RmlUi surfaces outside the supported in-game UI path.

Separate engine TODOs
---------------------

test/native_api_parity/lsan.supp contains narrow NVIDIA/DBus process-lifetime
rules and TODO rules for FreeType global font-cache allocations and SDL/OpenAL
loopback-audio bookkeeping. The latter two are engine/third-party teardown
issues, outside the Wasm implementation; remove them when ownership is fixed.

Remaining-work estimate
-----------------------

For the non-deferred local scope, there is no remaining implementation work.
The only follow-up validation is external platform execution, not another
1,000-function handwritten implementation:

1. Run the committed cross-platform workflow on its arm64 Linux and Windows
   runners when engine artifacts are available.
2. Address the separate FreeType and SDL/OpenAL teardown TODOs in the engine.

There is no remaining non-deferred Wasm implementation task in this worktree.
Spring runtime validation was performed through the existing scripted harness;
no interactive input or ad-hoc process control was used.

AI disclosure
-------------

Codex (GPT-5) generated and edited the implementation, artifacts, tests, CI,
and this ledger. Human review and verification remain required by the
repository AI policy.
