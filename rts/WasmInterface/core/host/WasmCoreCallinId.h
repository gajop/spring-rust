/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <string_view>

#include "wasm/generated/WasmCallinRegistry.h"

namespace recoil::wasm::core::detail {

consteval std::uint16_t CoreCallinOrdinal(std::string_view name)
{
	for (std::size_t index = 0;
		index < sizeof(recoil::wasm::generated::kCallins) /
			sizeof(recoil::wasm::generated::kCallins[0]);
		++index) {
		if (name == recoil::wasm::generated::kCallins[index].name)
			return static_cast<std::uint16_t>(index + 1u);
	}
	return 0;
}

} // namespace recoil::wasm::core::detail

// Every generated Callins.def descriptor has a stable per-build numeric Core
// ID: generated-registry index + 1. Only the hand-specialized hot callins need
// named enum constants here; every other valid ID is represented by casting the
// generated ordinal returned by ResolveCallin(). This avoids a hand-maintained
// 126-entry enum while keeping the hot specialized comparisons compile-time.
enum class WasmCoreCallin : std::uint16_t {
	Invalid = 0,
	GameFrame = recoil::wasm::core::detail::CoreCallinOrdinal("GameFrame"),
	GameFramePost = recoil::wasm::core::detail::CoreCallinOrdinal("GameFramePost"),
	Update = recoil::wasm::core::detail::CoreCallinOrdinal("Update"),
	UnitCreated = recoil::wasm::core::detail::CoreCallinOrdinal("UnitCreated"),
	UnitPreDamaged = recoil::wasm::core::detail::CoreCallinOrdinal("UnitPreDamaged"),
	AllowUnitCreation = recoil::wasm::core::detail::CoreCallinOrdinal("AllowUnitCreation"),
	AddConsoleLine = recoil::wasm::core::detail::CoreCallinOrdinal("AddConsoleLine"),
	CommandNotify = recoil::wasm::core::detail::CoreCallinOrdinal("CommandNotify"),
	DrawWorld = recoil::wasm::core::detail::CoreCallinOrdinal("DrawWorld"),
};

// Compile-time callin id for a literal event name. Every engine dispatch site
// knows which callin it is raising, so the id is a constant there and no
// dispatch layer has to hash or compare the name at runtime.
consteval WasmCoreCallin CoreCallinOf(std::string_view name)
{
	return static_cast<WasmCoreCallin>(
		recoil::wasm::core::detail::CoreCallinOrdinal(name));
}
