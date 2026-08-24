/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <array>
#include <string>
#include <string_view>
#include <variant>

#include "NativeInterface/WasmUiVisibility.h"
#include "NativeInterface/api/Callins.h"
#include "Sim/Features/Feature.h"
#include "Sim/Units/UnitDef.h"
#include "System/float3.h"
#include "WasmCoreCallinId.h"
#include "WasmCoreCallinPolicy.h"

namespace recoil::wasm::core {

// Core callins bypass WasmValue serialization, so UI filtering must operate on
// the native query before the record enters the guest. This mirrors the
// visibility/redaction policy in NativeInterfaceEventClient::SanitizeUiCallin
// while keeping synced/gaia dispatch allocation-free and serialization-free.
//
// Which policy a callin uses is fixed by its event name, so it is resolved once
// at compile time into a table indexed by the numeric callin id. Dispatch does
// a single load and, for the overwhelming majority of callins that have no
// visibility-sensitive payload, no call at all.
class UiCallinFilter {
public:
	using Handler = bool (*)(UiCallinFilter&, const void* query, bool& include,
		const void*& filteredQuery, std::string& error);

	// Only called when the routing line says this callin has a visibility
	// policy; UI_CALLIN_POLICIES is read when routes are built, not per call.
	bool Prepare(WasmCoreCallin callin, const void* query, bool& include,
		const void*& filteredQuery, std::string& error);

	// Handlers are static so the table can be built at compile time; the
	// per-dispatch state they need lives in the instance passed to them.
	template<typename T>
	static const T* Require(UiCallinFilter& filter, const void* query, std::string& error)
	{
		if (query != nullptr)
			return static_cast<const T*>(query);
		error = "Core UI callin " + std::string(filter.activeName) +
			" received a null query";
		return nullptr;
	}

	template<typename T>
	static T* Copy(UiCallinFilter& filter, const void* query, const void*& filteredQuery,
		std::string& error)
	{
		const T* typed = Require<T>(filter, query, error);
		if (typed == nullptr)
			return nullptr;
		filter.storage = *typed;
		T& copy = std::get<T>(filter.storage);
		filteredQuery = &copy;
		return &copy;
	}

	static bool VisibleUnit(int unitID)
	{
		return unitID >= 0 && WasmUiVisibility::FindUnit(
			unitID, WasmUiVisibility::UnitAccess::Visible) != nullptr;
	}

	static bool VisibleFeatureTeam(int featureID)
	{
		const CFeature* feature = WasmUiVisibility::FindFeature(featureID);
		return feature != nullptr &&
			(feature->allyteam < 0 || WasmUiVisibility::IsAllyTeamVisible(feature->allyteam));
	}

	template<typename T>
	static void RedactAttacker(T& value)
	{
		if (value.attackerID < 0)
			return;
		const CUnit* attacker = WasmUiVisibility::FindUnit(
			value.attackerID, WasmUiVisibility::UnitAccess::Visible);
		if (attacker == nullptr) {
			value.attackerID = -1;
			value.attackerDefID = -1;
			value.attackerTeam = -1;
			return;
		}
		if (!WasmUiVisibility::IsUnitTyped(attacker)) {
			value.attackerDefID = -1;
			return;
		}
		if (const UnitDef* def = WasmUiVisibility::EffectiveUnitDef(attacker))
			value.attackerDefID = def->id;
	}

	// --- handlers -------------------------------------------------------

	static bool DefaultCommand(UiCallinFilter& filter, const void* query, bool& include,
		const void*&, std::string& error)
	{
		const auto* value = Require<DefaultCommandQuery>(filter, query, error);
		if (value == nullptr) return false;
		include = (value->unitID < 0 || VisibleUnit(value->unitID)) &&
			(value->featureID < 0 || WasmUiVisibility::FindFeature(value->featureID) != nullptr);
		return true;
	}

	static bool WorldTooltip(UiCallinFilter& filter, const void* query, bool& include,
		const void*&, std::string& error)
	{
		const auto* value = Require<WorldTooltipQuery>(filter, query, error);
		if (value == nullptr) return false;
		if (value->kind == 1)
			include = VisibleUnit(value->unitID);
		else if (value->kind == 2)
			include = value->featureID >= 0 && WasmUiVisibility::FindFeature(value->featureID) != nullptr;
		else if (value->kind == 0)
			include = (value->unitID < 0 || VisibleUnit(value->unitID)) &&
				(value->featureID < 0 || WasmUiVisibility::FindFeature(value->featureID) != nullptr);
		return true;
	}

