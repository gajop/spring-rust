# Recoil Wasm Parity & Testing

## Goals

Wasm should be checked the same way the native Rust work is intended to be checked:

1. API/signature equivalence;
2. runtime result parity;
3. synced cross-platform equivalence.

These should become hard automated gates.

## 1. Generator regression baseline

Before refactoring `spring-native-codegen`:

- generate the current native Rust output deterministically;
- commit a golden snapshot under a generated/test location;
- add small synthetic-header tests for every semantic pattern;
- regenerate + diff in CI;
- make silent function omission an error.

Normal downstream module builds should consume committed/generated Rust output rather than require libclang themselves. Regeneration is a maintainer/CI operation.

## 2. Normalized signature model

Do not extend the existing regex/difflib signature comparison into a three-way parser.

Instead emit one normalized machine-readable signature artifact from each representation.

Conceptually:

- Lua API extraction -> normalized API JSON;
- NativeInterface/shared model -> normalized API JSON;
- generated Wasm/WIT -> normalized API JSON.

Compare semantic signatures rather than transport syntax.

Normalize:

- names;
- scalar types;
- records;
- flattened vectors;
- strings;
- lists;
- options;
- errors;
- callbacks;
- environment applicability.

## 3. Native ↔ Wasm signature gate

Run this during module-by-module Wasm generation, not only after the full surface is implemented.

For every generated interface:

- no missing function;
- no extra function;
- same semantic inputs;
- same semantic outputs;
- same environment applicability;
- explicit exclusions only.

This catches generator bugs before hundreds of functions are rolled out.

## 4. Lua ↔ Native ↔ Wasm signature gate

Once the Lua extractor is normalized to the same model, require all three representations to agree where the APIs are intended to be equivalent.

Keep intentional differences explicit.

## 5. Runtime parity harness

Reuse `test/native_api_parity`.

Do not build a second test philosophy for Wasm.

Extend the harness with a Wasm mode:

- build the representative module for `wasm32`;
- package/load it through the Wasm module path;
- run the same API test specification;
- normalize result values;
- compare Lua/native/Wasm.

Reuse the existing `known_issues.json` / parity-gap mechanism for intentional or unfinished cases.

## 6. Multi-ally/LOS fixture

The parity fixture uses two teams and two ally teams, so it exercises
LOS/radar-dependent behavior in the headless native gate. The fixture keeps
the following cases explicit:

- ally-owned unit;
- enemy unit in LOS;
- enemy unit in radar only;
- enemy unit not detected.

Add parity cases for:

- position;
- health;
- commands;
- definitions/type visibility;
- relevant resource/state reads;
- LOS module functions.

Check degraded/fuzzed values, not only success/failure.

The current Wasm archive fixture covers the component callout/callin
observation path. LuaUI visibility parity is implemented in Phase 8 and its
runtime comparison is part of the full Phase 9 parity gate.

## 7. Execution-environment parity

Compare Wasm against the matching Lua role:

- Rules synced ↔ LuaRules synced;
- Rules unsynced ↔ LuaRules unsynced;
- Gaia synced ↔ LuaGaia synced;
- Gaia unsynced ↔ LuaGaia unsynced;
- UI ↔ LuaUI through the enabled UI world and the widget parity fixture.

Do not compare a Gaia/Rules module to widget visibility semantics.

## 8. Adversarial boundary tests

Generate and hand-write tests around the small number of shared lowering mechanisms rather than one-off fuzzing per function.

Required targets:

- strings;
- lists;
- nested records;
- enums/options/results;
- handles/resources;
- callback registry;
- canonical allocator;
- re-entry guard;
- result limits;
- error conversion.

Differential fuzzing can compare native semantic conversion and Wasm conversion for the same generated type pattern.

## 9. Synced determinism tests

Reuse the existing cross-platform sync test infrastructure.

Add a Wasm synced scenario to the platform matrix.

Verify:

- module hash/config agreement;
- stable module ordering;
- repeatable simulation checksum;
- FP edge cases;
- RNG;
- limit/fuel boundaries;
- callbacks and host calls.

## 10. CI gates

The final CI should include:

- generator unit tests;
- generated snapshot diff;
- Rust native build/test;
- Wasm guest build/test;
- native ↔ Wasm signature gate;
- Lua ↔ native ↔ Wasm signature gate;
- runtime parity;
- adversarial ABI tests;
- cross-platform synced Wasm test.

Path triggers must include both NativeInterface and WasmInterface/codegen files.

Parity that requires a built engine should reuse existing engine build artifacts rather than rebuild the engine in every job.
