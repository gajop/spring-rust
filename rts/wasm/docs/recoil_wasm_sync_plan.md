# Next sync tests: repeatable replays, live peers and host-engine stress

Updated 2026-10-19 after the [engine/history review](recoil_sync_history_2026-10-19.md).
The [verification guide](recoil_wasm_sync_verification.md) describes the contract;
the [measured report](recoil_wasm_sync_report_2026-10-19.md) describes results
already obtained. This file is **planned work**, not additional passing evidence.
The reviewed implementation/tooling is committed locally as `5daee98e45`.
No push, release, upstream merge or workflow dispatch is part of this plan.

## Record once, replay on every target

Yes: run a scripted simulation on Linux, save its complete recorded input stream,
and replay it on other operating systems/architectures. First replay it several
times on the recording machine. Cross-platform agreement means little if that
machine cannot reproduce itself. Later add Windows/ARM recordings replayed back
on Linux so Linux is not the only recording source.

For each case, retain exact engine build inputs/artifacts, game/map and Wasm
bytes, Wasmtime version/features, configs, start script, compiler/CPU/OS details,
cache provenance and all demo packets. Build each platform engine from the same
candidate source/configuration; compile guest Wasm once and distribute those
same bytes. Preserve existing input validation: CPU/ping packets can themselves
be simulation inputs, and must not be removed just to make results agree.

Compare every frame, not only a final hash:

- Engine checksums, required module identities/environments and fault status.
- Full Wasm memory, globals, tables and synced job state using existing tools.
- Targeted canonical engine state for path queues/costs/waypoints, RNG and object
  lifetimes when investigating a suspected checksum blind spot. These additional
  engine diagnostics are still to implement. Do not hash native padding or
  addresses, and do not claim the ordinary checksum is a full-state hash.

Replay covers deterministic execution. Keep live two-peer tests for transport,
backlog, joins, reconnect/rejoin and catch-up. A replay pass cannot replace them.

## What already exists and what is missing

| Work | Current state |
|---|---|
| Exact record/replay corpus, per-frame Wasm state, failure controls, Rules/Gaia, several modules, portable target kit | Implemented and committed locally; short Linux/Wine results recorded |
| Reuse a saved corpus without rebuilding the guest | Implemented: `wasm_adversarial.py --corpus` |
| Repeated fresh-process replay of one corpus at the same worker count | Possible now with separate invocations/output directories; not the same as suite `--repeats` |
| Busy HAPFS/QTPFS scenarios with terrain, construction, path deletion and repair | Need workloads and targeted engine-state capture; current minimal parity fixture is not this coverage |
| Long-duration, low-worker, UI/renderer, cache and lifecycle matrices below | Planned; no results claimed |
| Native Windows hardware and ARM acceptance for the expanded suite | Still required; Wine is a separate result |
| Cross-platform full-game and floating-point test infrastructure | Existing manual workflows/tools to reuse; do not introduce a second CI system |

`wasm_sync_suite.py --repeats` currently records a new match per repeat. It varies
inputs as well as execution and is **not** the same-corpus race test. The current
fixture uses HAPFS; it is not QTPFS coverage simply because worker counts vary.

## Work order

### 1. Reproduce the concrete host-engine risks first

These findings deserve small reproductions and code review before expanding a
large platform matrix. Keep separate cases and fixes; do not merge unrelated
public PRs as a bundle.

