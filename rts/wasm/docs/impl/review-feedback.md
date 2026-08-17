# Review feedback

Date: 2026-08-17 (round 3)
Reviewed: commits `5d192af314`, `a51fa840e2`, plus the current uncommitted working tree.

Feedback on `review-response.md` and the state of the tree behind it. Round 1 and round 2 items that are now closed are listed briefly; the rest is new.

## Closed since the last round

- **Environment matrix.** Derived from the Lua loader registrations. Strict generation fails on a mutating function in an unsynced or UI world, an empty interface, or an interface no world imports.
- **Signature gate.** Four independently derived sources (model, parsed WIT, extracted native Rust, Lua extractor). Not an echo any more.
- **Split host adapter, generated-artifact hygiene, probe vacuity gate.** All done.
- **LuaUI position degradation.** `GetLuaErrorVector` is now applied at `UnitsInfo.cpp:675`, `UnitsInfo.cpp:699` and `UnsyncedRead.cpp:475`. This was the round 2 blocker and it is fixed.
- **UI visibility probes.** `ui_visibility_ally_los`, `ui_visibility_enemy_los`, `ui_visibility_radar_state`, `ui_visibility_radar_position` now exist. Real coverage where there was none.
- **Synced to unsynced channel.** `Messages.SendToUnsynced` plus the `RecvFromSynced` callin are implemented.

## 1. Allow\* callin aggregation is inverted (P0, gameplay correctness)

`WasmInterfaceSystem.cpp:369` implements `or-true` as `aggregateValue || moduleValue`. `Callins.def` assigns `or-true` to the whole Allow\* family.

Spring's semantics are the opposite. `CSyncedLuaHandle::AllowCommand` at `LuaHandleSynced.cpp:649` defaults to `true` when the callin is not defined, and the gadget handler denies as soon as any gadget returns false. Deny wins.

With `or-true`:

- Module A returns false (deny).
- Module B returns true (allow).
- Result is allow.

Any permissive module silently overrides every restrictive one. That breaks anti-cheat gadgets, build restrictions, targeting restrictions, and command filters. This affects all 19 Allow\* callins.

Fix: the rule for Allow\* is AND with a default of true. Add an `and-false` aggregation and use it. Also confirm the no-modules-registered case still yields true.

## 2. Allow\* callins are exposed to unsynced environments (P0, desync)

`Callins.def` declares 19 Allow\* callins with `rules-synced|rules-unsynced|gaia-synced|gaia-unsynced`.

Every one of these is declared on `CSyncedLuaHandle` (18 `bool Allow*` overrides in `LuaHandleSynced.h`) and on nothing else. They are synced-only in Lua.

An unsynced module's return value now decides a synced outcome. Unsynced state differs per client, so two clients can disagree on whether a command was allowed. That is a desync, and it is the exact thing `execution_environments.md:39` prohibits: unsynced local state cannot influence a future synced decision.

Combined with item 1, a single client's unsynced module returning true flips the result on that client alone.

Fix: restrict all Allow\* callins to synced environments only.

Note the generator assertion added last round covers mutating **callouts**. It does not look at callin environments, and `Callins.def` is hand-written, so nothing caught this. Add the equivalent check: a callin whose Lua counterpart is declared on `CSyncedLuaHandle` may not appear in an unsynced or UI environment.

## 3. The benchmark run is not usable (P1)

`benchmarking_results.md` reports all 33 tests as completed. It is not a usable measurement.

- **Scale 0.001.** 100,000-iteration tests ran 100 iterations.
- **The Wasm column is empty.** Of 33 rows, Wasm has one number: `callout_scalar` at exactly 2000.000 ns. A round 2 µs that matches the existing perf gate budget is not a measurement. Everything else is `—`.
- **Layer 1 measures the wrong thing, by the document's own admission**: "callin_\* rows currently measure the backend callback body loop inside the benchmark callback; they should not be read as a direct engine-dispatch measurement." Callin dispatch cost is unmeasured.
- **Timer resolution swamps the signal.** Every Lua value is a multiple of 19.07 ns (38.147, 57.220, 76.294, 228.882, 572.205, 7629.395). At scale 0.001 these are two to four timer ticks. Most callin rows read 0.000 ns.
- **One result is obviously wrong.** `wl_commands` shows native at 0.144 ms against Lua at 0.002 ms, native 72x slower. That is a benchmark bug, not a finding.
- **All five heightmap rows are unavailable.**

