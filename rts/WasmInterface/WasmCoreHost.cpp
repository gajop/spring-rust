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
		, budget(runtime.Config().instructionFuel, runtime.Config().hostWorkLimit,
			runtime.Config().resultBytesLimit)
#if defined(RECOIL_WASMTIME_AVAILABLE)
		, bindings(nativeInterface, &budget,
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
	WasmExecutionBudget budget;
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
	bool faulted = false;
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
	return true;
}

void WasmCoreHost::Unload(std::string_view moduleName)
{
	auto& hosts = Hosts();
	hosts.erase(std::remove_if(hosts.begin(), hosts.end(), [moduleName](const auto& host) {
		return host != nullptr && host->moduleName == moduleName;
	}), hosts.end());
}

void WasmCoreHost::UnloadAll()
{
	Hosts().clear();
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
	return host != nullptr && host->backend != nullptr && host->backend->faulted;
}

void WasmCoreHost::Fault(std::string reason)
{
	if (backend == nullptr)
		return;
	backend->faulted = true;
	backend->faultReason = reason.empty() ? "Core Wasm module faulted" : std::move(reason);
}

bool WasmCoreHost::FaultModule(std::string_view moduleName, std::string reason)
{
	WasmCoreHost* host = Find(moduleName);
	if (host == nullptr)
		return false;
	host->Fault(std::move(reason));
	return true;
}

