# Game Evidence

The proposal should be validated against several different Spring game styles.
No single game exercises the full compatibility surface.

## Beyond All Reason

Repository: <https://github.com/beyond-all-reason/Beyond-All-Reason>

BAR is strongly COB/BOS-oriented for unit scripts and also maintains tooling to
generate/compile that content. Earlier inspection of the repository found
roughly 849 BOS files, 851 COB files, and 31 Lua unit-script files.

BAR stresses:

- very large script counts;
- mechanical/generated animation code;
- COB-compatible waits/signals/weapon hooks;
- exporter/code-generation workflows;
- coexistence with broader LuaRules game logic.

This is the strongest case for keeping generated animation data and even
Rust-to-COB tooling as useful adjacent workflows.

## Zero-K

Repository: <https://github.com/ZeroK-RTS/Zero-K>

Current Zero-K unit scripts are strongly LUS/Lua-oriented. Scripts and gadgets
make heavy use of custom script functions, `Spring.UnitScript.GetScriptEnv`,
`CallAsUnit`, and game-specific `GG` APIs.

` scripts/amphraid.lua ` is a representative CUS target:

- looping walk animation;
- `Signal` / `SetSignalMask`;
- `WaitForMove` and `Sleep(0)` yields;
- periodic background work;
- asynchronous aiming;
- synchronous weapon queries and `BlockShot`;
- calls into Zero-K-specific overkill-prevention logic.

Zero-K stresses rich game-to-unit-script integration more than COB parity.

## MechCommander: Legacy

Repository: <https://github.com/SpringMCLegacy/SpringMCLegacy>

The current public source is LUS/Lua-based; repository inspection found no BOS
or COB unit scripts. Files such as `scripts/Mech.lua`, `Vehicle.lua`, and
`Infantry.lua` contain substantial per-unit gameplay state.

MCL stresses:

- ammo, heat, limbs, weapon state, and movement state;
- many concurrent script threads;
- custom externally called functions such as ammo/state changes;
- generic base scripts plus per-unit animation modules;
- large amounts of game-specific `GG` integration.

This is a strong target for typed Rust state and custom game APIs.

## SpringCabal / Area-17

Repository: <https://github.com/SpringCabal/Area-17>

`Scripts/human.lua` uses generated/keyframed animation data and a generic
animation player with sleeps between keyframes.

It stresses the generated-animation path and exposes an important async detail:
iterators or borrows over animation data should not need to survive suspension;
stable IDs and explicit indices are better durable state.

## Other games

Metal Factions and Tech Annihilation retain substantial BOS/COB unit-script
sets. Spring: 1944 is mixed Lua and BOS/COB.

Together these games suggest two broad compatibility axes:

1. TA-derived/generated animation workloads where COB remains important.
2. LUS-heavy games where custom gameplay integration is essential.

Rust CUS should be tested against both.
