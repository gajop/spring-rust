# Complete the owned API surface — no omissions

Date: 2026-08-22
Standalone task. Do this before any further refactor or docs work.

## The rule

**Every one of the 1354 callouts must be callable from the environment SDK in
every environment its mask permits. No omissions. No stubs. No exceptions
list.**

This is not "record why it was skipped". Skipping is not an outcome. If a shape
is hard to lower, lower it.

## Current state

The previous pass removed the `UnsupportedHostTarget` runtime sentinel, which
was correct. But it removed the *functions* rather than generating them:
`core_owned.rs` went from 1,416 to 896 `pub fn`. The failure mode improved;
the coverage went backwards.

Measured against `WasmCalloutRegistry.h`:

| | count |
| --- | ---: |
| callouts in the registry | 1,354 |
| reachable from the environment SDK | 833 |
| **missing** | **521** |

Every one of these has a working host binding and a working raw guest entry
point already. `GetTeamUnitsByDefs` carries mask `31u` — legal in all five
environments — is live in `WasmCoreUnitsQueryBorrowedBindings.cpp`, callable
via `units_query_borrowed.rs::get_team_units_by_defs_into`, and appears in no
environment module at all. Nothing here is blocked on engine work. It is
generator coverage only.

## The 521, by module

| module | missing |
| --- | ---: |
| `rml_ui` | 180 |
| `gfx` | 54 |
| `vfs` | 52 |
| `unsynced_ctrl` | 24 |
| `system_control` | 22 |
| `rules_params` | 15 |
| `units_commands` | 11 |
| `units_query` | 10 |
| `game` | 10 |
| `profiling` | 10 |
| `units_pieces` | 9 |
| `config` | 9 |
| `unit_control` | 9 |
| `input` | 8 |
| `teams` | 7 |
| `unit_defs` | 7 |
| `debug_input` | 7 |
| `units_info` | 6 |
| `projectiles` `encoding` `camera` `sound` `tracing` `unsynced_read` `unit_rendering` | 4 each |
| `features` `feature_defs` `weapon_defs` `path_finder` `selection` `messages` `terrain_control` | 3 each |
| `units_weapons` `terrain` `player` `platform` `move_ctrl` `display` `icons` `markers` `feature_control` | 2 each |
| `ground_decals` `projectile_control` `effects_control` `cob_script` | 1 each |

`rml_ui`, `gfx` and `vfs` are 286 of the 521 — over half. Start there; they
are also the three most likely to share a small number of unimplemented
shapes, so one adapter each may clear dozens of rows.

## Enforcement — do this first

Make the gap impossible to reintroduce. Add a check to
`rts/wasm/verify_codegen.py` and to `spring-api-codegen --strict`:

> for every callout in `WasmCalloutRegistry.h`, assert a callable entry exists
> in each environment module its `environmentMask` permits; fail the build
> otherwise.

Write it now, watch it fail with 521, and drive it to zero. With the gate in
place a future pass cannot quietly delete functions to make a count look clean
— which is exactly what happened this round.

Regenerating must never reduce the reachable surface.

## Work order

1. Land the gate (failing).
2. `rml_ui` (180) — largest, and likely a small number of repeated shapes.
3. `gfx` (54), `vfs` (52).
4. Everything else by descending count.
5. Rerun both synced parity contexts after each module; selected-case counts
   must rise as the surface grows.

Generator work only. Never hand-edit `rts/wasm/generated/`.

## Also in scope

**`core_environments.rs` is 207,563 lines** and duplicates full wrapper bodies
five times over — 3,272 `pub fn` against 807 `pub use`. The whole SDK went from
~105k to 311k lines for an environment split that should mostly be re-exports.

Per-environment reachable entries today: synced 751, unsynced/UI 859 (higher
because rendering callouts are legal there).

Generate environment modules as `pub use` re-exports of a single shared
implementation, with only genuinely env-specific wrappers written out. Do this
before adding 521 more functions, or the file lands near a million lines.

## Done when

- the gate passes: 1354/1354 reachable in every permitted environment
- `core_environments.rs` is re-export-based and back to a sane size
- both synced parity contexts pass with a higher selected count than 331
- `verify_codegen.py` green, workspace tests green, engine builds
