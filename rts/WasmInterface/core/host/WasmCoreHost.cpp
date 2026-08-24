/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreHost.h"

#include <algorithm>
#include <array>
#include <memory>
#include <utility>

#include "NativeInterface/NativeInterface.h"
#include "NativeInterface/api/Callins.h"
#include "WasmCoreAbi.h"
#include "WasmCoreBindings.h"
#include "WasmCoreValidation.h"
#include "WasmCoreVariableCallins.h"
#include "WasmResources.h"

#if __has_include("../wasm/generated/WasmCoreGeneratedCallinBindings.h")
#include "../wasm/generated/WasmCoreGeneratedCallinBindings.h"
#define RECOIL_WASM_CORE_GENERATED_CALLIN_BINDINGS 1
#endif

#if __has_include("../wasm/generated/WasmCoreGeneratedScratchCallinBindings.h")
#include "../wasm/generated/WasmCoreGeneratedScratchCallinBindings.h"
#define RECOIL_WASM_CORE_GENERATED_SCRATCH_CALLIN_BINDINGS 1
#endif

namespace {

// Bumped when an unsynced guest faults and cleared by the sweep that removes
// it. Dispatch reads this instead of scanning the registry every call.
std::size_t& PendingUnsyncedFaultCount()
{
	static std::size_t pending = 0;
	return pending;
}

std::vector<std::unique_ptr<WasmCoreHost>>& Hosts()
{
	static std::vector<std::unique_ptr<WasmCoreHost>> hosts;
	return hosts;
}

constexpr std::size_t CALLIN_COUNT =
	sizeof(recoil::wasm::generated::kCallins) /
		sizeof(recoil::wasm::generated::kCallins[0]);

constexpr std::uint64_t HashCallin(std::string_view name)
{
	std::uint64_t hash = 14695981039346656037ull;
	for (const unsigned char character : name) {
		hash ^= character;
		hash *= 1099511628211ull;
	}
	return hash;
}

constexpr std::size_t CallinHashCapacity()
{
	std::size_t capacity = 1;
	while (capacity < CALLIN_COUNT * 2u)
		capacity <<= 1u;
	return capacity;
}

constexpr std::size_t CALLIN_HASH_CAPACITY = CallinHashCapacity();
static_assert((CALLIN_HASH_CAPACITY & (CALLIN_HASH_CAPACITY - 1u)) == 0u);
static_assert(CALLIN_COUNT < (1u << 16));

struct CallinHashSlot {
	std::uint64_t hash = 0;
	std::uint16_t ordinal = 0;
};

constexpr auto BuildCallinHashIndex()
{
	std::array<CallinHashSlot, CALLIN_HASH_CAPACITY> index{};
	for (std::size_t callinIndex = 0; callinIndex < CALLIN_COUNT; ++callinIndex) {
		const std::uint64_t hash = HashCallin(recoil::wasm::generated::kCallins[callinIndex].name);
		std::size_t slot = static_cast<std::size_t>(hash) & (CALLIN_HASH_CAPACITY - 1u);
		while (index[slot].ordinal != 0)
			slot = (slot + 1u) & (CALLIN_HASH_CAPACITY - 1u);
		index[slot] = {
			.hash = hash,
			.ordinal = static_cast<std::uint16_t>(callinIndex + 1u),
		};
	}
	return index;
}

inline constexpr auto CALLIN_HASH_INDEX = BuildCallinHashIndex();

std::uint16_t ResolveCallinOrdinal(std::string_view name)
{
	const std::uint64_t hash = HashCallin(name);
	std::size_t slot = static_cast<std::size_t>(hash) & (CALLIN_HASH_CAPACITY - 1u);
	for (std::size_t probe = 0; probe < CALLIN_HASH_CAPACITY; ++probe) {
		const CallinHashSlot& candidate = CALLIN_HASH_INDEX[slot];
		if (candidate.ordinal == 0)
			return 0;
		if (candidate.hash == hash &&
			name == recoil::wasm::generated::kCallins[candidate.ordinal - 1u].name)
			return candidate.ordinal;
		slot = (slot + 1u) & (CALLIN_HASH_CAPACITY - 1u);
	}
	return 0;
}

} // namespace

struct WasmCoreHost::Backend {
	Backend(NativeInterface* nativeInterface, const WasmRuntime& runtime,
		WasmEnvironment environment)
		: runtime(&runtime)
		, hot{false, WasmExecutionBudget(runtime.Config().instructionFuel,
			runtime.Config().hostWorkLimit, runtime.Config().resultBytesLimit)}
#if defined(RECOIL_WASMTIME_AVAILABLE)
		, bindings(nativeInterface, &hot.budget,
			WasmEnvironmentMatrix::Policy(environment).synced, environment,
			runtime.Config().maxValueNodes)
#endif
	{
		(void)nativeInterface;
		(void)environment;
	}

