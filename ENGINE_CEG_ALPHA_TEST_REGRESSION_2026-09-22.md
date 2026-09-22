# Confirmed: legacy unit rendering clips Flove's CEGs

## Evidence

Reproduced in standalone **Lua Flove**, past the intro and in gameplay, on NVIDIA GTX 1660 Ti. No WASM menu was involved. Tests used an isolated game copy and configuration; production game files and engine source were not changed.

- Current executable: `build-amd64-linux/install/spring`, `2026.07.01-243-gc323d01`.
- Earlier executable: `../springcabal-games/spring-linux/spring`, `2026.07.01-240-g7308076`.
- On the current executable, immediately before particles: `GL_ALPHA_TEST=1`, `GL_ALPHA_TEST_FUNC=516` (`GL_GREATER`), reference `0.5`.
- On an identical paused frame, changing SoftParticles from 1 to 0 changed **zero pixels**. A diagnostic `gl.AlphaTest(false)` immediately before particles restored the translucent pink/white glows, changing 35,170 pixels.
- On the earlier executable, alpha testing was disabled and the glows were visible. Switching to `/unitdrawer 0` on the **same paused frame** enabled alpha testing at `>0.5` and removed the glows. This reproduces the defect without upgrading the executable or changing CEG definitions.

Artifacts are under `../springcabal-games/verification/out/`:

- `flove-ceg-gpu-alpha-test/last.stdout.txt` and `screens/`: current-engine alpha-state intervention.
- `flove-ceg-gpu-old-forced-legacy/screens/screen_2026-09-22_13-40-56-031.png`: visible glows.
- `flove-ceg-gpu-old-forced-legacy/screens/screen_2026-09-22_13-41-02-914.png`: same paused scene after forcing legacy; glows missing.
- `flove-ceg-callback-routing/last.stdout.txt`: state readback and material callback counts.

## Cause

`rts/Rendering/Common/ModelDrawer.h`, `DrawImpl<legacy>` (around line 317), enables `GL_ALPHA_TEST` without restoring it on exit. The legacy rendering path leaves the threshold at `GL_GREATER, 0.5`.

`rts/Rendering/Env/Particles/ProjectileDrawer.cpp`, `DrawAlpha` (around line 798), establishes blending/depth state but not alpha-test state. The inherited fixed-function test discards the translucent particle fragments.

Commit `872b0df821` added global `ForceLegacyPath()` calls to Lua and native `SetUnitLuaDraw` (`rts/Lua/LuaObjectRendering.cpp:674`, `rts/NativeInterface/api/UnsyncedCtrl.cpp:264`). This exposes the pre-existing legacy state leak in Flove. Both game versions use the Lua custom-material gadget that invokes this API.

## Recommended engine changes

1. **Fix the demonstrated state leak.** Restore alpha-test state at the legacy drawing boundary. Explicitly disable fixed-function alpha testing in the particle pass's scoped GL state (`AlphaTest(GL_FALSE)`), so particle rendering does not depend on the preceding renderer. This is the targeted CEG fix; no game-side alpha override or CEG edits are appropriate.
2. **Narrow the global fallback separately.** Existing custom-material rendering already uses a scoped legacy drawer (`ModelDrawer.h`, `DrawOpaquePassImpl`). Runtime tracing on the earlier executable confirms repeated `DrawUnit` callbacks for Flove's spire and trees while the main renderer remains modern. These units do not require a global switch. For ordinary non-material `luaDraw` units, however, the GL4 opaque batch currently submits models without that callback (`rts/Rendering/Units/UnitDrawer.cpp`, `CUnitDrawerGL4::DrawOpaqueObjects`). Replace the global switch with a scoped fallback for those units, preserving callback suppression and matrix transformations. A bare revert is not a complete callback fix.

Validate CEGs with both normal and explicitly forced legacy rendering; preserve existing material callbacks and test non-material callbacks returning both true and false through Lua/native APIs. The proposed engine changes have **not** been implemented or compiled here. This finding does not establish the cause of mouse-input or skeletal-animation issues.

## Reproduce

From `springcabal-games`, using the retained isolated test fixture:

```sh
PROBE_GPU=1 PROBE_FORCE_LEGACY=1 \
SPRING_ENGINE_BIN="$PWD/spring-linux/spring" \
bash verification/games/flove-ceg-probe.sh engine-ceg-repro
```

The fixture enters gameplay, triggers zap/upgrade commands, pauses, captures the particle-state comparisons, forces legacy rendering, and exits. It does not exercise physical mouse input.
