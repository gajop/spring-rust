# Recoil Wasm Sandbox & Synced Determinism

## Security goal

Wasm modules are sandboxed engine modules.

They do not receive generic host authority.

By default there is no:

- WASI;
- filesystem access;
- sockets/network;
- process execution;
- environment variables;
- host clocks;
- host entropy;
- native pointers;
- arbitrary native-library loading.

A module can only use Spring/engine functionality explicitly imported for its execution environment.

A Spring VFS API, if exposed, is an engine API and must be bounded/mediated appropriately; it is not equivalent to giving the guest OS filesystem access.

These restrictions apply to Wasm regardless of where the module came from.

## Native modules

Native modules are outside this sandbox.

If native module loading is disabled, the native-code execution path is absent. Any policy for enabling native modules is a separate engine design and is intentionally not specified here.

## Pre-instantiation validation

Before compiling/instantiating a module:

- cap module/component byte size;
- compute/verify module identity hash;
- validate allowed Wasm proposals/features;
- bound component/core-module nesting and import/export counts;
- inspect required environment/world;
- reject unknown imports;
- verify API/version compatibility;
- configure deterministic memory/resource limits.

Compile only after validation.

## Wasm feature policy

For synced execution, explicitly configure/disable features that could introduce unsupported nondeterminism or resource behavior.

Initial conservative policy:

- no threads/shared memory;
- no WASI;
- no relaxed SIMD until proven deterministic for supported targets;
- deterministic NaN handling/canonicalization where required;
- fixed runtime feature configuration across peers.

The exact allowed Wasm feature set is part of the synced module identity/configuration contract.

## Memory

Each instance has its own linear memory.

Synced and unsynced instances never share mutable guest memory.

For synced modules:

- configure deterministic maximum memory;
- pre-reserve where practical so `memory.grow` success does not depend on local machine pressure;
- bound variable-size host results;
- do not expose host memory pointers.

## Execution budget

Wasm instruction fuel alone does not bound work caused inside Spring host APIs.

Use deterministic per-instance budgets covering:

- guest instructions;
- host-call base cost;
- result-size/work cost where appropriate;
- callbacks/re-entry.

Do not use wall-clock time as the synced execution budget.

Phase 2 must determine the exact safe integration with Wasmtime fuel/canonical lowering rather than assume that debiting fuel inside a host callback has the desired fault point.

## Host-call validation

Every generated host adapter validates guest-controlled data before using it.

Examples:

- integer ranges;
- enum values;
- list/string lengths;
- handle ownership;
- buffer/result limits;
- environment applicability.

Centralize conversion/validation helpers and fuzz them heavily.

## Synced execution model

A synced Wasm instance is deterministic when all peers have the same:

- module bytes/hash;
- execution environment;
- Wasmtime version/config;
- feature set;
- memory/stack/resource limits;
- fuel/work cost schedule;
- engine state/inputs;
- module order.

Synced environments receive full simulation state, as their Lua gadget equivalents do. Per-player local views are not valid inputs to lockstep synced execution.

## Floating point

Core Wasm arithmetic is much more constrained than arbitrary native FP, but determinism still requires an explicit policy.

Requirements:

- identical runtime feature configuration;
- no relaxed SIMD until validated;
- canonicalize/handle NaNs consistently where observable;
- retain the engine's required FPU environment when entering JIT guest code;
- assert/check the engine FP control state in debug/sync builds;
- audit synced host math APIs that call engine/platform math.

Guest-internal libm code is part of the Wasm module bytes; host-side derived math remains an engine determinism concern.

Cross-platform tests should cover:

- NaN/min/max behavior;
- denormals;
- signed zero;
- sqrt edge cases;
- float-to-int boundary/trapping/saturating conversions.

## Randomness

Synced gameplay randomness must come from deterministic engine-provided state/RNG or deterministic guest algorithms seeded from agreed simulation state.

Do not expose OS entropy to synced modules.

## Host API determinism

Generated APIs used by synced modules must produce deterministic results.

Audit for:

- iteration order;
- maps/sets with unstable ordering;
- filesystem/environment dependencies;
- local clock/input state;
- platform-dependent host math;
- handle allocation if observable;
- local resource failures.

A failure dependent on local machine conditions must not masquerade as a deterministic guest trap.

## Sync checking

Do not hash the entire Wasm linear memory every frame.

Use existing engine sync checking for simulation state mutations.

Optionally include a full guest-memory/global digest in `SYNCDEBUG`-style tooling for diagnosing a divergence.

If a guest return value directly controls synchronized simulation without passing through an existing synced primitive, consider including that observable decision in sync checking.

## Traps/faults

A deterministic trap in synced Wasm is match-fatal for the initial design.

Reason: a trapped guest may have partially updated its own private state, so continuing to execute it cannot be assumed safe.

A future alternative is synchronized module disable at the same simulation frame on every peer.

Unsynced module faults can disable/unload that instance without affecting other instances.

## AOT cache safety

Serialized Wasmtime artifacts are native executable code.

Safest default:

- no AOT deserialization for content-supplied Wasm modules initially.

If AOT caching is enabled:

- artifacts must be generated by the host;
- select by module hash + exact Wasmtime/compiler/runtime configuration;
- store outside content-writable game data, or authenticate with a host-held key stored outside content-reachable storage;
- authentication failure is a cache miss;
- never deserialize guest-supplied artifacts.

## Security tests

Required adversarial coverage:

- denied/unknown imports;
- oversized modules/components;
- pathological component nesting;
- invalid strings/lists/enums/handles;
- hostile canonical allocator behavior;
- allocator re-entry into Spring;
- allocator trap/fuel burn;
- callback re-entry;
- resource/handle use-after-drop;
- memory/result exhaustion;
- VFS escape attempts;
- AOT-cache tampering;
- fault cleanup with multiple instances.

## Synced acceptance tests

Run the same synced Wasm scenario across the engine's supported sync-test platforms, including the existing Linux x86_64, Linux arm64, and Windows x86_64 matrix.

Require identical simulation sync results for:

- normal gameplay;
- RNG;
- FP edge cases;
- resource-limit boundaries;
- repeated module load/order;
- callback/callout activity.
