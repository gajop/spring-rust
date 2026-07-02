# Missing native bindings

Counts are against the current engine tree.

| Surface | Lua | Native | Missing |
|---------|----:|-------:|--------:|
| `VFS.*` registered functions | 37 | 36 direct / 48 native helpers | 1* |

\* The single gap is `VFS.Include`, which is intentionally absent — it is Lua-only
by nature (see "Not portable to native" below), not an unimplemented binding.

## Not portable to native (NOT a missing binding)

- `VFS.Include`: **fundamentally Lua-only — cannot be a native binding.** Verified
  in `LuaVFS::Include` (`rts/Lua/LuaVFS.cpp:312`): it `luaL_loadbuffer`s a Lua
  source file, `lua_setfenv`s it to a caller-supplied Lua environment table, then
  `lua_pcall`s it and returns `LUA_MULTRET` arbitrary Lua values. Every step
  requires a `lua_State` and Lua chunk/environment/return-value semantics that do
  not exist in the Lua-free native interface. Same bucket as `Handle`/
  `HandleSynced` (need `GetHandle(L)`). The native equivalent is simply
  `VFS.LoadFile` + the host language executing the bytes itself; there is no
  C-ABI form of "run this Lua chunk in my env". Treat as intentionally absent, not
  as a TODO.

## Needs Separate Native Design

- `Script.*`: Lua handle/runtime introspection and callin registration controls.

## Stale Generated Docs (no local action — upstream cache)

- `RmlUi.ClearDocumentPathRequests`
- `RmlUi.GetDocumentPathRequests`

Not registered/implemented in local `rts/Rml` / `rts/Lua`. They exist only in
upstream RecoilEngine (`rts/Rml/SolLua/bind/Global.cpp`) and appear here solely
because the cached upstream `lua_api.html` (`.cache/lua_api.html`) lists them.
There is nothing in this engine tree to change — this is downstream doc-gen drift,
not native binding work.

## SBC Texture Painting: Map Square Texture Bindings — RESOLVED (2026-06-10)

Both native map-square texture bindings failed to resolve Rust's `!native*` Gfx
textures (created via `Gfx::create_texture`), because they only consulted
`CNamedTextures`. Lua's equivalents resolve through the per-handle `LuaTextures`
set, whose native analogue is `GetNativeGfxTextureInfo`. Fixed in
`rts/NativeInterface/api/VFS.cpp`, rebuilt clean (`./docker-build-v2/build.sh
--compile linux`, exit 0):

- `NativeSetMapSquareTexture` — added native-Gfx branch (Gfx first, then
  `CNamedTextures` fallback), plus `#include "Rendering/GL/myGL.h"` (it used
  `GL_TEXTURE_2D` with no GL header — the reason it had never compiled).
- `NativeGetMapSquareTexture` — added the same native-Gfx-first branch (it had
  resolved *only* through `CNamedTextures`, the source Lua's get never uses). This
  was the cause of "tests pass but live paint turns black": the scratch FBO was
  never seeded from the real map square.

### Remaining follow-up (SBC-side, not done)

Live viewport behavior is still unverified, and the Rust call site swallows the
result (`let _ = vfs.get_map_square_texture(...)` in
`TextureManager::generate_map_textures`), so a failed seed is silent. Existing
tests pre-fill the tile FBO with a known color (e.g. blue) before painting, so
they validate the Rust shader/FBO/undo pipeline but never exercise a *successful*
seed-from-engine. Recommended:

1. Assert every `get_map_square_texture` / `set_map_square_texture` result in
   `generate_map_textures` succeeds.
2. Save the generated tile immediately after `generate_map_textures` *without*
   manually filling it; assert it is not all-black/transparent.
3. Exercise a payload captured from the live editor, not only synthetic FBOs.
4. If possible, read back the displayed map-square texture after binding.

## SBC Texture Painting: Native Gfx Named Texture Resolution — RESOLVED (2026-06-10)

Confirmed the diagnosis: native `Gfx.BindTexture` / `Gfx.TextureInfo` resolved
only native Gfx textures + atlases (`GetNativeTexture` then `GetNativeAtlas`),
with no path for ordinary named/asset/engine textures. Live editor brush/pattern
names (`patternTexture`, `brushTexture.diffuse`, `$heightmap`, `$ssmf_*`,
`$detail`, plain asset paths) therefore failed to bind → the shader ran with
correct geometry but sampled an empty unit → black paint. Synthetic tests passed
because they bind `!native*` FBOs from `Gfx::create_texture`, which native Gfx can
resolve.

Decision (per engine owner): the native binding **must use the same resolver Lua
does**, regardless of whether SBC needs it. Implemented by reusing Lua's actual
resolver, `LuaOpenGLUtils::ParseTextureImage` (the same function behind
`gl.Texture` / `gl.TextureInfo`), via `LuaMatTexture`. Built clean
(`./docker-build-v2/build.sh --compile linux`, exit 0).

Changes:

- `rts/Lua/LuaOpenGLUtils.cpp` — made `ParseTextureImage` safe to call with a
  null `lua_State` (it is now reachable from the non-Lua native interface):
  - the `*` (per-handle atlas) branch returns false when `L == nullptr`;
  - the default named-texture branch derives the `PersistOnReload` /
    `SecondaryGLContext` hints only when a handle exists, else passes `false`.
  - Behavior is byte-for-byte unchanged when `L != nullptr`. The `!` (per-handle
    Lua dynamic texture) branch already returned false on null `L`. Native callers
    therefore resolve everything Lua does *except* the two per-Lua-handle dynamic
    forms, which have no meaning without a Lua state (native has its own `!native*`
    and atlas registries, which are still checked first).
- `rts/NativeInterface/api/Gfx.cpp` — `Gfx.BindTexture` and `Gfx.TextureInfo` now
  fall back to `ParseTextureImage(nullptr, matTex, name)` after their native
  registry/atlas lookups miss, then bind / report size via
  `LuaMatTexture::GetTextureTarget()` / `GetTextureID()` / `GetSize()` — mirroring
  exactly what `gl.Texture` / `gl.TextureInfo` do.

Still unverified at runtime: that live paint now renders the requested color in
the viewport (needs an in-editor check, not just a compile).

Recommended tests (still worth adding):

1. Native parity: `Gfx.TextureInfo` succeeds on a named texture that Lua
   `gl.TextureInfo` accepts.
2. Native parity: `Gfx.BindTexture` binds that named texture and a shader/readback
   samples a non-black known pixel.
3. SBC smoke test using a live-style brush/pattern asset name (not a synthetic
   native FBO), asserting an expected painted color rather than only before/after
   byte inequality.

## SBC Texture Painting: Engine Constants / Live Terrain Readback Gaps (2026-06-13)

SBC's Rust texture slice hardcoded a set of OpenGL enum values (`GL_LINEAR`,
`GL_CLAMP_TO_EDGE`, `GL_REPEAT`, `GL_QUADS`, `GL_RGBA`, `GL_COLOR_BUFFER_BIT`, ...)
because `spring-native` did not expose a constants module. These are stable GL
values, so this was never the suspected source of the live paint bug — just
brittle ergonomics. **Resolved in follow-up 1 below:** the full `GL.*` table is
now exposed natively, so SBC can use named constants from `spring-native`.

