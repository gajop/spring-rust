# COB, LUS, and Rust CUS

Rust CUS should coexist with the two established unit-script styles rather than
pretend they solve the same problem in the same way.

| Property | COB | LUS | Rust CUS |
| --- | --- | --- | --- |
| Execution | engine bytecode VM | Lua | native or Core Wasm Rust |
| Sequential script UX | yes | yes | async Rust |
| Sleeps/waits/threads | yes | yes | yes |
| Engine piece animation | yes | yes | yes, via existing CUS animation machinery |
| Typed game state | limited VM values | dynamic Lua tables | normal Rust types |
| Arbitrary game-specific API | narrow/bridged | rich Lua/GG access | direct same-module Rust API |
| Custom named entry points | yes, externally callable | yes, script env functions | typed exports / direct Rust methods |
| Generated animation workflows | common | possible/common | first-class target |
| Same source native/Wasm | no | n/a | design goal |

## COB

COB is capable of complex local state machines: branches, functions, variables,
threads, sleeps, signals, waits, weapon hooks, and piece animation.

Its limitation is capability breadth. New game systems outside the exposed COB
VM operations require another bridge or engine extension.

That makes COB effective for local animation/state logic but a poor target for
"arbitrary Rust game code."

## LUS

LUS keeps the convenient sequential unit-script model while exposing Lua and
game-specific `GG` integration. Games such as Zero-K and MechCommander: Legacy
use this to put substantial gameplay state and custom functions in unit
scripts.

Rust CUS must preserve that ability to participate in game logic; otherwise it
would be closer to a modern COB frontend than an LUS replacement.

## Rust CUS

Rust CUS combines:

- the engine's existing CUS/piece-animation contract;
- sequential suspendable authoring through async Rust;
- typed persistent per-unit state;
- direct access to the owning game's Rust APIs;
- shared source across native and Wasm backends.

It is an execution/runtime proposal, not merely a source generator.
