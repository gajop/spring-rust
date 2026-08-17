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

UNVERIFIED-EXTERNAL
    Implemented and locally checked, but platform/CI execution is unavailable
    on this host.

Current snapshot
----------------

The core runtime, generator, visibility layer, all five environments, and
parity harness are implemented. Phases 0–9 have passed their local gates,
including UI runtime parity, the five-context Wasm matrix, rendering parity,
the non-ASAN frame-budget gate, and the final ASAN build/CTest run. Phase 10
has its local implementation and Linux build gates complete; platform/CI
execution remains host-limited validation, not a design deferral.

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
* The ASAN CTest run passes 38/38 registered tests; `test_WasmInterface`
  passes 774 assertions in 35 cases, and the isolated allocator re-entry,
  trap, and fuel tests pass 5, 5, and 6 assertions.
* Component resource transfer validates all pending fields before committing
  ownership; the focused fixture covers valid, invalid, stale, and
  post-failure resource use.

Phase 3 — WIT/callouts/worlds          DONE
--------------------------------------

* Rules/Gaia synced and unsynced worlds, plus the UI world, are generated from
  the canonical model. UI runtime loading is enabled in Phase 8.
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

Phase 8 — LuaUI environment            DONE
--------------------------------------

* LuaUI loads an optional LuaUI/wasm/manifest.txt and dispatches the UI world.
* UI host callins receive an owned, visibility-filtered copy under the local
  player/team/ally-team context, including degraded attacker and LOS/radar
  fields.
* The UI probe is generated from the widget metadata: 56 selected rows account
  for all 66 widget rows. Seven mutating order calls, two expected-error rows,
  and one rendering-only control are explicit exclusions.
* UI WIT, Rust bindings, probe source, and component compile successfully.
* The scripted UI runtime comparison passes 56/56 typed rows with zero vacuous
  rows, and the rendering parity run covers the widget stream and UI callins.
* UI visibility, degraded-value, local-context, and callin filtering checks
  pass in the generated fixture and the full rendering harness.

Phase 9 — Full Lua/native/Wasm parity DONE
-------------------------------------

* Native/Wasm normalized signatures, the archive-loaded Wasm fixture, the
  observation stream, and the parity harness are implemented.
* The five-context matrix is wired: Rules synced/unsynced, Gaia
  synced/unsynced, and UI.
* Final typed probes pass with zero vacuous rows: 106 unsynced gadget, 342
  synced gadget, 342 Gaia synced, 106 Gaia unsynced, and 56 UI rows.
* Rendering-enabled Lua/native parity passes with synced 1,854/1,854,
  unsynced 841/841, widget 231/231, all 149 deterministic callins covered on
  both sides, and all Gfx/RmlUi checks passing.
* This phase is an original-plan deliverable and is not DEFERRED.

Phase 10 — CI/build/release            UNVERIFIED-EXTERNAL
---------------------------------------

* CMake wiring, Wasmtime acquisition, generator/snapshot checks, Rust guests,
  allocator fixtures, ASAN test wiring, parity harness wiring, benchmark gates,
  and cross-platform workflow files are present.
* The amd64 Windows MinGW path selects/packages the Wasmtime DLL/import library
  for the engine. Local Windows runtime/ASAN tests are not available in the
  supplied image.
* The x86_64 host cannot execute the arm64 Docker image; native arm64 CI is the
  appropriate execution gate.
* The exact Linux ASAN build and full CTest pass, and the non-ASAN RELEASE
  `wasm-performance` target passes the frame-derived budgets.
* The committed workflow files provide the arm64/Windows synced parity matrix.
  Those external jobs are not executable on this host; this is validation
  pending for committed platform code, not missing local implementation.

Verification record
-------------------

Observed in the current pass:

* verify_codegen.py — PASS after UI integration; 55 modules, 1,353 functions,
  zero unsupported functions, and reproducible probes for all five contexts.
* All five parity guest components compile, including the UI component.
* Python syntax checks and generated-artifact diff checks — PASS.
* The exact ASAN Docker build and full CTest pass after the final UI/probe
  changes.
* Native/Lua parity, all five Wasm contexts, rendering parity, and the
  non-ASAN performance gate pass after the final generated artifacts.
* Windows engine-headless cross-build/package with tests disabled — PASS in the
  earlier platform pass.
* arm64 runtime and Windows runtime/ASAN CTest — not executable on this host;
  CI-only.

Explicit design deferrals
-------------------------

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

No required local implementation or test work remains. The arm64/Windows CI
jobs are the only remaining validation actions because their execution
environments are unavailable on this host.

The separate FreeType and SDL/OpenAL teardown entries remain engine TODOs.
No interactive input was used; no Spring process was launched during this
bookkeeping pass.

AI disclosure
-------------

Codex (GPT-5) generated and edited the implementation, artifacts, tests, CI,
and this ledger. Human review and verification remain required by the
repository AI policy.
