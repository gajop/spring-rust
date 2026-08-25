/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmInterfaceSystem.h"

#include <algorithm>
#include <array>
#include <string>
#include <string_view>

#include "NativeInterface/api/Callins.h"
#include "WasmCoreCallinPolicy.h"
#include "WasmCoreDispatchPlan.h"
#include "WasmCoreUiCallinFilter.h"
#include "WasmCoreHost.h"
#include "WasmCoreUiCallinFilter.h"
#include "wasm/generated/WasmCallinRegistry.h"

namespace {

WasmInterfaceSystem*& ActiveCoreSystem()
{
	static WasmInterfaceSystem* system = nullptr;
	return system;
}

using recoil::wasm::core::CallinName;
using recoil::wasm::core::CallinPolicy;
using recoil::wasm::core::CoreAggregation;
using recoil::wasm::core::CoreCallinPolicy;
using recoil::wasm::core::CoreResultKind;
using recoil::wasm::core::CORE_CALLIN_COUNT;

std::string& CoreStringResultStorage()
{
	// Native callin results expose const char*. Keep the final aggregate in
	// host-owned storage after returning from the guest; never expose a guest
	// linear-memory pointer through the native result record.
	thread_local std::string storage;
	return storage;
}

bool DispatchCoreModule(const recoil::wasm::core::WasmCoreDispatchPlan* plan,
	WasmCoreCallin callin, const void* query, void* result, std::string& error)
{
	const auto dispatchStage = spring::benchmark_callins::BeginStage(
		spring::benchmark_callins::Stage::ModuleDispatch);
	if (recoil::wasm::core::DispatchPlan(plan, query, result, error)) {
		spring::benchmark_callins::End(dispatchStage);
		return true;
	}
	spring::benchmark_callins::End(dispatchStage);
	// Only the failure path pays for building a message.
	error = "Core Wasm callin " + std::string(CallinName(callin)) + " failed in module " +
		std::string(WasmCoreHost::ModuleName(
			recoil::wasm::core::PlanHost(plan))) + ": " + error;
	return false;
}

} // namespace

void WasmInterfaceSystem::CoreSubscriberIndex::Rebuild(
	const std::vector<CoreModuleRecord>& modules)
{
	plans.clear();
	routes.fill(CallinRoute{});

	// Group subscribers by (callin, environment) so a dispatch reads one
	// contiguous span and never re-tests the generated environment mask or the
	// per-module callin bitset.
	for (std::size_t ordinal = 1; ordinal < CORE_CALLIN_COUNT; ++ordinal) {
		const auto callin = static_cast<WasmCoreCallin>(ordinal);
		const CoreCallinPolicy& policy = CallinPolicy(callin);
		if (!policy.valid)
			continue;
		// Fold the compile-time policy into the routing line so no dispatch has
		// to read the policy tables at all.
		CallinRoute& route = routes[ordinal];
		const recoil::wasm::core::UiCallinPolicy& uiPolicy =
			recoil::wasm::core::UI_CALLIN_POLICIES[ordinal];
		route.valid = true;
		route.aggregation = policy.aggregation;
		route.resultKind = policy.resultKind;
		route.uiNeedsFilter = uiPolicy.handler != nullptr;
		route.uiContributesResult = uiPolicy.contributesResult;
		for (const WasmEnvironment environment : WasmEnvironmentMatrix::All()) {
			const std::uint32_t bit = 1u << static_cast<std::uint32_t>(environment);
			if ((policy.environmentMask & bit) == 0)
				continue;
			Range& range = route.perEnvironment[static_cast<std::size_t>(environment)];
			range.begin = static_cast<std::uint32_t>(plans.size());
			for (const CoreModuleRecord& module : modules) {
				if (module.descriptor.environment != environment)
					continue;
				WasmCoreHost* host = module.host != nullptr
					? module.host
					: WasmCoreHost::ModuleHandle(module.descriptor.name);
				const auto* plan = WasmCoreHost::ModulePlan(host, callin);
				if (plan == nullptr)
					continue;
				plans.push_back(plan);
				++range.count;
			}
			if (range.count != 0)
				route.environmentMask |= bit;
		}
	}
}

