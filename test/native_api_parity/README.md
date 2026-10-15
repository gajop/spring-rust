# Native API Parity

Engine-owned fixture for comparing Lua `Spring.*` calls with the Rust native API.

## Layout

- `collect_spring_usage.py`: scan a game directory for `Spring.*` usage.
- `api_tests.json`: tiny manifest for category files in `api_tests/`.
- `api_tests/*.json`: data-driven parity test specs by API area.
- `fixtures/game.sdd`: tiny test game with one unit, one feature, and a LuaRules gadget.
- `native`: Rust native module loaded by the engine during the parity run.
- `run_harness.py`: builds the native or Wasm fixture, writes `script.txt`,
  starts Spring, and compares results.

## Usage

```bash
python3 test/native_api_parity/collect_spring_usage.py /home/gajop/projects/spring-projects/SBC.sdd
python3 test/native_api_parity/run_harness.py --mode both
```

SBC is only a usage corpus for planning coverage. The runnable test fixture lives here.

The harness writes one result stream per Spring Lua context:

- `synced_gadget.jsonl`
- `unsynced_gadget.jsonl`
- `widget.jsonl`
- `native.jsonl`

Use `--mode lua` to refresh Lua baselines, then `--mode native` while iterating on the native API.

By default the runner uses Spring's built-in blank-map generator:
`InitBlank=1`, a random `MapName`, and `blank_map_x/y` map options.

Use `--mode wasm` to build the raw Core-Wasm guest, package it under
`LuaRules/wasm/manifest.txt`, load it through the game VFS, and run the Lua
reference plus Core observation in one in-engine fixture process. This avoids
booting a second Spring instance solely to collect the baseline. The fixture
records both streams in `wasm.jsonl` and requests `quitforce` itself when its
checks are complete; the outer runner only waits for that process and applies
a bounded timeout. It never injects mouse or keyboard events.

The Core guest uses `test/wasm_api/guests/parity_guest/` and requires the
`wasm32-unknown-unknown` Rust target. The observation includes the synced unit
count plus explicit floating-point edge-case and deterministic RNG signatures.
The cross-platform workflow compares all of these fields, so a
platform-specific Wasm FP or RNG change cannot be hidden by an otherwise
matching gameplay count.

The harness writes `wasm.jsonl` for Wasm observations. Native runs additionally
write `native.jsonl` and the engine callin traces. A headless API-only run uses
`--mode both --skip-callin-compare`; the deterministic 149-callin driver is
enabled by the rendering fixture and should be run with
`--enable-rendering-tests`.

Tests that depend on rendering must set `requires_rendering: true` in
the relevant `api_tests/*.json` file and be run with the non-headless binary:

```bash
python3 test/native_api_parity/run_harness.py --spring build-linux/install/spring --spring-headless build-linux/install/spring-headless --enable-rendering-tests --mode both
```

Rendering-required tests are skipped, and therefore not counted as covered,
unless `--enable-rendering-tests` is set.

`--blank-map-terrain null --blank-map-height 0` exercises the runtime sparse/null-map
object-query path. The default `--blank-map-terrain blank` keeps the ordinary terrain path.

An ASAN-built engine carries its own LeakSanitizer suppressions for known
third-party leaks (`rts/System/Platform/LeakSanitizerSuppressions.h`), so the
harness passes none.

Use installed binaries for renderer runs. Running `build-linux/spring` directly
without the installed data tree can fail on assets such as `fonts/FreeSansBold.otf`;
`build-linux/install/spring` is the expected portable install layout.

## Evidence and stub checks

`compare.fields` are required results. Missing fields fail native and Wasm
comparisons, including when both sides omit the same field. An intentionally
optional field must be listed in `compare.optional_fields` with a nonempty
reason. Optionality does not supply behavioural evidence: absent/default fields
and shape-only comparisons cannot qualify a function as verified.

Native `*_if_present` helpers write `comparisons.jsonl`, keyed by test identity
and field. Each run also saves `comparison.json`; Wasm mutant checks require an
actual differing compared field rather than crediting a trap or absent row.

`aggregate_coverage.py` writes a function evidence ledger and module-level
headline, keeping label coverage in a separate inventory. `--gate` rejects
regressions and unverified additions. `--native-audit-log` includes the targeted
fixture evidence. `--write-ledger` updates the checked-in baseline only after
the gate passes. The baseline currently has no verified functions.

Offline checks need no running engine:

```sh
python3 -m unittest discover -s test/native_api_parity/tests -v
python3 test/native_api_parity/generate_mutants.py --check
python3 test/native_api_parity/sweep_mutants.py --list --limit 10 --output /tmp/mutant-inventory
```

A real mutant sweep requires `-DNATIVE_API_TEST_HOOKS=ON`. The engine accepts
`SPRING_NATIVE_API_MUTANT=module.Function:zero` (or `:noop`) only in that build.
It hands modules private API-table copies, leaving the global tables intact.
`sweep_mutants.py` runs a passing baseline followed by fresh mutant processes,
supports sharding, and records activation plus targeted failures. It is an
intensive integration check; generating the replacements does not execute them.

Probe builds/runs are serialized and restore the canonical non-rendering
generated sources afterward. `--skip-wasm-build` checks a fingerprint of the
probe selection, rendering flag, SDK sources and binary; old artifacts without
provenance require one rebuild. Rendering-enabled probes are generated with the
same flag used to run them.
