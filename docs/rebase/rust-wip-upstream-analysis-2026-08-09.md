# `rust-wip` upstreaming analysis

Date: 2026-08-09

This is an analysis of the rebased `rust-wip` branch. It does not create an
upstream branch, pull request, or BAR-targeted patch series.

## Executive summary

`upstream/master` was fetched from `git@github.com:beyond-all-reason/RecoilEngine.git`
and `rust-wip` was rebased onto `a863df099b94`. The rebase replayed all 136
branch commits and completed without conflicts. The rebased tip is
`3e57d2548fda`.

The resulting branch range is very large:

- 527 files changed relative to `upstream/master`.
- 152,607 insertions and 404 deletions.
- 159 changed files under `rts/NativeInterface/`.
- 81 changed files under `rust/`.
- 176 changed files under `test/native_api_parity/`.

Those three areas account for the bulk of the branch and should not be
upstreamed as part of ordinary BAR fixes. The native interface, generated Rust
bindings, parity fixtures, API inventories, and native-callin tracing need a
separate design/review track.

There is nevertheless a meaningful set of independent engine changes hidden
among the native work. The strongest candidates for later BAR upstreaming are:

1. `d26a43db0f` — `tools/unitsync/unitsync.cpp`: destroy the Lua memory pool
   during `UnInit()`.
2. `9079a5a02f` and `ded973db30` — recoverable archive-scanner failures should
   not assert, and duplicate archive-cache entries should be tolerated.
3. `eefcf06162` and `b59ac826e7` — make test construction optional and guard
   the duplicate `tests` CMake target.
4. `d5f2070821`, `43246ffdc7`, and `d4daed8b58` — correct Lua gadget callin
   dispatch and callin documentation. `43246ffdc7` should be split because it
   contains two unrelated fixes.
5. `b8c22d02c7`, `64f771ae9d`, and selected pieces of `7e27591a4a` — small
   Lua/rendering correctness fixes that can be made self-contained.
6. The RmlUi Lua binding and lifetime fixes from `677b239b29` through
   `eef5cd1108`, plus `3b7d05fad9`, after removing the native-parity-specific
   test placement and splitting the broad WIP commit.

The initial native squash, `8741135195`, also contains independent rendering,
Lua, content, and serialization changes. They must be extracted by file/hunk;
cherry-picking that commit would bring in the native interface and is not a
safe upstreaming unit.

No upstream push or pull request has been made. The current local branch is
intentionally ahead of the stale pre-rebase `origin/rust-wip` and has not been
force-pushed.

## Validation after the rebase

Passed:

- `./docker-build-v2/build.sh linux -DUSE_ASAN=ON`
- `./docker-build-v2/build.sh --compile linux -t check`
  - 32/32 CTest tests passed.
- `cargo fmt --manifest-path rust/Cargo.toml --all --check`
- `cargo test --manifest-path rust/Cargo.toml --workspace --lib`
  - 3/3 Rust library tests passed.
- `git diff --check upstream/master...HEAD`

The full `cargo test --workspace` currently fails while compiling native
examples, not engine tests. `rust/crates/spring-native/examples/echo_module.rs`
still implements old `unit_created`, `unit_destroyed`, and `unit_finished`
signatures and passes old boolean arguments to `get_unit_position`; the mock
example still passes old boolean arguments to `get_unit_separation`. This is a
native-interface follow-up, and is deliberately not mixed into the BAR-fix
candidate list below.

The ASAN build/check is useful evidence for memory safety and compile/runtime
integrity, but it does not make the native API semantically equivalent to Lua.
Parity harness results remain a separate native-interface validation track.

## Classification method

The comparison is `upstream/master...HEAD` after the rebase. A change is
treated as an upstream candidate only if it can be expressed without the Rust
native ABI, generated bindings, native module loader, or the native parity
harness. Changes that touch both surfaces are marked for extraction rather
than treated as ready-to-cherry-pick commits.

