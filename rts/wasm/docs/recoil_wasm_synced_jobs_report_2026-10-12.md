# Local synced Wasm verification and jobs — 2026-10-12

## Scope and checkout

Implementation is in `/home/gajop/worktrees/wasm-synced-jobs/spring-bar`, branch
`codex/wasm-synced-jobs`, based on `rust-wip` commit `45c811a942`. Engine code, SDK,
runner, and tests are checkpointed in local commits `18667ed2c8`, `fb0af0c8ef`
(pool teardown), `bdf066118f` (synced replay-player visibility), and `20b1ef0a8e` (result-chain fixture), plus `d9e5f46861` (precise runner diagnostics) and `3812017ff2` (SDK result
borrow enforcement); the callback-fuel fixture refinement is `0d2040f31d`. The native threading audit
fixes and sanitizer test support are checkpointed in `9ae5fbe155`; queued-slice
lifetime accounting and its blocked-worker regression are `4a2e544d27`. These are historical development checkpoints. The completed work and its
documentation were subsequently integrated into the main `rust-wip` checkout as
one local commit, at the user's request. Historical binary hashes and test
provenance below remain unchanged. The original
engine's code was already committed, so no additional pre-work code checkpoint
was necessary. Its pending documentation was copied into this worktree for
context and kept out of the engine-code checkpoint. The accidental Planet Wars
map article, its diagram files, and its index links were removed; the Planetary
Annihilation article remains.

All builds, tests, benchmarks, recordings, and replay comparisons ran on this
machine. No GitHub Actions were changed or invoked. No repositories were pushed.
Ship Game's working checkout was not edited. The runner used frozen replay-library
game snapshots and overlaid only test modules/manifests in isolated run directories.

The implementation follows the supplied
[sync plan](recoil_wasm_sync_verification.md), Ship Game navigation proposal's
“Threads and determinism” section, and QTPFS's `ExecuteQueuedSearches` /
`SyncUpdatedPathsSystem` background wait and ordered application of results.
This report covers the requested local phases; it does not claim the plan's
three-platform acceptance matrix is complete.

## Implemented behavior

The engine hashes complete synced Wasm linear memories after every frame, sorted
by module name, and includes identity, size, environment, fault state, and immutable
job submission metadata in the existing sync checksum. Unsynced instances are
excluded. Network cadence is game-owned; enabling a local per-frame log cannot
change that cadence. JSONL diagnostics preserve the checksum before and after the
current Wasm contribution. The “engine” value remains the existing cumulative
checksum, including prior frames' Wasm contributions.

The local Python runner starts fresh headless processes at specified pool counts,
checks every frame and required module, and reports the first differing frame and
module. It rejects faulted/missing modules, gaps, truncation, absent SYNCCHECK,
unexpected actual worker counts, and recorded desync warnings. New-engine recording
is supported and every playback is compared to its recording on every frame.
Coarse recorded checksum warnings cannot hide the first precise module difference;
equal worker streams with warnings still fail unless explicitly allowed for a
legacy/negative test.
Memory dumps and a page/byte diff provide localization.

The synced job API copies submitted bytes, issues global IDs, shares immutable
compiled code, and creates a fresh private store/instance per job on the existing
engine thread pool. Its engine/configuration and fuel match the source synced
module. Workers have no usable host imports. After sealing at frame F's end, the
engine waits at F+1's start and delivers all results in submission order. Callback
fuel shares the ordinary new-frame budget. Limits, errors, cancellation, source
runtime lifetime, and new-game ID reset are explicit. `submit-to` supports a small
separate worker module, without copying the live rules module's state.

See the [API and diagnostics contract](recoil_wasm_synced_jobs.md). Core ABI,
Rust SDK wrappers, and result-export macros are included.

## Local verification results

