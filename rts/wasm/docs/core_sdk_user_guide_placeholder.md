# Core SDK user guide

## Install

- Rust toolchain: stable
- Wasm target: `wasm32-unknown-unknown`
- Dependency: `spring` with feature `alloc`
- Generated SDK: `rts/wasm/generated/sdk/`
- Environment projection: `spring::rules_synced`

## Quickstart

- `use spring::rules_synced as api;`
- `api::units_info::get_unit_health(unit_id)`
- Return type: `Result<T>`
- Error field: `error.code`
- Guest export: `spring::export_environment_mask!(api::ENVIRONMENT_MASK);`

## Environment model

- One module per environment
- `rules_synced`: deterministic simulation
- `rules_unsynced`: local unsynced state
- `gaia_synced`: deterministic Gaia simulation
- `gaia_unsynced`: local Gaia state
- `ui`: UI visibility context
- Load marker: `SPRING_ENV_MASK`

## Sync rules

- Synced calls: deterministic inputs and outputs
- No wall clock in synced code
- No random device in synced code
- No filesystem authority in synced code
- No process control in synced code
- Replay check: `python3 test/wasm_api/tools/check_sync_replay.py`

## Debugging

- Build: `cargo build --target wasm32-unknown-unknown --release`
- Generate: `spring-api-codegen --strict`
- Verify: `python3 rts/wasm/verify_codegen.py`
- Runtime logs: `spring.log`
- Core errors: `ApiError.code`
- Missing wrapper: generation failure
- Fuel and epoch budgets: opt-in diagnostics
- Gameplay default: throughput-first
