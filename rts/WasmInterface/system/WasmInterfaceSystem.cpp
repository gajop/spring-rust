/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmInterfaceSystem.h"

#include <algorithm>
#include <limits>
#include <utility>

#include "NativeInterface/NativeInterface.h"
#include "NativeInterface/api/Callins.h"
#include "Sim/Units/Scripts/UnitScriptEngine.h"
#include "System/Log/ILog.h"
#include "WasmCoreHost.h"

namespace {
	bool IsCoreModule(const std::vector<std::uint8_t>& bytes)
	{
		return bytes.size() >= 8 && bytes[0] == 0x00 && bytes[1] == 'a' &&
			bytes[2] == 's' && bytes[3] == 'm' && bytes[4] == 0x01 &&
			bytes[5] == 0x00 && bytes[6] == 0x00 && bytes[7] == 0x00;
	}

	bool DescriptorLess(const WasmModuleDescriptor& left, const WasmModuleDescriptor& right)
	{
		if (left.environment != right.environment)
			return static_cast<unsigned>(left.environment) < static_cast<unsigned>(right.environment);
		if (left.order != right.order)
			return left.order < right.order;
		if (left.archive != right.archive)
			return left.archive < right.archive;
		return left.name < right.name;
	}
}

WasmInterfaceSystem::WasmInterfaceSystem(NativeInterface* nativeInterface,
	WasmRuntimeConfig runtimeConfig)
	: runtime(std::make_unique<WasmRuntime>(std::move(runtimeConfig)))
	, nativeInterface(nativeInterface)
{
	LOG("Wasm Core interface system initialized (backend: %s)",
		runtime->IsAvailable() ? "Wasmtime" : "unavailable");
}

WasmInterfaceSystem::~WasmInterfaceSystem()
{
	UnloadAll();
}

bool WasmInterfaceSystem::LoadModule(WasmModuleDescriptor descriptor, std::string& error)
{
	if (descriptor.name.empty()) {
		error = "Wasm module name is empty";
		return false;
	}
	if (!WasmEnvironmentMatrix::IsRuntimeEnabled(descriptor.environment)) {
		error = "Wasm module environment is disabled: " +
			std::string(WasmEnvironmentMatrix::Name(descriptor.environment));
		return false;
	}
	if (!IsCoreModule(descriptor.bytes)) {
		error = "Wasm module is not a Core WebAssembly binary";
		return false;
	}
	if (nativeInterface == nullptr) {
		error = "Core Wasm interface has no NativeInterface";
		return false;
	}
	if (std::any_of(coreModules.begin(), coreModules.end(), [&descriptor](const auto& module) {
			return module.descriptor.name == descriptor.name;
		})) {
		error = "duplicate Wasm module name: " + descriptor.name;
		return false;
	}

	WasmModuleIdentity identity;
	if (!WasmCoreHost::Load(descriptor.name, descriptor.bytes, nativeInterface,
			descriptor.environment, *runtime, identity, error)) {
		LOG_L(L_ERROR, "Failed to initialize Core Wasm module %s: %s",
			descriptor.name.c_str(), error.c_str());
		return false;
	}
	WasmCoreHost* host = WasmCoreHost::ModuleHandle(descriptor.name);
	CoreModuleRecord module{
		.descriptor = std::move(descriptor),
		.identity = std::move(identity),
		.host = host,
	};
	if (module.host == nullptr) {
		error = "Core Wasm host disappeared immediately after loading module " +
			module.descriptor.name;
		WasmCoreHost::Unload(module.descriptor.name);
		return false;
	}
	if (unitScriptEngine != nullptr)
		unitScriptEngine->AddCusBackend(module.host);
	coreModules.push_back(std::move(module));
	std::stable_sort(coreModules.begin(), coreModules.end(),
		[](const CoreModuleRecord& left, const CoreModuleRecord& right) {
			return DescriptorLess(left.descriptor, right.descriptor);
		});
	InvalidateSubscribers();
	return true;
}

bool WasmInterfaceSystem::LoadManifest(std::string_view manifest,
	const ModuleBytesProvider& bytesProvider, std::string& error)
{
	if (!bytesProvider) {
		error = "Wasm module bytes provider is empty";
		return false;
	}
	return LoadManifests({WasmManifestSource{"", std::string(manifest)}},
		[&bytesProvider](std::string_view, std::string_view path,
			std::vector<std::uint8_t>& bytes, std::string& providerError) {
			return bytesProvider(path, bytes, providerError);
		}, error);
}

