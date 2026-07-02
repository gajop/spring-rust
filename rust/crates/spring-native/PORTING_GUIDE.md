# Native API Porting Guide

## Evaluating Progress

```bash
cd /home/gajop/projects/spring-projects/spring-bar/rust/crates/spring-native

# 1. Build Rust (required before extracting API)
cd ../.. && cargo build && cd crates/spring-native

# 2. Extract Lua API from docs (uses 1-day cache)
python3 extract_lua_api.py

# 3. Extract Rust API from generated code
python3 extract_rust_api.py

# 4. Generate comparison report
python3 match_apis.py

# 5. View results
cat api_comparison.md | head -20
```

Target: 100% coverage of all exported functions (Spring.*, VFS.*, RmlUi.*, etc.).

---

## Architecture Overview

The Native API mirrors the Lua API structure:

| Lua Source File | Native Header | Native Implementation |
|-----------------|---------------|----------------------|
| LuaSyncedRead.cpp | Various (UnitsQuery.h, Features.h, etc.) | Various .cpp files |
| LuaSyncedCtrl.cpp | SyncedCtrl.h | SyncedCtrl.cpp |
| LuaSyncedMoveCtrl.cpp | MoveCtrl.h | MoveCtrl.cpp |
| LuaUnsyncedRead.cpp | UnsyncedRead.h (partial) | UnsyncedRead.cpp (partial) |
| LuaUnsyncedCtrl.cpp | **TODO: UnsyncedCtrl.h** | **TODO: UnsyncedCtrl.cpp** |

### File Locations

- C++ Headers: `rts/NativeInterface/api/*.h`
- C++ Implementations: `rts/NativeInterface/api/*.cpp`
- Rust Codegen: `rust/crates/spring-native-codegen/src/lib.rs`
- Rust Build Script: `rust/crates/spring-native/build.rs`

---

## Porting a Lua Function to Native API

### Step 1: Find the Lua Implementation

```bash
# Example: Find SetMouseCursor implementation
grep -n "SetMouseCursor" rts/Lua/LuaUnsyncedCtrl.cpp
```

### Step 2: Create Query/Result Structs in Header

Pattern in `*.h`:
```c
// Comment describing the function
struct SetMouseCursorQuery {
    const char* cursorName;
    float scale;           // use -1 for default
};
struct SetMouseCursorResult { const Error* error; bool success; };
```

Rules:
- Use `int32_t`, `uint32_t`, `float`, `bool` for primitives
- Use `const char*` for strings
- Use `const T*` + `uint32_t count` for arrays
- Use `Float3` from CommonTypes.h for 3D vectors
- All indices are 0-based (Lua uses 1-based)

### Step 3: Add to API Struct

```c
struct InputControlApi {
    // ... existing functions ...
    void (*SetMouseCursor)(const SetMouseCursorQuery* query, SetMouseCursorResult* result);
};
```

### Step 4: Implement in .cpp

Pattern:
```cpp
static void NativeSetMouseCursor(const SetMouseCursorQuery* query, SetMouseCursorResult* result)
{
    bufferPos = 0;
    result->error = nullptr;
    result->success = false;

    // Validation
    if (query->cursorName == nullptr) {
        result->error = MakeError(ERROR_INVALID_ARGUMENT, "Cursor name is null");
        return;
    }

    // Implementation (adapt from Lua version)
    mouse->ChangeCursor(query->cursorName);
    if (query->scale > 0) {
        mouse->cursorScale = query->scale;
    }

    result->success = true;
}
```

### Step 5: Register in API Struct

```cpp
static const InputControlApi INPUT_CONTROL_API = {
    // ... existing ...
    .SetMouseCursor = NativeSetMouseCursor,
};
```

### Step 6: Add Rust Codegen (if new API struct)

In `rust/crates/spring-native-codegen/src/lib.rs`, add generator function:

```rust
pub fn generate_input_control(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "InputControlApi",
            wrapper_struct: "InputControl",
        },
    )
}
```

### Step 7: Update build.rs

```rust
let sub_apis = [
    // ... existing ...
    ("input_control", spring_native_codegen::generate_input_control),
];
```

---

## Remaining Work

### Phase 1: Add Missing Rust Codegen (Quick)

APIs implemented in C++ but missing Rust bindings:

1. **COBScriptApi** in SyncedCtrl.h
   - `CallCOBScript`, `GetCOBScriptID`

2. **EffectsControlApi** in SyncedCtrl.h
   - `SpawnCEG`, `SpawnSFX`

3. **GameConfigApi** in SyncedCtrl.h
   - `SetNoPause`, `SetExperienceGrade`, `SetRadarErrorParams`, `SetSquareBuildingMask`

4. **UnsyncedReadApi** in UnsyncedRead.h
   - Unit rendering state queries (12 functions)

### Phase 2: UnsyncedCtrl.h/cpp (Large)

Create from scratch. Categories from LuaUnsyncedCtrl.cpp:

#### Camera Control (~15 functions)
- `SetCameraState`, `SetCameraTarget`, `SetCameraOffset`
- Dolly camera: `RunDollyCamera`, `PauseDollyCamera`, `ResumeDollyCamera`, `SetDollyCamera*`

#### Drawing Control (~20 functions)
- `SetDrawGround`, `SetDrawSky`, `SetDrawWater`
- `SetDrawGroundDeferred`, `SetDrawModelsDeferred`
- `SetAtmosphere`, `SetSunDirection`, `SetSunLighting`
- `SetMapShader`, `SetMapShadingTexture`, `SetSkyBoxTexture`
- `SetWaterParams`

