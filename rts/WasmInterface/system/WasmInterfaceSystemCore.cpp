/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmInterfaceSystem.h"

#include <algorithm>
#include <array>
#include <string>
#include <string_view>

#include "NativeInterface/api/Callins.h"
#include "WasmCoreHost.h"
#include "WasmCoreUiCallinFilter.h"
#include "wasm/generated/WasmCallinRegistry.h"

namespace {

WasmInterfaceSystem*& ActiveCoreSystem()
{
	static WasmInterfaceSystem* system = nullptr;
	return system;
}

constexpr std::size_t CORE_CALLIN_COUNT =
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

CoreAggregation ResolveAggregation(std::string_view value)
{
	if (value == "ignore") return CoreAggregation::Ignore;
	if (value == "or-true") return CoreAggregation::OrTrue;
	if (value == "and-false") return CoreAggregation::AndFalse;
	if (value == "first") return CoreAggregation::First;
	if (value == "first-non-empty") return CoreAggregation::FirstNonEmpty;
	return CoreAggregation::Unsupported;
}

CoreResultKind ResolveResultKind(std::string_view value, CoreAggregation aggregation)
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

std::string& CoreStringResultStorage()
{
	// Native callin results expose const char*. Keep the final aggregate in
	// host-owned storage after returning from the guest; never expose a guest
	// linear-memory pointer through the native result record.
	thread_local std::string storage;
	return storage;
}

bool DispatchCoreModule(const WasmInterfaceSystem::CoreCallinInvocation& invocation,
	WasmCoreHost* host, const WasmModuleDescriptor& module, WasmCoreCallin callin,
	std::string_view name, void* result, std::string& error)
{
	std::string moduleError;
	const auto dispatchStage = name == "DrawWorld"
		? spring::benchmark_callins::BeginStage("wasm", "callin_drawworld_module_dispatch")
		: spring::benchmark_callins::Token{};
	if (WasmCoreHost::DispatchModule(host, callin, invocation.query, result, moduleError))
	{
		spring::benchmark_callins::End(dispatchStage);
		return true;
	}
	spring::benchmark_callins::End(dispatchStage);
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
	const void* query, bool synced, void* nativeResult, bool& handled,
	std::string& error)
{
	handled = false;
	WasmInterfaceSystem* system = ActiveCoreSystem();
	if (system == nullptr || system->coreModules.empty())
		return true;

	const WasmCoreCallin callin = WasmCoreHost::ResolveCallin(name);
	if (callin == WasmCoreCallin::Invalid)
		return true;
	const auto selectionStage = name == "DrawWorld"
		? spring::benchmark_callins::BeginStage("wasm", "callin_drawworld_core_selection")
		: spring::benchmark_callins::Token{};

	// Budgets are frame-scoped rather than call-scoped. Reset every synced
	// instance immediately before the simulation GameFrame boundary and every
	// unsynced/UI instance immediately before the Update boundary. This makes
	// all later callins/callouts in the same frame share one deterministic
	// allowance and never lets a guest reset its own window through re-entry.
	const bool resetBudgetWindow =
		(synced && callin == WasmCoreCallin::GameFrame) ||
		(!synced && callin == WasmCoreCallin::Update);
	if (resetBudgetWindow) {
		for (CoreModuleRecord& module : system->coreModules) {
			if (WasmEnvironmentMatrix::Policy(module.descriptor.environment).synced != synced)
				continue;
			if (module.host == nullptr)
				module.host = WasmCoreHost::ModuleHandle(module.descriptor.name);
			std::string resetError;
			if (!WasmCoreHost::ResetBudget(module.host, resetError)) {
				error = "Core Wasm budget reset failed in module " +
					module.descriptor.name + ": " + resetError;
				WasmCoreHost::FaultModule(module.descriptor.name, error);
				return false;
			}
		}
	}

	if (system->coreEnvironmentMask == 0)
		return true;
	const auto hasCoreEnvironment = [mask = system->coreEnvironmentMask](WasmEnvironment environment) {
		const std::uint32_t bit = 1u << static_cast<std::uint32_t>(environment);
		return (mask & bit) != 0;
	};

	static constexpr std::array<WasmEnvironment, 2> syncedEnvironments{
		WasmEnvironment::RulesSynced, WasmEnvironment::GaiaSynced};
	static constexpr std::array<WasmEnvironment, 2> unsyncedEnvironments{
		WasmEnvironment::RulesUnsynced, WasmEnvironment::GaiaUnsynced};

	std::array<CoreCallinInvocation, 3> invocations{};
	std::size_t invocationCount = 0;
	const auto& primary = synced ? syncedEnvironments : unsyncedEnvironments;
	for (const WasmEnvironment environment : primary) {
		if (hasCoreEnvironment(environment))
			invocations[invocationCount++] = {environment, query, true};
	}

	recoil::wasm::core::UiCallinFilter uiFilter;
	if (hasCoreEnvironment(WasmEnvironment::UI)) {
		bool includeUi = true;
		const void* uiQuery = query;
		if (!uiFilter.Prepare(name, query, includeUi, uiQuery, error))
			return false;
		if (includeUi) {
			// EventHandler discards UI return values for these synced-control
			// events. The UI callback still runs, but cannot change simulation
	// aggregation. Keep this identical to the Core path.
			const bool contributesResult = name != "Explosion" &&
				name != "UnitUnitCollision" && name != "UnitFeatureCollision";
			invocations[invocationCount++] = {
				WasmEnvironment::UI, uiQuery, contributesResult};
		}
	}

	if (invocationCount == 0) {
		spring::benchmark_callins::End(selectionStage);
		return true;
	}

	spring::benchmark_callins::End(selectionStage);
	const auto aggregationStage = name == "DrawWorld"
		? spring::benchmark_callins::BeginStage("wasm", "callin_drawworld_core_aggregation")
		: spring::benchmark_callins::Token{};
	const bool success = system->DispatchCoreCallin(callin, name,
		std::span<const CoreCallinInvocation>(invocations.data(), invocationCount),
		nativeResult, handled, error);
	spring::benchmark_callins::End(aggregationStage);
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

bool WasmInterfaceSystem::DispatchCoreCallin(std::string_view name,
	std::span<const CoreCallinInvocation> invocations,
	void* nativeResult, bool& handled,
	std::string& error)
{
	const WasmCoreCallin callin = WasmCoreHost::ResolveCallin(name);
	if (callin == WasmCoreCallin::Invalid) {
		handled = false;
		error = "unknown Core Wasm callin: " + std::string(name);
		return false;
	}
	return DispatchCoreCallin(callin, name, invocations, nativeResult,
		handled, error);
}

bool WasmInterfaceSystem::DispatchCoreCallin(WasmCoreCallin callin,
	std::string_view diagnosticName, std::span<const CoreCallinInvocation> invocations,
	void* nativeResult, bool& handled,
	std::string& error)
{
	handled = false;
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
	BoolCallinResult boolDefault = {
		.error = nullptr,
		.value = aggregation == CoreAggregation::AndFalse,
	};
	if (nativeResult != nullptr && resultKind == CoreResultKind::Bool)
		boolDefault = *static_cast<const BoolCallinResult*>(nativeResult);
	BoolCallinResult boolAggregate = boolDefault;
	if (aggregation == CoreAggregation::AndFalse && nativeResult == nullptr)
		boolAggregate.value = true;

	IntCallinResult intDefault = {
		.error = nullptr,
		.value = 0,
	};
	if (nativeResult != nullptr && resultKind == CoreResultKind::Int)
		intDefault = *static_cast<const IntCallinResult*>(nativeResult);
	IntCallinResult intAggregate = intDefault;

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

	std::string stringAggregate;

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
				BoolCallinResult moduleResult = boolDefault;
				if (aggregation == CoreAggregation::OrTrue && nativeResult == nullptr)
					moduleResult.value = false;
				if (aggregation == CoreAggregation::AndFalse && nativeResult == nullptr)
					moduleResult.value = true;
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

			if (resultKind == CoreResultKind::Bool && aggregation == CoreAggregation::First) {
				BoolCallinResult moduleResult = boolDefault;
				if (!DispatchCoreModule(invocation, module.host, module.descriptor,
						callin, diagnosticName, &moduleResult, error))
					return false;
				if (invocation.contributesResult && !haveResult) {
					boolAggregate = moduleResult;
					haveResult = true;
				}
				continue;
			}

			if (resultKind == CoreResultKind::Int && aggregation == CoreAggregation::First) {
				IntCallinResult moduleResult = intDefault;
				if (!DispatchCoreModule(invocation, module.host, module.descriptor,
						callin, diagnosticName, &moduleResult, error))
					return false;
				if (invocation.contributesResult && !haveResult) {
					intAggregate = moduleResult;
					haveResult = true;
				}
				continue;
			}

			if (resultKind == CoreResultKind::Damage && aggregation == CoreAggregation::First) {
				if (invocation.query == nullptr) {
					error = "Core damage callin dispatch received a null query";
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

			if (resultKind == CoreResultKind::String &&
				aggregation == CoreAggregation::FirstNonEmpty) {
				StringCallinResult moduleResult = {
					.error = nullptr,
					.value = nullptr,
				};
				const bool mayContribute = invocation.contributesResult && !haveResult;
				if (!DispatchCoreModule(invocation, module.host, module.descriptor,
						callin, diagnosticName, mayContribute ? &moduleResult : nullptr, error))
					return false;
				if (mayContribute && moduleResult.value != nullptr && moduleResult.value[0] != '\0') {
					stringAggregate.assign(moduleResult.value);
					haveResult = true;
				}
				continue;
			}

			if (resultKind == CoreResultKind::OpaqueFirst && aggregation == CoreAggregation::First) {
				void* moduleResult = nullptr;
				const bool takeResult = invocation.contributesResult && !haveResult;
				if (takeResult)
					moduleResult = nativeResult;
				if (!DispatchCoreModule(invocation, module.host, module.descriptor,
						callin, diagnosticName, moduleResult, error))
					return false;
				if (takeResult)
					haveResult = true;
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
		return true;
	}

	if (resultKind == CoreResultKind::Int) {
		if (nativeResult != nullptr)
			*static_cast<IntCallinResult*>(nativeResult) = intAggregate;
		return true;
	}

	if (resultKind == CoreResultKind::Damage) {
		if (nativeResult != nullptr)
			*static_cast<DamageCallinResult*>(nativeResult) = damageAggregate;
		return true;
	}

	if (resultKind == CoreResultKind::AllowUnitCreation) {
		if (nativeResult != nullptr)
			*static_cast<AllowUnitCreationResult*>(nativeResult) = creationAggregate;
		return true;
	}

	if (resultKind == CoreResultKind::String) {
		std::string& storage = CoreStringResultStorage();
		storage = std::move(stringAggregate);
		if (nativeResult != nullptr) {
			auto* typedResult = static_cast<StringCallinResult*>(nativeResult);
			typedResult->error = nullptr;
			typedResult->value = storage.c_str();
		}
		return true;
	}

	// OpaqueFirst has already written the first contributing native result in place.
	return true;
}
