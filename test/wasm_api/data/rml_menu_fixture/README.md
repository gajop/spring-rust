# RmlUi menu fixture

This fixture is the smallest repository-owned Core-WASM menu package. It
contains a menu manifest, a Core-WASM module that imports `spring:rml-ui`, and
an RML document with a button, hover rule, image, and text field. The module
creates an RmlUi context, loads and shows `WasmMenu/menu.rml`, and then leaves
per-frame update/render work to the engine. The document keeps its stylesheet
and image references relative to that packaged directory. The same
`WasmMenu/menu.rml`, `menu.rcss`, and `fixture.png` are shared by the Core-Wasm
and native transports. The `native/` subdirectory is a complete Rust native
module using the repository's `spring-native` SDK; it creates the same context,
loads the same document, registers the same button callback, and queries its
context during shutdown.

Compile the module from this directory with:

```sh
wat2wasm WasmMenu/rml_menu.wat -o WasmMenu/rml_menu.wasm
```

Compile the native transport from the repository root with:

```sh
cargo build --manifest-path test/wasm_api/data/rml_menu_fixture/native/Cargo.toml --release
cp test/wasm_api/data/rml_menu_fixture/native/target/release/librml_menu_native.so \
   test/wasm_api/data/rml_menu_fixture/WasmMenu/libNativeMenu.so
```

The last copy uses the Linux name. On Windows use `NativeMenu.dll`; on macOS
use `libNativeMenu.dylib`. The engine also accepts `NativeMenu` in the
manifest's package directory and resolves the platform prefix and suffix.

Place `modinfo.lua`, `LuaMenu/`, and the resulting `WasmMenu/` directory in a
menu archive (`modtype = 5`, the existing menu archive type) and select that
archive with the normal Lua menu startup option. The fixture's minimal
`LuaMenu/main.lua` is only the host required by the engine; the RmlUi module
owns the visible menu. The fixture
deliberately uses the existing `manifest.json` path and line based `module(...)`
format used by `WasmStandaloneEnvironment`; it does not add a menu-specific
loader. A WASM-only package omits `NativeMenu`; a native-only package omits
`manifest.json`; when both are present the native module wins and the Wasm
module is not loaded.

For an isolated offscreen run from the repository root, use the engine built by
the documented Docker command and copy this directory into a temporary
`games/fixture.sdd` archive:

```sh
./docker-build-v2/build.sh linux -DUSE_ASAN=ON
fixture=test/wasm_api/data/rml_menu_fixture
run=/tmp/rml-menu-run
rm -rf "$run"
mkdir -p "$run/games/fixture.sdd" "$run/write"
cp "$fixture/modinfo.lua" "$run/games/fixture.sdd/"
cp -a "$fixture/LuaMenu" "$run/games/fixture.sdd/"
cp -a "$fixture/WasmMenu" "$run/games/fixture.sdd/"
cp -a build-amd64-linux/install/base "$run/"
cp -a build-amd64-linux/install/fonts "$run/"
xvfb-run -a build-amd64-linux/install/spring \
  --isolation --isolation-dir="$run" --write-dir="$run/write" --window \
  --menu='Rml Menu Validation 1' --hidden
```

The engine log must contain `Wasm manifest loaded from WasmMenu/manifest.json`
for a Wasm-only archive, or `Successfully loaded native module
WasmMenu/libNativeMenu.so` for a native-only archive. The visible fixture
contains a hoverable button, image, and text field; clicking the button changes
the status to `Clicked!`. Keep the run on Xvfb or another separate display.

The checked validation runs are retained under
`/tmp/rml-menu-validation-final-core` and
`/tmp/rml-menu-validation-final-native`; their logs show the selected
transport and their post-click screenshots show `Clicked!`. The fixture
verifies menu startup, packaged RML/RCSS/image lookup, rendering, input
dispatch, and native shutdown context access. The
[`transition/README.md`](transition/README.md) fixture exercises two complete
menu-to-game-to-menu cycles using a bare checked-in game and an engine-generated
map. It does not add a map archive.

The controller checks `WasmMenu/NativeMenu` first. The platform library name
is resolved using the existing native loader (`libNativeMenu.so` on Linux,
`NativeMenu.dll` on Windows, or `libNativeMenu.dylib` on macOS). If the native
library is absent, incompatible, or fails initialization, the controller falls
back to `WasmMenu/manifest.json`. When both are packaged, only the native
transport is activated, so menu callins and RmlUi input are delivered once.

Both transports own their RmlUi contexts for the menu phase. The engine
removes the menu event client and clears its contexts, data-model callbacks,
event listeners, and retained element handles before unloading the native
library or Wasm instance. A module should still close documents and release
its own handles during shutdown; returning from shutdown must leave no
callback that points into the module. Menu contexts are updated, rendered,
and receive input only while the menu is active; game-owned contexts take over
when a game starts.
