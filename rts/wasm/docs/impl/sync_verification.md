# Sync verification

- rung 1: `test/wasm_api/check_sync_replay.py`
- input: replay command writing `{run_dir}/sync_checksums.jsonl`
- row format: `{"frame": integer, "checksum": string}`
- gate: same binary, same replay, three runs, exact per-frame equality
- empty or missing streams: failure
- cross-platform replay comparison: `.github/workflows/wasm-cross-platform.yml`
- rung 3: Linux, arm64 Linux, and Windows Core guests, same synced fixture,
  frame-keyed observation-derived checksums compared in the cross-platform
  check
- current status: workflow wired; execution requires the external replay
  asset and all three runners
- the rung-3 checksum is not the engine's native replay checksum; it is a
  canonical SHA-256 over the exported frame/team-unit-count/floating-point/
  RNG observation tuple

- rung 2: `generated_synced_callout_audit.md`
- source: generated environment and transport metadata
- status: heuristic inventory; human review required
- doubtful rows: `review-required`
- deterministic claims: not made by the generator
