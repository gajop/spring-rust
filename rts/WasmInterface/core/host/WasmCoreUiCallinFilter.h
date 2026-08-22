/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <string>
#include <string_view>
#include <variant>

#include "NativeInterface/WasmUiVisibility.h"
#include "NativeInterface/api/Callins.h"
#include "Sim/Features/Feature.h"
#include "Sim/Units/UnitDef.h"
#include "System/float3.h"

namespace recoil::wasm::core {

// Core callins bypass WasmValue serialization, so UI filtering must operate on
// the native query before the record enters the guest. This mirrors the
// visibility/redaction policy in NativeInterfaceEventClient::SanitizeUiCallin
// while keeping synced/gaia dispatch allocation-free and serialization-free.
class UiCallinFilter {
public:
	bool Prepare(std::string_view name, const void* query, bool& include,
		const void*& filteredQuery, std::string& error)
	{
		include = true;
		filteredQuery = query;
		storage = std::monostate{};

		if (name == "UnitEnteredLos" || name == "UnitEnteredRadar" ||
			name == "UnitLeftLos" || name == "UnitLeftRadar")
			name = "UnitLosEvent";
		else if (name == "UnitCloaked" || name == "UnitDecloaked")
			name = "UnitCloakEvent";
		else if (name == "UnitEnteredAir" || name == "UnitEnteredUnderwater" ||
			name == "UnitEnteredWater" || name == "UnitLeftAir" ||
			name == "UnitLeftUnderwater" || name == "UnitLeftWater")
			name = "UnitMovementClassEvent";
		else if (name == "UnitArrivedAtGoal" || name == "UnitMoveFailed" ||
			name == "UnitMoved")
			name = "UnitMoveEvent";
		else if (name == "ProjectileCreated" || name == "ProjectileDestroyed")
			name = "ProjectileEvent";
		// DrawWorld is a hot UI callin with no visibility-sensitive payload. Do
		// not construct a UI perspective just to pass its empty query through.
		if (name == "DrawWorld")
			return true;

		WasmUiVisibility::ScopedContext uiContext(true);

		if (name == "DefaultCommand") {
			const auto* value = Require<DefaultCommandQuery>(query, name, error);
			if (value == nullptr) return false;
			include = (value->unitID < 0 || VisibleUnit(value->unitID)) &&
				(value->featureID < 0 || WasmUiVisibility::FindFeature(value->featureID) != nullptr);
			return true;
		}
		if (name == "WorldTooltip") {
			const auto* value = Require<WorldTooltipQuery>(query, name, error);
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
		if (name == "FeatureCreated")
			return FeatureCreatedDestroyed<FeatureCreatedQuery>(query, name, include, error);
		if (name == "FeatureDestroyed")
			return FeatureCreatedDestroyed<FeatureDestroyedQuery>(query, name, include, error);
		if (name == "FeatureMoved") {
			const auto* value = Require<FeatureMovedQuery>(query, name, error);
			if (value == nullptr) return false;
			include = VisibleFeatureTeam(value->featureID);
			return true;
		}
		if (name == "FeatureDamaged") {
			auto* value = Copy<FeatureDamagedQuery>(query, name, filteredQuery, error);
			if (value == nullptr) return false;
			include = VisibleFeatureTeam(value->featureID);
			if (include) RedactAttacker(*value);
			return true;
		}
		if (name == "ProjectileEvent") {
			const auto* value = Require<ProjectileEventQuery>(query, name, error);
			if (value == nullptr) return false;
			include = value->ownerID < 0 ||
				WasmUiVisibility::FindUnit(value->ownerID, WasmUiVisibility::UnitAccess::Ally) != nullptr;
			return true;
		}
		if (name == "Explosion") {
			auto* value = Copy<ExplosionQuery>(query, name, filteredQuery, error);
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
		if (name == "UnitLosEvent") {
			auto* value = Copy<UnitLosEventQuery>(query, name, filteredQuery, error);
			if (value == nullptr) return false;
			include = WasmUiVisibility::IsAllyTeamVisible(value->allyTeam);
			if (include && !WasmUiVisibility::FullRead()) {
				value->allyTeam = -1;
				value->unitDefID = -1;
			}
			return true;
		}
		if (name == "UnitSeismicPing") {
			auto* value = Copy<UnitSeismicPingQuery>(query, name, filteredQuery, error);
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
		if (name == "UnitLoaded")
			return UnitLoadedUnloaded<UnitLoadedQuery>(query, name, include, error);
		if (name == "UnitUnloaded")
			return UnitLoadedUnloaded<UnitUnloadedQuery>(query, name, include, error);
		if (name == "UnitTaken") {
			const auto* value = Require<UnitTakenQuery>(query, name, error);
			if (value == nullptr) return false;
			include = WasmUiVisibility::IsTeamVisible(value->oldTeam);
			return true;
		}
		if (name == "UnitGiven") {
			const auto* value = Require<UnitGivenQuery>(query, name, error);
			if (value == nullptr) return false;
			include = WasmUiVisibility::IsTeamVisible(value->newTeam);
			return true;
		}
		if (name == "UnitFinished") return UnitTeamVisible<UnitFinishedQuery>(query, name, include, error);
		if (name == "UnitReverseBuilt") return UnitTeamVisible<UnitReverseBuiltQuery>(query, name, include, error);
		if (name == "UnitConstructionDecayed") return UnitTeamVisible<UnitConstructionDecayedQuery>(query, name, include, error);
		if (name == "UnitIdle") return UnitTeamVisible<UnitIdleQuery>(query, name, include, error);
		if (name == "UnitCommand") return UnitTeamVisible<UnitCommandQuery>(query, name, include, error);
		if (name == "UnitCmdDone") return UnitTeamVisible<UnitCmdDoneQuery>(query, name, include, error);
		if (name == "UnitStunned") return UnitTeamVisible<UnitStunnedQuery>(query, name, include, error);
		if (name == "UnitExperience") return UnitTeamVisible<UnitExperienceQuery>(query, name, include, error);
		if (name == "UnitHarvestStorageFull") return UnitTeamVisible<UnitHarvestStorageFullQuery>(query, name, include, error);
		if (name == "UnitMovementClassEvent") return UnitTeamVisible<UnitMovementClassEventQuery>(query, name, include, error);
		if (name == "UnitCloakEvent") return UnitTeamVisible<UnitCloakEventQuery>(query, name, include, error);
		if (name == "UnitMoveEvent") return UnitTeamVisible<UnitMoveEventQuery>(query, name, include, error);
		if (name == "StockpileChanged") return UnitTeamVisible<StockpileChangedQuery>(query, name, include, error);
		if (name == "UnitDestroyed") {
			auto* value = Copy<UnitDestroyedQuery>(query, name, filteredQuery, error);
			if (value == nullptr) return false;
			include = WasmUiVisibility::IsTeamVisible(value->unitTeam);
			if (include) RedactAttacker(*value);
			return true;
		}
		if (name == "UnitDamaged") {
			auto* value = Copy<UnitDamagedQuery>(query, name, filteredQuery, error);
			if (value == nullptr) return false;
			include = WasmUiVisibility::IsTeamVisible(value->unitTeam);
			if (include) RedactAttacker(*value);
			return true;
		}
		if (name == "UnitCreated") {
			auto* value = Copy<UnitCreatedQuery>(query, name, filteredQuery, error);
			if (value == nullptr) return false;
			include = WasmUiVisibility::IsTeamVisible(value->unitTeam);
			if (include && value->builderID >= 0 && !VisibleUnit(value->builderID))
				value->builderID = -1;
			return true;
		}
		if (name == "UnitFromFactory") {
			auto* value = Copy<UnitFromFactoryQuery>(query, name, filteredQuery, error);
			if (value == nullptr) return false;
			include = WasmUiVisibility::IsTeamVisible(value->unitTeam);
			if (include && value->factoryID >= 0 && !VisibleUnit(value->factoryID)) {
				value->factoryID = -1;
				value->factoryDefID = -1;
			}
			return true;
		}
		if (name == "RenderUnitDestroyed")
			return UnitTeamVisible<RenderUnitDestroyedQuery>(query, name, include, error);
		if (name == "UnitUnitCollision") {
			const auto* value = Require<UnitUnitCollisionQuery>(query, name, error);
			if (value == nullptr) return false;
			include = VisibleUnit(value->colliderID) && VisibleUnit(value->collideeID);
			return true;
		}
		if (name == "UnitFeatureCollision") {
			const auto* value = Require<UnitFeatureCollisionQuery>(query, name, error);
			if (value == nullptr) return false;
			include = VisibleUnit(value->colliderID) &&
				WasmUiVisibility::FindFeature(value->collideeID) != nullptr;
			return true;
		}

		// No special visibility policy: send the original native query unchanged.
		return true;
	}

private:
	template<typename T>
	const T* Require(const void* query, std::string_view name, std::string& error) const
	{
		if (query != nullptr)
			return static_cast<const T*>(query);
		error = "Core UI callin " + std::string(name) + " received a null query";
		return nullptr;
	}

	template<typename T>
	T* Copy(const void* query, std::string_view name, const void*& filteredQuery,
		std::string& error)
	{
		const T* typed = Require<T>(query, name, error);
		if (typed == nullptr)
			return nullptr;
		storage = *typed;
		T& copy = std::get<T>(storage);
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

	template<typename T>
	bool FeatureCreatedDestroyed(const void* query, std::string_view name, bool& include,
		std::string& error) const
	{
		const auto* value = Require<T>(query, name, error);
		if (value == nullptr) return false;
		include = value->allyTeamID < 0 || WasmUiVisibility::IsAllyTeamVisible(value->allyTeamID);
		return true;
	}

	template<typename T>
	bool UnitLoadedUnloaded(const void* query, std::string_view name, bool& include,
		std::string& error) const
	{
		const auto* value = Require<T>(query, name, error);
		if (value == nullptr) return false;
		include = WasmUiVisibility::IsTeamVisible(value->unitTeam) ||
			WasmUiVisibility::IsTeamVisible(value->transportTeam);
		return true;
	}

	template<typename T>
	bool UnitTeamVisible(const void* query, std::string_view name, bool& include,
		std::string& error) const
	{
		const auto* value = Require<T>(query, name, error);
		if (value == nullptr) return false;
		include = WasmUiVisibility::IsTeamVisible(value->unitTeam);
		return true;
	}

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
};

} // namespace recoil::wasm::core
