# Wasm jobs: all environments and profiling, 2026-10-12

PASS for the local checks below. Code checkpoint:
`5b3d7b8ce27e501e0148ca3b4139df28ca0642fa`, on
`codex/wasm-synced-jobs` in `/home/gajop/worktrees/wasm-synced-jobs/spring-bar`.
This report identifies the development checkpoint used for the checks below.
The engine work, SDK, tests, and usage/profiling docs were subsequently combined
into one local integration commit on `rust-wip` in the main engine checkout, at
the user's request. Historical hashes below still identify the tested binaries.
No push, GitHub Actions, or changes to the Ship Game working checkout were made.

This extends the [original implementation/report](recoil_wasm_synced_jobs_report_2026-10-12.md).
Its 45,001-frame combat proof and 5.14x benchmark remain evidence for that report's
pinned `48038531…` binary at `4a2e544d27`; they are not relabeled as tests of this
new checkpoint. Current [API](recoil_wasm_synced_jobs.md) and
[profiling instructions](recoil_wasm_jobs_profiling.md) describe the extension.

## What changed

Both job imports now work in all seven Core worlds: RulesSynced, GaiaSynced,
RulesUnsynced, GaiaUnsynced, UI, Menu, and Intro. The synced worlds retain their
shared ordered queue and next-simulation-frame delivery. Every local submitting
module has its own IDs, quotas, queue, and Update boundaries. Local jobs progress
while simulation is paused; Intro uses DrawLoadScreen during loading. Local
initialization and input/draw callbacks can submit, with the documented window
latency. Cross-category and cross-system targets remain unavailable.

Local result callbacks preserve UI read visibility and Rml ownership. Module
unload and shutdown drain workers. Local execution failure faults the owner;
synced execution failure still aborts before partial batch publication. Private
worker settings/fuel, fresh instances, copied inputs/outputs, and forbidden host
imports apply in every world.

Intro dispatch now uses its own NativeInterface system. The active dispatch
pointer is thread-local, registry publication/lookup/teardown is synchronized,
and fault cleanup removes only the current system's modules. Private job workers
never take that registry lock. The replay stop diagnostic now stops a buffered
network-frame burst at its requested frame, fixing rendered-run overshoot.

Tracy zones identify queue/import costs, worker execution, the barrier,
collection, and result callbacks. Worker metadata includes owner, ID, logical
worker, export, source digest prefix, tick/frame, byte counts, and status.
Optional detailed stages cover setup, copies, execution, and teardown, including
early errors. Normal Tracy-disabled builds compile out zones/metadata and their
owner fields. Metadata formatting is guarded when no client is recording.

## Verification

All tests ran on this machine with the local build container; no remote runner.
Fixture overlays use an immutable Ship Game replay-library snapshot. The test
guest alternates slow/fast work and own/named submissions, checks global
isolation and input copies, validates ID/output ordering, and resubmits from
result callbacks. Admission into a different capability category is rejected.

| Check | Result |
|---|---|
| Normal/headless and rendered engines; matching profiling variants | Built; native job and manifest checks pass |
| Native queues, including initialization, input between updates, result chains, IDs, quotas, reset/cancel | 5,221 assertions, 8 cases |
| Import validation across all seven worlds | 111 assertions, 15 cases |
| New native queues under ASan/UBSan/leak detection | 5,221 assertions, 8 cases; no findings |
| New native queues under TSan | 5,221 assertions, 8 cases; no findings |
| Headless fresh recording plus 1/4-thread replay | All 601 rows equal, including recording comparisons; zero recorded desync warnings |
| Rendered fresh recording plus 1/4-thread replay | All 601 rows equal, including recording comparisons; zero recorded desync warnings; all seven guests produce results |
| Local jobs while paused | Headless sim frame 30 stays fixed; rendered sim frame 32 stays fixed; relevant local guests reach 100 results during the stable interval |
| Profiling replay, 1/4 threads | All 121 rows equal |
| Profiling versus normal engine, same demo | All 121 rows equal, including engine checksum and synced module/job hashes |
| Actual standalone loading Intro plus ordinary game modules | Rendered recording and 1/4-thread replay: all 121 rows equal; loading Intro returns results before simulation |
| Standalone Intro with concurrent game loading (`LoadingMT=1`) | Fresh recording and 1/4-thread replay: all 61 rows equal; Intro delivers 400 results before frame zero in every run |

