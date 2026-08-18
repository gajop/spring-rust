# Measured host-call costs

What a Wasm host call costs before any engine logic runs. Numbers are medians,
Wasmtime 42.0.1, one P-core of a 12700, scalar signature unless stated.

Reproduce with `test/wasm_api/hostcall_bench/run.sh`. That harness runs the
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

End-to-end engine measurements from `impl/benchmarking_results.md`, for scale.
These include real engine work, so they are not directly comparable to the
transport floors above; they are what any Wasm path has to beat.

| Row | Lua | Native |
| --- | ---: | ---: |
| `callout_scalar` | 32.9 | 6.0 |
| `callout_vec3` | 63.2 | 6.9 |
| `callin_unitcreated` | 200 | 23 |
| `callin_empty` | 314 | 20 |

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
`impl/benchmarking_results.md` carries the current numbers and these are a
snapshot of the shape.

It covers all three worlds (rules-synced, rules-unsynced, UI), so every row in
the table is measured on all four backends and none is reported unavailable.

Callouts, scale 1, ns:

| Row | C API dynamic | Rust typed CM | Lua |
| --- | ---: | ---: | ---: |
| `callout_scalar` | 188.7 | 41.4 | 30.4 |
| `callout_vec3` | 841.0 | 57.1 | 63.8 |
| `callout_string` | 324.8 | 117.1 | 73.4 |
| `callout_smalllist` | 1397.3 | 236.2 | 608.3 |
| `callout_biglist` | 7098.0 | 623.0 | 7906.4 |
| `callout_spatial` | 1965.4 | 336.8 | 735.1 |
| `callout_mutate` | 1095.8 | 102.1 | 213.3 |
| `callout_draw` | 354.9 | 41.5 | 80.1 |

Callins, scale 1, ns:

| Row | C API dynamic | Rust typed CM | Lua |
| --- | ---: | ---: | ---: |
| `callin_empty` | 1098 | 139 | 319 |
| `callin_update` | 1039 | 145 | 227 |
| `callin_unitcreated` | 1369 | 137 | 208 |
| `callin_allowunitcreation` | 2744 | 152 | 219 |
| `callin_unitpredamaged` | 3117 | 146 | 267 |
| `callin_4modules` | 3536 | 558 | 360 |
| `callin_drawworld` | 12222 | 7046 | 796 |

The guest callback case behaves the same way. `hm_brush_large` drives a
heightmap edit session in which the host calls back into the guest: 83.0 ms on
the C API, 13.9 ms typed, against 18.7 ms for Lua. The UI world's
glBegin/glEnd callback is the same shape and gives `wl_ui_draw` at 0.161 ms
typed against 1.453 ms on the C API and 0.322 ms for Lua.

Three things to read from this.

**`callout_vec3` goes to 55 ns.** The hand-specialised C callback above reached
452 ns and was within 25 ns of what looked like the floor. The typed transport
is eight times better than that, because it removes the incoming argument tree
as well as the outgoing one. The "floor for its exact signature" column is a
property of the C API, not of the signature.

**The typed callin cost is flat.** 134 to 160 ns whether the query record
carries one field or ten, against 1059 to 3144 ns over the same range on the C
API. The dynamic path pays per field in both directions; the typed path pays
per call.

**Against Lua the sign flips, but not everywhere.** The C API path is slower
than Lua on every row above. The typed host is faster than Lua on every
single-module sim callin and on six of eight callouts, losing on
`callout_scalar` and `callout_string`, where the absolute numbers are small
enough that the remaining per-call entry dominates.

Two callin rows go the other way, and both are about fan-out rather than about
the transport:

`callin_4modules` is 558 ns typed against 360 ns for Lua. Four modules cost
four component entries, 4.0x the 139 ns single-module row, while Lua's four
gadgets cost 1.13x its own single-module row because they share one state and
one boundary crossing. Wasm pays per instance where Lua pays per callback. A
shipping transport that expects many modules would need to care about this;
the C API path has the same shape, only worse in absolute terms.

`callin_drawworld` is 7046 ns typed against 796 ns for Lua. This one was
investigated, and it is not a dispatch cost. It is the cost of a cold entry.

Timing the component call from inside the host puts it at 6351 to 6775 ns,
matching the engine-side token, so the cost is inside `call_draw_world` and not
in the dispatch layer. Repeating the identical call immediately afterwards,
with the guest short-circuiting so neither call does any work, costs **216 ns**:

| | median |
| --- | ---: |
| first call of the frame | 6085 to 7154 ns |
| same call, immediately repeated | 216 ns |

DrawWorld runs once per rendered frame, and a frame of rendering evicts the
JIT code, the store and the component instance state from cache. The 216 ns
warm figure sits alongside the 139 ns `callin_empty` of the sim context, which
is the transport behaving normally.

The ambient effect is visible without Wasm at all: native's `callin_drawworld`
is 435 ns against its own 30 ns `callin_empty`, a 14x inflation from the same
cause. Wasm pays more because its working set is larger.

The practical cost model is therefore **one cold entry per frame plus cheap
warm entries**, not a cold entry per callin. A UI module taking several draw
callins per frame pays roughly 6 us once and about 0.2 us for each additional
one. Nothing in the host configuration changes this: `wasm_backtrace(false)`,
`native_unwind_info(false)` and the pooling allocator were all measured and all
land within noise.

`callin_unimplemented` is excluded from these comparisons. The guest exports
nothing, and the row is unstable across runs on every backend: the C API path
has measured 2472, 182, 1070 and 193 ns, and the typed path 126 and 24 ns. It
should not be read closely.

Being faster than Lua is the relevant comparison for a shipping transport, so
`benchmarking_results.md` states its ratios against the typed host rather than
against the dynamic C API.  Its columns name all three axes, `Wasm (C API,
dynamic, CM)` and `Wasm (Rust, typed, CM)`, because the native backend is also
written in Rust and the language is not the difference being measured.

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
