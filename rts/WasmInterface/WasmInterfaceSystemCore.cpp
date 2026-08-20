/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmInterfaceSystem.h"

#include <algorithm>
#include <array>
#include <optional>
#include <string_view>

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

constexpr std::size_t CORE_CALLIN_COUNT =
	static_cast<std::size_t>(WasmCoreCallin::DrawWorld) + 1;

enum class CoreAggregation : std::uint8_t {
	Ignore,
	OrTrue,
	AndFalse,
	First,
	Unsupported,
};

enum class CoreResultKind : std::uint8_t {
	None,
	Bool,
	Damage,
	AllowUnitCreation,
	Unsupported,
};

CoreAggregation ResolveAggregation(std::string_view value)
{
	if (value == "ignore") return CoreAggregation::Ignore;
	if (value == "or-true") return CoreAggregation::OrTrue;
	if (value == "and-false") return CoreAggregation::AndFalse;
	if (value == "first") return CoreAggregation::First;
	return CoreAggregation::Unsupported;
}

CoreResultKind ResolveResultKind(std::string_view value, CoreAggregation aggregation)
{
	if (aggregation == CoreAggregation::Ignore)
		return CoreResultKind::None;
	if (value == "BoolCallinResult") return CoreResultKind::Bool;
	if (value == "DamageCallinResult") return CoreResultKind::Damage;
	if (value == "AllowUnitCreationResult") return CoreResultKind::AllowUnitCreation;
	return CoreResultKind::Unsupported;
}

struct CoreCallinPolicy {
	const recoil::wasm::generated::CallinDescriptor* descriptor = nullptr;
	CoreAggregation aggregation = CoreAggregation::Unsupported;
	CoreResultKind resultKind = CoreResultKind::Unsupported;
};

const CoreCallinPolicy* FindCoreCallinPolicy(WasmCoreCallin callin)
{
	static const std::array<CoreCallinPolicy, CORE_CALLIN_COUNT> index = [] {
		std::array<CoreCallinPolicy, CORE_CALLIN_COUNT> entries{};
		for (const auto& descriptor : recoil::wasm::generated::kCallins) {
			const WasmCoreCallin key = WasmCoreHost::ResolveCallin(descriptor.name);
			const std::size_t slot = static_cast<std::size_t>(key);
			if (key == WasmCoreCallin::Invalid || slot >= entries.size())
				continue;
			const CoreAggregation aggregation = ResolveAggregation(descriptor.aggregation);
			entries[slot] = {
				.descriptor = &descriptor,
				.aggregation = aggregation,
				.resultKind = ResolveResultKind(descriptor.result, aggregation),
			};
		}
		return entries;
	}();
	const std::size_t slot = static_cast<std::size_t>(callin);
	if (slot >= index.size() || index[slot].descriptor == nullptr)
		return nullptr;
	return &index[slot];
}

bool ResolveCoreDispatchSide(WasmCoreCallin callin, bool& synced)
{
	switch (callin) {
		case WasmCoreCallin::GameFrame:
		case WasmCoreCallin::GameFramePost:
		case WasmCoreCallin::UnitCreated:
		case WasmCoreCallin::UnitPreDamaged:
		case WasmCoreCallin::AllowUnitCreation:
			synced = true;
			return true;
		case WasmCoreCallin::Update:
		case WasmCoreCallin::AddConsoleLine:
		case WasmCoreCallin::CommandNotify:
		case WasmCoreCallin::DrawWorld:
			synced = false;
			return true;
		case WasmCoreCallin::Invalid:
			return false;
	}
	return false;
}

