# Lua/Native Callback Mixup Changes

Only callback-shape fixes are listed here.

| Function | Change |
| --- | --- |
| `Spring.CallAsTeam` | Replaced `LuaFunctionRef`/`NativeLuaArgs` with `NativeCallback callback` + `void* userData`; native implementation now validates the team and invokes the native callback. |
| `Spring.SetHeightMapFunc` | Replaced Lua-shaped function args with `NativeCallback callback` + `void* userData`; native implementation wraps the callback in height-map edit tracking and calls `mapDamage->RecalcArea` for the edited rect. |
| `Spring.SetHeightMapFunc` undo recalc | Marks the height map dirty before `RecalcArea` whenever the native callback touched any valid height-map cell, so undo callbacks cannot leave normals/lighting stale. |
| `Spring.SetOriginalHeightMapFunc` | Replaced `LuaFunctionRef` with `NativeCallback callback` + `void* userData`; native implementation now sets original-height-map edit state while invoking the callback. |
| `Spring.SetSmoothMeshFunc` | Replaced `LuaFunctionRef`/`NativeLuaValue`/`NativeLuaArgs` with `NativeCallback callback` + `void* userData`; native implementation now sets smooth-mesh edit state while invoking the callback. |
| `TerrainControl.EditHeightMap` | Removed the extra native-only duplicate wrapper; `SetHeightMapFunc` is the native counterpart to the Lua API. |

Shared cleanup:

| Item | Change |
| --- | --- |
| `NativeCallback` | Added as the single generic native C callback type. |
| `LuaFunctionRef`, `NativeLuaValue`, `NativeLuaArgs` | Removed from the native C ABI headers. |
| `extract_lua_api.py` callback overrides | Replaced native-only fake types with Lua-facing callback/vararg parameter shapes. |

Verification:

| Command | Result |
| --- | --- |
| `cargo build --manifest-path rust/crates/spring-native/Cargo.toml` | Passed. |
| `python3 extract_lua_api.py && python3 extract_rust_api.py && python3 match_apis.py` | Passed; regenerated API docs/TODO. |
| `./docker-build-v2/build.sh linux` | Passed. |
