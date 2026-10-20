# Same-frame pure job batches — 2026-10-19

Historical implementation/test record. The selected feature is now committed locally; see [the session integration report](ship_game_session_2026-10-20.md) for final scope and checks. Capture-time installation and commit statements below are historical.

Codex-authored experimental implementation, unreviewed for upstream. All engine
and SDK changes remain uncommitted and unpushed in the private engine worktree.
Nothing is installed or enabled in the canonical game/engine checkout. This is
an execution primitive; no gameplay equivalence or performance gain is implied.

## Contract

The public SDK uses matching timing names: `run_this_tick` joins a batch and
returns its ordered results before the current callback continues;
`run_next_tick` queues one job for ordered delivery before the next callback.
A tick is a synced GameFrame or an unsynced Update. `submit` and `run_batch`
remain deprecated forwarding aliases; the engine wire ABI is unchanged.

```rust,ignore
use spring_sdk::jobs::BatchJob;
let jobs: Vec<_> = packets.iter().map(|packet| BatchJob {
    input: packet,
    max_output_bytes: reply_bound,
}).collect();
let results = spring.jobs().run_this_tick(MOTION, &jobs)?;
// results[i] is Result<Vec<u8>, JobError> for jobs[i].
```

`MOTION` is an existing `export_jobs!` worker token, optionally selected with
`.in_wasm_module("worker-module")`. A batch selects one worker/export for all
inputs. The engine copies every input, executes each in a fresh private store,
joins the ordinary ThreadPool, validates the entire barrier, and returns owned
results in input order. Pool width 1 runs on the caller thread. Workers retain
the existing deterministic Wasmtime configuration and import-as-trap policy.
No worker reads host state, clocks, RNG, other jobs or application globals.

The call does not invoke `spring:jobs/result`, call any caller scratch export,
flush CUS calls, or collect next-frame deliveries. Existing deferred submissions
keep their IDs and F+1 delivery boundary. Compatible target-module visibility
uses exactly the existing `submit-to` policy. A retained compiled program owns
its engine/module independently of its source owner. The ordinary active
callback owns the caller's lifetime; queues remain confined to their engine
thread. This API does not make module loading/unloading concurrent with an
active callback safe.

Admission validates the complete batch before committing quota or launching
work: 1–256 jobs, at most 1 MiB per input/output reservation, 16 MiB total input,
and 16 MiB total synchronous output reservations per frame/update window.
Synchronous and deferred jobs share the existing 256-job/16-MiB input quotas.
Output reserves the caller's declared bounds before execution; excess output
becomes that job's `InvalidOutput` status before a host output vector is copied.
Invalid descriptors and quota rejection admit nothing. Import validation and
reserved wire bytes still consume ordinary host-work budget.

Guest failures occupy ordered error slots (invalid export, guest trap/import
attempt, scratch-input overflow, malformed/oversized output). C++ allocation or
runtime failures are not ordinary results: the import returns a trap to unwind
the C ABI, and the synced queue retains the exception for its native frame
boundary to rethrow. Native resource failure must not become a completed frame
with a locally missing/faulted rules module. Reset at shutdown clears it.

The helper restores the calling thread's prior ThreadPool MT flag after the
join, including guest-failure paths. It does not change callback, CUS or UI
nesting flags. A nested attempt to advance/reset/remove a running queue is
rejected. Worker imports cannot re-enter the queue.

## ABI

One handwritten jobs-extension import follows the existing jobs binding pattern:

```text
spring:jobs/run-batch-to(
  module_ptr, module_len, export_ptr, export_len,
  input_ptr, input_len, output_ptr, output_capacity
) -> i64   // high u32 Core status, low u32 actual output wire length
```

An empty target name selects the caller. Input wire is LE u32 version=1, count,
then `(input_len, output_limit, input_bytes)` for each job. Output is LE u32
version=1, count, then `(job_status, output_len, output_bytes)`. Native pointers
and struct layouts never cross the boundary. The host preflights the entire
guest output reservation before execution; too-small output capacity rejects
without admission. All inputs are owned before output is written, so guest
input/output aliasing is valid. The safe Rust wrapper handles framing, bounded
allocation and complete response validation; applications contain no ABI unsafe
code. Its native backend joins the existing SDK pool and returns owned data.

