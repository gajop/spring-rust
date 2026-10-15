# Recoil Native and Wasm Interfaces: No Stubs Plan

Status: in progress (2026-10-15). Native gaps are resolved or explicitly classified; the static gate, generated mutants, comparator recording and evidence ledger are implemented. Full behavioural/Wasm verification remains incomplete.

Goal: every function in the NativeInterface API tables (`rts/NativeInterface/api/*.cpp`) does what its Lua counterpart in `rts/Lua/LuaSynced*` / `LuaUnsynced*` / `LuaOpenGL` / `LuaFBOs` does, for native Rust modules and for Wasm modules (`rts/wasm/generated`, `rts/WasmInterface`). A stub must fail CI on the day it is written, not when a game notices.

This plan extends [Parity & Testing](recoil_wasm_parity_testing.md) sections 5 (runtime parity harness) and 10 (CI gates). The generator now reads `verified` from the evidence ledger; it remains false for all current functions because complete evidence has not been collected.

## Implementation status (2026-10-15)

The implementation below is committed. The inventory is the historical 2026-10-11 baseline; remaining acceptance work is listed separately.

- A: full display returns use appended `GetScreenGeometryFull` / `HaveAdvShadingFull` slots, preserving older native result layouts. Native ABI minor version is 1.15. Legacy display queries now select the requested screen/usable rectangle.
- B: mouse deltas report the last delivered motion event in screen-up coordinates; feature-definition resurrection identity is explicitly unavailable (use the instance query); VFS distinguishes directories and reports backing-file permissions or zero for unavailable metadata; heading updates direction vectors and uses the requested up-vector smoothing. Water queries call `CGround`.
- C/D/E: semantic exceptions are recorded in `ParityExemptions.def` and source-hash-bound dispositions. Unsupported closest-position queries are excluded from Wasm and require `unverified-api` in the native Rust SDK. Dead residue was removed earlier.
- The libclang audit scans the model's native implementations using build commands and has candidate checks for S1–S6. It rejects new/stale findings, changed reviewed source, missing reasons and parse errors. Analysis releases each translation unit rather than retaining all engine ASTs. It is a conservative source audit, not a proof of behavioural correctness.
- Private native table copies and zero/no-op replacements are generated for all 1,425 modeled callouts. The targeted fixture passes 179 checks, kills 14 zero replacements and detects a success-no-op heading setter through readback. `NATIVE_API_TEST_HOOKS` adds process-scoped selection via `SPRING_NATIVE_API_MUTANT`; the fresh-process sweeper requires baseline success, activation and targeted failure. Hook-enabled source syntax checks pass; a hook-enabled engine build and full sweep remain pending. Replacement generation alone is not D5 evidence.
- All 13 `*_if_present` comparator helpers record whether a comparison actually happened. Declared fields are required in native and Wasm checks unless explicitly optional with a reason; both sides omitting a required field now fails. The ledger tracks missing/default/distinct fields and requires direct comparison, return mapping, mutants and Wasm environments before `verified`. Current evidence: **14 exercised, 1,411 untested, zero verified**. Aggregate reports headline function evidence. Strict generation rejects new identities without evidence/exemption, production Wasm imports withhold them and the native SDK requires `unverified-api`. Stale test references and increases in grandfathering fail gates.
- CI includes the audit, audit tests, generated-mutant check, targeted native run and ledger ratchet. Remote CI has not been executed.

Validation: both engine variants built; 12 audit tests, 15 code-generator library tests, 8 production-policy tests and 11 offline evidence tests passed. The targeted native parity run passed before the stricter required-field checks. Full native parity exposed destructive `kill_team`/`game_over` tests ending the shared fixture early; they now run once at the end, with runtime recheck pending. A Wasm rendering probe exposed missing rendering-flag forwarding; this is fixed, builds/runs are serialized, cached binaries carry selection/SDK fingerprints, and default projections are restored afterward. Its follow-up timed out during font discovery and is **not** a passing Wasm result. Generated Wasm artifacts are reproducible, the 57-file native snapshot is current, and changed codegen files pass rustfmt. The workspace format check passes after formatting the SDK, AI example and bindings tool. Testing remains light at the user's request; no further engine or benchmark runs were started.

Remaining acceptance work: strict required/optional return mappings across the full surface, full native/Wasm behavioural evidence in all environments, all-function mutant execution, and runtime rechecks of the fixture changes and hook-enabled engine. Full performance acceptance is tracked separately. The plans stay open; no `verified` claim is inferred from labels or from generating a mutant.

