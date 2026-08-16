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

## Executive status

The non-deferred implementation and its Linux runtime gates are complete. The
required ASAN Docker build, the full 38-test CTest suite, the Rust workspace,
the generator reproducibility checks, native/Lua parity, and all final Wasm
parity contexts pass. There is no known core Linux build, runtime, or parity
blocker.

The only items below that are not locally verified are platform executions that
the x86_64 host cannot perform (arm64 and Windows runtime/ASAN CI). The other
remaining entries are explicit original-design deferrals or separate engine
teardown TODOs; none is unfinished Wasm implementation work.

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
- Non-ASAN performance measurements were repeated and the gates were derived
  from simulation/UI frame budgets; ASAN reports the timings without enforcing
  those budgets.
- The required ASAN-configured Docker build completed successfully, including
  the engine, generated adapter, Wasm runtime, and Wasm test targets.

## Completed verification

- `./docker-build-v2/build.sh linux -DUSE_ASAN=ON` — PASS.
- `./docker-build-v2/build.sh --compile linux -t check` — 38/38 CTest entries
  PASS, including 713 `test_WasmInterface` assertions and isolated allocator
  re-entry, trap, and fuel tests.
- `cargo test --manifest-path rust/Cargo.toml --workspace` and Rust formatting
  — PASS; native snapshot check and Python syntax checks — PASS.
- `python3 rts/wasm/verify_codegen.py` — PASS. The generator produces 55
  modules, 1,353 functions, and zero unsupported functions; repeated parity
  fixture generation is byte-for-byte stable.
- Native/Lua API parity — PASS: headless streams 636/636, 180/180, and 71/71;
  rendering streams 638/638, 285/285, and 101/101. The rendering run covered
  all 149 deterministic callins on both sides with matching arguments and
  returns, plus all Gfx/Rml checks.
- Wasm parity — PASS: synced 314/314, unsynced 81/81, and rendering-enabled
  unsynced 81/81 typed probes. Every run reports zero vacuous rows and passes
  fixture discovery and deterministic Component observations.
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

## Deferred by the original design

These are deliberate design boundaries, not leftover implementation work:

- LuaUI Wasm runtime loading (Phase 9), pending LuaUI-specific local-player
  visibility, LOS/radar, degraded-value semantics, UI callins, and complete
  LuaUI↔Wasm RmlUi parity.
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

## Host-limited validation

- The current host is x86_64, so the arm64 Docker image cannot execute locally;
  the arm64 CI runner is the appropriate validation environment.
- The MinGW cross-build can build the engine and package the Wasmtime DLL, but
  the supplied Windows image does not provide the local ASAN test prerequisites
  needed for a Windows ASAN CTest result. Windows runtime execution remains a
  CI concern.
- The final runtime runs used only the existing scripted harness and its
  automatic quit path; no mouse, keyboard, or ad-hoc process control was used.
  No Spring process remains running from those checks.

## Remaining-work estimate

For the non-deferred local scope, there is no remaining implementation work.
The only follow-up is to let CI execute the already-committed arm64 and Windows
platform gates. The original-design deferrals and engine teardown TODOs are
listed separately and are not blockers for this Wasm delivery.

AI disclosure: Codex (GPT-5) generated and edited the implementation,
artifacts, tests, CI, and this report. Human review and verification remain
required by the repository AI policy.
