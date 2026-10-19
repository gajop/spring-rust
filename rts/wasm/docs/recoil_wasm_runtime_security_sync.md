# Recoil Wasm Runtime / Security / Sync

This document was split because the combined version became too large.

Use:

- [Synced Wasm Contract & Verification](recoil_wasm_sync_verification.md) — required lockstep behavior, current protections, reproducible tests and remaining gates.
- [Sync Evidence, 2026-10-19](recoil_wasm_sync_report_2026-10-19.md) — measured cross-platform/worker results and the FP regression, with engine-only raw evidence.

- [Runtime & ABI](recoil_wasm_runtime_abi.md) — Wasmtime, WIT/Component Model, callouts/callins, canonical ABI, callbacks, handles, performance.
- [Sandbox & Determinism](recoil_wasm_sandbox_determinism.md) — sandbox boundary, limits, synced determinism, FP/RNG, faults, AOT safety.
- [Execution Environments](recoil_wasm_execution_environments.md) — Rules/Gaia/UI roles and synced/unsynced instance separation.
- [Parity & Testing](recoil_wasm_parity_testing.md) — signature parity, runtime parity, LOS fixture, synced tests.