## Why the existing gates passed a stub

`NativeGetTeamLuaAI` returned no AI for every team until `45c811a942`. It had a parity test (`get_team_lua_ai` in `test/native_api_parity/api_tests/known_issue_regressions_01.json`) and the last full PASS run (`out-final-rendering-lsan/20260816-170030-953615`) recorded it as passing 3/3:

```
{"luaAI":"","hasLuaAI":false,"testName":"get_team_lua_ai","teamID":0,...,"case":1}
```

The fixture script written by `run_harness.py` has no `[AI]` section, so Lua also returned nothing. Three mechanisms let it through:

| Mechanism | Where | Effect |
|---|---|---|
| Label coverage | `coverage_summary()` in `run_harness.py`, `aggregate_coverage.py` | A function counts as tested when a passing test names it. The same run reported Lua `Spring.*` 791/791 and native counterparts 1134/1134 |
| Vacuous fixture values | `fixtures/game.sdd`, script in `write_script()` | Lua reference returned the type default in every case; a stub returning defaults agrees |
| Skipping comparators | `same_*_if_present` in `test/native_api_parity/native/src/support.rs` (1032 call sites) | A field absent from the Lua row is not compared at all |

The Wasm probe has a vacuity check (`WASM_VACUITY_THRESHOLD`), but it counts only rows whose status is not `pass`; a passing row of default values is not vacuous by that rule.

## Inventory

Counted 2026-10-11 at `45c811a942` (branch `rust-wip`).

### Method

1. Static scan of every API-table slot (`const XApi X_API = { .Name = Impl, ... }`) in the 49 `.cpp` files, with query struct fields parsed from the matching headers. Prototype regex scan; stage 1 replaces it with a libclang tool. Rules are the static rules S1–S6 below.
2. Manual triage of every hit against the Lua implementation.
3. Vacuity measurement over the Lua result streams of the last full PASS run (2026-08-16; the spec has grown since, so this undercounts current tests).

### Static scan

| Item | Count |
|---|---:|
| Functions in the model (`rts/wasm/generated/generation-report.json`) | 1423 in 59 modules |
| API-table slots scanned | 1444 (includes 12 sub-table pointers in `SyncedCtrl.cpp`/`UnsyncedCtrl.cpp`/`UnsyncedRead.cpp`, 11 bodies generated by `IMPL_SIMPLE_QUERY` in `Game.cpp` that the prototype cannot read) |
| Slots with any raw hit | 237 |
| Hits from empty query structs (`_unused` placeholder field) | 159, not stubs |
| Functions with named query fields never read | 25 |
| Result fields only ever assigned one literal | 57 fields in 41 functions |
| Comment or error-message markers (`Simplified`, `for now`, `Would need`, `Full implementation would`, `Not easily accessible`, `Delta not directly available`, `Default water level`, `Closest equivalent`, `not implemented`, `stubs`) | 17 sites |

### Triage

**A. Diverges from Lua (8 functions).** Each passes or is untested today because the fixture never produces the differing case.

| Function | Location | Lua does | Native does |
|---|---|---|---|
| `GetTeamInfo` | `Teams.cpp:78` | Returns the team's custom keys table | `customKeys = getTeamKeys ? "" : nullptr` |
| `GetPlayerInfo` | `Teams.cpp:384` | Returns the player's custom options | `customKeys = getPlayerOpts ? "" : nullptr` |
| `HaveAdvShading` | `Display.cpp:425` | Returns `true` and the ground drawer's `UseAdvShading()` | `enabled = true` ("Simplified: assume"); second value missing |
| `GetScreenGeometry` | `Display.cpp:110` | Honours `displayIndex` and `queryUsable` | Ignores both; returns the window's screen ("Simplified") |
| `Log` | `Messages.cpp:64` | Logs to the given section at the given level | Ignores `section` and `level` ("Simplified: just log to default section") |
| `TestMoveOrder` | `Utils.cpp:147` | Tests terrain/objects at `pos` and `dir` | Ignores `pos`, `dir`, `options`; returns `true` for any mobile unit def ("Full implementation would query pathfinder") |
| `GetUnitShieldState` | `UnitsInfo.cpp:1207` | Selects the shield by `weaponNum` | Ignores `weaponNum`; takes the first shield |
| `ClearAttachmentFBO` (`gl`) | `Gfx.cpp:2628` | Uses `target`; refuses when no non-default FBO is bound | Ignores `target`; clears whatever is bound, including the default framebuffer |

