# Lua / Native API Surface Contract

This file records API differences that are intentional. A surface must not be
listed here merely because the name matcher failed to find a counterpart. Any
unclassified difference remains an implementation or documentation gap until
the engine and native implementation have been compared.

## Directional surfaces

Lua callouts (`Lua → engine`) and native module calls (`native → engine`) are
separate interfaces. Native callback exports (`engine → native`) are also a
separate interface from Lua callins (`engine → Lua`). Matching one direction
does not prove parity in another direction.

The native module lifecycle and callback entry points are intentionally native
ABI symbols rather than Lua functions:

- `InitializeNativeModule`
- `Shutdown`
- the native callback exports for engine events

They are checked against the C++ event client and Rust trait/export inventory,
not counted as missing Lua functions.

## Confirmed Lua-only surfaces

### Global Lua helpers

`CallAsTeam` is installed directly in the Lua global table (`_G.CallAsTeam`)
by `LuaHandleSynced.cpp`; it is not `Spring.CallAsTeam`. The native
`SystemControl.call_as_team` operation is the corresponding capability, with
an intentional ABI boundary: Lua invokes an arbitrary Lua function and
forwards its complete return stack, while native accepts a typed callback and
returns a success flag. The parity fixture compares callback execution, team
selection, return count, and the shared marker/flag result shape. The source
inventory also records the engine-installed `loadstring`, `pairs`, `next`, and
`SendToUnsynced` helpers as runtime/callin boundaries; they are not missing
native callouts.

### `VFS.Include`

`VFS.Include` loads and executes an arbitrary Lua chunk in a selected Lua
environment and returns arbitrary Lua values. There is no meaningful typed
native equivalent. It must still be tested as a Lua API (registration,
argument behavior, return behavior, and error behavior), but it is excluded
from Lua/native result equality.

The current source-backed audit has no unclassified documented-only or
registered-only callout differences. Any future unmapped name must first be
classified by the source-registration audit before it is accepted here.

### `RmlUi.EventListener` virtual methods

`RmlUi.EventListener.OnAttach`, `OnDetach`, and `ProcessEvent` are registered
on an abstract, non-constructible Lua base type. Lua has no factory for this
type and no returned userdata on which those methods can be invoked; real Lua
listeners use the function/string callback overloads on `Element` and
`Context`. Native modules use the callback-registration ABI instead. These
methods are therefore Lua-only by design, and the parity fixture verifies that
the base type remains non-constructible.

### `Script.*`

The documented `Script.*` functions are Lua-handle operations: they inspect or
mutate the current embedded Lua handle, its call-in registration, watcher
masks, delayed callbacks, permissions, or Lua state. Native modules are loaded
through a separate ABI and do not own that Lua handle, so a same-named Rust
module would not be an equivalent implementation. The current documented
set is:

`Script.DelayByFrames`, `Script.GetCallInList`, `Script.GetCtrlTeam`,
`Script.GetFullCtrl`, `Script.GetFullRead`, `Script.GetGlobal`, `Script.GetName`,
`Script.GetReadAllyTeam`, `Script.GetReadTeam`, `Script.GetRegistry`,
`Script.GetSelectTeam`, `Script.GetSynced`, `Script.GetWatchAllowTarget`,
`Script.GetWatchExplosion`, `Script.GetWatchFeature`, `Script.GetWatchProjectile`,
`Script.GetWatchUnit`, `Script.GetWatchWeapon`, `Script.IsEngineMinVersion`,
`Script.Kill`, `Script.SetWatchAllowTarget`, `Script.SetWatchExplosion`,
`Script.SetWatchFeature`, `Script.SetWatchProjectile`, `Script.SetWatchUnit`,
`Script.SetWatchWeapon`, `Script.AddActionFallback`,
`Script.RemoveActionFallback`, `Script.PermitHelperAIs`, and
`Script.UpdateCallIn`.

`Script.PermitHelperAIs` is installed by LuaRules and `Script.UpdateCallIn` is
the exact runtime spelling. The generated inventory is filtered against active
Script-table registrations so stale documentation aliases do not count as
public APIs. The list is Lua-only by design, not exempt from Lua registration,
signature, argument/error, and result tests.

`Spring.InvokeNativeModule` is the inverse boundary: Lua sends a serialized
message into the loaded native module. It is documented, source-registered,
and smoke-tested, but has no native callout counterpart because the native
module cannot meaningfully invoke itself through the same API.

### `VFS.DownloadArchive` and `VFS.AbortDownload`

These operations are native counterparts, not Lua-only exceptions. The native
`VFSApi` appends both entries and calls the same queue, validation, and event
dispatch implementation used by the Lua wrappers. Deterministic invalid-input
and missing-download cancellation cases are covered by the parity fixture; a
successful download is intentionally not part of the offline deterministic
suite because it depends on external repositories.

### `VFS.ScanAllDirs` context

`VFS.ScanAllDirs` is registered by `LuaVFSDownload` in LuaUI/menu states, not
by the `LuaVFS` table installed in LuaRules or LuaGaia. The native
`Vfs.scan_all_dirs` operation is the same archive-scanner operation and is
available to native modules, so this is a context-placement difference rather
than a missing port. Its Lua and native signatures/results are tested in the
widget surface; an unsynced-gadget test must not call the Lua symbol and treat
its absence there as a parity failure.

