# Headless icon draw buffers, 2026-10-19

Historical experiment record. The selected source changes are now recorded in local
commits; see [the session integration report](ship_game_session_2026-10-20.md) for
the final scope, exclusions and installation status. Statements below describe the
original experiment at capture time.

Codex authored this local implementation and report. All engine changes remain uncommitted and unpushed, unreviewed for upstream, and not verified by a human player.

## Scope

Two `HEADLESS` early returns in `CUnitDrawerGLSL::DrawUnitMiniMapIcons` and `CUnitDrawerGLSL::DrawUnitIconsScreen` omit draw-only icon buffer population. Both functions otherwise traverse units and ghosts, compute screen/minimap positions, append four vertices and six indices per icon, and submit CPU buffers even when GL calls are headless stubs. No common or interactive playing-speed improvement is claimed.

The implementation adds eight lines to `rts/Rendering/Units/UnitDrawer.cpp`. It retains the existing profiling zone at each function entry. No other engine source is changed by this incremental patch.

## Contract audit

- Both skipped functions only read unit, ghost, camera, icon and minimap state; their writes affect local variables, render buffers and GL state. They contain no event callbacks or unit-state writes. Headless render-buffer element/submission statistics naturally no longer include these omitted quads.
- `CUnitDrawerData::Update` still updates interpolated draw positions, icon indices, screen-icon status, picking radii, and unit draw flags. `IsUnitIcon`, unit visibility, unit view positions, screen-rectangle queries and selection-box projection retain their existing inputs and implementations.
- `CGame::Draw`, its unsynced update, RmlGui update, native/Lua draw callins, minimap update, recording, profiling controls and surrounding event dispatch remain intact. The existing headless Wasm Draw-callin policy is unchanged. In particular, `DrawInMiniMapBackground` and `DrawInMiniMap` remain outside the skipped minimap-icon function.
- Legacy `DrawUnitIcons` is unchanged: it writes `unit->iconRadius`, which `TraceRay::GuiTraceRay` uses for picking. No broad world-draw or draw-loop skip is introduced.
- The interactive build does not compile either return, so the original arithmetic, buffer population, draw order and callbacks remain available without a runtime branch.

## Source and application boundary

Private worktree: `/home/gajop/.codex/worktrees/opt-headless-client/spring-bar`, branch `codex/opt-headless-client`, HEAD `6cedf0cde7f9e6f542bfb80ad7c6935fe5ff77b9`.

With explicit coordinator authorization, the same focused two-hunk patch was applied to `/home/gajop/projects/spring-projects/spring-bar/rts/Rendering/Units/UnitDrawer.cpp`. Before application, the original file, complete tracked diff and status were saved in the private evidence directory. `git apply --check` passed; both HEADs remained unchanged, both resulting files are byte-identical, and the original checkout's other tracked diffs remained byte-identical. The original build/install and binaries were not touched.

Evidence directory: `build-headless-perf/interactive-common/headless-icons-2026-10-19`. Patch `headless-icons.patch` SHA256: `5e71ee5ae7bd722928b0f94587d0d06764588c723bf25405d36b1741848e20b6`. `before.json` and `application.json` record application identities. `before/` preserves independent references to the previously tested common candidate binaries; newly staged binaries replace files with new inodes.

The previously preserved old headless control (`57b9dd6f…`), common headless control (`976db20c…`) and interactive common control (`d082bb23…`) remain distinct. The two common controls and their original source patch are preserved in this evidence directory's `before/`; `interactive-common/candidate` now holds the new binaries. Historical paths in the common-projection report describe the earlier checks. Game-only comparisons using the old headless control do not measure this incremental engine change.

## Verification

The private incremental build held the shared profiling lock and used one compiler job. Four actions compiled `UnitDrawer.cpp` for the interactive/headless engines and linked both successfully, with the previously retained CMake configuration and build image. Original populated vendor directories and ccache were read-only mounts. The original-engine build/install and binaries were untouched.

- Interactive `.text`, `.rodata`, `.data.rel.ro` and `.data` are individually byte-identical to the preserved common interactive control. The full ELF hashes differ, so the section comparisons are recorded explicitly rather than claiming identical binary files. This confirms that the guards do not change the built interactive executable code or those data sections.
- Both headless methods are one-byte functions containing `ret`; the linker/compiler shares their address. The saved disassembly addresses this alias explicitly. The per-unit projection and quad-emplacement loops cannot execute through either method in the headless candidate.
- The existing projection Catch test passed again: **46,702 assertions in 3 test cases**. No arithmetic or API implementation changed in this incremental patch. The focused diff passes `git diff --check` in both source trees.
- Source inspection retains the callback and read-state boundaries listed above. No new native game, replay, visual or performance run was made by this lane; the coordinator owns those serial integration comparisons. Earlier common-projection interactive/read-API fixtures remain applicable to the byte-identical executable sections, within their documented fixture limits.

Staged paths under `build-headless-perf/interactive-common/candidate`:

| File | SHA256 |
|---|---|
| `spring-headless` | `561412ab61e3c4f7fcfeaad3fb751da28133da930f00daea0168991df673b61e` |
| `spring` | `90462f227b8a212376bad0ec9059d67eee9cbeca2e9aaafa04819bfafce1433d` |

`compiled-verification.json`, `focused-test.log`, the two `.asm` files, `build.log`, `candidate-CMakeCache.txt` and `handoff-manifest.json` retain the checks and identities. `full-source.patch` includes the existing approved prerequisites, earlier headless/common changes, and these two guards; SHA256 `aed57b3e921a25a6dde34929f6b441609bab1ba73534746acc2215775895c907`. It is distinct from the focused two-hunk patch used on the original checkout.

No additional performance run has been made for this incremental patch. Headless client-loop cadence can change when a draw path becomes cheaper, so reduction in these functions must not be presented as a simulation-stage speedup or an interactive rendering gain.
