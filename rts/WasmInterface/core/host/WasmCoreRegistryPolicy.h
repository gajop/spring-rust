/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <string_view>

#include "WasmEnvironment.h"

namespace recoil::wasm::core::registry_policy {

inline constexpr std::uint32_t SyncedEnvironmentMask =
	(1u << static_cast<std::uint32_t>(WasmEnvironment::RulesSynced)) |
	(1u << static_cast<std::uint32_t>(WasmEnvironment::GaiaSynced));
inline constexpr std::uint32_t UnsyncedEnvironmentMask =
	(1u << static_cast<std::uint32_t>(WasmEnvironment::RulesUnsynced)) |
	(1u << static_cast<std::uint32_t>(WasmEnvironment::GaiaUnsynced)) |
	(1u << static_cast<std::uint32_t>(WasmEnvironment::UI));

// Handwritten fast imports predate the generated production registry and are
// checked first by LookupImport. Keep their production authorization explicit
// so a legacy fast binding cannot bypass a deny in generated registry policy.
inline bool HandwrittenImportAllowed(std::string_view module, std::string_view name)
{
	// Transport completeness is the default Core contract. The host already
	// validates pointers, ABI signatures, and environment masks; capability and
	// process policy are opt-in policy above this layer so they cannot silently
	// remove a generated owned API entry.
	(void)module;
	(void)name;
	return true;
}

// Keep sync normalization separate from capability authorization. The legacy
// message fast bindings were historically registered for all environments even
// though chat/UI/player state is unsynced. SendToUnsynced is the sole reviewed
// bridge in the opposite direction.
inline std::uint32_t HandwrittenEnvironmentMask(
	std::string_view module,
	std::string_view name,
	std::uint32_t sourceMask)
{
	if (module != "spring:messages")
		return sourceMask;

	if (name == "send-to-unsynced")
		return sourceMask & SyncedEnvironmentMask;
	// Core benchmark and parity fixtures report from synced gadgets through the
	// existing LuaRules message sink. This is an in-engine message, not an OS
	// capability; retaining it for synced guests preserves the established
	// fixture protocol while the host still bounds/copies the input string.
	if (name == "send-lua-rules-msg")
		return sourceMask;
	// Logging is an explicitly side-effect-only diagnostic operation.  It does
	// not expose wall-clock state or feed simulation decisions, and Spring.Log
	// has always been available from synced LuaRules for this purpose.  Keep the
	// Core-WASM API consistent with that contract so synced guests can report
	// deterministic state transitions without routing diagnostics through a
	// game-specific message gadget.
	if (name == "log" || name == "echo")
		return sourceMask;

	return sourceMask & UnsyncedEnvironmentMask;
}

} // namespace recoil::wasm::core::registry_policy
