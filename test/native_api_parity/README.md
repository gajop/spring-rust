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

When the rendering run uses an ASAN-built engine, the harness automatically
adds `lsan.supp` to `LSAN_OPTIONS`. LeakSanitizer remains enabled; the file
contains only the known process-lifetime driver/library entries. Its
FreeType/SDL entries are explicitly TODOs for a separate engine teardown fix,
outside the Wasm work.

Use installed binaries for renderer runs. Running `build-linux/spring` directly
without the installed data tree can fail on assets such as `fonts/FreeSansBold.otf`;
`build-linux/install/spring` is the expected portable install layout.
