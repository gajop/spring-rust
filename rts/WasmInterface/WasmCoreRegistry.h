/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <string_view>

#include "WasmEnvironment.h"

namespace recoil::wasm::core {

// Generated-shaped inventory for the Core ABI. The code generator should emit
// this from the same semantic model as WasmCalloutRegistry; validation and
// linker registration both consume these exact names/capability masks.
//
// Signature grammar is intentionally tiny and deterministic:
//   i32,i32->i32
//   i32->i64
//   ->
// Only Core numeric types used by the Spring ABI are accepted.
struct ImportDescriptor {
	std::string_view module;
	std::string_view name;
	std::string_view signature;
	std::uint32_t environmentMask;
};

inline constexpr std::string_view UnitsInfoModule = "spring:units-info";
inline constexpr std::string_view GetUnitDefIDImport = "get-unit-def-id";
inline constexpr std::string_view GetUnitTeamImport = "get-unit-team";
inline constexpr std::string_view GetUnitIsDeadImport = "get-unit-is-dead";
inline constexpr std::string_view GetUnitExperienceImport = "get-unit-experience";
inline constexpr std::string_view GetUnitPositionImport = "get-unit-position";
inline constexpr std::string_view GetUnitVelocityImport = "get-unit-velocity";
inline constexpr std::string_view GetUnitHealthImport = "get-unit-health";

inline constexpr std::string_view GameFrameExport = "spring:callin/game-frame";
inline constexpr std::string_view GameFramePostExport = "spring:callin/game-frame-post";
inline constexpr std::string_view UpdateExport = "spring:callin/update";
inline constexpr std::string_view UnitCreatedExport = "spring:callin/unit-created";
inline constexpr std::string_view UnitPreDamagedExport = "spring:callin/unit-pre-damaged";
inline constexpr std::string_view AllowUnitCreationExport = "spring:callin/allow-unit-creation";
inline constexpr std::string_view DrawWorldExport = "spring:callin/draw-world";

inline constexpr std::uint32_t AllEnvironmentMask =
	(1u << static_cast<std::uint32_t>(WasmEnvironment::RulesSynced)) |
	(1u << static_cast<std::uint32_t>(WasmEnvironment::RulesUnsynced)) |
	(1u << static_cast<std::uint32_t>(WasmEnvironment::GaiaSynced)) |
	(1u << static_cast<std::uint32_t>(WasmEnvironment::GaiaUnsynced)) |
	(1u << static_cast<std::uint32_t>(WasmEnvironment::UI));

inline constexpr ImportDescriptor kImports[] = {
	{UnitsInfoModule, GetUnitDefIDImport, "i32->i64", AllEnvironmentMask},
	{UnitsInfoModule, GetUnitTeamImport, "i32->i64", AllEnvironmentMask},
	{UnitsInfoModule, GetUnitIsDeadImport, "i32->i64", AllEnvironmentMask},
	{UnitsInfoModule, GetUnitExperienceImport, "i32->i64", AllEnvironmentMask},
	{UnitsInfoModule, GetUnitPositionImport, "i32,i32,i32->i32", AllEnvironmentMask},
	{UnitsInfoModule, GetUnitVelocityImport, "i32,i32->i32", AllEnvironmentMask},
	{UnitsInfoModule, GetUnitHealthImport, "i32,i32->i32", AllEnvironmentMask},
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
