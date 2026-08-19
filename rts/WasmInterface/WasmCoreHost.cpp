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

bool IsCoreModule(const std::vector<std::uint8_t>& bytes)
{
	return bytes.size() >= 8 && bytes[0] == 0x00 && bytes[1] == 'a' &&
		bytes[2] == 's' && bytes[3] == 'm' && bytes[4] == 0x01 &&
		bytes[5] == 0x00 && bytes[6] == 0x00 && bytes[7] == 0x00;
}

WasmRuntimeConfig CoreRuntimeConfig()
{
	WasmRuntimeConfig config;
	config.allowThreads = false;
	config.allowRelaxedSimd = false;
	config.allowWasi = false;
	config.allowAotDeserialization = false;
	return config;
}

std::vector<std::unique_ptr<WasmCoreHost>>& Hosts()
{
	static std::vector<std::unique_ptr<WasmCoreHost>> hosts;
	return hosts;
}

} // namespace

struct WasmCoreHost::Backend {
	explicit Backend(NativeInterface* nativeInterface)
		: runtime(CoreRuntimeConfig())
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

	WasmRuntime runtime;
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
	NativeInterface* nativeInterface, WasmEnvironment environment, std::string& error)
{
	if (!Enabled())
		return false;
	if (!IsCoreModule(moduleBytes)) {
		error = "Core Wasm host requires a core module";
		return false;
	}
	if (environment != WasmEnvironment::RulesSynced && environment != WasmEnvironment::GaiaSynced) {
		error = "Core Wasm host currently supports synced gadget environments only";
		return false;
	}
	if (nativeInterface == nullptr) {
		error = "Core Wasm host has no NativeInterface";
		return false;
	}

	auto backend = std::make_unique<Backend>(nativeInterface);
	const WasmValidationResult validation = backend->runtime.ValidateModule(
		moduleBytes, environment, WasmEnvironmentMatrix::Name(environment));
	if (!validation.valid) {
		error = "Core Wasm validation failed: " + validation.error;
		return false;
	}
	if (!backend->runtime.IsAvailable()) {
		error = "Wasmtime is unavailable for the Core Wasm host";
		return false;
	}

#if defined(RECOIL_WASMTIME_AVAILABLE)
	auto* engine = static_cast<wasm_engine_t*>(backend->runtime.BackendEngine());
	backend->store = wasmtime_store_new(engine, nullptr, nullptr);
	if (backend->store == nullptr) {
		error = "Core Wasm host could not create a store";
		return false;
	}
	wasmtime_store_limiter(backend->store,
		static_cast<std::int64_t>(backend->runtime.Config().maxMemoryPages) * 65536,
		static_cast<std::int64_t>(backend->runtime.Config().maxTableElements), 2, 2, 2);
	if (backend->runtime.Config().instructionFuel != 0) {
		if (wasmtime_error_t* fuelError = wasmtime_context_set_fuel(
				wasmtime_store_context(backend->store), backend->runtime.Config().instructionFuel);
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
	if (backend == nullptr || backend->faulted) {
		error = backend == nullptr ? "Core Wasm host has no backend" : backend->faultReason;
		return false;
	}
#if defined(RECOIL_WASMTIME_AVAILABLE)
	if (!backend->bindings.HasGameFrame())
		return true;
	const auto* typed = static_cast<const GameFrameQuery*>(query);
	if (typed == nullptr) {
		error = "Core Wasm GameFrame query is null";
		return false;
	}
	if (!backend->budget.ChargeHost(1)) {
		error = "Core Wasm callin host-work budget exhausted";
		backend->faulted = true;
		backend->faultReason = error;
		return false;
	}
	if (!backend->bindings.GameFrame(wasmtime_store_context(backend->store),
		typed->gameFrame, error)) {
		backend->faulted = true;
		backend->faultReason = error;
		return false;
	}
	return true;
#else
	(void)query;
	error = "Wasmtime is unavailable for the Core Wasm host";
	return false;
#endif
}

bool WasmCoreHost::DispatchCallin(std::string_view name, const void* query, void*,
	std::string& error)
{
	if (name != "GameFrame" || query == nullptr)
		return false;

	bool handled = false;
	for (const auto& host : Hosts()) {
#if defined(RECOIL_WASMTIME_AVAILABLE)
		if (host->backend == nullptr || !host->backend->bindings.HasGameFrame())
			continue;
#endif
		handled = true;
		std::string hostError;
		if (!host->InvokeGameFrame(query, hostError) && error.empty())
			error = hostError;
	}
	return handled;
}