| Check | Result |
|---|---|
| Phase 1, frozen short Ship Game scenario, no job fixture | PASS: frames 0–1200, identical engine checksums and Wasm hashes at 1 and 4 workers; fresh recording, no recorded desync warnings |
| Phase 2, eight jobs per frame, integer and f32 output checked against inline computation | PASS: frames 0–1200, identical hashes at 1, 2, 4, and 8 workers; fresh recording; fuel 1,000,000,000; no recorded desync warnings |
| Rust SDK result macro, same-module and named submissions | PASS: frames 0–60, 1 vs 4 workers; callbacks verify IDs, next-frame delivery, copied inputs, and fresh globals |
| Rust submitter plus distinct 64 KiB worker module | PASS: frames 0–600, 1 vs 4 workers; fresh recording, finite fuel, identical checksums and all three synced modules' hashes |
| Result callbacks resubmit the next batch without a GameFrame export | PASS: frames 0–600, 1 vs 4 workers, fresh recording; callbacks also read the current engine frame and burn fuel to verify per-frame reset |
| Synced player-query regression | PASS: old engine traps at replay frame 0; fixed engine passes frames 0–60 at 1 and 4 workers, with no recorded warnings |
| Full Hammer and anvil vs Peltast combat scenario plus eight jobs per frame | PASS: all 45,001 frame records (0–45000) identical at 1 and 4 workers and against the fresh recording; finite fuel 100,000,000,000; no recorded desync warnings |
| Deliberate desync | PASS: first difference frame 17, `sync-jobs-test`; same module SHA-512, different memory hashes; dump differs at offsets 16 and 17 on page 0 |
| Job runtime/real-pool tests | PASS: 4,656 assertions in 7 cases |
| Existing Wasm manifest and engine thread-pool tests | PASS: manifest 111 assertions in 15 cases; additional job-import tests check both signatures and all environment masks |
| AddressSanitizer + UBSan + leak detection, native job tests | PASS; 4,656 assertions with the prebuilt Wasmtime library |
| ThreadSanitizer native pool suite | PASS: 2,035,923 assertions in 16 cases, including plain-result publication and 270 cached-group reuse cycles at 1/2/4 threads |
| ThreadSanitizer native job suite | PASS: 4,656 assertions in 7 cases, with the test-only Wasmtime limitations below |
| Runner validation/source-preservation tests | PASS: 10 cases |
| Rust SDK Wasm checks, alloc build, and Clippy; Rust guest build/Clippy | PASS |
| Rust result-borrow compile-fail check | PASS: ordinary handler compiles; a handler requiring permanent borrowed bytes fails with E0308 |
| Per-job benchmark equality checks | PASS: 46,570 assertions; 48 measurements preserved |

The full combat recording reaches frame 45,000 with 192 unit creations,
23,672 projectile creations/destructions, 19,338 damage events, and 79 unit
destructions; peaks are 113 ships and 139 projectiles. This verifies the scenario
contains sustained game activity. The final queue-drain build records the same complete world trace (SHA-256
`74fbb790ec552aaffa36b90cd76234127ca3b75b72335c740b120c51aa55487c`).
Both complete playbacks match the recording's engine checksums and both synced
modules' hashes at every frame, with no recorded desync warnings. The fresh demo's
SHA-256 is `e0651edfe2ebf1a4d1d0cd8a23c8c2a28ba1d49c4d569d5c0e4ce20666102465`.
Each process delivers 360,000 job callbacks by frame 45,000, for 1,080,000 across
the recording and two playbacks. Each process's final eight submissions are
drained/cancelled at shutdown.

The eight-job game fixture deliberately makes its first job much slower than
later jobs, mutates scratch after admission, and checks every returned value,
private mutable-global count, frame stamp, monotonically increasing ID, and float
bits against inline work. Admission self-tests cover an unsynced target, an empty
export name, and an out-of-bounds name. Any wrong outcome traps the fixture; its
presence is required every frame, so two identically failed runs cannot pass.
Frame 1200 delivers 9,600 jobs; its final eight submissions are drained/cancelled
at shutdown. Thread counts include the simulation thread.