The shared integer-pack cases stay within the exact integer range of this
engine's 32-bit Lua number type. Rust's `u32`/`i32` slices can represent a
wider domain; that is a native type-domain difference, not evidence that Lua
can accept the same exact numeric inputs.

### Process-local timing and profiler values

The Lua and native parity processes have independent render clocks and
process-local profiler registries. Timing values therefore use the manifest's
explicit numeric tolerance or shape comparison. `GetProfilerRecordNames`
returns the same engine-owned registry through both implementations, but the
registry can contain a different number of records depending on which engine
components have initialized in that process; its parity check consequently
asserts the returned numeric shape rather than treating the process-local
count as a semantic API mismatch.

### Lua-handle lifecycle and message callins

The following documented callins are Lua-only by design because they belong to
the engine's embedded Lua-handle lifecycle or to communication between its
synced and unsynced Lua states. A native module is loaded and messaged through
different ABI entry points, so adding a same-named native callback would not
preserve the Lua contract:

| Lua callin | Reason |
| --- | --- |
| `Initialize` | Native modules use `InitializeNativeModule`. |
| `LoadCode` | Native modules are loaded as shared libraries, not Lua chunks. |
| `RecvFromSynced` | It is IPC between the two Lua handles. |
| `GotChatMsg` | It is Lua-handle chat routing. |
| `RecvLuaMsg` | It is Lua-handle message routing; native uses `HandleLuaMsg`. |
| `RecvSkirmishAIMessage` | It is Lua-handle skirmish-AI routing. |

These are still required to have Lua-side registration/argument/error tests;
they are excluded from Lua/native equality because no equivalent native call
exists by design.

## Confirmed native-only representation surfaces

The following are native representation mechanisms, not omissions from the
Lua API:

- explicit Rust receiver handles for Lua userdata such as RmlUi contexts,
  documents, elements, events, VAOs, and VBOs;
- Rust option/descriptor records that expand Lua optional arguments and C ABI
  presence flags into typed Rust values;
- native callback result/error records and FFI memory-management helpers;
- native event callbacks that have no Lua callin because they are internal
  native integration hooks, such as native module lifecycle, Lua-call bridge,
  and native-only object movement notifications.

These still require signature and behavior tests at their own interface
boundary. They must not be used as evidence that a corresponding Lua surface
was tested. The current native-only callback names are:

- `CollectGarbage`
- `DrawAlphaFeaturesLua`, `DrawAlphaUnitsLua`, `DrawOpaqueFeaturesLua`,
  `DrawOpaqueUnitsLua`
- `FeatureMoved`, `UnitMoved`
- `HandleLuaCall`, `HandleLuaMsg`
- `LastMessagePosition`
- `Pong`

`Shutdown` is a separate lifecycle label rather than a shared callback:
Lua's hook shuts down an embedded Lua handle, while native's hook shuts down
the loaded native module. It has no event payload and is excluded from the
shared value-parity count.

The callin audit currently reports 190 documented namespace rows, 156 unique
Lua callin names, 149 shared event names, and one lifecycle-only label. The
runtime deterministic driver compares the 149 shared callbacks that are
eligible for that driver after excluding the lifecycle-only `Shutdown` label.

## Other Lua-facing categories

These categories are tracked separately from ordinary function callouts:

- `math.*` is a global Lua table installed by `LuaMathExtra`; its native Rust
  counterpart is the `MathExtra` module. It is a namespace spelling difference,
  not a native-only API.
- Lua constants, definition proxies (`UnitDefs`, `FeatureDefs`, and related
  tables), and userdata properties/metamethods are object/proxy surfaces rather
  than ordinary `Spring.*` functions. They require their own registration,
  signature/property, and value tests.
- RmlUi userdata methods and properties are mapped separately from global
  `RmlUi.*` functions. Rust receiver handles and typed property accessors are
  native representation details; they are not evidence that a Lua method was
  omitted.
- `gl.*` is the Lua OpenGL surface. Rust `Gfx` methods that map to it are
  checked separately from native-only typed helpers.
- LuaFont, VAO, VBO, FBO, and RBO userdata registrations are audited in
  `lua_userdata_surface_audit.md`; integer Rust handles are an intentional
  ownership representation rather than missing Lua userdata types.

## Audit completion boundary

The source/signature classification is complete for the currently discovered
function, userdata, and callin inventories:

- Lua constants and definition proxies are tracked as their own proxy surface;
- userdata methods/properties are tracked by `audit_lua_userdata_surfaces.py`;
- engine-to-Lua and engine-to-native callbacks are tracked by
  `audit_callins.py`;
- every Rust-only label is assigned a representation/integration/typed-
  extension category in `api_surface_audit.md`.

This classification does not waive runtime coverage: the parity aggregate must
still drive matched surfaces to 100%, and native-only surfaces need their own
native-interface tests where they are executable. A source or signature audit
failure remains an implementation/documentation gap.

## Required audit statuses

Every discovered surface should end in exactly one of these states:

- `matched_and_tested`: both directions exist and behavior/signatures agree;
- `lua_only_tested`: Lua exists, no native counterpart is required by this
  contract;
- `native_only_tested`: native exists as an intentional representation or
  integration surface;
- `documented_but_unregistered`;
- `registered_but_undocumented`;
- `unresolved_gap`.

The last state is not a waiver. It is the queue for implementation or an
explicit design decision.
