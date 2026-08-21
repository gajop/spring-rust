# Core Wasm benchmark results

Run: 2026-08-22, bounded suite, seed `424242`, scale `0.1`, timeout `30s`,
Docker-built headless/legacy binaries. The complete generated table was
written to `/tmp/core-bench-bounded.md`; this document records the headline
measurements and their limits.

The matrix ran real Lua, native, and Core rows for callins, callouts,
workloads, memory, heightmap, and draw profiles. Rows marked `unavailable` by
the established benchmark harness remain unavailable; they are not treated as
zero or as a passing comparison.

## Core versus native

| Profile | Core/native range | Notes |
| --- | ---: | --- |
| callins | 1.66–5.58× | eight deterministic callback rows |
| callouts | 1.01–4.39× | scalar, vec3, string, list, spatial, mutate |
| workloads | 1.04–3.53× | five non-rendering workload rows |
| heightmap | 0.96–1.64× where both exist | several Lua/native rows are unavailable |

Selected callout measurements (Core vs native): scalar `1.70×`, vec3 `3.25×`,
string `2.35×`, small-list `4.39×`, big-list `2.06×`, spatial `2.22×`, and
mutate `1.01×`. Core wins the mutate case and is near native on rules-parameter
and some heightmap rows; it loses on most callout/callin paths. That loss is
reported plainly because performance is a release requirement.

The suite did not enable mandatory fuel or epoch interruption. The default
runtime therefore measures the actual throughput path used by gameplay rather
than a security budget that would add per-call cost or create false-positive
crashes. Opt-in hostile/diagnostic budgets require a separate run and must not
be mixed into this baseline.

## Reproduction

```sh
python3 test/native_api_parity/run_benchmarks_core.py \
  --suite --bounded-suite --seed 424242 --scale 0.1 --timeout 30 \
  --spring-headless build-amd64-linux/install/spring-headless \
  --spring build-amd64-linux/install/spring \
  --results /tmp/core-bench-bounded.md
```
