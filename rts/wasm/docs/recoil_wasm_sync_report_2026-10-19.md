# Wasm sync verification — 2026-10-19

**Result:** the engine-owned official-SDK fixture matches across a Linux host
and the Windows executable under Wine for **901 consecutive frames**, with
full linear-memory and job-state hashes plus engine checksums compared on every
frame. Recorded playback also matches with **1, 4 and 8 workers**. Deliberately
bad native FP settings reproduced a Wasm arithmetic bug; the local boundary fix
passes the regression, while an unprotected call still demonstrates the fault.

This supports broader integration of the tested synced Wasm paths. It does not
certify every host API, native Windows hardware, ARM or arbitrary guest programs.
The [sync contract and reproduction guide](recoil_wasm_sync_verification.md)
state the requirements and remaining gates.

## Source and artifacts

These results were collected from a local working tree based on
`6cedf0cde7f9e6f542bfb80ad7c6935fe5ff77b9`, with the Wasm FP boundary changes
and adversarial tests applied. They are not a release or a claim that the base
commit alone contains the fix. After verification, the implementation and tooling
were committed locally as `5daee98e45`. The archived manifests retain the original
pre-commit source/artifact provenance. No push, release or CI dispatch was made.

The fixture is built against `spring-sdk`, using the engine's public
`test/native_api_parity/fixtures/game.sdd` content and a generated map. Its
synced memory is fixed at 16 MiB. The runtime is Wasmtime 42.0.1. The same
compiled Wasm bytes run on both platforms. No downstream game assets or binary
packages are part of this evidence.

| Artifact | SHA-256 |
|---|---|
| Linux `spring-headless` | `67eff45fb4fe281a57a150025028be2f8b5b928f41586670d19ae4a5b35e8814` |
| Windows `spring-headless.exe` | `a24430d901dd5970e82b25dd6fe35216a867198acd8e71b9061dde2a1f1d6e88` |
| Final official-SDK guest | `09616e5e48abe85d5653196515ff7e10e9268d2ebb5951c3a6c899d07f67be25` |

The [evidence manifest](data/sync_adversarial_2026-10-19/manifest.json) records
DLL hashes, guest/source hashes, the original tracked-patch hash, actual worker
counts, module identities and comparisons for each run. It also indexes the
compressed per-frame logs and their checksums. Engine binaries are not bundled
in the docs. Rebuilding may produce different binary hashes; retain the exact
candidate artifacts when collecting a new result.

## Measured checks

Frame ranges are inclusive. “901 frames” means frames 0 through 900.

| Check | Compared configurations | Result |
|---|---|---|
| Live Linux / Windows-under-Wine match, final guest | Linux host: 1 worker; Windows guest: 4; real engine networking over loopback | All 901 rows match |
| Recorded playback, final guest | Linux recording versus replay with 1, 4 and 8 workers | All 241 rows match in each comparison |
| Longer recorded playback, earlier guest build | Linux recording versus replay with 1, 4 and 8 workers | All 901 rows match in each comparison |
| Clock negative control, final guest | Same code; local clock deliberately read at frame 7 | First difference is frame 7, module `parity` |
| Initial FP regression, before fix | Four host rounding modes × subnormal flushing off/on | 5 of 42 assertions fail |
| Same initial FP regression, after fix | Same 42-assertion test | All pass |
| Expanded C++ Wasm job suite | FP/SIMD/conversions, trap recovery, direct-call negative control, worker state/order/limits | 5,321 assertions pass in 11 cases |

The earlier long-replay guest has SHA-256
`4778fce13de6f68defc55e86c3279111a14b7104da74eaf54123477668e93347`.
A subsequent test-only Clippy annotation changed the guest's source locations
and binary. The final guest was checked separately in the 901-frame live test,
241-frame replay matrix and frame-7 negative control; the two guest hashes must
not be conflated.

The final runner pins the engine, DLLs and compiled guest before starting and
verifies actual pool sizes in the frame logs. Worker-count changes are observed
configuration changes, not just different requested settings.

## What the fixture exercises

The [guest](../../../test/wasm_api/guests/sync_adversarial/src/lib.rs) checks
unit IDs, positions and health; a stable typed missing-key error; complete
player information; repeated heap allocation/reuse; and scalar floating-point
edge cases, including NaN payloads, signed zero and subnormals.

It submits eight jobs with deliberately unequal work durations at regular
frames. Each result must arrive in submission order at exactly the next frame
and equal the same calculation done inline. Mutating the submitted input after
submission must not affect the result. Guest assertions fault the module if
these checks fail; a missing/faulted fixture cannot count as successful sync.

The C++ suite separately exercises real Wasmtime scalar/SIMD arithmetic,
f64-to-f32 demotion, integer-to-float conversion and caller FP-control restoration
through successful calls and traps. Its other job tests cover fresh private
memory/globals, fuel and memory limits, forbidden worker imports, copied inputs,
cancellation, owner lifetime, ordered delivery and thread-pool reuse.