bool WasmInterfaceSystem::LoadManifests(const std::vector<WasmManifestSource>& sources,
	const ArchiveModuleBytesProvider& bytesProvider, std::string& error)
{
	if (!bytesProvider) {
		error = "Wasm archive module bytes provider is empty";
		return false;
	}
	constexpr std::size_t maxDeclarations = 256;
	std::vector<WasmModuleDeclaration> declarations;
	for (const auto& source : sources) {
		std::vector<WasmModuleDeclaration> parsed;
		if (!WasmModuleManifest::Parse(source.text, parsed, error))
			return false;
		if (parsed.size() > maxDeclarations - std::min(maxDeclarations, declarations.size())) {
			error = "Wasm manifests contain more than " + std::to_string(maxDeclarations) +
				" modules";
			return false;
		}
		for (auto& declaration : parsed) {
			if (std::any_of(declarations.begin(), declarations.end(), [&declaration](const auto& existing) {
					return existing.name == declaration.name;
				})) {
				error = "Wasm manifests contain duplicate module " + declaration.name;
				return false;
			}
			if (std::any_of(coreModules.begin(), coreModules.end(), [&declaration](const auto& existing) {
					return existing.descriptor.name == declaration.name;
				})) {
				error = "Wasm manifests duplicate loaded module " + declaration.name;
				return false;
			}
			declaration.archive = source.archive;
			declarations.push_back(std::move(declaration));
		}
	}

	std::vector<std::string> loaded;
	for (const auto& declaration : declarations) {
		std::vector<std::uint8_t> bytes;
		if (!bytesProvider(declaration.archive, declaration.path, bytes, error)) {
			for (const auto& name : loaded)
				UnloadModule(name);
			return false;
		}
		WasmModuleDescriptor descriptor;
		descriptor.name = declaration.name;
		descriptor.source = declaration.path;
		descriptor.environment = declaration.environment;
		descriptor.order = declaration.order;
		descriptor.interfaceVersion = declaration.interfaceVersion;
		descriptor.bytes = std::move(bytes);
		descriptor.archive = declaration.archive;
		if (!LoadModule(std::move(descriptor), error)) {
			for (const auto& name : loaded)
				UnloadModule(name);
			return false;
		}
		loaded.push_back(declaration.name);
	}
	return true;
}

bool WasmInterfaceSystem::UnloadModule(std::string_view moduleName)
{
	const auto iter = std::find_if(coreModules.begin(), coreModules.end(),
		[moduleName](const CoreModuleRecord& module) {
			return module.descriptor.name == moduleName;
		});
	if (iter == coreModules.end())
		return false;
	if (unitScriptEngine != nullptr)
		unitScriptEngine->RemoveCusBackend(iter->host);
	coreModules.erase(iter);
	WasmCoreHost::Unload(moduleName);
	InvalidateSubscribers();
	return true;
}

void WasmInterfaceSystem::UnloadAll()
{
	for (const CoreModuleRecord& module : coreModules) {
		if (unitScriptEngine != nullptr)
			unitScriptEngine->RemoveCusBackend(module.host);
		WasmCoreHost::Unload(module.descriptor.name);
	}
	coreModules.clear();
	InvalidateSubscribers();
}

void WasmInterfaceSystem::Update()
{
	if (WasmCoreHost::PendingUnsyncedFaults() == 0)
		return;
	RemoveFaultedUnsyncedModules();
}

void WasmInterfaceSystem::Tick(std::uint32_t frame)
{
	for (const CoreModuleRecord& module : coreModules) {
		if (module.host != nullptr)
			module.host->Tick(frame);
	}
}

bool WasmInterfaceSystem::DispatchSyncedMessage(std::string_view message, std::string& error)
{
	if (message.size() > std::numeric_limits<std::uint32_t>::max()) {
		error = "synced Wasm message exceeds the 32-bit length limit";
		return false;
	}
	RecvFromSyncedQuery query{message.data(), static_cast<std::uint32_t>(message.size())};
	bool handled = false;
	return DispatchActiveCoreCallin(CoreCallinOf("RecvFromSynced"), &query, false,
		nullptr, handled, error);
}

std::size_t WasmInterfaceSystem::ModuleCount() const
{
	return coreModules.size();
}

bool WasmInterfaceSystem::HasModules(WasmEnvironment environment) const
{
	return HasCoreModules(environment);
}

std::vector<std::string> WasmInterfaceSystem::SyncedConfiguration() const
{
	std::vector<std::pair<int, std::string>> ordered;
	for (const CoreModuleRecord& module : coreModules) {
		if (!WasmEnvironmentMatrix::Policy(module.descriptor.environment).synced)
			continue;
		ordered.emplace_back(module.descriptor.order,
			module.descriptor.name + "|" +
			WasmEnvironmentMatrix::Name(module.descriptor.environment) + "|" +
			std::to_string(module.descriptor.order) + "|" + module.descriptor.archive + "|" +
			module.identity.sha512 + "|" + runtime->ConfigurationIdentity() + "|interface=" +
			module.descriptor.interfaceVersion + "|abi=core-v1");
	}
	std::sort(ordered.begin(), ordered.end());
	std::vector<std::string> result;
	result.reserve(ordered.size());
	for (auto& entry : ordered)
		result.push_back(std::move(entry.second));
	return result;
}