	~Backend()
	{
#if defined(RECOIL_WASMTIME_AVAILABLE)
		if (linker != nullptr)
			wasmtime_linker_delete(linker);
		if (module != nullptr)
			wasmtime_module_delete(module);
		if (store != nullptr)
			wasmtime_store_delete(store);
#endif
	}

	const WasmRuntime* runtime = nullptr;
	recoil::wasm::core::HotGuestState hot;
	// Resolved once at load: one entry per callin the guest exports. Handed to
	// the dispatcher as stable pointers, so this vector is never grown again.
	std::vector<recoil::wasm::core::WasmCoreDispatchPlan> plans;
	std::array<std::uint64_t, (CALLIN_COUNT + 63u) / 64u> implementedCallins{};
#if defined(RECOIL_WASMTIME_AVAILABLE)
	recoil::wasm::core::InstanceBindings bindings;
	recoil::wasm::core::VariableCallinBindings variableCallins;
#if defined(RECOIL_WASM_CORE_GENERATED_CALLIN_BINDINGS)
	recoil::wasm::core::generated::GeneratedCallinBindings generatedCallins;
	std::string generatedStringResultStorage;
#endif
#if defined(RECOIL_WASM_CORE_GENERATED_SCRATCH_CALLIN_BINDINGS)
	recoil::wasm::core::generated::GeneratedScratchCallinBindings generatedScratchCallins;
#endif
	wasmtime_store_t* store = nullptr;
	wasmtime_linker_t* linker = nullptr;
	wasmtime_module_t* module = nullptr;
	wasmtime_instance_t instance{};
#endif
	std::string faultReason;
};

WasmCoreHost::WasmCoreHost(std::string moduleName, WasmEnvironment environment,
	std::unique_ptr<Backend> backend)
	: moduleName(std::move(moduleName))
	, environment(environment)
	, backend(std::move(backend))
{
}

WasmCoreHost::~WasmCoreHost() = default;

WasmCoreHost* WasmCoreHost::Find(std::string_view moduleName)
{
	const auto& hosts = Hosts();
	const auto iter = std::find_if(hosts.begin(), hosts.end(), [moduleName](const auto& host) {
		return host != nullptr && host->moduleName == moduleName;
	});
	return iter == hosts.end() ? nullptr : iter->get();
}

bool WasmCoreHost::Enabled()
{
	// Core is the production transport. This is a routing decision, not a
	// runtime-availability probe: unsupported builds must enter Load() and fail
	// explicitly rather than silently falling back to the legacy scalar Core ABI.
	return true;
}

WasmCoreCallin WasmCoreHost::ResolveCallin(std::string_view name)
{
	return static_cast<WasmCoreCallin>(ResolveCallinOrdinal(name));
}

