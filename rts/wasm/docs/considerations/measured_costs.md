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
one boundary crossing. Wasm pays per instance where Lua pays per callback; the
C API path has the same shape, only worse in absolute terms.

The expected deployment is one module per game, possibly one more for a map or
mutator that does very little, so the linear term is multiplied by a small
number and this row does not constrain the choice. It is recorded because it
would matter to a design that wanted many small modules, which this is not.

`callin_drawworld` is 5896 ns typed against 650 ns for Lua. It is a cold entry
rather than a dispatch cost, and it is genuinely worse for Wasm than for the
other backends. Comparing each backend against its own `callin_empty`, in
absolute terms rather than as a ratio:

| Backend | `callin_empty` | `callin_drawworld` | extra |
| --- | ---: | ---: | ---: |
| Lua | 319 | 650 | +331 |
| Native | 30 | 420 | +390 |
| Rust typed CM | 141 | 5896 | +5755 |
| C API dynamic | 1111 | 12193 | +11082 |

Lua and native pay about a third of a microsecond for being called from the
render path. The typed host pays nearly six. This is not a shared ambient
effect that happens to show up as a large ratio; Wasm really is 15 times worse
here, because its working set is far larger than a Lua closure or a function
pointer.

The mechanism was confirmed rather than assumed. Timing the component call from
inside the host matches the engine-side token, so the cost is inside
`call_draw_world`. Calling it three times in a row then gives:

| | median |
| --- | ---: |
| the frame's first call | 5896 ns |
| immediately repeated | 226 ns |
| repeated after walking a 64 MB buffer | 4199 ns |

A synthetic cache walk with no rendering anywhere near it reproduces most of
the cost, which is what makes cache and TLB eviction the mechanism rather than
a guess. A frame of rendering evicts more thoroughly than a linear walk does,
which accounts for the rest.

**The 226 ns figure is a diagnostic, not a cost model.** It is the same call
microseconds later in the same frame, which is not what a once-per-frame draw
callin does. A callin dispatched once per frame pays the cold price every
frame. Warmth only helps a module that crosses the boundary several times in
quick succession.

That case is the normal one for drawing, though, and the table already measures
it. `wl_ui_draw` is a real draw workload: one DrawWorld entry followed by many
callouts. It costs 0.163 ms typed against 0.306 ms for Lua, so the composite
wins by 1.9x with the cold entry included. `callout_draw`, the crossings inside
that entry, is 42 ns. The isolated `callin_drawworld` row measures the fixed
overhead with no work attached to amortise it, which is the least flattering
framing available for Wasm.

In absolute terms the fixed overhead is small: 5.9 us once per frame is 0.035%
of a 16.7 ms frame at 60 fps, of which about 5.5 us is what Lua would not have
paid. Ten draw callins spread far enough apart to each go cold would still be
under half a percent of the frame. No host configuration moves it:
`wasm_backtrace(false)`, `native_unwind_info(false)` and the pooling allocator
were all measured and all land within noise. The lever, if it ever mattered,
would be shrinking the guest's linear memory to cut TLB pressure.

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
being bought, namely a separate instance with its own memory. Neither is a
reason to decide against it, and both are recorded so they are not rediscovered
as surprises:

**Fan-out is linear.** Four modules cost 3.9x one module; four Lua gadgets cost
1.14x, because they share one state and one crossing. The intended deployment
is one module per game plus perhaps a small map or mutator module, so this
multiplies a number that stays near one. It would matter to a design built
around many small modules.

**Entering Wasm cold costs about 6 us.** Anything called once per frame from
the render path pays that every frame, because a frame of rendering evicts the
instance from cache; only bursts of crossings get the 0.2 us warm price. Lua
and native pay about 0.4 us for the same transition, so this is a real Wasm
cost and not a shared effect. It is also small in absolute terms, 0.035% of a
16.7 ms frame, and it is fixed overhead: the moment a draw callin does actual
work the composite wins anyway, with `wl_ui_draw` at 0.163 ms against Lua's
0.306 ms.

Neither is reachable by more host work. The optimisation pass found no
configuration headroom, and the remaining per-call overhead above the transport
floor (141 ns measured against a 67 ns floor for a component callin) is engine
dispatch, not the binding style.

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