More importantly, SBC also hardcodes the SMF diffuse texture-square size as
`1024` (`TextureManager::TEXTURE_SIZE`, `TerrainChangeTextureCommand` tile-space
conversion, and tests). This matches current engine code:

- `SMFReadMap::bigTexSize = SQUARE_SIZE * bigSquareSize`
- `SMFRenderState.cpp` / `SMFVertProg.glsl` use `SMF_TEXSQUARE_SIZE = 1024`

But Rust should not rely on this being an implicit constant. It should be exposed
through a native map/SMF query, ideally together with the number of texture
squares. That would let SBC size FBOs and iterate valid map-square indices from
engine state instead of recomputing them from metal-map dimensions.

### Requested engine/native follow-ups

1. **DONE — GL constants exposed (2026-06-13).** Mirrored the full
   `rts/Lua/LuaConstGL.cpp` `GL.*` table (271 constants) as a `GLConstant` enum in
   `rts/NativeInterface/api/Constants.h`, generated from `rts/lib/glad/glad.h` so
   the values are correct by construction. Enumerators use a `GLC_` prefix to
   avoid colliding with the loader's `GL_*` macros; `api/Gfx.cpp` `static_assert`s
   every `GLC_*` against the real `GL_*` (all 271 pass — a wrong literal fails the
   engine build). Wired through bindgen (`spring-native-sys/build.rs` allowlist)
   and re-exported with clean names in `spring-native/src/constants.rs`
   (`pub const GL_LINEAR: u32 = ...`, etc). Engine build + `cargo build -p
   spring-native` both clean; bindgen emits all 271. SBC can now drop its hardcoded
   GL literals.

   **2026-06-13 follow-up — GL_RGBA gap closed.** The original mirror only
   covered the constants Lua actually registers via `PUSH_GL`; `GL_RGBA` (`0x1908`)
   and friends live in `LuaConstGL.cpp` only as commented "Not included, but
   useful" doc entries, so they were excluded. Added that documented-useful set
   (17 constants: `GL_RGB`, `GL_RGBA`, `GL_DEPTH_COMPONENT`, `GL_STENCIL_INDEX`,
   `GL_RGBA16F_ARB`, `GL_RGBA32F_ARB`, the `GL_FRAMEBUFFER*_EXT` targets and the
   FBO-status codes) — exactly what native `ReadPixels` / `CreateFBO` / `IsValidFBO`
   callers need. Same self-verifying mechanism (`static_assert` against the real
   `GL_*`; the 2 EXT-only names absent from glad core were excluded so nothing
   breaks). Total now **288** constants. Engine + `cargo build -p spring-native`
   clean; `GLConstant_GLC_RGBA = 6408 = 0x1908` confirmed in the generated
   bindings. **SBC can drop the local `0x1908` `read_pixels` literal.**

2. **DONE — SMF map-square metadata exposed (2026-06-13).** This has **no Lua
   equivalent** (there is no `numBigTex*`/`bigTexSize` Lua getter), so it is net-new
   native API — added in a manner consistent with the existing modules and marked
   `NEW` in the source. Added `VFS.GetMapSquareTextureInfo` (next to
   `Get/SetMapSquareTexture`, since it describes the same square grid):
   - `rts/NativeInterface/api/VFS.h` — `GetMapSquareTextureInfoQuery {_unused}` /
     `GetMapSquareTextureInfoResult {error, squareSize, numSquaresX, numSquaresZ}`,
     plus the `VFSApi` entry; both commented `NEW (no Lua equivalent)`.
   - `rts/NativeInterface/api/VFS.cpp` — `NativeGetMapSquareTextureInfo` reads
     `CSMFReadMap::{bigTexSize, numBigTexX, numBigTexY}` (`readMap` is always
     `CSMFReadMap`; SM3 was removed, so the `static_cast` is safe).
   - Auto-flows through the existing codegen/bindgen — no hand-written Rust:
     `vfs.get_map_square_texture_info() -> Result<(i32,i32,i32), Error>` returning
     `(squareSize, numSquaresX, numSquaresZ)`. Engine build + `cargo build -p
     spring-native` both clean. SBC can drop the hardcoded `1024` and the
     metal-map-derived square-count recomputation.

