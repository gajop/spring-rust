# Native API Parity TODO

Goal: reach 100% API parity coverage.

Current baseline from `out/20260522-222638/report.md`:

- Lua: 213 / 778 functions tested (27.4%)
- Native Rust: 245 / 876 functions tested (28.0%)
- Full generated gap list: `out/20260522-222638/coverage_details.md`

Context baseline:

- Synced gadget: 213 / 612 runtime Spring APIs tested; 399 untested; 23 known mismatch issues affecting 29 APIs.
- Unsynced gadget: 42 / 596 runtime Spring APIs tested; 554 untested; 23 known mismatch issues affecting 23 APIs.
- Widget: 42 / 597 runtime Spring APIs tested; 555 untested; 23 known mismatch issues affecting 23 APIs.

Known problem baseline:

- known_mismatch: 23 issues; 29 Lua APIs; 29 native APIs.
- inventory_mismatch: 2 issues; 2 Lua APIs; 2 native APIs.
- inventory_missing_docstring: 1 issue; 2 Lua APIs; 2 native APIs.

Process:

- Add tests as metadata, not one-off test code.
- Prefer broad generated/table-driven batches.
- Keep `report.md` short: totals, failures, links.
- Keep full function gap details in `coverage_details.md`.
- Mark a function tested only when the Lua and native observable behavior really matches.
- Keep known parity problems as data in `known_issues.json`; do not silently omit them.

## Framework Status

- [x] Canonical test metadata split across `api_tests/*.json`.
- [x] Python coverage/report metadata loaded from the canonical spec.
- [x] Generated Rust test registry in `native/src/generated_tests.rs`.
- [x] Generated Lua test metadata in `LuaRules/Utilities/generated_api_tests.lua`.
- [x] Lua runtime follows generated spec order and attaches custom hooks.
- [x] Generic Lua runtime supports read-only API rows from spec params/calls/returns.
- [x] Fixture creation/cleanup is centralized.
- [x] Reports show context, kind, case count, params, and full API gaps.
- [x] Reports show runtime `Spring` function totals per context.
- [x] Reports show known problem counts by status and by runtime context.
- [x] Portable read-only generated tests run in unsynced gadget and widget contexts.
- [x] Harness compare requires native completion, avoiding false passes on partial streams.
- [x] Rendering-required tests can be marked in spec and run with non-headless Spring.
- [x] Harness defaults use installed `spring`/`spring-headless` binaries so renderer assets are available.
- [ ] Replace remaining custom setter/getter Lua value generators with generic spec param generation.
- [ ] Generate common Rust field extractors/comparators from spec.
- [ ] Add unsynced gadget/widget-specific generated test contexts.

## Next Batches

- [ ] Read-only game/system/display getters.
- [ ] Read-only teams/player getters.
- [ ] Read-only unit query/list/count getters.
- [ ] Read-only feature query/list/count getters.
- [ ] Unit definitions getters.
- [ ] Feature definitions getters.
- [ ] Weapon definitions getters.
- [ ] Projectile getters and projectile fixture setup.
- [ ] Rules params setters/getters for game/team/player/unit/feature.
- [ ] Unit command/queue APIs.
- [ ] Unit piece/model APIs.
- [ ] Feature piece/model APIs.
- [ ] LOS/radar/query APIs.
- [ ] Map/terrain/metal/grass/smooth/original height map APIs.
- [ ] Teams/resources setters/getters.
- [ ] Unit state setters/getters.
- [ ] Feature state setters/getters.
- [ ] Projectiles/effects/spawn APIs.
- [ ] UI/input/camera/display unsynced APIs.
- [ ] VFS/config/sound/messages/misc APIs.
- [ ] Destructive APIs with isolated fixture objects.
- [ ] APIs requiring multi-team/player fixture setup.
- [ ] APIs requiring selection/widget-only context.
- [ ] APIs requiring map decals/lights/assets.
- [ ] APIs with known Lua/native semantic mismatches; canonical list is `known_issues.json`.

## Coverage Accounting

- [ ] Keep parsing total Lua functions from `rust/crates/spring-native/lua_functions.md`.
- [ ] Keep parsing total native functions from `rust/crates/spring-native/rust_functions.md`.
- [ ] Make `coverage_details.md` the source for untested function names.
- [ ] Reduce Lua untested count to 0.
- [ ] Reduce native untested count to 0.
- [ ] Keep unknown tested names at 0.
- [ ] Require full Lua/native run pass before considering a batch complete.
