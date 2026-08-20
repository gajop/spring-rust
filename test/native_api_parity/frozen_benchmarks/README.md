# Frozen benchmark baselines

Real measurements for backends that are not currently being changed, so a run
can re-measure only the backend under work and still produce the complete
comparison table in `rts/wasm/docs/impl/benchmarking_results.md`.

One CSV per profile and backend, plus `metadata.json` recording the engine
revision and CPU the numbers came from. The columns are the raw row fields the
runner emits, so a frozen row round-trips into exactly the record a live run
would have produced. Nothing here is interpolated or hand-edited.

## Re-measure only the backend you are working on

```sh
RECOIL_BENCHMARK_BACKENDS=wasm_core \
  python3 test/native_api_parity/run_benchmarks_core.py --suite
```

The other four columns come from this directory, and the generated report says
so in a note above the table.

## Record a new baseline

```sh
RECOIL_BENCHMARK_FREEZE=1 \
  python3 test/native_api_parity/run_benchmarks_core.py --suite
```

Only the backends that run are written back, so combining the two variables
re-freezes just that backend and leaves the rest untouched.

`RECOIL_BENCHMARK_FROZEN_ROOT` points the store somewhere else if you want to
compare against a baseline without overwriting the committed one.

## When to refresh

Freeze on an otherwise idle release build. Re-freeze everything when the
hardware, compiler, or engine baseline changes enough that cross-backend ratios
stop being meaningful — mixing measurements from different machines makes the
ratio columns silently wrong, which is why `metadata.json` records the machine.