3. **OPEN — parity test.** A native/parity test for `SetMapSquareTexture` that
   validates the live terrain renderer samples the supplied texture, not just that
   an FBO holds the expected pixels.

4. **Engine analysis — no engine bug found in the shared render path.** The
   stretched-texture artifact is unlikely to originate in the engine's terrain
   renderer:
   - `SMFVertProg.glsl` builds `diffuseTexCoords` per square as a content-
     independent `0..1` range (`vertexWorldPos.xz / SMF_TEXSQR_SIZE - texSquare`),
     sampled with `CLAMP_TO_EDGE`. A supplied texture cannot cause a texcoord-scale
     "stretch" through this shared path — Lua- and native-supplied square textures
     go through the exact same `SetupBigSquare` → `SetSquareTexGen` path.
   - `CSMFGroundTextures::SetSquareLuaTexture` binds the supplied texture
     **verbatim**: it deletes the raw streamed texture, stores the supplied id, and
     never generates mipmaps or overrides filters for it. The engine sets up
     mips/`GL_TEXTURE_MAX_LEVEL`/`LINEAR_MIPMAP_LINEAR` only for its *own* streamed
     squares, at their creation. A supplied texture keeps whatever params/mip state
     it was created with.
   - Therefore the most likely cause is the supplied FBO texture's own
     parameters / mip completeness (set on the creating side), not an engine-side
     renderer assumption. Confirming this needs a live in-editor observation, which
     is out of engine scope.

## SBC Heightmap: Heightmap Dimensions Getter (2026-06-16)

SBC's heightmap slice (`native/src/sbc/heightmap/commands/{import,export,load_map}_command.rs`)
needs the heightmap grid size — the number of height points per axis — to read/write
the whole map. There is **no native getter for this**.

As an interim, the three heightmap commands now require the map size as a command
argument: Lua sends `Game.mapSize{X,Z}` and the shared `heightmap_io::grid_dims`
helper converts to point counts (`mapSize / SQUARE_SIZE + 1`). This works but means
native cannot read/write the heightmap without Lua handing it the dimensions — every
caller has to remember to pass them.

(The earlier metal-map proxy — recovering map size from `MetalMap::get_metal_map_size()`
via the metal/heightmap resolution ratio — has been removed; it was the same
anti-pattern called out for the texture square count in follow-up 2 above.)

### Resolved (2026-06-17)

Added `Terrain.GetHeightMapSize -> {pointsX, pointsZ}` as a native-only API:

- `rts/NativeInterface/api/Terrain.h` — added `GetHeightMapSizeQuery` /
  `GetHeightMapSizeResult {error, pointsX, pointsZ}` and the `TerrainApi` entry.
- `rts/NativeInterface/api/Terrain.cpp` — implemented `NativeGetHeightMapSize`
  from `mapDims.mapxp1` / `mapDims.mapyp1`, with the normal terrain `Map not
  ready` error if `readMap` is unavailable.
- Auto-flows through the existing Rust codegen as
  `terrain.get_height_map_size() -> Result<(i32, i32), Error>`.

SBC can now drop the `mapSize{X,Z}` command argument and source dimensions
natively.

## Lua `Game.*` Constants / Metadata Native Surface (2026-06-17)

Lua exposes a broad ambient `Game` table via `LuaConstGame.cpp`. Native did not
have a single equivalent surface: `GameApi` exposed some query functions, and
`Constants.h` exposed command/COB/GL constants, but scalar `Game.*` constants and
loaded map/mod metadata still required either magic values or Lua handoff.

### Resolved (2026-06-17)

Added a native split that matches the real nature of the Lua fields:

- True engine constants are now exposed as `GameConstant` in
  `rts/NativeInterface/api/Constants.h` and re-exported from Rust as
  `spring_native::constants::GAME_*`:
  - `GAME_MAX_TEAMS`, `GAME_MAX_PLAYERS`, `GAME_MAX_AIS`
  - `GAME_MAX_UNITS`, `GAME_MAX_FEATURES`, `GAME_MAX_PROJECTILES`
  - `GAME_MAX_WEAPONS_PER_UNIT`
  - `GAME_SQUARE_SIZE`, `GAME_METAL_MAP_SQUARE_SIZE`
  - `GAME_BUILD_SQUARE_SIZE`, `GAME_BUILD_GRID_RESOLUTION`,
    `GAME_FOOTPRINT_SCALE`
  - `GAME_GAME_SPEED`, `GAME_UNIT_SLOWUPDATE_RATE`,
    `GAME_TEAM_SLOWUPDATE_RATE`
- `rts/NativeInterface/api/Game.{h,cpp}` now exposes grouped scalar metadata
  getters for Lua `Game.*` fields that depend on loaded setup/map/mod state:
  - `Game.GetGameSetupInfo`
  - `Game.GetGameMapInfo`
  - `Game.GetGameModInfo`
  - `Game.GetGameRulesInfo` (including runtime `Game.maxUnits`)
- Rust codegen auto-exposes those as:
  - `game.get_game_setup_info() -> Result<sys::GameSetupInfo, Error>`
  - `game.get_game_map_info() -> Result<sys::GameMapInfo, Error>`
  - `game.get_game_mod_info() -> Result<sys::GameModInfo, Error>`
  - `game.get_game_rules_info() -> Result<sys::GameRulesInfo, Error>`

Implementation notes:

- `Constants.h` keeps literal ABI values for C-header friendliness; `Game.cpp`
  `static_assert`s them against `GlobalConstants.h` / `MetalMap.h` so drift fails
  the engine build.
- Map/mod checksum strings are returned when `archiveScanner` is available;
  otherwise the pointer is null, matching Lua's "only present when available"
  behavior without turning absence into an error.