const WasmInterfaceSystem::CoreSubscriberIndex::CallinRoute&
WasmInterfaceSystem::CoreSubscriberIndex::Route(WasmCoreCallin callin) const
{
	const std::size_t slot = static_cast<std::size_t>(callin);
	return routes[slot < routes.size() ? slot : 0u];
}

std::span<const recoil::wasm::core::WasmCoreDispatchPlan* const>
WasmInterfaceSystem::CoreSubscriberIndex::Plans(
	const CallinRoute& route, WasmEnvironment environment) const
{
	const Range& range = route.perEnvironment[static_cast<std::size_t>(environment)];
	return {plans.data() + range.begin, range.count};
}

const WasmInterfaceSystem::CoreSubscriberIndex& WasmInterfaceSystem::Subscribers()
{
	if (coreSubscribersDirty) {
		coreSubscribers.Rebuild(coreModules);
		coreSubscribersDirty = false;
	}
	return coreSubscribers;
}

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

bool WasmInterfaceSystem::DispatchActiveCoreCallin(WasmCoreCallin callin,
	const void* query, bool synced, void* nativeResult, bool& handled,
	std::string& error)
{
	handled = false;
	WasmInterfaceSystem* system = ActiveCoreSystem();
	if (system == nullptr || system->coreModules.empty())
		return true;
	if (callin == WasmCoreCallin::Invalid)
		return true;

	// Budgets are frame-scoped rather than call-scoped. Reset every synced
	// instance immediately before the simulation GameFrame boundary and every
	// unsynced/UI instance immediately before the Update boundary. This makes
	// all later callins/callouts in the same frame share one deterministic
	// allowance and never lets a guest reset its own window through re-entry.
	const bool resetBudgetWindow =
		(synced && callin == WasmCoreCallin::GameFrame) ||
		(!synced && callin == WasmCoreCallin::Update);
	if (resetBudgetWindow && !system->ResetBudgetWindow(synced, error))
		return false;

	const auto selectionStage = spring::benchmark_callins::BeginStage(
		spring::benchmark_callins::Stage::CoreSelection);

	// Which worlds this dispatch may reach is fixed by the callin id and the
	// loaded module set, so the common "nobody implements this" case costs one
	// mask test and never builds an invocation list or a UI perspective.
	static constexpr std::uint32_t syncedBits =
		(1u << static_cast<std::uint32_t>(WasmEnvironment::RulesSynced)) |
		(1u << static_cast<std::uint32_t>(WasmEnvironment::GaiaSynced));
	static constexpr std::uint32_t unsyncedBits =
		(1u << static_cast<std::uint32_t>(WasmEnvironment::RulesUnsynced)) |
		(1u << static_cast<std::uint32_t>(WasmEnvironment::GaiaUnsynced));
	static constexpr std::uint32_t uiBit =
		1u << static_cast<std::uint32_t>(WasmEnvironment::UI);
	static constexpr std::uint32_t menuBit =
		1u << static_cast<std::uint32_t>(WasmEnvironment::Menu);
	static constexpr std::uint32_t introBit =
		1u << static_cast<std::uint32_t>(WasmEnvironment::Intro);
	static constexpr std::uint32_t standaloneBits = uiBit | menuBit | introBit;

	const CoreSubscriberIndex& subscribers = system->Subscribers();
	const CoreSubscriberIndex::CallinRoute& route = subscribers.Route(callin);
	const std::uint32_t reachable =
		route.environmentMask & ((synced ? syncedBits : unsyncedBits) | standaloneBits);
	if (reachable == 0) {
		spring::benchmark_callins::End(selectionStage);
		return true;
	}

	static constexpr std::array<WasmEnvironment, 2> syncedEnvironments{
		WasmEnvironment::RulesSynced, WasmEnvironment::GaiaSynced};
	static constexpr std::array<WasmEnvironment, 2> unsyncedEnvironments{
		WasmEnvironment::RulesUnsynced, WasmEnvironment::GaiaUnsynced};

	// Callins that discard guest return values — every draw and frame event
	// among them — need no invocation list and no aggregation state. Run them
	// straight off the routing line already loaded above.
	if (route.aggregation == CoreAggregation::Ignore) {
		spring::benchmark_callins::End(selectionStage);
		const auto aggregationStage = spring::benchmark_callins::BeginStage(
			spring::benchmark_callins::Stage::CoreAggregation);
		bool success = system->DispatchIgnoredCallin(callin, route, query, reachable,
			synced ? syncedEnvironments : unsyncedEnvironments, handled, error);
		spring::benchmark_callins::End(aggregationStage);
		if (!synced && WasmCoreHost::PendingUnsyncedFaults() != 0)
			system->RemoveFaultedUnsyncedModules();
		return success;
	}

	std::array<CoreCallinInvocation, 5> invocations{};
	std::size_t invocationCount = 0;
	const auto& primary = synced ? syncedEnvironments : unsyncedEnvironments;
	for (const WasmEnvironment environment : primary) {
		if ((reachable & (1u << static_cast<std::uint32_t>(environment))) != 0)
			invocations[invocationCount++] = {environment, query, true};
	}

	recoil::wasm::core::UiCallinFilter uiFilter;
	if ((reachable & uiBit) != 0) {
		bool includeUi = true;
		const void* uiQuery = query;
		if (route.uiNeedsFilter &&
			!uiFilter.Prepare(callin, query, includeUi, uiQuery, error))
			return false;
		if (includeUi) {
			invocations[invocationCount++] = {WasmEnvironment::UI, uiQuery,
				route.uiContributesResult};
		}
	}

	if ((reachable & menuBit) != 0)
		invocations[invocationCount++] = {WasmEnvironment::Menu, query, true};
	if ((reachable & introBit) != 0)
		invocations[invocationCount++] = {WasmEnvironment::Intro, query, true};

	if (invocationCount == 0) {
		spring::benchmark_callins::End(selectionStage);
		return true;
	}

	spring::benchmark_callins::End(selectionStage);
	const auto aggregationStage = spring::benchmark_callins::BeginStage(
		spring::benchmark_callins::Stage::CoreAggregation);
	const bool success = system->DispatchCoreCallin(callin,
		std::span<const CoreCallinInvocation>(invocations.data(), invocationCount),
		nativeResult, handled, error);
	spring::benchmark_callins::End(aggregationStage);

	// Faults are rare. Only pay for the sweep when one actually happened.
	if (!synced && WasmCoreHost::PendingUnsyncedFaults() != 0)
		system->RemoveFaultedUnsyncedModules();
	return success;
}

