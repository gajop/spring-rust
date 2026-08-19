/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <string_view>

#include "WasmEnvironment.h"

namespace recoil::wasm::core {

struct ImportDescriptor {
	std::string_view module;
	std::string_view name;
	std::string_view signature;
	std::uint32_t environmentMask;
};

inline constexpr std::string_view UnitsInfoModule = "spring:units-info";
inline constexpr std::string_view UnitsQueryModule = "spring:units-query";
inline constexpr std::string_view UnitDefsModule = "spring:unit-defs";

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
	{UnitsInfoModule, "get-unit-def-id", "i32->i64", AllEnvironmentMask},
	{UnitsInfoModule, "get-unit-team", "i32->i64", AllEnvironmentMask},
	{UnitsInfoModule, "get-unit-is-dead", "i32->i64", AllEnvironmentMask},
	{UnitsInfoModule, "get-unit-experience", "i32->i64", AllEnvironmentMask},
	{UnitsInfoModule, "get-unit-position", "i32,i32,i32->i32", AllEnvironmentMask},
	{UnitsInfoModule, "get-unit-velocity", "i32,i32->i32", AllEnvironmentMask},
	{UnitsInfoModule, "get-unit-health", "i32,i32->i32", AllEnvironmentMask},

	{UnitsQueryModule, "valid-unit-id", "i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-all-units", "i32,i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-team-units", "i32,i32,i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-team-unit-def-count", "i32,i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-team-unit-count", "i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-units-in-rectangle", "f32,f32,f32,f32,i32,i32,i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-units-in-box", "f32,f32,f32,f32,f32,f32,i32,i32,i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-units-in-sphere", "f32,f32,f32,f32,i32,i32,i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-units-in-cylinder", "f32,f32,f32,i32,i32,i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-unit-nearest-ally", "i32,f32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-unit-nearest-enemy", "i32,f32,i32->i64", AllEnvironmentMask},
	{UnitsQueryModule, "get-unit-separation", "i32,i32,i32->i64", AllEnvironmentMask},

	// Core strings are raw bytes. `capacity` is measured in bytes, there is no
	// NUL terminator, and BufferOverflow reports the exact required byte count.
	{UnitDefsModule, "get-unit-def-name", "i32,i32,i32->i64", AllEnvironmentMask},
	{UnitDefsModule, "get-unit-def-human-name", "i32,i32,i32->i64", AllEnvironmentMask},
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