	template<typename T>
	static bool FeatureCreatedDestroyed(UiCallinFilter& filter, const void* query,
		bool& include, const void*&, std::string& error)
	{
		const auto* value = Require<T>(filter, query, error);
		if (value == nullptr) return false;
		include = value->allyTeamID < 0 || WasmUiVisibility::IsAllyTeamVisible(value->allyTeamID);
		return true;
	}

	static bool FeatureMoved(UiCallinFilter& filter, const void* query, bool& include,
		const void*&, std::string& error)
	{
		const auto* value = Require<FeatureMovedQuery>(filter, query, error);
		if (value == nullptr) return false;
		include = VisibleFeatureTeam(value->featureID);
		return true;
	}

	static bool FeatureDamaged(UiCallinFilter& filter, const void* query, bool& include,
		const void*& filteredQuery, std::string& error)
	{
		auto* value = Copy<FeatureDamagedQuery>(filter, query, filteredQuery, error);
		if (value == nullptr) return false;
		include = VisibleFeatureTeam(value->featureID);
		if (include) RedactAttacker(*value);
		return true;
	}

	static bool ProjectileEvent(UiCallinFilter& filter, const void* query, bool& include,
		const void*&, std::string& error)
	{
		const auto* value = Require<ProjectileEventQuery>(filter, query, error);
		if (value == nullptr) return false;
		include = value->ownerID < 0 ||
			WasmUiVisibility::FindUnit(value->ownerID, WasmUiVisibility::UnitAccess::Ally) != nullptr;
		return true;
	}

	static bool Explosion(UiCallinFilter& filter, const void* query, bool& include,
		const void*& filteredQuery, std::string& error)
	{
		auto* value = Copy<ExplosionQuery>(filter, query, filteredQuery, error);
		if (value == nullptr) return false;
		if (!WasmUiVisibility::FullRead() &&
			!WasmUiVisibility::IsPositionVisible(float3{value->pos.x, value->pos.y, value->pos.z})) {
			include = false;
			return true;
		}
		if (value->ownerID >= 0 && !VisibleUnit(value->ownerID))
			value->ownerID = -1;
		if (value->projectileID >= 0 && WasmUiVisibility::FindProjectile(value->projectileID) == nullptr)
			value->projectileID = -1;
		return true;
	}

	static bool UnitLosEvent(UiCallinFilter& filter, const void* query, bool& include,
		const void*& filteredQuery, std::string& error)
	{
		auto* value = Copy<UnitLosEventQuery>(filter, query, filteredQuery, error);
		if (value == nullptr) return false;
		include = WasmUiVisibility::IsAllyTeamVisible(value->allyTeam);
		if (include && !WasmUiVisibility::FullRead()) {
			value->allyTeam = -1;
			value->unitDefID = -1;
		}
		return true;
	}

	static bool UnitSeismicPing(UiCallinFilter& filter, const void* query, bool& include,
		const void*& filteredQuery, std::string& error)
	{
		auto* value = Copy<UnitSeismicPingQuery>(filter, query, filteredQuery, error);
		if (value == nullptr) return false;
		include = WasmUiVisibility::IsAllyTeamVisible(value->allyTeam);
		if (!include || WasmUiVisibility::FullRead())
			return true;
		const CUnit* unit = WasmUiVisibility::FindUnit(
			value->unitID, WasmUiVisibility::UnitAccess::Visible);
		if (unit != nullptr && WasmUiVisibility::IsUnitInLos(unit)) {
			include = false;
			return true;
		}
		value->allyTeam = -1;
		value->unitID = -1;
		value->unitDefID = -1;
		return true;
	}

	template<typename T>
	static bool UnitLoadedUnloaded(UiCallinFilter& filter, const void* query, bool& include,
		const void*&, std::string& error)
	{
		const auto* value = Require<T>(filter, query, error);
		if (value == nullptr) return false;
		include = WasmUiVisibility::IsTeamVisible(value->unitTeam) ||
			WasmUiVisibility::IsTeamVisible(value->transportTeam);
		return true;
	}

	static bool UnitTaken(UiCallinFilter& filter, const void* query, bool& include,
		const void*&, std::string& error)
	{
		const auto* value = Require<UnitTakenQuery>(filter, query, error);
		if (value == nullptr) return false;
		include = WasmUiVisibility::IsTeamVisible(value->oldTeam);
		return true;
	}

	static bool UnitGiven(UiCallinFilter& filter, const void* query, bool& include,
		const void*&, std::string& error)
	{
		const auto* value = Require<UnitGivenQuery>(filter, query, error);
		if (value == nullptr) return false;
		include = WasmUiVisibility::IsTeamVisible(value->newTeam);
		return true;
	}