**B. Native-only fields or parameters with invented values (9 functions).** No Lua counterpart to compare against, so parity tests cannot catch them.

| Function | Location | Field |
|---|---|---|
| `GetUnitWeaponState` | `UnitsWeapons.cpp:113` | `turnRate = 0` ("Not easily accessible") |
| `GetMouseState` | `Input.cpp:100` | `dx`, `dy = 0` ("Delta not directly available") |
| `GetPieceProjectileParams` | `Projectiles.cpp:242` | `modelPieceNum`, `modelObjectType = 0`, `modelName = ""` |
| `GetFeatureDefByID` | `FeatureDefs.cpp:84` | `resurrectAs = ""` ("Would need to look up unit def") |
| `GetFactoryCounts` | `UnitsCommands.cpp:323` | `currentCount` ("Simplified") |
| `GetFileInfo` (VFS) | `VFS.cpp:150` | `mode = 0644`, `isDirectory = false` |
| `GetUnitShieldState` | `UnitsInfo.cpp:1230` | `shieldAlpha = 1.0f` |
| `GetUnitDefByID` | `UnitDefs.cpp:340` | `canHover = hoverAttack` ("Closest equivalent"; a different property) |
| `SetUnitHeading` | `SyncedCtrl.cpp:3009` | `useSmoothing` ignored |

**C. Agrees with Lua only because Lua is also a stub or constant (7).** Each needs a decision: implement both, remove from the native API, or exempt with a reason.

| Function | Location | Note |
|---|---|---|
| `GetClosestValidPosition` | `LOS.cpp:202` | Lua body is `// FIXME -- finish this; return 0;`. Native returns `NOT_IMPLEMENTED_ERROR`. Allowed as a production Wasm import that always fails |
| `GetScreenViewTrans` (`gl`) | `Gfx.cpp:5071` | Lua returns a file-static `float3` that is never written |
| `GetWaterPlaneLevel`, `GetWaterLevel` | `Terrain.cpp:118`, `:131` | Hard-coded `0.0f` instead of calling `CGround`; correct only while `CGround::GetWaterPlaneLevel()` is `consteval 0` |
| `AddUnitImpulse.decayRate` | `SyncedCtrl.cpp:1370` | Lua ignores it too |
| `SpawnSFX.radius`, `.damage` | `SyncedCtrl.cpp:6179` | Lua ignores them too (commented) |
| `SetWMCaption.titleShort` | `UnsyncedCtrl.cpp:661` | Lua ignores it too |

**D. Intentional native semantics (16 functions).** Exempt with a reason: `GarbageCollectCtrl` (no Lua heap; success no-op), `GetLuaMemUsage` handle fields (no Lua handle), 7 `Memory.Free*Array` `length` fields, `GetUnitMapCentroid` (Lua map/array distinction is a table shape), `GetCameraState.useTable`, `GetUnitLosState.raw` (native returns both forms), `GetProjectileDamages.tag` (native returns all tags), `MatrixDataFromProjectilesVBO.teamID` and `FontGetTextWidth`/`FontGetTextHeight` position fields (shared query structs).

**E. Dead stub residue (3).** `UnitsCommands.cpp:36` `NOT_IMPLEMENTED_ERROR` ("Command issuing not implemented", unused); `UnsyncedCtrl.cpp:1006` `NativeStubNotAvailable` (unused); `UnsyncedRead.cpp:1091` "Additional stubs" banner over real implementations.

The Rust layers (`rust/crates/spring-native/src`, `rts/wasm/generated/sdk`) have no `todo!`/`unimplemented!` or stub markers; they are generated and snapshot-checked. Generated Wasm adapters dispatch through `state->native-><module>-><Function>`, so a C++ stub reaches native and Wasm modules alike.

### Vacuity of existing tests

Last full PASS run, 847 tests with `compare.fields` and Lua rows:

| Measure | Tests |
|---|---:|
| Every compared field held its type default (`nil`, `0`, `false`, `""`, empty) in every case | 279 (263 excluding deliberate `_missing`/`_invalid`/`_nil` negative tests) |
| At least one compared field default or absent in every case | 89 more |
| Identical result in every case | 650 |