Do not iterate on this table. Fix the harness first: run at scale 1, get the Wasm column populated, measure callin dispatch from the engine side rather than inside the callback, and use a timer with resolution well under the smallest quantity being measured. A row that cannot be measured should be absent, not present and blank.

## 4. Synced timers add a determinism surface for benchmarking (P1)

`SPRING_ENABLE_SYNCED_TIMERS` exposes `Spring.GetTimerMicros` to synced Lua (`LuaSyncedRead.cpp:944`) and the equivalent synced Wasm callout (`NativeInterfaceWasmAdapter.cpp:692`), backed by `System/SyncedTiming.h`.

Failing closed is the right design, and the stated constraint that the value must not affect lockstep is correct. Two concerns remain:

- The gate is an environment variable, so it is per client. If it is ever set on some clients and not others, and any synced path branches on availability rather than on the value, that is a desync in a code path nobody will be testing.
- Engine synced Lua (`LuaHandleSynced.cpp`, `LuaSyncedRead.cpp`) was modified to support a benchmark. That is a permanent change to the synced surface for a temporary need.

Prefer keeping the benchmark timing outside synced code. If it stays, add a test that a synced module cannot observe whether the flag is set.

## 5. Uncommitted again (P1)

77 modified and untracked entries, including engine Lua, `Callins.def`, the Wasm adapter, and regenerated artifacts. The last commit is 12:04; the working tree is several hours newer. Same problem as round 2. Commit before reporting.

## 6. Smaller items

- **Error vector coverage.** Applied at three sites. `UnitsInfo.cpp` has six position-returning functions. Confirm each one that Lua degrades is covered, and that mid-position and aim-position paths are included.
- **A removed probe row needs justification.** `review-response.md` says the radar-only `IsUnitInLos` row was dropped because Lua returns no values for a radar-only unit. If Lua returns nothing and Wasm returns something, that is a parity difference, not a reason to delete the row. Encode "returns no values" as the expected result and keep the comparison.
- **The implementer edited the benchmark spec.** `benchmarking.md` now carries a paragraph describing the harness's own batching and team-assignment choices. Implementation notes belong in the results file. Keep the spec as the spec.
- **Perf budgets are not enforced under ASAN.** Correct choice, but say so when reporting a passing CTest run so the non-ASAN run is not assumed.

## 7. Still open from the response's own list

Recorded here so they do not drift out of view. No disagreement with the classification, only with treating the phase as close to done while they stand:

- Coverage reported against the generated world surface rather than the old suite denominator. Current state is roughly 46% synced, 9% unsynced, 5% UI.
- Mutation-heavy APIs need end-to-end checks. 28 verified mutations out of 741 synced functions.
- Registered Lua functions outside the documented extractor surface (roughly 250) need normalization or a reviewed exclusion.
- Differential fuzzing between native and Wasm conversion.
- A performance fixture that goes through a generated adapter instead of a hand-assembled core import.
- A multi-component result-bearing aggregation test. Items 1 and 2 above are exactly what that test would have caught.
- arm64 and Windows runtime execution.

## 8. Process

- The round 2 blocker was fixed properly, and the response document's structure is a clear improvement. Keep that format.
- Items 1 and 2 are the same failure mode as the previous two rounds: a rule was chosen without checking it against the Lua behaviour it is supposed to match, and no gate covers the area. Before declaring the callin work done, write the test that fails when aggregation is inverted and when a synced callin reaches an unsynced world.
- Phase numbering is still swapped relative to `recoil_wasm_implementation_plan.md` (8 is parity, 9 is LuaUI).
