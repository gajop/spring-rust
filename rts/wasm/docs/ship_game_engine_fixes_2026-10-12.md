# Ship Game engine fixes, 2026-10-12

Implemented and verified locally on `rust-wip`, following
[requests_from_ship_game.md](requests_from_ship_game.md). Ship Game working
files were not edited. Only test modules/manifests were overlaid into immutable
snapshot copies. No push or GitHub Actions run.

## Feature exhaustion

`CFeatureHandler::LoadFeature` now tests `featureMemPool.can_alloc()` before
placement construction. `CanAddFeature(-1)` requires a free ID. Explicit IDs
retain the existing availability/recycling checks. A full pool returns null,
and Native/Core creation returns no feature ID (`-1`), with no allocation or
creation event. Games should test the returned ID and degrade or retry later.

The capacity test belongs before allocation: testing it inside `AddFeature`
would incorrectly reject the last valid object after its memory page was
allocated. Existing delayed ID reuse is preserved. The test frees one object
and waits until frame 33, after the 32-frame recycling tick, to allocate again.

The cap remains 32,000. `MAX_UNITS + MAX_FEATURES` must fit the combined 16-bit
ID representation; 32,000 units plus 48,000 features would not. Raising the cap
requires a wider-ID migration and audit, including network/save/reference
compatibility. Lowering the unit cap or adding an ineffective modrule is not
part of this fix.

## Recoverable optional timers

The existing raw `spring:desync` imports return a complete 64-bit scalar timer,
with no status field. A packed status cannot be added without losing timer bits
or breaking already-built guests. Their disabled fallback is now deterministic
zero, which makes existing profiling paths produce zero durations and keeps
rules loaded. Import budget and malformed-ABI traps remain in force.

New checked imports let the Rust SDK return real errors while preserving all
64 timer bits:

| Import | Signature | Result |
|---|---|---|
| `spring:desync/try-get-timer` | `(output_ptr:i32) -> i32` | Core status; writes an eight-byte little-endian timer on success |
| `spring:desync/try-get-timer-micros` | `(output_ptr:i32) -> i32` | Same, in microseconds |

Both return `NotAvailable=4` when synced timing is disabled, without reading a
clock or writing output. When enabled, invalid output ranges return
`OutOfBounds=2`. Native service errors also return status. The SDK's existing
`spring::desync::{get_timer,get_timer_micros}` signatures stay `Result<u64>`;
newly rebuilt guests use these imports and require an engine containing them.
Already-built scalar-import guests continue to load. Optional profiling can
use `.unwrap_or(0)` or handle `NotAvailable` explicitly.

Real synced clock reads still require the process-level diagnostic opt-in
`SPRING_ENABLE_SYNCED_TIMERS=1`; enabling it intentionally permits desyncs.
Unsynced worlds retain their clock access. Worker instances still have no host
imports, including timers. `diff-timers` remains a pure calculation over inputs.

## Atlas reporting

GroundFX finalization now has the same headless logging guard as projectile
textures. Headless runs do not have a usable graphics atlas allocation; a
rendered allocation failure still logs an error. Neither texture-size limits
nor rendered atlas behavior changed. The prior rendered acceptance log already
had no groundFX failure; the new rendered check confirms it remains successful.

## Local verification

* Wasm import validation: 127 assertions in 16 cases, including both checked
  timer signatures in all seven environments and rejection of a wrong ABI.
* SDK Wasm clippy with warnings denied: pass.
* The Rust fixture checks SDK `NotAvailable`, legacy zero fallbacks, and pure
  zero timer differences every frame. Its optional feature stress fills the
  pool, makes repeated rejected creation calls, destroys an object, checks
  reuse after ID recycling, and confirms rules are still active at frame 36.
  Fresh recording and 1/4-thread playback match all 37 frame logs, with zero
  recorded desync warnings.
* A single-process enabled WAT probe checks positive full-width timer output,
  microsecond units, and invalid output ranges. It passes. This diagnostic
  probe is deliberately not presented as a deterministic replay test.
* Headless stress logs have no groundFX error. An Xvfb rendered fresh recording
  and 1/4-thread replay match all seven frame logs and have no groundFX error.

The engine targets were rebuilt incrementally in the local build container.
Large binaries, immutable overlays, demos and logs remain under `out/local-sync`.
Small acceptance results/provenance are in
[the evidence directory](data/ship_game_engine_fixes_2026-10-12/).

Build the fixture from its directory so the fixed-memory linker setting is
applied:

```sh
cd test/wasm_api/guests/engine_notes_guest
cargo build --release --locked --target wasm32-unknown-unknown --features feature-pool-stress
```

From the engine checkout, pass the resulting
`target/wasm32-unknown-unknown/release/recoil_engine_notes_guest.wasm` to the
replay runner's `--fixture`, with `--record-first --threads 1 4 --frames 36
--instruction-fuel 1000000000`. Use a pinned Ship Game snapshot and valid
version-5 source demo; the runner creates fresh recordings for comparison.
The enabled probe is
[timers_enabled.wat](../../../test/wasm_api/data/synced_jobs/timers_enabled.wat).
