# Rust Intro loading screens

`CLoadScreen` loads `WasmIntro/manifest.json` in a standalone environment before the world
finishes loading. A module declares environment `intro` and exports `DrawLoadScreen` and
`LoadProgress`. When one loads, it replaces the default LuaIntro widgets; games with no Intro
module retain LuaIntro. The engine owns the draw loop and destroys Intro at the end of loading.
`DrawLoadScreen` receives the normal screen matrices, lighting and GL state, with that state
restored after dispatch. A Rust loader does not depend on Lua widgets setting it up first.

The standalone native interface includes `GameApi`. The generated availability matrix exposes
`Game.get_mod_option` to Intro, matching LuaIntro's single-option reader. The native option
readers require the received `gameSetup`, rather than the later game-world objects. This lets a
Rust loading screen choose its briefing from the current start script on a host or a client.

Intro uses the standalone Wasm feature configuration, before the game's modrules are applied.
Standalone Menu and Intro accept fixed-width SIMD Rust builds; relaxed SIMD stays off.
Generic profiling timer imports are not available in the current Core
standalone surface; animation can follow draws while progress follows `LoadProgress` messages.

Verification: regenerate and run `python3 rts/wasm/verify_codegen.py`, build `engine-legacy`,
then exercise a real game archive with a Rust Intro. Ship Game's `tools/intro_test.py` captures
the loading screen, checks current mode options and monotonic milestones, and waits for the HUD
after Intro shuts down. The dated game report holds its selected screenshots and provenance.

Implementation assistance: OpenAI Codex authored these Intro changes and the matching game
loader. Automated engine checks do not imply human visual acceptance.