Examples among the 263: `get_team_lua_ai`, `is_game_over`, `get_local_player_id`, `get_selection_box`, `get_active_command`, `is_unit_selected`, `get_build_spacing`. Core Wasm: `verified_total` is 0 of 1423 executable imports.

## Definition of a stub

A function is a stub if any rule below holds and no exemption covers it. Every rule is checked by a tool; none needs review judgement.

### Static rules (`rts/wasm/check_native_stubs.py`, no engine)

| Rule | Check |
|---|---|
| S1 Missing slot | Table slot is `nullptr`, absent, or points to a function that only sets an error |
| S2 Unconditional error | On every path past readiness/validity guards, `result->error` is set to a non-null error |
| S3 Ignored input | A query field is never read (`query->field`) and the query is not passed whole to a helper |
| S4 Constant output | A result field is assigned the same literal on every path (default initialisation followed by a real assignment does not count) |
| S5 Constant function | Past the guards, every result assignment is a literal |
| S6 Marker | A comment or error message in the body matches the marker list (`stub`, `TODO`, `FIXME`, `not implemented`, `simplif`, `for now`, `would need`, `not easily/directly available/accessible`, `placeholder`, `assume`) |

The tool parses with libclang using the engine's compilation commands, so macros such as `IMPL_SIMPLE_QUERY` resolve. The implemented report is `rts/wasm/docs/generated/native_stub_audit.json`. Handwritten Wasm adapter auditing remains additional work; the current audit covers modeled native slots.

### Dynamic rules (parity harness, engine run)

| Rule | Check |
|---|---|
| D1 Unexercised | No test calls the function in any environment where `model.json` makes it available |
| D2 Vacuous | A compared field holds the type default in every case, or every case returns an identical result, without a `constant` declaration in the spec |
| D3 Unchecked return | A Lua return value (from `@return` in the Lua source) has no compared native field and no exemption |
| D4 Skipped comparison | A `*_if_present` comparison was skipped in every case |
| D5 Surviving mutant | The function's tests still pass with its table slot replaced by a generated stub |

D5 subsumes most stubs directly: a test that cannot tell the real function from a zero-returning one does not test it.

## Exemptions

New file `rts/NativeInterface/api/ParityExemptions.def`, same syntax and review rule as `WasmAnnotations.def` ("an audit trail, not a way to silence unknown declarations"):

```
UNSUPPORTED(los, GetClosestValidPosition, Lua implementation is an unfinished FIXME; removed from Wasm imports)
IGNORED_FIELD(effects_control, SpawnSFX, radius, accepted for Lua ABI compatibility; EmitAbsSFX takes no radius)
CONSTANT_FIELD(gfx, GetScreenViewTrans, x, Lua returns a never-written static)
LUA_DIFFERS(system_control, GarbageCollectCtrl, native modules have no Lua heap)
FIXTURE_LIMITED(display, GetScreenGeometry, CI host has one display; second display covered by the nightly rendering run with two Xvfb screens)
```

| Kind | Effect |
|---|---|
| `UNSUPPORTED` | Generator omits the function from Wasm imports and the public Rust SDK; no binding exists that can only fail |
| `IGNORED_FIELD`, `CONSTANT_FIELD` | Silences S3/S4 and D2 for that field only |
| `LUA_DIFFERS` | The spec must assert the documented native result instead of skipping |
| `FIXTURE_LIMITED` | Reason must name the replacement check; D1/D2 waived only for the cases the fixture cannot produce |

Every entry needs a reason. The tool fails on an entry that matches no hit (stale) and on a missing reason. Temporary gaps with an owner go in the existing `test/native_api_parity/known_issues.json` (empty today); `rust/crates/spring-native/api_surface_contract.md` keeps the Lua-only surfaces. Triage categories A, B and E start as `known_issues.json` entries so the gates can turn on before the fixes land; C and D become `ParityExemptions.def` entries or fixes.

## Behavioural parity tests

### Oracle and comparisons

Lua is the oracle. Native and Wasm are each compared to Lua per case, and native to Wasm directly by test id, case and seed (`--test-seed`).

| What | Rule |
|---|---|
| Return values | Every Lua `@return` maps to a native result field (D3). Tables compare by key; arrays by order unless the spec says set |
| Numbers | Exact after conversion to the native type; `compare.epsilon` only with a reason in the spec. Native against Wasm is always exact |
| Errors | Lua `nil`/no return ↔ native `exists=false` or `ERROR_INVALID_ARGUMENT`/`ERROR_NOT_FOUND`; Lua raise ↔ native error. Each function with ID or range inputs gets at least one invalid case and at least one valid case with a non-default result |
| Setters | The target value must differ from the pre-read value; the effect is read back through the Lua getter and the native getter, or through the callin trace when no getter exists |
| Strings and handles | Compared by value; handle IDs by the object they resolve to |