- Lua's nested dynamic tables (`Game.springCategories`, `Game.armorTypes`,
  `Game.envDamageTypes`, `Game.collisionFlags`, `Game.speedModClasses`,
  `Game.scriptNotifyTypes`, `Game.textColorCodes`) are **not** covered by this
  scalar pass. Some are loaded-game data, not compile-time constants, and should
  be exposed through typed list/map getters if SBC needs them.

## Bug: `Spring.InvokeNativeModule` truncates Lua strings at embedded NUL — RESOLVED (2026-06-17)

The native-module entry point treats the Lua string argument as a
null-terminated C string, so any message containing an embedded `0x00` byte is
silently truncated at the first NUL before it reaches the native module.

Lua strings are length-prefixed and may contain arbitrary bytes (including NUL),
so the binding should pass the buffer together with its length (e.g.
`lua_tolstring` + an explicit `size_t len`, forwarded as `(ptr, len)` to the
module) rather than relying on NUL termination.

Observed from SBC: a `LoadMapCommand` whose JSON carried a raw heightmap blob
(little-endian `f32`, first bytes `0x00`) arrived at the Rust module as
`{...,"heightmap":"` — everything from the first NUL onward was gone — and
serde_json reported `EOF while parsing a string at line 1 column 65`.

Resolved in the native bridge:

- `LuaUnsyncedCtrl::InvokeNativeModule` and `LuaSyncedCtrl::InvokeNativeModule`
  now read the Lua string with `luaL_checklstring`, preserving the explicit Lua
  byte length even when the buffer contains embedded NULs.
- `NativeInterfaceSystem::HandleLuaCall` and
  `NativeInterfaceEventClient::HandleLuaCall` now forward `(message,
  messageLength)`.
- `HandleLuaCallQuery` now includes `messageLength`.
- Rust module entry glue now decodes `std::str::from_utf8` over
  `message[..messageLength]` instead of using `CStr::from_ptr`, so valid UTF-8
  strings containing `\0` arrive intact.

SBC no longer sends binary over this channel (it passes a file path and the
module reads the file itself), but the bridge no longer silently truncates
Lua-sent messages at a NUL.

Regression coverage added in `spring-native`: module-entry decode is factored
through `lua_call_message_to_str`, with a unit test that feeds a buffer
containing an embedded NUL and asserts the full byte sequence survives. A second
test keeps the null-pointer error path covered.

NB (separate concern, not an engine bug): even with length-delimited transport,
raw bytes are not valid JSON — control bytes `0x00–0x1F` must be escaped as
`\u00XX` (or the payload base64-encoded). A JSON command channel is text-only by
construction; binary payloads need encoding regardless of the transport fix.

## Bug: `VFS.ReadFileAsString` truncates valid text at embedded NUL — RESOLVED (2026-06-17)

Audit follow-up from the `InvokeNativeModule` NUL bug: most Native API
`const char*` fields are names, keys, paths, labels, chat text, or engine
metadata, and remain intentionally NUL-terminated text. Rust outbound wrappers
use `CString::new`, so embedded NULs in those text inputs are rejected instead
of silently truncated.

`VFS.ReadFileAsString` was the main same-family exception: the engine read the
entire file into `std::string`, but returned only `content.c_str()` to native
callers. Rust then used `CStr::from_ptr`, so a UTF-8 text file containing `\0`
would be truncated at the first NUL.

Resolved by adding `contentLength` to `ReadFileAsStringResult`, setting it from
`content.size()`, and teaching Rust codegen to decode `const char* + uint32_t`
result pairs as length-aware `Option<String>`. The public Rust method remains
`read_file_as_string(...) -> Result<Option<String>, Error>`, but internally no
longer depends on C string termination for this result.

Binary file APIs were already correct: `VFS.ReadFile`, `VFS.LoadFile`,
compression/decompression, hashing, and unpack helpers use `uint8_t* + size`.
Use those for arbitrary bytes; `ReadFileAsString` is still a text convenience.

## Bug: `Spring.SetSunDirection` does not update ground/terrain lighting — RESOLVED (2026-06-19, 2nd fix)

Changing the sun direction has (almost) no visible effect until some *other*
lighting call forces a refresh — e.g. editing any sun color via
`Spring.SetSunLighting`. In the SpringBoard map editor: drag the sun-direction
sliders → scene lighting/shadows barely move; then nudge a sun unit/ground color
→ the previously-set direction suddenly "takes."

The shared root cause was that both setters changed the light vector without
flagging the lighting system dirty, whereas `SetSunLighting` did call
`sunLighting->SetUpdated()`.

```cpp
ISky::GetSky()->GetLight()->SetLightDir(float4(dir.SafeNormalize(), intensity));
sunLighting->SetUpdated();
```

`SetLightDir` mutates the sky light, but the renderer (shader uniforms / shadow
pass) only recomputes when the lighting system is marked updated. Touching a
color flips that flag and the stale direction gets picked up — hence the
"only changes when I change sun color" symptom.

Fixed in both public paths: Lua `Spring.SetSunDirection` and native
`UnsyncedCtrl.SetSunDirection` now call `sunLighting->SetUpdated()` immediately
after `SetLightDir(...)`. `ISkyLight::Update()` already detects direction changes
for cubemap/sky work; the explicit lighting flag guarantees the dependent map
shading refresh is also requested in the same draw cycle. The intensity argument
was left unchanged because its semantics are a separate issue.

This was not a native-binding parity gap: Lua and native had the same shared
engine refresh omission, and both are now fixed. SBC does not need a workaround.

### Re-opened (2026-06-19): `SetUpdated()` fix was insufficient — wrong direction store

After the engine was rebuilt (statically-linked `spring` relinked *after* the
`UnsyncedCtrl.cpp.o` recompile — verified, the running binary has the fix), the
SBC editor still shows **no terrain/ground lighting change** when dragging the
sun-direction sliders; it only "takes" when a sun **color** is then changed.
Same symptom as before. So `sunLighting->SetUpdated()` did not resolve it.

