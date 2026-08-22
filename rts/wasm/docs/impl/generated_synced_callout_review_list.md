# Synced callout review list

Source: `../generated/generated_synced_callout_audit.md`.

Status: heuristic output; human decision required.

| review | rows | modules |
| --- | ---: | --- |
| `candidate` | 568 | 47 |
| `review-required` | 175 | 17 |

## `review-required`

| module | rows | decision focus |
| --- | ---: | --- |
| `unsynced_ctrl` | 82 | UI and renderer state |
| `vfs` | 32 | filesystem and archive state |
| `system_control` | 12 | process and lifecycle authority |
| `path_finder` | 9 | path cache and request state |
| `sound` | 8 | audio device state |
| `debug_input` | 7 | synthetic input |
| `tracing` | 7 | ray and renderer state |
| `config` | 4 | mutable runtime configuration |
| `camera` | 2 | camera state |
| `feature_control` | 2 | fire and smoke time |
| `features` | 2 | fire and smoke time |
| `unsynced_read` | 2 | unsynced visibility state |
| `display` | 1 | display state |
| `move_ctrl` | 1 | path result state |
| `profiling` | 1 | timer source |
| `projectile_control` | 1 | projectile state |
| `projectiles` | 1 | projectile state |
| `units_info` | 1 | self-damage timer |

## `candidate`

Candidate rows are the remaining 568 callouts across 47 modules. Review first
by mutating flag, then by module ownership and environment mask. The audit is
the row-level source; this table prevents the heuristic label from being
treated as approval.

## Decision record

- Synced-safe: human reviewer and deterministic replay evidence required.
- Unsynced-only: environment mask must exclude both synced environments.
- Process authority: keep out of production Core unless separately approved.
- File access: keep out of production Core unless separately approved.
- Timing source: keep out of synced Core unless deterministic by contract.
