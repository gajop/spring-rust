# Handoff: Wasm benchmark work

Date: 2026-08-18

## Task

Benchmarks only. Do not work on parity coverage, do not chase the 39% unsynced
number, do not touch LuaUI. Two jobs in order:

1. Fix the benchmark harness bugs listed below.
2. Fix the Wasm callout dispatch cost, then re-measure.

## Repo state

- Repo: `/home/gajop/projects/spring-projects/spring-bar`, branch `rust-wip`.
- Working tree clean. Branch is ahead of `origin/rust-wip` and unpushed.
- Backup ref before a history rewrite: `backup/pre-date-rewrite-20260818`.
- Build command: `./docker-build-v2/build.sh linux`. Native cmake configure is
  broken (FindThreads / `--icf=all`), so always use the docker script.
- ASAN build is for correctness only. Performance budgets are reported but not
  enforced under ASAN (`TestWasmInterface.cpp` lines 717, 761, 784, 832).
  Benchmarks must run release, no ASAN.

Recent commits:

```
7eabf5fcbd  Add Lua vs native ratio column to the benchmark report
472ae31fdd  Expand Wasm parity coverage and fix native API index/move semantics
bf1bbb33fc  Finalize Wasm benchmark reporting and ownership
473b1da485  Complete Wasm API parity and benchmarking
4aa9c0d5d2  Complete Wasm environment parity implementation
```

## Key files

| Path | What |
| --- | --- |
| `rts/wasm/docs/benchmarking.md` | The spec. 33 tests across 5 layers. Do not edit it to match the implementation. |
| `rts/wasm/docs/impl/benchmarking_results.md` | Generated results table. Generator-owned, table only. |
| `test/native_api_parity/run_benchmarks.py` | The harness. |
| `rts/wasm/docs/impl/review-feedback.md` | Standing review findings. |

## Job 1: benchmark harness bugs

Evidence is in `test/native_api_parity/out/benchmark/suite-20260817-222112-694815/`
(bounded run, per-profile scale 0.01 to 0.1).

**1a. The heightmap profile is broken at low scale.**

Every heightmap row has `invocations: 1` regardless of `nominalInvocations`
(10,000 / 1,000 / 100 / 10). Every Wasm heightmap row reports exactly
`medianMs: 0.002` with spread 0.000 to 0.001, because the Wasm clock quantum is
roughly 1 to 2 microseconds and the scaled workload is below it.

`innerNs` is then derived by dividing that constant by the inner-call count,
which yields 2000 ns, 125 ns, 7.8 ns for small, medium, large. Per-call cost
appearing to fall 256x as the brush grows is a division artifact. Native shows
the same pattern more weakly (132 ns vs 31.8 ns).

Fix: require a minimum measured duration per sample (say 50x the clock quantum)
and raise the iteration or invocation count until it is met, or mark the row
unavailable. Never divide a quantized constant to produce a per-call figure.

**1b. `callin_drawworld` runs 2 iterations.**

That is the source of the 2.44 second Wasm value and Lua's 19.9 ms in the
committed table. Both are noise. Give the draw profile enough iterations or
drop the row.

**1c. No timer-resolution guard anywhere.**

Lua's clock quantum is 19.07 ns (every Lua value in the older table was a
multiple of it). Wasm's is roughly 1 to 2 microseconds. Nothing checks the
measured quantity against the granularity. Add that check once, centrally, and
both 1a and 1b stop being possible.

**1d. `hm_callback_empty` violates its own scale invariance.**

It has zero inner calls, so per-invocation cost must not move with scale. It
reads 0.002 ms in the bounded run and 11.432 ms in the committed scale-1 table.
A 5,700x swing means at least one run is wrong. Treat the committed heightmap
column as untrusted until this is explained.

Note the scale-1 summaries are gone. Only bounded runs (0.01 / 0.1) survive
under `out/benchmark/`, so the committed table cannot be cross-checked. Re-run
at scale 1 after fixing the above.

## Job 2: why Wasm callouts are slow

This is the real finding. It is not inherent to Wasm or the Component Model.

Every callout re-resolves its target by string comparison on every call:

1. `CanonicalModule()` in `rts/wasm/generated/WasmCalloutRegistry.h` linear-scans
   about 60 interface descriptors doing string compares.
2. The per-module dispatcher (tail of each `rts/wasm/generated/WasmHostAdapter_<module>.cpp`)
   is a linear `if` chain, up to about 80 candidates for `units_info`.
3. Each comparison calls `detail::FunctionEquals` at
   `rts/wasm/generated/WasmHostAdapterSupport.h:465`, which compares character by
   character through `std::tolower`. That is a locale-aware, non-inlined call
   per character.

The target is already known at bind time. `rts/WasmInterface/WasmModule.cpp:1406-1409`
builds a `WasmHostFunctionData` at import registration holding `moduleName` and
`functionName`, and the trampoline registered at line 1418 hands those strings
back down to be re-parsed on every invocation.

The same trampoline also, per call:

- heap-allocates a `std::vector<WasmValue>`;
- constructs a `std::string error`;
- issues a `wasmtime_component_func_type_param_nth` type query per argument.

This matches the measured shape: 561 ns scalar, 1,628 ns vec3 (longer name,
more result fields), and `callin_unimplemented` at 2,472 ns against native's
16 ns because dispatch never short-circuits.

**Fix**

Resolve the dispatch target once at import registration and store a function
pointer or a stable index in `WasmHostFunctionData`. The call path then becomes
an indirect call with no string work. This is a generator change (emit a
per-module dispatch table plus a lookup used only at bind time) and a small
runtime change in `WasmModule.cpp`.

Secondary, after the above lands and is measured: reuse a per-module scratch
buffer for the argument vector instead of allocating per call, and cache the
parameter types at bind time rather than querying per call.

**Do not** conclude anything about Component Model overhead until this is fixed.
The current numbers are measuring string comparison, not transport.

## Context worth keeping

`wl_compute` (a numeric loop with zero engine calls) has Wasm at 0.324 ms
against Lua's 1.444 ms, 4.5x faster. The VM is fast. The binding is slow. That
test exists specifically to separate those two, and it is doing its job.

Frame budget for judging results: 33 ms at 30 Hz sim, 16 ms at 60 fps, 7 ms at
144 fps. Target for hot-path callouts is roughly 0.5 microseconds.

## Verification

After changes:

1. `./docker-build-v2/build.sh linux` (release, no ASAN) for the benchmark.
2. Re-run the full suite at scale 1 and regenerate
   `rts/wasm/docs/impl/benchmarking_results.md`.
3. Confirm the report keeps the three ratio columns: `Lua vs native`,
   `Wasm vs Lua`, `Wasm vs native`.
4. `python3 rts/wasm/verify_codegen.py` if the generator changed.
5. Separately, an ASAN build plus CTest for correctness.

## Working preferences

Plain language. No em dashes. Concise. Skip the editorializing, state what was
done and what the numbers say.
