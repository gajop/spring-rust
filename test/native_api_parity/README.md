# Native API Parity

Engine-owned fixture for comparing Lua `Spring.*` calls with the Rust native API.

## Layout

- `collect_spring_usage.py`: scan a game directory for `Spring.*` usage.
- `api_tests.json`: tiny manifest for category files in `api_tests/`.
- `api_tests/*.json`: data-driven parity test specs by API area.
- `fixtures/game.sdd`: tiny test game with one unit, one feature, and a LuaRules gadget.
- `native`: Rust native module loaded by the engine during the parity run.
- `run_harness.py`: builds the native module, writes `script.txt`, starts Spring, and compares results.

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

Tests that depend on rendering must set `requires_rendering: true` in
the relevant `api_tests/*.json` file and be run with the non-headless binary:

```bash
python3 test/native_api_parity/run_harness.py --spring build-linux/install/spring --spring-headless build-linux/install/spring-headless --enable-rendering-tests --mode both
```

Rendering-required tests are skipped, and therefore not counted as covered,
unless `--enable-rendering-tests` is set.

Use installed binaries for renderer runs. Running `build-linux/spring` directly
without the installed data tree can fail on assets such as `fonts/FreeSansBold.otf`;
`build-linux/install/spring` is the expected portable install layout.