	template<typename T>
	static bool UnitTeamVisible(UiCallinFilter& filter, const void* query, bool& include,
		const void*&, std::string& error)
	{
		const auto* value = Require<T>(filter, query, error);
		if (value == nullptr) return false;
		include = WasmUiVisibility::IsTeamVisible(value->unitTeam);
		return true;
	}

	static bool UnitDestroyed(UiCallinFilter& filter, const void* query, bool& include,
		const void*& filteredQuery, std::string& error)
	{
		auto* value = Copy<UnitDestroyedQuery>(filter, query, filteredQuery, error);
		if (value == nullptr) return false;
		include = WasmUiVisibility::IsTeamVisible(value->unitTeam);
		if (include) RedactAttacker(*value);
		return true;
	}

	static bool UnitDamaged(UiCallinFilter& filter, const void* query, bool& include,
		const void*& filteredQuery, std::string& error)
	{
		auto* value = Copy<UnitDamagedQuery>(filter, query, filteredQuery, error);
		if (value == nullptr) return false;
		include = WasmUiVisibility::IsTeamVisible(value->unitTeam);
		if (include) RedactAttacker(*value);
		return true;
	}

	static bool UnitCreated(UiCallinFilter& filter, const void* query, bool& include,
		const void*& filteredQuery, std::string& error)
	{
		auto* value = Copy<UnitCreatedQuery>(filter, query, filteredQuery, error);
		if (value == nullptr) return false;
		include = WasmUiVisibility::IsTeamVisible(value->unitTeam);
		if (include && value->builderID >= 0 && !VisibleUnit(value->builderID))
			value->builderID = -1;
		return true;
	}

	static bool UnitFromFactory(UiCallinFilter& filter, const void* query, bool& include,
		const void*& filteredQuery, std::string& error)
	{
		auto* value = Copy<UnitFromFactoryQuery>(filter, query, filteredQuery, error);
		if (value == nullptr) return false;
		include = WasmUiVisibility::IsTeamVisible(value->unitTeam);
		if (include && value->factoryID >= 0 && !VisibleUnit(value->factoryID)) {
			value->factoryID = -1;
			value->factoryDefID = -1;
		}
		return true;
	}

	static bool UnitUnitCollision(UiCallinFilter& filter, const void* query, bool& include,
		const void*&, std::string& error)
	{
		const auto* value = Require<UnitUnitCollisionQuery>(filter, query, error);
		if (value == nullptr) return false;
		include = VisibleUnit(value->colliderID) && VisibleUnit(value->collideeID);
		return true;
	}