#### Input Control (~15 functions)
- `SetMouseCursor`, `AssignMouseCursor`, `ReplaceMouseCursor`
- `SetClipboard`, `GetClipboard`
- `SetActiveCommand`
- `SDLStartTextInput`, `SDLStopTextInput`, `SDLSetTextInputRect`

#### Selection Control (~10 functions)
- `SelectUnitMap`, `DeselectUnitMap`
- `SetBoxSelectionByEngine`, `GetBoxSelectionByEngine`
- `SetBuildFacing`, `SetBuildSpacing`

#### Chat/Messages (~10 functions)
- `SendPublicChat`, `SendAllyChat`, `SendSpectatorChat`, `SendPrivateChat`
- `SendCommands`, `SendLuaMenuMsg`
- `MarkerAddPoint`, `MarkerAddLine`, `MarkerErasePosition`

#### Unit Rendering Control (~15 functions)
- `SetUnitNoDraw`, `SetUnitNoMinimap`, `SetUnitNoSelect`, `SetUnitNoGroup`
- `SetUnitEngineDrawMask`, `SetUnitAlwaysUpdateMatrix`
- `SetUnitIconDraw`, `SetUnitLeaveTracks`
- `ClearUnitsPreviousDrawFlag`

#### Feature Rendering Control (~10 functions)
- `SetFeatureNoDraw`, `SetFeatureEngineDrawMask`
- `SetFeatureAlwaysUpdateMatrix`, `SetFeatureFade`
- `ClearFeaturesPreviousDrawFlag`

#### Ground Decals (~25 functions)
- `CreateGroundDecal`, `DestroyGroundDecal`, `GetAllGroundDecals`
- `GetGroundDecal*` (15+ getters)
- `SetGroundDecal*` (10+ setters)

#### Icons (~5 functions)
- `AddUnitIcon`, `FreeUnitIcon`
- `GetIconData`, `GetAllIconDataArray`

#### System (~10 functions)
- `Quit`, `Reload`, `Restart`
- `SetWindowGeometry`, `SetWindowMaximized`, `SetWindowMinimized`
- `SetWMCaption`, `SetWMIcon`
- `SetVideoCapturingMode`

#### Misc (~10 functions)
- `GarbageCollectCtrl`, `ClearWatchDogTimer`
- `LoadCmdColorsConfig`, `LoadCtrlPanelConfig`
- `ForceLayoutUpdate`, `ForceTesselationUpdate`
- `PreloadUnitDefModel`, `PreloadFeatureDefModel`, `PreloadSoundItem`

### Phase 3: UnsyncedRead.h/cpp Expansion (Large)

Expand existing UnsyncedRead.h. Categories from LuaUnsyncedRead.cpp:

#### Camera Queries (~10 functions)
- `GetCameraRotation`, `GetCameraVectors`
- `GetFrustumPlanes`

#### Visibility Queries (~10 functions)
- `GetVisibleUnits`, `GetVisibleFeatures`, `GetVisibleProjectiles`
- `GetUnitsInScreenRectangle`, `GetFeaturesInScreenRectangle`
- `IsUnitVisible`, `IsUnitInView`, `IsUnitIcon`

#### Timer Functions (~5 functions)
- `GetTimer`, `GetTimerMicros`, `DiffTimers`
- `GetFrameTimer`, `GetDrawSeconds`

#### Game State (~10 functions)
- `GetGameState`, `GetGameName`, `IsReplay`, `GetReplayLength`
- `GetMenuName`, `GetGatherMode`

#### Input Queries (~15 functions)
- `GetKeyCode`, `GetKeySymbol`, `GetKeyBindings`
- `GetScanSymbol`, `GetKeyFromScanSymbol`
- `GetActionHotKeys`, `GetInvertQueueKey`
- `GetMouseButtonsPressed`
- `GetActiveCmdDesc`, `GetActiveCmdDescs`, `GetCmdDescIndex`

#### Display Queries (~10 functions)
- `GetDualViewGeometry`, `GetMiniMapDualScreen`, `GetMiniMapRotation`
- `GetMapDrawMode`, `GetWaterMode`
- `GetLosViewColors`

#### Selection Queries (~5 functions)
- `IsUnitSelected`, `IsUnitAllied`
- `GetGroupUnitsCount`, `GetGroupUnitsCounts`, `GetGroupUnitsSorted`

#### Rendering Queries (~10 functions)
- `GetRenderUnits`, `GetRenderFeatures`
- `GetRenderUnitsDrawFlagChanged`, `GetRenderFeaturesDrawFlagChanged`
- `GetAllProjectiles`

#### Feature Rendering State (~10 functions)
- `GetFeatureNoDraw`, `GetFeatureLuaDraw`, `GetFeatureEngineDrawMask`
- `GetFeatureDrawFlag`, `GetFeatureAlwaysUpdateMatrix`
- `GetFeatureTransformMatrix`, `GetFeatureSelectionVolumeData`
- `GetFeatureFireTime`, `GetFeatureSmokeTime`

#### Sound (~5 functions)
- `GetSoundDevices`, `GetSoundEffectParams`

#### Profiling (~5 functions)
- `GetProfilerTimeRecord`, `GetProfilerRecordNames`
- `GetLuaMemUsage`, `GetVidMemUsage`, `GetSyncedGCInfo`

---

## Testing

After implementing, rebuild and re-run comparison:

```bash
# Build C++ (from spring-bar root)
./docker-build-v2/build.sh

# Build Rust
cd rust && cargo build

# Check coverage
cd crates/spring-native
python3 extract_rust_api.py
python3 match_apis.py
cat api_comparison.md | head -20
```

---

## Notes

- Thread-local scratch buffers are used for dynamic allocations
- Error handling uses static error objects + dynamic MakeError()
- All function pointers use Query/Result pattern
- Lua 1-indexed → Native 0-indexed conversion happens in wrapper