Root cause is deeper: the sun direction lives in **two** stores, and the
ground/terrain + grass renderers read the one `SetSunDirection` never touches.

- `SetSunDirection` updates only `ISky::GetLight()->SetLightDir(...)` — i.e.
  `skyLight->lightDir`. Consumers of *that*: `ModernSky` (`ModernSky.cpp:80`),
  water (`BumpWater.cpp:437`, `DynWater.cpp:343`), cubemap — these would update.
- But the **ground/terrain SMF shader** takes its light vector from
  `mapInfo->light.sunDir`, not from `skyLight`:
  `cont/base/springcontent/shaders/GLSL/SMFVertProg.glsl:7`
  → `uniform vec4 lightDir;       // mapInfo->light.sunDir`.
- **Grass** reads it directly too: `GrassDrawer.cpp:376`
  → `grassShader->SetUniform3v("sunDir", &mapInfo->light.sunDir.x);`.

`SetSunDirection` never writes `mapInfo->light.sunDir`, so the ground/grass
shading keeps the map's original sun vector regardless of how the sliders move.
`sunLighting->SetUpdated()` only refreshes lighting **colors**, which is why
nudging a color appears to "apply" the direction — that path re-uploads ground
lighting state — but the direction uniform sourced from `mapInfo->light.sunDir`
is still stale, so what actually changes there needs confirming (it may be that
the color-update path coincidentally re-derives the ground light dir, or that
only colors move and the apparent "direction" change is the user reading the
re-lit scene).

Suggested fix direction for the engine agent:
1. Decide the single source of truth for the run-time sun direction used by
   **ground/terrain + grass** shading. Today it's the static `mapInfo->light.sunDir`.
2. Make `SetSunDirection` (both Lua `LuaUnsyncedCtrl` and native
   `NativeSetSunDirection`) update **that** store — e.g. also set
   `mapInfo->light.sunDir` (and `sunDir{X,Z}` basis if shaders use them) — and
   request the ground/SMF render-state's uniform refresh, not just
   `sunLighting->SetUpdated()`.
3. Verify in-editor (SBC Lighting editor → sun-direction sliders) that terrain
   shadows/shading move immediately, with no color change needed.

SBC side is confirmed correct and unchanged: the native command deserializes
`dirX/dirY/dirZ`, calls `set_sun_direction`, and an in-engine integration test
(`set_sun_direction`) shows `Gfx::GetSun("dir")` changes and undo restores it —
the new direction *is* reaching the engine; the ground renderer just never reads
from where it's stored.

#### Fix applied (2026-06-19)

Traced the ground/model `lightDir` uniform to `SMFRenderStateGLSL::UpdateShaderSkyUniforms()`
(`rts/Map/SMF/SMFRenderState.cpp:341`), which reads the *live* sky-light dir
(`ISky::GetSky()->GetLight()->GetLightDir()`). It runs only via
`CSMFGroundDrawer::SunChanged()` ← the `SunChanged` event — and that event was
fired only by `CSunLighting::operator=` (`SunLighting.cpp:97`), i.e. a color
change. `SetSunDirection` never fired it, so the ground/model shaders kept the
stale direction until an unrelated `SetSunLighting` triggered the event.
(`sunLighting->SetUpdated()` only gates the shading-texture refresh in
`WorldDrawer.cpp:277`, not the shader uniform.)

Fix: fire `eventHandler.SunChanged();` right after `SetLightDir(...)` in **both**
sun-direction entry points, mirroring what a color change does —
`LuaUnsyncedCtrl::SetSunDirection` and `NativeInterface` `NativeSetSunDirection`.
The shadow/shading-texture path was already covered by `ISkyLight::Update()`
returning true on a dir change; this adds the missing ground/model shader-uniform
refresh. Built clean via `./docker-build-v2/build.sh linux` (exit 0). Pending
final in-editor confirmation: drag the sun-direction sliders in the SBC Lighting
editor and check the terrain relights immediately (no color change needed).

#### Remaining grass failure fixed (2026-06-20)

The event fix refreshed SMF terrain, but grass still sourced `sunDir` directly
from immutable map metadata (`mapInfo->light.sunDir`). Grass now reads the live
`ISkyLight::GetLightDir()` instead, matching SMF terrain and the other runtime
renderers. The Lua and native setters therefore update sky/water/shadows, cached
terrain/model uniforms, and grass from one direction change; callers do not need
a second workaround.

## Native APIs exercised by the current SBC port PR — please sanity-check

The PR ports SBC editor slices to native (terrain brushes, textures/shading,
heightmap IO, map settings). The bindings below were checked against their Lua
counterparts for arguments, units, and update/refresh semantics.

**`UnsyncedCtrl`** (apply unsynced render/map-settings state):
- `set_sun_direction` *(shared Lua/native refresh bug fixed above)*
- `set_sun_lighting`
- `set_atmosphere`
- `set_water_params` — self-refreshing: like Lua, native recreates the current
  water renderer with `IWater::SetWater(currentMode)` and calls
  `waterRendering->SetUpdated()`. A follow-up `Messages::send_commands("water",
  <mode>)` is redundant and should not be required.
- `set_map_rendering_params`
- `set_map_shading_texture`

**`SyncedCtrl`**:
- `team().set_global_los`
- `terrain()` → `add_height_map`, `set_height_map_func` (synced heightmap edits)

**`Terrain`** (synced terrain edits + reads):
- `add_height_map`, `set_height_map`, `set_height_map_func` — edits are batched;
  the wrapper calls `MarkHeightMapUpdated()` and `mapDamage->RecalcArea()` for
  the touched rectangle. `RecalcArea` updates synced terrain, features, smooth
  mesh, LOS, and pathing; the read-map draw update then processes unsynced height,
  normals, and shading textures.
