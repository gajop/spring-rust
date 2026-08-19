/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreHost.h"

#include <algorithm>
#include <cstdlib>
#include <memory>
#include <utility>

#include "NativeInterface/NativeInterface.h"
#include "NativeInterface/api/Callins.h"
#include "WasmCoreAbi.h"
#include "WasmCoreBindings.h"
#include "WasmCoreValidation.h"
#include "WasmResources.h"

namespace {

bool TruthyEnvironment(const char* name)
{
	const char* value = std::getenv(name);
	if (value == nullptr)
		return false;
	const std::string_view setting(value);
	return setting == "1" || setting == "true" || setting == "TRUE" ||
		setting == "yes" || setting == "YES" || setting == "on" || setting == "ON";
}

std::vector<std::unique_ptr<WasmCoreHost>>& Hosts()
{
	static std::vector<std::unique_ptr<WasmCoreHost>> hosts;
	return hosts;
}

bool KnownCallin(std::string_view name)
{
	return name == "GameFrame" || name == "GameFramePost" || name == "Update" ||
		name == "UnitCreated" || name == "UnitPreDamaged" ||
		name == "AllowUnitCreation" || name == "DrawWorld";
}

} // namespace

struct WasmCoreHost::Backend {
	Backend(NativeInterface* nativeInterface, const WasmRuntime& runtime)
		: runtime(&runtime)
		, budget(runtime.Config().instructionFuel, runtime.Config().hostWorkLimit,
			runtime.Config().resultBytesLimit)
#if defined(RECOIL_WASMTIME_AVAILABLE)
		, bindings(nativeInterface, &budget)
#endif
	{
		(void)nativeInterface;
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
#if defined(RECOIL_WASMTIME_AVAILABLE)
	recoil::wasm::core::InstanceBindings bindings;
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

bool WasmCoreHost::Enabled()
{
	static const bool enabled = TruthyEnvironment("SPRING_WASM_CORE_HOST");
	return enabled;
}

bool WasmCoreHost::Load(std::string moduleName, const std::vector<std::uint8_t>& moduleBytes,
	NativeInterface* nativeInterface, WasmEnvironment environment,
	const WasmRuntime& runtime, WasmModuleIdentity& identity, std::string& error)
{
	if (!Enabled())
		return false;
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

	auto backend = std::make_unique<Backend>(nativeInterface, runtime);

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
	if (!backend->bindings.Bind(wasmtime_store_context(backend->store),
		backend->instance, error)) {
		error = "Core Wasm binding failed: " + error;
		return false;
	}
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
		return host->moduleName == moduleName;
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
	float newDamage = typed->damage;
	float impulseMult = 1.0f;
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

bool WasmCoreHost::InvokeDrawWorld(std::string& error)
{
#if defined(RECOIL_WASMTIME_AVAILABLE)
	return backend->bindings.DrawWorld(wasmtime_store_context(backend->store), error);
#else
	error = "Wasmtime is unavailable for the Core Wasm host";
	return false;
#endif
}

bool WasmCoreHost::Invoke(std::string_view name, const void* query, void* result,
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
	if (!backend->budget.ChargeHost(1)) {
		error = "Core Wasm callin host-work budget exhausted";
		backend->faulted = true;
		backend->faultReason = error;
		return false;
	}

	bool success = false;
	if (name == "GameFrame")
		success = InvokeGameFrame(query, error);
	else if (name == "GameFramePost")
		success = InvokeGameFramePost(query, error);
	else if (name == "Update")
		success = InvokeUpdate(query, error);
	else if (name == "UnitCreated")
		success = InvokeUnitCreated(query, error);
	else if (name == "UnitPreDamaged")
		success = InvokeUnitPreDamaged(query, result, error);
	else if (name == "AllowUnitCreation")
		success = InvokeAllowUnitCreation(query, result, error);
	else if (name == "DrawWorld")
		success = InvokeDrawWorld(error);
	else
		return false;

	if (!success) {
		backend->faulted = true;
		backend->faultReason = error.empty() ? "Core Wasm callin failed" : error;
	}
	return success;
}

bool WasmCoreHost::DispatchCallin(std::string_view name, const void* query, void* result,
	std::string& error)
{
	if (!KnownCallin(name))
		return false;

	bool handled = false;
	for (const auto& host : Hosts()) {
		if (host->backend == nullptr)
			continue;
#if defined(RECOIL_WASMTIME_AVAILABLE)
		bool present = false;
		if (name == "GameFrame") present = host->backend->bindings.HasGameFrame();
		else if (name == "GameFramePost") present = host->backend->bindings.HasGameFramePost();
		else if (name == "Update") present = host->backend->bindings.HasUpdate();
		else if (name == "UnitCreated") present = host->backend->bindings.HasUnitCreated();
		else if (name == "UnitPreDamaged") present = host->backend->bindings.HasUnitPreDamaged();
		else if (name == "AllowUnitCreation") present = host->backend->bindings.HasAllowUnitCreation();
		else if (name == "DrawWorld") present = host->backend->bindings.HasDrawWorld();
		if (!present)
			continue;
#endif
		handled = true;
		std::string hostError;
		if (!host->Invoke(name, query, result, hostError) && error.empty())
			error = hostError;
	}
	return handled;
}
