# Measured host-call costs

What a Wasm host call costs before any engine logic runs. Numbers are medians,
Wasmtime 42.0.1, one P-core of a 12700, scalar signature unless stated.

Reproduce with `test/wasm_api/guests/hostcall_bench/run.sh`. That harness runs the
same two guests against two hosts, the Wasmtime C API and the Wasmtime Rust
API, and checks guest return values so an optimised-away loop cannot pass as a
fast one.

The Rust side of that harness had drifted: the guest WIT grew `get-vec3-opts`
and the Rust host never implemented it, so the whole Rust half failed to
instantiate and printed a linker error instead of numbers. Repaired, and the
figures below reproduce.

## Guest to host (callout)

| Transport | scalar | `result<s32, e>` | `result<vec3, e>` |
| --- | ---: | ---: | ---: |
| core wasm, Rust typed | 3.2 | n/a | n/a |
| core wasm, C API unchecked | 4.1 | n/a | n/a |
| core wasm, C API checked | 34.5 | n/a | n/a |
| component, Rust typed | 24.7 | 37.0 | 47.9 |
| component, C API dynamic | 81.2 | 126.4 | 234.7 |

Add one two-field record as an *argument*, matching the shape of a real
callout, and the dynamic C API goes from 228.9 to **428.8**. Wasmtime builds
that argument as a heap value tree before the host callback is entered, so
nothing written on the host side removes it. The typed path pays about 1 ns for
the same structure.

## Host to guest (callin)

| Transport | scalar | 4-field record |
| --- | ---: | ---: |
| core wasm, C API unchecked | 10.8 | n/a |
| core wasm, Rust typed | 12.5 | n/a |
| component, Rust typed | 67.2 | 69.5 |
| component, C API dynamic | 118.8 | 201.5 |
| core wasm, C API checked | 139.0 | n/a |

## Reference points

End-to-end engine measurements from `generated/benchmarking_results.md`, for scale.
These include real engine work, so they are not directly comparable to the
transport floors above; they are what any Wasm path has to beat.

| Row | Lua | Native |
| --- | ---: | ---: |
| `callout_scalar` | 29.6 | 5.9 |
| `callout_vec3` | 61.7 | 5.8 |
| `callin_unitcreated` | 209 | 22 |
| `callin_empty` | 319 | 30 |

## Confirmed in the engine

The floors were checked against the real engine by hand-writing specialised
import callbacks for two callouts, going straight from the component value to
the native call with no intermediate value tree, then running the callouts
profile. The specialisation was reverted after measuring.

| Row | Generic path | Specialised | Floor for its exact signature |
| --- | ---: | ---: | ---: |
| `callout_scalar` | 183.6 | 128.0 | 120.4 |
| `callout_vec3` | 881.1 | 452.2 | 428.8 |

Both land within about 25 ns of the floor, which is the real native call plus
entry. The five untouched callout rows reproduced their recorded values within
noise in the same run.

Two conclusions. The floors are real and an adapter can reach them. And
removing the engine's own marshalling is worth less than the size of that code
suggests, because what remains is Wasmtime's dynamic value handling, not ours.

**That second conclusion is about the dynamic C API, not about Wasm.** It was
originally written as though ~200 ns of Wasmtime-built argument tree were
irreducible. It is irreducible *on that transport*: the C API has to
materialise the tree before the host callback is entered. A typed host never
builds it. See the next section.

## End to end through Rust static bindings

A second host was built that reaches the Component Model through Wasmtime's
Rust static bindings instead of the dynamic C API
(`rust/crates/spring-wasm-typed-host/`). Same WIT, same wit-bindgen, same
componentize step, same `.wasm` bytes: only the host differs, so the pair
isolates the transport. Both run in the same suite as separate backends, so
`generated/benchmarking_results.md` carries the current numbers and these are a
snapshot of the shape.

It covers all three worlds (rules-synced, rules-unsynced, UI), so every row in
the table is measured on all four backends and none is reported unavailable.

Callouts, scale 1, ns:

| Row | C API dynamic | Rust typed CM | Lua |
| --- | ---: | ---: | ---: |
| `callout_scalar` | 180.7 | 42.3 | 32.4 |
| `callout_vec3` | 887.2 | 54.3 | 61.7 |
| `callout_string` | 328.0 | 127.5 | 78.1 |
| `callout_smalllist` | 1442.7 | 250.8 | 628.3 |
| `callout_biglist` | 6990.0 | 630.0 | 8152.0 |
| `callout_spatial` | 1812.7 | 357.2 | 577.9 |
| `callout_mutate` | 1165.7 | 99.8 | 215.9 |
| `callout_draw` | 360.0 | 41.1 | 80.2 |

Callins, scale 1, cold cache, ns:

| Row | C API dynamic | Rust typed CM | Lua |
| --- | ---: | ---: | ---: |
| `callin_empty` | 13104 | 5338 | 6255 |
| `callin_update` | 12714 | 4804 | 5628 |
| `callin_unitcreated` | 14533 | 5592 | 5421 |
| `callin_allowunitcreation` | 19926 | 5826 | 5753 |
| `callin_unitpredamaged` | 21850 | 5531 | 5636 |
| `callin_4modules` | 24765 | 13070 | 6126 |
| `callin_drawworld` | 25844 | 12359 | 2293 |

