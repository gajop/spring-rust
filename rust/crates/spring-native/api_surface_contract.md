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
inventory separately tracks other global helpers (`loadstring`, `pairs`,
`next`, and `SendToUnsynced`) until their signatures and native ownership are
audited.

### `VFS.Include`

`VFS.Include` loads and executes an arbitrary Lua chunk in a selected Lua
environment and returns arbitrary Lua values. There is no meaningful typed
native equivalent. It must still be tested as a Lua API (registration,
argument behavior, return behavior, and error behavior), but it is excluded
from Lua/native result equality.

The audit keeps every other unmapped Lua callout unresolved until its source
registration and native counterpart have been compared.

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
`Script.RemoveActionFallback`, and `Script.UpdateCallin`.

`Script.GetCallInList` was found in the engine registration table without a
generated documentation entry; it is now included in the local API inventory.
The list is Lua-only by design, not exempt from Lua registration, signature,
argument/error, and result tests.

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

## Deliberately unresolved until audited

The following categories are not yet classified as intentional differences:

- Lua constant tables, definition proxies, and userdata metamethods;
- native-only labels not listed in a source-backed confirmed category;
- exact runtime parity tests for any newly bridged engine-to-native callins;
- native-only `Gfx`/`Vfs` helpers until each has a source-level classification
  and an executable native-interface test.

These remain `unresolved` in the audit until source registration, signatures,
and executable behavior have been checked. A missing native callback may need
to be ported; a native-only label may need Lua documentation; or the surface
may eventually be added to this intentional list with a concrete reason.

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