bool DispatchCoreModule(const WasmInterfaceSystem::CoreCallinInvocation& invocation,
	WasmCoreHost* host, const WasmModuleDescriptor& module, WasmCoreCallin callin,
	std::string_view name, void* result, std::string& error)
{
	std::string moduleError;
	if (WasmCoreHost::DispatchModule(host, callin, invocation.query, result, moduleError))
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

	const WasmCoreCallin callin = WasmCoreHost::ResolveCallin(name);
	bool synced = false;
	if (!ResolveCoreDispatchSide(callin, synced))
		return true;

	static constexpr std::array<WasmEnvironment, 2> syncedEnvironments{
		WasmEnvironment::RulesSynced, WasmEnvironment::GaiaSynced};
	static constexpr std::array<WasmEnvironment, 2> unsyncedEnvironments{
		WasmEnvironment::RulesUnsynced, WasmEnvironment::GaiaUnsynced};

	std::array<CoreCallinInvocation, 3> invocations{};
	std::size_t invocationCount = 0;
	const auto& primary = synced ? syncedEnvironments : unsyncedEnvironments;
	for (const WasmEnvironment environment : primary) {
		if (system->HasCoreModules(environment))
			invocations[invocationCount++] = {environment, query, true};
	}

	std::optional<UnitCreatedQuery> uiUnitCreated;
	if (system->HasCoreModules(WasmEnvironment::UI)) {
		bool includeUi = true;
		const void* uiQuery = query;
		if (callin == WasmCoreCallin::UnitCreated) {
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
			invocations[invocationCount++] = {WasmEnvironment::UI, uiQuery, true};
	}

	if (invocationCount == 0)
		return true;

	const bool success = system->DispatchCoreCallin(callin, name,
		std::span<const CoreCallinInvocation>(invocations.data(), invocationCount),
		nullptr, nativeResult, handled, error);
	if (!synced && WasmCoreHost::RemoveFaultedUnsynced() != 0) {
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
	std::span<const CoreCallinInvocation> invocations,
	WasmValue* valueResult, void* nativeResult, bool& handled,
	std::string& error)
{
	const WasmCoreCallin callin = WasmCoreHost::ResolveCallin(name);
	if (callin == WasmCoreCallin::Invalid) {
		handled = false;
		if (valueResult != nullptr)
			*valueResult = WasmValue::Unit();
		error = "unknown Core Wasm callin: " + std::string(name);
		return false;
	}
	return DispatchCoreCallin(callin, name, invocations, valueResult, nativeResult,
		handled, error);
}

bool WasmInterfaceSystem::DispatchCoreCallin(WasmCoreCallin callin,
	std::string_view diagnosticName, std::span<const CoreCallinInvocation> invocations,
	WasmValue* valueResult, void* nativeResult, bool& handled,
	std::string& error)
{
	handled = false;
	if (valueResult != nullptr)
		*valueResult = WasmValue::Unit();
	if (callin == WasmCoreCallin::Invalid) {
		error = "unknown Core Wasm callin: " + std::string(diagnosticName);
		return false;
	}

	const CoreCallinPolicy* policy = FindCoreCallinPolicy(callin);
	if (policy == nullptr) {
		error = "Core Wasm callin has no generated descriptor: " +
			std::string(diagnosticName);
		return false;
	}
	const auto* descriptor = policy->descriptor;
	const CoreAggregation aggregation = policy->aggregation;
	const CoreResultKind resultKind = policy->resultKind;
	if (aggregation == CoreAggregation::Unsupported || resultKind == CoreResultKind::Unsupported) {
		error = "native Core aggregation is not implemented for callin " +
			std::string(diagnosticName) + " (result " + descriptor->result +
			", aggregation " + descriptor->aggregation + ")";
		return false;
	}

	bool haveResult = false;
	BoolCallinResult boolAggregate = {
		.error = nullptr,
		.value = aggregation == CoreAggregation::AndFalse,
	};
	DamageCallinResult damageDefault = {
		.error = nullptr,
		.newDamage = 0.0f,
		.impulseMult = 1.0f,
	};
	if (nativeResult != nullptr && resultKind == CoreResultKind::Damage)
		damageDefault = *static_cast<const DamageCallinResult*>(nativeResult);
	DamageCallinResult damageAggregate = damageDefault;
	AllowUnitCreationResult creationDefault = {
		.error = nullptr,
		.allow = true,
		.dropOrder = true,
	};
	if (nativeResult != nullptr && resultKind == CoreResultKind::AllowUnitCreation)
		creationDefault = *static_cast<const AllowUnitCreationResult*>(nativeResult);
	AllowUnitCreationResult creationAggregate = creationDefault;

	for (const CoreCallinInvocation& invocation : invocations) {
		const std::uint32_t environmentBit =
			1u << static_cast<std::uint32_t>(invocation.environment);
		if ((descriptor->environmentMask & environmentBit) == 0)
			continue;

		for (CoreModuleRecord& module : coreModules) {
			if (module.descriptor.environment != invocation.environment)
				continue;
			if (module.host == nullptr)
				module.host = WasmCoreHost::ModuleHandle(module.descriptor.name);
			if (!WasmCoreHost::ModuleHasCallin(module.host, callin))
				continue;

			handled = true;
			if (aggregation == CoreAggregation::Ignore) {
				if (!DispatchCoreModule(invocation, module.host, module.descriptor,
						callin, diagnosticName, nullptr, error))
					return false;
				continue;
			}

			if (resultKind == CoreResultKind::Bool &&
				(aggregation == CoreAggregation::OrTrue || aggregation == CoreAggregation::AndFalse)) {
				BoolCallinResult moduleResult = {
					.error = nullptr,
					.value = aggregation == CoreAggregation::AndFalse,
				};
				if (!DispatchCoreModule(invocation, module.host, module.descriptor,
						callin, diagnosticName, &moduleResult, error))
					return false;
				if (!invocation.contributesResult)
					continue;
				if (aggregation == CoreAggregation::OrTrue)
					boolAggregate.value = boolAggregate.value || moduleResult.value;
				else
					boolAggregate.value = boolAggregate.value && moduleResult.value;
				haveResult = true;
				continue;
			}

			if (resultKind == CoreResultKind::Damage && aggregation == CoreAggregation::First) {
				if (invocation.query == nullptr) {
					error = "Core UnitPreDamaged dispatch received a null query";
					return false;
				}
				DamageCallinResult moduleResult = damageDefault;
				if (!DispatchCoreModule(invocation, module.host, module.descriptor,
						callin, diagnosticName, &moduleResult, error))
					return false;
				if (invocation.contributesResult && !haveResult) {
					damageAggregate = moduleResult;
					haveResult = true;
				}
				continue;
			}

			if (resultKind == CoreResultKind::AllowUnitCreation &&
				aggregation == CoreAggregation::First) {
				AllowUnitCreationResult moduleResult = creationDefault;
				if (!DispatchCoreModule(invocation, module.host, module.descriptor,
						callin, diagnosticName, &moduleResult, error))
					return false;
				if (invocation.contributesResult && !haveResult) {
					creationAggregate = moduleResult;
					haveResult = true;
				}
				continue;
			}

			error = "Core Wasm callin policy combination is unsupported: " +
				std::string(diagnosticName);
			return false;
		}
	}

	if (!handled || !haveResult)
		return true;

	if (resultKind == CoreResultKind::Bool) {
		if (nativeResult != nullptr)
			*static_cast<BoolCallinResult*>(nativeResult) = boolAggregate;
		if (valueResult != nullptr) {
			*valueResult = WasmValue::Record({
				{"value", WasmValue::Bool(boolAggregate.value)},
			});
		}
		return true;
	}

	if (resultKind == CoreResultKind::Damage) {
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

	if (resultKind == CoreResultKind::AllowUnitCreation) {
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

	return true;
}
