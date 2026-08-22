# Core Wasm benchmark results

This hand-written document records how to read the benchmark artifacts. It
does not duplicate measured values; generated output is the only source for
numbers.

The active comparison is
[`benchmarking_results.md`](../generated/benchmarking_results.md). It contains
only fully paired Lua/Native/Core medians. The generated
[`benchmarking_losses.md`](../generated/benchmarking_losses.md) is the short
loss list. Core-only transport ceilings and nested DrawWorld stages are in
[`benchmarking_diagnostics.md`](../generated/benchmarking_diagnostics.md), and
missing or unavailable peers are in
[`benchmarking_unpaired.md`](../generated/benchmarking_unpaired.md).

Raw samples, p99, spread, and run metadata remain in
[`test/native_api_parity/frozen_benchmarks/`](../../../test/native_api_parity/frozen_benchmarks/).
The old Component Model report and its CSVs are retained as historical
reference; they are not active comparison inputs.

## Interpretation

Core/Lua is the headline comparison. A loss is a row where the Core median is
not below the Lua median; the generated loss artifact is authoritative. The
comparison table deliberately does not hide missing backends by converting
them to zero or by treating an unavailable measurement as a win.

The benchmark producers use one configured repeat count for every timed row.
The runner validates that count before freezing baselines. CSVs retain every
sample and the derived p99 and spread; markdown stays deliberately compact.

DrawWorld stage rows are nested diagnostics and must not be summed. The fair
comparison is the paired UI DrawWorld row. A synced empty callin is a
different fixture and is not a DrawWorld baseline.

Fuel and epoch interruption are disabled for the throughput baseline. The
default gameplay path remains throughput-first; hostile or diagnostic budgets
must be measured separately and must not be mixed into this comparison.

The active fixture includes wide-argument and payload-scaling callout rows.
The full callin inventory is larger than the currently exercised deterministic
callin fixture; unmeasured inventory remains an open coverage task and is not
represented as fabricated green data.

## Reproduction

```sh
RECOIL_BENCHMARK_FREEZE=1 python3 test/native_api_parity/run_benchmarks_core.py \
  --suite --bounded-suite --repeats 50 --scale 0.1 --timeout 60 \
  --spring-headless build-amd64-linux/install/spring-headless \
  --spring build-amd64-linux/install/spring \
  --results rts/wasm/docs/generated/benchmarking_results.md
```
