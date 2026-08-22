# Sync verification

- rung 1: `test/wasm_api/check_sync_replay.py`
- input: replay command writing `{run_dir}/sync_checksums.jsonl`
- row format: `{"frame": integer, "checksum": string}`
- gate: same binary, same replay, three runs, exact per-frame equality
- empty or missing streams: failure
- cross-platform replay comparison: rung 3
- current status: runner wired; no checked-in replay stream yet

- rung 2: `generated_synced_callout_audit.md`
- source: generated environment and transport metadata
- status: heuristic inventory; human review required
- doubtful rows: `review-required`
- deterministic claims: not made by the generator