The commit hashes below are the post-rebase hashes. Older hashes on
`origin/rust-wip` are from the pre-rebase history and should not be used for a
future cherry-pick without mapping them first.

## Candidate changes

### High-confidence, self-contained candidates

| Commit | Files | Assessment | Repro/test state |
| --- | --- | --- | --- |
| `d26a43db0f` Fix unitsync Lua memory pool cleanup | `tools/unitsync/unitsync.cpp` | A focused cleanup fix: include `LuaMemPool.h` and call `LuaMemPool::KillStatic()` from `UnInit()`. This is the cleanest candidate and should remain a separate commit for easy upstream selection. | Compiles and the ASAN `testUnitSync` CTest case passes. There is no focused repeated `Init`/`UnInit` allocator regression test. |
| `9079a5a02f` Allow blank map checksum failure in debug | `rts/System/FileSystem/ArchiveScanner.cpp` | Removes an unconditional assertion from a branch that already invalidates the cache entry and returns failure. Independent of native modules. | `test/native_api_parity/engine_bugs/ENGINE-001-blank-map-checksum-debug.md` contains a detailed debug/ASAN reproduction. It is not a normal CTest regression test yet. |
| `ded973db30` Tolerate duplicate archive cache file entries | `rts/System/FileSystem/ArchiveScanner.cpp` | Makes cache loading recover from duplicate file entries, logs the overwrite, and clears the destination map before repopulating it. Good robustness candidate; can be paired with the previous archive-scanner fix or reviewed separately. | No focused regression test. The ASAN engine check passes, but does not intentionally feed a duplicate cache. |
| `eefcf06162` cmake: allow release builds without tests | `CMakeLists.txt` | Adds the conventional `BUILD_TESTING` option around test setup. Self-contained build-system improvement. | The ASAN Docker configure/build succeeds. A dedicated configure matrix for `BUILD_TESTING=OFF` would make the upstream case complete. |
| `b59ac826e7` build: guard duplicate `tests` custom target | `test/CMakeLists.txt` | Prevents a clean configure failure when a vendored dependency has already created a `tests` target. Self-contained. | The current clean Docker configure/build succeeds. No isolated configure regression fixture. |
| `d5f2070821` Fix Lua projectile destroyed callin arguments | `cont/base/springcontent/LuaGadgets/gadgets.lua` | Forwards `ownerID` and `proWeaponDefID` to every gadget, matching the engine callin payload. A direct Lua/content fix. | Covered indirectly by the callin/parity work in this branch; there is no small standalone gadget regression test suitable for BAR yet. |
| `d4daed8b58` Correct Lua callin documentation | `rts/Lua/LuaHandle.cpp`, `rts/Lua/LuaHandleSynced.cpp` | Documentation-only corrections for `PlayerRemoved`, key labels, minimap state, and `DrawShield`. Safe and easy to review. | Signature/doc generation can validate it; no runtime test is needed, but the generated-doc diff should be attached to an upstream patch. |

### Good candidates, but need extraction or a focused test