| Case | Work and evidence required | Reason/source |
|---|---|---|
| `heading-boundary` | Reproduce the old conversion at +/-pi and braking/turn wrap boundaries; review missing upstream #3342 against local code; preserve before/after regression | #3075 is already local, but [#3342](https://github.com/beyond-all-reason/RecoilEngine/pull/3342) is not |
| `hapfs-worker-slot` | Build/destroy blockers while estimator vertex costs update; inspect/assert real worker scratch ownership; repeat one demo with 2/3/4/6/8 workers and each count unchanged across repetitions | [#3530](https://github.com/beyond-all-reason/RecoilEngine/pull/3530) is an untested proposal; inspected local loop index differs from actual worker ID |
| `qtpfs-registry` | Create/delete/requeue paths after background search launch; demonstrate overlap or a sanitizer report; compare waiting versus deferred mutation without changing result order | [#3448](https://github.com/beyond-all-reason/RecoilEngine/pull/3448), externally reported replay failures at two workers; local code has the described access pattern |
| `external-path` | Call the actual enabled synced path API, require usable waypoints, release paths and continue simulation; cover invalid/deleted IDs and pending replacement | Separate crash [#3459](https://github.com/beyond-all-reason/RecoilEngine/pull/3459), plus untested HAPFS edge fixes [#3531](https://github.com/beyond-all-reason/RecoilEngine/pull/3531) |
| `pool-lifetime` | Review release/acquire pairs and queue-pointer lifetime; stress nested tasks, main-thread help, sleeping workers, reuse and shutdown; targeted TSan plus engine replay | Local `c24a0727aa` changes generic ThreadPool, so Wasm-only job tests cannot cover every user |

Use engine/Lua fixtures for engine-only path APIs, and official SDK fixtures for
available Wasm/native callouts. Confirm which bindings exist before promising a
Wasm version of a test. No downstream private game modification is needed to
reproduce a host-engine bug. Use public repro content where suitable and keep
its revision/license in the manifest.

For every candidate: record an unfixed failure, isolate its cause, preserve a
small regression and then run the same saved inputs after a fix. An inability
to reproduce is recorded as such; code inspection alone does not establish a
particular public issue's cause. Sanitizers can expose races/UB but a clean run
cannot prove determinism. Compare production builds too.

### 2. Extend the repeat/replay automation

Use one immutable corpus per scenario/seed/build. Run heavy cases serially in
fresh processes and isolated write directories. Add a resumable matrix manifest
that enumerates **corpus x platform x worker count x repetition x presentation**,
keeps failures, verifies artifact hashes before reuse and reports missing cells.
This matrix extension is planned; the existing suite resume option does not
supply all these axes.

Start with ten identical-corpus repetitions at each of 1, 2, 4 and 8 workers on
Linux; add 3 and 6 for scratch-slot and uneven task-splitting cases. Ten is an
initial stress budget, not a statistical proof. For flaky bugs, record failure
rates across enough runs to assess the fix rather than accepting one lucky pass.
Compare each run both to the recording and to the other same-count repetitions.

The existing CLI already supports separate same-corpus invocations. For example,
after a record run created `/tmp/wasm-record/corpus`, these replay the same bytes
twice with two workers (fresh output directories required):

```sh
python3 test/synctest/wasm_adversarial.py \
  --corpus /tmp/wasm-record/corpus --workers 2 \
  --output /tmp/wasm-replay-two-01
python3 test/synctest/wasm_adversarial.py \
  --corpus /tmp/wasm-record/corpus --workers 2 \
  --output /tmp/wasm-replay-two-02
```

Keep the existing 900-frame smoke cases. Add busy scenarios at 30,000 and 60,000
frames: [#2982](https://github.com/beyond-all-reason/RecoilEngine/issues/2982)
first differed at 55,801. At 30 sim frames/sec, these are about 17 and 33 minutes
of game time; actual wall time depends on replay speed and host capacity. Use
explicit 3,600/7,200-second process timeouts as initial limits and record timeout
separately from desync. The default 120 seconds is unsuitable for endurance
work. A long run of the minimal fixture alone does not substitute for path stress.

### 3. Add workloads drawn from past failures

| Case family | Vary while preserving the same synced inputs | Compare/review |
|---|---|---|
| `path-budget` | One peer issues unsynced path queries; many dirty paths; both pathfinders in separate cases | Synced cache contents, update order, resulting paths and checksums (#2279/#2370) |
| `path-lifetime` | GC/allocation pressure, explicit release, owner death, path repair/sharing, terraform, capture/gift and movement-definition changes | Queued versus visible results and deletion frame (#2691/#2695, June 2026 repairs) |
| `local-presentation` | Headless/rendered, low/high particle limits, renderer options, different selections/hover and read-only UI workload | Projectile IDs/lifetimes, dependency callbacks, RNG and build-power reads (#1174, old particle fixes, investigate #3373) |
| `map-cache` | Isolated cold/warm caches, duplicate model basenames, image typemaps, metal-spot derivation and resource read failures | Exact loaded content and derived terrain/path/model data; deterministic rejection on failure |
| `ordering` | Equal-cost paths/targets, mixed-case strings/locales, simultaneous sleep/job completion and callback reentry | Stable final tie breakers, list/callback order and RNG draws; inspect SDK adapters and Rust collection use |
| `numeric-boundary` | Scalar/SIMD/host math, heading wrap, NaNs/infinities/subnormals, missing/optional arguments | Bit results or deterministic errors before mutation; argument initialization and FP scopes |
| `fault-pressure` | Controlled guest fuel/traps, allocation/resource failures and worker errors | Expected explicit failures; no missing required module counted as a pass |
| `diagnostic-neutrality` | Logging on one peer, off on the other; state scans on/off; profiler enabled/disabled | Same sim, RNG and recorded input stream; profiler/logging must not add random draws (#4645) |

An unsynced widget that sends different orders changes the input stream. Either
replay the recorded orders or suppress its command-producing part for the
presentation comparison. Do not classify legitimately different orders as a
desync or hide the difference by filtering packets.

Inspect callback iteration/mutation, thread-local scratch, captured pointers,
registry/component lifetime and shared host buffers as well as tests. Background
jobs should consume a stable snapshot and publish results at a defined sim point.
“Running on the main thread” alone does not rule out a concurrent background reader.

### Identify whether a difference belongs to Wasm, the binding or the engine

For a reduced failing host-call sequence, run an equivalent Lua/engine fixture
alongside the official-SDK Wasm fixture where the APIs allow it. Log normalized
arguments, ordered results, callback order and errors without changing execution.
If both reproduce the same path bug, it is a shared host issue; if the host result
already differs at the boundary, inspect conversion, scratch storage and callback
reentry before blaming guest arithmetic. If inputs and host results agree but
guest state first differs, isolate the Wasmtime operation and FP environment.
These are investigation steps, not assumptions about which layer is correct.

Retain pure-Wasm fixtures as well: hidden globals/tables, memory and bulk
operations, scalar/SIMD numeric boundaries, traps and copied-job inputs. Add
SDK list-order/lifetime/reentry tests around real callins and callouts. Changing
a runtime optimization or compiler setting needs its own controlled candidate
comparison; do not change several settings while diagnosing a flaky replay.

### 4. Run the same corpus on actual platforms, then live peers

1. Linux x86-64 repeated replay, then native Windows x86-64 and Linux ARM64.
   Keep Wine as its own execution environment, never label it native Windows.
   Add compiler/build variants after the basic matrix is reliable. Use the same
   source candidate for cross-platform comparisons; different engine revisions
   may deliberately change simulation and need separate baselines.
2. Reuse `tools/sync-test` floating-point vectors and
   `.github/workflows/streflop-float-test.yml` logic. The present workflow compares
   live x86 SSE results against stored NEON reference data; this does not replace
   running ARM hardware for the candidate build.
3. Reuse the public BAR full-game synctest/startscript and existing manual
   `.github/workflows/synctest.yml` artifact comparison. Include representative
   path-heavy public replays in addition to the minimal Wasm guest. Record the
   actual BAR/map pin from the workflow when running; do not copy a stale README
   version into new evidence. No automatic CI expansion or dispatch is planned.
4. Retain live Linux/native-Windows pairs with delay/backlog, loss/reordering
   where supported, abrupt disconnect/rejoin and catch-up under ongoing commands.
   Compare per-frame state while both peers simulate the same recorded frames,
   and require agreement after catch-up. Preserve early joins and failure cases,
   not just a screenshot of the resumed match.
5. Add Windows/ARM-origin recordings replayed elsewhere. A prebuilt portable kit
   should need only the matching engine/dependencies and a lightweight test runner
   on the remote PC, not Rust or private game source.

## Completion and reporting

For each case record **planned / implemented / run-pass / run-fail / blocked**,
source scenario and corpus hashes, requested and actual workers, module set,
frame count, timeout, first differing frame/state and links to raw local evidence.
A crash, hang, missing row/module, changed input, or inspection error is a failure
or explicitly invalid run, never zero desyncs. Engine warning parsers themselves
need coverage (BAR #3869 only fixed detection).

When a difference appears, retain both peers' first dumps and the preceding
state/input window immediately. Engine warning time can lag the first divergent
path/cache value. Reduce the scenario while keeping the original immutable
failure. Use subsystem diagnostics to locate the first bad write, then inspect
its inputs, ordering and lifetime. Do not weaken thresholds or discard fields.

Acceptance requires the declared matrix to be complete, required modules present,
all positive comparisons exact, deliberate failure controls detected, and every
remaining failure named with its impact. Passing tests support the tested APIs,
workloads and platforms; they do not prove arbitrary games race-free. Broad Wasm
integration claims must state native Windows/ARM status and host-engine gaps.
Keep all private evidence local. Local commits are authorized; publication, push
and release remain outside the authorized scope.
