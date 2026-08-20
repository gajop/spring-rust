# Core Wasm benchmark comparison

`run_benchmarks_core.py` is the Core-aware entrypoint for the existing native API parity benchmark. It imports `run_benchmarks.py` and keeps the same fixtures, scales, test names and validation rules, while appending a fifth backend:

- Lua
- native C API
- dynamic Component Model host
- typed Rust Component Model host
- unchecked Core Wasm host

Run the bounded comparison suite with the same engine binaries used by the existing runner:

```bash
python3 test/native_api_parity/run_benchmarks_core.py \
  --suite --bounded-suite \
  --spring-headless ./spring-headless \
  --spring ./spring
```

Individual profiles use the existing flags, for example:

```bash
python3 test/native_api_parity/run_benchmarks_core.py --callouts \
  --spring-headless ./spring-headless --spring ./spring
python3 test/native_api_parity/run_benchmarks_core.py --callins \
  --spring-headless ./spring-headless --spring ./spring
python3 test/native_api_parity/run_benchmarks_core.py --heightmap \
  --spring-headless ./spring-headless --spring ./spring
python3 test/native_api_parity/run_benchmarks_core.py --workloads \
  --spring-headless ./spring-headless --spring ./spring
python3 test/native_api_parity/run_benchmarks_core.py --memory \
  --spring-headless ./spring-headless --spring ./spring
python3 test/native_api_parity/run_benchmarks_core.py --draw \
  --spring-headless ./spring-headless --spring ./spring
```

The Core runner builds a raw wasm32 guest for each Component benchmark variant and sets `SPRING_WASM_CORE_HOST=1` only for the Core process. Synced Core guests are linked with fixed 16 MiB memory because synced validation requires memory `min == max`.

Core guest-produced benchmark rows use the historical `benchmark_wasm.jsonl` Lua sink while engine-side Core callin timing uses `benchmark_wasm_core.jsonl`. The Core runner merges both streams and normalizes their backend label to `wasm_core` before applying the existing row validation.

The generated comparison table adds `Wasm (C API, unchecked, Core)` and the `Core vs native` / `Typed vs Core` ratios. Do not populate or commit measured values from a different machine merely to fill the new column; regenerate the table from one complete comparison run.
