/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "NativeInterfaceEventClient.h"

#include <array>
#include <cctype>
#include <cmath>
#include <cstring>
#include <limits>
#include <utility>

#include "NativeInterface/api/Constants.h"
#include "Game/Game.h"
#include "Game/Action.h"
#include "Game/GameHelper.h"
#include "Game/UI/KeySet.h"
#include "Game/UI/MiniMap.h"
#include "Rendering/GlobalRendering.h"
#include "Lua/LuaConfig.h"
#include "Lua/LuaMaterial.h"
#include "Sim/Features/Feature.h"
#include "Sim/Features/FeatureDef.h"
#include "Sim/Projectiles/Projectile.h"
#include "Sim/Projectiles/WeaponProjectiles/WeaponProjectile.h"
#include "Sim/Misc/Resource.h"
#include "Sim/Objects/SolidObject.h"
#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitDef.h"
#include "Sim/Units/BuildInfo.h"
#include "Sim/Units/CommandAI/Command.h"
#include "Sim/Units/CommandAI/CommandDescription.h"
#include "Sim/Weapons/Weapon.h"
#include "System/Log/ILog.h"
#include "System/BenchmarkCallins.h"
#include "System/Input/KeyInput.h"
#include "System/Platform/SDL1_keysym.h"
#include "System/Platform/SharedLib.h"
#include "System/Rectangle.h"
#include "System/float3.h"
#include "WasmInterface/WasmEnvironment.h"
#include "WasmInterface/WasmInterfaceSystem.h"
#include "WasmInterface/WasmTypedHost.h"
#include "NativeInterface/WasmUiVisibility.h"
#include "wasm/generated/WasmHostAdapter.h"

#include <SDL_keyboard.h>
#include <SDL_keycode.h>
#ifdef SYNCCHECK
#include "System/Sync/SyncChecker.h"
#endif

