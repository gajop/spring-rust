/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmInterfaceSystem.h"

#include "WasmCoreHost.h"
#include "WasmTypedHost.h"

#include <algorithm>
#include <cctype>
#include <iterator>
#include <map>
#include <unordered_map>
#include <limits>
#include <utility>

#include "System/Log/ILog.h"
#include "wasm/generated/WasmCallinRegistry.h"

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

	std::string ToWitName(std::string_view value)
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

	const recoil::wasm::generated::CallinDescriptor* FindCallin(std::string_view name)
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

	const std::string& CallinExportPath(
		const recoil::wasm::generated::CallinDescriptor* descriptor,
		WasmEnvironment environment)
	{
		static std::map<std::pair<const void*, WasmEnvironment>, std::string> paths;
		const auto key = std::make_pair(static_cast<const void*>(descriptor), environment);
		const auto iter = paths.find(key);
		if (iter != paths.end())
			return iter->second;
		return paths.emplace(key, "recoil:spring-api/callins-" +
			ToWitName(WasmEnvironmentMatrix::Name(environment)) + "@1.0.0/" +
			ToWitName(descriptor->name)).first->second;
	}

	bool UnwrapCallinResult(const WasmValue& value, WasmValue& payload,
		std::string& error)
	{
		const auto* record = std::get_if<WasmValueRecord>(&value.storage);
		if (record == nullptr) {
			error = "Wasm callin returned a value outside its result envelope";
			return false;
		}
		const auto okIter = record->find("ok");
		const auto* ok = okIter == record->end() ? nullptr :
			std::get_if<bool>(&okIter->second.storage);
		if (ok == nullptr) {
			error = "Wasm callin result envelope has an invalid success flag";
			return false;
		}
		const auto valueIter = record->find("value");
		if (!*ok) {
			error = "Wasm callin returned spring error";
			if (valueIter != record->end()) {
				if (const auto* errorRecord = std::get_if<WasmValueRecord>(
					&valueIter->second.storage)) {
					const auto codeIter = errorRecord->find("code");
					if (codeIter != errorRecord->end()) {
						if (const auto* code = std::get_if<std::int64_t>(
							&codeIter->second.storage))
							error += " (code " + std::to_string(*code) + ")";
						else if (const auto* code = std::get_if<std::uint64_t>(
							&codeIter->second.storage))
							error += " (code " + std::to_string(*code) + ")";
					}
				}
			}
			return false;
		}
		payload = valueIter == record->end() ? WasmValue::Unit() : valueIter->second;
		return true;
	}

	bool ReadBooleanResult(const WasmValue& payload, bool& value, std::string& error)
	{
		const auto* record = std::get_if<WasmValueRecord>(&payload.storage);
		if (record == nullptr) {
			error = "Wasm callin boolean result is not a record";
			return false;
		}
		const auto iter = record->find("value");
		if (iter == record->end()) {
			error = "Wasm callin boolean result has no value field";
			return false;
		}
		const auto* boolean = std::get_if<bool>(&iter->second.storage);
		if (boolean == nullptr) {
			error = "Wasm callin boolean result has a non-boolean value";
			return false;
		}
		value = *boolean;
		return true;
	}

	bool ReadStringResult(const WasmValue& payload, std::string& value,
		std::string& error)
	{
		const auto* record = std::get_if<WasmValueRecord>(&payload.storage);
		if (record == nullptr) {
			error = "Wasm callin string result is not a record";
			return false;
		}
		const auto iter = record->find("value");
		if (iter == record->end()) {
			error = "Wasm callin string result has no value field";
			return false;
		}
		const auto* string = std::get_if<std::string>(&iter->second.storage);
		if (string == nullptr) {
			error = "Wasm callin string result has a non-string value";
			return false;
		}
		value = *string;
		return true;
	}
}

