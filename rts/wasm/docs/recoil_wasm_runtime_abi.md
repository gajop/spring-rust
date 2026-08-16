# Recoil Wasm Runtime & ABI

## Runtime choice

Use Wasmtime as the initial runtime.

Use WIT/Component Model as the initial ABI because it provides semantic strings, lists, records, results, enums, and resources instead of exposing a pointer-offset ABI to module authors.

This is a provisional architecture choice until proven against the exact pinned C/C++ Wasmtime release.

## Required runtime spike

Before building the full generator, a small C++ prototype must prove:

1. host-defined component imports from C/C++;
2. strings, lists, records, options/results, enums;
3. host-owned resources/drop behavior;
4. fuel and memory/resource limits;
5. import/world registration and useful missing-interface diagnostics;
6. version matching/side-by-side interface versions;
7. callback/re-entry mechanics;
8. acceptable cost for high-frequency calls.

If these fail materially, the same semantic API model can generate a core-Wasm ABI instead.

## Callout path

```mermaid
flowchart LR
    G["Guest"] --> W["WIT binding"]
    W --> C["Canonical ABI"]
    C --> A["Host adapter"]
    A --> N["NativeInterface"]
```

The generated host adapter:

1. receives semantic values;
2. validates bounds/types/handles;
3. builds the existing NativeInterface query;
4. calls the authoritative NativeInterface API;
5. converts the result;
6. returns semantic values/errors.

No engine behavior is duplicated.

## Canonical ABI allocator and re-entry

Variable-size Component Model returns may execute guest allocator code (`cabi_realloc`) while lowering a host result.

Treat this as real guest execution.

Rules:

- a Spring import cannot recursively enter another Spring import by default;
- the guard covers the complete host-call/canonical-return period, not only the C++ callback body;
- explicit callback cases may opt into controlled nesting;
- allocator traps/fuel behavior is tested adversarially;
- maximum variable-size results are bounded.

## Mutation safety

A guest-visible failure should not occur after an irreversible engine mutation if the return path still requires arbitrary guest execution.

Across the reviewed API surface, the current variable-size mutating exception is `CallCOBScript`.

Its requested return count is declared before the engine call, so the implementation can:

1. validate the bound;
2. ensure the return capacity/budget can be satisfied;
3. only then perform the mutation.

The generator should enforce:

> mutating + variable-size result requires an explicit reviewed exception.

Future functions that violate this rule fail generation until a safe transport/result strategy is defined.

## Callins

Engine callins use generated Wasm exports.

Keep existing engine conversion shared:

`engine event -> NativeInterface public query -> backend dispatch`.

For Wasm, dispatch lowers the already-normalized event to the exported WIT function.

The callin inventory defines environment applicability, defaults, aliases, and aggregation behavior.

## Callbacks

RmlUi and a small number of APIs require callbacks.

Initial callback model:

- guest registers a callback ID/closure;
- host owns a subscription/resource associated with the instance;
- host synchronously re-enters the same Wasm instance when the callback fires;
- callback execution consumes the same instance budget;
- drop/unload releases all callback resources;
- nested Spring calls are allowed only for callback classes explicitly marked as re-entrant.

Phase 2 proves the mechanism; later API generation applies it broadly.

`VFS.UseArchive` should preferably become a scoped host operation rather than create another generic callback pattern.

## Handles/resources

Do not mechanically map every `uint64_t` handle to a Component Model resource.

Choose per handle family:

- stable integer/opaque ID when value identity/order is observable and existing semantics depend on it;
- Component Model resource when ownership/drop is the important behavior.

All handles are instance-owned and validated before use.

## Errors

Expose stable error codes/categories.

Keep detailed host error messages host-side by default.

Reasons:

- messages may include host-sensitive paths/details;
- copying them requires variable-size guest allocation;
- stable codes are easier to keep compatible and deterministic.

Interfaces that genuinely need text can opt in explicitly.

## Performance

The Component Model is the default, but performance is a decision gate.

Measure before full generation:

- scalar callout overhead;
- string/list/record result cost;
- callin cost;
- callback re-entry;
- high-frequency `Gfx` calls.

`Gfx` is the primary candidate for a generated core-Wasm fast transport if Component Model overhead is unacceptable.

The public Rust API should not depend on which transport is used.

## Wasmtime build integration

Pin the exact Wasmtime version and configuration as an engine dependency.

Phase 2 decides:

- prebuilt C API binaries versus source build;
- Linux/Windows/arm64 acquisition;
- JIT versus optional host-generated AOT cache.

Serialized Wasmtime artifacts are executable native code. Cache handling belongs to the sandbox/security rules and is independent of where the Wasm module came from.