bool WasmCoreHost::Load(std::string moduleName, const std::vector<std::uint8_t>& moduleBytes,
	NativeInterface* nativeInterface, WasmEnvironment environment,
	const WasmRuntime& runtime, WasmModuleIdentity& identity, std::string& error)
{
	if (!Enabled())
		return false;
	if (moduleName.empty()) {
		error = "Core Wasm module name is empty";
		return false;
	}
	if (nativeInterface == nullptr) {
		error = "Core Wasm host has no NativeInterface";
		return false;
	}

	const WasmValidationResult validation = recoil::wasm::core::ValidateModule(
		moduleBytes, environment, RECOIL_WASM_INTERFACE_VERSION_NUMBER, runtime.Config());
	if (!validation.valid) {
		error = "Core Wasm validation failed: " + validation.error;
		return false;
	}
	identity = validation.identity;
	if (!runtime.IsAvailable()) {
		error = "Wasmtime is unavailable for the Core Wasm host";
		return false;
	}

	auto backend = std::make_unique<Backend>(nativeInterface, runtime, environment);

#if defined(RECOIL_WASMTIME_AVAILABLE)
	auto* engine = static_cast<wasm_engine_t*>(runtime.BackendEngine());
	backend->store = wasmtime_store_new(engine, nullptr, nullptr);
	if (backend->store == nullptr) {
		error = "Core Wasm host could not create a store";
		return false;
	}
	wasmtime_store_limiter(backend->store,
		static_cast<std::int64_t>(runtime.Config().maxMemoryPages) * 65536,
		static_cast<std::int64_t>(runtime.Config().maxTableElements), 2, 2, 2);
	if (runtime.Config().instructionFuel != 0) {
		if (wasmtime_error_t* fuelError = wasmtime_context_set_fuel(
				wasmtime_store_context(backend->store), runtime.Config().instructionFuel);
			fuelError != nullptr) {
			error = "Core Wasm host could not configure fuel: " +
				recoil::wasm::core::ErrorMessage(fuelError);
			return false;
		}
	}

	if (wasmtime_error_t* compileError = wasmtime_module_new(engine, moduleBytes.data(),
			moduleBytes.size(), &backend->module);
		compileError != nullptr) {
		error = "Core Wasm compilation failed: " +
			recoil::wasm::core::ErrorMessage(compileError);
		return false;
	}
	backend->linker = wasmtime_linker_new(engine);
	if (backend->linker == nullptr) {
		error = "Core Wasm host could not create a linker";
		return false;
	}
	if (!backend->bindings.RegisterImports(backend->linker, error)) {
		error = "Core Wasm import registration failed: " + error;
		return false;
	}
	if (wasmtime_error_t* linkError = wasmtime_linker_define_unknown_imports_as_traps(
			backend->linker, backend->module);
		linkError != nullptr) {
		error = "Core Wasm import policy failed: " +
			recoil::wasm::core::ErrorMessage(linkError);
		return false;
	}

	wasm_trap_t* trap = nullptr;
	if (wasmtime_error_t* instantiateError = wasmtime_linker_instantiate(backend->linker,
			wasmtime_store_context(backend->store), backend->module, &backend->instance, &trap);
		instantiateError != nullptr) {
		error = "Core Wasm instantiation failed: " +
			recoil::wasm::core::ErrorMessage(instantiateError);
		if (trap != nullptr)
			error += ": " + recoil::wasm::core::TrapMessage(trap);
		return false;
	}
	if (trap != nullptr) {
		error = "Core Wasm start trapped: " + recoil::wasm::core::TrapMessage(trap);
		return false;
	}
	auto* context = wasmtime_store_context(backend->store);
	recoil::wasm::core::RawExport environmentMarker;
	const wasm_valkind_t markerResults[] = {WASM_I32};
	if (!environmentMarker.Resolve(context, backend->instance, "SPRING_ENV_MASK", 15, {},
			std::span<const wasm_valkind_t>(markerResults, 1), false, error)) {
		error = "Core Wasm environment marker binding failed: " + error;
		return false;
	}
	wasmtime_val_raw_t markerSlot{};
	if (!environmentMarker.Call(context, &markerSlot, 1, error)) {
		error = "Core Wasm environment marker call failed: " + error;
		return false;
	}
	const std::int32_t expectedEnvironmentMask =
		static_cast<std::int32_t>(1u << static_cast<std::uint32_t>(environment));
	if (markerSlot.i32 != expectedEnvironmentMask) {
		error = "Core Wasm environment marker mismatch: manifest=" +
			std::string(WasmEnvironmentMatrix::Name(environment)) +
			" guest-mask=" + std::to_string(static_cast<std::uint32_t>(markerSlot.i32));
		return false;
	}
	if (!backend->bindings.Bind(context, backend->instance, error)) {
		error = "Core Wasm binding failed: " + error;
		return false;
	}
	if (!backend->variableCallins.Bind(context, backend->instance,
			backend->bindings.Host().memory, error)) {
		error = "Core Wasm variable callin binding failed: " + error;
		return false;
	}
#if defined(RECOIL_WASM_CORE_GENERATED_CALLIN_BINDINGS)
	if (!backend->generatedCallins.Bind(context, backend->instance, error)) {
		error = "generated Core Wasm callin binding failed: " + error;
		return false;
	}
#endif
#if defined(RECOIL_WASM_CORE_GENERATED_SCRATCH_CALLIN_BINDINGS)
	if (!backend->generatedScratchCallins.Bind(context, backend->instance,
			backend->bindings.Host().memory, error)) {
		error = "generated scratch Core Wasm callin binding failed: " + error;
		return false;
	}
#endif
	// Resolve export presence once. Callin dispatch is a hot path, while the
	// module's export set is immutable for its lifetime.
	auto markCallin = [&backend](std::uint16_t ordinal, bool present) {
		if (present && ordinal != 0 && ordinal <= CALLIN_COUNT)
			backend->implementedCallins[ordinal / 64u] |=
				std::uint64_t{1} << (ordinal % 64u);
	};
	markCallin(static_cast<std::uint16_t>(WasmCoreCallin::GameFrame),
		backend->bindings.HasGameFrame());
	markCallin(static_cast<std::uint16_t>(WasmCoreCallin::GameFramePost),
		backend->bindings.HasGameFramePost());
	markCallin(static_cast<std::uint16_t>(WasmCoreCallin::Update), backend->bindings.HasUpdate());
	markCallin(static_cast<std::uint16_t>(WasmCoreCallin::UnitCreated),
		backend->bindings.HasUnitCreated());
	markCallin(static_cast<std::uint16_t>(WasmCoreCallin::UnitPreDamaged),
		backend->bindings.HasUnitPreDamaged());
	markCallin(static_cast<std::uint16_t>(WasmCoreCallin::AllowUnitCreation),
		backend->bindings.HasAllowUnitCreation());
	markCallin(static_cast<std::uint16_t>(WasmCoreCallin::AddConsoleLine),
		backend->variableCallins.HasAddConsoleLine());
	markCallin(static_cast<std::uint16_t>(WasmCoreCallin::CommandNotify),
		backend->variableCallins.HasCommandNotify());
	markCallin(static_cast<std::uint16_t>(WasmCoreCallin::DrawWorld),
		backend->bindings.HasDrawWorld());
#if defined(RECOIL_WASM_CORE_GENERATED_CALLIN_BINDINGS)
	for (std::uint16_t ordinal = 1; ordinal <= CALLIN_COUNT; ++ordinal)
		markCallin(ordinal, backend->generatedCallins.Has(ordinal));
#endif
#if defined(RECOIL_WASM_CORE_GENERATED_SCRATCH_CALLIN_BINDINGS)
	for (std::uint16_t ordinal = 1; ordinal <= CALLIN_COUNT; ++ordinal)
		markCallin(ordinal, backend->generatedScratchCallins.Has(ordinal));
#endif
#else
	error = "Wasmtime is unavailable for the Core Wasm host";
	return false;
#endif

	Unload(moduleName);
	Hosts().emplace_back(new WasmCoreHost(std::move(moduleName), environment,
		std::move(backend)));
	// Resolve the dispatch plans once, now, so no callin ever pays for it.
	Hosts().back()->BuildDispatchPlans();
	return true;
}

