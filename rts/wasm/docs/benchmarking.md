# Lua / native / Wasm benchmarking plan

Compare the three backends against each other. Nothing does this today.

**33 tests, 3 backends, 99 runs.** Budget context: 33 ms/frame at 30 Hz sim, 16 ms at 60 fps, 7 ms at 144 fps.

## Fixed scenario

All tests use the same setup: 1,000 units, 2 teams, 2 ally teams, fixed seed, release build, no ASAN, same machine.

## Layers

| Layer | Tests | Measures |
| --- | ---: | --- |
| 1. Callins | 9 | Engine dispatching into the module |
| 2. Callouts | 8 | Module calling the Spring API |
| 2b. Heightmap | 5 | Callback-scoped batch edits (Springboard) |
| 3. Memory and GC | 5 | Garbage, GC pauses, memory growth |
| 4. Workloads | 6 | Complete jobs implemented three times |

## Layer 1: callins

Report ns per dispatch.

| ID | Setup | Iterations |
| --- | --- | ---: |
| `callin_empty` | Module implements `GameFrame` with empty body | 10,000 frames |
| `callin_gameframe` | `GameFrame`, reads the int arg | 10,000 frames |
| `callin_update` | `Update`, reads the float arg | 10,000 render frames |
| `callin_drawworld` | `DrawWorld`, empty body, inside draw pass | 10,000 render frames |
| `callin_unitcreated` | `UnitCreated`, reads 4 int args | 5,000 spawns |
| `callin_unitpredamaged` | `UnitPreDamaged`, ~10 args, returns 2 numbers | 50,000 damage events |
| `callin_allowunitcreation` | `AllowUnitCreation`, returns bool | 5,000 build attempts |
| `callin_unimplemented` | Module implements nothing, engine still checks | 10,000 frames |
| `callin_4modules` | 4 modules each with empty `GameFrame` | 10,000 frames |

`callin_empty` is the dispatch floor. `callin_unimplemented` is the skip cost. `callin_4modules` divided by 4 shows whether fan-out is linear.

## Layer 2: callouts

Report ns per call. Loop in one callin, no other work.

| ID | Call | Iterations |
| --- | --- | ---: |
| `callout_scalar` | `GetUnitDefID(unitID)` | 100,000 |
| `callout_vec3` | `GetUnitPosition(unitID)` | 100,000 |
| `callout_string` | `GetUnitDefName(unitDefID)` | 50,000 |
| `callout_smalllist` | `GetUnitCommands(unitID, 5)` | 20,000 |
| `callout_biglist` | `GetTeamUnits(0)`, 1,000 ids returned | 1,000 |
| `callout_spatial` | `GetUnitsInCylinder(x, z, 300)`, ~100 hits | 10,000 |
| `callout_mutate` | `SetUnitRulesParam(unitID, "bench", n)` | 100,000 |
| `callout_draw` | `gl.Vertex` equivalent, one draw pass | 100,000 |

## Layer 2b: heightmap

`Spring.SetHeightMap` and `AddHeightMap` only work inside a `SetHeightMapFunc(callback)` scope. The engine sets an in-heightmap flag, runs the callback, clears it, then does one terrain update. So this is the one API where a host callback wraps a batch of guest callouts, which makes it both the Springboard case and the re-entry guard stress test.

Report ms per `SetHeightMapFunc` invocation and ns per inner `SetHeightMap` call.

| ID | Setup | Inner calls | Invocations |
| --- | --- | ---: | ---: |
| `hm_callback_empty` | `SetHeightMapFunc` with empty callback body | 0 | 10,000 |
| `hm_brush_small` | 32×32 brush of `SetHeightMap` | 1,024 | 1,000 |
| `hm_brush_medium` | 128×128 brush of `SetHeightMap` | 16,384 | 100 |
| `hm_brush_large` | 512×512 region of `SetHeightMap` | 262,144 | 10 |
| `hm_region_op` | `LevelHeightMap` over the same 512×512 region, no callback | 1 | 1,000 |

`hm_callback_empty` isolates callback entry and exit cost. `hm_region_op` is the baseline showing what the per-point loop costs versus a single region call.

`SetOriginalHeightMapFunc` and `SetSmoothMeshFunc` use the same pattern. Benchmark one, assume the others match.

## Layer 3: memory and GC

| ID | What to run | Report |
| --- | --- | --- |
| `mem_per_call_small` | 100,000 × `GetUnitPosition` | bytes and allocation count |
| `mem_per_call_list` | 1,000 × `GetTeamUnits` | bytes and allocation count |
| `gc_pause` | 10,000 frames of `wl_unit_scan` | `GetSyncedGCInfo(true)`, total pause time |
| `frame_spike` | 5,000 frames of `wl_unit_scan` | worst frame, p99 |
| `mem_growth` | Same run as `frame_spike` | peak and steady-state memory |

Sources: `Spring.GetLuaMemUsage` and the Wasm `get-lua-mem-usage`, which report bytes and allocation count. Same functions exist for all three backends.

## Layer 4: workloads

5,000 sim frames each. Report ms per frame.

| ID | Per frame |
| --- | --- |
| `wl_unit_scan` | Walk all 1,000 team units, read position, health, def id, filter health below 50%, count |
| `wl_area_effect` | For 100 units, `GetUnitsInCylinder(r=300)`, sum the results |
| `wl_rules_params` | Set one rules param on 1,000 units, read all back |
| `wl_commands` | `GiveOrderToUnit` move order on 200 units |
| `wl_ui_draw` | Draw 2,000 world lines per render frame |
| `wl_compute` | 100,000 iteration numeric loop, zero engine calls |

`wl_compute` separates language speed from binding speed.

## Rules

- Lua baseline written properly and reviewed by someone who writes BAR Lua: localized function references, no `table.insert` or `pairs` in hot loops.
- Warm up before measuring.
- 5 repeat runs per test, report median and spread.
- Record CPU model.

The Lua benchmark localizes hot-loop state and batches the heightmap cases over
real `GameFrame` callbacks. Batching reduces scheduler overhead without
changing the per-`SetHeightMapFunc` invocation count or the measured inner
call count. The benchmark fixture keeps all 1,000 benchmark units on team 0 so
`GetTeamUnits(0)` remains the specified 1,000-id list; the game still has two
teams and two ally teams as required by the scenario.

## Output

One row per test:

| Test | Lua | Native | Wasm | Wasm vs Lua | Wasm vs native |
| --- | ---: | ---: | ---: | ---: | ---: |

## Decisions this feeds

- Is Wasm worth using over Lua.
- What is lost against native.
- Does Gfx need a raw pointer transport instead of the Component Model.
- Are the current per-call budgets set anywhere near right.

## Sources

- [Lua Performance, Spring wiki](https://springrts.com/wiki/Lua_Performance)
- [Lua in SpringRTS, Wikibooks](https://en.wikibooks.org/wiki/Lua_in_SpringRTS)
- [Updated widget/gadget profilers, Spring forums](https://springrts.com/phpbb/viewtopic.php?t=36817)
- [Lua garbage collection info, Spring forums](https://springrts.com/phpbb/viewtopic.php?t=31049)
- [Recoil Lua API](https://recoilengine.org/docs/lua-api/)
