# Synced Wasm: contract, evidence and verification

Lockstep sync is a required property of the Wasm backend. A RulesSynced or
GaiaSynced module must produce the same simulation results on every peer when
it receives the same module, engine configuration and simulation inputs.
This requirement applies to guest arithmetic, engine callouts, callbacks and
background jobs.

The engine-owned tests now provide evidence for broader Wasm integration on
x86-64 Linux and the Windows executable under Wine. They exercise the official
`spring-sdk`, real networking and recorded playback independently of any game.
The [2026-10-19 report](recoil_wasm_sync_report_2026-10-19.md) contains the measured
matrix, a reproduced floating-point bug, its regression checks and portable
raw evidence. The FP fix and test suite are committed locally in `5daee98e45`;
they are not part of an already released engine.

Native Windows hardware and ARM remain acceptance work for this suite. A
passing finite fixture is evidence for the tested paths, not proof that every
possible module or host call is deterministic.

## Follow-up plan and historical engine failures

The [next test plan](recoil_wasm_sync_plan.md) adds immutable cross-platform replay
corpora, repeated playback at the same worker count, long pathfinding stress and
live reconnect coverage. Its [historical review](recoil_sync_history_2026-10-19.md)
links actual engine fixes, public reports and current code-inspection findings.
Those are planned tests and suspected host-engine gaps, not extra passing results.

## What must agree

Peers need the same Wasm bytes, module names/order and execution environments;
compatible engine builds with the same Wasmtime version and feature settings;
the same memory/table limits and fuel/host-work budgets; and the same sequence
of simulation events and network inputs. A module hash alone does not establish
agreement on all those settings. Keep start scripts and engine configs with
results.

“Same inputs” includes recorded player-stat updates, player joins and commands,
not just a random seed. Two fresh matches can legitimately differ. Replay the
same recorded stream, or compare peers participating in the same live match.
Unsynced UI/rendering instances have separate memory and may differ per client.
The unrestricted `All` environment does not acquire a lockstep guarantee simply
because it can access simulation APIs.

## How the engine keeps execution deterministic