void WasmCoreHost::Unload(std::string_view moduleName)
{
	auto& hosts = Hosts();
	hosts.erase(std::remove_if(hosts.begin(), hosts.end(), [moduleName](const auto& host) {
		return host != nullptr && host->moduleName == moduleName;
	}), hosts.end());
	// Unloading may have taken a faulted guest with it. Leaving the pending
	// count high would put the fault sweep back on every dispatch.
	RecountPendingUnsyncedFaults();
}

void WasmCoreHost::UnloadAll()
{
	Hosts().clear();
	PendingUnsyncedFaultCount() = 0;
}

std::string_view WasmCoreHost::ModuleName(const WasmCoreHost* host)
{
	return host != nullptr ? std::string_view(host->moduleName) : std::string_view{};
}

bool WasmCoreHost::AnyActive()
{
	return !Hosts().empty();
}

bool WasmCoreHost::AnyActive(WasmEnvironment environment)
{
	return std::any_of(Hosts().begin(), Hosts().end(), [environment](const auto& host) {
		return host != nullptr && host->environment == environment;
	});
}

bool WasmCoreHost::HasModule(std::string_view moduleName)
{
	return Find(moduleName) != nullptr;
}

bool WasmCoreHost::ModuleFaulted(std::string_view moduleName)
{
	const WasmCoreHost* host = Find(moduleName);
	return host != nullptr && host->backend != nullptr && host->backend->hot.faulted;
}

void WasmCoreHost::Fault(std::string reason)
{
	if (backend == nullptr || backend->hot.faulted)
		return;
	backend->hot.faulted = true;
	backend->faultReason = reason.empty() ? "Core Wasm module faulted" : std::move(reason);
	if (!WasmEnvironmentMatrix::Policy(environment).synced)
		++PendingUnsyncedFaultCount();
}

bool WasmCoreHost::FaultModule(std::string_view moduleName, std::string reason)
{
	WasmCoreHost* host = Find(moduleName);
	if (host == nullptr)
		return false;
	host->Fault(std::move(reason));
	return true;
}

void WasmCoreHost::RecountPendingUnsyncedFaults()
{
	std::size_t pending = 0;
	for (const auto& host : Hosts()) {
		if (host != nullptr && host->backend != nullptr && host->backend->hot.faulted &&
			!WasmEnvironmentMatrix::Policy(host->environment).synced)
			++pending;
	}
	PendingUnsyncedFaultCount() = pending;
}

std::size_t WasmCoreHost::PendingUnsyncedFaults()
{
	return PendingUnsyncedFaultCount();
}

std::size_t WasmCoreHost::RemoveFaultedUnsynced()
{
	// Callers poll this on the dispatch path, so a run with no faults must not
	// touch the registry at all.
	if (PendingUnsyncedFaultCount() == 0)
		return 0;
	PendingUnsyncedFaultCount() = 0;
	auto& hosts = Hosts();
	const std::size_t before = hosts.size();
	hosts.erase(std::remove_if(hosts.begin(), hosts.end(), [](const auto& host) {
		return host != nullptr && host->backend != nullptr && host->backend->hot.faulted &&
			!WasmEnvironmentMatrix::Policy(host->environment).synced;
	}), hosts.end());
	return before - hosts.size();
}

bool WasmCoreHost::ResetBudgetImpl(std::string& error)
{
	if (backend == nullptr || backend->runtime == nullptr) {
		error = "Core Wasm module has no execution backend";
		return false;
	}
	const WasmRuntimeConfig& config = backend->runtime->Config();
	backend->hot.budget.Reset(config.instructionFuel, config.hostWorkLimit,
		config.resultBytesLimit);
#if defined(RECOIL_WASMTIME_AVAILABLE)
	if (config.instructionFuel != 0) {
		if (wasmtime_error_t* fuelError = wasmtime_context_set_fuel(
				wasmtime_store_context(backend->store), config.instructionFuel);
			fuelError != nullptr) {
			error = "Core Wasm host could not reset fuel: " +
				recoil::wasm::core::ErrorMessage(fuelError);
			return false;
		}
	}
#else
	(void)config;
#endif
	return true;
}