## Floating-point regression

Before the fix, identical Wasm inputs inherited the native thread's rounding
and subnormal controls. Concrete examples:

| Calculation | Required result | Observed with bad native controls |
|---|---|---|
| f32 `1 + 2^-24` | Nearest-even `1.0`, bits `0x3f800000` | Upward rounding produced `0x3f800001` |
| f32 smallest normal × `0.5` | Subnormal bits `0x00400000` | Flush-to-zero produced zero |
| Corresponding f64 tie/underflow cases | Nearest-even and gradual underflow | Also differed |

This is a reproduced runtime boundary weakness under deliberately altered native
FP controls. It is not evidence that an ordinary gameplay desync was caused by
those controls. The initial [before log](data/sync_adversarial_2026-10-19/fp-before.log.gz)
and [same-test after log](data/sync_adversarial_2026-10-19/fp-initial-after.log)
preserve that distinction.

`WasmFloatEnvironment` now establishes guest rounding/subnormal controls at the
actual Core callin/export/callback boundaries and restores caller controls on
exit. The common import guard restores guest controls after native host work.
No networking, simulation algorithm, public API or generated binding output
changed. The ordinary engine FP settings remain the caller's settings.

The expanded test also calls Wasmtime directly, deliberately bypassing Spring's
entry guard. That call still produces the upward-rounded/flushed values. This
negative control establishes that the regression inputs really expose the
problem; constant expected outputs or an unexercised FP mode cannot pass it.

An additional test-harness issue appeared when a later test deliberately
trapped: Catch's per-test signal handlers replaced Wasmtime's JIT trap handlers
and reported `SIGILL` as a native crash. Only the Wasm job target now uses Catch
without signal/SEH interception. Wasmtime handles its own guest traps; a genuine
native crash still terminates the process and fails the test. The
[final C++ log](data/sync_adversarial_2026-10-19/cpp-jobs-final.log) records the
expanded suite after this correction.

## Distinguishing different inputs from a desync

The fixture retains the **entire** typed player-info result, including ping
and CPU usage. Separate fresh matches differed when CPU-stat packets were
applied on different frames and carried different values. These were different
network input streams, even though the module, random seed and scripted unit
commands were the same.

The [saved player-stat transitions](data/sync_adversarial_2026-10-19/fresh-input-stat-updates.json)
show the different inputs. For example, one diagnostic match first reported
a nonzero CPU usage at frame 45 (`0.022637796`); another did so at frame 39 (`0.040314652`). In the live
same-match test, peers observed the same updates on the same simulation frames.
Playback of a recording preserved those inputs and matched across worker counts.
No player fields or guest-memory bytes were excluded to make the comparisons
pass. The diagnostic fresh-match differences are not counted as successful
equal-input checks or reported as a proven runtime desync.

## Coverage limits of the initial batch

- Windows used Wine on the Linux machine. This exercises the Windows executable,
  Wasmtime DLL and binding ABI, but does not replace a native Windows hardware
  run. No native-PC acceptance result is claimed for this batch.
- The AArch64 FP-control implementation has no hardware result here. The same
  guest and FP regression must run on ARM before claiming that platform gate.
- The initial schema-1 logs cover linear memory, module identity, queued jobs
  and engine checksums. They do not inspect globals/tables. The later schema-2
  work below is a separate candidate and evidence batch; it does not upgrade
  these archived logs retroactively.
- The initial fixture loads one RulesSynced module. It is not a complete GaiaSynced,
  multi-module, host-API, callback, all-proposals or long-game certification.
- Hash agreement over these finite scenarios does not prove arbitrary programs
  correct. Keep the original failing cases and rerun the integration gates when
  the runtime or shared bindings change.
- No new scheduled/CI matrix was installed or dispatched. Native Windows,
  ARM, broader APIs and long scenario coverage remain explicit follow-up work.

## Inspect the saved logs without running an engine

The manifest and compressed frame logs are engine-only evidence, stored beside
this report. For example:

```sh
gzip -dc rts/wasm/docs/data/sync_adversarial_2026-10-19/live-linux-wine-live-host.jsonl.gz > /tmp/wasm-host.jsonl
gzip -dc rts/wasm/docs/data/sync_adversarial_2026-10-19/live-linux-wine-live-guest.jsonl.gz > /tmp/wasm-guest.jsonl
python3 test/synctest/wasm_sync_replay.py compare /tmp/wasm-host.jsonl /tmp/wasm-guest.jsonl
```