The result chain exports no `GameFrame`. Its callbacks burn 500,000 LCG iterations
each, so 600 frames consume more than one billion guest instructions and cannot
pass without the new-frame reset. A negative variant raises each callback to
50 million iterations under a one-billion frame allowance: the first callback
submits ID 3, and the second exhausts fuel at frame 1. The surviving rules module's
job hash is `971560b110149498`, the hash of next ID 4, proving that first callback
finished. The runner rejects the missing fixture instead of accepting two faulted
streams. Both callbacks therefore share the frame allowance rather than receiving
separate fuel grants. Reproduce the negative WAT by replacing `500000 call $burn`
with `50000000 call $burn` and recording only through frame 2.

The Rust result macro coerces its handler to a function accepting any callback
borrowed lifetime. A handler requiring `&'static [u8]` is rejected, preventing safe
code from retaining an engine-reused result slice. The normal and negative guest
checks run via `test/wasm_api/check_synced_jobs_result_lifetime.py`. This enforcement
has zero generated-code overhead in the tested guest: rebuilding its external-worker
Wasm produces the same SHA-256 as before the type check.

The sanitizer build instruments the native C++ queue/runtime wrappers, not the
prebuilt Wasmtime Rust library. Its initial static-C++-runtime link crashed during
CPU-topology startup before tests; disabling `HAVE_STATIC_LIBSTDCXX_FLAG` and
`HAVE_STATIC_LIBGCC_FLAG` produced a shared-runtime link that passes ASan/UBSan
with leak detection. These flags affect only the separate sanitizer build.

The ThreadSanitizer audit found real native races in Linux wake-word access,
contended mutex unlock, worker exit flags, task deletion, and a post-enqueue
virtual call on self-deleting tasks. The wake word now uses lock-free `atomic_ref`
with its raw kernel layout preserved; contended unlock stores atomically, and
exit flags are atomic. Completion and final-worker-access checks acquire their
release publications. Notification metadata is read before enqueue makes a task
available for deletion. The acquire on task completion also explicitly publishes
job result writes to the next-frame reader. The new plain-memory regression
checks 270 batches at each of 1/2/4 threads, exceeding the 256 cached task slots.
The complete native pool suite passes TSan without test-side serialization or
race suppressions; earlier failing logs are retained. A further regression blocks
one worker while the waiter completes all callbacks. The old pool returns with a
raw slice-group pointer still queued, making later cached-group reuse unsafe. It
fails two assertions in this regression. Slice groups now count outstanding queue
entries, including rescheduled background entries, and the barrier acquires their
final releases before reusing the callable or consuming results. The blocked-worker
regression and all nested pool tests pass with this change.

The TSan job suite uses GNU link wrappers only in its separate sanitizer target:
Wasmtime compilation is synchronous and VM instantiation/deletion share a
lifecycle mutex. These make uninstrumented Rust compiler ownership transfers and
VM address reuse observable to TSan. Work exports and native job dispatch, result writes, ordering, and cancellation
still use real parallel engine workers. Instantiation includes any Wasm start
function and is serialized in these TSan tests.
This is narrower than instrumenting Wasmtime itself: neither its Rust internals
nor JIT guest instructions are checked. `report_signal_unsafe=0` disables its
signal-unwind allocation warning, not data-race reports. The process is launched
with `setarch x86_64 -R` in a `seccomp=unconfined` container because this host's ASLR
layout otherwise prevents TSan startup. No host-wide ASLR setting was changed.
GCC also warns that TSan cannot model `atomic_thread_fence` inside the vendored
concurrent queue; a clean run is evidence for these exercised schedules, not a
formal proof of that queue. Normal builds and timings contain none of the test
wrappers.

Unit tests additionally cover NaN canonicalization, forbidden host calls, finite
fuel exhaustion/replenishment, output/input bounds, memory growth limits, source
runtime destruction, global order across two owners, batch/aggregate admission
limits, teardown during background work, and ordered delivery of guest failures. A 270-cycle
reload test exceeds the thread pool's 256 cached task groups. SpringApp now drains
jobs before stopping/recreating its pool, including exit with a final undelivered batch.