std::size_t WasmCoreHost::RemoveFaultedUnsynced()
{
	auto& hosts = Hosts();
	const std::size_t before = hosts.size();
	hosts.erase(std::remove_if(hosts.begin(), hosts.end(), [](const auto& host) {
		return host != nullptr && host->backend != nullptr && host->backend->faulted &&
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
	backend->budget.Reset(config.instructionFuel, config.hostWorkLimit,
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

bool WasmCoreHost::InvokeGameFrame(const void* query, std::string& error)
{
#if defined(RECOIL_WASMTIME_AVAILABLE)
	const auto* typed = static_cast<const GameFrameQuery*>(query);
	if (typed == nullptr) {
		error = "Core Wasm GameFrame query is null";
		return false;
	}
	return backend->bindings.GameFrame(wasmtime_store_context(backend->store),
		typed->gameFrame, error);
#else
	(void)query;
	error = "Wasmtime is unavailable for the Core Wasm host";
	return false;
#endif
}

bool WasmCoreHost::InvokeGameFramePost(const void* query, std::string& error)
{
#if defined(RECOIL_WASMTIME_AVAILABLE)
	const auto* typed = static_cast<const GameFramePostQuery*>(query);
	if (typed == nullptr) {
		error = "Core Wasm GameFramePost query is null";
		return false;
	}
	return backend->bindings.GameFramePost(wasmtime_store_context(backend->store),
		typed->gameFrame, error);
#else
	(void)query;
	error = "Wasmtime is unavailable for the Core Wasm host";
	return false;
#endif
}

bool WasmCoreHost::InvokeUpdate(const void* query, std::string& error)
{
#if defined(RECOIL_WASMTIME_AVAILABLE)
	const auto* typed = static_cast<const UpdateQuery*>(query);
	if (typed == nullptr) {
		error = "Core Wasm Update query is null";
		return false;
	}
	return backend->bindings.Update(wasmtime_store_context(backend->store),
		typed->deltaSeconds, error);
#else
	(void)query;
	error = "Wasmtime is unavailable for the Core Wasm host";
	return false;
#endif
}

bool WasmCoreHost::InvokeUnitCreated(const void* query, std::string& error)
{
#if defined(RECOIL_WASMTIME_AVAILABLE)
	const auto* typed = static_cast<const UnitCreatedQuery*>(query);
	if (typed == nullptr) {
		error = "Core Wasm UnitCreated query is null";
		return false;
	}
	return backend->bindings.UnitCreated(wasmtime_store_context(backend->store),
		typed->unitID, typed->unitDefID, typed->unitTeam, typed->builderID, error);
#else
	(void)query;
	error = "Wasmtime is unavailable for the Core Wasm host";
	return false;
#endif
}

bool WasmCoreHost::InvokeUnitPreDamaged(const void* query, void* result,
	std::string& error)
{
#if defined(RECOIL_WASMTIME_AVAILABLE)
	const auto* typed = static_cast<const UnitDamagedQuery*>(query);
	auto* typedResult = static_cast<DamageCallinResult*>(result);
	if (typed == nullptr) {
		error = "Core Wasm UnitPreDamaged query is null";
		return false;
	}
	float newDamage = typedResult == nullptr ? typed->damage : typedResult->newDamage;
	float impulseMult = typedResult == nullptr ? 1.0f : typedResult->impulseMult;
	if (!backend->bindings.UnitPreDamaged(wasmtime_store_context(backend->store),
		typed->unitID, typed->unitDefID, typed->unitTeam, typed->damage, typed->paralyzer,
		typed->weaponDefID, typed->projectileID, typed->attackerID,
		typed->attackerDefID, typed->attackerTeam, newDamage, impulseMult, error))
		return false;
	if (typedResult != nullptr) {
		typedResult->newDamage = newDamage;
		typedResult->impulseMult = impulseMult;
	}
	return true;
#else
	(void)query;
	(void)result;
	error = "Wasmtime is unavailable for the Core Wasm host";
	return false;
#endif
}

bool WasmCoreHost::InvokeAllowUnitCreation(const void* query, void* result,
	std::string& error)
{
#if defined(RECOIL_WASMTIME_AVAILABLE)
	const auto* typed = static_cast<const AllowUnitCreationQuery*>(query);
	auto* typedResult = static_cast<AllowUnitCreationResult*>(result);
	if (typed == nullptr) {
		error = "Core Wasm AllowUnitCreation query is null";
		return false;
	}
	bool allow = typedResult == nullptr ? true : typedResult->allow;
	bool dropOrder = typedResult == nullptr ? false : typedResult->dropOrder;
	if (!backend->bindings.AllowUnitCreation(wasmtime_store_context(backend->store),
		typed->unitDefID, typed->builderID, typed->builderTeam, typed->hasBuildInfo,
		typed->buildPos.x, typed->buildPos.y, typed->buildPos.z, typed->buildFacing,
		allow, dropOrder, error))
		return false;
	if (typedResult != nullptr) {
		typedResult->allow = allow;
		typedResult->dropOrder = dropOrder;
	}
	return true;
#else
	(void)query;
	(void)result;
	error = "Wasmtime is unavailable for the Core Wasm host";
	return false;
#endif
}

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
		wasmtime_store_context(backend->store), backend->budget,
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
		wasmtime_store_context(backend->store), backend->budget,
		backend->bindings.Host().memory, *typed, *typedResult, error);
#else
	(void)query;
	(void)result;
	error = "Wasmtime is unavailable for the Core Wasm host";
	return false;
#endif
}

bool WasmCoreHost::InvokeDrawWorld(std::string& error)
{
#if defined(RECOIL_WASMTIME_AVAILABLE)
	return backend->bindings.DrawWorld(wasmtime_store_context(backend->store), error);
#else
	error = "Wasmtime is unavailable for the Core Wasm host";
	return false;
#endif
}

bool WasmCoreHost::Invoke(WasmCoreCallin callin, const void* query, void* result,
	std::string& error)
{
	if (backend == nullptr) {
		error = "Core Wasm host has no backend";
		return false;
	}
	if (backend->faulted) {
		error = backend->faultReason;
		return false;
	}
	if (callin == WasmCoreCallin::Invalid) {
		error = "unsupported Core Wasm callin";
		return false;
	}
	if (!HasCallin(callin))
		return true;
	if (!backend->budget.ChargeHost(1)) {
		error = "Core Wasm callin host-work budget exhausted";
		Fault(error);
		return false;
	}

	bool success = false;
	switch (callin) {
		case WasmCoreCallin::GameFrame: success = InvokeGameFrame(query, error); break;
		case WasmCoreCallin::GameFramePost: success = InvokeGameFramePost(query, error); break;
		case WasmCoreCallin::Update: success = InvokeUpdate(query, error); break;
		case WasmCoreCallin::UnitCreated: success = InvokeUnitCreated(query, error); break;
		case WasmCoreCallin::UnitPreDamaged: success = InvokeUnitPreDamaged(query, result, error); break;
		case WasmCoreCallin::AllowUnitCreation: success = InvokeAllowUnitCreation(query, result, error); break;
		case WasmCoreCallin::AddConsoleLine: success = InvokeAddConsoleLine(query, result, error); break;
		case WasmCoreCallin::CommandNotify: success = InvokeCommandNotify(query, result, error); break;
		case WasmCoreCallin::DrawWorld: success = InvokeDrawWorld(error); break;
		case WasmCoreCallin::Invalid: break;
		default: {
			const std::uint16_t ordinal = static_cast<std::uint16_t>(callin);
#if defined(RECOIL_WASM_CORE_GENERATED_CALLIN_BINDINGS)
			if (backend->generatedCallins.Has(ordinal)) {
				success = backend->generatedCallins.Invoke(ordinal,
					wasmtime_store_context(backend->store), backend->budget,
					backend->bindings.Host().memory, backend->generatedStringResultStorage,
					query, result, error);
				break;
			}
#endif
#if defined(RECOIL_WASM_CORE_GENERATED_SCRATCH_CALLIN_BINDINGS)
			if (backend->generatedScratchCallins.Has(ordinal)) {
				success = backend->generatedScratchCallins.Invoke(ordinal,
					wasmtime_store_context(backend->store), backend->budget,
					backend->bindings.Host().memory, query, result, error);
				break;
			}
#endif
			break;
		}
	}

	if (!success)
		Fault(error.empty() ? "Core Wasm callin failed" : error);
	return success;
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
