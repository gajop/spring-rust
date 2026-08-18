# Measured host-call costs

What a Wasm host call costs before any engine logic runs. Numbers are medians,
Wasmtime 42.0.1, one P-core of a 12700, scalar signature unless stated.

Reproduce with `test/wasm_api/hostcall_bench/run.sh`. That harness runs the
same two guests against two hosts, the Wasmtime C API and the Wasmtime Rust
API, and checks guest return values so an optimised-away loop cannot pass as a
fast one.

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

Callouts, scale 1, ns:

| Row | C API dynamic | Rust typed CM | Lua |
| --- | ---: | ---: | ---: |
| `callout_scalar` | 175.4 | 41.6 | 30.7 |
| `callout_vec3` | 885.6 | 54.9 | 57.8 |
| `callout_string` | 332.8 | 99.5 | 78.1 |
| `callout_smalllist` | 1399.7 | 219.0 | 596.0 |
| `callout_biglist` | 6768.0 | 592.0 | 7994.2 |
| `callout_spatial` | 1948.2 | 318.2 | 570.6 |
| `callout_mutate` | 1058.9 | 96.9 | 206.0 |

Callins, scale 1, ns:

| Row | C API dynamic | Rust typed CM | Lua |
| --- | ---: | ---: | ---: |
| `callin_empty` | 1070 | 151 | 316 |
| `callin_update` | 1059 | 134 | 227 |
| `callin_unitcreated` | 1385 | 159 | 200 |
| `callin_allowunitcreation` | 2770 | 160 | 218 |
| `callin_unitpredamaged` | 3144 | 154 | 273 |

The guest callback case behaves the same way. `hm_brush_large` drives a
heightmap edit session in which the host calls back into the guest: 83.4 ms on
the C API, 13.9 ms typed, against 19.9 ms for Lua.

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

**Against Lua the sign flips.** The C API path is slower than Lua on every row
above. The typed host is faster than Lua on every implemented callin and on
five of seven callouts, losing only on `callout_scalar` and `callout_string`,
where the absolute numbers are small enough that the remaining per-call entry
dominates. The one callin exception is `callin_unimplemented`, where the guest
exports nothing and Lua's 62 ns is hard to beat; that row is also unstable
across runs and should not be read closely.

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
