# Native API Parity Fix Index

This file indexes the fix commits found while rebasing and testing the native
API/RmlUi work. It is intentionally separate from the fix commits themselves so
it can be updated as a tracking document without changing the standalone bug
fixes.

Line counts use `git show --numstat`. "Total changed" is additions plus
deletions.

## Existing Engine Fixes

| Commit | Fix | Report / repro | Files | Lines |
| --- | --- | --- | --- | ---: |
| `57fd617cea` | Allow generated blank-map archive checksum failure to return normally in debug/ASAN instead of asserting. | `test/native_api_parity/engine_bugs/ENGINE-001-blank-map-checksum-debug.md` | `rts/System/FileSystem/ArchiveScanner.cpp`<br>`test/native_api_parity/engine_bugs/ENGINE-001-blank-map-checksum-debug.md` | +49 / -1, total changed 50 |

## Lua RmlUi Engine Fixes

| Commit | Fix | Report / repro | Files | Lines |
| --- | --- | --- | --- | ---: |
| `1e89f6e645` | `Document:AppendToStyleSheet` works on documents without an existing stylesheet. | `test/native_api_parity/rmlui_lua_bugs/LUA-RML-007-append-stylesheet-empty-document.lua` | `rts/Rml/SolLua/bind/Document.cpp`<br>`test/native_api_parity/rmlui_lua_bugs/LUA-RML-007-append-stylesheet-empty-document.lua` | +62 / -2, total changed 64 |
| `3f56f76d4c` | `RmlUi.version` is exposed as a Lua string value. | `test/native_api_parity/rmlui_lua_bugs/LUA-RML-001-version-field.lua` | `rts/Rml/SolLua/bind/Global.cpp`<br>`test/native_api_parity/rmlui_lua_bugs/LUA-RML-001-version-field.lua` | +24 / -1, total changed 25 |
| `9273ac9bcb` | `Context:CreateDocument` returns Lua document userdata, not raw base document userdata. | `test/native_api_parity/rmlui_lua_bugs/LUA-RML-002-create-document-userdata.lua` | `rts/Rml/SolLua/bind/Context.cpp`<br>`test/native_api_parity/rmlui_lua_bugs/LUA-RML-002-create-document-userdata.lua` | +49 / -4, total changed 53 |
| `d10cb2aa83` | `Context:UnloadDocument` accepts Lua-created document userdata. | `test/native_api_parity/rmlui_lua_bugs/LUA-RML-003-unload-created-document.lua` | `rts/Rml/SolLua/bind/Context.cpp`<br>`test/native_api_parity/rmlui_lua_bugs/LUA-RML-003-unload-created-document.lua` | +44 / -1, total changed 45 |
| `436dfb587c` | `Element:DispatchEvent` accepts table parameters and passes them to event listeners. | `test/native_api_parity/rmlui_lua_bugs/LUA-RML-004-dispatch-event-table.lua` | `rts/Rml/SolLua/bind/Element.cpp`<br>`test/native_api_parity/rmlui_lua_bugs/LUA-RML-004-dispatch-event-table.lua` | +112 / -1, total changed 113 |
| `bed9951822` | `Element:GetValue` returns form-control values, including select values. | `test/native_api_parity/rmlui_lua_bugs/LUA-RML-005-select-get-value.lua` | `rts/Rml/SolLua/bind/Element.cpp`<br>`test/native_api_parity/rmlui_lua_bugs/LUA-RML-005-select-get-value.lua` | +54 / -5, total changed 59 |
| `3ec77d93ed` | Select option proxy exposes the option value under the documented `value` field. | `test/native_api_parity/rmlui_lua_bugs/LUA-RML-006-select-options-field.lua` | `rts/Rml/SolLua/bind/ElementForm.cpp`<br>`test/native_api_parity/rmlui_lua_bugs/LUA-RML-006-select-options-field.lua` | +55 / -1, total changed 56 |
| `46983cca9c` | Removing or unloading the active RmlUi debug context detaches the debugger first. | `test/native_api_parity/rmlui_lua_bugs/LUA-RML-008-debug-context-removal.lua` | `rts/NativeInterface/api/RmlUi.cpp`<br>`rts/Rml/Backends/RmlUi_Backend.cpp`<br>`rts/Rml/Backends/RmlUi_Backend.h`<br>`rts/Rml/SolLua/bind/Context.cpp`<br>`test/native_api_parity/fixtures/game.sdd/LuaUI/main.lua`<br>`test/native_api_parity/rmlui_lua_bugs/LUA-RML-008-debug-context-removal.lua` | +76 / -1, total changed 77 |
| `197a195ca6` | Lua table-backed data-model arrays expose the reserved `.size` child used by RmlUi expressions. | `test/native_api_parity/rmlui_lua_bugs/LUA-RML-010-data-model-array-size.lua` | `rts/Rml/SolLua/plugin/SolLuaDataModel.cpp`<br>`test/native_api_parity/fixtures/game.sdd/LuaUI/main.lua`<br>`test/native_api_parity/rmlui_lua_bugs/LUA-RML-010-data-model-array-size.lua` | +189 / -0, total changed 189 |