- `get_height_map_size` (grid points), `get_ground_height`, `get_ground_extremes` (min/max — used for heightmap export scaling)
- `add_grass`, `remove_grass`, `get_grass`

**`MetalMap`**:
- `get_metal_map_size`, `get_metal_amount`, `set_metal_amount`

**`Gfx`** (textures/shading slice + map-settings undo snapshots):
- textures: `create_texture`, `delete_texture`, `generate_mipmap`, `bind_texture`, `texture_info`
- shaders: `create_shader`, `use_shader`, `get_shader_log`, `get_uniform_location`, `uniform`, `uniform_int`, `uniform_matrix`
- render-to-texture / immediate-mode: `render_to_texture`, `begin_end`, `blending`, `color`, `vertex`, `tex_rect`, `multi_tex_coord`
- read-back for undo snapshots: `get_sun`, `get_atmosphere`, `get_water_rendering`

**`Display`**: `get_water_mode`

**`Messages`**: `send_commands`, `echo`

**`Game`** (used in integration-test assertions): `get_global_los`

**Constants** (`spring_native::constants`): `GAME_SQUARE_SIZE`; GL enums `GL_RGBA`, `GL_REPEAT`, `GL_CLAMP_TO_EDGE`, `GL_LINEAR`, `GL_LINEAR_MIPMAP_NEAREST`, `GL_QUADS`, `GL_COLOR_BUFFER_BIT`.

Sanity-check conclusions:

- Heightmap writes already perform the required terrain, normals/shading, LOS,
  and pathing refresh; no separate refresh call is needed.
- `set_water_params` already performs the same renderer recreation and dirty
  flagging as Lua; no command-based refresh is needed.
- `get_ground_extremes` directly returns `GetInitMinHeight()`,
  `GetInitMaxHeight()`, `GetCurrMinHeight()`, and `GetCurrMaxHeight()`. SBC should
  use the latter two tuple fields for current export scaling.

## Terrain generator: Spring compute backend gaps — RESOLVED (2026-06-20)

The terrain generator is a Rust library embedded by SpringBoard. It compiles a
terrain graph, executes its operators as compute shaders, and returns GPU
textures for height and material preview. Its Spring backend must execute inside
Spring's existing OpenGL context and use `spring-native`; it must not create a
second graphics device or copy preview results through CPU memory.

### Existing API that must be reused

The compute surface itself is already present. No new dispatch API or opaque
engine texture-handle type is required:

1. `NativeModule::draw_screen` provides the valid unsynced GL context. It is
   the native equivalent of SpringBoard's `delayGL`; all terrain GPU work is
   submitted from this callback.
2. `Gfx.CreateShader` already accepts a GLSL `compute` source, and the existing
   shader lifecycle, log, uniform, and `UseShader` calls apply to compute
   programs.
3. `Gfx.DispatchCompute` already accepts three workgroup counts and an optional
   post-dispatch barrier mask. `Gfx.MemoryBarrier` provides a separate barrier.
4. `Gfx.CreateTexture` returns a native texture name (`!native*`).
   `DeleteTexture`, `TextureInfo`, and `ChangeTextureParams` provide the existing
   lifecycle and metadata operations. The terrain SDK should wrap this name in
   a Rust owned handle whose `Drop` queues deletion on a later `draw_screen`;
   this ownership wrapper belongs in Rust, not in the engine ABI.
5. `Gfx.BindImageTexture` binds native textures as compute images.
6. `Gfx.BindTexture` binds sampled textures and already resolves native,
   atlas, ordinary named, asset, and engine textures through the same fallback
   used by Lua `gl.Texture`.
7. `Gfx.RenderToTexture`/FBO operations plus `Gfx.ReadPixels` provide float
   readback for explicit export. Preview textures remain on the GPU.
8. `Gfx.HasExtension` and `Gfx.GetNumber` provide capability and limit queries.

The terrain generator supplies GLSL compute implementations for its operators.
The engine is not expected to translate WGSL or create a wgpu device.

### Gap 1: `BindImageTexture` must use the shared texture resolver — RESOLVED

Implemented in `Gfx.cpp`. `BindTexture`, `TextureInfo`, and `BindImageTexture`
now share one resolver with native texture, native atlas, then Lua-compatible
named/asset/engine texture lookup order. Empty image names still unbind, unknown
non-empty names return `NOT_FOUND_ERROR`, and `unit == GL_MAX_IMAGE_UNITS` is
now correctly rejected.

`Gfx.BindImageTexture` currently calls only `GetNativeTexture`. It therefore
accepts `!native*` textures created by `Gfx.CreateTexture`, but rejects ordinary
Spring texture names such as `$heightmap` and other editor/engine-owned inputs.
This differs from `Gfx.BindTexture` and `Gfx.TextureInfo`, whose 2026-06-10 fix
falls back to `LuaOpenGLUtils::ParseTextureImage(nullptr, ...)`.

Change `Gfx.BindImageTexture` to use a shared resolver with the same lookup order
as the existing native texture APIs:

1. Resolve the native Gfx registry first.
2. Resolve native atlases where applicable.
3. Fall back to `LuaOpenGLUtils::ParseTextureImage(nullptr, ...)` for ordinary
   named, asset, and engine textures.
4. Preserve an empty/null name as image unit unbinding (`texture = 0`).
5. Return `NOT_FOUND_ERROR` for an unknown non-empty name.
6. Keep the caller-supplied mip level, layer, layered flag, access, and image
   format; these map directly to `glBindImageTexture`.
7. Reject `unit >= GL_MAX_IMAGE_UNITS`. The current comparison is `>` and
   incorrectly permits the first out-of-range unit because the queried value is
   a count.

