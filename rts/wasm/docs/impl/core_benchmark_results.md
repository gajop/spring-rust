# Core Wasm benchmark results

Run: 2026-08-22, bounded draw suite, seed `424242`, scale `0.1`, timeout `30s`,
five repeats.
Docker-built headless and legacy binaries.

Core replaces Lua, so Lua vs Core is the headline comparison:

| Profile | Core vs Lua |
| --- | --- |
| workloads | 4.0–24.6× faster |
| callouts | 1.1–6.8× faster |
| callins | 1.5–2.4× faster |
| draw callouts | approximately 5.3× faster |

The callin rows measure 2–5 µs operations with approximately ±2–4 µs variance.
Those ratios are near noise. Callout and workload rows are tighter and are the
measurements to quote.

## Selected Lua versus Core measurements

| Profile | Test | Lua | Core | Lua vs Core |
| --- | --- | ---: | ---: | ---: |
| callouts | scalar | 32.592 ns | 11.000 ns | 2.96× |
| callouts | vec3 | 64.993 ns | 19.700 ns | 3.30× |
| callouts | string | 71.001 ns | 64.800 ns | 1.10× |
| callouts | small list | 636.458 ns | 94.000 ns | 6.77× |
| callouts | big list | 5288.125 ns | 1010.000 ns | 5.24× |
| callouts | spatial | 765.324 ns | 504.000 ns | 1.52× |
| callouts | mutate | 172.496 ns | 37.100 ns | 4.65× |
| workloads | unit scan | 0.417879 ms | 0.017000 ms | 24.58× |
| workloads | area effect | 0.364442 ms | 0.092000 ms | 3.96× |
| workloads | rules params | 0.491900 ms | 0.022000 ms | 22.36× |
| workloads | commands | 0.377910 ms | 0.055000 ms | 6.87× |
| workloads | compute | 1.535490 ms | 0.336000 ms | 4.57× |
| draw | draw callout | 77.000 ns | 14.688 ns | 5.24× |
| draw | UI draw workload | 0.031000 ms | 0.005875 ms | 5.28× |

## Native floor reference

Native is an in-process C call without the Wasm boundary. It is a floor
reference, not the replacement target.

Core/native ranges were 1.66–5.58× for callins, 1.01–4.39× for callouts,
1.04–3.53× for workloads, and 0.96–1.64× for heightmap rows where both
backends were available. Core loses to native on most rows; that is expected
from the boundary and remains useful for regression tracking.

The current `callin_drawworld` sample is Core 4434 ns versus Lua 2700 ns.
The prior samples were Core 4711 ns versus Lua 2090 ns and Core 5311 ns versus
Lua 2731 ns. Both backends carry microsecond-scale spread on this 2–5 µs row,
so this remains a profile target rather than an unverified optimization target.

Rows marked `unavailable` by the harness remain unavailable. They are not zero
and are not treated as passing comparisons.

Fuel and epoch interruption were disabled for this baseline. The default
runtime is throughput-first. Opt-in hostile or diagnostic budgets require a
separate run and must not be mixed into gameplay measurements.

## Reference floor

Naked Wasmtime Core crossings, measured outside the engine: about 4 ns/call
guest→host and about 11 ns/call host→guest. Engine paths are not comparable to
these directly — they include Spring dispatch, safety policy, and real
NativeInterface work — but they bound what the transport itself can cost.

## Reproduction

```sh
python3 test/native_api_parity/run_benchmarks_core.py \
  --suite --bounded-suite --seed 424242 --scale 0.1 --timeout 30 \
  --spring-headless build-amd64-linux/install/spring-headless \
  --spring build-amd64-linux/install/spring \
  --results /tmp/core-bench-bounded.md
```
