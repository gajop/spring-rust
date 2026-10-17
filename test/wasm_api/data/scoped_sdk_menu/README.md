# Same-source scoped SDK menu proof

This generic fixture builds the same Rust source and RML as a native menu and
as a Core-Wasm menu. It uses the new `spring-sdk` module factory, scoped callouts,
ordinary callin arguments, typed Rml bindings and retained callbacks.
It does not contain or migrate a downstream game/menu.

With the SDK engine built in `build-amd64-linux/install/`, run from repository
root (requires Cargo, its Wasm target, Xvfb, libclang and Python Pillow):

```sh
python3 -m pip install -r test/wasm_api/data/scoped_sdk_menu/requirements.txt
LIBCLANG_PATH=/usr/lib/x86_64-linux-gnu \
  python3 test/wasm_api/data/scoped_sdk_menu/run.py --output /tmp/scoped-sdk-proof
```

The output directory must be new so earlier evidence is preserved. An alternate
engine can be selected with `--engine /absolute/path/to/spring`.

Each transport checks all seven scalar binding types, row data, an exact UTF-8
string/f32 event, immediate reentry of the same retained listener, and typed
binding invalidation on close. It launches the existing bare transition fixture
twice using generated maps, then quits naturally. Each game loads a separate
scoped RulesSynced module and verifies initialization, a real GameFrame
callin, owned query results and shutdown. Its build declares fixed memory,
as required for synced Core-Wasm guests. Menu reloads recreate the
module; a config counter verifies three actual activations across those loads.
The start script is read from menu VFS mode `e` and must be nonempty.

`DrawScreenPost` saves `write/scoped-sdk-menu.png`. The runner rejects a missing
or blank panel/title, compares native/Wasm pixel hashes, checks logs and saved
shutdown state, and retains `result.json` for each transport. Timeout cleanup
terminates only the test's own process group. The minimal LuaMenu host reports
checkpoints and permits drawing; the SDK module owns the UI and transitions.

A successful run requires both transport results to have `ok: true`, the same
pixel hash and exit code zero. This Linux fixture does not claim native Windows
or multiplayer interoperability testing.

The negative lifecycle case deliberately panics after creating its UI:

```sh
LIBCLANG_PATH=/usr/lib/x86_64-linux-gnu \
  python3 test/wasm_api/data/scoped_sdk_menu/run.py --fail-init --output /tmp/scoped-sdk-init-failure
```

A separate Wasm probe verifies the failed module's context is absent, then
creates and removes a context with the same name. Native fallback runs the
probe after rejection; the Wasm manifest rolls back the probe after the second
module fails. Both cases must reject the failed initializer, report cleanup,
receive no activation callin and quit with exit code zero.

Use `--all-game` to load the game through the same backend as the menu, in the
`All` environment. This verifies terrain editing inside its synchronous callback,
reading the resulting height, drawing, one GameFrame invocation per frame, and
native initialization on a different loading thread from later game callins.
`LoadingMT=1` is explicit in the runner. Both game launches must pass, and the
menus must still render identical pixels and quit normally.

`--wine /path/to/wine64 --engine /path/to/spring.exe` builds the native fixture
for `x86_64-pc-windows-gnu` and runs both backends with the caller's dedicated
Wine prefix. `--all-game` checks the native Windows game too. Windows and Linux
results are compared within each platform because their font rasterizers differ.
