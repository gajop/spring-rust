# rust-wip rebase audit, 2026-07-02

This file tracks the rebase of the Rust/native binding work onto the BAR-aligned
`origin/master`. The squashed Rust/native work is the commit containing this
file after the rebase completes; its final SHA is intentionally not recorded
here because it is not the meaningful anchor.

## Fixed refs

- Old base: `af288d81580b203c199f69f8c69268d16703da5d`
- Target upstream: `origin/master` at `d313449ce9fc71b65280285651162f4061fdf171`
- Local source tip, for recovery only: `backup/rust-v2-pre-rebase-20260702` at `22308574242e6953263a3532f87438adaee61b1d`
- Local source branch: `rust-v2`
- New branch: `rust-wip`

## Generated inventories

- Incoming upstream commits: `docs/rebase/incoming-origin-master-commits.txt`
- Incoming upstream touched files: `docs/rebase/incoming-origin-master-files.txt`
- Local Rust/native source commits: `docs/rebase/local-rust-v2-commits.txt`
- Local Rust/native source touched files: `docs/rebase/local-rust-v2-files.txt`
- Preflight merge/conflict output: `docs/rebase/merge-tree-preflight.txt`

## Size of the move

- Incoming upstream range: 276 commits; 640 files changed, 114583 insertions, 45007 deletions.
- Local Rust/native range: 8 commits; 383 files changed, 96880 insertions, 101 deletions.

## Initial conflict set

The preflight merge reported conflicts in these paths:

- `.gitignore`
- `docker-build-v2/build.sh`
- `docker-build/scripts/02_configure.sh`
- `rts/Lua/LuaSyncedRead.cpp`
- `rts/Rendering/Units/UnitDrawer.cpp`
- `rts/Rendering/Units/UnitDrawer.h`
- `rts/Sim/Misc/CustomColorPalette.cpp`
- `rts/Sim/Misc/CustomColorPalette.h`
- `rts/System/LoadSave/DemoReader.cpp`
- `rts/lib/RmlUi`

## Resolution log

- Resolved: `.gitignore` keeps upstream IDE entries and local Rust artifact ignores.
- Resolved: `docker-build-v2/build.sh` takes upstream; local non-TTY Docker fix is superseded by upstream `TTY_FLAG` handling.
- Resolved: `docker-build/scripts/02_configure.sh` stays deleted with upstream Docker v1 removal; local `CMAKE_EXPORT_COMPILE_COMMANDS` addition is not resurrected here.
- Resolved: `rts/Lua/LuaSyncedRead.cpp` takes upstream documentation and implementation around the conflicted hunks.
- Resolved: `rts/Rendering/Units/UnitDrawer.cpp` takes upstream status-array / `DrawBuildSquare` implementation.
- Resolved: `rts/Rendering/Units/UnitDrawer.h` takes upstream layout with one `EngineBuildSquareRendering()` accessor.
- Resolved: `rts/Sim/Misc/CustomColorPalette.cpp` takes upstream lifecycle implementation.
- Resolved: `rts/Sim/Misc/CustomColorPalette.h` takes upstream lifecycle interface.
- Resolved: `rts/System/LoadSave/DemoReader.cpp` takes upstream lower-case extension handling.
- Resolved: `rts/lib/RmlUi` rebased local WidgetScroll ASAN fix onto upstream RmlUi `2230d1a6`, producing submodule commit `e4eb7616778bc2a714507d971b8a93387aafa475`.

## Semantic review TODO

These upstream areas need more than conflict resolution because they may affect
Rust/native interfaces or runtime behavior:

- Checked: `GetClosestEnemyUnit`, `GetUnitMoveDefID`, replay path APIs, and
  custom unit icon APIs already exist in the native API/Rust surface after the
  rebase.
- Fixed: Lua resource-pack upstream changes broke the native compatibility
  `GameRulesInfo` mapping. The old scalar fields now read from the new
  `SResourcePack` fields, matching the compatibility names still exposed by
  `LuaConstGame`.
- Fixed: custom color palette lifecycle is taken from upstream; duplicate local
  `paletteIndex` declarations/assignments were removed.
- Fixed: upstream `CSyncChecker::NewGameFrame()` in `CGame::SimFrame()` was
  restored while keeping native interface initialization.
- Fixed: parity test module rules-param checks now use the public Rust
  `RulesParamValue` enum instead of the raw bindgen union.
- Ported: RmlUi 6.2 submodule move plus local WidgetScroll ASAN fix. Submodule
  commit `e4eb7616778bc2a714507d971b8a93387aafa475` was pushed to
  `public-gajop/rust-wip-rmlui-2230d1a6-widgetscroll` so the superproject
  branch is fetchable.
- Fixed: `Platform.isHeadless` / `Platform.architecture` now have native C API,
  generated Rust wrappers, API docs, and native-vs-Lua parity coverage.
- Fixed: `.cargo/config.toml` now uses a checkout-relative `LIBCLANG_PATH`.
- Fixed: `NativeInterfaceSystem::Reload()` no longer contains a local fallback
  absolute plugin path; it is a no-op unless `SPRING_NATIVE_MODULE` is set.
