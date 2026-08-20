/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmInterfaceSystem.h"

#include <algorithm>
#include <array>
#include <optional>
#include <string_view>
#include <unordered_map>

#include "NativeInterface/WasmUiVisibility.h"
#include "NativeInterface/api/Callins.h"
#include "WasmCoreHost.h"
#include "wasm/generated/WasmCallinRegistry.h"

namespace {

WasmInterfaceSystem*& ActiveCoreSystem()
{
	static WasmInterfaceSystem* system = nullptr;
	return system;
}

const recoil::wasm::generated::CallinDescriptor* FindCoreCallin(std::string_view name)
{
	static const std::unordered_map<std::string_view,
		const recoil::wasm::generated::CallinDescriptor*> index = [] {
		std::unordered_map<std::string_view,
			const recoil::wasm::generated::CallinDescriptor*> entries;
		for (const auto& callin : recoil::wasm::generated::kCallins)
			entries.emplace(callin.name, &callin);
		return entries;
	}();
	const auto iter = index.find(name);
	return iter == index.end() ? nullptr : iter->second;
}

bool Is(std::string_view value, const char* expected)
{
	return value == expected;
}

bool ResolveCoreDispatchSide(std::string_view name, bool& synced)
{
	if (name == "GameFrame" || name == "GameFramePost" ||
		name == "UnitCreated" || name == "UnitPreDamaged" ||
		name == "AllowUnitCreation") {
		synced = true;
		return true;
	}
	if (name == "Update" || name == "AddConsoleLine" ||
		name == "CommandNotify" || name == "DrawWorld") {
		synced = false;
		return true;
	}
	return false;
}

bool DispatchCoreModule(const WasmInterfaceSystem::CoreCallinInvocation& invocation,
	const WasmModuleDescriptor& module, std::string_view name, void* result,
	std::string& error)
{
	std::string moduleError;
	if (WasmCoreHost::DispatchModule(module.name, name, invocation.query, result, moduleError))
		return true;
	error = "Core Wasm callin " + std::string(name) + " failed in module " +
		module.name + ": " + moduleError;
	return false;
}

} // namespace

WasmInterfaceSystem::CoreDispatchRegistration::CoreDispatchRegistration(
	WasmInterfaceSystem* system)
	: owner(system)
	, previous(ActiveCoreSystem())
{
	ActiveCoreSystem() = owner;
}

WasmInterfaceSystem::CoreDispatchRegistration::~CoreDispatchRegistration()
{
	if (ActiveCoreSystem() == owner)
		ActiveCoreSystem() = previous;
}

bool WasmInterfaceSystem::DispatchActiveCoreCallin(std::string_view name,
	const void* query, void* nativeResult, bool& handled, std::string& error)
{
	handled = false;
	WasmInterfaceSystem* system = ActiveCoreSystem();
	if (system == nullptr || !WasmCoreHost::AnyActive())
		return true;

	bool synced = false;
	if (!ResolveCoreDispatchSide(name, synced))
		return true;

	static constexpr std::array<WasmEnvironment, 2> syncedEnvironments{
		WasmEnvironment::RulesSynced, WasmEnvironment::GaiaSynced};
	static constexpr std::array<WasmEnvironment, 2> unsyncedEnvironments{
		WasmEnvironment::RulesUnsynced, WasmEnvironment::GaiaUnsynced};

	std::vector<CoreCallinInvocation> invocations;
	invocations.reserve(3);
	const auto& primary = synced ? syncedEnvironments : unsyncedEnvironments;
	for (const WasmEnvironment environment : primary) {
		if (system->HasCoreModules(environment))
			invocations.push_back({environment, query, true});
	}

	std::optional<UnitCreatedQuery> uiUnitCreated;
	if (system->HasCoreModules(WasmEnvironment::UI)) {
		bool includeUi = true;
		const void* uiQuery = query;
		if (name == "UnitCreated") {
			const auto* typed = static_cast<const UnitCreatedQuery*>(query);
			if (typed == nullptr) {
				error = "Core UnitCreated dispatch received a null query";
				return false;
			}
			WasmUiVisibility::ScopedContext uiContext(true);
			includeUi = WasmUiVisibility::IsTeamVisible(typed->unitTeam);
			if (includeUi) {
				uiUnitCreated = *typed;
				if (uiUnitCreated->builderID >= 0 &&
					WasmUiVisibility::FindUnit(uiUnitCreated->builderID,
						WasmUiVisibility::UnitAccess::Visible) == nullptr)
					uiUnitCreated->builderID = -1;
				uiQuery = &*uiUnitCreated;
			}
		}
		if (includeUi)
			invocations.push_back({WasmEnvironment::UI, uiQuery, true});
	}

	if (invocations.empty())
		return true;

	const bool success = system->DispatchCoreCallin(name, invocations, nullptr,
		nativeResult, handled, error);
	if (!synced) {
		// Unsynced faults are isolated/removable. Keep the system's descriptor
		// inventory in sync with the host registry immediately so a faulted UI or
		// unsynced gadget cannot remain visible to HasCoreModules/ModuleCount.
		WasmCoreHost::RemoveFaultedUnsynced();
		system->coreModules.erase(std::remove_if(system->coreModules.begin(),
			system->coreModules.end(), [](const CoreModuleRecord& module) {
				return !WasmCoreHost::HasModule(module.descriptor.name);
			}), system->coreModules.end());
	}
	return success;
}

bool WasmInterfaceSystem::HasCoreModules(WasmEnvironment environment) const
{
	return std::any_of(coreModules.begin(), coreModules.end(),
		[environment](const CoreModuleRecord& module) {
			return module.descriptor.environment == environment;
		});
}