| Risk | Current Core Wasm behavior | Source |
|---|---|---|
| Different NaN payloads or CPU instruction choices | NaN canonicalization enabled; relaxed SIMD disabled by default, with deterministic mode configured | [WasmRuntime.cpp](../../WasmInterface/runtime/WasmRuntime.cpp) |
| Guest threads racing | Wasm threads/shared memory disabled by default | [WasmRuntime.h](../../WasmInterface/runtime/WasmRuntime.h) |
| Memory/table growth depending on the machine | Synced memory and tables must declare equal minimum and maximum; Core start functions are rejected | [WasmCoreValidation.cpp](../../WasmInterface/core/host/WasmCoreValidation.cpp) |
| Native FP settings changing guest calculations | Scoped guest FP controls at callins, helper exports and callbacks; restored on returning to the caller and from host imports | [WasmFloatEnvironment.h](../../WasmInterface/runtime/WasmFloatEnvironment.h), [FP regression](recoil_wasm_sync_report_2026-10-19.md#floating-point-regression) |
| Local clocks/OS randomness driving simulation | No generic WASI/OS access; use agreed seeds or synced engine RNG. Synced timers require explicit diagnostic opt-in | [Sandbox policy](recoil_wasm_sandbox_determinism.md), [timer bindings](../../WasmInterface/core/bindings/WasmCoreDesyncBindings.cpp) |
| Different thread counts moving job results between frames | Copied inputs, fresh worker instances, a next-frame barrier and delivery in global submission order | [Job contract](recoil_wasm_synced_jobs.md#execution-contract) |
| Time-based execution limits | Instruction fuel and host-work counters; no wall-time budget for synced work | [Runtime settings](recoil_wasm_sandbox_determinism.md#execution-budget) |

These are engine responsibilities. A game still needs deterministic simulation
logic and must use its synced entry points. The sandbox cannot make a local
clock, client-specific input or an unstable iteration order into a valid synced
input. Host APIs must preserve their synced behavior through the binding,
including errors, list order, string contents and callback results.

### Floating-point boundaries

The FP boundary fix selects nearest-even rounding and keeps subnormal values
instead of inheriting arbitrary native rounding or flush-to-zero controls.
It restores the caller's controls after guest execution. Host import scopes also
restore the guest's controls before Wasm resumes, including nested callbacks.
Workers use the same guarded helper-export path. This changes Wasm execution
boundaries; it does not change the engine's global FP policy, networking or API.

The regression changes native FP controls deliberately, then executes real
Wasmtime scalar/SIMD arithmetic and conversions. It checks output bits and
caller-control restoration after success and traps. A direct unprotected
Wasmtime call must still expose the bad rounding/underflow behavior, proving
that the inputs actually exercise the issue. See the [report](recoil_wasm_sync_report_2026-10-19.md).

### Jobs and failures

A worker gets copied bytes and a fresh instance, not the submitting module's
live memory/globals. It cannot call engine imports, clocks, RNG or submit more
jobs. Frame F jobs are sealed after that frame and delivered before ordinary
frame F+1 updates, in submission order, even when later jobs finish first.
Waiting may slow real time; it must not change the delivery frame.

An isolated worker trap is an explicit job error. An engine/runtime allocation
failure must not become a machine-dependent successful result. Ordinary Core
module faults currently deregister the module; sync acceptance must reject
faulted or missing required modules rather than treating their absence as a
successful match. See [fault handling](../../WasmInterface/system/WasmInterfaceSystemCore.cpp)
and the [job error contract](recoil_wasm_synced_jobs.md#execution-contract).

## What a matching hash establishes

[WasmSyncVerification.cpp](../../WasmInterface/system/WasmSyncVerification.cpp)
records each synced module in name order. The
[comparison tool](../../../test/synctest/wasm_sync_replay.py) checks its entire
record and the engine checksums, frame by frame.

| Field | Compared data |
|---|---|
| `identity`, `name`, `environment` | Module SHA-512 identity and placement |
| `bytes`, `hash` | Size and XXH3-64 hash of **all linear-memory bytes**, including unused heap/stack bytes |
| `state_bytes`, `state_hash` (schema 2) | Canonical bytes/hash of all globals and table entries, including hidden ones |
| `jobs_hash` | Synced queue digest, including admitted inputs and job metadata |
| `faulted` | Module fault flag |
| `engine`, `checksum` | Engine checksum before and after this frame's Wasm contribution |
| `sim` | The same running checksum over the engine's own synced state only, without any frame's Wasm contribution |
| `workers` | Actual pool size, checked against the requested size; it is allowed to differ between runs |

The engine checksum is cumulative: `engine` can include earlier frames' Wasm
contributions. These hashes are diagnostics, not a cryptographic proof of
state equality.

The Wasm contribution holds each module's identity, memory size and job digest,
so `engine` and `checksum` differ from the first frame that mixes it in whenever
two runs use different builds of a module, or one runs Wasm jobs and the other
their fallback, even when the simulation is the same. `sim` leaves that
contribution out (`CSyncChecker::SyncMeta`): compare it with
`wasm_sync_replay.py compare --engine-state A.jsonl B.jsonl` to ask whether two
such runs simulate the same game. The network checksum is unchanged.

Schema 2 adds globals and tables. The loader validates the original module,
then adds private export aliases without changing guest instructions, indices
or public exports. All synced instances use this transformation, whether or
not logging is enabled. Identity remains the hash of the original module.
Numeric values keep exact bits; function references serialize as original
module indices, not machine addresses or Wasmtime store IDs. Non-null opaque
host references cause an explicit inspection error. See
[WasmCoreState.cpp](../../WasmInterface/core/host/WasmCoreState.cpp).

This adds module-load metadata and original-module validation, not per-call
SDK work. State scans run only for the existing opt-in full-state diagnostics
or game-requested hash interval. The public native/Wasm SDK API is unchanged.
The scanner does not expose passive data/element segment drop status; the new
bulk-operation workload checks later behavior, not immediate drop-flag equality.
Old schema-1 evidence covers memory only and cannot be compared as schema 2.

### Normal play and diagnostic checking

| Mode | Cost and detection |
|---|---|
| Normal play: `system.WasmStateHashInterval = 0` | Existing engine sync checks plus module metadata and cached job digests; **no full guest-state scan** |
| Game opts into `system.WasmStateHashInterval = N > 0` | Adds full memory plus globals/tables to the network checksum every N frames, including frame zero; differences between scans can escape detection |
| Local `WasmSyncLog = /path/sync.jsonl` | Hashes full linear memory plus globals/tables every frame for offline comparison, without changing what is mixed into the network checksum |

A client may enable logging while another does not. Logging must not cause a
desync itself. Full-memory scans can be expensive for large modules; the
[measured cost and policy](recoil_wasm_state_hash_cost_2026-10-12.md) explain why
they are not enabled by default. Matching ordinary network checksums alone is
weaker evidence than the full-memory diagnostic runs below.

## Reproduce the engine-owned tests

Run from the engine repository root. Use a fresh output directory for each
command. The fixture uses public engine parity content and the official SDK;
no downstream game, private assets, account or public service is needed.
The engine must be built with Wasmtime and `SYNCCHECK`; Cargo and its
`wasm32-unknown-unknown` target are needed to build the guest.

Build the Linux engine and C++ regression executable using the normal
[build instructions](../../../AGENTS.md#building-the-engine):

```sh
docker-build-v2/build.sh -j 2 linux -DBUILD_TESTING=ON
docker-build-v2/build.sh --compile -j 2 linux -t test_WasmSyncedJobs
WASM_JOB_FIXTURE="$PWD/test/wasm_api/data/synced_jobs/worker.wat" \
  build-amd64-linux/test/test_WasmSyncedJobs
```

Record one match, then replay its exact input stream with 1, 4 and 8 workers:

```sh
python3 test/synctest/wasm_adversarial.py --frames 900 \
  --output /tmp/wasm-sync-replay
```

Compare two real networked Linux processes (host: 1 worker; guest: 4):

```sh
python3 test/synctest/wasm_adversarial.py --live --frames 900 \
  --output /tmp/wasm-sync-live
```

After building a matching Windows engine, run its executable under a working
64-bit Wine installation with a **dedicated test prefix**:

```sh
python3 test/synctest/wasm_adversarial.py --live --frames 900 \
  --guest-spring /path/to/windows-engine/spring-headless.exe \
  --wine /path/to/wine64 --wine-prefix /path/to/test-prefix \
  --output /tmp/wasm-sync-linux-wine
```

The runner pins copies of the engine, Windows DLLs and compiled guest before
starting. It saves source/config provenance, start scripts, demos, logs and
comparisons. `--frames 900` includes frames 0 through 900: **901 rows**. The
runner rejects missing/noncontiguous frames, absent synced modules, faulted
records, failed fixture assertions and incorrect actual worker counts. The
runner checks the exact expected name/environment/identity set on every frame;
losing the same module on both peers also fails.

Check that a bad run is detected, using identical guest bytes in both runs:

```sh
python3 test/synctest/wasm_adversarial.py --negative-control --frames 15 \
  --workers 1 4 --output /tmp/wasm-sync-clock-control
```

This explicitly enables a local-clock read at frame 7. Success means the tool
finds the first difference in module `rules-0` at **frame 7**, not that the runs
match. The runner removes `SPRING_ENABLE_SYNCED_TIMERS` from ordinary runs.
Never enable that diagnostic clock in normal lockstep testing or gameplay.

For another module/game, use the existing
[replay runner](../../../test/synctest/README.md) and preserve its exact input
stream. `--fresh-hosts` is diagnostic: fresh matches can have different player
CPU/ping packets even when their seed and scripted unit commands agree.

## Investigating a difference

1. Keep both engines, guest bytes, configs, demos and unmodified logs. Record
   artifact hashes and uncommitted source changes; a Git HEAD alone is not
   enough for a dirty build.
2. Run `wasm_sync_replay.py compare A.jsonl B.jsonl`. It identifies the first
   differing frame/module or engine checksum. Do not compare only final state
   or tolerate a percentage of different bytes.
3. Rerun that same input stream with `WasmSyncDumpFrame` set to the first
   differing frame and `WasmSyncLog` set. Memory files are named
   `sync.jsonl.frame-F.module-I.bin`, with I in sorted module-name order.
4. Use `wasm_sync_replay.py memory-diff A.bin B.bin` to list pages and offsets.
   Use `decode-state SNAPSHOT.state.bin` for indexed globals/table values.
   Trace them to the guest data or host result that changed. Recheck network
   inputs before assuming a runtime error.
5. Preserve the failing regression and add a passing fix plus a deliberate
   failure check. Do not discard bytes, omit a faulted module, hide differences
   behind rounded subsystem hashes or enable old-demo warning exceptions for
   final acceptance.

## Gates for broader integration

The current evidence supports broader integration and testing of synced Wasm
on the tested x86-64 paths. Maintain that evidence when the runtime, generated
bindings, shared host adapters, FP handling, job ordering or synced APIs change:

- Run the C++ arithmetic/job suite, the official-SDK live/replay cases and the
  deliberate failure controls against the exact candidate artifacts.
- Add a regression for each discovered sync failure and keep the per-frame logs.
- Test native Windows hardware and Linux ARM64 for the same fixture; rerun the
  arithmetic regression on ARM hardware. Wine cannot substitute for either.
- Run the [seeded multi-module suite](../../../test/synctest/README.md#repeatable-deeper-matrix)
  for Rules/Gaia, nested callbacks, cross-world jobs, hidden state and bulk
  operations. Keep extending API/game scenarios; current fixtures are finite.
- Use the [saved corpus](../../../test/synctest/README.md#reuse-the-exact-workload-on-another-platform)
  to compare an additional platform against the same recorded inputs and
  prebuilt modules, without requiring a Rust toolchain there.
- Add routine cross-platform regression execution using existing engine build
  artifacts. The local suite has not been added to or run through GitHub
  Actions. Broad game replay matrices and long-running gates remain future work.

The [sandbox policy](recoil_wasm_sandbox_determinism.md),
[API parity tests](recoil_wasm_parity_testing.md) and
[job contract](recoil_wasm_synced_jobs.md) complement these checks. They answer
different questions: allowed access, equivalent API behavior and deterministic
parallel work all matter to a reliable synced backend.