### Contexts

| Lua role | Native | Wasm environment |
|---|---|---|
| LuaRules synced (`synced_gadget`) | Compared | `rules-synced` |
| LuaRules unsynced (`unsynced_gadget`) | Compared | `rules-unsynced` |
| LuaGaia synced / unsynced | Compared via the same specs | `gaia-synced`, `gaia-unsynced` |
| LuaUI (`widget`) | Not compared (native client has full read; `native_comparable()`) | `ui`, with the ally / enemy-in-LOS / radar-only / undetected cases from Parity §6 |

A function is exercised in every environment its `model.json` entry lists, at least once in a synced and once in an unsynced environment when it is available in both. Synced Wasm observations also go through the existing cross-platform comparison in `wasm-cross-platform.yml`.

### Fixtures

Cases come from the spec's `params` ranges with seeded random draws, plus generated boundary cases per type (valid fixture ID, `-1`, one past the last ID; out-of-map coordinates; empty string). Global queries that cannot vary inside one run get a second script variant with different values, run in the same CI job.

Additions the triage requires:

| Addition | Makes non-vacuous |
|---|---|
| Lua AI on `TEAM1` (`[AI0]` with a game `LuaAI.lua`) | `GetTeamLuaAI`, `GetAIInfo` |
| Team custom keys and player custom options in the script | `GetTeamInfo`, `GetPlayerInfo` |
| Unit def with two weapons, a shield in the second slot, stockpile, transport | `GetUnitShieldState`, `GetUnitWeaponState`, `GetUnitStockpile`, `GetUnitIsTransporting` |
| A unit killed with piece explosions | `GetPieceProjectileParams`, `GetPieceProjectileName` |
| Factory with a queue, game/team/unit rules params, non-default map and mod options | `GetFactoryCounts`, rules params, option queries |
| Infolog capture compared per line (section, level, text) | `Log` |
| Script variant: second ally team layout, different start boxes | Global team/ally queries |

## Stub mutants

The generator emits one mutant per modeled table slot into `test/native_api_parity/generated/NativeApiMutants.h` from `model.json`, so every new function gets one without hand work:

| Mutant | Body | Applies to |
|---|---|---|
| `zero` | `*result = Result{};` (no error, all fields default) | All |
| `noop` | No effect; `success = true` | Mutating functions (`mutating` in the model) |

With the CMake option `NATIVE_API_TEST_HOOKS` (on in the CI ASAN build, off in release), `NativeInterfaceSystem` hands modules private copies of the API tables when `SPRING_NATIVE_API_MUTANT=module.Function:zero|noop` is set. Wasm reaches the same copies through `state->native`. Invalid selections fail before modules are instantiated.

The sweeper runs passing baselines and mutants in fresh processes, preventing destructive setters from contaminating later tests. A matching failure and activation log are required; crashes, missing observations and unrelated failures are inconclusive. Wasm kill evidence additionally requires an actual compared field difference. Full sweep execution is pending.

## Coverage metric and ledger

Denominator: the 1423 functions in `model.json` (callouts; the 132 callins in `core-callin-executable-coverage.json` are a later stage). Each function gets one level:

| Level | Meaning |
|---|---|
| `exempt` | `UNSUPPORTED` or fully `LUA_DIFFERS`/`FIXTURE_LIMITED` with a replacement check |
| `untested` | D1 |
| `exercised` | Called and compared, but D2, D3 or D4 holds |
| `behavioural` | No D1–D4 |
| `verified` | `behavioural`, no surviving mutant, Wasm compared on the same fields in every production environment |

`aggregate_coverage.py` writes `test/native_api_parity/parity_ledger.json` (checked in): per function its level, tests, environments, cases with a non-default value per field, killed mutants, and the run it came from. The report shows levels per module and replaces label coverage as the headline; label coverage stays in the details file. The generator copies `verified` into `core-executable-coverage.json`.

## CI gates

