# Candidate directions

Three directions, and one that was considered and dropped. Costs come from
[measured_costs.md](measured_costs.md); the two axes they combine are in
[host_binding_styles.md](host_binding_styles.md) and
[abi_choice.md](abi_choice.md).

## 1. Keep the transport, delete our own marshalling

Same Wasmtime, same Component Model, same dynamic C API. The only change is
removing the intermediate `WasmValue` layer, so generated adapters read and
write Wasmtime's value structs directly instead of going through a
`std::map<std::string, WasmValue>` per record.

**Measured, not projected.** Prototyped on two callouts and reverted:

| Row | Before | After | Lua |
| --- | ---: | ---: | ---: |
| `callout_scalar` | 183.6 | 128.0 | 32.9 |
| `callout_vec3` | 881.1 | 452.2 | 63.2 |

Real, and not enough. What remains is Wasmtime's own dynamic value handling,
which cannot be removed without changing transport. Guest tooling is unaffected
and no new build dependency appears.

Rough cost across the full surface: 1,200 to 1,500 hand-written lines, 30 to 45
hours.

## 2. Wasmtime's Rust API, keep the Component Model

Reach Wasmtime through its native API instead of its C wrapper, so argument
types are fixed at compile time and no value trees are built. Same WIT, same
wit-bindgen, same components.

Transport floor drops from 126 to 37 ns per scalar callout, and from 119 to 67
ns per callin.

**Guests are untouched.** Byte-for-byte identical `.wasm` files keep working.
That is the strongest argument for this direction.

The ceiling is the Component Model's own floor, so this lands near Lua parity
on callouts rather than past it: 37 ns of transport against Lua's 33 ns end to
end, before the engine does anything.

Cost across the full surface: 4,000 to 5,500 hand-written lines, 100 to 160
hours, plus a Rust toolchain in each build image.

## 3. Core wasm with our own layout

Drop the Component Model. Transport floor drops to about 4 ns per callout and
11 ns per callin, from plain C++ against the library already linked. No Rust
required: the C API's unchecked entry points are as fast as Rust's typed ones.

This is the only direction measured to have room to beat Lua rather than match
it.

The cost is not the host. It is that the data layout, the guest bindings
generator and per-pointer memory validation become ours, permanently and per
language. See [guest_toolchain_impact.md](guest_toolchain_impact.md).

Cost across the full surface: 4,500 to 6,000 hand-written lines with about
1,500 deleted, 130 to 200 hours.

A Rust host on top of core wasm was also measured and is **not** worth
building for speed: 3.2 ns against C's 4.1 on callouts, and slower than C on
callins at 12.5 against 10.8. It would buy compile-time checking of generated
bindings, nothing more.

## Dropped: hybrid

Core wasm for hot calls, Component Model for the rest. Dropped because which
calls are hot is application-dependent, so the split cannot be decided once,
and it leaves two ABIs to maintain forever.

## Projections and their reliability

Anything below is estimated, not measured, and estimates in this area have
already been wrong once by 190 ns. Treat them as a reason to prototype, not as
results.

| | `callout_scalar` | `callout_vec3` | vs Lua |
| --- | ---: | ---: | --- |
| current | 183.6 (measured) | 881.1 (measured) | 5.6x, 13.9x |
| direction 1 | 128.0 (measured) | 452.2 (measured) | 3.9x, 7.2x |
| direction 2 | ~50 (est) | ~60 (est) | roughly parity |
| direction 3 | ~15 to 20 (est) | ~20 (est) | faster than Lua |
| Lua | 32.9 | 63.2 | |
| Native | 6.0 | 6.9 | |

Direction 1 is measured because it was prototyped. Directions 2 and 3 are not,
and the prototypes that would settle them are in `impl/handoff.md` and
`impl/todo.md`.
