# Core wasm or the Component Model

The second axis after [host binding styles](host_binding_styles.md). This one
is about which ABI the call crosses, and unlike the binding style it is visible
to guest authors. That consequence is in
[guest_toolchain_impact.md](guest_toolchain_impact.md).

## What each one is

**Core wasm** is WebAssembly as the machine sees it. Four types exist: i32,
i64, f32, f64. Plus linear memory. A function is a fixed list of those scalars
in and out. Nothing else is defined, so passing a string means the host and
guest agreeing on a convention: the guest exports an allocator, the host writes
bytes into guest memory, and a pointer and a length cross as two i32s. That
convention is an ABI somebody has to write and keep both sides of in sync.

**The Component Model** standardises exactly that convention. It adds an
interface type system (strings, lists, records, variants, options, results,
resources), a canonical ABI specifying how each type lowers into core scalars
and memory, per-instance encapsulation, and WIT as the language those
interfaces are written in. wit-bindgen then generates guest bindings from the
same WIT the host uses.

## Cost

From [measured_costs.md](measured_costs.md), using the best binding style
available on each ABI:

| | callout, scalar | callin, scalar |
| --- | ---: | ---: |
| core wasm | 3.2 to 4.1 ns | 10.8 to 12.5 ns |
| Component Model | 24.7 ns | 67.2 ns |

The Component Model's overhead is its canonical ABI entry and exit
bookkeeping. It is not a binding artefact: the figures above are already the
typed path, and no binding style removes it.

For context, Lua's callout is 33 ns end to end. So the Component Model's
transport floor alone is close to Lua's whole cost, while core wasm's leaves
room to spare.

## Comparison

| Dimension | Decides the choice | Core wasm | Component Model |
| --- | --- | --- | --- |
| Types available | **yes** | four scalars and linear memory | strings, lists, records, variants, options, results, resources |
| Who specifies the data layout | **yes** | we do | the Component Model specification |
| Guest bindings generator | **yes** | we write one per language | wit-bindgen, upstream |
| Callout floor | **yes** | 3.2 to 4.1 ns | 24.7 ns |
| Callin floor | **yes** | 10.8 to 12.5 ns | 67.2 ns |
| Who validates guest memory access | **yes** | we do, per pointer | Wasmtime, per canonical lifting |
| Fast host binding style available from C | **yes** | yes, unchecked | no, dynamic only |
| Handles and ownership | no | integer handle tables we maintain | resources, typed |
| Interface versioning and composition | no | ours to define | part of the model |
| Sandboxing, traps, determinism | no | same | same |
| Fuel, per-instance memory, the JIT | no | same | same |
| Toolchains that can produce a guest | no | anything targeting wasm | anything wit-bindgen or an equivalent supports |

## What is kept either way

Dropping the Component Model does not mean dropping Wasmtime. Sandboxing,
per-instance linear memory, trap handling, fuel metering, Cranelift's code
quality and the execution-environment matrix are all core-wasm features. A pure
compute loop already runs about five times faster than Lua, and that is
unaffected by this choice.

## What is given up

- WIT as a machine-checked interface language, and wit-bindgen with it.
- A data layout specified and maintained by someone else.
- Resources as a typed concept rather than integer handles we police.
- Guest authors reaching for standard tooling instead of ours.

Some of that is less than it appears, because `model.json` is already an
interface description and the generator already emits both sides. The part that
does not shrink is the per-language guest tooling.

## What is taken on

- A written layout specification, precise enough that a third party can
  implement it: struct layouts, alignment, string and list encoding, arena
  ownership, error reporting, the callback table, version negotiation.
- Bounds and alignment validation on every value read out of guest memory,
  which the canonical ABI does today.
- A compatibility story for when a record gains a field.