bool WasmInterfaceSystem::HasComponentModules(WasmEnvironment environment) const
{
	return std::any_of(modules.begin(), modules.end(),
		[environment](const std::unique_ptr<WasmModule>& module) {
			return module->Descriptor().environment == environment;
		});
}

bool WasmInterfaceSystem::DispatchCoreCallin(std::string_view name,
	const std::vector<CoreCallinInvocation>& invocations,
	WasmValue* valueResult, void* nativeResult, bool& handled,
	std::string& error)
{
	handled = false;
	if (valueResult != nullptr)
		*valueResult = WasmValue::Unit();

	const auto* descriptor = FindCoreCallin(name);
	if (descriptor == nullptr) {
		error = "unknown Core Wasm callin: " + std::string(name);
		return false;
	}

	const std::string_view aggregation = descriptor->aggregation;
	const std::string_view resultType = descriptor->result;
	bool haveResult = false;

	BoolCallinResult boolAggregate = {
		.error = nullptr,
		.value = Is(aggregation, "and-false"),
	};
	DamageCallinResult damageDefault = {
		.error = nullptr,
		.newDamage = 0.0f,
		.impulseMult = 1.0f,
	};
	if (nativeResult != nullptr && Is(resultType, "DamageCallinResult"))
		damageDefault = *static_cast<const DamageCallinResult*>(nativeResult);
	DamageCallinResult damageAggregate = damageDefault;
	AllowUnitCreationResult creationDefault = {
		.error = nullptr,
		.allow = true,
		.dropOrder = true,
	};
	if (nativeResult != nullptr && Is(resultType, "AllowUnitCreationResult"))
		creationDefault = *static_cast<const AllowUnitCreationResult*>(nativeResult);
	AllowUnitCreationResult creationAggregate = creationDefault;

	for (const CoreCallinInvocation& invocation : invocations) {
		const std::uint32_t environmentBit =
			1u << static_cast<std::uint32_t>(invocation.environment);
		if ((descriptor->environmentMask & environmentBit) == 0)
			continue;

		for (const CoreModuleRecord& module : coreModules) {
			if (module.descriptor.environment != invocation.environment ||
				!WasmCoreHost::ModuleHasCallin(module.descriptor.name, name))
				continue;

			handled = true;
			if (Is(aggregation, "ignore")) {
				if (!DispatchCoreModule(invocation, module.descriptor, name, nullptr, error))
					return false;
				continue;
			}

			if (Is(resultType, "BoolCallinResult") &&
				(Is(aggregation, "or-true") || Is(aggregation, "and-false"))) {
				BoolCallinResult moduleResult = {
					.error = nullptr,
					.value = Is(aggregation, "and-false"),
				};
				if (!DispatchCoreModule(invocation, module.descriptor, name, &moduleResult, error))
					return false;
				if (!invocation.contributesResult)
					continue;
				if (Is(aggregation, "or-true"))
					boolAggregate.value = boolAggregate.value || moduleResult.value;
				else
					boolAggregate.value = boolAggregate.value && moduleResult.value;
				haveResult = true;
				continue;
			}

			if (Is(resultType, "DamageCallinResult") && Is(aggregation, "first")) {
				if (invocation.query == nullptr) {
					error = "Core UnitPreDamaged dispatch received a null query";
					return false;
				}
				DamageCallinResult moduleResult = damageDefault;
				if (!DispatchCoreModule(invocation, module.descriptor, name, &moduleResult, error))
					return false;
				if (invocation.contributesResult && !haveResult) {
					damageAggregate = moduleResult;
					haveResult = true;
				}
				continue;
			}

			if (Is(resultType, "AllowUnitCreationResult") && Is(aggregation, "first")) {
				AllowUnitCreationResult moduleResult = creationDefault;
				if (!DispatchCoreModule(invocation, module.descriptor, name, &moduleResult, error))
					return false;
				if (invocation.contributesResult && !haveResult) {
					creationAggregate = moduleResult;
					haveResult = true;
				}
				continue;
			}

			error = "native Core aggregation is not implemented for callin " +
				std::string(name) + " (result " + std::string(resultType) +
				", aggregation " + std::string(aggregation) + ")";
			return false;
		}
	}

	if (!handled || !haveResult)
		return true;

	if (Is(resultType, "BoolCallinResult")) {
		if (nativeResult != nullptr)
			*static_cast<BoolCallinResult*>(nativeResult) = boolAggregate;
		if (valueResult != nullptr) {
			*valueResult = WasmValue::Record({
				{"value", WasmValue::Bool(boolAggregate.value)},
			});
		}
		return true;
	}

	if (Is(resultType, "DamageCallinResult")) {
		if (nativeResult != nullptr)
			*static_cast<DamageCallinResult*>(nativeResult) = damageAggregate;
		if (valueResult != nullptr) {
			*valueResult = WasmValue::Record({
				{"newDamage", WasmValue::F64(damageAggregate.newDamage)},
				{"impulseMult", WasmValue::F64(damageAggregate.impulseMult)},
			});
		}
		return true;
	}

	if (Is(resultType, "AllowUnitCreationResult")) {
		if (nativeResult != nullptr)
			*static_cast<AllowUnitCreationResult*>(nativeResult) = creationAggregate;
		if (valueResult != nullptr) {
			*valueResult = WasmValue::Record({
				{"allow", WasmValue::Bool(creationAggregate.allow)},
				{"dropOrder", WasmValue::Bool(creationAggregate.dropOrder)},
			});
		}
		return true;
	}

	error = "native Core result conversion is not implemented for callin " +
		std::string(name);
	return false;
}
