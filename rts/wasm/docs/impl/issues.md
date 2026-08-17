# Wasm implementation status and remaining work

Date: 2026-08-17

This is the short, honest status report for the implementation plan in
`recoil_wasm_implementation_plan.md`. The old wording was too optimistic and
used “plan-scoped” without explaining whether an item was unfinished. This
file uses four explicit categories instead:

- **Implemented** — code is present; the listed evidence is available.
- **Implemented, verification pending** — code is present, but a final test
  or runtime run has not yet been repeated after the latest changes.
- **Deferred by the original design** — intentionally not part of this
  delivery, as stated in the original documents.
- **Separate engine TODO** — unrelated to Wasm and not a Wasm blocker.
- **Unverified external validation** — local implementation is present, but
  this host cannot execute the relevant platform or CI job.

## Executive status

The Wasm runtime, generator, visibility layer, all five local environments,
and the parity harness are implemented. Phases 0–9 of the original plan have
passed their local gates, including the UI runtime comparison, the full
Lua/native/Wasm matrix, rendering parity, the non-ASAN performance gate, and
the final ASAN build/CTest run. There is no remaining local Wasm
implementation blocker.

The only unverified items are platform/CI executions that cannot run on this
x86_64 host or in the supplied Windows image. They are validation of committed
code, not silently unfinished implementation. The SDL/OpenAL and FreeType
entries below are separate engine teardown TODOs, also outside this Wasm
delivery.

## Implemented

- The generator consumes the canonical native snapshot and produces 55
  modules, 1,353 functions, and zero unsupported functions. Annotation typos
  and duplicates now fail generation instead of being silently ignored.
- Generated signature comparison now detects missing and extra functions,
  query/result drift, record/enum drift, environment drift, mutability and
  visibility drift, lowering-status drift, and callin inventory drift.
- The signature gate is a round trip: it compares the model independently with
  parsed WIT, emitted native Rust, and Lua-extracted signatures. The generated
  `signatures.json` artifact is not a model echo.
- Execution environments come from the repository Lua loader registrations and
  the model's mutating flag. Generator assertions reject mutating functions in
  unsynced/UI worlds, empty interfaces, and interfaces imported by no world.
- Generated C++/WIT/Rust artifacts cover the automatic, annotated, and
  explicitly manual callout paths. Registry-driven tests ensure manual
  entries reach reviewed native fallback code and that canonical callins have
  serializers.
- The host adapter is split into one common file plus one translation unit per
  generated module, and CMake compiles the split set in both engine consumers.
- The Wasmtime 42 Component Model runtime contains the tested scalar, string,
  bytes, list, record, tuple, option, result, enum, flags, variant, and
  resource lowering paths. Limits, fuel, result-size, nesting, import/export,
  memory/table, handle ownership, callback policy, and re-entry checks are in
  place.
- Core Wasm imports now fail closed: only the registered `spring.add-i32`
  host function is accepted. WASI and other operating-system authority remain
  denied. Wasmtime NaN canonicalization is enabled and included in runtime
  identity.
- Component imports are prevalidated against the generated callable inventory
  in production builds. Named Component type exports are accepted as types;
  they are not callable host capabilities. Synthetic lowering fixtures use a
  compile-time unit-test-only exception and cannot enable it in production.
- Module manifests, archive identity, deterministic ordering, atomic batch
  loading, synced configuration identity, lifecycle cleanup, callback
  ownership, and synced fault handling are implemented.
- RmlUi listener removal and data-model event cleanup are wired at the ABI and
  adapter boundary. The module disables callback lifetime before cleanup and
  releases registered subscriptions in reverse order.
- The unsynced parity guest no longer treats an arbitrary number of early
  `Update` calls as readiness; it gates its observation on the engine game
  frame. Synced, unsynced, and rendering-enabled Wasm runs confirmed the fix.
- Component resource transfer validates a complete pending transfer before
  committing ownership, so a later nested-field failure cannot consume a valid
  host resource. The focused resource fixture covers valid, invalid, stale,
  and post-failure use.
- The focused RmlUi callback-lifetime test covers context listeners, element
  listeners, and data-model event bindings during module cleanup.
- LuaUI Wasm loading is enabled from the optional LuaUI manifest. UI dispatch
  uses an owned copy of each callin payload, a LuaUI-specific visibility
  context, object/event filtering, and degraded attacker/LOS/radar fields.
- The UI parity component is generated from the widget API metadata, covers 56
  selected rows, and accounts for all 66 widget rows. The ten exclusions are
  seven mutating order calls, two expected-error cases, and one rendering-only
  control.
- Non-ASAN performance measurements were repeated and the gates were derived
  from simulation/UI frame budgets; the RELEASE gate measured 461 ns scalar,
  5,496 ns shaped Component result, 18 ns callback, and 232 ns Gfx callout,
  against 2,000/6,000/500/1,000 ns budgets. ASAN reports these timings without
  enforcing the RELEASE budgets.
