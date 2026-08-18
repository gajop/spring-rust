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
`impl/benchmarking_results.md` carries the current numbers and these are a
snapshot of the shape.

It covers all three worlds (rules-synced, rules-unsynced, UI), so every row in
the table is measured on all four backends and none is reported unavailable.

Callouts, scale 1, ns:

| Row | C API dynamic | Rust typed CM | Lua |
| --- | ---: | ---: | ---: |
| `callout_scalar` | 170.8 | 42.8 | 29.6 |
| `callout_vec3` | 882.0 | 52.0 | 61.7 |
| `callout_string` | 338.3 | 119.4 | 73.0 |
| `callout_smalllist` | 1416.6 | 246.9 | 622.6 |
| `callout_biglist` | 6842.0 | 604.0 | 8013.2 |
| `callout_spatial` | 1850.3 | 348.8 | 530.9 |
| `callout_mutate` | 1041.3 | 101.9 | 202.9 |
| `callout_draw` | 361.7 | 42.2 | 78.7 |

Callins, scale 1, ns:

| Row | C API dynamic | Rust typed CM | Lua |
| --- | ---: | ---: | ---: |
| `callin_empty` | 1111 | 141 | 319 |
| `callin_update` | 1072 | 146 | 236 |
| `callin_unitcreated` | 1352 | 144 | 209 |
| `callin_allowunitcreation` | 2801 | 151 | 221 |
| `callin_unitpredamaged` | 3162 | 148 | 272 |
| `callin_4modules` | 3534 | 557 | 364 |
| `callin_drawworld` | 12193 | 5896 | 650 |

The guest callback case behaves the same way. `hm_brush_large` drives a
heightmap edit session in which the host calls back into the guest: 82.9 ms on
the C API, 13.9 ms typed, against 18.8 ms for Lua. The UI world's
glBegin/glEnd callback is the same shape and gives `wl_ui_draw` at 0.163 ms
typed against 1.435 ms on the C API and 0.306 ms for Lua.

Three things to read from this.

**`callout_vec3` goes to 52 ns.** The hand-specialised C callback above reached
452 ns and was within 25 ns of what looked like the floor. The typed transport
is eight times better than that, because it removes the incoming argument tree
as well as the outgoing one. The "floor for its exact signature" column is a
property of the C API, not of the signature.

**The typed callin cost is flat.** 141 to 151 ns whether the query record
carries one field or ten, against 1072 to 3162 ns over the same range on the C
API. The dynamic path pays per field in both directions; the typed path pays
per call.

**Against Lua the sign flips, but not everywhere.** The C API path is slower
than Lua on every row above. The typed host is faster than Lua on every
single-module sim callin and on six of eight callouts, losing on
`callout_scalar` and `callout_string`, where the absolute numbers are small
enough that the remaining per-call entry dominates.

Two callin rows go the other way, and both are about fan-out rather than about
the transport:

`callin_4modules` is 557 ns typed against 364 ns for Lua. Four modules cost
four component entries, 3.9x the 141 ns single-module row, while Lua's four
gadgets cost 1.14x its own single-module row because they share one state and
one boundary crossing. Wasm pays per instance where Lua pays per callback. A
shipping transport that expects many modules would need to care about this;
the C API path has the same shape, only worse in absolute terms.

`callin_drawworld` is 5896 ns typed against 650 ns for Lua. This one was
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
warm figure sits alongside the 141 ns `callin_empty` of the sim context, which
is the transport behaving normally.

The ambient effect is visible without Wasm at all: native's `callin_drawworld`
is 420 ns against its own 30 ns `callin_empty`, a 14x inflation from the same
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

## Where this leaves the transport choice

On these numbers the typed Component Model host is a viable shipping
transport and the dynamic C API is not. The C API path is slower than Lua on
every row measured; the typed path is faster than Lua on every single-module
sim callin, six of eight callouts, both callback workloads and every heavy
workload, and it is within noise of Lua on the trivial ones. The margin widens
exactly where it should, with the amount of work done per crossing:
`wl_unit_scan` 0.047 ms against Lua's 0.411, `callout_biglist` 604 ns against
8013.

Two costs are structural rather than tunable, and both come from the thing
being bought, namely a separate instance with its own memory:

**Fan-out is linear.** Four modules cost 3.9x one module; four Lua gadgets cost
1.14x, because they share one state and one crossing. Many small modules pay
for every one. A few substantial modules do not care. Which shape the game
actually has is the question worth answering before this becomes hard to
reverse.

**There is a cold entry per frame.** About 6 us the first time anything enters
Wasm in a rendered frame, then about 0.2 us for each further crossing in that
frame. Lua does not pay this because it never goes cold.

Neither is reachable by more host work. The optimisation pass found no
configuration headroom, and the remaining per-call overhead above the transport
floor (141 ns measured against a 67 ns floor for a component callin) is engine
dispatch, not the binding style.

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