## Native Binding Fixes

These are native/Rust binding fixes, so they do not have the separate Lua engine
bug report requirement.

| Commit | Fix | Files | Lines |
| --- | --- | --- | ---: |
| `e140e36e88` | Native RmlUi stylesheet append behavior matches the Lua fix. | `rts/NativeInterface/api/RmlUi.cpp`<br>`rust/crates/spring-native/PORTING_TODO.txt`<br>`test/native_api_parity/native/src/lib.rs`<br>`test/native_api_parity/native/src/rml_checks.rs` | +147 / -28, total changed 175 |
| `08eb5cc6d2` | Native binding/codegen behavior for invalid/error results is corrected. | `rts/NativeInterface/api/RmlUi.cpp`<br>`rust/crates/spring-native-codegen/src/lib.rs`<br>`rust/crates/spring-native/src/error.rs` | +42 / -10, total changed 52 |
| `24db236b69` | Native RmlUi data-model handles are invalidated when the model or owning context is removed. | `rts/NativeInterface/api/RmlUi.cpp`<br>`test/native_api_parity/native/src/rml_checks.rs` | +44 / -3, total changed 47 |

## RmlUi Submodule Fix

The parent repository points `rts/lib/RmlUi` at `e4eb761677`, advancing it from
`2230d1a6e8`. The submodule commit is:

| Submodule commit | Fix | Report / repro | Files | Lines |
| --- | --- | --- | --- | ---: |
| `e4eb7616` | Fix heap-use-after-free/use-after-poison in `WidgetScroll` destructor. | `test/native_api_parity/rmlui_lua_bugs/LUA-RML-009-scrollbar-inner-rml-teardown.lua` | `rts/lib/RmlUi/Source/Core/WidgetScroll.cpp` | +9 / -0, total changed 9 |

Attribution check:

- Vanilla vendored RmlUi 6.2 (`2230d1a6e8`) crashes under ASAN when a Lua
  widget creates a real scroll container, verifies it can scroll, then replaces
  the document `inner_rml`.
- The crash path is `Element::SetInnerRML -> Element::~Element ->
  ElementScroll::~ElementScroll -> WidgetScroll::~WidgetScroll ->
  EventDispatcher::DetachEvent`.
- The same Lua canary passes after advancing the submodule to `e4eb761677`.

This is Lua-visible from this engine, but the stale listener removal is in the
vendored RmlUi scrollbar implementation. It should be treated as a
submodule/upstream fix candidate; a standalone upstream-facing RmlUi sample
would still be useful before submitting outside this branch.

The Lua parity canary and standalone repro were added separately in
`945d9c47cb` (`Add RmlUi scrollbar teardown repro`) so the reproduction remains
reviewable independently of the fix index.
