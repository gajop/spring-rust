# Host binding styles

How the host passes values across the Wasmtime boundary. This is a host-side
property only: a guest cannot observe which style is used, and the module
bytes, imports, exports and behaviour are identical under all three.

The other axis, which ABI the call crosses, is in
[abi_choice.md](abi_choice.md).

## The three styles

**Checked**, also called dynamic. The host passes tagged unions carrying a
`kind` field, and Wasmtime validates them against the signature on every call.
`wasmtime_func_call`, `wasmtime_linker_define_func`, and on the Component Model
side `wasmtime_component_func_call` and
`wasmtime_component_linker_instance_add_func`.

**Unchecked**. The host passes flat untagged 16-byte slots
(`wasmtime_val_raw_t`), already in the layout the compiled trampoline reads.
`wasmtime_func_call_unchecked`, `wasmtime_linker_define_func_unchecked`. The
host asserts the array matches the signature it registered.

**Rust typed**. Types are fixed at compile time, so monomorphised code writes
into the same flat slots with no tags and no runtime check. `Linker::func_wrap`,
`Func::typed`. `Func::typed` verifies the signature once at lookup and returns a
`TypedFunc` whose call path is internally unchecked.

Really two ideas: one dynamic, and one static where the difference is only who
guarantees the types match. Unchecked takes the host's word. Rust typed has the
compiler prove it.

## Availability

| Binding style | Core-Wasm ABI | Component Model ABI |
| --- | --- | --- |
| Checked / dynamic | yes | yes |
| Unchecked | yes | **no** |
| Rust typed | yes | yes |

There is no unchecked style for the Component Model, because a record, string
or list has no single flat representation that can cross a C ABI without a type
system to describe it. Core wasm has four scalar types with one obvious raw
layout, so the escape hatch exists there.

The consequence: reaching the Component Model's fast path requires the Rust
API. Reaching core wasm's does not.

## Comparison

Costs are from [measured_costs.md](measured_costs.md), scalar signature,
written as core / component.

| Dimension | Decides the choice | Checked / dynamic | Unchecked | Rust typed |
| --- | --- | --- | --- | --- |
| Works on the Component Model ABI | **yes** | yes | **no** | yes |
| Callout cost | **yes** | 34.5 / 81.2 ns | 4.1 ns | 3.2 / 24.7 ns |
| Callin cost | **yes** | 139.0 / 118.8 ns | 10.8 ns | 12.5 / 67.2 ns |
| A host-side type mismatch can corrupt memory | **yes** | no, clean error | **yes** | no, compile error |
| Requires a Rust staticlib in the host | **yes** | no | no | yes |
| Cost scaling with argument count | no | grows per argument | flat | flat |
| Safe to hand-write | no | yes | generator only | yes |
| Diagnostic on a mismatch | no | descriptive, at runtime | none, silent corruption | compile error |
| Guest trap is contained | no | yes | yes | yes |
| Guest can escape the sandbox | no | no | no | no |
| Determinism and sync behaviour | no | unaffected | unaffected | unaffected |
| Guest language or toolchain constraints | no | none | none | none |

## Why most rows are ties

Containment, sandboxing and determinism sit below the binding style. A guest
trap is caught by the code Cranelift compiled; sandboxing comes from the linear
memory model and from the guest only reaching imports it is given; determinism
comes from the Wasm semantics and the values supplied. All three styles move
identical bits, and `wasmtime_val_raw_t` documents every field as little-endian
with a static-asserted 16-byte size, so there is no representation drift.

## What "unchecked" means

Not that the guest is unchecked. It means Wasmtime stops verifying that the
host passed an argument array matching a signature the host itself registered.
Both halves come from the host, so a mismatch is a host bug, not an attack
surface.

That bug class is static: it does not depend on guest input, so a mismatched
signature is wrong on the first call and every call, and surfaces under any
test touching the function.

So unchecked is a poor fit for hand-written bindings, where a signature and its
slot reads drift apart during editing, and a good fit for generated ones, where
both halves come from one generator run. Since the two forms take the same
arguments, a generator can emit both and select the checked one in debug and
ASAN builds, turning the runtime validation into an assertion on its own output
at no cost to release builds.

## The Rust staticlib requirement

Rust typed is the only style that changes how the host is built, since it is
reached through the Wasmtime crate rather than its C wrapper. Building Wasmtime
from source is the ordinary way to use it from Rust; the prebuilt C API release
exists for embedders that are not Rust programs.

Measured for 42.0.1 with the feature set matching the C API release: a clean
release build compiles 143 crates in 65 seconds on 20 cores and leaves about
780 MB in the cargo target directory. The prebuilt static library is 48 MB.

So the compile time is not the problem. The work is:

- a Rust toolchain in each build image
- a target per platform, including `x86_64-pc-windows-gnu` for MinGW builds.
  Upstream's prebuilt Windows C API archive ships an MSVC-ABI static library
  MinGW cannot link, which is why `WasmtimeDependency.cmake` falls back to the
  DLL and import-library pair. A Rust staticlib for the GNU target would avoid
  that mismatch, but the combination should be verified rather than assumed.
- pinning through `Cargo.lock` instead of a checksummed SDK download, which is
  equivalent for reproducibility
- a CI cache so the build is not repeated every run