bool WasmCoreHost::FuelRemainingImpl(std::uint64_t& fuel, std::string& error) const
{
	fuel = 0;
	if (backend == nullptr || backend->runtime == nullptr) {
		error = "Core Wasm module has no execution backend";
		return false;
	}
	if (backend->runtime->Config().instructionFuel == 0)
		return true;
#if defined(RECOIL_WASMTIME_AVAILABLE)
	if (wasmtime_error_t* fuelError = wasmtime_context_get_fuel(
				wasmtime_store_context(backend->store), &fuel);
		fuelError != nullptr) {
		error = "Core Wasm host could not query fuel: " +
				recoil::wasm::core::ErrorMessage(fuelError);
		return false;
	}
	return true;
#else
	error = "Wasmtime is unavailable for the Core Wasm host";
	return false;
#endif
}

bool WasmCoreHost::ResetBudget(std::string_view moduleName, std::string& error)
{
	WasmCoreHost* host = Find(moduleName);
	if (host == nullptr) {
		error = "Core Wasm module not found: " + std::string(moduleName);
		return false;
	}
	return host->ResetBudgetImpl(error);
}

bool WasmCoreHost::FuelRemaining(std::string_view moduleName, std::uint64_t& fuel,
	std::string& error)
{
	const WasmCoreHost* host = Find(moduleName);
	if (host == nullptr) {
		error = "Core Wasm module not found: " + std::string(moduleName);
		return false;
	}
	return host->FuelRemainingImpl(fuel, error);
}

bool WasmCoreHost::HasCallin(WasmCoreCallin callin) const
{
	if (backend == nullptr || callin == WasmCoreCallin::Invalid)
		return false;
	const std::uint16_t ordinal = static_cast<std::uint16_t>(callin);
	return ordinal <= CALLIN_COUNT &&
		(backend->implementedCallins[ordinal / 64u] &
			(std::uint64_t{1} << (ordinal % 64u))) != 0;
}

