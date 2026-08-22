# Sync verification

- rung 1: `test/wasm_api/tools/check_sync_replay.py`
- input: replay command writing `{run_dir}/sync_checksums.jsonl`
- row format: `{"frame": integer, "checksum": string}`
- gate: same binary, same replay, three runs, exact per-frame equality
- empty or missing streams: failure
- cross-platform replay comparison: `.github/workflows/wasm-cross-platform.yml`
- rung 3: Linux, arm64 Linux, and Windows Core guests, same synced fixture,
  frame-keyed observation-derived checksums compared in the cross-platform
  check
- current status: the checked-in `test/native_api_parity/fixtures/game.sdd`
  plus its deterministic blank-map seed is the replay fixture. A local
  headless Core-vs-reference run completed on 2026-08-22 with `--cases 3`,
  producing matching guest/reference observation streams and zero reported
  probe failures. The three-platform workflow still requires a successful
  engine-build artifact for the tested SHA; it has not been falsely marked
  green locally.
- the rung-3 checksum is not the engine's native replay checksum; it is a
  canonical SHA-256 over the exported frame/team-unit-count/floating-point/
  RNG observation tuple

- rung 2: `generated_synced_callout_audit.md`
- source: generated environment and transport metadata
- status: heuristic inventory; human review required
- doubtful rows: `review-required`
- deterministic claims: not made by the generator