	static bool UnitFeatureCollision(UiCallinFilter& filter, const void* query, bool& include,
		const void*&, std::string& error)
	{
		const auto* value = Require<UnitFeatureCollisionQuery>(filter, query, error);
		if (value == nullptr) return false;
		include = VisibleUnit(value->colliderID) &&
			WasmUiVisibility::FindFeature(value->collideeID) != nullptr;
		return true;
	}

private:
	using Storage = std::variant<
		std::monostate,
		FeatureDamagedQuery,
		ExplosionQuery,
		UnitLosEventQuery,
		UnitSeismicPingQuery,
		UnitDestroyedQuery,
		UnitDamagedQuery,
		UnitCreatedQuery,
		UnitFromFactoryQuery>;
	Storage storage;
	std::string_view activeName;
};

// Several engine events share one visibility policy. The registry already
// records that grouping, but under a canonical name chosen for the query
// struct, so the aliases the filter cares about are spelled out here.
constexpr std::string_view UiFilterKey(std::string_view name)
{
	if (name == "UnitEnteredLos" || name == "UnitEnteredRadar" ||
		name == "UnitLeftLos" || name == "UnitLeftRadar")
		return "UnitLosEvent";
	if (name == "UnitCloaked" || name == "UnitDecloaked")
		return "UnitCloakEvent";
	if (name == "UnitEnteredAir" || name == "UnitEnteredUnderwater" ||
		name == "UnitEnteredWater" || name == "UnitLeftAir" ||
		name == "UnitLeftUnderwater" || name == "UnitLeftWater")
		return "UnitMovementClassEvent";
	if (name == "UnitArrivedAtGoal" || name == "UnitMoveFailed" || name == "UnitMoved")
		return "UnitMoveEvent";
	if (name == "ProjectileCreated" || name == "ProjectileDestroyed")
		return "ProjectileEvent";
	return name;
}

constexpr UiCallinFilter::Handler ResolveUiFilterHandler(std::string_view name)
{
	using F = UiCallinFilter;
	const std::string_view key = UiFilterKey(name);
	if (key == "DefaultCommand") return &F::DefaultCommand;
	if (key == "WorldTooltip") return &F::WorldTooltip;
	if (key == "FeatureCreated") return &F::FeatureCreatedDestroyed<FeatureCreatedQuery>;
	if (key == "FeatureDestroyed") return &F::FeatureCreatedDestroyed<FeatureDestroyedQuery>;
	if (key == "FeatureMoved") return &F::FeatureMoved;
	if (key == "FeatureDamaged") return &F::FeatureDamaged;
	if (key == "ProjectileEvent") return &F::ProjectileEvent;
	if (key == "Explosion") return &F::Explosion;
	if (key == "UnitLosEvent") return &F::UnitLosEvent;
	if (key == "UnitSeismicPing") return &F::UnitSeismicPing;
	if (key == "UnitLoaded") return &F::UnitLoadedUnloaded<UnitLoadedQuery>;
	if (key == "UnitUnloaded") return &F::UnitLoadedUnloaded<UnitUnloadedQuery>;
	if (key == "UnitTaken") return &F::UnitTaken;
	if (key == "UnitGiven") return &F::UnitGiven;
	if (key == "UnitFinished") return &F::UnitTeamVisible<UnitFinishedQuery>;
	if (key == "UnitReverseBuilt") return &F::UnitTeamVisible<UnitReverseBuiltQuery>;
	if (key == "UnitConstructionDecayed") return &F::UnitTeamVisible<UnitConstructionDecayedQuery>;
	if (key == "UnitIdle") return &F::UnitTeamVisible<UnitIdleQuery>;
	if (key == "UnitCommand") return &F::UnitTeamVisible<UnitCommandQuery>;
	if (key == "UnitCmdDone") return &F::UnitTeamVisible<UnitCmdDoneQuery>;
	if (key == "UnitStunned") return &F::UnitTeamVisible<UnitStunnedQuery>;
	if (key == "UnitExperience") return &F::UnitTeamVisible<UnitExperienceQuery>;
	if (key == "UnitHarvestStorageFull") return &F::UnitTeamVisible<UnitHarvestStorageFullQuery>;
	if (key == "UnitMovementClassEvent") return &F::UnitTeamVisible<UnitMovementClassEventQuery>;
	if (key == "UnitCloakEvent") return &F::UnitTeamVisible<UnitCloakEventQuery>;
	if (key == "UnitMoveEvent") return &F::UnitTeamVisible<UnitMoveEventQuery>;
	if (key == "StockpileChanged") return &F::UnitTeamVisible<StockpileChangedQuery>;
	if (key == "RenderUnitDestroyed") return &F::UnitTeamVisible<RenderUnitDestroyedQuery>;
	if (key == "UnitDestroyed") return &F::UnitDestroyed;
	if (key == "UnitDamaged") return &F::UnitDamaged;
	if (key == "UnitCreated") return &F::UnitCreated;
	if (key == "UnitFromFactory") return &F::UnitFromFactory;
	if (key == "UnitUnitCollision") return &F::UnitUnitCollision;
	if (key == "UnitFeatureCollision") return &F::UnitFeatureCollision;
	// No special visibility policy: send the original native query unchanged.
	return nullptr;
}

struct UiCallinPolicy {
	UiCallinFilter::Handler handler = nullptr;
	bool contributesResult = true;
};

inline constexpr std::array<UiCallinPolicy, CORE_CALLIN_COUNT> UI_CALLIN_POLICIES = [] {
	std::array<UiCallinPolicy, CORE_CALLIN_COUNT> entries{};
	for (std::size_t index = 0; index + 1u < CORE_CALLIN_COUNT; ++index) {
		const char* name = recoil::wasm::generated::kCallins[index].name;
		entries[index + 1u] = {
			.handler = ResolveUiFilterHandler(name),
			.contributesResult = ResolveUiContributesResult(name),
		};
	}
	return entries;
}();

inline bool UiCallinFilter::Prepare(WasmCoreCallin callin, const void* query, bool& include,
	const void*& filteredQuery, std::string& error)
{
	include = true;
	filteredQuery = query;
	storage = std::monostate{};

	const std::size_t slot = static_cast<std::size_t>(callin);
	const UiCallinPolicy& policy =
		UI_CALLIN_POLICIES[slot < CORE_CALLIN_COUNT ? slot : 0u];
	if (policy.handler == nullptr)
		return true;

	activeName = CallinName(callin);
	WasmUiVisibility::ScopedContext uiContext(true);
	return policy.handler(*this, query, include, filteredQuery, error);
}

} // namespace recoil::wasm::core
