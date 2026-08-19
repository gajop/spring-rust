/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <string_view>

#include "WasmEnvironment.h"

namespace recoil::wasm::core {

// Generated-shaped inventory for the Core ABI. The next codegen pass should
// emit this from the same semantic model as WasmCalloutRegistry; keeping even
// the vertical slice in one table prevents the validator and linker from
// drifting apart.
struct ImportDescriptor {
	std::string_view module;
	std::string_view name;
	std::uint32_t environmentMask;
};

inline constexpr std::string_view UnitsInfoModule = "spring:units-info";
inline constexpr std::string_view GetUnitDefIDImport = "get-unit-def-id";
inline constexpr std::string_view GetUnitPositionImport = "get-unit-position";
inline constexpr std::string_view GameFrameExport = "spring:callin/game-frame";

inline constexpr std::uint32_t SyncedEnvironmentMask =
	(1u << static_cast<std::uint32_t>(WasmEnvironment::RulesSynced)) |
	(1u << static_cast<std::uint32_t>(WasmEnvironment::GaiaSynced));

inline constexpr ImportDescriptor kImports[] = {
	{UnitsInfoModule, GetUnitDefIDImport, SyncedEnvironmentMask},
	{UnitsInfoModule, GetUnitPositionImport, SyncedEnvironmentMask},
};

inline const ImportDescriptor* FindImport(std::string_view module, std::string_view name)
{
	for (const ImportDescriptor& import : kImports) {
		if (import.module == module && import.name == name)
			return &import;
	}
	return nullptr;
}

inline bool ImportAllowed(std::string_view module, std::string_view name,
	WasmEnvironment environment)
{
	const ImportDescriptor* import = FindImport(module, name);
	if (import == nullptr)
		return false;
	const std::uint32_t bit = 1u << static_cast<std::uint32_t>(environment);
	return (import->environmentMask & bit) != 0;
}

} // namespace recoil::wasm::core