#if defined(RECOIL_WASMTIME_AVAILABLE)
namespace {

using recoil::wasm::core::HotGuestState;
using recoil::wasm::core::WasmCoreDispatchPlan;

// One unchecked host->guest call through the function cached in the plan. This
// is the only place a Core callin enters Wasmtime.
bool CallPlan(const WasmCoreDispatchPlan& plan, wasmtime_val_raw_t* slots,
	std::size_t slotCount, std::string& error)
{
	const auto entryStage = spring::benchmark_callins::BeginStage(
		spring::benchmark_callins::Stage::WasmtimeEntry);
	bool success = true;
	wasm_trap_t* trap = nullptr;
	if (wasmtime_error_t* callError = wasmtime_func_call_unchecked(
			plan.context, &plan.function, slots, slotCount, &trap);
		callError != nullptr) {
		error = "core Wasm export call failed: " +
			recoil::wasm::core::ErrorMessage(callError);
		if (trap != nullptr)
			error += ": " + recoil::wasm::core::TrapMessage(trap);
		success = false;
	} else if (trap != nullptr) {
		error = "core Wasm export trapped: " + recoil::wasm::core::TrapMessage(trap);
		success = false;
	}
	spring::benchmark_callins::End(entryStage);
	return success;
}

template<typename T>
const T* TypedQuery(const void* query, std::string_view name, std::string& error)
{
	if (query != nullptr)
		return static_cast<const T*>(query);
	error = "Core Wasm " + std::string(name) + " query is null";
	return nullptr;
}

bool PlanVoid(const WasmCoreDispatchPlan& plan, const void*, void*, std::string& error)
{
	return CallPlan(plan, nullptr, 0, error);
}

bool PlanGameFrame(const WasmCoreDispatchPlan& plan, const void* query, void*,
	std::string& error)
{
	const auto* typed = TypedQuery<GameFrameQuery>(query, "GameFrame", error);
	if (typed == nullptr)
		return false;
	wasmtime_val_raw_t slot{};
	slot.i32 = typed->gameFrame;
	return CallPlan(plan, &slot, 1, error);
}

bool PlanGameFramePost(const WasmCoreDispatchPlan& plan, const void* query, void*,
	std::string& error)
{
	const auto* typed = TypedQuery<GameFramePostQuery>(query, "GameFramePost", error);
	if (typed == nullptr)
		return false;
	wasmtime_val_raw_t slot{};
	slot.i32 = typed->gameFrame;
	return CallPlan(plan, &slot, 1, error);
}

bool PlanUpdate(const WasmCoreDispatchPlan& plan, const void* query, void*,
	std::string& error)
{
	const auto* typed = TypedQuery<UpdateQuery>(query, "Update", error);
	if (typed == nullptr)
		return false;
	wasmtime_val_raw_t slot{};
	slot.f32 = typed->deltaSeconds;
	return CallPlan(plan, &slot, 1, error);
}

bool PlanUnitCreated(const WasmCoreDispatchPlan& plan, const void* query, void*,
	std::string& error)
{
	const auto* typed = TypedQuery<UnitCreatedQuery>(query, "UnitCreated", error);
	if (typed == nullptr)
		return false;
	std::array<wasmtime_val_raw_t, 4> slots{};
	slots[0].i32 = typed->unitID;
	slots[1].i32 = typed->unitDefID;
	slots[2].i32 = typed->unitTeam;
	slots[3].i32 = typed->builderID;
	return CallPlan(plan, slots.data(), slots.size(), error);
}

bool PlanUnitPreDamaged(const WasmCoreDispatchPlan& plan, const void* query, void* result,
	std::string& error)
{
	const auto* typed = TypedQuery<UnitDamagedQuery>(query, "UnitPreDamaged", error);
	if (typed == nullptr)
		return false;
	auto* typedResult = static_cast<DamageCallinResult*>(result);
	std::array<wasmtime_val_raw_t, 10> slots{};
	slots[0].i32 = typed->unitID;
	slots[1].i32 = typed->unitDefID;
	slots[2].i32 = typed->unitTeam;
	slots[3].f32 = typed->damage;
	slots[4].i32 = typed->paralyzer ? 1 : 0;
	slots[5].i32 = typed->weaponDefID;
	slots[6].i32 = typed->projectileID;
	slots[7].i32 = typed->attackerID;
	slots[8].i32 = typed->attackerDefID;
	slots[9].i32 = typed->attackerTeam;
	if (!CallPlan(plan, slots.data(), slots.size(), error))
		return false;
	if (typedResult == nullptr)
		return true;
	recoil::wasm::core::UnpackF32Pair(static_cast<std::uint64_t>(slots[0].i64),
		typedResult->newDamage, typedResult->impulseMult);
	return true;
}

bool PlanAllowUnitCreation(const WasmCoreDispatchPlan& plan, const void* query,
	void* result, std::string& error)
{
	const auto* typed = TypedQuery<AllowUnitCreationQuery>(query, "AllowUnitCreation", error);
	if (typed == nullptr)
		return false;
	std::array<wasmtime_val_raw_t, 8> slots{};
	slots[0].i32 = typed->unitDefID;
	slots[1].i32 = typed->builderID;
	slots[2].i32 = typed->builderTeam;
	slots[3].i32 = typed->hasBuildInfo ? 1 : 0;
	slots[4].f32 = typed->buildPos.x;
	slots[5].f32 = typed->buildPos.y;
	slots[6].f32 = typed->buildPos.z;
	slots[7].i32 = typed->buildFacing;
	if (!CallPlan(plan, slots.data(), slots.size(), error))
		return false;
	const std::uint32_t flags = static_cast<std::uint32_t>(slots[0].i32);
	if ((flags & ~0x3u) != 0) {
		error = "core Wasm allow-unit-creation returned invalid result flags";
		return false;
	}
	if (auto* typedResult = static_cast<AllowUnitCreationResult*>(result);
		typedResult != nullptr) {
		typedResult->allow = (flags & 0x1u) != 0;
		typedResult->dropOrder = (flags & 0x2u) != 0;
	}
	return true;
}

// Variable-payload and generated callins keep their existing marshalling, which
// needs the backend's scratch region and binding tables.
bool PlanAddConsoleLine(const WasmCoreDispatchPlan& plan, const void* query, void* result,
	std::string& error)
{
	return plan.host->InvokeAddConsoleLine(query, result, error);
}

bool PlanCommandNotify(const WasmCoreDispatchPlan& plan, const void* query, void* result,
	std::string& error)
{
	return plan.host->InvokeCommandNotify(query, result, error);
}

bool PlanGenerated(const WasmCoreDispatchPlan& plan, const void* query, void* result,
	std::string& error)
{
	return plan.host->InvokeGenerated(plan.callin, query, result, error);
}

} // namespace
#endif

bool WasmCoreHost::InvokeAddConsoleLine(const void* query, void* result,
	std::string& error)
{
#if defined(RECOIL_WASMTIME_AVAILABLE)
	const auto* typed = static_cast<const AddConsoleLineQuery*>(query);
	if (typed == nullptr) {
		error = "Core Wasm AddConsoleLine query is null";
		return false;
	}
	BoolCallinResult fallback{};
	auto* typedResult = static_cast<BoolCallinResult*>(result);
	if (typedResult == nullptr)
		typedResult = &fallback;
	return backend->variableCallins.AddConsoleLine(
		wasmtime_store_context(backend->store), backend->hot.budget,
		backend->bindings.Host().memory, *typed, *typedResult, error);
#else
	(void)query;
	(void)result;
	error = "Wasmtime is unavailable for the Core Wasm host";
	return false;
#endif
}