- The exact requested `./docker-build-v2/build.sh linux -DUSE_ASAN=ON` build
  completed after the final UI/probe changes, and the in-container CTest gate
  passed all 38 registered tests.

## Verification completed in the current pass

- `./docker-build-v2/build.sh linux -DUSE_ASAN=ON` — PASS after the final
  source and generated-artifact changes.
- `./docker-build-v2/build.sh --compile linux -t check` — 38/38 CTest entries
  PASS. `test_WasmInterface` passed 774 assertions in 35 cases; the isolated
  allocator cases passed 5, 5, and 6 assertions respectively.
- `verify_codegen.py` — PASS: 55 modules, 1,353 functions, zero unsupported
  functions, and reproducible probes for Rules/Gaia synced/unsynced plus UI.
- Rust/Python static checks, allocator guest tests, componentization, embed
  generation, formatting, and generated-artifact reproducibility — PASS.
- Non-ASAN RELEASE `wasm-performance` target — PASS with the measurements
  recorded above.
- Native/Lua API parity — PASS in
  `test/native_api_parity/out/20260817-114122-756465`.
- Wasm parity — PASS for all five contexts, with typed probe rows
  `unsynced_gadget 106`, `synced_gadget 342`, `gaia_synced 342`,
  `gaia_unsynced 106`, and `ui 56`; each had zero vacuous rows and matching
  result streams. The runs are recorded in output directories
  `20260817-112739-000382`, `20260817-113001-172475`,
  `20260817-113114-668220`, `20260817-113226-056795`, and
  `20260817-113445-446452`.
- Full rendering parity — PASS in
  `test/native_api_parity/out/20260817-114242-874115`: synced 1,854/1,854,
  unsynced 841/841, widget 231/231, 149/149 deterministic callins on both
  sides, and all Gfx/RmlUi checks passing.
- The amd64 Windows MinGW engine-headless cross-build and package completed
  with tests disabled because the supplied image lacks its test prerequisites.
- `.gitignore` excludes generated JSON and harness targets; `.gitattributes`
  marks generated artifacts while leaving WIT readable; the reproducibility
  gate covers the generated Wasm probe as well as the core output.

## Implemented, verification pending (host/CI-only)

These are external validation limits, not unfinished local implementation:

- The arm64 Docker image cannot execute on this x86_64 host. The committed
  cross-platform workflow runs the synced Wasm fixture on amd64 Linux, arm64
  Linux, and amd64 Windows when the corresponding engine artifacts and CI
  runners are available.
- The supplied Windows image built the engine successfully, but did not have
  the PNG and sanitizer prerequisites for Windows ASAN CTest or local runtime
  execution. Those remain CI checks.
- A formal GitHub Actions YAML linter is not installed on this host; the
  workflow scripts and path coverage were inspected, while GitHub remains the
  authoritative workflow validator.

## Unverified external validation

These are not local implementation gaps:

- The arm64 Docker image cannot execute on this x86_64 host. The committed
  workflow is the validation path for the arm64 synced parity job.
- The supplied Windows image built and packaged the engine, but does not have
  the prerequisites for Windows ASAN CTest or local runtime execution. Those
  jobs remain CI validation.
- A formal GitHub Actions YAML linter is not installed on this host; the
  workflow was inspected and GitHub remains the authoritative validator.

## Deferred by the original design

These are deliberate design boundaries, not leftover implementation work:

- WASI and general operating-system imports.
- Content-supplied Wasmtime AOT deserialization. The original design leaves
  host-generated authenticated caching optional and rejects untrusted native
  code.
- Native opaque archive pointers in Wasm payloads; archive access uses the
  explicit archive-aware interface instead.
- Native-module trust policy, LuaMenu, LuaIntro, LuaParser/definition parsing,
  and RmlUi surfaces outside the supported in-game UI path.

## Separate engine TODOs

`test/native_api_parity/lsan.supp` intentionally keeps the known NVIDIA/DBus
process-lifetime suppressions narrow. It also records TODO suppressions for
FreeType global font-cache allocations and SDL/OpenAL loopback-audio
bookkeeping. Those are engine/third-party teardown issues, outside the Wasm
scope; they should be removed when ownership and teardown are fixed. They do
not justify suppressing arbitrary leaks from Spring or Wasm modules.

## Test operation

All Spring runtime evidence above used the existing parity harness and its
automatic scripted quit path. No mouse, keyboard, or ad-hoc process control
was used.

## Remaining-work estimate

No required local implementation or test work remains. The remaining actions
are the external arm64/Windows CI gates described above; they validate the
committed workflow and platform artifacts and cannot be completed by this
host.

Only the WASI/AOT/LuaMenu/LuaIntro/LuaParser boundaries and the separate engine
teardown TODOs are deferred or out of scope.

AI disclosure: Codex (GPT-5) generated and edited the implementation,
artifacts, tests, CI, and this report. Human review and verification remain
required by the repository AI policy.
