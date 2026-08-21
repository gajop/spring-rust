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
	// system-control contains process lifecycle/watchdog/restart authority. The
	// sole reviewed Core capability is CallAsTeam: it only scopes simulation
	// team context around one synchronous guest callback and does not cross the
	// process/OS boundary.
	if (module == "spring:system-control")
		return name == "call-as-team";

	// messages.send-commands feeds guest-controlled text to
	// guihandler->RunCustomCommands. That is engine command authority, not a
	// normal message capability.
	if (module == "spring:messages" && name == "send-commands")
		return false;

	// These namespaces exist for engine/unit performance diagnostics. In
	// particular spring:desync exposes wall clocks to synced code behind the
	// process-local SPRING_ENABLE_SYNCED_TIMERS switch. That switch is not part
	// of synced runtime identity, so neither namespace is valid production ABI.
	if (module == "spring:benchmark" || module == "spring:desync") {
#if defined(UNIT_TEST)
		return true;
#else
		return false;
#endif
	}

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

	return sourceMask & UnsyncedEnvironmentMask;
}

} // namespace recoil::wasm::core::registry_policy