## Validation and rollout

Source tests cover pool widths 1/2/4/8 with deliberately skewed work, fresh
stores, compiled-program lifetime, preserved deferred IDs/delivery, shared
quotas and atomic rejection, Update-window accounting, traps/import attempts,
output limits, malformed LE frames, MT-flag restoration and retained host
failure. Native SDK tests cover ordered owned results, deferred coexistence,
quotas, failures, active application callback/reborrow and borrow rejection.

The isolated real-import fixture is
`test/wasm_api/data/synced_jobs/batch_in_game.wat`. It checks mixed ordered
failures, overlapping input/output, descriptor/capacity/range rejection, fresh
globals, ordinary imports after the join, and subsequent F+1 delivery. It emits
`WASM_BATCH_IMPORT_PASSED` at frame 1 only after its assertions complete.

Focused Linux checks passed:

- C++ `test_WasmSyncedJobs`: all 15 non-benchmark cases, 6,212 assertions,
  including existing deferred-job coverage. Each case ran in a fresh process.
  An aggregate run reproduced a Catch/Wasmtime POSIX signal-handler conflict:
  Catch's handler between cases intercepted an intentional guest trap as SIGILL.
  The same trap case passed alone (36 assertions). The aggregate failure and
  all isolated logs are retained; no aggregate `ctest` pass is claimed.
- `cargo test --locked -p spring-sdk -p spring-sdk-internal --features
  spring-sdk-internal/alloc -- --test-threads=1`: SDK 59 unit tests, internal 28
  unit tests, one integration test and nine doctests passed. Four doc examples
  remain ignored. Initial bindgen discovery failed before source compilation;
  rerunning with installed `LIBCLANG_PATH=/usr/lib/llvm-18/lib` passed.
- The same packages passed `cargo clippy --target wasm32-unknown-unknown
  --all-targets -- -D warnings` with the alloc feature. `git diff --check` and
  targeted Rust formatting checks passed. The Python fixture source parses.

Raw logs and the isolated-case receipt are in private
`build-headless-perf/same-frame-jobs/`; `cpp-summary.json` records all assertion
counts. Tests use the existing private scratch build and private Rust cache.
No control binary or install was overwritten. The one-job private headless
incremental build linked successfully. Its separately staged binary is
`build-headless-perf/same-frame-jobs/candidate/spring-headless`, SHA256
`7193a7700aa9591b4db6b4273417e8b44758f101be83469c254bfe3af895632b`.
The old `57b9dd6f...` control was hashed and remains unchanged. The candidate
includes earlier private common/headless drawing changes as well as this API:
game serial/batch comparisons must use this same new engine on both sides.

The actual-import fixture then passed at configured pool widths **1 and 4**,
both exit 0. Native logs prove requested/effective pool widths before each
`WASM_BATCH_IMPORT_PASSED` marker. It verifies active-guest -> fresh-store ->
active-guest execution at width 1, deterministic mixed job errors, overlapping
buffers, preflight rejection, ordinary imports after return, and preserved F+1
delivery. `import-check-v1/` retains generated Wasm, command/config/start
scripts, copied minimal game, logs and `results.json`. This is correctness
evidence; no timing/performance comparison was made.

The candidate directory's `spring` is a stable symlink to the preserved
interactive control, solely for headless benchmark hashguard compatibility.
It has **not** been rebuilt with this API and must not run same-frame guests.
`candidate-runtime.json` records the separate binary identities and successful
`ldd` dependency resolution for both. No interactive execution, Windows test or
platform/replay equivalence claim is made for this API.

Only the coordinator-authorized private opt-nav-worker `target/debug` and
`target/wasm32-unknown-unknown/debug` compiler caches were removed under EX to
make room. Exact per-file hashes/sizes and preserved release/frozen Wasm
before/after checks are retained in `cache-cleanup.json` and its two manifests.
All preserved artifact hashes matched. No release artifact, frozen runtime,
old engine control, canonical checkout or user asset cache was removed.

Real game integration needs explicit immutable snapshots and an audit of live
read/write order; simply preparing all ships and applying all outputs later can
change observed headings. Platform/replay equivalence and measured benefit are
required before any default adoption. Main keeps the verified serial path.
