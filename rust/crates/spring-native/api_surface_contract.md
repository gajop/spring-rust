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

### `VFS.Include`

`VFS.Include` loads and executes an arbitrary Lua chunk in a selected Lua
environment and returns arbitrary Lua values. There is no meaningful typed
native equivalent. It must still be tested as a Lua API (registration,
argument behavior, return behavior, and error behavior), but it is excluded
from Lua/native result equality.

No other currently unmatched Lua callout is declared Lua-only by this file.

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
was tested.

## Deliberately unresolved until audited

The following categories are not yet classified as intentional differences:

- Lua `Script.*` functions;
- Lua-only control callins such as `Allow*`, `UnitPreDamaged`, and
  `CommandFallback`;
- Lua-only rendering callins such as `DrawUnit`, `DrawFeature`, and
  `DrawProjectile`;
- Lua input/message callins such as `RecvLuaMsg`, `GotChatMsg`, and
  `RecvFromSynced`;
- Lua constant tables, definition proxies, and userdata metamethods;
- native-only `Gfx`, `RmlUi`, and `Vfs` labels beyond the representation
  mechanisms listed above.

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