This should report 901 matching frames. For new runs, follow the
[reproduction guide](recoil_wasm_sync_verification.md#reproduce-the-engine-owned-tests)
and use new output directories; do not overwrite the original evidence.


## Follow-up: hidden state and repeatable deeper testing

The follow-up candidate adds globals/table inspection and a seeded,
resumable test runner. This is separate from the original artifact hashes and
schema-1 logs above. The loader validates original bytes before adding private
export aliases; module identity remains the original SHA-512. References use
module function indices so hashes remain portable across processes/platforms.
No guest instructions or generated SDK files are edited. Native SDK API,
network code and simulation algorithms are unchanged by this follow-up.

The C++ suite now passes **5,387 assertions in 13 cases**. The added regression
changes hidden globals and table entries while checking that every linear
memory byte remains unchanged. It repeats in independent Wasmtime stores,
checks export-name collision handling, and rejects opaque host references
instead of serializing their addresses. The first four-module smoke run also
matches its recording against 1/4/8-worker replays for all 91 frames.

The official SDK workload now supports Rules and Gaia builds, multiple modules,
seeded job inputs, jobs submitted during result delivery, cross-module and
cross-world worker targets, terrain callbacks that reenter the engine, synced
RNG, team resources and unit states. A separate WAT module exercises private
state, indirect calls, passive segment initialization/drop, memory copy/fill
and failed growth under fixed limits.

[Automation commands](../../../test/synctest/README.md#repeatable-deeper-matrix)
cover serial matrices, source/artifact pinning, structured failures and resume.
The runner requires schema 2 and the exact expected name/environment/identity
set on every frame; missing the same module on both peers cannot pass. Python
regressions cover this, hidden-state-only differences, malformed/truncated
streams, resumed passes, corrupted evidence, preserved failed attempts and
changed-input refusal and portable path ordering (**21 tests pass**).

A saved corpus carries exact prebuilt Wasm, a recording and reference hashes
for another platform. Replaying it needs no Rust/WAT compiler; only map-mount
paths are relocated, with network packet bytes preserved and hashed. See the
[platform procedure](../../../test/synctest/README.md#reuse-the-exact-workload-on-another-platform).

The passive data/element segment **drop flags** are not exposed by Wasmtime's C
API. Bulk-operation behavior is exercised, but immediate drop-flag equality is
not claimed. Native Windows and ARM hardware acceptance are still outstanding;
the known Windows machine was unreachable over SSH in this follow-up. No CI
job was dispatched and nothing was pushed or released. The source was committed
locally after this measurement run, as noted above.


### Follow-up measured results

The [schema-2 evidence manifest](data/sync_deep_2026-10-19/manifest.json)
contains source/artifact hashes, raw per-frame streams, negative-control
comparisons and the additional automation checks. This candidate uses Linux
engine SHA-256 `a992d617d348ba45387b711c9a113b7a72526e0e8ec95a0525f1834f04469c49`
and Windows engine `b7b512a61492ba53717a98b26ac14d0b5a705a3d229b9fd0d838ac54a7476a77`.
The SDK Rules guest is `a1a88a01c20b02cd37e99df5bf97a7b09cfc214ef1454ac44b539624d3fc34bd`;
Gaia/module identities and DLL hashes are recorded per case.

| Check | Result |
|---|---|
| Seeds 1 and 16,777,215; two Rules modules plus low-level state workload; recording vs 1/4/8-worker replays | All 901 frames match in all six comparisons |
| Same seeds; two Rules + two Gaia + state workload; recording vs 1/4/8-worker replays | All 901 frames match in all six comparisons |
| Same five-module workload; live Linux host vs Windows executable under Wine; both seeds | All 901 frames match in both comparisons |
| Hidden-state-only negative control | First difference frame 7, `hidden-state`; memory hash identical, globals/table hash different |
| SDK clock-memory negative control | First difference frame 7, `rules-0` |
| Short automation matrix, seed 424,242 | Six 91-frame replay comparisons pass; both negative controls pass |
| Full-state network hashing interval 0 and interval 1; diagnostic logging disabled on extra playback | Recorded network checksums still match; explicitly a policy check, not a full-state comparison of unlogged memory |
| Standalone local test kit; no Git checkout or Rust/WAT compilation; Windows executable under Wine | Saved Linux reference matches 1/4/8-worker playback for all 901 frames; unlogged replay reaches frame 900 with no recorded desync warning |
| Real resume of deep and short matrices | All eight/four verified cases reused without launching another engine; regression tests separately cover corrupted evidence and retrying failed attempts |

These are 17 matching 901-frame comparisons, six additional 91-frame
comparisons, and separate deliberate failure/policy checks. Frames are never
sampled or tolerated by percentage. The later runner refinements add unlogged
policy checking, stronger input validation and standalone-kit support; the deep
matrix's already tested engine and guest binaries did not change.

The kit's [file manifest](data/sync_deep_2026-10-19/portable-kit-manifest.json)
and command in `README.txt` make the next native Windows/ARM run repeatable.
The kit contains public parity/base content, not downstream game assets, and
has not been transferred or published. This batch still does **not** establish
native Windows or ARM hardware acceptance, exhaustive host-API coverage, or
immediate passive-segment drop-flag inspection. Headless parity content also
retains its known font/dummy-weapon warnings in the raw console logs; this sync
result is not a claim that every unrelated engine/game check was run.