Do not introduce a second naming scheme or expose raw GL texture IDs to Rust.
The accepted names and resolver behavior must match `BindTexture` and
`TextureInfo`. This lets SpringBoard pass an existing texture name to the terrain
SDK, while the SDK records the handle as borrowed and never deletes it.

The minimum parity test creates or selects a named engine texture which
`Gfx.TextureInfo` accepts, binds it read-only through `BindImageTexture`, runs a
compute shader which copies one known texel to a native output texture, and
checks the output. Include unknown-name, unbind, and `unit == maxUnits` error
cases.

### Gap 2: upload pixels into a native texture — RESOLVED

Added `Gfx.UploadTexture` and generated Rust `gfx.upload_texture(...)`. Uploads
are restricted to `!native*` registry textures and accept the payload as
`&[u8]`. The implementation validates target/dimensional rules, actual allocated
mip dimensions, destination bounds, scalar format/type combinations, exact
overflow-checked byte size, and null data. It supports 1D, 2D, cube faces, 2D
arrays, and 3D textures; multisample uploads are rejected.

The upload saves and restores the texture binding, pixel-unpack buffer binding,
and all affected unpack stride/skip/alignment/byte-order state. Mipmap generation
remains explicit. Cube texture creation now allocates all six faces, and the
headless GL stub covers the newly used 1D upload entry point.

Rust constants now include `GL_RED`, `GL_RG`, `GL_HALF_FLOAT`,
`GL_MAX_IMAGE_UNITS`, and all six `GL_TEXTURE_CUBE_MAP_*` face targets in
addition to the previously exposed upload types and internal formats. A Rust ABI
test verifies that all upload region fields and the `&[u8]` pointer/length pair
reach the native query unchanged.

Runtime verification completed on 2026-06-20 through the native API parity
harness and the installed renderer. The test created two `2x2` `R32F` native
textures, uploaded `[1.0, 2.0, 3.0, 4.0]`, bound them as read-only and write-only
images, dispatched a GLSL compute shader, applied an image-access barrier, and
read the destination through its FBO. The result was exactly
`[2.0, 4.0, 6.0, 8.0]`. The persistent test is
`test/native_api_parity/native/src/gfx_checks.rs` and reports
`gfx_compute_upload: pass` from `NativeModule::draw_screen`.

`Gfx.CreateTexture` allocates empty storage. `Gfx.CopyToTexture` copies from the
current framebuffer; it cannot upload decoded heightmaps, masks, or material
assets held by Rust. Add a native-only `Gfx.UploadTexture` operation next to the
other texture functions.

Proposed ABI, following the existing query/result and pointer-plus-size
conventions:

```cpp
struct GfxUploadTextureQuery {
	const char* name;
	uint32_t target;
	int32_t level;
	int32_t xoff;
	int32_t yoff;
	int32_t zoff;
	int32_t width;
	int32_t height;
	int32_t depth;
	uint32_t format;
	uint32_t pixelType;
	const uint8_t* data;
	uint32_t dataSize;
};

struct GfxApi {
	// Existing entries remain in their current order.
	void (*UploadTexture)(
		const GfxUploadTextureQuery* query,
		GfxEmptyResult* result
	);
};
```

Field semantics:

- `name` is an owned native Gfx texture name returned by `CreateTexture`.
  Upload must not mutate an arbitrary engine-owned or asset texture.
- `target` follows `CopyToTexture`: `0` uses the stored texture target. A
  cube-map upload supplies one of the six `GL_TEXTURE_CUBE_MAP_*` face targets.
- `level` is the destination mip level.
- offsets and extents select the destination region. For 1D textures, height
  and depth are `1`; for 2D and cube faces, depth is `1`; for 3D and array
  textures, `zoff` and `depth` select slices.
- `format` is the incoming pixel layout (`GL_RED`, `GL_RG`, `GL_RGB`, or
  `GL_RGBA`), not the texture's internal format.
- `pixelType` is the incoming component type, initially the uncompressed scalar
  types accepted by `glTexSubImage*`, including `GL_UNSIGNED_BYTE`,
  `GL_UNSIGNED_SHORT`, and `GL_FLOAT`.
- `data`/`dataSize` is a tightly packed byte slice. Rows have no caller-provided
  padding. The engine sets unpack alignment to `1` for the call and restores the
  previous pixel-store state afterward.

The implementation must:

1. Resolve only `GetNativeTexture(name)` and return `NOT_FOUND_ERROR` otherwise.
2. Validate non-negative offsets, positive extents, mip bounds, destination
   bounds, supported target, format/type, non-null data for non-zero size, and
   the exact required byte count. All size arithmetic must be overflow-checked.
3. Select `glTexSubImage1D`, `glTexSubImage2D`, or `glTexSubImage3D` from the
   stored texture target. For a cube map, require `target` to be one of its six
   face targets and call `glTexSubImage2D` for that face. For every other target,
   require `target == 0` or equality with the stored target.
4. Save and restore the affected texture binding and pixel-unpack state. It must
   not leak GL state into Spring's renderer.
5. Clear/check GL errors using the same helpers as `CreateTexture`, and return a
   normal native `Error` instead of panicking or silently accepting a partial
   upload.
6. Leave mip generation explicit through the existing `GenerateMipmap` call.

The generated Rust API should expose the payload as `&[u8]`, using the same
`const uint8_t* data` + `uint32_t dataSize` pairing already used by VFS. A
representative signature is:

```rust
gfx.upload_texture(
    name,
    target,
    level,
    x_offset,
    y_offset,
    z_offset,
    width,
    height,
    depth,
    format,
    pixel_type,
    data,
) -> Result<(), Error>
```