bool WasmCoreHost::InvokeCommandNotify(const void* query, void* result,
	std::string& error)
{
#if defined(RECOIL_WASMTIME_AVAILABLE)
	const auto* typed = static_cast<const CommandNotifyQuery*>(query);
	if (typed == nullptr) {
		error = "Core Wasm CommandNotify query is null";
		return false;
	}
	BoolCallinResult fallback{};
	auto* typedResult = static_cast<BoolCallinResult*>(result);
	if (typedResult == nullptr)
		typedResult = &fallback;
	return backend->variableCallins.CommandNotify(
		wasmtime_store_context(backend->store), backend->hot.budget,
		backend->bindings.Host().memory, *typed, *typedResult, error);
#else
	(void)query;
	(void)result;
	error = "Wasmtime is unavailable for the Core Wasm host";
	return false;
#endif
}

bool WasmCoreHost::InvokeGenerated(WasmCoreCallin callin, const void* query,
	void* result, std::string& error)
{
#if defined(RECOIL_WASMTIME_AVAILABLE)
	const std::uint16_t ordinal = static_cast<std::uint16_t>(callin);
#if defined(RECOIL_WASM_CORE_GENERATED_CALLIN_BINDINGS)
	if (backend->generatedCallins.Has(ordinal)) {
		return backend->generatedCallins.Invoke(ordinal,
			wasmtime_store_context(backend->store), backend->hot.budget,
			backend->bindings.Host().memory, backend->generatedStringResultStorage,
			query, result, error);
	}
#endif
#if defined(RECOIL_WASM_CORE_GENERATED_SCRATCH_CALLIN_BINDINGS)
	if (backend->generatedScratchCallins.Has(ordinal)) {
		return backend->generatedScratchCallins.Invoke(ordinal,
			wasmtime_store_context(backend->store), backend->hot.budget,
			backend->bindings.Host().memory, query, result, error);
	}
#endif
	error = "Core Wasm callin has no generated binding";
	return false;
#else
	(void)callin;
	(void)query;
	(void)result;
	error = "Wasmtime is unavailable for the Core Wasm host";
	return false;
#endif
}

// Build one plan per exported callin. Everything resolved here — the guest
// function, the store context, the invoker, the mutable hot state — is fixed
// for the module's lifetime, so no dispatch has to rediscover any of it.
void WasmCoreHost::BuildDispatchPlans()
{
#if defined(RECOIL_WASMTIME_AVAILABLE)
	using recoil::wasm::core::WasmCoreDispatchPlan;
	if (backend == nullptr)
		return;

	wasmtime_context_t* context = wasmtime_store_context(backend->store);
	const bool ui = environment == WasmEnvironment::UI;

	std::vector<WasmCoreDispatchPlan>& plans = backend->plans;
	plans.clear();
	std::size_t exported = 0;
	for (std::uint16_t ordinal = 1; ordinal <= CALLIN_COUNT; ++ordinal) {
		if (HasCallin(static_cast<WasmCoreCallin>(ordinal)))
			++exported;
	}
	plans.reserve(exported);

	const auto add = [&](WasmCoreCallin callin, WasmCoreDispatchPlan::Invoke invoke,
		const recoil::wasm::core::RawExport* function) {
		WasmCoreDispatchPlan plan;
		plan.invoke = invoke;
		plan.context = context;
		plan.hot = &backend->hot;
		plan.host = this;
		plan.callin = callin;
		plan.uiEnvironment = ui;
		if (function != nullptr)
			plan.function = function->Function();
		plans.push_back(plan);
	};

	for (std::uint16_t ordinal = 1; ordinal <= CALLIN_COUNT; ++ordinal) {
		const auto callin = static_cast<WasmCoreCallin>(ordinal);
		if (!HasCallin(callin))
			continue;
		const recoil::wasm::core::InstanceBindings& bindings = backend->bindings;
		switch (callin) {
			case WasmCoreCallin::GameFrame:
				add(callin, &PlanGameFrame, &bindings.GameFrameExport());
				break;
			case WasmCoreCallin::GameFramePost:
				add(callin, &PlanGameFramePost, &bindings.GameFramePostExport());
				break;
			case WasmCoreCallin::Update:
				add(callin, &PlanUpdate, &bindings.UpdateExport());
				break;
			case WasmCoreCallin::UnitCreated:
				add(callin, &PlanUnitCreated, &bindings.UnitCreatedExport());
				break;
			case WasmCoreCallin::UnitPreDamaged:
				add(callin, &PlanUnitPreDamaged, &bindings.UnitPreDamagedExport());
				break;
			case WasmCoreCallin::AllowUnitCreation:
				add(callin, &PlanAllowUnitCreation, &bindings.AllowUnitCreationExport());
				break;
			case WasmCoreCallin::DrawWorld:
				add(callin, &PlanVoid, &bindings.DrawWorldExport());
				break;
			case WasmCoreCallin::AddConsoleLine:
				add(callin, &PlanAddConsoleLine, nullptr);
				break;
			case WasmCoreCallin::CommandNotify:
				add(callin, &PlanCommandNotify, nullptr);
				break;
			default:
				add(callin, &PlanGenerated, nullptr);
				break;
		}
	}
#endif
}

