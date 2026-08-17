/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmInterfaceSystem.h"

#include <algorithm>
#include <cctype>
#include <iterator>

#include "System/Log/ILog.h"
#include "wasm/generated/WasmCallinRegistry.h"

namespace {
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
		const auto* begin = std::begin(recoil::wasm::generated::kCallins);
		const auto* end = std::end(recoil::wasm::generated::kCallins);
		const auto iter = std::find_if(begin, end, [name](const auto& callin) {
			return name == callin.name;
		});
		return iter == end ? nullptr : &*iter;
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

WasmInterfaceSystem::WasmInterfaceSystem(WasmHostAdapter* hostAdapter)
	: runtime(std::make_unique<WasmRuntime>())
	, hostAdapter(hostAdapter)
{
	LOG("Wasm interface system initialized (policy backend: %s)",
		runtime->IsAvailable() ? "Wasmtime Component Model" : "unavailable");
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
	if (std::any_of(modules.begin(), modules.end(), [&descriptor](const auto& module) {
		return module->Descriptor().name == descriptor.name;
	})) {
		error = "duplicate Wasm module name: " + descriptor.name;
		return false;
	}

	auto module = std::make_unique<WasmModule>(nextInstanceID++, std::move(descriptor), *runtime,
		hostAdapter);
	if (!module->Initialize(error)) {
		LOG_L(L_ERROR, "Failed to initialize Wasm module: %s", error.c_str());
		return false;
	}
	modules.push_back(std::move(module));
	std::stable_sort(modules.begin(), modules.end(), [](const auto& left, const auto& right) {
		if (left->Descriptor().environment != right->Descriptor().environment)
			return static_cast<unsigned>(left->Descriptor().environment) <
				static_cast<unsigned>(right->Descriptor().environment);
		if (left->Descriptor().order != right->Descriptor().order)
			return left->Descriptor().order < right->Descriptor().order;
		if (left->Descriptor().archive != right->Descriptor().archive)
		return left->Descriptor().archive < right->Descriptor().archive;
		return left->Descriptor().name < right->Descriptor().name;
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
			if (std::any_of(modules.begin(), modules.end(), [&declaration](const auto& module) {
				return module->Descriptor().name == declaration.name;
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
	const auto iter = std::find_if(modules.begin(), modules.end(), [moduleName](const auto& module) {
		return module->Descriptor().name == moduleName;
	});
	if (iter == modules.end())
		return false;
	modules.erase(iter);
	return true;
}

void WasmInterfaceSystem::UnloadAll()
{
	modules.clear();
}

void WasmInterfaceSystem::Update()
{
	// Reload/unload is performed at explicit safe lifecycle points. A faulted
	// unsynced instance can be removed without affecting other instances; a
	// synced fault is retained for match-fatal reporting.
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
	canonicalEvent.name = descriptor->canonical;
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
	const std::string exportPath = "recoil:spring-api/callins-" +
		ToWitName(WasmEnvironmentMatrix::Name(environment)) + "@1.0.0/" +
		ToWitName(descriptor->canonical);
	bool haveResult = false;
	for (const auto& module : modules) {
		if (module->Descriptor().environment != environment)
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

		if (descriptor->aggregation == "or-true") {
			bool moduleValue = false;
			if (!ReadBooleanResult(payload, moduleValue, error)) {
				if (WasmEnvironmentMatrix::Policy(environment).synced)
					module->Fault(error);
				return false;
			}
			bool aggregateValue = false;
			if (haveResult && !ReadBooleanResult(result, aggregateValue, error)) {
				FaultSyncedModules(environment, error);
				return false;
			}
			result = WasmValue::Record({
				{"value", WasmValue::Bool(aggregateValue || moduleValue)},
			});
			haveResult = true;
			continue;
		}

		if (descriptor->aggregation == "first") {
			if (!haveResult)
				result = payload;
			haveResult = true;
			continue;
		}

		if (descriptor->aggregation == "first-non-empty") {
			std::string moduleValue;
			if (!ReadStringResult(payload, moduleValue, error)) {
				if (WasmEnvironmentMatrix::Policy(environment).synced)
					module->Fault(error);
				return false;
			}
			if (!haveResult) {
				result = payload;
			} else if (!moduleValue.empty()) {
				std::string aggregateValue;
				if (!ReadStringResult(result, aggregateValue, error)) {
					FaultSyncedModules(environment, error);
					return false;
				}
				if (aggregateValue.empty())
					result = payload;
			}
			haveResult = true;
			continue;
		}

		error = "unknown Wasm callin aggregation rule: " +
			std::string(descriptor->aggregation);
		FaultSyncedModules(environment, error);
		return false;
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
		// Fan-out callers intentionally provide the candidate environments for
		// an engine event.  The canonical callin inventory is the final filter;
		// an event that is not meaningful in one candidate environment must not
		// turn an otherwise valid dispatch into an error.
		if ((descriptor->environmentMask & environmentBit) == 0)
			continue;
		WasmValue environmentResult;
		if (!DispatchCallin(name, invocation.arguments, environment, environmentResult, error))
			return false;
		if (!invocation.contributesResult || environmentResult.IsUnit() ||
			descriptor->aggregation == "ignore")
			continue;

		if (descriptor->aggregation == "or-true") {
			bool environmentValue = false;
			if (!ReadBooleanResult(environmentResult, environmentValue, error)) {
				FaultSyncedModules(environment, error);
				return false;
			}
			bool aggregateValue = false;
			if (haveResult && !ReadBooleanResult(result, aggregateValue, error)) {
				FaultSyncedModules(environment, error);
				return false;
			}
			result = WasmValue::Record({
				{"value", WasmValue::Bool(aggregateValue || environmentValue)},
			});
			haveResult = true;
			continue;
		}

		if (descriptor->aggregation == "first") {
			if (!haveResult)
				result = environmentResult;
			haveResult = true;
			continue;
		}

		if (descriptor->aggregation == "first-non-empty") {
			std::string environmentValue;
			if (!ReadStringResult(environmentResult, environmentValue, error)) {
				FaultSyncedModules(environment, error);
				return false;
			}
			if (!haveResult) {
				result = environmentResult;
			} else if (!environmentValue.empty()) {
				std::string aggregateValue;
				if (!ReadStringResult(result, aggregateValue, error)) {
					FaultSyncedModules(environment, error);
					return false;
				}
				if (aggregateValue.empty())
					result = environmentResult;
			}
			haveResult = true;
			continue;
		}

		error = "unknown Wasm callin aggregation rule: " +
			std::string(descriptor->aggregation);
		for (const CallinInvocation& invocation : invocations)
			FaultSyncedModules(invocation.environment, error);
		return false;
	}
	return true;
}

std::size_t WasmInterfaceSystem::ModuleCount() const
{
	return modules.size();
}

std::vector<std::string> WasmInterfaceSystem::SyncedConfiguration() const
{
	std::vector<std::string> result;
	for (const auto& module : modules) {
		if (!WasmEnvironmentMatrix::Policy(module->Descriptor().environment).synced)
			continue;
		result.push_back(module->Descriptor().name + "|" +
			WasmEnvironmentMatrix::Name(module->Descriptor().environment) + "|" +
			std::to_string(module->Descriptor().order) + "|" + module->Descriptor().archive + "|" +
			module->Identity().sha512 + "|" +
			runtime->ConfigurationIdentity() + "|interface=" +
			module->Descriptor().interfaceVersion);
	}
	return result;
}