| Gate | Job | Fails on |
|---|---|---|
| Static stub audit | `wasm.yml` `check`, before the engine build | Any S1–S6 hit not covered by an exemption or a `known_issues.json` entry; stale or reasonless exemption |
| Return mapping | Same | A Lua `@return` with no native field and no exemption (needs `@return` extraction in `rust/crates/spring-native/extract_lua_api.py`; `signatures.json` holds parameters only) |
| Behavioural parity | `wasm.yml` "Run native parity fixture" and "Run Wasm parity fixture" with `--mutants --ledger-out` | Any mismatch; any function whose ledger level is below its checked-in level; any function not in the checked-in ledger (new) below `verified`; any D5 survivor |
| Full aggregate | Nightly: rendering run (`--enable-rendering-tests`, 116 rendering tests today) plus the isolated process runs, merged by `aggregate_coverage.py --gate` | Same as above over the full surface |
| Generation | `verify_codegen.py` | Generated `verified` flags or production imports that disagree with the ledger and exemptions |

The ledger is a ratchet: levels only rise, the count of grandfathered functions only falls. Updating the checked-in ledger is a maintainer command (`aggregate_coverage.py --write-ledger`), reviewed like a snapshot diff.

Trigger changes: add `rts/Lua/**` to the `wasm.yml` path filters (the oracle can break parity), and run `wasm.yml` on pushes to `rust-wip` (today its push trigger is `master` only).

## Codegen prevention

`spring-native-codegen` reads `parity_ledger.json` and `ParityExemptions.def`:

- **New function, no evidence**: emitted to the parity guest and the native test crate so it can be tested; not emitted to the production Wasm import registry (`production_import_allowed = false`, reason "unverified: no parity evidence"); in the public `spring-native` crate behind `#[cfg(feature = "unverified-api")]`. A game using it opts in by name.
- **`UNSUPPORTED`**: not emitted anywhere public, consistent with [API & Code Generation](recoil_wasm_api_codegen.md): no runtime stubs for unavailable functions.
- **Existing unverified functions**: listed as `grandfathered` in the ledger; stay public until verified. The list may only shrink.
- **Stale evidence**: a ledger entry naming a test id missing from `api_tests/*.json` or `surface_tests.json` is a generator error.
- **Report**: `generation-report.json` gains `verified`, `grandfathered`, `exempt` counts.

Strict mode is the default in CI, as for unknown semantic types today.

## Order of work

1. **Static audit**: `check_native_stubs.py` with libclang, `ParityExemptions.def`, `native_stub_audit.md`; triage categories A, B, E into `known_issues.json`; gate in `wasm.yml` on "no new hits".
2. **Ledger and honest metric**: function-level ledger keyed by `model.json`; D1–D4 in `run_harness.py` and `aggregate_coverage.py`; strict comparators replacing `*_if_present` (absent Lua field must be declared optional); first checked-in ledger with the measured levels.
3. **Fixture enrichment**: the additions table above; boundary cases from parameter types; second script variant. Re-measure vacuity.
4. **Fix the triaged stubs**: A and B in the order Ship Game and the SpringCabal ports call them, each fix with a non-vacuous test; decide each C entry; delete E. Each removes its `known_issues.json` entry.
5. **Return mapping**: `@return` extraction and D3.
6. **Mutants**: generated `NativeApiMutants.h`, `NATIVE_API_TEST_HOOKS`, harness mutant pass, D5 in the gate.
7. **Wasm**: same field set and non-vacuity per Wasm environment; native-vs-Wasm direct comparison; fill `verified`.
8. **Codegen gating**: production imports and the Rust SDK feature gate driven by the ledger; strict mode on.
9. **Burn-down**: grandfathered functions to `verified`, module by module, largest callers first.
10. **Callins**: the same ledger levels for the 132 callins, using the existing callin trace comparison (`generate_callin_trace.py`).

## Acceptance

- `check_native_stubs.py` reports no unexempted hit; every exemption has a reason and matches a hit.
- Reverting `45c811a942` fails CI three ways: S5 on `GetTeamLuaAI`, D2 (the fixture has a Lua AI), D5.
- Replacing one function per module with its `zero` mutant in a scratch branch fails the PR gate for every module.
- Adding a NativeInterface function without a parity test leaves it out of production Wasm imports and the default Rust SDK, and the PR gate names it.
- Every function in `model.json` is `verified` or `exempt`; `verified_total` in `core-executable-coverage.json` equals 1423 minus exemptions; no grandfathered entries remain.
- The coverage report headline is the behavioural level per module, not label coverage.
