# Ship Game multiplayer engine fixes

2026-10-17 · base `b8011e0870` plus this change · AI assistance: OpenAI Codex authored the
implementation and ran the automated checks. Human play across machines is still pending.

Joining clients now regenerate an `InitBlank`/`MapSeed` map from the received setup before mounting
it. Hosts and demo playback keep their existing generation paths. The dedicated server generates
its map before calculating checksums and loading start positions; its target includes the generator
and Squish dependency.

`GAME/EnforceArchiveChecks=1` makes map and game archive mismatches fatal before simulation.
Its default is false, preserving existing launchers. The value follows the host's received script
and is copied/reset with the other `CGameSetup` state.

Live rendered verification exposed a separate shutdown crash. GDB stopped in `VBO::Delete`, from
static destruction of `NativeVBO`, after `SpringApp::Kill` had destroyed the GL context. Native
interface graphics registries now release owned buffers, VAOs, fonts, atlases and raw GL objects
in `CGlobalRendering::PreKill`, while the context is current. Repeating the same runs exits zero.

| Check | Evidence | Result |
|---|---|---|
| Linux legacy, headless and dedicated build | `docker-build-v2/build.sh --compile -j 4 linux -t engine-headless engine-legacy engine-dedicated` | pass |
| Two rendered clients, ordinary player movement, production and formation messages, different worker counts | `/tmp/ship-lan-orders-3/result.json` and per-client `sync.jsonl` | matching full Wasm state and engine checksums for frames 0–900; both exit 0; no warnings/errors |
| Dedicated host, two rendered joining clients, generated null map | `/tmp/ship-lan-dedicated-2/result.json` | matching frames 0–300; both clients exit 0; no warnings/errors |
| Deliberately changed guest archive | `/tmp/ship-lan-mismatch-1/result.json` | refused before any simulation frame |
| Shutdown localization | `/tmp/ship-lan-shutdown-gdb.log` | original crash reproduced in single-player; stack identifies native VBO static destruction |

Game harness: `ship-game-multiplayer.sdd/tools/multiplayer_check.py`. Its test-only Lua input driver
is appended to a frozen archive shared by both clients, and sends normal player commands; it never
writes synced state. Normal diagnostic logging alone requests memory hashes; it does not turn on
heavy network state hashing. Raw logs are local artifacts, not portable fixtures.

Reconnect, late spectators, replays, Windows and separate physical machines are not accepted by
this table. The game packager downloads an independently published engine pre-release and never
builds an engine. New engine binaries must be released separately before packaging these fixes.
