# WASM game API gaps found 2026-09-27

Found in Ship Game while gating the Lancer's lance on its charge. Delete items as they
are done.

## 1. Weapon numbers mean different weapons in the setters and the getters

`UnitsWeapons` reads take the public one-based weapon number
(`GetWeaponByOneBasedNumber`, `rts/NativeInterface/api/UnitsWeapons.cpp`).
Several `UnitControl` weapon setters take a zero-based index instead
(`rts/NativeInterface/api/SyncedCtrl.cpp`):

- `NativeSetUnitWeaponState`
- `NativeUnitWeaponFire`
- `NativeUnitWeaponHoldFire`

Their comment says "the Lua parity adapter converts Lua's one-based number before
reaching here". A Wasm module calling the native API directly gets no such adapter. It
also has no way to tell which convention a call uses: both take an `i32 weaponNum`.

Meanwhile, other setters in the same file keep the one-based convention
(`WeaponIndexFromOneBasedNumber`: `SetUnitTarget`, the shield state calls, ...).

What it does in practice: `set_unit_weapon_state(unit, 1, "reloadFrame", f)`
followed by `get_unit_weapon_state(unit, 1, "reloadFrame")` writes the *second* weapon and
reads the *first*. The write reports success. A unit with one weapon gets an "Invalid weapon
number" error, which every caller ignores.

Ship Game calls `set_unit_weapon_state(unit, 1, "aimReady", ...)` on every ship, meaning the
first weapon. It has been setting each ship's second weapon, or nothing, all along. Its
Lancer now subtracts one by hand in `host::weapons::hold_until`, and nothing else has been
changed.

Wanted: one convention for every weapon-number argument in the native API, preferably the
public one-based number that the reads, Lua and the docs use. The Lua parity adapter's
conversion moves into the one place that needs it. Games with workarounds (Ship Game's
`hold_until`) then drop them.