Initial finite budgets exposed real failures and were rejected: 100 million fuel
faulted the short game's rules at frame 270 and the combat game's rules at frame
194. One billion fuel faulted a combat route search at frame 8,010. These artifacts
remain in `out/local-sync/final-1200`, `final-combat-45000`, and
`final-combat-45000-fuel1b`; none count as acceptance. Increasing the scenario's
finite allowance changes no game code or worker determinism settings. Small-budget
exhaustion remains covered independently.

The combat scenario initially exposed a pre-existing native player-query leak:
replay playback adds a local observer, and Wasm `GetPlayerList` included that
observer although Lua's synced equivalent excludes it. The resulting guest
allocation difference changed six bytes on pages 15 and 19 at frame zero.
`GetPlayerInfo` and related player queries now apply the same recorded-player
visibility rule, using a nested thread-local synced-import context that also works
without SYNCCHECK. Unsynced imports retain access to replay observers.
The targeted [player regression fixture](../../../test/wasm_api/data/synced_jobs/replay_players.wat)
checks the actual engine API, not just its policy predicate: it traps on the old
engine at playback frame zero and passes on the fixed engine. Complete 512 MiB
frame-zero dumps now have the same SHA-256 for recording, one-worker playback,
and four-worker playback. The combat smoke run also passes strict recorded checksums.
Earlier failed/truncated diagnostic runs remain in `out/local-sync/`; they do not
count as acceptance and no bytes are masked or excluded from hashing.

The deliberate desync fixture opts into the existing test-only synced timer
surface with `SPRING_ENABLE_SYNCED_TIMERS=1`, writes that timer into memory only
at frame 17, and runs the same binary in both processes. Its old replay uses the
explicit legacy-checksum exception. Ordinary final acceptance uses new recordings
and no exception. The fixture is absent from the actual game checkout.

## Measured job costs

Machine: Intel Core i7-12700, 12 physical / 20 logical CPUs, Linux amd64, GCC 13.1
RELEASE, Wasmtime 42.0.1. Pinned Docker build image:
`sha256:31b3667ba0d37620b4ad9b45d1840c8d3e95317f2b233044d2e17697ce8fe61d`.
Benchmark worker memory is 128 KiB, inputs 64 B / 4 KiB / 64 KiB. It compares the
same pure LCG work in the same runtime/configuration, called inside a persistent
guest instance versus freshly instantiated jobs. Every isolated output is compared
to the inline output before reporting a sample. Compilation is amortized separately.

Selected medians, fuel enabled (1 billion), 4 KiB input, 12 B output, in µs/job:

| LCG iterations | Inline | Instantiate/setup | Input copy | Dispatch/work | Output copy | Isolated total |
|---:|---:|---:|---:|---:|---:|---:|
| 0 | 0.005 | 3.542 | 1.971 | 0.045 | 0.047 | 9.957 |
| 1,000 | 0.875 | 3.531 | 1.967 | 0.923 | 0.037 | 10.829 |
| 100,000 | 86.013 | 3.366 | 1.883 | 85.332 | 0.036 | 94.800 |

Setup includes store/linker/instance creation, resolving exports, and input helpers.
Input copy includes first-touch memory cost; output copy includes allocating owned
result bytes. Total includes teardown. Per-stage medians need not sum to the total
median. Inline timing is a batch of 301 guest calls divided by 301; it excludes
transport/setup. Isolated timings discard 10 warmups and take 301 samples. The
clock's cost is material for tiny dispatch numbers. Compilation was about 1.93 ms
with fuel enabled in this run. The earlier no-fuel series was colder and has higher
setup times; it should not be interpreted as fuel improving instantiation.

For 64-job batches with 4 KiB input and 100,000 iterations, queue total wall time
divided by jobs, fuel enabled:

| Pool threads | µs/job | Relative to one-thread queued run |
|---:|---:|---:|
| 1 | 96.749 | 1.00× |
| 2 | 51.658 | 1.87× |
| 4 | 28.273 | 3.42× |
| 8 | 18.832 | 5.14× |

