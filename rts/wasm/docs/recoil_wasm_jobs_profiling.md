# Profiling Wasm jobs locally

The engine uses its existing Tracy integration. Job profiling is optional; a
`TRACY_ENABLE=OFF` build compiles out the zones, metadata, and owner-label fields.
Keep the normal build for representative timings and use a separate profiling
build to explain them. Debug symbols stored in an executable do not themselves
add per-job work. Frame pointers, JIT debug information, sampling, and recording
Tracy events can affect performance.

## Build and capture

From the engine checkout, configure a separate build with optimized code and
symbols. The additional job-stage option defaults to OFF:

```sh
docker-build-v2/build.sh --build-dir build-amd64-linux-jobs-profile --configure linux \
  -DCMAKE_BUILD_TYPE=RELWITHDEBINFO -DTRACY_ENABLE=ON -DTRACY_ON_DEMAND=ON \
  -DTRACY_NO_SYSTEM_TRACING=ON -DRECOIL_WASM_JOB_DETAIL_ZONES=ON
docker-build-v2/build.sh --build-dir build-amd64-linux-jobs-profile --compile linux
```

For CPU call stacks, add `-fno-omit-frame-pointer` and
`-mno-omit-leaf-frame-pointer` to the profiling build's C/C++ flags. Keep any
existing toolchain flags. These were enabled in the local verification build.

Build the capture and CSV tools from the **same vendored Tracy source** as the
engine, to avoid protocol/version mismatches:

```sh
cmake -S rts/lib/tracy/capture -B out/local-sync/tools/tracy-capture \
  -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=g++-13 -DNO_PARALLEL_STL=ON
cmake --build out/local-sync/tools/tracy-capture -j4
cmake -S rts/lib/tracy/csvexport -B out/local-sync/tools/tracy-csvexport \
  -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=g++-13 -DNO_PARALLEL_STL=ON
cmake --build out/local-sync/tools/tracy-csvexport -j4
out/local-sync/tools/tracy-capture/tracy-capture \
  -a 127.0.0.1 -o out/local-sync/jobs.tracy -s 10
```

Start the engine while capture waits, or connect the matching Tracy GUI. Select
a short representative interval. A headless run can execute millions of local
Update calls before simulation starts, producing very large captures. With
on-demand tracing, disconnected zones are inactive; formatted job metadata is
guarded by zone activity, and detailed stages check connection once per job.
The enabled profiling build still has instrumentation checks when disconnected.

## Are workers finishing evenly?

Inspect `Wasm::Jobs::Worker` on the thread timelines. Each zone has the job ID,
submitting owner, source-module digest prefix, export, input size, submission
frame/local tick, logical pool worker, and final status/output size. Local IDs
are per owner, so correlate using **owner plus ID**, rather than ID alone.
Logical worker zero is the calling thread, which also executes pool work.

For a batch, compare each thread's job count, summed worker time, and last job
end. The span from the earliest last finish to the latest last finish measures
the batch's completion imbalance. Also inspect idle gaps and whether some
threads have consistently longer jobs. Similar job counts do not guarantee
similar workloads or execution speeds.

`Seal` marks dispatch and `Barrier` marks waiting for the whole batch. The barrier
can include work performed by the calling thread; its duration is **not pure idle
time**. Workers may start inside Seal, and one-thread operation can finish its
jobs there. `Collect` covers conversion of validated batch outputs to deliveries.
Synced results arrive in submission order next sim frame; local results arrive
at their owner's next local boundary. CPU finish order does not change delivery.

## Where is the time going?

Coarse zones distinguish `SubmitImport`, `Admission`, `Seal`, `Worker`, `Barrier`,
`Collect`, and `ResultCallback`. ResultCallback includes scratch helpers, output
copy into the live instance, the guest callback, budget handling, and flushing.

With `RECOIL_WASM_JOB_DETAIL_ZONES=ON`, each worker also contains:

| Zone | Work covered |
|---|---|
| Setup | Store/linker creation, instantiation, export resolution, scratch helpers |
| InputCopy | Input bytes into private linear memory |
| Execute | Work export invocation |
| OutputCopy | Output range validation and copy to owned bytes |
| Teardown | Linker and store destruction, including early error exits |

These are wall-clock durations. A slow zone can include descheduling or waiting,
so use CPU samples to distinguish computation from elapsed time. Start with
coarse zones; enable detailed stages for a short capture when needed. There is
no separate worker logger or shared profiling aggregation lock.

State-check profiling has `Wasm::Sync::Metadata` and, only when a full scan is
explicitly enabled, its child `Wasm::Sync::FullMemoryHash` (with the memory byte
count). Normal play must have no FullMemoryHash zones. `WasmSyncLog` performs
expensive diagnostic scans every frame; disable it when measuring normal game
throughput. Full network memory checks are also disabled by default.

CSV export supplies names, source locations, start times, durations, and Tracy
thread identifiers:

```sh
out/local-sync/tools/tracy-csvexport/tracy-csvexport -u -f 'Wasm::Jobs::' \
  out/local-sync/jobs.tracy > out/local-sync/jobs-zones.csv
```

CSV thread identifiers are not necessarily OS TIDs. CSV does not export zone
text/job IDs; use the GUI for owner/ID correlation. Sequential native benchmark
batches can be grouped by Seal through Barrier end, with exactly 64 Worker zones
per batch. This grouping is unsuitable for overlapping local owner queues.

## CPU sampling with perf or Tracy

Tracy timelines answer when jobs run and how long stages take. CPU sampling
answers which instructions/functions consume CPU inside a stage. Tracy's Linux
sampling also uses `perf_event_open`; having Tracy does not bypass kernel
sampling permissions. This verification used Tracy timelines with system tracing
disabled, plus a separate perf run for Wasmtime's JIT symbols.

On this machine, the user authorized changing `kernel.perf_event_paranoid` from
4 to **2**, persisted in `/etc/sysctl.d/60-recoil-profiling.conf`. This permits
sampling one's own user-space processes; it does not grant unrestricted
system-wide/kernel profiling. Timeline instrumentation needs no such change.

For a focused native benchmark, run from a writable directory so JIT dumps and
generated symbol objects stay together. Substitute absolute checkout paths:

```sh
SPRING_WASM_JITDUMP=1 WASM_JOB_FIXTURE=/checkout/test/wasm_api/data/synced_jobs/worker.wat \
  RAYON_NUM_THREADS=2 perf record -k 1 -e cycles:u -F 499 --call-graph fp \
  -o jobs.perf.data -- /checkout/build-amd64-linux-jobs-profile/test/test_WasmSyncedJobs \
  '[.benchmark]'
perf inject --jit -i jobs.perf.data -o jobs.perf.jit.data
perf report --stdio --no-children -i jobs.perf.jit.data --sort dso,symbol
```

`-k 1` supplies the clock required by JIT injection. `SPRING_WASM_JITDUMP=1`
enables the runtime's existing Wasmtime JIT dump/debug-info option; private job
instances inherit that engine. The tested report resolved guest `work`, Wasm
trampolines, Wasmtime functions, C++ job functions, and libc allocation/copy
functions. Some stack links remained unresolved: symbols improve attribution
without guaranteeing complete unwinding through every JIT/native transition.
On hybrid CPUs, perf reports core and atom PMU events separately.

To investigate a game, replace the benchmark command with a pinned engine replay
command. Profile one process at a time. Compare matching inputs, fuel, worker
count, and job sizes, and report speedup from the normal build separately from
instrumented timings. Sampling alone cannot prove the cause of a scaling limit;
combine it with completion imbalance, stage timings, and repeated controlled
measurements.
