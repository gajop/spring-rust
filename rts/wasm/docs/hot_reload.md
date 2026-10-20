# Local WASM development reload

`/reloadwasmmodules` rereads and restarts the game's loaded UI, rules-unsynced and
gaia-unsynced guests. Select one world with `/reloadwasmmodules ui`,
`/reloadwasmmodules rules-unsynced`, or `/reloadwasmmodules gaia-unsynced`.
`all` has the same local-only meaning as omitting the argument.

The request runs from `NativeInterfaceSystem::Update`, after event callbacks return.
Each replacement comes from its original archive and source path. The engine reads
and validates all selected binaries before shutting down a working guest. Reload
cleans its RmlUi contexts and owned resources, initializes a fresh guest, and rebuilds
the dispatch subscriptions. Faulted local guests retain their last good declaration
and can also be repaired this way. An initialization failure attempts to restore the
previous binaries.

Guest memory and local UI/visual state restart; this is not state migration. The
engine does not replay GamePreload/GameStart or change simulation guests, unit
definitions, the manifest's module list, or the menu/intro runtimes. Synced and
simulation-capable `all` guests require a new match. The existing `/luaui reload`
and `/reloadnativemodules` commands retain their original behavior.

## Native verification

On a private display, launch a game with a writable `.sdd` UI module and verify:

1. Replace the UI binary with invalid bytes. Reload `ui`: validation reports the
   error and the current HUD still accepts clicks.
2. Replace it with a valid, changed binary. Reload `ui`: a fresh HUD initializes and
   works in the same match. Repeat to check resource and context teardown.
3. Request `rules-synced`: the command reports the supported local worlds and leaves
   the simulation running.

Codex authored this implementation. It was built and exercised with Ship Game in
a private native engine instance; upstream submission still needs human review.