bool WasmInterfaceSystem::DispatchIgnoredCallin(WasmCoreCallin callin,
	const CoreSubscriberIndex::CallinRoute& route, const void* query,
	std::uint32_t reachable, std::span<const WasmEnvironment> primary,
	bool& handled, std::string& error)
{
	const CoreSubscriberIndex& subscribers = coreSubscribers;
	const auto run = [&](WasmEnvironment environment, const void* environmentQuery) {
		for (const auto* plan : subscribers.Plans(route, environment)) {
			handled = true;
			if (!DispatchCoreModule(plan, callin, environmentQuery, nullptr, error))
				return false;
		}
		return true;
	};

	for (const WasmEnvironment environment : primary) {
		if ((reachable & (1u << static_cast<std::uint32_t>(environment))) == 0)
			continue;
		if (!run(environment, query))
			return false;
	}

	static constexpr std::uint32_t uiBit =
		1u << static_cast<std::uint32_t>(WasmEnvironment::UI);
	if ((reachable & uiBit) != 0) {
		if (!route.uiNeedsFilter) {
			if (!run(WasmEnvironment::UI, query))
				return false;
		} else {
			recoil::wasm::core::UiCallinFilter uiFilter;
			bool includeUi = true;
			const void* uiQuery = query;
			if (!uiFilter.Prepare(callin, query, includeUi, uiQuery, error))
				return false;
			if (includeUi && !run(WasmEnvironment::UI, uiQuery))
				return false;
		}
	}

	static constexpr std::uint32_t menuBit =
		1u << static_cast<std::uint32_t>(WasmEnvironment::Menu);
	static constexpr std::uint32_t introBit =
		1u << static_cast<std::uint32_t>(WasmEnvironment::Intro);
	if ((reachable & menuBit) != 0) {
		if (!run(WasmEnvironment::Menu, query))
			return false;
	}
	if ((reachable & introBit) != 0) {
		if (!run(WasmEnvironment::Intro, query))
			return false;
	}
	return true;
}