Tests must cover `R32F` height data, `RGBA8` material data, a non-zero subregion
upload, undersized/oversized payload rejection, out-of-bounds regions, wrong
texture ownership, and GL-state restoration. Verify uploaded contents by
sampling or image-loading them in a compute shader and reading a separate output
texture; this tests the actual terrain-backend path.

### Required Spring backend flow after these fixes

1. SpringBoard queues terrain work and calls the SDK from `draw_screen`.
2. The SDK treats Spring-provided texture names as borrowed handles.
3. The SDK creates intermediate and output textures with `CreateTexture`; those
   names are owned by the SDK and deleted through `DeleteTexture` in a later GL
   callback.
4. File-backed inputs are decoded by Rust and transferred with
   `UploadTexture`.
5. Operators compile through `CreateShader`, bind sampled inputs with
   `BindTexture` or storage images with `BindImageTexture`, set uniforms,
   dispatch through `DispatchCompute`, and use its barrier mask or
   `MemoryBarrier` between dependent passes.
6. The SDK returns borrowed views of its output texture names to SpringBoard for
   preview. It retains ownership until the preview is replaced or the generator
   is dropped.
7. Export uses the existing FBO/readback path and is explicitly requested; live
   preview performs no CPU readback.

These two engine changes are sufficient for the planned 2D texture-based
terrain graph. SSBOs, indirect dispatch, persistent mapped buffers, a second
resource-handle system, and a new compute scheduler are not prerequisites.

## Objects slice: unit `crashing` getter — RESOLVED (2026-06-20)

SBC's objects slice serializes a unit's `crashing` flag. The **set** side was
already bound (`SyncedCtrl` `set_unit_crashing` ↔ `Spring.SetUnitCrashing`), but
there was **no read path**, so the native unit serializer left `crashing` unset.
Lua reads it via `Spring.GetUnitMoveTypeData(id).aircraftState == "crashing"`
(aircraft-only), but the native `MoveTypeData` struct exposes only kinematics —
it has no `aircraftState` field.

Added a direct getter `UnitsInfo.GetUnitCrashing` (next to `GetUnitNeutral`),
mirroring the aircraft check in `NativeSetUnitCrashing`:

- `rts/NativeInterface/api/UnitsInfo.h` — `GetUnitCrashingQuery {unitID}` /
  `GetUnitCrashingResult {error, isAircraft, crashing}`, with the function pointer
  **appended last** in `UnitsInfoApi` (ABI-safe; offsets only grow, host+module
  rebuild together).
- `rts/NativeInterface/api/UnitsInfo.cpp` — `NativeGetUnitCrashing` resolves
  `dynamic_cast<AAirMoveType*>(unit->moveType)`; non-aircraft return
  `isAircraft=false` (no error), aircraft return
  `crashing = (aircraftState == AIRCRAFT_CRASHING)`. Added the
  `Sim/MoveTypes/AAirMoveType.h` include.

Auto-flows through codegen: `units_info().get_unit_crashing(unit_id) ->
Result<(bool, bool), Error>` returning `(isAircraft, crashing)`. SBC maps
non-aircraft to `None` (matching Lua's nil), so `crashing` is serialized only for
aircraft. Engine rebuilt clean (`./docker-build-v2/build.sh --compile linux`,
exit 0); `cargo build -p spring-native` regenerates the binding.

## Added: `UnsyncedCtrl.SetWaterTexture` binding — DONE (2026-06-20)

The value-typed `WaterParams` struct (passed by value to `SetWaterParams`) cannot
carry the three water *texture* path strings that Lua `Spring.SetWaterParams`
accepts via its `LUA_TSTRING` keys (`texture`, `foamTexture`, `normalTexture`) —
so SBC's native water command silently dropped them (the editor's water texture
was stuck on the map default). The native API already separates string-bearing
texture setters from param structs (`SetMapShadingTexture`, `SetSkyBoxTexture`),
so a dedicated binding fits that convention better than stuffing `const char*`
into `WaterParams`.

Added (mirrors `SetMapShadingTexture`):
- `rts/NativeInterface/api/UnsyncedCtrl.h` — `SetWaterTextureQuery {texType, texName}`
  / `SetWaterTextureResult {error, success}`, and the `SetWaterTexture` function
  pointer appended at the **end** of the API vtable (ABI-safe; no version bump
  needed since host+module rebuild together and the offset only grows).
- `rts/NativeInterface/api/UnsyncedCtrl.cpp` — `NativeSetWaterTexture` switches on
  `texType` and assigns `waterRendering->{texture,foamTexture,normalTexture}` (the
  exact Lua `LUA_TSTRING` behavior); registered as the last vtable entry (C++
  designated-initializer order).

Also added the symmetric getter `GetWaterTexture(texType) -> texName` (mirrors
`gl.GetWaterRendering`'s string keys: `GetWaterTextureResult { const Error* error;
const char* texName; }`, returning `waterRendering->{...}.c_str()`), so the command
can snapshot the current path for undo.

Both auto-flow through codegen (no hand-written Rust):
`unsynced_ctrl().set_water_texture(tex_type, tex_name) -> Result<bool, Error>` and
`get_water_texture(tex_type) -> Result<Option<String>, Error>`. SBC's
`SetWaterParamsCommand` applies `texture`/`foamTexture`/`normalTexture` via the
setter (its existing `SendCommands("water <mode>")` reload makes them visible) and
snapshots/restores via the getter — so a texture change **is undoable**. Engine
built clean (`./docker-build-v2/build.sh linux`, exit 0).

Codegen fix needed along the way: `GetWaterTexture` is the first `const char*`
result in the `unsynced_ctrl` module, whose host wrapper
(`crates/spring-native/src/unsynced_ctrl.rs`) didn't `use std::ffi::CStr` (other
modules with string getters already do). Added that import — the generated code
emits a bare `CStr::from_ptr`, resolved by the host module's `use`.
