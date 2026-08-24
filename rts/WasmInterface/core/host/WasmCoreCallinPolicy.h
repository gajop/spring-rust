/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <array>
#include <cstddef>
#include <cstdint>
#include <string_view>

#include "WasmCoreCallinId.h"
#include "wasm/generated/WasmCallinRegistry.h"

// Every property the Core dispatcher needs about a callin is a pure function of
// the generated descriptor, so it is resolved once at compile time and indexed
// by the numeric callin id. Dispatch never re-derives policy from the event
// name: the name exists only for diagnostics.
namespace recoil::wasm::core {

inline constexpr std::size_t CORE_CALLIN_COUNT =
	(sizeof(recoil::wasm::generated::kCallins) /
		sizeof(recoil::wasm::generated::kCallins[0])) + 1u;

enum class CoreAggregation : std::uint8_t {
	Ignore,
	OrTrue,
	AndFalse,
	First,
	FirstNonEmpty,
	Unsupported,
};

enum class CoreResultKind : std::uint8_t {
	None,
	Bool,
	Int,
	Damage,
	AllowUnitCreation,
	String,
	// Generated fixed-result callins can write their concrete native result
	// directly. For first-result aggregation the dispatcher does not need to
	// understand that struct: only the first contributing module receives the
	// caller's result pointer, while later modules still run with a null sink.
	OpaqueFirst,
	Unsupported,
};

constexpr CoreAggregation ResolveAggregation(std::string_view value)
{
	if (value == "ignore") return CoreAggregation::Ignore;
	if (value == "or-true") return CoreAggregation::OrTrue;
	if (value == "and-false") return CoreAggregation::AndFalse;
	if (value == "first") return CoreAggregation::First;
	if (value == "first-non-empty") return CoreAggregation::FirstNonEmpty;
	return CoreAggregation::Unsupported;
}

constexpr CoreResultKind ResolveResultKind(std::string_view value, CoreAggregation aggregation)
{
	if (aggregation == CoreAggregation::Ignore)
		return CoreResultKind::None;
	if (value == "BoolCallinResult") return CoreResultKind::Bool;
	if (value == "IntCallinResult") return CoreResultKind::Int;
	if (value == "DamageCallinResult") return CoreResultKind::Damage;
	if (value == "AllowUnitCreationResult") return CoreResultKind::AllowUnitCreation;
	if (aggregation == CoreAggregation::FirstNonEmpty && value == "StringCallinResult")
		return CoreResultKind::String;
	if (aggregation == CoreAggregation::First)
		return CoreResultKind::OpaqueFirst;
	return CoreResultKind::Unsupported;
}

// EventHandler discards UI return values for these synced-control events. The
// UI callback still runs, but cannot change simulation aggregation.
constexpr bool ResolveUiContributesResult(std::string_view name)
{
	return name != "Explosion" && name != "UnitUnitCollision" &&
		name != "UnitFeatureCollision";
}

struct CoreCallinPolicy {
	const recoil::wasm::generated::CallinDescriptor* descriptor = nullptr;
	std::uint32_t environmentMask = 0;
	CoreAggregation aggregation = CoreAggregation::Unsupported;
	CoreResultKind resultKind = CoreResultKind::Unsupported;
	bool uiContributesResult = true;
	bool valid = false;
};

inline constexpr std::array<CoreCallinPolicy, CORE_CALLIN_COUNT> CORE_CALLIN_POLICIES = [] {
	std::array<CoreCallinPolicy, CORE_CALLIN_COUNT> entries{};
	for (std::size_t index = 0; index + 1u < CORE_CALLIN_COUNT; ++index) {
		// The numeric callin id is the generated-registry index plus one; slot
		// zero stays the invalid entry.
		const auto& descriptor = recoil::wasm::generated::kCallins[index];
		const CoreAggregation aggregation = ResolveAggregation(descriptor.aggregation);
		entries[index + 1u] = {
			.descriptor = &descriptor,
			.environmentMask = descriptor.environmentMask,
			.aggregation = aggregation,
			.resultKind = ResolveResultKind(descriptor.result, aggregation),
			.uiContributesResult = ResolveUiContributesResult(descriptor.name),
			.valid = true,
		};
	}
	return entries;
}();

constexpr const CoreCallinPolicy& CallinPolicy(WasmCoreCallin callin)
{
	const std::size_t slot = static_cast<std::size_t>(callin);
	return CORE_CALLIN_POLICIES[slot < CORE_CALLIN_COUNT ? slot : 0u];
}

// Diagnostic name for a resolved callin. Dispatch carries the numeric id and
// only pays for the name when building an error message.
constexpr std::string_view CallinName(WasmCoreCallin callin)
{
	const CoreCallinPolicy& policy = CallinPolicy(callin);
	return policy.valid ? std::string_view(policy.descriptor->name)
		: std::string_view("<invalid>");
}

} // namespace recoil::wasm::core