const recoil::wasm::core::WasmCoreDispatchPlan* WasmCoreHost::PlanFor(
	WasmCoreCallin callin) const
{
	if (backend == nullptr)
		return nullptr;
	for (const auto& plan : backend->plans) {
		if (plan.callin == callin)
			return &plan;
	}
	return nullptr;
}

const recoil::wasm::core::WasmCoreDispatchPlan* WasmCoreHost::ModulePlan(
	const WasmCoreHost* host, WasmCoreCallin callin)
{
	return host != nullptr ? host->PlanFor(callin) : nullptr;
}

namespace recoil::wasm::core {

bool DispatchPlanRejected(const WasmCoreDispatchPlan* plan, std::string& error)
{
	if (plan == nullptr) {
		error = "Core Wasm dispatch plan is null";
		return false;
	}
	error = WasmCoreHost::FaultReason(plan->host);
	return false;
}

bool DispatchPlanExhausted(const WasmCoreDispatchPlan* plan, std::string& error)
{
	error = "Core Wasm callin host-work budget exhausted";
	WasmCoreHost::FaultHost(plan->host, error);
	return false;
}

bool DispatchPlanFailed(const WasmCoreDispatchPlan* plan, std::string& error)
{
	WasmCoreHost::FaultHost(plan->host,
		error.empty() ? "Core Wasm callin failed" : error);
	return false;
}

} // namespace recoil::wasm::core

std::string WasmCoreHost::FaultReason(const WasmCoreHost* host)
{
	return host != nullptr && host->backend != nullptr ? host->backend->faultReason
		: std::string("Core Wasm module faulted");
}

void WasmCoreHost::FaultHost(WasmCoreHost* host, std::string reason)
{
	if (host != nullptr)
		host->Fault(std::move(reason));
}

bool WasmCoreHost::Dispatch(const recoil::wasm::core::WasmCoreDispatchPlan* plan,
	const void* query, void* result, std::string& error)
{
#if defined(RECOIL_WASMTIME_AVAILABLE)
	return recoil::wasm::core::DispatchPlan(plan, query, result, error);
#else
	(void)plan;
	(void)query;
	(void)result;
	error = "Wasmtime is unavailable for the Core Wasm host";
	return false;
#endif
}

bool WasmCoreHost::DispatchModule(WasmCoreHost* host, WasmCoreCallin callin,
	const void* query, void* result, std::string& error)
{
	if (host == nullptr) {
		error = "Core Wasm module handle is null";
		return false;
	}
	const recoil::wasm::core::WasmCoreDispatchPlan* plan = host->PlanFor(callin);
	if (plan == nullptr)
		return true;
	return Dispatch(plan, query, result, error);
}

bool WasmCoreHost::Invoke(WasmCoreCallin callin, const void* query, void* result,
	std::string& error)
{
	return DispatchModule(this, callin, query, result, error);
}

bool WasmCoreHost::DispatchModule(std::string_view moduleName, WasmCoreCallin callin,
	const void* query, void* result, std::string& error)
{
	WasmCoreHost* host = Find(moduleName);
	if (host == nullptr) {
		error = "Core Wasm module not found: " + std::string(moduleName);
		return false;
	}
	return host->Invoke(callin, query, result, error);
}

bool WasmCoreHost::DispatchModule(std::string_view moduleName, std::string_view name,
	const void* query, void* result, std::string& error)
{
	const WasmCoreCallin callin = ResolveCallin(name);
	if (callin == WasmCoreCallin::Invalid) {
		error = "unsupported Core Wasm callin: " + std::string(name);
		return false;
	}
	return DispatchModule(moduleName, callin, query, result, error);
}

bool WasmCoreHost::DispatchEnvironment(WasmEnvironment environment, std::string_view name,
	const void* query, void* result, std::string& error)
{
	const WasmCoreCallin callin = ResolveCallin(name);
	if (callin == WasmCoreCallin::Invalid) {
		error = "unsupported Core Wasm callin: " + std::string(name);
		return false;
	}
	bool handled = false;
	for (const auto& host : Hosts()) {
		if (host == nullptr || host->environment != environment || !host->HasCallin(callin))
			continue;
		handled = true;
		std::string hostError;
		if (!host->Invoke(callin, query, result, hostError) && error.empty())
			error = hostError;
	}
	return handled;
}

bool WasmCoreHost::DispatchCallin(std::string_view name, const void* query, void* result,
	std::string& error)
{
	const WasmCoreCallin callin = ResolveCallin(name);
	if (callin == WasmCoreCallin::Invalid)
		return false;
	bool handled = false;
	for (const auto& host : Hosts()) {
		if (host == nullptr || !host->HasCallin(callin))
			continue;
		handled = true;
		std::string hostError;
		if (!host->Invoke(callin, query, result, hostError) && error.empty())
			error = hostError;
	}
	return handled;
}