bool WasmInterfaceSystem::AggregateCallinResult(std::string_view aggregation,
	const WasmValue& value, bool& haveResult, WasmValue& result, std::string& error)
{
	if (aggregation == "or-true") {
		bool moduleValue = false;
		if (!ReadBooleanResult(value, moduleValue, error))
			return false;
		bool aggregateValue = false;
		if (haveResult && !ReadBooleanResult(result, aggregateValue, error))
			return false;
		result = WasmValue::Record({
			{"value", WasmValue::Bool(aggregateValue || moduleValue)},
		});
		haveResult = true;
		return true;
	}

	if (aggregation == "and-false") {
		bool moduleValue = true;
		if (!ReadBooleanResult(value, moduleValue, error))
			return false;
		bool aggregateValue = true;
		if (haveResult && !ReadBooleanResult(result, aggregateValue, error))
			return false;
		result = WasmValue::Record({
			{"value", WasmValue::Bool(aggregateValue && moduleValue)},
		});
		haveResult = true;
		return true;
	}

	if (aggregation == "first") {
		if (!haveResult)
			result = value;
		haveResult = true;
		return true;
	}

	if (aggregation == "first-non-empty") {
		std::string moduleValue;
		if (!ReadStringResult(value, moduleValue, error))
			return false;
		if (!haveResult) {
			result = value;
		} else if (!moduleValue.empty()) {
			std::string aggregateValue;
			if (!ReadStringResult(result, aggregateValue, error))
				return false;
			if (aggregateValue.empty())
				result = value;
		}
		haveResult = true;
		return true;
	}

	error = "unknown Wasm callin aggregation rule: " + std::string(aggregation);
	return false;
}

