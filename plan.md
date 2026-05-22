Goal
Build a scalable native-vs-Lua parity test framework for Spring APIs, targeting 100% inventory coverage:

- Lua: currently 65 / 776
- Native Rust: currently 62 / 876
- Current detail source: test/native_api_parity/out/20260519-074702/coverage_details.md

Current State
Relevant files:

- test/native_api_parity/run_harness.py
- test/native_api_parity/fixtures/game.sdd/LuaRules/Gadgets/native_api_parity.lua
- test/native_api_parity/native/src/lib.rs
- test/native_api_parity/TODO.md

Current weakness:

- Lua tests, Rust tests, and Python coverage metadata are three separate sources of truth.
- Requirements like “needs unit/feature/projectile” are implicit.
- Params and optional params are ad hoc.
- Rust registry is name-driven but still hand-written per function.

Target Architecture
Create one canonical spec file, e.g.:

test/native_api_parity/api_tests.yaml

Each entry should describe:

- id
- context: synced_gadget, unsynced_gadget, widget
- kind: readonly, setter_getter, create_destroy, query_list, side_effect
- requires: fixtures like unit, feature, projectile, team_pair, ground_point
- Lua API names
- Native API names
- params and generators
- comparison fields
- known semantic caveats

Example:

- id: unit_health
context: synced_gadget
kind: setter_getter
requires: [unit]
lua:
	set: Spring.SetUnitHealth
	get: Spring.GetUnitHealth
native:
	set: UnitControl.set_unit_health
	get: UnitsInfo.get_unit_health
params:
	health: { type: f32, range: [100, 900] }
	paralyzeDamage: { type: f32, range: [0, 30], optional_cases: [value] }
	captureProgress: { type: f32, range: [0, 0.3] }
	buildProgress: { type: f32, range: [0.4, 1] }
compare:
	epsilon: 0.05
	fields: [health, maxHealth, paralyzeDamage, captureProgress, buildProgress]

Phase 1: Single Source Of Truth

1. Add api_tests.yaml.
2. Move all current 44-ish checks into it.
3. Generate Python CHECK_COVERAGE from spec.
4. Generate coverage details from spec + inventories.
5. Keep report.md compact.

Acceptance:

- run_harness.py --mode compare still works.
- Report still shows Lua/native totals.
- No duplicated coverage metadata in Python.

Phase 2: Fixture System
Create a fixture manager in Lua with declared resources:

requires: [unit, feature, ground_point]

Fixture manager should provide:

- unitID
- featureID
- projectileID
- teamID
- allyTeamID
- groundX, groundZ
- selected unit
- visible/in-LOS unit
- rules param key/value
- feature/unit defs

Acceptance:

- Tests no longer hand-create unit/feature inline.
- Fixture setup is reusable per case.
- Fixture cleanup is explicit for destructive APIs.

Phase 3: Param Model
Implement param generation from spec:

- scalar: i32, f32, bool, string
- enum: fixed values
- object/table
- float2, float3, int3
- optional cases: omit, nil, value
- default-value cases
- valid ranges only, not fuzzing

Example:

params:
pos:
	type: float3
	generator: map_position
	expands_to: [x, y, z]
snapToGround:
	type: bool
	optional_cases: [omit, false, true]

Acceptance:

- One test can generate multiple valid cases.
- Optional params are tested systematically.
- Report includes case count per function.

Phase 4: Lua Test Generator
Generate or load Lua tests from spec.

Preferred:

- Python reads api_tests.yaml
- writes generated Lua table file:
fixtures/game.sdd/LuaRules/Utilities/generated_api_tests.lua

Lua runner becomes an interpreter:

- resolve fixtures
- generate params
- call Lua setter/getter
- send normalized JSON row
- request native setter when applicable

Acceptance:

- Most Lua-side test additions are spec-only.
- Custom Lua hooks only for unusual cases.

Phase 5: Rust Macro/Codegen
Rust cannot dynamically call methods by string, so use macro/codegen.

Add generated file:
test/native_api_parity/native/src/generated_tests.rs

Shape:

native_tests! {
	unit_health {
		check = check_unit_health,
		set = set_unit_health,
	}
}

Better long term:

- generate registry entries
- generate field extraction
- generate common comparisons
- allow custom handlers where needed

Acceptance:

- TESTS table is generated.
- Param extraction boilerplate is generated where possible.
- Custom Rust functions only exist for semantic exceptions.

Phase 6: Comparison Semantics
Standardize comparison types:

- exact bool/int/string
- float epsilon
- vector epsilon
- unordered list/set
- sorted list
- optional/nil normalization
- Lua table vs Rust struct field mapping
- “overlap only” comparisons for known semantic differences

Spec example:

compare:
mode: unordered_set
field: unitIDs

Acceptance:

- No custom comparator unless necessary.
- Known semantic mismatches are explicitly labeled.

Phase 7: Context Coverage
Split by execution context:

- synced gadget
- unsynced gadget
- widget/LuaUI
- maybe LuaMenu later if needed

Spec declares availability:

contexts: [synced_gadget, unsynced_gadget]

Acceptance:

- APIs are tested in valid contexts.
- Report shows per-context coverage.

Phase 8: Coverage Accounting
Keep using:

- rust/crates/spring-native/lua_functions.md
- rust/crates/spring-native/rust_functions.md

Generated outputs:

- report.md: concise summary
- coverage_details.md: full tested/untested list
- optional failures.md: only failures
- optional semantic_mismatches.md: known non-equivalent APIs

Acceptance:

- Unknown tested names must stay 0.
- Untested count should monotonically go down.
- Report must not contain giant lists.

Phase 9: Expansion Order
Recommended order:

1. Pure read-only game/system/math conversions.
2. Teams/player read-only getters.
3. Unit query/list/count getters.
4. Feature query/list/count getters.
5. Unit defs / feature defs / weapon defs.
6. Unit setter/getter state APIs.
7. Feature setter/getter state APIs.
8. Rules params.
9. Terrain/map/metal/grass/smooth mesh APIs.
10. Projectiles with projectile fixture setup.
11. Commands/queues.
12. Piece/model APIs.
13. Selection/widget-only APIs.
14. Camera/input/display unsynced APIs.
15. Destructive/create APIs with isolated cleanup.
16. Known semantic mismatch audit.

Important Design Rule
Do not count a function as covered just because it was invoked. Count it only when the framework verifies equivalent
observable behavior between Lua and native, or explicitly records an overlap/mismatch category.

Validation Commands
Use these after each batch:

python3 -m py_compile test/native_api_parity/run_harness.py
cargo build --manifest-path test/native_api_parity/native/Cargo.toml --release
python3 test/native_api_parity/run_harness.py --spring build-linux/spring-headless --mode both --timeout 120 --skip-native-
build --cases 5
python3 test/native_api_parity/run_harness.py --mode compare

Immediate Next Task
Implement Phase 1:

- create api_tests.yaml
- migrate current tests into it
- make run_harness.py derive coverage/report metadata from it
- leave Lua/Rust execution mostly as-is until Phase 4/5

That gets the project onto one source of truth before adding hundreds more functions.
