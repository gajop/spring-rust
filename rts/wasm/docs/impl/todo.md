# Todo: unchecked core-wasm host

Follows `handoff.md`. Independent of it, but more expensive, so the cheaper
prototype goes first.

## Goal

Build a Wasmtime host that drops the Component Model, links guests as plain
core-wasm modules over a data layout we define, and produce every row of
`benchmarking_results.md` through it.

Host language is **to be decided**, and the measurement says it barely matters:
core wasm through the C API's unchecked entry points is 4.1 ns per callout and
10.8 ns per callin, against 3.2 and 12.5 through Wasmtime's typed Rust API. C
is faster on callins, Rust marginally faster on callouts, both within noise.

Default to C unless something other than speed argues for Rust, because C needs
no Rust toolchain in the engine build and keeps the checksum-pinned prebuilt
SDK. Rust would buy compile-time checking of the generated bindings.

## Why

This is the only direction measured to have room to beat Lua rather than match
it. Transport floor is about 4 ns per callout against Lua's 33 ns end to end.
The projected end-to-end figure is 15 to 20 ns and it is a guess. See
[../considerations/options.md](../considerations/options.md), direction 3.

## Do the cheap version first

The full table costs 40 to 60 hours, mostly because strings, lists, variants
and the callback all need layout rules before anything runs.

The decisive number needs none of that. A cut-down prototype with
`units_info::get_unit_def_id`, `units_info::get_unit_position` and one callin is
roughly **10 to 15 hours** and answers whether core wasm reaches about 15 ns
end to end in the real engine. Only expand to the full table if that lands.

This mirrors what settled direction 1: two functions, one afternoon, a real
number instead of an estimate.

## Scope for the full version

Same surface as `handoff.md`: 16 callouts, 7 callins. Fix the four broken callin
rows first, as described there, or the comparison is meaningless.

## What has to be designed

The layout, before any code. It has to be precise enough that a third party
could implement it:

- scalars in raw slots; aggregates in a guest arena
- strings and lists as pointer and length, with a stated owner and lifetime
- a return area convention
- struct layout and alignment rules
- error reporting without `result<T, e>`
- a callback table for `set_height_map_func`, reached through core wasm's
  indirect call
- version negotiation

## Host work

- Core-wasm module loading. The engine already has this path for fixtures, in
  the `isComponent == false` branch of `WasmModule.cpp`.
- Register imports with `wasmtime_linker_define_func_unchecked`, not
  `wasmtime_linker_define_func`. The checked form costs 34.5 ns against 4.1.
- Call guest exports with `wasmtime_func_call_unchecked`, not
  `wasmtime_func_call`. The checked form costs 139 ns against 10.8.
- Guest memory helpers with bounds and alignment validation on every read. This
  is the security boundary the canonical ABI handles today; it becomes ours.
- Generate both checked and unchecked forms and select checked in debug and
  ASAN builds, so the runtime validation acts as an assertion on the
  generator's own output at no cost to release builds.

## Guest work

This is what makes it more expensive than `handoff.md`, where guests are
untouched.

A core-wasm version of the benchmark guest. The measurement harness inside it,
about 600 lines, carries over unchanged; only the API layer is rewritten. The
16 callouts and 7 callins need hand-written binding shims for the prototype, or
a generator backend if it goes further.

Note that all five existing guests (`benchmark_guest`, `parity_guest`,
`aggregation_guest`, `allocator_guest`, `value_guest`) call
`wit_bindgen::generate!` directly, so none of them carry over.

## The decision this does not settle

Performance is measurable and this prototype will measure it. The cost that
outlives the prototype is that guest bindings become a product we maintain, per
language, forever. That is a question about who writes gadgets and in what,
covered in
[../considerations/guest_toolchain_impact.md](../considerations/guest_toolchain_impact.md).

A fast prototype does not answer it. Both need answering before this direction
is chosen.

## Build and run

Same as `handoff.md`.