These totals include admission copies, pool dispatch, worker execution, barrier,
and creation of engine-owned deliveries; they exclude the live Wasm submit-import boundary, result callbacks, and per-frame state hashing.
Each queue measurement is a median of 11 batches. At four threads, admission was
0.178 µs/job, sealing/dispatch 0.011 µs/job, and the barrier interval 28.034 µs/job.
At one thread, work executes during sealing. Empty/tiny jobs do not scale usefully:
eight threads cost about 12.3 µs/job with no computation. A 64 KiB echo measured
13.763 µs input copying and 1.888 µs output copying with fuel enabled.

Coarse jobs can amortize the roughly 10 µs fixed cost of this small module;
fine-grained per-ship submissions would be expensive. A 512 MiB source module is
not represented by these setup numbers. Use a small worker and batch meaningful
work. These are local microbenchmarks, not a measured navigation implementation or
an end-to-end Ship Game speedup. Concurrent game runs were not active during the
captured benchmark; CPU frequency/first-touch variation still limits precision.

## Reproduction and evidence

Small machine-readable results, provenance, benchmark rows, and test logs are in
`data/synced_jobs_2026-10-12/`, including the
[benchmark rows](data/synced_jobs_2026-10-12/benchmark.json),
[four-count replay result](data/synced_jobs_2026-10-12/short-result.json),
[full combat replay result](data/synced_jobs_2026-10-12/combat-result.json),
[combat provenance](data/synced_jobs_2026-10-12/combat-provenance.json),
[complete log summaries](data/synced_jobs_2026-10-12/final-queue-drain-combat-45000-logs.json), and
[desync offsets](data/synced_jobs_2026-10-12/desync-memory-diff.json). Full configs, fresh demos,
JSONL logs, game overlays, console/infolog files, and dumps remain under
`/home/gajop/worktrees/wasm-synced-jobs/spring-bar/out/local-sync/`.
The final engine, including the native threading fixes, is preserved as
`runtime-queue-drain-final/spring-headless`, SHA-256
`48038531af98d2c708111b976adb00d3a0c137dcd48d617e8bb189d4695ccf92`.
The earlier native threading build is `runtime-native-final/spring-headless`,
SHA-256 `c1271d35aa4f2a50c6401eec9ae511633c4f1a55d612bcfd90f58bbb7988b3ff`.
The preceding player-visibility build is `runtime-player-visibility/spring-headless`,
SHA-256 `a47e1f04158c1c69f7ac3f5058a9c39f1745c3bc99308fa273d3b14e32ad7bef`.
Earlier accepted runs used `runtime-verified/spring-headless`, SHA-256
`da2ffb86523c590ee4dcf609887b374d953f257ee077df729606aee88f5efef7`;
the subsequent teardown build is preserved as `runtime-final-shutdown/spring-headless`,
SHA-256 `d38a110d79420c33029f7c8b9455197fefe7ff2a96ce6487041868f232a8b8e6`.
Each provenance file identifies the exact binary used.
The WAT job fixture's SHA-256 is
`d2578f91b82c906f3b695b472247fe423252f41445cd6c994f561abd0e3d5d8c`.
Every run records its source demo and newly recorded demo hashes. Logs include
each game module's full SHA-512 identity.

Configure locally with the pinned Docker build scripts and initialize submodules:

```sh
git submodule update --init --recursive
SKIP_RUST_CHECK=1 docker-build-v2/build.sh --configure linux -DCMAKE_BUILD_TYPE=RELEASE \
  -DBUILD_spring-legacy=ON -DBUILD_spring-dedicated=OFF -DBUILD_TESTING=ON
```

The host-wide Rust precheck requires libclang, unavailable in the host environment;
C++ configuration used `SKIP_RUST_CHECK=1` and the changed SDK/guest crates were
checked separately. No generated NativeInterface API declaration was changed. The legacy target must
be configured for this headless build. Compilation/tests
were run directly with CMake inside the pinned build container, targeting
`engine-headless`, `test_WasmSyncedJobs`, `test_WasmModuleManifest`, and
`test_ThreadPool`. The wrapper's automatic install encountered an unrelated
unbuilt `mimalloc-obj` install target; the verified headless binary itself builds
and runs. Its runtime directory reuses the original install's `base` and `fonts`.
No original engine binary was overwritten.

