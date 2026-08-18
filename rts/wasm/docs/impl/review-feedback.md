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

---

# Round 4

Date: 2026-08-18
Reviewed: the benchmark and callout-optimisation work in the current
uncommitted tree, plus the transport measurements in
`../considerations/measured_costs.md`.

## 9. Four callin benchmark rows measure nothing (P1, measurement validity)

`benchmarking_results.md` publishes Wasm figures for `callin_unitcreated`,
`callin_unitpredamaged`, `callin_allowunitcreation` and `callin_update`. None of
them reach a guest.

`test/wasm_api/benchmark_guest/src/lib.rs` implements `game_frame`,
`game_frame_post` and `update` and nothing else, so the first three rows time
the engine's path for a callin the guest never exports. Separately, `Update` is
dispatched with `synced=false` and so only reaches unsynced environments, while
the benchmark guest loads in `RulesSynced`; `callin_update` at 138 ns is not a
guest dispatch at all, and reads as the fastest Wasm callin only because
nothing happens.

Half the Wasm callin rows are therefore engine overhead wearing a Wasm label,
and the Wasm-versus-Lua ratios on those lines are not comparisons. Either the
guest implements those callins, or the rows are marked unavailable. This is the
failure mode item 8 already names: a number was published without checking it
measured what its label claims, and no gate covers it.

## 10. The core-wasm path uses the expensive C API entry points (P2)

`WasmModule.cpp:1967` registers the core host import with
`wasmtime_linker_define_func`, and guest exports are called with
`wasmtime_func_call`. Measured against the same functions, the checked forms
cost 34.5 ns and 139.0 ns per call; `wasmtime_linker_define_func_unchecked` and
`wasmtime_func_call_unchecked` cost 4.1 ns and 10.8 ns.

Only test fixtures use this path, so nothing ships slower for it today. It
matters because item 7's outstanding "performance fixture that goes through a
generated adapter instead of a hand-assembled core import" would inherit the
choice, and because `wasmtime_func_call` being slower than the entire Component
Model path is unintuitive enough to be worth recording.

## 11. Every callout converts through two value trees (P2, measured)

A callout lowers `wasmtime_component_val_t` into `WasmValue`
(`WasmModule.cpp:LowerComponentValue` and `LiftComponentValueTyped`), then the
generated adapter lowers `WasmValue` into the native struct
(`Read_*Query` / `Write_*Result`). `WasmValueRecord` is
`std::map<std::string, WasmValue>`, so a three-field record costs three map
nodes and three `std::string` keys, and the outbound side adds a
`wasm_name_new` copy per field plus a linear `FindSemanticRecordField` search.

Measured by hand-writing specialised callbacks for two callouts and reverting
them: `callout_scalar` 183.6 to 128.0 ns, `callout_vec3` 881.1 to 452.2 ns. So
646 of `callout_vec3`'s 881 ns was our own marshalling, not Wasmtime's.

Worth knowing before anyone spends time on it: the remainder is not reducible
on this transport. Wasmtime builds the incoming argument as a heap value tree
before the host callback is entered, and for the `(s32, record{bool, bool})`
signature of `get-unit-position` that costs 200 ns on its own.

## 12. The spread column can exceed the value (P3, cosmetic)

Several `benchmarking_results.md` rows print a spread larger than the
measurement, for example `callin_unitcreated` at 200 ns plus or minus 170 ns.
The medians are sound; the spread is reported as max minus min. Reporting p5 to
p95 would stop the table looking unreliable where it is not.

---

# Round 4 response

Date: 2026-08-18

## 9. Four callin rows measure nothing — fixed

Correct on all four, and all four were understated. The three missing callins
are now declared in the benchmark world (`benchmark_guest/build.rs`, not
`parity.wit`, which is generated and shared with the parity harness) and
implemented with empty bodies so the row times dispatch rather than a body.

`Update` is dispatched `synced=false`, so no synced guest can ever receive it.
It now has its own callin sub-run against an unsynced guest, under a distinct
`update` variant so the synced runs stop recording a row for a dispatch that
reaches nothing.

| test | before | after |
| --- | ---: | ---: |
| `callin_update` | 138 | 1080 |
| `callin_unitcreated` | 445 | 1378 |
| `callin_allowunitcreation` | 686 | 2673 |
| `callin_unitpredamaged` | 885 | 3185 |

`update` now lands on top of `callin_empty` at 1080, which is the consistency
check that both are real dispatches, and the ordering tracks payload size.

## 10. Checked C API entry points — fixed

`wasmtime_linker_define_func_unchecked` and `wasmtime_func_call_unchecked` are
now used on the core-wasm path. Unchecked carries no type information, so the
functype is read once and every slot is filled and read against it, and
unsupported result types are rejected before the call rather than after.
`test_WasmInterface` is 2552 assertions green.

## 11. Double value-tree conversion — measured, and the conclusion revised

The 646 ns finding stands. The closing claim does not.

Item 11 says the remaining ~200 ns of Wasmtime-built argument tree is "not
reducible on this transport". That is exactly right, and the qualifier turns
out to be the important part: it is a property of the dynamic C API, which must
materialise the tree before entering the host callback. A host using Rust static bindings never builds it.

`callout_vec3`: 886 ns on the C API path, 55 ns on the typed path. The
hand-specialised C callbacks reached 452 ns; the typed transport is eight times
better than that, because it removes the incoming tree as well as the outgoing
one.

The same effect shows on callins, where the typed cost is flat across query
sizes while the C API path scales with them.

Both transports now run in the same suite as separate backends, so this is a
standing comparison rather than a one-off measurement. The tables and the
revised reasoning are in
[../considerations/measured_costs.md](../considerations/measured_costs.md);
[handoff.md](handoff.md) carries the working state and what is still
unmeasured.

## 12. Spread column — closed, will not fix

The engine-side callin recorder reports p5 to p95, which covers the row the
item cites. Guest-side rows stay max minus min. Not worth further attention.

---

# Round 4 follow-up

Date: 2026-08-18

Two rows published as `unavailable` in the round 4 table are now measured, so
the table has no unavailable cells left.

`callin_4modules`: the typed host was a process singleton, so a four-module run
entered one instance. Hosts are now per module, keyed by module name. The row
measures 531 ns against 146 ns for a single module, a 3.6x fan-out. This also
fixed a lifetime bug, since the singleton outlived the module list.

`draw`: the UI world is implemented. It was expected to be large because the
guest's world imports `gfx` wholesale, but dead-code elimination runs before
componentization, so the draw component imports two of the 237 gfx callouts.
The guest is unchanged, so the rows measure what they measured before.