| Commit | Independent change to extract | Assessment | Repro/test state |
| --- | --- | --- | --- |
| `f405ee9505` Demote unused GLSL attribute diagnostics | `rts/Rendering/Shaders/Shader.cpp` | Changes an expected “unused attribute” message from warning to debug. Low-risk noise reduction, but partly a logging-policy choice rather than a correctness fix. | No focused test; requires a shader/log review. |
| `b8c22d02c7` Preserve fractional Lua alpha-test references | `rts/Lua/LuaOpenGL.cpp` | Uses `luaL_checkfloat` instead of `luaL_checkint` for the alpha reference. Clear Lua API correctness fix. | No standalone test. A graphics parity case can demonstrate fractional input, but the existing parity harness is not an upstream-ready test by itself. |
| `64f771ae9d` Reject deleted Lua shader handles | `rts/Lua/LuaShaders.cpp` | Rejects a freed program slot whose index remains within the vector. Small safety fix. | No standalone CTest case; should get a Lua regression widget or a focused Lua shader test before upstreaming. |
| `94fc7876c9` Fix camera edge scrolling outside map viewport | `rts/Game/Camera.cpp` | Uses the actual window-outside state and rejects mouse Y values outside the active viewport instead of turning a lower UI/status area into bottom-edge scrolling. It is engine-generic and contains no editor-specific knowledge. | No automated camera test. Needs a deterministic manual reproduction or a headless camera-state test that distinguishes window height from map viewport height. The earlier camera/UI branch in `7e27591a4a` should not be cherry-picked as-is; `94fc7876c9` is the later narrowed version. |
| `adaa17bc45` Keep map gesture release outside RmlUi | `rts/Game/UI/MouseHandler.cpp` | Preserves ownership of a map gesture when the pointer crosses an RmlUi surface before release. This is a generic input-ownership fix, but it belongs in a small Rml/input patch rather than a native-interface patch. | No focused automated test. Manual drag/release coverage exists conceptually but should be turned into an engine input test. |
| `b932f76e2c` Preserve SDL key modifiers for synthetic key events | `rts/System/SpringApp.cpp` | Treats the modifier bits attached to a synthetic SDL key event as authoritative when the polled keyboard state lags. Potentially useful beyond native modules. | No focused test. Needs an SDL event injection test or a reproducible X11/input report. |
| `43246ffdc7` Fix LuaRules move-failed dispatch and synced debug hooks | `cont/base/springcontent/LuaGadgets/gadgets.lua`; `rts/Lua/LuaHandleSynced.cpp` | Two separate fixes: forwarding `UnitMoveFailed`, and exposing synced `debug` hooks. Split them before upstreaming. | The callin portion is exercised indirectly by the callin parity work; the debug namespace has no small direct test. |
| `7e27591a4a` Fix engine Lua and UI compatibility behavior | Multiple engine/Lua/rendering/Rml files | Not suitable as one upstream commit. Extract at least: `LuaSyncedRead::GetUnitsInPlanes` using the correct table argument; `creg` default-alignment handling; deferred FBO construction; `SetSunDirection` notifying lighting/shader listeners; the `GetVisibleFeatures` null-definition guard; and the documentation-only Lua corrections. Keep native Rml path-request wrappers out. | The ASAN engine check passes. The individual fixes lack focused regression tests; the rendering pieces need a GL/manual check. |
| `2cc086ec09`, `d8249208d2`, `ce202c536f` | `rts/build/cmake/UtilVersion.cmake`; `docker-build-v2/scripts/package.sh` | Tagless version derivation and safer packaging are potentially useful to BAR, but they encode a version/package policy. Keep the version commits together and review packaging separately. | The engine build passes, but the package script was not run as part of this audit. |
| `ae70eb48e6` | `rts/System/Platform/Win/win32.h` | `#undef MemoryBarrier` is a small Windows compile fix, but it was introduced for the native API build. Upstream only after a native-free Windows build reproduces the collision. | Linux ASAN cannot validate it; no Windows build was run locally. |

### RmlUi Lua binding and lifetime fixes

These are real engine-side fixes and should not be dismissed as “native-only,”
but the current commits mix them with a native parity harness and sometimes
with broad WIP changes. They should be upstreamed as a small, ordered series:

| Commit(s) | Files/behavior | Assessment | Repro/test state |
| --- | --- | --- | --- |
| `677b239b29`, `8f25a19d56`, `c829e16da6`, `fea8734a16`, `688cec76ad`, `777aeac1d9`, `7b8198134b` | `rts/Rml/SolLua/bind/{Global,Context,Element,ElementForm,Document}.cpp` | Correct documented Lua binding types and conversions: `RmlUi.version`, created-document userdata, document unloading, `DispatchEvent` tables, form-control values, select options, and stylesheet append. These are strong upstream candidates as separate focused fixes or a short Rml binding series. | Each of these commits has a Lua repro widget under `test/native_api_parity/rmlui_lua_bugs/` except where behavior is covered by the broader Rml checks. The widgets need relocation/adaptation to BAR’s normal Lua test convention. The CTest suite and ASAN build pass. |
| `32f7d683e0` | `rts/Rml/Backends/RmlUi_Backend.{cpp,h}`, `rts/Rml/SolLua/bind/Context.cpp` | Detaches the debugger before a context is unloaded and tracks the active debug context. Generic lifetime safety. | A dedicated Lua repro widget exists (`LUA-RML-008`); no standalone C++ regression test. |
| `8b5e5aba82` | `rts/Rml/SolLua/plugin/SolLuaDataModel.cpp` | Exposes the array `size` pseudo-field before trying ordinary table lookup. Direct binding correctness fix. | Covered by the Lua data-model checks; no isolated upstream-ready test. |
| `68160f5656`, `5f0f0ba783`, `33f93201fc` | Rml backend teardown, GL3 layer pop, SolLua element liveness, stale `SetClass`/`inner_rml`/`SetAttribute` handling; `test/engine/Rml/TestSolLuaPluginShutdown.cpp` | Important ASAN-driven lifetime fixes. `68160f5656` is explicitly a broad WIP commit and mentions the downstream editor; split it into generic Rml backend, renderer, and SolLua lifetime patches before upstreaming. | The C++ test covers plugin shutdown and element lifetime; current CTest includes it and passes under ASAN. The Lua stale-handle scenarios should be made independent of the native parity harness. |
| `eef5cd1108` | `rts/Rml/SolLua/plugin/SolLuaEventListener.cpp`; `test/engine/Rml/TestSolLuaPluginShutdown.cpp` | Prevents a listener from touching itself after a Lua callback destroys its element. Strong, generic use-after-free fix with an appropriate unit test. | The C++ test includes the self-destroying listener case and passes in the ASAN CTest run. This is close to upstream-ready after removing downstream-specific wording and confirming RmlUi version compatibility. |
| `3b7d05fad9` | `rts/Rml/SolLua/bind/Element.cpp` | Converts element collections to Lua tables and handles string-list pseudo-class arguments. Generic API behavior fix. | Covered by the parity/Rml behavior checks, but no focused upstream CTest or Lua widget is currently separated out. |

The repro `1a5074f3a7` (`LUA-RML-009-scrollbar-inner-rml-teardown.lua`) is
different: it documents an apparent vendored RmlUi `WidgetScroll` teardown
use-after-free, but this branch does not contain a corresponding fix. It should
be treated as an open RmlUi dependency issue, not submitted to BAR as a solved
engine fix.

## Independent changes hidden in `8741135195`

`8741135195` is a 381-file native-interface squash, so it must not be
cherry-picked wholesale. The following non-native hunks deserve separate
review:

| Files | Change | Recommendation |
| --- | --- | --- |
| `rts/Rendering/Env/Decals/GroundDecalHandler.cpp`; `cont/base/bitmaps/CMakeLists.txt`; the two `*_normal.bmp` files | Skip normal-map files when enumerating track decals and add the missing base normal-map assets. | Candidate as one rendering/content patch. Needs a rendering smoke test and asset review. |
| `rts/Rendering/Env/GrassDrawer.cpp`; `rts/Lua/LuaUnsyncedCtrl.cpp` | Use the current sky light direction for grass and notify lighting/shader listeners from `SetSunDirection`. | Candidate rendering correctness patch; extract together only if the dependency is confirmed. Needs GL/manual verification. |
| `rts/Lua/LuaUnsyncedRead.cpp` | Avoid dereferencing a null feature definition in `GetVisibleFeatures`. | Small crash-prevention candidate; add a feature-lifecycle regression test. |
| `rts/Lua/LuaSyncedCtrl.cpp` | Allow zero radius/height for unit/feature radius setters and add documentation changes. | Behavior change needs API review and a test proving zero is a supported value; do not extract solely because native bindings use it. |
| `rts/System/creg/creg.cpp` in the later `7f29d490ab` commit | Use ordinary `operator new` when the requested alignment does not exceed the default. | Potential correctness candidate; pair with a focused allocation/serialization test. |
| `rts/Lua/LuaZip.{cpp,h}` | Factor archive zipping into a reusable C++ result-returning helper. | The motivation is native integration, so defer unless BAR wants the internal API independently. |
| `cont/base/springcontent/gamedata/resources.lua` | Replace obsolete scar bitmap names with the available TGA resources. | Likely a standalone base-content fix, but it needs a content/package review. |
| `rts/Map/ReadMap.h`, `rts/Net/NetCommands.cpp`, `rts/Game/*`, `rts/LuaOpenGLUtils.*`, `rts/Rml/*` | Native module lifecycle, message dispatch, texture bridging, and height-map hooks. | Native-interface plumbing; explicitly exclude from ordinary BAR fixes. |