Every callin row is measured cold: a 64 MB cache walk runs before each
dispatch so the CPU cache does not contain the callin target. Real callins
fire once per sim frame (~30 Hz) or once per render frame (~60 Hz), with a
full frame of engine work in between, so a warm-cache measurement would be
misleading. The eviction runs before the timer starts, so only the cold
re-entry is measured, not the walk itself.

The guest callback case behaves the same way. `hm_brush_large` drives a
heightmap edit session in which the host calls back into the guest: 87.8 ms on
the C API, 14.3 ms typed, against 20.5 ms for Lua. The UI world's
glBegin/glEnd callback is the same shape and gives `wl_ui_draw` at 0.169 ms
typed against 1.468 ms on the C API and 0.324 ms for Lua.

Three things to read from this.

**`callout_vec3` goes to 54 ns.** The hand-specialised C callback above reached
452 ns and was within 25 ns of what looked like the floor. The typed transport
is eight times better than that, because it removes the incoming argument tree
as well as the outgoing one. The "floor for its exact signature" column is a
property of the C API, not of the signature.

**Typed Wasm and Lua are the same cost for sim callins.** With cold cache the
ratios are 0.97x to 1.17x across the five single-module sim callins. Neither
is meaningfully faster than the other at the dispatch boundary. The C API path
is 2x to 4x worse.

**The callin cost is dominated by cache, not transport.** The old warm-cache
numbers (141 ns typed, 319 ns Lua) made it look like the typed host was 2x
faster. That was an artifact: a tight benchmark loop keeps the cache warm,
but real callins never get that. Both pay about 5 to 6 us cold, which is
cache and TLB reload, not dispatch logic.

`callin_4modules` is 13070 ns typed against 6126 ns for Lua. Four modules
cost four cold entries; Lua's four gadgets share one state and one crossing.
The expected deployment is one module per game, possibly one more for a map or
mutator, so this multiplies a number near one.

`callin_drawworld` is 12359 ns typed against 2293 ns for Lua. The render path
evicts more aggressively than the sim loop (GPU work, larger working set), so
all backends pay more here, but Wasm's working set is largest and it pays the
most. In absolute terms 12 us is 0.07% of a 16.7 ms frame. The composite
draw workload `wl_ui_draw` still wins: 0.169 ms typed against 0.324 ms for
Lua, because the callouts inside the draw entry are fast (41 ns each) and
there are many of them.

`benchmarking_results.md` states its ratios against the active Core host rather than
against the dynamic C API. Its columns name all three axes, `Wasm (C API,
dynamic, CM)` and `Wasm (Rust, typed, CM)`, because the native backend is also
written in Rust and the language is not the difference being measured.

## Where this leaves the transport choice

On these numbers the typed Component Model host is a viable shipping
transport and the dynamic C API is not.

Callins are dominated by cache reload, not transport overhead. Under
cold-cache measurement (matching real per-frame usage), the typed host and Lua
cost about the same for single-module sim callins (5 to 6 us each). The C API
path is 2x to 4x worse. DrawWorld costs more for Wasm (12 us vs 2 us for Lua)
because the render path evicts more aggressively, but the composite draw
workload still wins (0.169 ms vs 0.324 ms).

Callouts are where the typed host pulls ahead. It beats Lua on six of eight
rows, and the margin widens with the amount of work done per crossing:
`wl_unit_scan` 0.051 ms against Lua's 0.432, `callout_biglist` 630 ns against
8152.

Two costs are structural rather than tunable, and both come from the thing
being bought, namely a separate instance with its own memory. Neither is a
reason to decide against it, and both are recorded so they are not rediscovered
as surprises:

**Fan-out is linear.** Four modules cost four cold entries; Lua's four gadgets
share one crossing. The intended deployment is one module per game plus perhaps
a small map or mutator module, so this multiplies a number near one.

**Cold callin entry is about 5 us for both Lua and typed Wasm on the sim
path, and higher on the render path.** This is cache and TLB reload, not
dispatch logic. It is small in absolute terms (0.03% to 0.07% of a 16.7 ms
frame) and is amortised by any real work inside the callin.

The recommendation is to commit to the typed Component Model host and drop the
dynamic C API as a candidate.

## Notes that cost time to rediscover

**Feature flags change the numbers.** The `wasmtime` crate's default features
include `component-model-async` and `stack-switching`, which the prebuilt C API
release does not have. With crate defaults a component callin measures 348 ns
instead of 67 ns. Any Rust-side comparison must pin the feature set to match
whatever the C side is built with.

**`wasmtime_func_call` is not the cheap way to call core wasm.** It rebuilds
the function type from the engine's type registry and typechecks every argument
on each call. `wasmtime_func_call_unchecked` on the same function is thirteen
times faster.
