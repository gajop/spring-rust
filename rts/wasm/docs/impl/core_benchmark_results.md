# Core Wasm benchmark results

Run: 2026-08-22, paired Core comparison runs, seed `424242`, five repeats.
Docker-built headless and legacy binaries.

Core replaces Lua, so Lua vs Core is the headline comparison:

| Profile | Core vs Lua |
| --- | --- |
| workloads | Core is faster in the measured rows |
| callouts | Core wins most rows; the 256-byte payload row loses |
| callins | differences are inside the observed noise band |
| DrawWorld | Core is slower in the current paired UI fixture |

The generated report is the authoritative data artifact. It records sample
count, p50, p99 and spread for every emitted row; the CSV baselines retain the
raw samples. Callin rows measure 2–10 µs operations with spreads comparable to
their medians, so individual callin wins and losses are not stable findings
without more repeats.

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

The current `callin_drawworld` run is Core 9609 ns versus Lua 2064 ns. Both
backends carry microsecond-scale spread on this row, so it is a target for
investigation, not evidence that a small optimization is real.

Rows marked `unavailable` by the harness remain unavailable. They are not zero
and are not treated as passing comparisons.

## Expanded callin run

The callin run measured eight rows. Seven were faster on Core; one was slower:

| Callin | Lua median ns | Core median ns | Result |
| --- | ---: | ---: | --- |
| `unitpredamaged` | 5484 | 2891 | Core faster |
| `allowunitcreation` | 4692 | 2699 | Core faster |
| `unitcreated` | 4889 | 2750 | Core faster |
| `empty` | 5498 | 3073 | Core faster |
| `gameframe` | 5339 | 3395 | Core faster |
| `unimplemented` | 1438 | 1454 | **Core loses** |
| `4modules` | 5600 | 4315 | Core faster |
| `update` | 4228 | 2844 | Core faster |

These are cold-cache engine-boundary medians with spreads between 1666 and
4487 ns. The spread is large enough that none of these eight rows supports a
stable percentage claim. The generated table remains useful as a regression
signal when compared against its frozen noise band.

The authoritative event inventory has 169 event declarations and 163 loaded
native symbols. The benchmark fixture currently reaches eight representative
rows, not all 179 names claimed by the older planning document. The variable
callin fixture is excluded from the table because its headless run does not
produce an `AddConsoleLine` row; the harness correctly refuses to turn that
missing observation into zero.

## Drawworld attribution

The repeated draw run measured the following nested diagnostics:

| Stage | Core median ns | Notes |
| --- | ---: | --- |
| complete `callin_drawworld` boundary | 9609 | spread 5046, p99 17196 |
| native dispatch boundary | 8820 | spread 4999, p99 15232 |
| core selection | 461 | spread 963, p99 2272 |
| core aggregation | 5999 | spread 3936, p99 11013 |
| module dispatch | 4762 | spread 3772, p99 9748 |
| Wasmtime entry / empty guest body | 2368 | spread 2299, p99 4351 |
| visibility-context diagnostic | 116 | spread 892, p99 4630 |
| argument marshalling | 0 bytes | DrawWorld has no arguments or return value |

Lua measured 2064 ns for the same UI `DrawWorld` boundary. The diagnostic
stages are nested, not additive: they must not be summed. The useful bounded
differences are approximately 789 ns from the complete boundary to native
dispatch, 1237 ns from aggregation to module dispatch, and 2394 ns from
module dispatch to Wasmtime entry. The remaining dispatch/binding work is
therefore located between those boundaries, rather than being one unmeasured
3822 ns block.

Lua's `empty` row is a synced GameFrame fixture, while `callin_drawworld`
is a UI DrawWorld fixture. They are different contexts and are not valid
baselines for one another. The fair comparison is the paired UI DrawWorld row;
the old 5498-versus-2322 comparison was a fixture mismatch, not evidence of a
Lua special case.

## Transport-class coverage

The Core-only ceiling run added representatives for every shape family that
the fixture can exercise without pretending they are Lua/Core peers:

| Representative | Core median ns | Shape |
| --- | ---: | --- |
| `fixed_struct` | 27.6 | fixed record input |
| `string_in_borrowed` | 11.0 | borrowed string input |
| `f32_list_in_borrowed` | 11.6 | borrowed numeric list input |
| `string_out_reuse` | 24.4 | caller-owned reusable string output |
| `list_out_reuse` | 494.0 | caller-owned reusable list output |
| `nested_list_out_reuse` | 30.5 | nested adapted output |
| `spatial_list_reuse` | 255.0 | spatial list output |
| `string_list_out_reuse` | 20.0 | adapted `list<string>` output |

The population inventory remains 806 fixed, 191 variable-input-borrowed, 90
variable-output-caller-owned, 89 dynamic-output-caller-owned, 71
handwritten-reviewed, 52 variable-io-borrowed-input/caller-owned-output, 22
variable-input-nested-adapted, 16 variable-input-borrowed-mixed-fixed, 13
fixed-option, and 4 variable-input-adapted. The ceiling rows are transport
diagnostics; the seven peer-comparison callout rows above remain the only
Lua/Core callout results in this run.

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
  --results rts/wasm/docs/generated/benchmarking_results.md
```