## Changes not suitable for ordinary BAR upstreaming

### Native interface and generated Rust

The following are intentionally deferred as a separate project:

- `8741135195` native-interface foundation and engine hooks.
- `NativeInterface:*`, `spring-native:*`, and the native portions of
  `Align native API behavior with Lua` / `Align native bindings with engine
  API behavior`.
- Rust code generation, bindgen output, FFI safety changes, module loading,
  VFS ownership, native Rml data models, native camera/input control, and
  native graphics resources.
- Native callin implementation and payload work, including
  `1b87db0b9c`, `7c5e67cafb`, `9a4a140fa4`, `f13fa4b558`, and the surrounding
  callin tracing commits.

These changes are valuable, but upstreaming them would require BAR to review a
new ABI, ownership model, module lifecycle, and compatibility policy. They are
not “small fixes” that can safely accompany the candidates above.

### Parity harness and generated inventories

The `test/native_api_parity/` additions, API extraction scripts, generated
function inventories, signature comparison, Lua/native callin tracing, and
large behavior fixture set are validation infrastructure for the native work.
They should not be added to BAR as part of a non-native bug fix unless a
separate, native-free subset is deliberately designed.

### Fork/build-specific changes

The CI/libclang and fork-release commits (`51cca317a5`, `61538278a5`,
`91b1a5cdef`, `1995e62fb3`, `97fd1dd00c`, `4abd48b47e`, `ca87823fa4`,
`5c1fbee088`, `52e14ab777`, `177d5e9df3`, and related commits) are tailored to
the native build and fork runner setup. They should not be upstreamed without
BAR CI-owner review.

The no-sync-check/FP-trap changes (`e5773f8162`, `09f6dd0879`, and
`f5fef1fce8`) are also a build-mode cluster, not ordinary engine fixes. Their
current commit messages and rationale are downstream-specific; they need a
separate maintainer decision before any extraction.

## Suggested future upstreaming plans

No plan below has been started.

1. **Immediate focused fixes:** upstream `d26a43db0f`, the two
   `ArchiveScanner` fixes, `eefcf06162`, and `b59ac826e7` as individually
   reviewable commits. Add small tests where practical.
2. **Lua/content correctness series:** extract the projectile callin,
   `UnitMoveFailed`, callin documentation, Lua OpenGL/shader fixes, and the
   independent pieces of `8741135195`.
3. **RmlUi maintenance series:** first relocate/adapt the Lua repros, then
   submit the binding fixes and the ASAN-backed SolLua lifetime fixes in small
   commits. Keep the unresolved scrollbar teardown issue separate.
4. **Rendering/input review:** isolate camera viewport handling, map gesture
   ownership, synthetic modifiers, FBO initialization, and sun/grass updates;
   attach deterministic or manual reproduction steps before asking BAR to
   review them.
5. **Native interface proposal:** handle the Rust ABI, module lifecycle,
   generated API, and parity harness only after BAR agrees on the architecture.

## Current repository state

The report itself is the only intended new source-controlled change from this
analysis. The working branch remains `rust-wip`, rebased locally onto
`upstream/master`; no external project was modified and no remote was pushed.