From this worktree, benchmark and runner unit tests:

```sh
WASM_JOB_FIXTURE="$PWD/test/wasm_api/data/synced_jobs/worker.wat" \
  RAYON_NUM_THREADS=2 build-amd64-linux/test/test_WasmSyncedJobs '[.benchmark]'
python3 -m unittest discover -s test/synctest -p test_wasm_sync_replay.py -v
python3 test/wasm_api/check_synced_jobs_result_lifetime.py
wat2wasm test/wasm_api/data/synced_jobs/jobs_in_game.wat -o /tmp/jobs.wasm
```

The TSan build uses `-fsanitize=thread -fno-omit-frame-pointer` in both
`CMAKE_C_FLAGS` and `CMAKE_CXX_FLAGS`, `-fsanitize=thread` in
`CMAKE_EXE_LINKER_FLAGS`, and both static-runtime flags set to `FALSE`. Inside the
same pinned container, with its separate build mounted at `/build/out`, run:

```sh
TSAN_OPTIONS=halt_on_error=1 setarch x86_64 -R /build/out/test/test_ThreadPool
TSAN_OPTIONS=halt_on_error=1:report_signal_unsafe=0 \
  WASM_JOB_FIXTURE=/build/src/test/wasm_api/data/synced_jobs/worker.wat \
  RAYON_NUM_THREADS=2 setarch x86_64 -R /build/out/test/test_WasmSyncedJobs
```

The container needs `--security-opt seccomp=unconfined` for the process-local
`setarch` operation. The [complete passing native sanitizer log](data/synced_jobs_2026-10-12/queue-drain-tsan.log)
and preceding failure logs preserve the audit evidence.

Exact replay invocations are stored in each `provenance.json`, including frozen
game/demo paths. The final queue-drain build repeats acceptance in
`final-queue-drain-1200`, `final-queue-drain-nojobs`,
`final-queue-drain-rust-worker`, `final-queue-drain-result-chain`, and
`final-queue-drain-desync`. Canonical small result/provenance files identify the
exact build and run. The full final recording and replay pair is
`final-queue-drain-combat-45000`.

Earlier completed cases remain in `final-native-*`, `final-player-visibility-*`,
`final-rust-result-lifetime`, and `final-result-chain-fuel`. The interrupted
`final-native-combat-45000` recording was stopped for the queue-drain fix and is
not acceptance. For the full run, use `--frames 45000 --timeout 10800 --threads
1 4 --record-first --instruction-fuel 100000000000` with the frozen
`20261010-125550-hammer-and-anvil-vs-peltast-skirmish` game/demo and `/tmp/jobs.wasm`.
Always choose a new output directory.

The Rust fixture builds with `cargo build --locked --release --target
wasm32-unknown-unknown` in `test/wasm_api/guests/synced_jobs_guest`; add
`--features external-worker` and runner `--worker-fixture` compiled from
`dedicated_worker.wat` to check a separate module. SDK checks use its standalone
`--manifest-path rust/crates/spring/Cargo.toml`; the root Rust workspace excludes
that crate. Its pre-existing alloc-enabled CUS compile-test visibility warning is
outside this change; the normal alloc build and no-alloc job-export compile test
both pass with warnings denied.

## Verification limits

This establishes repeatability across worker counts on this Linux machine with
the tested pinned games and modules. Windows, ARM64, different CPUs, rendered
clients, and two live network peers were not verified here. Linear-memory hashing
does not observe live Wasm globals/tables or replace serialization/save-load
support. Full-memory hashing of Ship Game's 512 MiB every frame is intentionally
expensive diagnostic coverage, and is not a free production setting. XXH3-64 and
the existing engine checksum are diagnostics, not collision-proof proofs.

The jobs carry explicitly copied input; this does not parallelize Ship Game's
actual navigation code. Adopting the facility for real navigation requires a small
worker, a frozen input schema, appropriate job sizes, and the same replay checks.
No readiness, wall-clock deadline, worker-count decision, shared memory, or worker
host access enters the synced API.