The TSan build checks the instrumented native queue and runtime wrappers; the
vendored Rust runtime and generated guest instructions are not fully TSan
instrumented. Separate headless and rendered recordings contain different pause
input timings; equality is asserted within each pinned demo, not between those
two independent input streams. Fixtures do not replace production game logic.

## Profiling evidence and interpretation

An idle native benchmark capture contains 264 batches of 64 jobs, with 16,896
Worker zones. Each batch has exactly 64 workers between dispatch and completed
barrier. The last eleven batches use eight pool threads, 100,000 LCG iterations,
4 KiB input, and fuel 1,000,000,000. Their median wall duration is 1,433.636 us,
and median earliest-to-latest thread finishing spread is 174.368 us. One
representative batch distributes 7–9 jobs per thread and finishes 163.125 us
apart. These metrics are available from thread timelines; equal counts alone
do not establish equal work or speed.

For those instrumented batches, median summed stage time per job is setup
30.643 us, input copy 9.546 us, execution 103.693 us, output copy 0.316 us, and
teardown 23.105 us. These wall-clock numbers include tracing overhead and
descheduling. They explain what the tool can expose, rather than replace the
original unprofiled timing or prove the precise cause of its 5.14x scaling.
The benchmark still passes its 46,570 assertions with profiling enabled.

A fresh normal-build benchmark also passes 46,570 assertions and emits all 48
measurement rows. For the same long-work/fuel/input case, total time per job is
100.690 us at one thread, 53.004 us at two, 31.648 us at four, and 19.801 us at
eight (5.09x). These separately measured normal timings are retained alongside
the trace analysis; instrumented stage durations are not substituted for them.

A real headless replay capture contains 37,948 Worker and SubmitImport zones,
37,936 ResultCallback zones, and every detailed stage. The final undelivered
jobs are canceled at shutdown. The matching replay hashes prove the captured
metadata does not change synced state. This 20.98-second headless capture also
contains 59 million total engine zones, so the guide recommends short windows.

With explicit user authorization, `kernel.perf_event_paranoid` was changed from
4 to **2** and persisted in `/etc/sysctl.d/60-recoil-profiling.conf`. An own-process
`cycles:u` probe and a real benchmark sampling run succeed. JIT injection with
`perf record -k 1` resolves guest `work`/`pure`, Wasm trampolines, Wasmtime,
C++ job functions, and libc allocation/copy functions. The sample run records
1,165 samples with zero lost samples. Some stack transitions remain unresolved.
Its purpose is demonstrating usable attribution, not a new timing baseline.

Tracy timelines require no sampling permission. Linux CPU sampling, including
Tracy's sampling implementation, uses the kernel perf interface. This setting
allows owned user-space process sampling; it does not grant unrestricted
kernel/system-wide sampling. The tested Tracy build disables system tracing
and uses perf separately for JIT attribution.

## Reproduction and artifacts

The acceptance commands and hashes are in
[verification.json](data/jobs_followup_2026-10-12/verification.json), with native
sanitizer logs, build checks, pause markers, a perf report, and
[per-thread benchmark analysis](data/jobs_followup_2026-10-12/benchmark_trace_analysis.json).
Large raw traces and executables stay in ignored `out/local-sync` locally.

Normal headless SHA256:
`2026309d4f5a6e64c64b981426b9ce1f631c4b78fe5046858a6912e3448dc87e`.
Normal rendered SHA256:
`b0a50b4b181d7cfb49006e9b8e97fae772ada867bdb8b5ccf0ef9c69853b8b62`.
Profiling headless SHA256:
`048ac973b6f8fce77f57b2dce61820d750e956a91b383c4d79cdea09eef8b01a`.

The normal binaries match the accepted `runtime-all-env-stop-final` pins byte
for byte; the last source adjustment added only Tracy-conditional owner labels.
Verified copies are `out/local-sync/runtime-all-env-verified` and
`out/local-sync/runtime-jobs-profile-verified`. The engine's embedded version
remains the configure-time base; binary hashes above identify tested builds.

Prepare the fixture with `test/wasm_api/prepare_jobs_environments.py --game
<immutable snapshot> --output <new game.sdd> --pause`. Add `--loading-intro`
for a standalone loading package. Run the Phase 1 runner against the normal
headless or rendered engine with `--record-first --threads 1 4 --frames 600
--instruction-fuel 1000000000`. Rendered checks use Xvfb. See the profiling guide
for matching Tracy tools, short captures, CSV export, JIT dumps, and perf commands.