- Fixed: native module version loading now rejects modules that require a newer
  minor API version than the host.
- Fixed: headless parity harness default now prefers `build-amd64-linux/install`
  before stale `build-linux/install` output.
- Fixed: parity harness comparison now honors per-test numeric epsilon for stream
  comparison, avoiding false failures from tiny runtime float drift.
- Fixed: parity harness native height-map setter now uses the native
  `SetHeightMapFunc` guard, matching the native API contract.
- Fixed: tracked local workflow cleanup: `.claude/settings.local.json` was
  removed from the branch, and the root `plan.md` note was moved to
  `test/native_api_parity/PARITY_PLAN.md`.
- Fixed: upstream `DrawBuildSquare` now has a native callin query/result,
  engine dispatch through `NativeInterfaceEventClient`, and a safe Rust
  `NativeModule::draw_build_square` callback with status bytes exposed as a
  slice.
- Checked: Lua sandbox/socket initialization survived the full headless parity
  run; the run completed cleanly and logged the active LuaSocket restrictions.
- Checked: native message bridge and `Spring.InvokeNativeModule` passed the full
  headless parity harness against the rebased engine.
- Checked: `NativeInterfaceSystem` load path passed runtime parity harness with
  `SPRING_NATIVE_MODULE`; reload fallback path was removed as above.
- Checked: rendering-enabled parity harness ran on the rebuilt `spring` binary
  with a real GL renderer and passed the native `gfx_compute_upload` smoke.
- Fixed: full game-rules resource packs are exposed through the native/Rust
  `Game.get_game_rules_resource_info` query while preserving the old
  `GameRulesInfo` scalar compatibility layout. The parity harness now checks
  both the Lua-visible compatibility constants and the relevant native
  resource-pack components.

## Build log

- `docker-build-v2/build.sh -j 8 linux` initially failed on duplicate
  `CSolidObject::paletteIndex`; fixed by keeping upstream's field/comment.
- `docker-build-v2/build.sh --compile -j 8 linux` then failed on old
  `CModInfo` resource-cost scalar names in `NativeInterface/api/Game.cpp`;
  fixed by mapping the compatibility fields to the new resource-pack members.
- `cargo check --workspace` from `rust/`: passed.
- `cargo test --workspace` from `rust/`: passed.
- `cargo check` from `test/native_api_parity/native`: initially failed on
  `RulesParamValue` type drift, then passed after the helper fix.
- `cargo test` from `test/native_api_parity/native`: passed.
- `cargo fmt --all --check` from `rust/`: passed.
- `cargo fmt --check` from `test/native_api_parity/native`: failed because the
  WIP parity crate is broadly not rustfmt-formatted. This was not mass-formatted
  during the rebase to avoid unrelated churn.
- `docker-build-v2/build.sh --compile -j 8 linux`: passed, producing/installing
  `spring`, `spring-dedicated`, `spring-headless`, `libunitsync.so`, and test
  executables.
- Follow-up `cargo test --workspace` from `rust/`: passed after adding the
  Platform wrapper and updating the mock example initializer.
- Follow-up `cargo fmt --all --check` from `rust/`: passed.
- Follow-up `cargo check` and `cargo test` from
  `test/native_api_parity/native`: passed after adding Platform parity and the
  native height-map edit guard.
- Follow-up `docker-build-v2/build.sh --compile -j 8 linux`: initially caught a
  `Platform::GetArchitectureStr()` string lifetime issue; fixed by returning a
  stable function-local static string, then passed.
- `python3 test/native_api_parity/run_harness.py --mode both --timeout 180`:
  initially exposed a stale default `build-linux` binary path, a headless Gfx
  smoke-test issue, and native height-map guard misuse; fixed all three, then
  passed with output in `test/native_api_parity/out/20260703-004856`.
- `python3 extract_rust_api.py` and `python3 match_apis.py` from
  `rust/crates/spring-native`: passed; regenerated Rust API inventory and API
  comparison docs.
- Follow-up `cargo check --workspace` from `rust/`: passed after adding
  `Game.get_game_rules_resource_info` and `NativeModule::draw_build_square`.
- Follow-up `cargo check` from `test/native_api_parity/native`: passed after
  adding `game_rules_info` parity coverage.
- Follow-up `docker-build-v2/build.sh --compile -j 8 linux`: passed after the
  resource-pack and `DrawBuildSquare` native API changes.
- `python3 test/native_api_parity/run_harness.py --mode both --timeout 180`:
  passed with output in `test/native_api_parity/out/20260703-073701`.
- `python3 test/native_api_parity/run_harness.py --mode both
  --enable-rendering-tests --timeout 180`: passed with output in
  `test/native_api_parity/out/20260703-073712`; report shows rendering tests
  enabled, GL renderer present, `game_rules_info` 3/0, and
  `gfx_compute_upload` 1/0.
- Follow-up `cargo test --workspace` from `rust/`: passed.
- Follow-up `cargo test` from `test/native_api_parity/native`: passed.
- Follow-up `python3 extract_rust_api.py` and `python3 match_apis.py` from
  `rust/crates/spring-native`: passed; regenerated Rust API inventory and API
  comparison docs (`1284` Rust functions, `522` Rust-only native surfaces).
