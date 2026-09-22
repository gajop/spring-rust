# Ship Game: Core WASM gaps, note 13

Found while building SpringBoard's Rendering Lab, which talks to the game's modules through
`SendLuaRulesMsg` / `HandleLuaMsg` / `SendToUnsynced`.

## 13.1 A callin payload over the module's scratch size takes the engine down

`SendLuaRulesMsg` with a ~10 KB string reaches the synced module's `HandleLuaMsg`. With the
module exported at `scratch: 4096` the callin fails:

```
Warning: Core Wasm module shipgame-rules was deregistered due to fault
Error: Core Wasm callin HandleLuaMsg failed: ... generated Core numeric list exceeds scratch capacity or native endian is unsupported
[CrashHandler] Error: Segmentation fault ...
```

Two things, in order of importance:

1. The segfault right after the deregistration is the engine's. A module that faults should be
   dropped and the game go on (as it does for an `unreachable` trap, see note 12's OOM), not
   crash the process. Reproduce: any Core WASM rules module with `scratch: 4096`, then
   `Spring.SendLuaRulesMsg(string.rep("x", 8000))` from anywhere.
2. Oversized payloads should degrade rather than fault: either the byte-list marshalling
   allocates for the callin (a `Vec<u8>` argument has no reason to go through a fixed scratch),
   or the callin is skipped with one warning naming the size and the limit. Today the limit is
   also undocumented; `export_callin_scratch!` says nothing about what needs to fit.

Worked around on the game side by raising `scratch` to 65536.

## 13.2 `SendToUnsynced` reaches the rules-unsynced module twice

One `send_to_unsynced("lab|list")` from the synced module produces two `recv_from_synced`
calls in the rules-unsynced module (`shipgame-look`), with identical bytes. Seen in every
message; logged from `dbg_lab.rs`. Harmless for idempotent messages, wrong for counters.

## 13.3 A Core WASM UI module cannot give orders

`GiveOrder`, `GiveOrderToUnit`, `GiveOrderToUnitArray` and `GiveOrderArrayToUnit` are only
exported to `rules-synced` and `gaia-synced`. LuaUI has `Spring.GiveOrder*` (a player order over
the network, as a click gives), and a native module has `GiveOrder` on the selection
(`selectedUnitsHandler.GiveCommand`). A WASM `ui` module has neither, so a game's HUD has to
send its orders to its own synced module as messages and re-issue them there. Ask: export the
unsynced player-order calls to `ui` (and `rules-unsynced`), with LuaUI's semantics.

## 13.4 AllowCommand cannot tell a player's order from the game's own

A player's move (native `GiveOrder` on the selection, the same path as a right-click) reaches a
rules-synced module's `AllowCommand` as `from_synced=true, from_lua=false, player_num=Some(0)`.
A synced WASM module's own `GiveOrderToUnit` arrives identically. Lua synced orders set
`fromLua`; WASM synced orders should set the equivalent (or pass `player_num=None`), so a game
can route only player orders.

## 13.5 Weapon-piece warning for every Core WASM-scripted unit at creation

With the engine built 2026-09-22 23:48 (uncommitted CUS work: `RefreshWeaponPieces`,
`killed_pending`), every unit whose script a Core WASM module attaches logs, once per weapon, at
frame 2: `Warning: gen_mantis: weapon1: Neither AimFromWeapon nor QueryWeapon defined or
returned invalid pieceids` (130 lines for 40 ships in `duel-mantis-shield`). The engine asks
before the module has attached its script. The build before it (`243-gc323d01`) did not warn.
Weapons work once the script attaches. Ask: don't warn until the script has had its chance
to attach, or re-check at `RefreshWeaponPieces` and warn only then.