WasmInterfaceSystem::WasmInterfaceSystem(WasmHostAdapter* hostAdapter,
	WasmRuntimeConfig runtimeConfig)
	: runtime(std::make_unique<WasmRuntime>(std::move(runtimeConfig)))
	, hostAdapter(hostAdapter)
{
	LOG("Wasm interface system initialized (policy backend: %s)",
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
	const bool duplicateComponent = std::any_of(modules.begin(), modules.end(),
		[&descriptor](const auto& module) {
			return module->Descriptor().name == descriptor.name;
		});
	const bool duplicateCore = std::any_of(coreModules.begin(), coreModules.end(),
		[&descriptor](const CoreModuleRecord& module) {
			return module.descriptor.name == descriptor.name;
		});
	if (duplicateComponent || duplicateCore) {
		error = "duplicate Wasm module name: " + descriptor.name;
		return false;
	}

	const bool coreModule = IsCoreModule(descriptor.bytes);
	if (coreModule && WasmCoreHost::Enabled()) {
		if (hostAdapter == nullptr || hostAdapter->NativeInterfaceHandle() == nullptr) {
			error = "Core Wasm requires the NativeInterface host adapter";
			return false;
		}
		WasmModuleIdentity identity;
		std::string coreError;
		if (!WasmCoreHost::Load(descriptor.name, descriptor.bytes,
				static_cast<NativeInterface*>(hostAdapter->NativeInterfaceHandle()),
				descriptor.environment, *runtime, identity, coreError)) {
			error = "could not start the Core Wasm host: " + coreError;
			LOG_L(L_ERROR, "%s", error.c_str());
			return false;
		}
		coreModules.push_back({std::move(descriptor), std::move(identity)});
		std::stable_sort(coreModules.begin(), coreModules.end(),
			[](const CoreModuleRecord& left, const CoreModuleRecord& right) {
				return DescriptorLess(left.descriptor, right.descriptor);
			});
		return true;
	}

	const std::string moduleName = descriptor.name;
	if (!coreModule && WasmTypedHost::TypedEnabled() && hostAdapter != nullptr) {
		const SpringTypedWorld world = descriptor.environment == WasmEnvironment::UI
			? SpringTypedWorld::UI
			: WasmEnvironmentMatrix::Policy(descriptor.environment).synced
				? SpringTypedWorld::RulesSynced
				: SpringTypedWorld::RulesUnsynced;
		std::string typedError;
		if (!WasmTypedHost::Load(descriptor.name, descriptor.bytes,
			static_cast<NativeInterface*>(hostAdapter->NativeInterfaceHandle()), world,
			typedError)) {
			error = "could not start the typed Wasm host: " + typedError;
			LOG_L(L_ERROR, "%s", error.c_str());
			return false;
		}
	}

	auto module = std::make_unique<WasmModule>(nextInstanceID++, std::move(descriptor), *runtime,
		hostAdapter);
	if (!module->Initialize(error)) {
		if (WasmTypedHost::TypedEnabled())
			WasmTypedHost::Unload(moduleName);
		LOG_L(L_ERROR, "Failed to initialize Wasm module: %s", error.c_str());
		return false;
	}
	modules.push_back(std::move(module));
	std::stable_sort(modules.begin(), modules.end(), [](const auto& left, const auto& right) {
		return DescriptorLess(left->Descriptor(), right->Descriptor());
	});
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
			const bool loadedComponent = std::any_of(modules.begin(), modules.end(),
				[&declaration](const auto& module) {
					return module->Descriptor().name == declaration.name;
				});
			const bool loadedCore = std::any_of(coreModules.begin(), coreModules.end(),
				[&declaration](const CoreModuleRecord& module) {
					return module.descriptor.name == declaration.name;
				});
			if (loadedComponent || loadedCore) {
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
	const auto coreIter = std::find_if(coreModules.begin(), coreModules.end(),
		[moduleName](const CoreModuleRecord& module) {
			return module.descriptor.name == moduleName;
		});
	if (coreIter != coreModules.end()) {
		coreModules.erase(coreIter);
		WasmCoreHost::Unload(moduleName);
		return true;
	}

	const auto iter = std::find_if(modules.begin(), modules.end(), [moduleName](const auto& module) {
		return module->Descriptor().name == moduleName;
	});
	if (iter == modules.end())
		return false;
	modules.erase(iter);
	WasmTypedHost::Unload(moduleName);
	return true;
}

void WasmInterfaceSystem::UnloadAll()
{
	modules.clear();
	coreModules.clear();
	WasmCoreHost::UnloadAll();
	WasmTypedHost::UnloadAll();
}

void WasmInterfaceSystem::Update()
{
	modules.erase(std::remove_if(modules.begin(), modules.end(), [](const auto& module) {
		if (module->State() != WasmModuleState::Faulted ||
			WasmEnvironmentMatrix::Policy(module->Descriptor().environment).synced)
			return false;
		LOG_L(L_WARNING, "Unsynced Wasm module faulted and was unloaded: %s: %s",
			module->Descriptor().name.c_str(), module->FaultReason().c_str());
		return true;
	}), modules.end());
}

void WasmInterfaceSystem::FaultSyncedModules(WasmEnvironment environment,
	std::string_view reason)
{
	if (!WasmEnvironmentMatrix::Policy(environment).synced)
		return;
	for (const auto& module : modules) {
		if (module->Descriptor().environment == environment)
			module->Fault(std::string(reason));
	}
}

bool WasmInterfaceSystem::DispatchCallin(const WasmCallinEvent& event,
	WasmEnvironment environment, std::string& error)
{
	const auto* descriptor = FindCallin(event.name);
	if (descriptor == nullptr) {
		error = "unknown Wasm callin: " + event.name;
		return false;
	}
	const std::uint32_t environmentBit = 1u << static_cast<std::uint32_t>(environment);
	if ((descriptor->environmentMask & environmentBit) == 0) {
		error = "callin is not available in environment " +
			std::string(WasmEnvironmentMatrix::Name(environment));
		return false;
	}
	WasmCallinEvent canonicalEvent = event;
	canonicalEvent.name = descriptor->name;
	for (const auto& module : modules) {
		if (module->Descriptor().environment != environment)
			continue;
		if (!WasmDispatch::Dispatch(*module, canonicalEvent, error)) {
			if (WasmEnvironmentMatrix::Policy(environment).synced)
				module->Fault(error);
			return false;
		}
	}
	return true;
}

bool WasmInterfaceSystem::DispatchCallin(std::string_view name,
	const std::vector<WasmValue>& arguments, WasmEnvironment environment,
	std::string& error)
{
	WasmValue ignored;
	return DispatchCallin(name, arguments, environment, ignored, error);
}

bool WasmInterfaceSystem::DispatchCallin(std::string_view name,
	const std::vector<WasmValue>& arguments, WasmEnvironment environment,
	WasmValue& result, std::string& error)
{
	result = WasmValue::Unit();
	const auto* descriptor = FindCallin(name);
	if (descriptor == nullptr) {
		error = "unknown Wasm callin: " + std::string(name);
		return false;
	}
	const std::uint32_t environmentBit = 1u << static_cast<std::uint32_t>(environment);
	if ((descriptor->environmentMask & environmentBit) == 0) {
		error = "callin is not available in environment " +
			std::string(WasmEnvironmentMatrix::Name(environment));
		return false;
	}
	const auto matches = [environment](const std::unique_ptr<WasmModule>& module) {
		return module->Descriptor().environment == environment;
	};
	if (std::none_of(modules.begin(), modules.end(), matches))
		return true;
	const std::string& exportPath = CallinExportPath(descriptor, environment);
	bool haveResult = false;
	for (const auto& module : modules) {
		if (!matches(module))
			continue;
		WasmValue rawResult;
		if (!module->Callin(exportPath, arguments, rawResult, error)) {
			if (WasmEnvironmentMatrix::Policy(environment).synced)
				module->Fault(error);
			return false;
		}
		if (rawResult.IsUnit())
			continue;
		WasmValue payload;
		if (!UnwrapCallinResult(rawResult, payload, error)) {
			if (WasmEnvironmentMatrix::Policy(environment).synced)
				module->Fault(error);
			return false;
		}
		if (descriptor->aggregation == "ignore")
			continue;

		if (!AggregateCallinResult(descriptor->aggregation, payload, haveResult, result, error)) {
			if (WasmEnvironmentMatrix::Policy(environment).synced)
				module->Fault(error);
			return false;
		}
	}
	return true;
}

bool WasmInterfaceSystem::DispatchCallin(std::string_view name,
	const std::vector<WasmValue>& arguments,
	const std::vector<WasmEnvironment>& environments, WasmValue& result,
	std::string& error)
{
	std::vector<CallinInvocation> invocations;
	invocations.reserve(environments.size());
	for (const WasmEnvironment environment : environments)
		invocations.push_back({environment, arguments});
	return DispatchCallin(name, invocations, result, error);
}

bool WasmInterfaceSystem::DispatchCallin(std::string_view name,
	const std::vector<CallinInvocation>& invocations, WasmValue& result,
	std::string& error)
{
	result = WasmValue::Unit();
	const auto* descriptor = FindCallin(name);
	if (descriptor == nullptr) {
		error = "unknown Wasm callin: " + std::string(name);
		return false;
	}
	bool haveResult = false;
	for (const CallinInvocation& invocation : invocations) {
		const WasmEnvironment environment = invocation.environment;
		const std::uint32_t environmentBit = 1u << static_cast<std::uint32_t>(environment);
		if ((descriptor->environmentMask & environmentBit) == 0)
			continue;
		WasmValue environmentResult;
		if (!DispatchCallin(name, invocation.arguments, environment, environmentResult, error))
			return false;
		if (!invocation.contributesResult || environmentResult.IsUnit() ||
			descriptor->aggregation == "ignore")
			continue;

		if (!AggregateCallinResult(descriptor->aggregation, environmentResult,
			haveResult, result, error)) {
			for (const CallinInvocation& candidate : invocations)
				FaultSyncedModules(candidate.environment, error);
			return false;
		}
	}
	return true;
}

bool WasmInterfaceSystem::DispatchSyncedMessage(std::string_view message,
	std::string& error)
{
	if (message.size() > std::numeric_limits<std::uint32_t>::max()) {
		error = "synced Wasm message exceeds the 32-bit length limit";
		return false;
	}

	WasmValue result;
	return DispatchCallin("RecvFromSynced", {
		WasmValue::Record({
			{"message", WasmValue::String(std::string(message))},
			{"messageLength", WasmValue::U64(message.size())},
		}),
	},
		{WasmEnvironment::RulesUnsynced, WasmEnvironment::GaiaUnsynced},
		result, error);
}

std::size_t WasmInterfaceSystem::ModuleCount() const
{
	return modules.size() + coreModules.size();
}

bool WasmInterfaceSystem::HasModules(WasmEnvironment environment) const
{
	const bool component = std::any_of(modules.begin(), modules.end(),
		[environment](const auto& module) {
			return module->Descriptor().environment == environment;
		});
	if (component)
		return true;
	return std::any_of(coreModules.begin(), coreModules.end(),
		[environment](const CoreModuleRecord& module) {
			return module.descriptor.environment == environment;
		});
}

std::vector<std::string> WasmInterfaceSystem::SyncedConfiguration() const
{
	// Component and Core modules live in two separate lists, so the listing has
	// to be re-sequenced rather than concatenated. Order by the declared module
	// order the same way a single-transport load would, and fall back to the
	// composed text only to break ties deterministically.
	std::vector<std::pair<int, std::string>> ordered;
	for (const auto& module : modules) {
		if (!WasmEnvironmentMatrix::Policy(module->Descriptor().environment).synced)
			continue;
		ordered.emplace_back(module->Descriptor().order,
			module->Descriptor().name + "|" +
			WasmEnvironmentMatrix::Name(module->Descriptor().environment) + "|" +
			std::to_string(module->Descriptor().order) + "|" + module->Descriptor().archive + "|" +
			module->Identity().sha512 + "|" +
			runtime->ConfigurationIdentity() + "|interface=" +
			module->Descriptor().interfaceVersion);
	}
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