bool WasmInterfaceSystem::ResetBudgetWindow(bool synced, std::string& error)
{
	for (CoreModuleRecord& module : coreModules) {
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
	return true;
}

void WasmInterfaceSystem::RemoveFaultedUnsyncedModules()
{
	if (WasmCoreHost::RemoveFaultedUnsynced() == 0)
		return;
	coreModules.erase(std::remove_if(coreModules.begin(), coreModules.end(),
		[](const CoreModuleRecord& module) {
			return !WasmCoreHost::HasModule(module.descriptor.name);
		}), coreModules.end());
	InvalidateSubscribers();
}

bool WasmInterfaceSystem::HasCoreModules(WasmEnvironment environment) const
{
	return std::any_of(coreModules.begin(), coreModules.end(),
		[environment](const CoreModuleRecord& module) {
			return module.descriptor.environment == environment;
		});
}

bool WasmInterfaceSystem::DispatchCoreCallin(WasmCoreCallin callin,
	std::span<const CoreCallinInvocation> invocations,
	void* nativeResult, bool& handled,
	std::string& error)
{
	handled = false;
	const CoreCallinPolicy& policy = CallinPolicy(callin);
	if (!policy.valid) {
		error = "Core Wasm callin has no generated descriptor: " +
			std::string(CallinName(callin));
		return false;
	}
	const CoreAggregation aggregation = policy.aggregation;
	const CoreResultKind resultKind = policy.resultKind;
	if (aggregation == CoreAggregation::Unsupported || resultKind == CoreResultKind::Unsupported) {
		error = "native Core aggregation is not implemented for callin " +
			std::string(CallinName(callin)) + " (result " + policy.descriptor->result +
			", aggregation " + policy.descriptor->aggregation + ")";
		return false;
	}

	const CoreSubscriberIndex& subscribers = Subscribers();
	const CoreSubscriberIndex::CallinRoute& route = subscribers.Route(callin);

	// Most callins — every draw and frame event among them — discard guest
	// return values. Aggregation state for six result shapes is dead work
	// there, so that case never builds any.
	if (aggregation == CoreAggregation::Ignore) {
		for (const CoreCallinInvocation& invocation : invocations) {
			for (const auto* plan : subscribers.Plans(route, invocation.environment)) {
				handled = true;
				if (!DispatchCoreModule(plan, callin, invocation.query, nullptr, error))
					return false;
			}
		}
		return true;
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
		// The subscriber list already excludes modules whose world the callin does
		// not reach and modules that do not export it, so every entry here is a
		// guest that will actually run.
		for (const auto* plan : subscribers.Plans(route, invocation.environment)) {
			handled = true;
			if (resultKind == CoreResultKind::Bool &&
				(aggregation == CoreAggregation::OrTrue || aggregation == CoreAggregation::AndFalse)) {
				BoolCallinResult moduleResult = boolDefault;
				if (aggregation == CoreAggregation::OrTrue && nativeResult == nullptr)
					moduleResult.value = false;
				if (aggregation == CoreAggregation::AndFalse && nativeResult == nullptr)
					moduleResult.value = true;
				if (!DispatchCoreModule(plan, callin, invocation.query,
						&moduleResult, error))
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
				if (!DispatchCoreModule(plan, callin, invocation.query,
						&moduleResult, error))
					return false;
				if (invocation.contributesResult && !haveResult) {
					boolAggregate = moduleResult;
					haveResult = true;
				}
				continue;
			}

			if (resultKind == CoreResultKind::Int && aggregation == CoreAggregation::First) {
				IntCallinResult moduleResult = intDefault;
				if (!DispatchCoreModule(plan, callin, invocation.query,
						&moduleResult, error))
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
				if (!DispatchCoreModule(plan, callin, invocation.query,
						&moduleResult, error))
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
				if (!DispatchCoreModule(plan, callin, invocation.query,
						&moduleResult, error))
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
				if (!DispatchCoreModule(plan, callin, invocation.query,
						mayContribute ? &moduleResult : nullptr, error))
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
				if (!DispatchCoreModule(plan, callin, invocation.query,
						moduleResult, error))
					return false;
				if (takeResult)
					haveResult = true;
				continue;
			}

			error = "Core Wasm callin policy combination is unsupported: " +
				std::string(CallinName(callin));
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