#define LOAD_SYMBOL(SymbolName)                                                               \
	{                                                                                           \
		m_##SymbolName##FuncPtr = m_sharedLib->FindAddressTyped<fptr::SymbolName##FuncPtr>(      \
			#SymbolName);                                                                          \
		if (m_##SymbolName##FuncPtr == nullptr)                                                   \
			LOG_L(L_ERROR, "Failed to load native module symbol " #SymbolName);                     \
	}

namespace {
	// The public native command-option ABI uses a compact bit layout which is
	// intentionally different from Command's internal simulation bitfield.
	// Constants.h exposes these names to C and bindgen, not to the C++ engine
	// build, so keep the C++ conversion beside the callin boundary explicit.
	constexpr uint8_t NATIVE_CMD_OPT_INTERNAL = (1 << 0);
	constexpr uint8_t NATIVE_CMD_OPT_RIGHT = (1 << 1);
	constexpr uint8_t NATIVE_CMD_OPT_SHIFT = (1 << 2);
	constexpr uint8_t NATIVE_CMD_OPT_CTRL = (1 << 3);
	constexpr uint8_t NATIVE_CMD_OPT_ALT = (1 << 4);
	constexpr uint8_t NATIVE_CMD_OPT_META = (1 << 5);

	struct LuaMousePosition {
		int x;
		int y;
	};

	LuaMousePosition ToLuaMousePosition(int x, int y)
	{
		// CMouseHandler and CEventHandler use renderer coordinates here:
		// x is screen-relative and y is top-origin.  CLuaHandle converts
		// both before invoking Lua callins, so native callbacks use the same
		// view-relative, bottom-origin contract.
		if (globalRendering == nullptr)
			return {x, y};

		return {
			x - globalRendering->viewPosX,
			globalRendering->viewSizeY - y - 1,
		};
	}

	std::string ToWitFieldName(std::string_view value)
	{
		std::string result;
		result.reserve(value.size() + value.size() / 3);
		for (std::size_t index = 0; index < value.size(); ++index) {
			const unsigned char character = static_cast<unsigned char>(value[index]);
			const bool uppercase = std::isupper(character) != 0;
			const bool previousUppercase = index > 0 &&
				std::isupper(static_cast<unsigned char>(value[index - 1])) != 0;
			const bool nextLowercase = index + 1 < value.size() &&
				std::islower(static_cast<unsigned char>(value[index + 1])) != 0;
			if (uppercase && index != 0 && (!previousUppercase || nextLowercase))
				result.push_back('-');
			result.push_back(static_cast<char>(std::tolower(character)));
		}
		return result;
	}

	const WasmValue* FindWasmField(const WasmValueRecord& record, std::string_view name)
	{
		const auto iter = record.find(std::string(name));
		if (iter != record.end())
			return &iter->second;
		const std::string witName = ToWitFieldName(name);
		for (const auto& [fieldName, fieldValue] : record) {
			if (fieldName == witName)
				return &fieldValue;
		}
		return nullptr;
	}

	WasmValue* FindWasmField(WasmValueRecord& record, std::string_view name)
	{
		auto iter = record.find(std::string(name));
		if (iter != record.end())
			return &iter->second;
		const std::string witName = ToWitFieldName(name);
		for (auto& [fieldName, fieldValue] : record) {
			if (fieldName == witName)
				return &fieldValue;
		}
		return nullptr;
	}

	bool ReadWasmBoolField(const WasmValueRecord& record, std::string_view name, bool& value)
	{
		const WasmValue* field = FindWasmField(record, name);
		const auto* boolean = field == nullptr ? nullptr : std::get_if<bool>(&field->storage);
		if (boolean == nullptr)
			return false;
		value = *boolean;
		return true;
	}

	bool ReadWasmFloatField(const WasmValueRecord& record, std::string_view name, float& value)
	{
		const WasmValue* field = FindWasmField(record, name);
		const auto* number = field == nullptr ? nullptr : std::get_if<double>(&field->storage);
		if (number == nullptr || !std::isfinite(*number) ||
			*number < -std::numeric_limits<float>::max() ||
			*number > std::numeric_limits<float>::max())
			return false;
		value = static_cast<float>(*number);
		return true;
	}

	bool ReadWasmIntField(const WasmValueRecord& record, std::string_view name, int& value)
	{
		const WasmValue* field = FindWasmField(record, name);
		if (field == nullptr)
			return false;
		if (const auto* signedValue = std::get_if<std::int64_t>(&field->storage)) {
			if (*signedValue < std::numeric_limits<int>::min() ||
				*signedValue > std::numeric_limits<int>::max())
				return false;
			value = static_cast<int>(*signedValue);
			return true;
		}
		if (const auto* unsignedValue = std::get_if<std::uint64_t>(&field->storage)) {
			if (*unsignedValue > static_cast<std::uint64_t>(std::numeric_limits<int>::max()))
				return false;
			value = static_cast<int>(*unsignedValue);
			return true;
		}
		return false;
	}

	const WasmValueRecord* WasmResultRecord(const WasmValue& value)
	{
		return std::get_if<WasmValueRecord>(&value.storage);
	}

	bool SetWasmIntField(WasmValueRecord& record, std::string_view name, int value)
	{
		WasmValue* field = FindWasmField(record, name);
		if (field == nullptr)
			return false;
		field->storage = static_cast<std::int64_t>(value);
		return true;
	}

	void RedactAttacker(WasmValueRecord& record)
	{
		int attackerID = -1;
		if (!ReadWasmIntField(record, "attackerID", attackerID) || attackerID < 0)
			return;

		const CUnit* attacker = WasmUiVisibility::FindUnit(
			attackerID, WasmUiVisibility::UnitAccess::Visible);
		if (attacker == nullptr) {
			SetWasmIntField(record, "attackerID", -1);
			SetWasmIntField(record, "attackerDefID", -1);
			SetWasmIntField(record, "attackerTeam", -1);
			return;
		}
		if (!WasmUiVisibility::IsUnitTyped(attacker))
			SetWasmIntField(record, "attackerDefID", -1);
		else if (const UnitDef* def = WasmUiVisibility::EffectiveUnitDef(attacker))
			SetWasmIntField(record, "attackerDefID", def->id);
	}

	// The native event client has full-read access so that it can continue to
	// serve native modules. Wasm UI modules must receive the same filtered
	// event stream as CLuaUI; this copy is made after native serialization and
	// before the value enters the UI world.
	bool SanitizeUiCallin(std::string_view name, WasmValue& value)
	{
		// NativeInterface dispatches legacy callin aliases by their source
		// spelling, while the generated inventory aggregates them under one
		// canonical query. Apply the visibility rule to both spellings.
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

		auto* record = std::get_if<WasmValueRecord>(&value.storage);
		if (record == nullptr)
			return true;

		auto unitTeamVisible = [&]() {
			int team = -1;
			return ReadWasmIntField(*record, "unitTeam", team) &&
				WasmUiVisibility::IsTeamVisible(team);
		};
		auto eitherTeamVisible = [&](std::string_view first, std::string_view second) {
			int firstTeam = -1;
			int secondTeam = -1;
			const bool haveFirst = ReadWasmIntField(*record, first, firstTeam);
			const bool haveSecond = ReadWasmIntField(*record, second, secondTeam);
			return (haveFirst && WasmUiVisibility::IsTeamVisible(firstTeam)) ||
				(haveSecond && WasmUiVisibility::IsTeamVisible(secondTeam));
		};
		auto visibleUnit = [&](std::string_view field) {
			int unitID = -1;
			return ReadWasmIntField(*record, field, unitID) && unitID >= 0 &&
				WasmUiVisibility::FindUnit(unitID, WasmUiVisibility::UnitAccess::Visible) != nullptr;
		};
		auto visiblePosition = [&]() {
			const WasmValue* positionValue = FindWasmField(*record, "pos");
			const auto* position = positionValue == nullptr ? nullptr :
				std::get_if<WasmValueRecord>(&positionValue->storage);
			if (position == nullptr)
				return false;
			float x = 0.0f;
			float y = 0.0f;
			float z = 0.0f;
			return ReadWasmFloatField(*position, "x", x) &&
				ReadWasmFloatField(*position, "y", y) &&
				ReadWasmFloatField(*position, "z", z) &&
				WasmUiVisibility::IsPositionVisible(float3{x, y, z});
		};

		if (name == "DefaultCommand") {
			int unitID = -1;
			int featureID = -1;
			if (ReadWasmIntField(*record, "unitID", unitID) && unitID >= 0 && !visibleUnit("unitID"))
				return false;
			if (ReadWasmIntField(*record, "featureID", featureID) && featureID >= 0 &&
				WasmUiVisibility::FindFeature(featureID) == nullptr)
				return false;
			return true;
		}

		if (name == "WorldTooltip") {
			int kind = 0;
			ReadWasmIntField(*record, "kind", kind);
			if (kind == 1)
				return visibleUnit("unitID");
			if (kind == 2) {
				int featureID = -1;
				return ReadWasmIntField(*record, "featureID", featureID) &&
					WasmUiVisibility::FindFeature(featureID) != nullptr;
			}
			if (kind == 0) {
				int unitID = -1;
				int featureID = -1;
				const bool unitOK = !ReadWasmIntField(*record, "unitID", unitID) ||
					unitID < 0 || visibleUnit("unitID");
				const bool featureOK = !ReadWasmIntField(*record, "featureID", featureID) ||
					featureID < 0 || WasmUiVisibility::FindFeature(featureID) != nullptr;
				return unitOK && featureOK;
			}
			return true;
		}

		if (name == "FeatureCreated" || name == "FeatureDestroyed") {
			int allyTeam = -1;
			if (!ReadWasmIntField(*record, "allyTeamID", allyTeam))
				return false;
			return allyTeam < 0 || WasmUiVisibility::IsAllyTeamVisible(allyTeam);
		}

		if (name == "FeatureMoved" || name == "FeatureDamaged") {
			int featureID = -1;
			if (!ReadWasmIntField(*record, "featureID", featureID))
				return false;
			const CFeature* feature = WasmUiVisibility::FindFeature(featureID);
			if (feature == nullptr ||
				(feature->allyteam >= 0 && !WasmUiVisibility::IsAllyTeamVisible(feature->allyteam)))
				return false;
			if (name == "FeatureDamaged")
				RedactAttacker(*record);
			return true;
		}

		if (name == "ProjectileCreated" || name == "ProjectileDestroyed" ||
			name == "ProjectileEvent") {
			int ownerID = -1;
			if (!ReadWasmIntField(*record, "ownerID", ownerID) || ownerID < 0)
				return true;
			return WasmUiVisibility::FindUnit(ownerID, WasmUiVisibility::UnitAccess::Ally) != nullptr;
		}

		if (name == "Explosion") {
			if (!WasmUiVisibility::FullRead() && !visiblePosition())
				return false;
			int ownerID = -1;
			if (ReadWasmIntField(*record, "ownerID", ownerID) && ownerID >= 0 &&
				WasmUiVisibility::FindUnit(ownerID, WasmUiVisibility::UnitAccess::Visible) == nullptr)
				SetWasmIntField(*record, "ownerID", -1);
			int projectileID = -1;
			if (ReadWasmIntField(*record, "projectileID", projectileID) && projectileID >= 0 &&
				WasmUiVisibility::FindProjectile(projectileID) == nullptr)
				SetWasmIntField(*record, "projectileID", -1);
			return true;
		}

		if (name == "UnitLosEvent") {
			int allyTeam = -1;
			if (!ReadWasmIntField(*record, "allyTeam", allyTeam) ||
				!WasmUiVisibility::IsAllyTeamVisible(allyTeam))
				return false;
			if (!WasmUiVisibility::FullRead()) {
				SetWasmIntField(*record, "allyTeam", -1);
				SetWasmIntField(*record, "unitDefID", -1);
			}
			return true;
		}

		if (name == "UnitSeismicPing") {
			int unitID = -1;
			int allyTeam = -1;
			if (!ReadWasmIntField(*record, "unitID", unitID) ||
				!ReadWasmIntField(*record, "allyTeam", allyTeam) ||
				!WasmUiVisibility::IsAllyTeamVisible(allyTeam))
				return false;
			if (!WasmUiVisibility::FullRead()) {
				// LuaUI receives radar pings for its ally team, including pings
				// emitted by enemy units.  It suppresses a ping only when the
				// source unit is already in LOS, and omits the source identity
				// for the non-full-read form.
				const CUnit* unit = WasmUiVisibility::FindUnit(
					unitID, WasmUiVisibility::UnitAccess::Visible);
				if (unit != nullptr && WasmUiVisibility::IsUnitInLos(unit))
					return false;
				SetWasmIntField(*record, "allyTeam", -1);
				SetWasmIntField(*record, "unitID", -1);
				SetWasmIntField(*record, "unitDefID", -1);
			}
			return true;
		}

		if (name == "UnitLoaded" || name == "UnitUnloaded")
			return eitherTeamVisible("unitTeam", "transportTeam");

		if (name == "UnitTaken") {
			int team = -1;
			return ReadWasmIntField(*record, "oldTeam", team) &&
				WasmUiVisibility::IsTeamVisible(team);
		}
		if (name == "UnitGiven") {
			int team = -1;
			return ReadWasmIntField(*record, "newTeam", team) &&
				WasmUiVisibility::IsTeamVisible(team);
		}

		if (name == "UnitFinished" || name == "UnitReverseBuilt" ||
			name == "UnitConstructionDecayed" || name == "UnitIdle" ||
			name == "UnitCommand" || name == "UnitCmdDone" ||
			name == "UnitStunned" || name == "UnitExperience" ||
			name == "UnitHarvestStorageFull" || name == "UnitMovementClassEvent" ||
			name == "UnitCloakEvent" || name == "UnitMoveEvent" ||
			name == "StockpileChanged")
			return unitTeamVisible();

		if (name == "UnitDestroyed" || name == "UnitDamaged") {
			if (!unitTeamVisible())
				return false;
			RedactAttacker(*record);
			return true;
		}

		if (name == "UnitCreated") {
			if (!unitTeamVisible())
				return false;
			int builderID = -1;
			if (ReadWasmIntField(*record, "builderID", builderID) && builderID >= 0 &&
				!visibleUnit("builderID"))
				SetWasmIntField(*record, "builderID", -1);
			return true;
		}

		if (name == "UnitFromFactory") {
			if (!unitTeamVisible())
				return false;
			int factoryID = -1;
			if (ReadWasmIntField(*record, "factoryID", factoryID) && factoryID >= 0 &&
				!visibleUnit("factoryID")) {
				SetWasmIntField(*record, "factoryID", -1);
				SetWasmIntField(*record, "factoryDefID", -1);
			}
			return true;
		}

		if (name == "RenderUnitDestroyed")
			return unitTeamVisible();

		if (name == "UnitUnitCollision")
			return visibleUnit("colliderID") && visibleUnit("collideeID");
		if (name == "UnitFeatureCollision") {
			int featureID = -1;
			return visibleUnit("colliderID") &&
				ReadWasmIntField(*record, "collideeID", featureID) &&
				WasmUiVisibility::FindFeature(featureID) != nullptr;
		}

		return true;
	}

	class ScopedNativeSyncedCode {
	public:
		ScopedNativeSyncedCode(bool synced)
			: synced(synced)
		{
		#ifdef SYNCCHECK
			if (synced)
				CSyncChecker::EnterSyncedCode();
		#endif
		}

		~ScopedNativeSyncedCode()
		{
		#ifdef SYNCCHECK
			if (synced)
				CSyncChecker::LeaveSyncedCode();
		#endif
		}

	private:
		bool synced;
	};
}

NativeInterfaceEventClient::NativeInterfaceEventClient(NativeInterface* nativeInterface,
	SharedLib* sharedLib, WasmInterfaceSystem* wasmSystem)
	: CEventClient("[NativeInterfaceEventClient]", 23253, false)
	, m_nativeInterface(nativeInterface)
	, m_sharedLib(sharedLib)
	, m_wasmSystem(wasmSystem)
{
}

bool NativeInterfaceEventClient::DispatchWasmCallin(std::string_view name,
	const void* query, bool synced, WasmValue* result, void* nativeResult)
{
	// Core is the production transport. Keep it ahead of both the historical
	// typed transport and the Component/WasmValue path so supported callins
	// never pay query serialization or heap-based fan-out costs.
	if (m_wasmSystem != nullptr) {
		bool coreHandled = false;
		std::string coreError;
		if (!WasmInterfaceSystem::DispatchActiveCoreCallin(
				name, query, nativeResult, coreHandled, coreError)) {
			if (!coreError.empty()) {
				LOG_L(L_ERROR, "Core Wasm callin %s failed: %s",
					std::string(name).c_str(), coreError.c_str());
			}
			return false;
		}
		if (coreHandled) {
			// Direct transports write result structs rather than constructing a
			// WasmValue. A non-null nativeResult means the caller requested one.
			if (result != nullptr)
				*result = WasmValue::Unit();
			return nativeResult != nullptr;
		}
	}

	// The typed Rust host is a raw historical reference transport for the same
	// guests. It is also direct-result: handled value callins write nativeResult.
	if (WasmTypedHost::Enabled() && WasmTypedHost::AnyActive()) {
		std::string typedError;
		if (WasmTypedHost::DispatchCallin(name, query, nativeResult, typedError)) {
			if (!typedError.empty())
				LOG_L(L_WARNING, "%s", typedError.c_str());
			if (result != nullptr)
				*result = WasmValue::Unit();
			return nativeResult != nullptr;
		}
	}

	if (m_wasmSystem == nullptr)
		return false;

	static constexpr std::array<WasmEnvironment, 2> syncedEnvironments{
		WasmEnvironment::RulesSynced, WasmEnvironment::GaiaSynced};
	static constexpr std::array<WasmEnvironment, 2> unsyncedEnvironments{
		WasmEnvironment::RulesUnsynced, WasmEnvironment::GaiaUnsynced};
	const auto& primaryEnvironments = synced ? syncedEnvironments : unsyncedEnvironments;
	const bool hasComponentModules =
		m_wasmSystem->HasComponentModules(primaryEnvironments[0]) ||
		m_wasmSystem->HasComponentModules(primaryEnvironments[1]) ||
		m_wasmSystem->HasComponentModules(WasmEnvironment::UI);
	if (!hasComponentModules)
		return false;

	WasmValue value;
	std::string conversionError;
	if (!recoil::wasm::generated::SerializeCallinQuery(name, query, value, conversionError)) {
		// Manual/opaque callin shapes are intentionally skipped until their
		// explicit adapter exists. A malformed generated conversion is still
		// useful diagnostic information and should not be silent.
		if (conversionError != "native callin query requires an explicit Wasm serializer") {
			LOG_L(L_WARNING, "Could not serialize native callin %s for Wasm: %s",
				std::string(name).c_str(), conversionError.c_str());
		}
		return false;
	}
	// UI is an unsynced environment, but LuaUI also receives lifecycle events
	// that are dispatched from the engine's synced path (for example
	// GameStart). Give it an owned, visibility-filtered copy of the query;
	// the generated callin environment mask remains authoritative.
	std::vector<WasmInterfaceSystem::CallinInvocation> invocations;
	invocations.reserve(3);
	// An environment with no Component module needs no query copy at all.
	for (const WasmEnvironment environment : primaryEnvironments) {
		if (!m_wasmSystem->HasComponentModules(environment))
			continue;
		invocations.push_back({environment, {value}});
	}
	if (m_wasmSystem->HasComponentModules(WasmEnvironment::UI)) {
		WasmUiVisibility::ScopedContext uiContext(true);
		WasmValue uiValue = value;
		// EventHandler discards return values from unsynced Lua clients for
		// these synced control callins.  Keep dispatching the UI callback for
		// parity, but do not let it influence the simulation result.
		const bool uiContributesResult = name != "Explosion" &&
			name != "UnitUnitCollision" && name != "UnitFeatureCollision";
		if (SanitizeUiCallin(name, uiValue))
			invocations.push_back({WasmEnvironment::UI, {std::move(uiValue)}, uiContributesResult});
	}
	WasmValue aggregate;
	std::string error;
	if (!m_wasmSystem->DispatchCallin(name, invocations, aggregate, error)) {
		if (!error.empty())
			LOG_L(L_ERROR, "Wasm callin %s failed: %s", std::string(name).c_str(), error.c_str());
		return false;
	}
	if (aggregate.IsUnit())
		return false;
	if (result != nullptr)
		*result = std::move(aggregate);
	return true;
}

bool NativeInterfaceEventClient::DispatchWasmBoolCallin(std::string_view name,
	const void* query, bool synced, bool& result)
{
	WasmValue wasmResult;
	BoolCallinResult directResult = {.error = nullptr, .value = false};
	if (!DispatchWasmCallin(name, query, synced, &wasmResult, &directResult))
		return false;
	if (wasmResult.IsUnit()) {
		if (directResult.error != nullptr) {
			LOG_L(L_WARNING, "Wasm callin %s returned a direct boolean error: %s",
				std::string(name).c_str(), directResult.error->message);
			return false;
		}
		result = directResult.value;
		return true;
	}
	const auto* record = std::get_if<WasmValueRecord>(&wasmResult.storage);
	if (record == nullptr) {
		LOG_L(L_WARNING, "Wasm callin %s returned a non-record boolean result",
			std::string(name).c_str());
		return false;
	}
	const auto iter = record->find("value");
	const auto* value = iter == record->end() ? nullptr :
		std::get_if<bool>(&iter->second.storage);
	if (value == nullptr) {
		LOG_L(L_WARNING, "Wasm callin %s returned an invalid boolean result",
			std::string(name).c_str());
		return false;
	}
	result = *value;
	return true;
}

bool NativeInterfaceEventClient::DispatchWasmStringCallin(std::string_view name,
	const void* query, bool synced, std::string& result)
{
	WasmValue wasmResult;
	if (!DispatchWasmCallin(name, query, synced, &wasmResult))
		return false;
	const auto* record = std::get_if<WasmValueRecord>(&wasmResult.storage);
	if (record == nullptr) {
		LOG_L(L_WARNING, "Wasm callin %s returned a non-record string result",
			std::string(name).c_str());
		return false;
	}
	const auto iter = record->find("value");
	const auto* value = iter == record->end() ? nullptr :
		std::get_if<std::string>(&iter->second.storage);
	if (value == nullptr) {
		LOG_L(L_WARNING, "Wasm callin %s returned an invalid string result",
			std::string(name).c_str());
		return false;
	}
	result = *value;
	return true;
}

bool NativeInterfaceEventClient::DispatchWasmIntegerCallin(std::string_view name,
	const void* query, bool synced, int& result)
{
	WasmValue wasmResult;
	if (!DispatchWasmCallin(name, query, synced, &wasmResult))
		return false;
	const auto* record = std::get_if<WasmValueRecord>(&wasmResult.storage);
	if (record == nullptr) {
		LOG_L(L_WARNING, "Wasm callin %s returned a non-record integer result",
			std::string(name).c_str());
		return false;
	}
	const auto iter = record->find("value");
	if (iter == record->end()) {
		LOG_L(L_WARNING, "Wasm callin %s returned an integer result without a value",
			std::string(name).c_str());
		return false;
	}
	if (const auto* value = std::get_if<std::int64_t>(&iter->second.storage)) {
		if (*value < std::numeric_limits<int>::min() || *value > std::numeric_limits<int>::max())
			return false;
		result = static_cast<int>(*value);
		return true;
	}
	if (const auto* value = std::get_if<std::uint64_t>(&iter->second.storage)) {
		if (*value > static_cast<std::uint64_t>(std::numeric_limits<int>::max()))
			return false;
		result = static_cast<int>(*value);
		return true;
	}
	LOG_L(L_WARNING, "Wasm callin %s returned a non-integer value", std::string(name).c_str());
	return false;
}

void NativeInterfaceEventClient::LoadSymbols() {
	LOG("Loading symbols from native module...");

	LOAD_SYMBOL(InitializeNativeModule);
	LOAD_SYMBOL(Load);
	LOAD_SYMBOL(DownloadFailed);
	LOAD_SYMBOL(DownloadFinished);
	LOAD_SYMBOL(DownloadProgress);
	LOAD_SYMBOL(DownloadQueued);
	LOAD_SYMBOL(DownloadStarted);
	LOAD_SYMBOL(FeatureCreated);
	LOAD_SYMBOL(FeatureDestroyed);
	LOAD_SYMBOL(GameID);
	LOAD_SYMBOL(GamePaused);
	LOAD_SYMBOL(GamePreload);
	LOAD_SYMBOL(GameStart);
	LOAD_SYMBOL(GameOver);
	LOAD_SYMBOL(GameFrame);
	LOAD_SYMBOL(GameFramePost);
	LOAD_SYMBOL(PlayerAdded);
	LOAD_SYMBOL(PlayerChanged);
	LOAD_SYMBOL(PlayerRemoved);
	LOAD_SYMBOL(RenderUnitDestroyed);
	LOAD_SYMBOL(Shutdown);
	LOAD_SYMBOL(TeamChanged);
	LOAD_SYMBOL(TeamDied);
	LOAD_SYMBOL(UnitCreated);
	LOAD_SYMBOL(UnitDestroyed);
	LOAD_SYMBOL(UnitExperience);
	LOAD_SYMBOL(UnitFinished);
	LOAD_SYMBOL(UnitReverseBuilt);
	LOAD_SYMBOL(UnitConstructionDecayed);
	LOAD_SYMBOL(UnitFromFactory);
	LOAD_SYMBOL(UnitGiven);
	LOAD_SYMBOL(UnitIdle);
	LOAD_SYMBOL(UnitCommand);
	LOAD_SYMBOL(CommandFallback);
	LOAD_SYMBOL(AllowCommand);
	LOAD_SYMBOL(AllowUnitCreation);
	LOAD_SYMBOL(AllowUnitTransfer);
	LOAD_SYMBOL(AllowUnitBuildStep);
	LOAD_SYMBOL(AllowUnitCaptureStep);
	LOAD_SYMBOL(AllowUnitTransport);
	LOAD_SYMBOL(AllowUnitTransportLoad);
	LOAD_SYMBOL(AllowUnitTransportUnload);
	LOAD_SYMBOL(AllowUnitCloak);
	LOAD_SYMBOL(AllowUnitDecloak);
	LOAD_SYMBOL(AllowUnitKamikaze);
	LOAD_SYMBOL(AllowFeatureCreation);
	LOAD_SYMBOL(AllowFeatureBuildStep);
	LOAD_SYMBOL(AllowResourceLevel);
	LOAD_SYMBOL(AllowResourceTransfer);
	LOAD_SYMBOL(ResourceExcess);
	LOAD_SYMBOL(AllowDirectUnitControl);
	LOAD_SYMBOL(AllowBuilderHoldFire);
	LOAD_SYMBOL(AllowStartPosition);
	LOAD_SYMBOL(TerraformComplete);
	LOAD_SYMBOL(MoveCtrlNotify);
	LOAD_SYMBOL(AllowWeaponTargetCheck);
	LOAD_SYMBOL(AllowWeaponTarget);
	LOAD_SYMBOL(AllowWeaponInterceptTarget);
	LOAD_SYMBOL(UnitPreDamaged);
	LOAD_SYMBOL(FeaturePreDamaged);
	LOAD_SYMBOL(ShieldPreDamaged);
	LOAD_SYMBOL(UnitCmdDone);
	LOAD_SYMBOL(UnitDamaged);
	LOAD_SYMBOL(UnitHarvestStorageFull);
	LOAD_SYMBOL(UnitSeismicPing);
	LOAD_SYMBOL(UnitEnteredRadar);
	LOAD_SYMBOL(UnitEnteredLos);
	LOAD_SYMBOL(UnitLeftRadar);
	LOAD_SYMBOL(UnitLeftLos);
	LOAD_SYMBOL(UnitEnteredUnderwater);
	LOAD_SYMBOL(UnitEnteredWater);
	LOAD_SYMBOL(UnitEnteredAir);
	LOAD_SYMBOL(UnitLeftUnderwater);
	LOAD_SYMBOL(UnitLeftWater);
	LOAD_SYMBOL(UnitLeftAir);
	LOAD_SYMBOL(UnitLoaded);
	LOAD_SYMBOL(UnitStunned);
	LOAD_SYMBOL(UnitTaken);
	LOAD_SYMBOL(UnitUnloaded);
	LOAD_SYMBOL(UnitCloaked);
	LOAD_SYMBOL(UnitDecloaked);
	LOAD_SYMBOL(UnitMoved);
	LOAD_SYMBOL(UnitMoveFailed);
	LOAD_SYMBOL(UnitArrivedAtGoal);
	LOAD_SYMBOL(UnitUnitCollision);
	LOAD_SYMBOL(UnitFeatureCollision);
	LOAD_SYMBOL(FeatureMoved);
	LOAD_SYMBOL(FeatureDamaged);
	LOAD_SYMBOL(ProjectileCreated);
	LOAD_SYMBOL(ProjectileDestroyed);
	LOAD_SYMBOL(Explosion);
	LOAD_SYMBOL(HandleLuaMsg);
	LOAD_SYMBOL(HandleLuaCall);
	LOAD_SYMBOL(Update);
	LOAD_SYMBOL(Save);
	LOAD_SYMBOL(DrawScreen);
	LOAD_SYMBOL(DrawGenesis);
	LOAD_SYMBOL(DrawWorld);
	LOAD_SYMBOL(DrawWorldPreUnit);
	LOAD_SYMBOL(DrawPreDecals);
	LOAD_SYMBOL(DrawWorldPreParticles);
	LOAD_SYMBOL(DrawWaterPost);
	LOAD_SYMBOL(DrawWorldShadow);
	LOAD_SYMBOL(DrawShadowPassTransparent);
	LOAD_SYMBOL(DrawWorldReflection);
	LOAD_SYMBOL(DrawWorldRefraction);
	LOAD_SYMBOL(DrawGroundPreForward);
	LOAD_SYMBOL(DrawGroundPostForward);
	LOAD_SYMBOL(DrawGroundPreDeferred);
	LOAD_SYMBOL(DrawGroundDeferred);
	LOAD_SYMBOL(DrawGroundPostDeferred);
	LOAD_SYMBOL(DrawUnitsPostDeferred);
	LOAD_SYMBOL(DrawFeaturesPostDeferred);
	LOAD_SYMBOL(DrawScreenEffects);
	LOAD_SYMBOL(DrawScreenPost);
	LOAD_SYMBOL(DrawInMiniMap);
	LOAD_SYMBOL(DrawInMiniMapBackground);
	LOAD_SYMBOL(DrawBuildSquare);
	LOAD_SYMBOL(DrawOpaqueUnitsLua);
	LOAD_SYMBOL(DrawOpaqueFeaturesLua);
	LOAD_SYMBOL(DrawAlphaUnitsLua);
	LOAD_SYMBOL(DrawAlphaFeaturesLua);
	LOAD_SYMBOL(DrawShadowUnitsLua);
	LOAD_SYMBOL(DrawShadowFeaturesLua);
	LOAD_SYMBOL(DrawUnit);
	LOAD_SYMBOL(DrawFeature);
	LOAD_SYMBOL(DrawShield);
	LOAD_SYMBOL(DrawProjectile);
	LOAD_SYMBOL(DrawMaterial);
	LOAD_SYMBOL(LastMessagePosition);
	LOAD_SYMBOL(UnsyncedHeightMapUpdate);
	LOAD_SYMBOL(KeyMapChanged);
	LOAD_SYMBOL(KeyPress);
	LOAD_SYMBOL(KeyRelease);
	LOAD_SYMBOL(TextInput);
	LOAD_SYMBOL(TextEditing);
	LOAD_SYMBOL(MouseMove);
	LOAD_SYMBOL(MousePress);
	LOAD_SYMBOL(MouseRelease);
	LOAD_SYMBOL(MouseWheel);
	LOAD_SYMBOL(IsAbove);
	LOAD_SYMBOL(GetTooltip);
	LOAD_SYMBOL(DefaultCommand);
	LOAD_SYMBOL(ActiveCommandChanged);
	LOAD_SYMBOL(CameraRotationChanged);
	LOAD_SYMBOL(CameraPositionChanged);
	LOAD_SYMBOL(CommandNotify);
	LOAD_SYMBOL(AddConsoleLine);
	LOAD_SYMBOL(GroupChanged);
	LOAD_SYMBOL(MiniMapRotationChanged);
	LOAD_SYMBOL(MiniMapStateChanged);
	LOAD_SYMBOL(MiniMapGeometryChanged);
	LOAD_SYMBOL(GameSetup);
	LOAD_SYMBOL(WorldTooltip);
	LOAD_SYMBOL(MapDrawCmd);
	LOAD_SYMBOL(ViewResize);
	LOAD_SYMBOL(SunChanged);
	LOAD_SYMBOL(FontsChanged);
	LOAD_SYMBOL(GameProgress);
	LOAD_SYMBOL(StockpileChanged);
	LOAD_SYMBOL(CollectGarbage);
	LOAD_SYMBOL(Pong);
}

void* NativeInterfaceEventClient::Initialize() {
	if (m_InitializeNativeModuleFuncPtr == nullptr) {
		LOG_L(L_ERROR, "InitializeNativeModule function not loaded");
		return nullptr;
	}

	LOG("Initializing native module...");

	InitializeNativeModuleQuery query = {};
	query.hostVersionMajor = NATIVE_API_MAJOR(NATIVE_API_CURRENT_VERSION);
	query.hostVersionMinor = NATIVE_API_MINOR(NATIVE_API_CURRENT_VERSION);
	query.hostVersionPatch = NATIVE_API_PATCH(NATIVE_API_CURRENT_VERSION);

	InitializeNativeModuleResult result = {};
	m_InitializeNativeModuleFuncPtr(m_nativeInterface, &query, &result);

	if (result.error != nullptr) {
		LOG_L(L_ERROR, "Failed to initialize native module: %s", result.error->message);
		return nullptr;
	}

	LOG("Native module initialized successfully (module version: %u.%u.%u)",
		result.moduleVersionMajor, result.moduleVersionMinor, result.moduleVersionPatch);

	m_moduleData = result.moduleData;
	m_initialized = true;
	return m_moduleData;
}

void NativeInterfaceEventClient::Shutdown() {
	if (!m_initialized)
		return;

	if (m_ShutdownFuncPtr != nullptr) {
		ShutdownQuery query = {};
		ShutdownResult result = {};
		m_ShutdownFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		if (result.error != nullptr)
			LOG_L(L_ERROR, "Native module shutdown failed: %s", result.error->message);
	}

	m_moduleData = nullptr;
	m_initialized = false;
}

static NativeCallinCommand ToNativeCallinCommand(const Command& command)
{
	const uint8_t engineOptions = command.GetOpts();
	uint8_t nativeOptions = 0;
	if (engineOptions & INTERNAL_ORDER) nativeOptions |= NATIVE_CMD_OPT_INTERNAL;
	if (engineOptions & RIGHT_MOUSE_KEY) nativeOptions |= NATIVE_CMD_OPT_RIGHT;
	if (engineOptions & SHIFT_KEY) nativeOptions |= NATIVE_CMD_OPT_SHIFT;
	if (engineOptions & CONTROL_KEY) nativeOptions |= NATIVE_CMD_OPT_CTRL;
	if (engineOptions & ALT_KEY) nativeOptions |= NATIVE_CMD_OPT_ALT;
	if (engineOptions & META_KEY) nativeOptions |= NATIVE_CMD_OPT_META;
	return {
		.id = command.GetID(), .timeOut = command.GetTimeOut(), .pageIndex = command.GetpageIndex(),
		.numParams = command.GetNumParams(), .tag = command.GetTag(), .options = nativeOptions,
		.params = command.GetParams()
	};
}

void NativeInterfaceEventClient::Load(IArchive* archive) {
	ArchiveCallinQuery query = {.archive = archive};
	DispatchWasmCallin("Load", &query, true);
	if (m_LoadFuncPtr) { ArchiveCallinResult result = {}; m_LoadFuncPtr(m_nativeInterface, m_moduleData, &query, &result); }
}

void NativeInterfaceEventClient::GamePreload() { GamePreloadQuery query = {}; DispatchWasmCallin("GamePreload", &query, true); if (m_GamePreloadFuncPtr) { GamePreloadResult result = {}; m_GamePreloadFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::GameStart() { GameStartQuery query = {}; DispatchWasmCallin("GameStart", &query, true); if (m_GameStartFuncPtr) { GameStartResult result = {}; m_GameStartFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }

void NativeInterfaceEventClient::GameOver(const std::vector<unsigned char>& winningAllyTeams) {
	GameOverEventQuery query = {.winningAllyTeams = winningAllyTeams.data(), .count = static_cast<uint32_t>(winningAllyTeams.size())};
	DispatchWasmCallin("GameOver", &query, true);
	if (m_GameOverFuncPtr) { GameOverEventResult result = {}; m_GameOverFuncPtr(m_nativeInterface, m_moduleData, &query, &result); }
}

void NativeInterfaceEventClient::GameFrame(int gameFrame) {
	GameFrameQuery query = {.gameFrame = gameFrame};
	const auto wasmToken = spring::benchmark_callins::Begin("wasm", spring::benchmark_callins::GameFrameTestName());
	DispatchWasmCallin("GameFrame", &query, true);
	spring::benchmark_callins::End(wasmToken);
	const bool benchmarkUnimplemented = spring::benchmark_callins::IsCase("callins") && spring::benchmark_callins::IsVariant("unimplemented");
	const auto nativeToken = benchmarkUnimplemented ? spring::benchmark_callins::Begin("native", "callin_unimplemented") : spring::benchmark_callins::Token{};
	if (!benchmarkUnimplemented && m_GameFrameFuncPtr) {
		const auto implementedToken = spring::benchmark_callins::Begin("native", spring::benchmark_callins::GameFrameTestName());
		GameFrameResult result = {};
		m_GameFrameFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		spring::benchmark_callins::End(implementedToken);
	}
	spring::benchmark_callins::End(nativeToken);
}

void NativeInterfaceEventClient::GameFramePost(int gameFrame) { GameFramePostQuery query = {.gameFrame = gameFrame}; DispatchWasmCallin("GameFramePost", &query, true); if (m_GameFramePostFuncPtr) { GameFramePostResult result = {}; m_GameFramePostFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }

void NativeInterfaceEventClient::Update() {
	UpdateQuery query = {.deltaSeconds = (game != nullptr) ? game->updateDeltaSeconds : 0.0f};
	const auto wasmToken = spring::benchmark_callins::Begin("wasm", "callin_update");
	DispatchWasmCallin("Update", &query, false);
	spring::benchmark_callins::End(wasmToken);
	if (m_UpdateFuncPtr) { const auto token = spring::benchmark_callins::Begin("native", "callin_update"); UpdateResult result = {}; m_UpdateFuncPtr(m_nativeInterface, m_moduleData, &query, &result); spring::benchmark_callins::End(token); }
}

void NativeInterfaceEventClient::DrawScreen() { DrawScreenQuery query = {.viewSizeX = (globalRendering != nullptr) ? globalRendering->viewSizeX : 0, .viewSizeY = (globalRendering != nullptr) ? globalRendering->viewSizeY : 0}; DispatchWasmCallin("DrawScreen", &query, false); if (m_DrawScreenFuncPtr) { DrawScreenResult result = {}; m_DrawScreenFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }

#define DISPATCH_SIMPLE_CALLIN(EventName) \
	void NativeInterfaceEventClient::EventName() { SimpleCallinQuery query = {}; const auto wasmToken = spring::benchmark_callins::Begin("wasm", spring::benchmark_callins::EventTestName(#EventName)); DispatchWasmCallin(#EventName, &query, false); spring::benchmark_callins::End(wasmToken); if (m_##EventName##FuncPtr) { const auto nativeToken = spring::benchmark_callins::Begin("native", spring::benchmark_callins::EventTestName(#EventName)); SimpleCallinResult result = {}; m_##EventName##FuncPtr(m_nativeInterface, m_moduleData, &query, &result); spring::benchmark_callins::End(nativeToken); } }
DISPATCH_SIMPLE_CALLIN(DrawGenesis)
DISPATCH_SIMPLE_CALLIN(DrawWorld)
DISPATCH_SIMPLE_CALLIN(DrawWorldPreUnit)
DISPATCH_SIMPLE_CALLIN(DrawPreDecals)
DISPATCH_SIMPLE_CALLIN(DrawWaterPost)
DISPATCH_SIMPLE_CALLIN(DrawWorldShadow)
DISPATCH_SIMPLE_CALLIN(DrawShadowPassTransparent)
DISPATCH_SIMPLE_CALLIN(DrawWorldReflection)
DISPATCH_SIMPLE_CALLIN(DrawWorldRefraction)
DISPATCH_SIMPLE_CALLIN(DrawGroundPreForward)
DISPATCH_SIMPLE_CALLIN(DrawGroundPostForward)
DISPATCH_SIMPLE_CALLIN(DrawGroundPreDeferred)
DISPATCH_SIMPLE_CALLIN(DrawGroundDeferred)
DISPATCH_SIMPLE_CALLIN(DrawGroundPostDeferred)
DISPATCH_SIMPLE_CALLIN(DrawUnitsPostDeferred)
DISPATCH_SIMPLE_CALLIN(DrawFeaturesPostDeferred)
DISPATCH_SIMPLE_CALLIN(DrawShadowUnitsLua)
DISPATCH_SIMPLE_CALLIN(DrawShadowFeaturesLua)
#undef DISPATCH_SIMPLE_CALLIN

#define DISPATCH_SCREEN_CALLIN(EventName) \
	void NativeInterfaceEventClient::EventName() { DrawScreenQuery query = {.viewSizeX = (globalRendering != nullptr) ? globalRendering->viewSizeX : 0, .viewSizeY = (globalRendering != nullptr) ? globalRendering->viewSizeY : 0}; DispatchWasmCallin(#EventName, &query, false); if (m_##EventName##FuncPtr) { DrawScreenResult result = {}; m_##EventName##FuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
DISPATCH_SCREEN_CALLIN(DrawScreenEffects)
DISPATCH_SCREEN_CALLIN(DrawScreenPost)
#undef DISPATCH_SCREEN_CALLIN

#define DISPATCH_MINIMAP_DRAW_CALLIN(EventName) \
	void NativeInterfaceEventClient::EventName() { MiniMapDrawQuery query = {.sizeX = (minimap != nullptr) ? minimap->GetSizeX() : 0, .sizeY = (minimap != nullptr) ? minimap->GetSizeY() : 0}; DispatchWasmCallin(#EventName, &query, false); if (m_##EventName##FuncPtr) { SimpleCallinResult result = {}; m_##EventName##FuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
DISPATCH_MINIMAP_DRAW_CALLIN(DrawInMiniMap)
DISPATCH_MINIMAP_DRAW_CALLIN(DrawInMiniMapBackground)
#undef DISPATCH_MINIMAP_DRAW_CALLIN

#define BOOL_OR_NATIVE_BODY(Func, QueryExpr, Synced) \
	QueryExpr; bool wasmValue = false; const bool hasWasmValue = DispatchWasmBoolCallin(#Func, &query, Synced, wasmValue); if (m_##Func##FuncPtr == nullptr) return hasWasmValue && wasmValue; BoolCallinResult nativeResult = {.value = false}; m_##Func##FuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult); return nativeResult.value || (hasWasmValue && wasmValue)

bool NativeInterfaceEventClient::DrawUnit(const CUnit* unit) { BOOL_OR_NATIVE_BODY(DrawUnit, DrawUnitQuery query = {.unitID = (unit != nullptr) ? unit->id : -1, .drawMode = (game != nullptr) ? static_cast<int>(game->GetDrawMode()) : 0}, false); }
bool NativeInterfaceEventClient::DrawFeature(const CFeature* feature) { BOOL_OR_NATIVE_BODY(DrawFeature, DrawFeatureQuery query = {.featureID = (feature != nullptr) ? feature->id : -1, .drawMode = (game != nullptr) ? static_cast<int>(game->GetDrawMode()) : 0}, false); }
bool NativeInterfaceEventClient::DrawShield(const CUnit* unit, const CWeapon* weapon) { BOOL_OR_NATIVE_BODY(DrawShield, DrawShieldQuery query = {.unitID = (unit != nullptr) ? unit->id : -1, .weaponID = (weapon != nullptr) ? weapon->weaponNum + LUA_WEAPON_BASE_INDEX : -1, .drawMode = (game != nullptr) ? static_cast<int>(game->GetDrawMode()) : 0}, false); }
bool NativeInterfaceEventClient::DrawProjectile(const CProjectile* projectile) { BOOL_OR_NATIVE_BODY(DrawProjectile, DrawProjectileQuery query = {.projectileID = (projectile != nullptr) ? projectile->id : -1, .drawMode = (game != nullptr) ? static_cast<int>(game->GetDrawMode()) : 0}, false); }
bool NativeInterfaceEventClient::DrawMaterial(const LuaMaterial* material) { BOOL_OR_NATIVE_BODY(DrawMaterial, DrawMaterialQuery query = {.uuid = (material != nullptr) ? material->uuid : -1, .drawMode = (game != nullptr) ? static_cast<int>(game->GetDrawMode()) : 0}, false); }
#undef BOOL_OR_NATIVE_BODY

void NativeInterfaceEventClient::DrawWorldPreParticles(bool drawAboveWater, bool drawBelowWater, bool drawReflection, bool drawRefraction) { DrawWorldPreParticlesQuery query = {.drawAboveWater = drawAboveWater, .drawBelowWater = drawBelowWater, .drawReflection = drawReflection, .drawRefraction = drawRefraction}; DispatchWasmCallin("DrawWorldPreParticles", &query, false); if (m_DrawWorldPreParticlesFuncPtr) { DrawWorldPreParticlesResult result = {}; m_DrawWorldPreParticlesFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }

void NativeInterfaceEventClient::DrawBuildSquare(int unitDefID, int x, int z, int facing, const std::vector<uint8_t>& statuses) { DrawBuildSquareQuery query = {.unitDefID = unitDefID, .x = x, .z = z, .facing = facing, .statuses = statuses.data(), .statusCount = static_cast<uint32_t>(statuses.size())}; DispatchWasmCallin("DrawBuildSquare", &query, false); if (m_DrawBuildSquareFuncPtr) { DrawBuildSquareResult result = {}; m_DrawBuildSquareFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }

#define DISPATCH_DRAW_OBJECTS_LUA(EventName) \
	void NativeInterfaceEventClient::EventName(bool deferredPass, bool drawReflection, bool drawRefraction) { DrawObjectsLuaQuery query = {.deferredPass = deferredPass, .drawReflection = drawReflection, .drawRefraction = drawRefraction}; DispatchWasmCallin(#EventName, &query, false); if (m_##EventName##FuncPtr) { DrawObjectsLuaResult result = {}; m_##EventName##FuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
DISPATCH_DRAW_OBJECTS_LUA(DrawOpaqueUnitsLua)
DISPATCH_DRAW_OBJECTS_LUA(DrawOpaqueFeaturesLua)
#undef DISPATCH_DRAW_OBJECTS_LUA

#define DISPATCH_DRAW_ALPHA_OBJECTS_LUA(EventName) \
	void NativeInterfaceEventClient::EventName(bool drawReflection, bool drawRefraction) { DrawAlphaObjectsLuaQuery query = {.drawReflection = drawReflection, .drawRefraction = drawRefraction}; DispatchWasmCallin(#EventName, &query, false); if (m_##EventName##FuncPtr) { DrawAlphaObjectsLuaResult result = {}; m_##EventName##FuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
DISPATCH_DRAW_ALPHA_OBJECTS_LUA(DrawAlphaUnitsLua)
DISPATCH_DRAW_ALPHA_OBJECTS_LUA(DrawAlphaFeaturesLua)
#undef DISPATCH_DRAW_ALPHA_OBJECTS_LUA

void NativeInterfaceEventClient::GamePaused(int playerID, bool paused) { GamePausedQuery query = {.playerID = playerID, .paused = paused}; DispatchWasmCallin("GamePaused", &query, true); if (m_GamePausedFuncPtr) { GamePausedResult result = {}; m_GamePausedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::GameID(const unsigned char* gameID, unsigned int numBytes) { GameIDQuery query = {.gameID = gameID, .numBytes = numBytes}; DispatchWasmCallin("GameID", &query, true); if (m_GameIDFuncPtr) { GameIDResult result = {}; m_GameIDFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::TeamDied(int teamID) { TeamDiedQuery query = {.teamID = teamID}; DispatchWasmCallin("TeamDied", &query, true); if (m_TeamDiedFuncPtr) { TeamDiedResult result = {}; m_TeamDiedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::TeamChanged(int teamID) { TeamChangedQuery query = {.teamID = teamID}; DispatchWasmCallin("TeamChanged", &query, true); if (m_TeamChangedFuncPtr) { TeamChangedResult result = {}; m_TeamChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::PlayerChanged(int playerID) { PlayerChangedQuery query = {.playerID = playerID}; DispatchWasmCallin("PlayerChanged", &query, true); if (m_PlayerChangedFuncPtr) { PlayerChangedResult result = {}; m_PlayerChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::PlayerAdded(int playerID) { PlayerAddedQuery query = {.playerID = playerID}; DispatchWasmCallin("PlayerAdded", &query, true); if (m_PlayerAddedFuncPtr) { PlayerAddedResult result = {}; m_PlayerAddedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::PlayerRemoved(int playerID, int reason) { PlayerRemovedQuery query = {.playerID = playerID, .reason = reason}; DispatchWasmCallin("PlayerRemoved", &query, true); if (m_PlayerRemovedFuncPtr) { PlayerRemovedResult result = {}; m_PlayerRemovedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }

void NativeInterfaceEventClient::UnitCreated(const CUnit* unit, const CUnit* builder) { UnitCreatedQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team, .builderID = builder != nullptr ? builder->id : -1}; const auto token = spring::benchmark_callins::Begin("wasm", "callin_unitcreated"); DispatchWasmCallin("UnitCreated", &query, true); spring::benchmark_callins::End(token); if (m_UnitCreatedFuncPtr) { const auto nativeToken = spring::benchmark_callins::Begin("native", "callin_unitcreated"); UnitCreatedResult result = {}; m_UnitCreatedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); spring::benchmark_callins::End(nativeToken); } }
void NativeInterfaceEventClient::UnitFinished(const CUnit* unit) { UnitFinishedQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team}; DispatchWasmCallin("UnitFinished", &query, true); if (m_UnitFinishedFuncPtr) { UnitFinishedResult result = {}; m_UnitFinishedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::UnitReverseBuilt(const CUnit* unit) { UnitReverseBuiltQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team}; DispatchWasmCallin("UnitReverseBuilt", &query, true); if (m_UnitReverseBuiltFuncPtr) { UnitReverseBuiltResult result = {}; m_UnitReverseBuiltFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::UnitConstructionDecayed(const CUnit* unit, float timeSinceLastBuild, float iterationPeriod, float part) { UnitConstructionDecayedQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team, .timeSinceLastBuild = timeSinceLastBuild, .iterationPeriod = iterationPeriod, .part = part}; DispatchWasmCallin("UnitConstructionDecayed", &query, true); if (m_UnitConstructionDecayedFuncPtr) { UnitConstructionDecayedResult result = {}; m_UnitConstructionDecayedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::UnitFromFactory(const CUnit* unit, const CUnit* factory, bool userOrders) { UnitFromFactoryQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team, .factoryID = factory->id, .factoryDefID = (factory->unitDef != nullptr) ? factory->unitDef->id : -1, .userOrders = userOrders}; DispatchWasmCallin("UnitFromFactory", &query, true); if (m_UnitFromFactoryFuncPtr) { UnitFromFactoryResult result = {}; m_UnitFromFactoryFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::UnitDestroyed(const CUnit* unit, const CUnit* attacker, int weaponDefID) { UnitDestroyedQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team, .attackerID = attacker != nullptr ? attacker->id : -1, .attackerDefID = (attacker != nullptr && attacker->unitDef != nullptr) ? attacker->unitDef->id : -1, .attackerTeam = attacker != nullptr ? attacker->team : -1, .weaponDefID = weaponDefID}; DispatchWasmCallin("UnitDestroyed", &query, true); if (m_UnitDestroyedFuncPtr) { UnitDestroyedResult result = {}; m_UnitDestroyedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::UnitTaken(const CUnit* unit, int oldTeam, int newTeam) { UnitTakenQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .oldTeam = oldTeam, .newTeam = newTeam}; DispatchWasmCallin("UnitTaken", &query, true); if (m_UnitTakenFuncPtr) { UnitTakenResult result = {}; m_UnitTakenFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::UnitGiven(const CUnit* unit, int oldTeam, int newTeam) { UnitGivenQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .oldTeam = oldTeam, .newTeam = newTeam}; DispatchWasmCallin("UnitGiven", &query, true); if (m_UnitGivenFuncPtr) { UnitGivenResult result = {}; m_UnitGivenFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::UnitIdle(const CUnit* unit) { UnitIdleQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team}; DispatchWasmCallin("UnitIdle", &query, true); if (m_UnitIdleFuncPtr) { UnitIdleResult result = {}; m_UnitIdleFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }

void NativeInterfaceEventClient::UnitCommand(const CUnit* unit, const Command& command, int playerNum, bool fromSynced, bool fromLua) { UnitCommandQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team, .command = ToNativeCallinCommand(command), .playerNum = playerNum, .fromSynced = fromSynced, .fromLua = fromLua}; DispatchWasmCallin("UnitCommand", &query, fromSynced); if (m_UnitCommandFuncPtr) { UnitCommandResult result = {}; m_UnitCommandFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }

bool NativeInterfaceEventClient::CommandFallback(const CUnit* unit, const Command& command) { CommandFallbackQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team, .command = ToNativeCallinCommand(command)}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("CommandFallback", &query, true, wasmValue); if (m_CommandFallbackFuncPtr == nullptr) return has && wasmValue; BoolCallinResult nativeResult = {.value = false}; m_CommandFallbackFuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult); return nativeResult.value || (has && wasmValue); }

bool NativeInterfaceEventClient::AllowCommand(const CUnit* unit, const Command& command, int playerNum, bool fromSynced, bool fromLua) { UnitCommandQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team, .command = ToNativeCallinCommand(command), .playerNum = playerNum, .fromSynced = fromSynced, .fromLua = fromLua}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("AllowCommand", &query, fromSynced, wasmValue); if (m_AllowCommandFuncPtr == nullptr) return has ? wasmValue : true; BoolCallinResult nativeResult = {.value = true}; m_AllowCommandFuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult); return nativeResult.value && (!has || wasmValue); }

std::pair<bool, bool> NativeInterfaceEventClient::AllowUnitCreation(const UnitDef* unitDef, const CUnit* builder, const BuildInfo* buildInfo) {
	AllowUnitCreationQuery query = {.unitDefID = (unitDef != nullptr) ? unitDef->id : -1, .builderID = (builder != nullptr) ? builder->id : -1, .builderTeam = (builder != nullptr) ? builder->team : -1, .hasBuildInfo = (buildInfo != nullptr), .buildPos = (buildInfo != nullptr) ? Float3{buildInfo->pos.x, buildInfo->pos.y, buildInfo->pos.z} : Float3{}, .buildFacing = (buildInfo != nullptr) ? buildInfo->buildFacing : 0};
	const auto wasmToken = spring::benchmark_callins::Begin("wasm", "callin_allowunitcreation");
	WasmValue wasmResult;
	AllowUnitCreationResult directResult = {.allow = true, .dropOrder = true};
	const bool hasWasmResult = DispatchWasmCallin("AllowUnitCreation", &query, true, &wasmResult, &directResult);
	spring::benchmark_callins::End(wasmToken);
	bool wasmAllow = true;
	bool wasmDropOrder = true;
	const WasmValueRecord* wasmRecord = hasWasmResult ? WasmResultRecord(wasmResult) : nullptr;
	bool hasWasmFields = wasmRecord != nullptr && ReadWasmBoolField(*wasmRecord, "allow", wasmAllow) && ReadWasmBoolField(*wasmRecord, "dropOrder", wasmDropOrder);
	if (!hasWasmFields && hasWasmResult && wasmResult.IsUnit()) { wasmAllow = directResult.allow; wasmDropOrder = directResult.dropOrder; hasWasmFields = true; }
	if (m_AllowUnitCreationFuncPtr == nullptr) return hasWasmFields ? std::pair{wasmAllow, wasmDropOrder} : std::pair{true, true};
	const auto nativeToken = spring::benchmark_callins::Begin("native", "callin_allowunitcreation");
	AllowUnitCreationResult nativeResult = {.allow = true, .dropOrder = true};
	m_AllowUnitCreationFuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult);
	spring::benchmark_callins::End(nativeToken);
	return {nativeResult.allow, nativeResult.dropOrder};
}

#define ALLOW_AND_NATIVE(Func, QueryExpr, DefaultValue) \
	QueryExpr; bool wasmValue = false; const bool has = DispatchWasmBoolCallin(#Func, &query, true, wasmValue); if (m_##Func##FuncPtr == nullptr) return has ? wasmValue : DefaultValue; BoolCallinResult nativeResult = {.value = DefaultValue}; m_##Func##FuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult); return nativeResult.value && (!has || wasmValue)

bool NativeInterfaceEventClient::AllowUnitTransfer(const CUnit* unit, int newTeam, bool capture) { ALLOW_AND_NATIVE(AllowUnitTransfer, AllowUnitTransferQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .oldTeam = unit->team, .newTeam = newTeam, .capture = capture}, true); }
bool NativeInterfaceEventClient::AllowUnitBuildStep(const CUnit* builder, const CUnit* unit, float part) { ALLOW_AND_NATIVE(AllowUnitBuildStep, AllowUnitBuildStepQuery query = {.builderID = builder->id, .builderTeam = builder->team, .unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .part = part}, true); }
bool NativeInterfaceEventClient::AllowUnitCaptureStep(const CUnit* builder, const CUnit* unit, float part) { AllowUnitBuildStepQuery query = {.builderID = builder->id, .builderTeam = builder->team, .unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .part = part}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("AllowUnitCaptureStep", &query, true, wasmValue); if (m_AllowUnitCaptureStepFuncPtr == nullptr) return has ? wasmValue : true; BoolCallinResult nativeResult = {.value = true}; m_AllowUnitCaptureStepFuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult); return nativeResult.value && (!has || wasmValue); }
bool NativeInterfaceEventClient::AllowUnitTransport(const CUnit* transporter, const CUnit* transportee) { ALLOW_AND_NATIVE(AllowUnitTransport, AllowUnitTransportQuery query = {.transporterID = transporter->id, .transporterDefID = (transporter->unitDef != nullptr) ? transporter->unitDef->id : -1, .transporterTeam = transporter->team, .transporteeID = transportee->id, .transporteeDefID = (transportee->unitDef != nullptr) ? transportee->unitDef->id : -1, .transporteeTeam = transportee->team}, true); }
#undef ALLOW_AND_NATIVE

bool NativeInterfaceEventClient::AllowUnitTransportLoad(const CUnit* transporter, const CUnit* transportee, const float3& loadPos, bool allowed) { AllowUnitTransportPositionQuery query = {.units = {.transporterID = transporter->id, .transporterDefID = (transporter->unitDef != nullptr) ? transporter->unitDef->id : -1, .transporterTeam = transporter->team, .transporteeID = transportee->id, .transporteeDefID = (transportee->unitDef != nullptr) ? transportee->unitDef->id : -1, .transporteeTeam = transportee->team}, .position = {.x = loadPos.x, .y = loadPos.y, .z = loadPos.z}, .allowed = allowed}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("AllowUnitTransportLoad", &query, true, wasmValue); if (m_AllowUnitTransportLoadFuncPtr == nullptr) return has ? wasmValue : allowed; BoolCallinResult nativeResult = {.value = allowed}; m_AllowUnitTransportLoadFuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult); return nativeResult.value && (!has || wasmValue); }
bool NativeInterfaceEventClient::AllowUnitTransportUnload(const CUnit* transporter, const CUnit* transportee, const float3& unloadPos, bool allowed) { AllowUnitTransportPositionQuery query = {.units = {.transporterID = transporter->id, .transporterDefID = (transporter->unitDef != nullptr) ? transporter->unitDef->id : -1, .transporterTeam = transporter->team, .transporteeID = transportee->id, .transporteeDefID = (transportee->unitDef != nullptr) ? transportee->unitDef->id : -1, .transporteeTeam = transportee->team}, .position = {.x = unloadPos.x, .y = unloadPos.y, .z = unloadPos.z}, .allowed = allowed}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("AllowUnitTransportUnload", &query, true, wasmValue); if (m_AllowUnitTransportUnloadFuncPtr == nullptr) return has ? wasmValue : allowed; BoolCallinResult nativeResult = {.value = allowed}; m_AllowUnitTransportUnloadFuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult); return nativeResult.value && (!has || wasmValue); }
bool NativeInterfaceEventClient::AllowUnitCloak(const CUnit* unit, const CUnit* enemy) { AllowUnitCloakQuery query = {.unitID = unit->id, .hasEnemy = (enemy != nullptr), .enemyID = (enemy != nullptr) ? enemy->id : -1}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("AllowUnitCloak", &query, true, wasmValue); if (m_AllowUnitCloakFuncPtr == nullptr) return has ? wasmValue : true; BoolCallinResult nativeResult = {.value = true}; m_AllowUnitCloakFuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult); return nativeResult.value && (!has || wasmValue); }
bool NativeInterfaceEventClient::AllowUnitDecloak(const CUnit* unit, const CSolidObject* object, const CWeapon* weapon) { AllowUnitDecloakQuery query = {.unitID = unit->id, .hasObject = (object != nullptr), .objectID = (object != nullptr) ? object->id : -1, .hasWeapon = (weapon != nullptr), .weaponNum = (weapon != nullptr) ? weapon->weaponNum : -1}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("AllowUnitDecloak", &query, true, wasmValue); if (m_AllowUnitDecloakFuncPtr == nullptr) return has ? wasmValue : true; BoolCallinResult nativeResult = {.value = true}; m_AllowUnitDecloakFuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult); return nativeResult.value && (!has || wasmValue); }
bool NativeInterfaceEventClient::AllowUnitKamikaze(const CUnit* unit, const CUnit* target, bool allowed) { AllowUnitKamikazeQuery query = {.unitID = unit->id, .targetID = (target != nullptr) ? target->id : -1, .allowed = allowed}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("AllowUnitKamikaze", &query, true, wasmValue); if (m_AllowUnitKamikazeFuncPtr == nullptr) return has ? wasmValue : allowed; BoolCallinResult nativeResult = {.value = allowed}; m_AllowUnitKamikazeFuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult); return nativeResult.value && (!has || wasmValue); }

void NativeInterfaceEventClient::UnitCmdDone(const CUnit* unit, const Command& command) { UnitCmdDoneQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team, .command = ToNativeCallinCommand(command)}; DispatchWasmCallin("UnitCmdDone", &query, true); if (m_UnitCmdDoneFuncPtr) { UnitCmdDoneResult result = {}; m_UnitCmdDoneFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::UnitDamaged(const CUnit* unit, const CUnit* attacker, float damage, int weaponDefID, int projectileID, bool paralyzer) { UnitDamagedQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team, .damage = damage, .paralyzer = paralyzer, .weaponDefID = weaponDefID, .projectileID = projectileID, .attackerID = (attacker != nullptr) ? attacker->id : -1, .attackerDefID = (attacker != nullptr && attacker->unitDef != nullptr) ? attacker->unitDef->id : -1, .attackerTeam = (attacker != nullptr) ? attacker->team : -1}; DispatchWasmCallin("UnitDamaged", &query, true); if (m_UnitDamagedFuncPtr) { UnitDamagedResult result = {}; m_UnitDamagedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::UnitHarvestStorageFull(const CUnit* unit) { UnitHarvestStorageFullQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team}; DispatchWasmCallin("UnitHarvestStorageFull", &query, true); if (m_UnitHarvestStorageFullFuncPtr) { UnitHarvestStorageFullResult result = {}; m_UnitHarvestStorageFullFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::UnitSeismicPing(const CUnit* unit, int allyTeam, const float3& pos, float strength) { UnitSeismicPingQuery query = {.pos = {.x = pos.x, .y = pos.y, .z = pos.z}, .strength = strength, .allyTeam = allyTeam, .unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1}; DispatchWasmCallin("UnitSeismicPing", &query, true); if (m_UnitSeismicPingFuncPtr) { UnitSeismicPingResult result = {}; m_UnitSeismicPingFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }

#define DISPATCH_UNIT_LOS_EVENT(EventName) \
	void NativeInterfaceEventClient::EventName(const CUnit* unit, int allyTeam) { UnitLosEventQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team, .allyTeam = allyTeam}; DispatchWasmCallin(#EventName, &query, true); if (m_##EventName##FuncPtr) { UnitLosEventResult result = {}; m_##EventName##FuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
DISPATCH_UNIT_LOS_EVENT(UnitEnteredRadar)
DISPATCH_UNIT_LOS_EVENT(UnitEnteredLos)
DISPATCH_UNIT_LOS_EVENT(UnitLeftRadar)
DISPATCH_UNIT_LOS_EVENT(UnitLeftLos)
#undef DISPATCH_UNIT_LOS_EVENT

#define DISPATCH_UNIT_MOVEMENT_CLASS_EVENT(EventName) \
	void NativeInterfaceEventClient::EventName(const CUnit* unit) { UnitMovementClassEventQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team}; DispatchWasmCallin(#EventName, &query, true); if (m_##EventName##FuncPtr) { UnitMovementClassEventResult result = {}; m_##EventName##FuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
DISPATCH_UNIT_MOVEMENT_CLASS_EVENT(UnitEnteredUnderwater)
DISPATCH_UNIT_MOVEMENT_CLASS_EVENT(UnitEnteredWater)
DISPATCH_UNIT_MOVEMENT_CLASS_EVENT(UnitEnteredAir)
DISPATCH_UNIT_MOVEMENT_CLASS_EVENT(UnitLeftUnderwater)
DISPATCH_UNIT_MOVEMENT_CLASS_EVENT(UnitLeftWater)
DISPATCH_UNIT_MOVEMENT_CLASS_EVENT(UnitLeftAir)
#undef DISPATCH_UNIT_MOVEMENT_CLASS_EVENT

void NativeInterfaceEventClient::UnitStunned(const CUnit* unit, bool stunned) { UnitStunnedQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team, .stunned = stunned}; DispatchWasmCallin("UnitStunned", &query, true); if (m_UnitStunnedFuncPtr) { UnitStunnedResult result = {}; m_UnitStunnedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::UnitExperience(const CUnit* unit, float oldExperience) { UnitExperienceQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team, .experience = unit->experience, .oldExperience = oldExperience}; DispatchWasmCallin("UnitExperience", &query, true); if (m_UnitExperienceFuncPtr) { UnitExperienceResult result = {}; m_UnitExperienceFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::UnitLoaded(const CUnit* unit, const CUnit* transport) { UnitLoadedQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team, .transportID = transport->id, .transportTeam = transport->team}; DispatchWasmCallin("UnitLoaded", &query, true); if (m_UnitLoadedFuncPtr) { UnitLoadedResult result = {}; m_UnitLoadedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::UnitUnloaded(const CUnit* unit, const CUnit* transport) { UnitUnloadedQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team, .transportID = transport->id, .transportTeam = transport->team}; DispatchWasmCallin("UnitUnloaded", &query, true); if (m_UnitUnloadedFuncPtr) { UnitUnloadedResult result = {}; m_UnitUnloadedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::UnitCloaked(const CUnit* unit) { UnitCloakEventQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team}; DispatchWasmCallin("UnitCloaked", &query, true); if (m_UnitCloakedFuncPtr) { UnitCloakEventResult result = {}; m_UnitCloakedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::UnitDecloaked(const CUnit* unit) { UnitCloakEventQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team}; DispatchWasmCallin("UnitDecloaked", &query, true); if (m_UnitDecloakedFuncPtr) { UnitCloakEventResult result = {}; m_UnitDecloakedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }

#define DISPATCH_UNIT_MOVE_EVENT(EventName) \
	void NativeInterfaceEventClient::EventName(const CUnit* unit) { UnitMoveEventQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team}; DispatchWasmCallin(#EventName, &query, true); if (m_##EventName##FuncPtr) { UnitMoveEventResult result = {}; m_##EventName##FuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
DISPATCH_UNIT_MOVE_EVENT(UnitMoved)
DISPATCH_UNIT_MOVE_EVENT(UnitMoveFailed)
DISPATCH_UNIT_MOVE_EVENT(UnitArrivedAtGoal)
#undef DISPATCH_UNIT_MOVE_EVENT

bool NativeInterfaceEventClient::UnitUnitCollision(const CUnit* collider, const CUnit* collidee) { UnitUnitCollisionQuery query = {.colliderID = (collider != nullptr) ? collider->id : -1, .collideeID = (collidee != nullptr) ? collidee->id : -1}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("UnitUnitCollision", &query, true, wasmValue); if (m_UnitUnitCollisionFuncPtr) { BoolCallinResult result = {.value = false}; m_UnitUnitCollisionFuncPtr(m_nativeInterface, m_moduleData, &query, &result); return result.value || (has && wasmValue); } return has && wasmValue; }
bool NativeInterfaceEventClient::UnitFeatureCollision(const CUnit* collider, const CFeature* collidee) { UnitFeatureCollisionQuery query = {.colliderID = (collider != nullptr) ? collider->id : -1, .collideeID = (collidee != nullptr) ? collidee->id : -1}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("UnitFeatureCollision", &query, true, wasmValue); if (m_UnitFeatureCollisionFuncPtr) { BoolCallinResult result = {.value = false}; m_UnitFeatureCollisionFuncPtr(m_nativeInterface, m_moduleData, &query, &result); return result.value || (has && wasmValue); } return has && wasmValue; }
void NativeInterfaceEventClient::RenderUnitDestroyed(const CUnit* unit) { RenderUnitDestroyedQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team}; DispatchWasmCallin("RenderUnitDestroyed", &query, false); if (m_RenderUnitDestroyedFuncPtr) { RenderUnitDestroyedResult result = {}; m_RenderUnitDestroyedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }

void NativeInterfaceEventClient::FeatureCreated(const CFeature* feature) { FeatureCreatedQuery query = {.featureID = feature->id, .allyTeamID = feature->allyteam}; DispatchWasmCallin("FeatureCreated", &query, true); if (m_FeatureCreatedFuncPtr) { FeatureCreatedResult result = {}; m_FeatureCreatedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::FeatureDestroyed(const CFeature* feature) { FeatureDestroyedQuery query = {.featureID = feature->id, .allyTeamID = feature->allyteam}; DispatchWasmCallin("FeatureDestroyed", &query, true); if (m_FeatureDestroyedFuncPtr) { FeatureDestroyedResult result = {}; m_FeatureDestroyedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::FeatureDamaged(const CFeature* feature, const CUnit* attacker, float damage, int weaponDefID, int projectileID) { FeatureDamagedQuery query = {.featureID = feature->id, .featureDefID = (feature->def != nullptr) ? feature->def->id : -1, .featureTeam = feature->team, .damage = damage, .weaponDefID = weaponDefID, .projectileID = projectileID, .attackerID = (attacker != nullptr) ? attacker->id : -1, .attackerDefID = (attacker != nullptr && attacker->unitDef != nullptr) ? attacker->unitDef->id : -1, .attackerTeam = (attacker != nullptr) ? attacker->team : -1}; DispatchWasmCallin("FeatureDamaged", &query, true); if (m_FeatureDamagedFuncPtr) { FeatureDamagedResult result = {}; m_FeatureDamagedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }

bool NativeInterfaceEventClient::AllowFeatureCreation(const FeatureDef* featureDef, int allyTeamID, const float3& pos) { AllowFeatureCreationQuery query = {.featureDefID = (featureDef != nullptr) ? featureDef->id : -1, .teamID = allyTeamID, .position = {.x = pos.x, .y = pos.y, .z = pos.z}}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("AllowFeatureCreation", &query, true, wasmValue); if (m_AllowFeatureCreationFuncPtr == nullptr) return has ? wasmValue : true; BoolCallinResult nativeResult = {.value = true}; m_AllowFeatureCreationFuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult); return nativeResult.value || (has && wasmValue); }
bool NativeInterfaceEventClient::AllowFeatureBuildStep(const CUnit* builder, const CFeature* feature, float part) { AllowFeatureBuildStepQuery query = {.builderID = builder->id, .builderTeam = builder->team, .featureID = feature->id, .featureDefID = (feature->def != nullptr) ? feature->def->id : -1, .part = part}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("AllowFeatureBuildStep", &query, true, wasmValue); if (m_AllowFeatureBuildStepFuncPtr == nullptr) return has ? wasmValue : true; BoolCallinResult nativeResult = {.value = true}; m_AllowFeatureBuildStepFuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult); return nativeResult.value || (has && wasmValue); }
bool NativeInterfaceEventClient::AllowResourceLevel(int teamID, const std::string& type, float level) { AllowResourceLevelQuery query = {.teamID = teamID, .type = type.c_str(), .level = level}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("AllowResourceLevel", &query, true, wasmValue); if (m_AllowResourceLevelFuncPtr == nullptr) return has ? wasmValue : true; BoolCallinResult nativeResult = {.value = true}; m_AllowResourceLevelFuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult); return nativeResult.value || (has && wasmValue); }
bool NativeInterfaceEventClient::AllowResourceTransfer(int oldTeam, int newTeam, const char* type, float amount) { AllowResourceTransferQuery query = {.oldTeam = oldTeam, .newTeam = newTeam, .type = type, .amount = amount}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("AllowResourceTransfer", &query, true, wasmValue); if (m_AllowResourceTransferFuncPtr == nullptr) return has ? wasmValue : true; BoolCallinResult nativeResult = {.value = true}; m_AllowResourceTransferFuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult); return nativeResult.value || (has && wasmValue); }

bool NativeInterfaceEventClient::ResourceExcess(const std::map<int, SResourcePack>& excess) { std::vector<ResourceExcessEntry> entries; entries.reserve(excess.size()); for (const auto& [teamID, resources] : excess) entries.push_back({.teamID = teamID, .resources = {resources[0], resources[1]}}); ResourceExcessQuery query = {.entries = entries.data(), .count = static_cast<uint32_t>(entries.size())}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("ResourceExcess", &query, true, wasmValue); if (m_ResourceExcessFuncPtr == nullptr) return has && wasmValue; BoolCallinResult nativeResult = {.value = false}; m_ResourceExcessFuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult); return nativeResult.value || (has && wasmValue); }
bool NativeInterfaceEventClient::AllowDirectUnitControl(int playerID, const CUnit* unit) { AllowDirectUnitControlQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team, .playerID = playerID}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("AllowDirectUnitControl", &query, true, wasmValue); if (m_AllowDirectUnitControlFuncPtr == nullptr) return has ? wasmValue : true; BoolCallinResult nativeResult = {.value = true}; m_AllowDirectUnitControlFuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult); return nativeResult.value || (has && wasmValue); }
bool NativeInterfaceEventClient::AllowBuilderHoldFire(const CUnit* unit, int action) { AllowBuilderHoldFireQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .action = action}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("AllowBuilderHoldFire", &query, true, wasmValue); if (m_AllowBuilderHoldFireFuncPtr == nullptr) return has ? wasmValue : true; BoolCallinResult nativeResult = {.value = true}; m_AllowBuilderHoldFireFuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult); return nativeResult.value || (has && wasmValue); }
bool NativeInterfaceEventClient::AllowStartPosition(int playerID, int teamID, unsigned char readyState, const float3& clampedPos, const float3& rawPickPos) { AllowStartPositionQuery query = {.playerID = playerID, .teamID = teamID, .readyState = readyState, .clampedPos = {.x = clampedPos.x, .y = clampedPos.y, .z = clampedPos.z}, .rawPickPos = {.x = rawPickPos.x, .y = rawPickPos.y, .z = rawPickPos.z}}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("AllowStartPosition", &query, true, wasmValue); if (m_AllowStartPositionFuncPtr == nullptr) return has ? wasmValue : true; BoolCallinResult nativeResult = {.value = true}; m_AllowStartPositionFuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult); return nativeResult.value && (!has || wasmValue); }
bool NativeInterfaceEventClient::TerraformComplete(const CUnit* unit, const CUnit* build) { TerraformCompleteQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team, .buildUnitID = build->id, .buildUnitDefID = (build->unitDef != nullptr) ? build->unitDef->id : -1, .buildUnitTeam = build->team}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("TerraformComplete", &query, true, wasmValue); if (m_TerraformCompleteFuncPtr == nullptr) return has && wasmValue; BoolCallinResult nativeResult = {.value = false}; m_TerraformCompleteFuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult); return nativeResult.value || (has && wasmValue); }
bool NativeInterfaceEventClient::MoveCtrlNotify(const CUnit* unit, int data) { MoveCtrlNotifyQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team, .data = data}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("MoveCtrlNotify", &query, true, wasmValue); if (m_MoveCtrlNotifyFuncPtr == nullptr) return has && wasmValue; BoolCallinResult nativeResult = {.value = false}; m_MoveCtrlNotifyFuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult); return nativeResult.value || (has && wasmValue); }

void NativeInterfaceEventClient::FeatureMoved(const CFeature* feature, const float3& oldpos) { FeatureMovedQuery query = {.featureID = feature->id, .oldPos = {.x = oldpos.x, .y = oldpos.y, .z = oldpos.z}}; DispatchWasmCallin("FeatureMoved", &query, true); if (m_FeatureMovedFuncPtr) { FeatureMovedResult result = {}; m_FeatureMovedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::ProjectileCreated(const CProjectile* proj) { const auto* weaponProjectile = proj->weapon ? static_cast<const CWeaponProjectile*>(proj) : nullptr; const auto* weaponDef = (weaponProjectile != nullptr) ? weaponProjectile->GetWeaponDef() : nullptr; ProjectileEventQuery query = {.projectileID = proj->id, .ownerID = static_cast<int32_t>(proj->GetOwnerID()), .weaponDefID = (weaponDef != nullptr) ? weaponDef->id : -1}; DispatchWasmCallin("ProjectileCreated", &query, true); if (m_ProjectileCreatedFuncPtr) { ProjectileEventResult result = {}; m_ProjectileCreatedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::ProjectileDestroyed(const CProjectile* proj) { const auto* weaponProjectile = proj->weapon ? static_cast<const CWeaponProjectile*>(proj) : nullptr; const auto* weaponDef = (weaponProjectile != nullptr) ? weaponProjectile->GetWeaponDef() : nullptr; ProjectileEventQuery query = {.projectileID = proj->id, .ownerID = static_cast<int32_t>(proj->GetOwnerID()), .weaponDefID = (weaponDef != nullptr) ? weaponDef->id : -1}; DispatchWasmCallin("ProjectileDestroyed", &query, true); if (m_ProjectileDestroyedFuncPtr) { ProjectileEventResult result = {}; m_ProjectileDestroyedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }

bool NativeInterfaceEventClient::Explosion(int weaponID, const WeaponDef* weaponDef, const CExplosionParams& params) { (void)weaponDef; if (weaponID < 0) return false; ExplosionQuery query = {.weaponDefID = weaponID, .pos = {.x = params.pos.x, .y = params.pos.y, .z = params.pos.z}, .ownerID = (params.owner != nullptr) ? params.owner->id : -1, .projectileID = static_cast<int32_t>(params.projectileID)}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("Explosion", &query, true, wasmValue); if (m_ExplosionFuncPtr) { BoolCallinResult result = {.value = false}; m_ExplosionFuncPtr(m_nativeInterface, m_moduleData, &query, &result); return result.value || (has && wasmValue); } return has && wasmValue; }

int NativeInterfaceEventClient::AllowWeaponTargetCheck(unsigned int attackerID, unsigned int attackerWeaponNum, unsigned int attackerWeaponDefID) { AllowWeaponTargetCheckQuery query = {.attackerID = static_cast<int32_t>(attackerID), .attackerWeaponNum = static_cast<int32_t>(attackerWeaponNum + LUA_WEAPON_BASE_INDEX), .attackerWeaponDefID = static_cast<int32_t>(attackerWeaponDefID)}; int wasmValue = -1; const bool has = DispatchWasmIntegerCallin("AllowWeaponTargetCheck", &query, true, wasmValue); if (m_AllowWeaponTargetCheckFuncPtr == nullptr) return has ? wasmValue : -1; IntCallinResult nativeResult = {.value = -1}; m_AllowWeaponTargetCheckFuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult); return has && wasmValue != -1 ? wasmValue : nativeResult.value; }

bool NativeInterfaceEventClient::AllowWeaponTarget(unsigned int attackerID, unsigned int targetID, unsigned int attackerWeaponNum, unsigned int attackerWeaponDefID, float* targetPriority) { const int attackerWeaponNumber = static_cast<int>(attackerWeaponNum); AllowWeaponTargetQuery query = {.attackerID = static_cast<int32_t>(attackerID), .targetID = static_cast<int32_t>(targetID), .attackerWeaponNum = attackerWeaponNumber + LUA_WEAPON_BASE_INDEX * (attackerWeaponNumber >= 0), .attackerWeaponDefID = static_cast<int32_t>(attackerWeaponDefID), .hasTargetPriority = (targetPriority != nullptr), .targetPriority = (targetPriority != nullptr) ? *targetPriority : 0.0f}; WasmValue wasmResult; const bool has = DispatchWasmCallin("AllowWeaponTarget", &query, true, &wasmResult); bool wasmAllowed = true; float wasmPriority = query.targetPriority; const WasmValueRecord* record = has ? WasmResultRecord(wasmResult) : nullptr; const bool fields = record != nullptr && ReadWasmBoolField(*record, "allowed", wasmAllowed) && ReadWasmFloatField(*record, "targetPriority", wasmPriority); if (m_AllowWeaponTargetFuncPtr == nullptr) { if (fields && targetPriority != nullptr) *targetPriority = wasmPriority; return fields ? wasmAllowed : true; } AllowWeaponTargetResult nativeResult = {.allowed = true, .targetPriority = query.targetPriority}; m_AllowWeaponTargetFuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult); if (targetPriority != nullptr) *targetPriority = nativeResult.targetPriority; return nativeResult.allowed; }

bool NativeInterfaceEventClient::AllowWeaponInterceptTarget(const CUnit* interceptorUnit, const CWeapon* interceptorWeapon, const CProjectile* interceptorTarget) { AllowWeaponInterceptTargetQuery query = {.interceptorUnitID = (interceptorUnit != nullptr) ? interceptorUnit->id : -1, .interceptorWeaponID = (interceptorWeapon != nullptr) ? interceptorWeapon->weaponNum + LUA_WEAPON_BASE_INDEX : -1, .interceptorTargetID = (interceptorTarget != nullptr) ? interceptorTarget->id : -1}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("AllowWeaponInterceptTarget", &query, true, wasmValue); if (m_AllowWeaponInterceptTargetFuncPtr == nullptr) return has ? wasmValue : true; BoolCallinResult nativeResult = {.value = true}; m_AllowWeaponInterceptTargetFuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult); return nativeResult.value || (has && wasmValue); }

bool NativeInterfaceEventClient::UnitPreDamaged(const CUnit* unit, const CUnit* attacker, float damage, int weaponDefID, int projectileID, bool paralyzer, float* newDamage, float* impulseMult) {
	UnitDamagedQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team, .damage = damage, .paralyzer = paralyzer, .weaponDefID = weaponDefID, .projectileID = projectileID, .attackerID = (attacker != nullptr) ? attacker->id : -1, .attackerDefID = (attacker != nullptr && attacker->unitDef != nullptr) ? attacker->unitDef->id : -1, .attackerTeam = (attacker != nullptr) ? attacker->team : -1};
	const auto wasmToken = spring::benchmark_callins::Begin("wasm", "callin_unitpredamaged");
	WasmValue wasmResult;
	DamageCallinResult directResult = {.newDamage = (newDamage != nullptr) ? *newDamage : damage, .impulseMult = (impulseMult != nullptr) ? *impulseMult : 1.0f};
	const bool hasWasmResult = DispatchWasmCallin("UnitPreDamaged", &query, true, &wasmResult, &directResult);
	spring::benchmark_callins::End(wasmToken);
	float wasmDamage = (newDamage != nullptr) ? *newDamage : damage;
	float wasmImpulse = (impulseMult != nullptr) ? *impulseMult : 1.0f;
	const WasmValueRecord* record = hasWasmResult ? WasmResultRecord(wasmResult) : nullptr;
	bool fields = record != nullptr && ReadWasmFloatField(*record, "newDamage", wasmDamage) && ReadWasmFloatField(*record, "impulseMult", wasmImpulse);
	if (!fields && hasWasmResult && wasmResult.IsUnit()) { wasmDamage = directResult.newDamage; wasmImpulse = directResult.impulseMult; fields = true; }
	if (m_UnitPreDamagedFuncPtr == nullptr) { if (fields) { if (newDamage != nullptr) *newDamage = wasmDamage; if (impulseMult != nullptr) *impulseMult = wasmImpulse; return wasmDamage == 0.0f && wasmImpulse == 0.0f; } return false; }
	const auto nativeToken = spring::benchmark_callins::Begin("native", "callin_unitpredamaged");
	DamageCallinResult nativeResult = {.newDamage = (newDamage != nullptr) ? *newDamage : damage, .impulseMult = (impulseMult != nullptr) ? *impulseMult : 1.0f};
	m_UnitPreDamagedFuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult);
	spring::benchmark_callins::End(nativeToken);
	if (newDamage != nullptr) *newDamage = nativeResult.newDamage;
	if (impulseMult != nullptr) *impulseMult = nativeResult.impulseMult;
	return nativeResult.newDamage == 0.0f && nativeResult.impulseMult == 0.0f;
}

bool NativeInterfaceEventClient::FeaturePreDamaged(const CFeature* feature, const CUnit* attacker, float damage, int weaponDefID, int projectileID, float* newDamage, float* impulseMult) { FeatureDamagedQuery query = {.featureID = feature->id, .featureDefID = (feature->def != nullptr) ? feature->def->id : -1, .featureTeam = feature->team, .damage = damage, .weaponDefID = weaponDefID, .projectileID = projectileID, .attackerID = (attacker != nullptr) ? attacker->id : -1, .attackerDefID = (attacker != nullptr && attacker->unitDef != nullptr) ? attacker->unitDef->id : -1, .attackerTeam = (attacker != nullptr) ? attacker->team : -1}; WasmValue wasmResult; const bool has = DispatchWasmCallin("FeaturePreDamaged", &query, true, &wasmResult); float wasmDamage = (newDamage != nullptr) ? *newDamage : damage; float wasmImpulse = (impulseMult != nullptr) ? *impulseMult : 1.0f; const WasmValueRecord* record = has ? WasmResultRecord(wasmResult) : nullptr; const bool fields = record != nullptr && ReadWasmFloatField(*record, "newDamage", wasmDamage) && ReadWasmFloatField(*record, "impulseMult", wasmImpulse); if (m_FeaturePreDamagedFuncPtr == nullptr) { if (fields) { if (newDamage != nullptr) *newDamage = wasmDamage; if (impulseMult != nullptr) *impulseMult = wasmImpulse; return wasmDamage == 0.0f && wasmImpulse == 0.0f; } return false; } DamageCallinResult nativeResult = {.newDamage = (newDamage != nullptr) ? *newDamage : damage, .impulseMult = (impulseMult != nullptr) ? *impulseMult : 1.0f}; m_FeaturePreDamagedFuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult); if (newDamage != nullptr) *newDamage = nativeResult.newDamage; if (impulseMult != nullptr) *impulseMult = nativeResult.impulseMult; return nativeResult.newDamage == 0.0f && nativeResult.impulseMult == 0.0f; }

bool NativeInterfaceEventClient::ShieldPreDamaged(const CProjectile* projectile, const CWeapon* shieldEmitter, const CUnit* shieldCarrier, bool bounceProjectile, const CWeapon* beamEmitter, const CUnit* beamCarrier, const float3& startPos, const float3& hitPos) { ShieldPreDamagedQuery query = {.projectileID = (projectile != nullptr) ? projectile->id : -1, .projectileOwnerID = (projectile != nullptr) ? static_cast<int32_t>(projectile->GetOwnerID()) : -1, .shieldWeaponNum = (shieldEmitter != nullptr) ? shieldEmitter->weaponNum + LUA_WEAPON_BASE_INDEX : -1, .shieldCarrierID = (shieldCarrier != nullptr) ? shieldCarrier->id : -1, .bounceProjectile = bounceProjectile, .beamEmitterWeaponNum = (projectile == nullptr && beamEmitter != nullptr) ? beamEmitter->weaponNum + LUA_WEAPON_BASE_INDEX : -1, .beamEmitterUnitID = (projectile == nullptr && beamCarrier != nullptr) ? beamCarrier->id : -1, .startPos = {.x = startPos.x, .y = startPos.y, .z = startPos.z}, .hitPos = {.x = hitPos.x, .y = hitPos.y, .z = hitPos.z}}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("ShieldPreDamaged", &query, true, wasmValue); if (m_ShieldPreDamagedFuncPtr == nullptr) return has && wasmValue; BoolCallinResult nativeResult = {.value = false}; m_ShieldPreDamagedFuncPtr(m_nativeInterface, m_moduleData, &query, &nativeResult); return nativeResult.value || (has && wasmValue); }

void NativeInterfaceEventClient::DownloadFailed(int ID, int errorID) { DownloadFailedQuery query = {.downloadID = ID, .errorID = errorID}; DispatchWasmCallin("DownloadFailed", &query, false); if (m_DownloadFailedFuncPtr) { DownloadFailedResult result = {}; m_DownloadFailedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::DownloadFinished(int ID) { DownloadFinishedQuery query = {.downloadID = ID}; DispatchWasmCallin("DownloadFinished", &query, false); if (m_DownloadFinishedFuncPtr) { DownloadFinishedResult result = {}; m_DownloadFinishedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::DownloadProgress(int ID, long downloaded, long total) { DownloadProgressQuery query = {.downloadID = ID, .downloaded = downloaded, .total = total}; DispatchWasmCallin("DownloadProgress", &query, false); if (m_DownloadProgressFuncPtr) { DownloadProgressResult result = {}; m_DownloadProgressFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::DownloadQueued(int ID, const std::string& archiveName, const std::string& archiveType) { DownloadQueuedQuery query = {.downloadID = ID, .archiveName = archiveName.c_str(), .archiveType = archiveType.c_str()}; DispatchWasmCallin("DownloadQueued", &query, false); if (m_DownloadQueuedFuncPtr) { DownloadQueuedResult result = {}; m_DownloadQueuedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::DownloadStarted(int ID) { DownloadStartedQuery query = {.downloadID = ID}; DispatchWasmCallin("DownloadStarted", &query, false); if (m_DownloadStartedFuncPtr) { DownloadStartedResult result = {}; m_DownloadStartedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::Save(zipFile archive) { ArchiveCallinQuery query = {.archive = archive}; DispatchWasmCallin("Save", &query, false); if (m_SaveFuncPtr) { ArchiveCallinResult result = {}; m_SaveFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::LastMessagePosition(const float3& pos) { LastMessagePositionQuery query = {.pos = {.x = pos.x, .y = pos.y, .z = pos.z}}; DispatchWasmCallin("LastMessagePosition", &query, false); if (m_LastMessagePositionFuncPtr) { LastMessagePositionResult result = {}; m_LastMessagePositionFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::UnsyncedHeightMapUpdate(const SRectangle& rect) { RectChangedQuery query = {.x1 = rect.x1, .z1 = rect.z1, .x2 = rect.x2, .z2 = rect.z2}; DispatchWasmCallin("UnsyncedHeightMapUpdate", &query, false); if (m_UnsyncedHeightMapUpdateFuncPtr) { RectChangedResult result = {}; m_UnsyncedHeightMapUpdateFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }

bool NativeInterfaceEventClient::KeyMapChanged() { SimpleCallinQuery query = {}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("KeyMapChanged", &query, false, wasmValue); if (m_KeyMapChangedFuncPtr) { BoolCallinResult result = {.value = false}; m_KeyMapChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); return result.value || (has && wasmValue); } return has && wasmValue; }

bool NativeInterfaceEventClient::KeyPress(int keyCode, int scanCode, bool isRepeat) { if (suppressNextKeyPress) { suppressNextKeyPress = false; return false; } const ActionList& actionList = (game != nullptr) ? game->GetLastActionList() : ActionList{}; std::vector<KeyAction> actions; actions.reserve(actionList.size()); for (const Action& action : actionList) actions.push_back({.command = action.command.c_str(), .extra = action.extra.c_str(), .boundWith = action.boundWith.c_str()}); const CKeySet keySet(keyCode); const std::string label = keySet.GetString(true); KeyPressQuery query = {.keyCode = SDL21_keysyms(keyCode), .alt = !!KeyInput::GetKeyModState(KMOD_ALT), .ctrl = !!KeyInput::GetKeyModState(KMOD_CTRL), .meta = !!KeyInput::GetKeyModState(KMOD_GUI), .shift = !!KeyInput::GetKeyModState(KMOD_SHIFT), .isRepeat = isRepeat, .label = label.c_str(), .utf32Char = 0, .scanCode = scanCode, .actionList = actions.data(), .actionCount = static_cast<uint32_t>(actions.size())}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("KeyPress", &query, false, wasmValue); if (m_KeyPressFuncPtr) { BoolCallinResult result = {.value = false}; m_KeyPressFuncPtr(m_nativeInterface, m_moduleData, &query, &result); return result.value || (has && wasmValue); } return has && wasmValue; }

bool NativeInterfaceEventClient::KeyRelease(int keyCode, int scanCode) { if (suppressNextKeyRelease) { suppressNextKeyRelease = false; return false; } const ActionList& actionList = (game != nullptr) ? game->GetLastActionList() : ActionList{}; std::vector<KeyAction> actions; actions.reserve(actionList.size()); for (const Action& action : actionList) actions.push_back({.command = action.command.c_str(), .extra = action.extra.c_str(), .boundWith = action.boundWith.c_str()}); const CKeySet keySet(keyCode); const std::string label = keySet.GetString(true); KeyReleaseQuery query = {.keyCode = SDL21_keysyms(keyCode), .alt = !!KeyInput::GetKeyModState(KMOD_ALT), .ctrl = !!KeyInput::GetKeyModState(KMOD_CTRL), .meta = !!KeyInput::GetKeyModState(KMOD_GUI), .shift = !!KeyInput::GetKeyModState(KMOD_SHIFT), .label = label.c_str(), .utf32Char = 0, .scanCode = scanCode, .actionList = actions.data(), .actionCount = static_cast<uint32_t>(actions.size())}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("KeyRelease", &query, false, wasmValue); if (m_KeyReleaseFuncPtr) { BoolCallinResult result = {.value = false}; m_KeyReleaseFuncPtr(m_nativeInterface, m_moduleData, &query, &result); return result.value || (has && wasmValue); } return has && wasmValue; }

bool NativeInterfaceEventClient::TextInput(const std::string& utf8) { TextInputQuery query = {.utf8 = utf8.c_str()}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("TextInput", &query, false, wasmValue); if (m_TextInputFuncPtr) { BoolCallinResult result = {.value = false}; m_TextInputFuncPtr(m_nativeInterface, m_moduleData, &query, &result); return result.value || (has && wasmValue); } return has && wasmValue; }
bool NativeInterfaceEventClient::TextEditing(const std::string& utf8, unsigned int start, unsigned int length) { TextEditingQuery query = {.utf8 = utf8.c_str(), .start = start, .length = length}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("TextEditing", &query, false, wasmValue); if (m_TextEditingFuncPtr) { BoolCallinResult result = {.value = false}; m_TextEditingFuncPtr(m_nativeInterface, m_moduleData, &query, &result); return result.value || (has && wasmValue); } return has && wasmValue; }

bool NativeInterfaceEventClient::MouseMove(int x, int y, int dx, int dy, int button) { const LuaMousePosition position = ToLuaMousePosition(x, y); MouseMoveQuery query = {.x = position.x, .y = position.y, .dx = dx, .dy = -dy, .button = button}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("MouseMove", &query, false, wasmValue); if (m_MouseMoveFuncPtr) { BoolCallinResult result = {.value = false}; m_MouseMoveFuncPtr(m_nativeInterface, m_moduleData, &query, &result); return result.value || (has && wasmValue); } return has && wasmValue; }
bool NativeInterfaceEventClient::MousePress(int x, int y, int button) { const LuaMousePosition position = ToLuaMousePosition(x, y); MousePressQuery query = {.x = position.x, .y = position.y, .button = button}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("MousePress", &query, false, wasmValue); if (m_MousePressFuncPtr) { BoolCallinResult result = {.value = false}; m_MousePressFuncPtr(m_nativeInterface, m_moduleData, &query, &result); return result.value || (has && wasmValue); } return has && wasmValue; }
void NativeInterfaceEventClient::MouseRelease(int x, int y, int button) { const LuaMousePosition position = ToLuaMousePosition(x, y); MouseReleaseQuery query = {.x = position.x, .y = position.y, .button = button}; DispatchWasmCallin("MouseRelease", &query, false); if (m_MouseReleaseFuncPtr) { MouseReleaseResult result = {}; m_MouseReleaseFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
bool NativeInterfaceEventClient::MouseWheel(bool up, float value) { MouseWheelQuery query = {.up = up, .value = value}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("MouseWheel", &query, false, wasmValue); if (m_MouseWheelFuncPtr) { BoolCallinResult result = {.value = false}; m_MouseWheelFuncPtr(m_nativeInterface, m_moduleData, &query, &result); return result.value || (has && wasmValue); } return has && wasmValue; }
bool NativeInterfaceEventClient::IsAbove(int x, int y) { const LuaMousePosition position = ToLuaMousePosition(x, y); ScreenPositionQuery query = {.x = position.x, .y = position.y}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("IsAbove", &query, false, wasmValue); if (m_IsAboveFuncPtr) { BoolCallinResult result = {.value = false}; m_IsAboveFuncPtr(m_nativeInterface, m_moduleData, &query, &result); return result.value || (has && wasmValue); } return has && wasmValue; }

std::string NativeInterfaceEventClient::GetTooltip(int x, int y) { const LuaMousePosition position = ToLuaMousePosition(x, y); ScreenPositionQuery query = {.x = position.x, .y = position.y}; std::string wasmValue; const bool has = DispatchWasmStringCallin("GetTooltip", &query, false, wasmValue); if (m_GetTooltipFuncPtr) { StringCallinResult result = {}; m_GetTooltipFuncPtr(m_nativeInterface, m_moduleData, &query, &result); if (result.value != nullptr && result.value[0] != '\0') return result.value; } return has ? wasmValue : ""; }

bool NativeInterfaceEventClient::DefaultCommand(const CUnit* unit, const CFeature* feature, int& cmd) { DefaultCommandQuery query = {.unitID = (unit != nullptr) ? unit->id : -1, .featureID = (feature != nullptr) ? feature->id : -1, .currentCommand = cmd}; WasmValue wasmResult; const bool has = DispatchWasmCallin("DefaultCommand", &query, false, &wasmResult); bool wasmValue = false; int wasmCommand = cmd; const WasmValueRecord* record = has ? WasmResultRecord(wasmResult) : nullptr; const bool fields = record != nullptr && ReadWasmBoolField(*record, "value", wasmValue) && ReadWasmIntField(*record, "command", wasmCommand); if (m_DefaultCommandFuncPtr) { DefaultCommandResult result = {.value = false, .command = cmd}; m_DefaultCommandFuncPtr(m_nativeInterface, m_moduleData, &query, &result); if (result.value) cmd = result.command; return result.value; } if (fields && wasmValue) cmd = wasmCommand; return fields && wasmValue; }

void NativeInterfaceEventClient::ActiveCommandChanged(const SCommandDescription* cmdDesc) { ActiveCommandChangedQuery query = {.cmdID = (cmdDesc != nullptr) ? cmdDesc->id : -1, .cmdType = (cmdDesc != nullptr) ? cmdDesc->type : -1, .name = (cmdDesc != nullptr) ? cmdDesc->name.c_str() : "", .action = (cmdDesc != nullptr) ? cmdDesc->action.c_str() : "", .tooltip = (cmdDesc != nullptr) ? cmdDesc->tooltip.c_str() : ""}; DispatchWasmCallin("ActiveCommandChanged", &query, false); if (m_ActiveCommandChangedFuncPtr) { ActiveCommandChangedResult result = {}; m_ActiveCommandChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::CameraRotationChanged(const float3& rot) { Float3CallinQuery query = {.value = {.x = rot.x, .y = rot.y, .z = rot.z}}; DispatchWasmCallin("CameraRotationChanged", &query, false); if (m_CameraRotationChangedFuncPtr) { Float3CallinResult result = {}; m_CameraRotationChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::CameraPositionChanged(const float3& pos) { Float3CallinQuery query = {.value = {.x = pos.x, .y = pos.y, .z = pos.z}}; DispatchWasmCallin("CameraPositionChanged", &query, false); if (m_CameraPositionChangedFuncPtr) { Float3CallinResult result = {}; m_CameraPositionChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }

bool NativeInterfaceEventClient::CommandNotify(const Command& cmd) { CommandNotifyQuery query = {.command = ToNativeCallinCommand(cmd)}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("CommandNotify", &query, false, wasmValue); if (m_CommandNotifyFuncPtr) { BoolCallinResult result = {.value = false}; m_CommandNotifyFuncPtr(m_nativeInterface, m_moduleData, &query, &result); return result.value || (has && wasmValue); } return has && wasmValue; }
bool NativeInterfaceEventClient::AddConsoleLine(const std::string& msg, const std::string& section, int level) { AddConsoleLineQuery query = {.message = msg.c_str(), .section = section.c_str(), .level = level}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("AddConsoleLine", &query, false, wasmValue); if (m_AddConsoleLineFuncPtr) { BoolCallinResult result = {.value = false}; m_AddConsoleLineFuncPtr(m_nativeInterface, m_moduleData, &query, &result); return result.value || (has && wasmValue); } return has && wasmValue; }
bool NativeInterfaceEventClient::GroupChanged(int groupID) { GroupChangedQuery query = {.groupID = groupID}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("GroupChanged", &query, false, wasmValue); if (m_GroupChangedFuncPtr) { BoolCallinResult result = {.value = false}; m_GroupChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); return result.value || (has && wasmValue); } return has && wasmValue; }

void NativeInterfaceEventClient::MiniMapRotationChanged(float newRot, float oldRot) { MiniMapRotationChangedQuery query = {.newRot = newRot, .oldRot = oldRot}; DispatchWasmCallin("MiniMapRotationChanged", &query, false); if (m_MiniMapRotationChangedFuncPtr) { SimpleCallinResult result = {}; m_MiniMapRotationChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::MiniMapStateChanged(bool isMinimized, bool isMaximized, bool isSlaved) { MiniMapStateChangedQuery query = {.isMinimized = isMinimized, .isMaximized = isMaximized, .isSlaved = isSlaved}; DispatchWasmCallin("MiniMapStateChanged", &query, false); if (m_MiniMapStateChangedFuncPtr) { SimpleCallinResult result = {}; m_MiniMapStateChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::MiniMapGeometryChanged(int2 newPos, int2 newDim, int2 oldPos, int2 oldDim) { MiniMapGeometryChangedQuery query = {.newPosX = newPos.x, .newPosY = newPos.y, .newDimX = newDim.x, .newDimY = newDim.y, .oldPosX = oldPos.x, .oldPosY = oldPos.y, .oldDimX = oldDim.x, .oldDimY = oldDim.y}; DispatchWasmCallin("MiniMapGeometryChanged", &query, false); if (m_MiniMapGeometryChangedFuncPtr) { SimpleCallinResult result = {}; m_MiniMapGeometryChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }

bool NativeInterfaceEventClient::GameSetup(const std::string& state, bool& ready, const std::vector<std::pair<int, std::string>>& playerStates) { std::vector<GameSetupPlayerState> states; states.reserve(playerStates.size()); for (const auto& [playerID, playerState] : playerStates) states.push_back({.playerID = playerID, .state = playerState.c_str()}); GameSetupQuery query = {.state = state.c_str(), .ready = ready, .playerStates = states.data(), .playerStateCount = static_cast<uint32_t>(states.size())}; DispatchWasmCallin("GameSetup", &query, false); if (m_GameSetupFuncPtr) { GameSetupResult result = {.handled = false, .ready = ready}; m_GameSetupFuncPtr(m_nativeInterface, m_moduleData, &query, &result); if (result.handled) ready = result.ready; return result.handled; } return false; }

std::string NativeInterfaceEventClient::WorldTooltip(const CUnit* unit, const CFeature* feature, const float3* groundPos) { WorldTooltipQuery query = {.kind = (unit != nullptr) ? 1 : ((feature != nullptr) ? 2 : ((groundPos != nullptr) ? 3 : 0)), .unitID = (unit != nullptr) ? unit->id : -1, .featureID = (feature != nullptr) ? feature->id : -1, .groundPos = (groundPos != nullptr) ? Float3{.x = groundPos->x, .y = groundPos->y, .z = groundPos->z} : Float3{}}; std::string wasmValue; const bool has = DispatchWasmStringCallin("WorldTooltip", &query, false, wasmValue); if (m_WorldTooltipFuncPtr) { StringCallinResult result = {}; m_WorldTooltipFuncPtr(m_nativeInterface, m_moduleData, &query, &result); if (result.value != nullptr && result.value[0] != '\0') return result.value; } return has ? wasmValue : ""; }

bool NativeInterfaceEventClient::MapDrawCmd(int playerID, int type, const float3* pos0, const float3* pos1, const std::string* label) { MapDrawCmdQuery query = {.playerID = playerID, .type = type, .hasPos0 = (pos0 != nullptr), .pos0 = (pos0 != nullptr) ? Float3{.x = pos0->x, .y = pos0->y, .z = pos0->z} : Float3{}, .hasPos1 = (pos1 != nullptr), .pos1 = (pos1 != nullptr) ? Float3{.x = pos1->x, .y = pos1->y, .z = pos1->z} : Float3{}, .hasLabel = (label != nullptr), .label = (label != nullptr) ? label->c_str() : ""}; bool wasmValue = false; const bool has = DispatchWasmBoolCallin("MapDrawCmd", &query, false, wasmValue); if (m_MapDrawCmdFuncPtr) { BoolCallinResult result = {.value = false}; m_MapDrawCmdFuncPtr(m_nativeInterface, m_moduleData, &query, &result); return result.value || (has && wasmValue); } return has && wasmValue; }

void NativeInterfaceEventClient::ViewResize() { const int winPosY_bl = (globalRendering != nullptr) ? globalRendering->screenSizeY - globalRendering->winSizeY - globalRendering->winPosY : 0; ViewResizeQuery query = {.screenSizeX = (globalRendering != nullptr) ? globalRendering->screenSizeX : 0, .screenSizeY = (globalRendering != nullptr) ? globalRendering->screenSizeY : 0, .screenPosX = (globalRendering != nullptr) ? globalRendering->screenPosX : 0, .screenPosY = (globalRendering != nullptr) ? globalRendering->screenPosY : 0, .windowSizeX = (globalRendering != nullptr) ? globalRendering->winSizeX : 0, .windowSizeY = (globalRendering != nullptr) ? globalRendering->winSizeY : 0, .windowPosX = (globalRendering != nullptr) ? globalRendering->winPosX : 0, .windowPosY = winPosY_bl, .windowBorderTop = (globalRendering != nullptr) ? globalRendering->winBorder[0] : 0, .windowBorderLeft = (globalRendering != nullptr) ? globalRendering->winBorder[1] : 0, .windowBorderBottom = (globalRendering != nullptr) ? globalRendering->winBorder[2] : 0, .windowBorderRight = (globalRendering != nullptr) ? globalRendering->winBorder[3] : 0, .viewSizeX = (globalRendering != nullptr) ? globalRendering->viewSizeX : 0, .viewSizeY = (globalRendering != nullptr) ? globalRendering->viewSizeY : 0, .viewPosX = (globalRendering != nullptr) ? globalRendering->viewPosX : 0, .viewPosY = (globalRendering != nullptr) ? globalRendering->viewPosY : 0}; DispatchWasmCallin("ViewResize", &query, false); if (m_ViewResizeFuncPtr) { ViewResizeResult result = {}; m_ViewResizeFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::SunChanged() { SunChangedQuery query = {}; DispatchWasmCallin("SunChanged", &query, false); if (m_SunChangedFuncPtr) { SunChangedResult result = {}; m_SunChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::FontsChanged() { SimpleCallinQuery query = {}; DispatchWasmCallin("FontsChanged", &query, false); if (m_FontsChangedFuncPtr) { SimpleCallinResult result = {}; m_FontsChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::GameProgress(int gameFrame) { GameProgressQuery query = {.gameFrame = gameFrame}; DispatchWasmCallin("GameProgress", &query, false); if (m_GameProgressFuncPtr) { GameProgressResult result = {}; m_GameProgressFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::StockpileChanged(const CUnit* unit, const CWeapon* weapon, int oldCount) { StockpileChangedQuery query = {.unitID = unit->id, .unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1, .unitTeam = unit->team, .weaponNum = (weapon != nullptr) ? weapon->weaponNum + 1 : -1, .oldCount = oldCount, .newCount = (weapon != nullptr) ? weapon->numStockpiled : -1}; DispatchWasmCallin("StockpileChanged", &query, true); if (m_StockpileChangedFuncPtr) { StockpileChangedResult result = {}; m_StockpileChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::CollectGarbage(bool forced) { CollectGarbageQuery query = {.forced = forced}; DispatchWasmCallin("CollectGarbage", &query, false); if (m_CollectGarbageFuncPtr) { CollectGarbageResult result = {}; m_CollectGarbageFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::Pong(uint8_t pingTag, const spring_time pktSendTime, const spring_time pktRecvTime) { PongQuery query = {.pingTag = pingTag, .packetSendTimeMillis = pktSendTime.toMilliSecsi(), .packetRecvTimeMillis = pktRecvTime.toMilliSecsi()}; DispatchWasmCallin("Pong", &query, false); if (m_PongFuncPtr) { PongResult result = {}; m_PongFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }

void NativeInterfaceEventClient::HandleLuaMsg(int playerID, int script, int mode, const std::vector<std::uint8_t>& data) { HandleLuaMsgQuery query = {.playerID = playerID, .script = script, .mode = mode, .data = data.data(), .dataLength = static_cast<int32_t>(data.size())}; DispatchWasmCallin("HandleLuaMsg", &query, false); if (m_HandleLuaMsgFuncPtr) { HandleLuaMsgResult result = {}; m_HandleLuaMsgFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
void NativeInterfaceEventClient::HandleLuaCall(const char* msg, size_t msgLength, bool synced) { HandleLuaCallQuery query = {.message = msg, .messageLength = static_cast<uint32_t>(msgLength)}; DispatchWasmCallin("HandleLuaCall", &query, synced); if (m_HandleLuaCallFuncPtr) { HandleLuaCallResult result = {}; ScopedNativeSyncedCode syncedCode(synced); m_HandleLuaCallFuncPtr(m_nativeInterface, m_moduleData, &query, &result); } }
