# Wasm implementation status

This file is a status index, not a measurement log. See
[benchmarking_results.md](benchmarking_results.md) for generated benchmark
values and [review-response.md](review-response.md) for the review mapping.

## Implemented

- Canonical model generation for WIT, Rust, native adapters, registries, and
  signatures.
- Capability-derived synced, unsynced, Gaia, and LuaUI worlds.
- Wasmtime component runtime, limits, resource ownership, callbacks, cleanup,
  import validation, and re-entry protection.
- Native/Wasm parity harness, independent signature checks, fixture discovery,
  visibility filtering, and reproducible probe generation.
- LuaUI loading and owned visibility-filtered dispatch.
- Deterministic synced timing and opt-in benchmark instrumentation.
- Automatic generation of the benchmark matrix.
- Direct synced-to-unsynced messaging coverage.
- Active Lua-loader registrations are audited against the documented surface,
  with compatibility aliases recorded as explicit exclusions.

## Required implementation work

These are incomplete original-plan deliverables, not intentional deferrals:

- Complete broad API and mutation-heavy end-to-end coverage.
- Expand the bounded differential conversion corpus across the remaining
  shared lowering families.
- Complete broad post-change parity coverage and platform gates where
  available.
- Commit the finished change set.

## Verification unavailable locally

- arm64 runtime execution requires an arm64 runner.
- Windows runtime and sanitizer execution require the Windows CI environment.

## Deferred by the original design

- WASI/OS authority, content-supplied AOT deserialization, and opaque native
  archive pointers in Wasm values.
- Native-module trust policy, LuaMenu, LuaIntro, LuaParser/definition parsing,
  and unsupported RmlUi surfaces.

## Separate engine TODOs

FreeType and SDL/OpenAL teardown/leak ownership issues remain outside this Wasm
implementation. Their narrow suppressions are explicitly TODOs, not a blanket
Wasm leak policy.
