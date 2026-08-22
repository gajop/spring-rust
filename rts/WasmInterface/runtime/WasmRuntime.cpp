/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmRuntime.h"

#include <algorithm>
#include <array>
#include <cctype>
#include <charconv>
#include <iomanip>
#include <limits>
#include <sstream>
#include <span>
#include <utility>

#include "System/Sync/SHA512.hpp"
#include "wasm/generated/WasmCalloutRegistry.h"

#if defined(RECOIL_WASMTIME_AVAILABLE)
#include <wasmtime.h>
#endif

namespace {
	struct Reader {
		const std::vector<std::uint8_t>& bytes;
		std::size_t offset = 0;

		bool ReadByte(std::uint8_t& value)
		{
			if (offset >= bytes.size())
				return false;
			value = bytes[offset++];
			return true;
		}

		bool ReadLeb(std::uint64_t& value)
		{
			value = 0;
			for (unsigned shift = 0; shift < 64; shift += 7) {
				std::uint8_t byte = 0;
				if (!ReadByte(byte))
					return false;
				if (shift == 63 && byte > 1)
					return false;
				value |= static_cast<std::uint64_t>(byte & 0x7f) << shift;
				if ((byte & 0x80) == 0)
					return true;
			}
			return false;
		}

		bool ReadString(std::string& value)
		{
			std::uint64_t length = 0;
			if (!ReadLeb(length) || length > bytes.size() - offset)
				return false;
			value.assign(reinterpret_cast<const char*>(bytes.data() + offset), static_cast<std::size_t>(length));
			offset += static_cast<std::size_t>(length);
			return true;
		}
	};

	bool SkipLimits(Reader& reader, const WasmRuntimeConfig& config, bool memory, std::string& error)
	{
		std::uint64_t flags = 0;
		std::uint64_t minimum = 0;
		std::uint64_t maximum = 0;
		if (!reader.ReadLeb(flags) || !reader.ReadLeb(minimum)) {
			error = "truncated Wasm limits";
			return false;
		}
		if ((flags & ~0x03u) != 0) {
			error = "unsupported Wasm memory/table limit flags";
			return false;
		}
		if ((flags & 0x02) != 0 && !config.allowThreads) {
			error = "shared memory/threads are not allowed";
			return false;
		}
		if ((flags & 0x01) != 0) {
			if (!reader.ReadLeb(maximum)) {
				error = "truncated Wasm maximum";
				return false;
			}
			if (maximum < minimum) {
				error = "Wasm maximum is smaller than its minimum";
				return false;
			}
		}
		const auto limit = memory ? config.maxMemoryPages : config.maxTableElements;
		if (minimum > limit || ((flags & 0x01) != 0 && maximum > limit)) {
			error = "Wasm memory/table limit exceeds configured maximum";
			return false;
		}
		return true;
	}

	bool SkipImportDescription(Reader& reader, std::uint8_t kind,
		const WasmRuntimeConfig& config, std::string& error)
	{
		std::uint64_t ignored = 0;
		switch (kind) {
			case 0: // function
				return reader.ReadLeb(ignored);
			case 1: { // table
				std::uint8_t elementType = 0;
				return reader.ReadByte(elementType) && SkipLimits(reader, config, false, error);
			}
			case 2: // memory
				return SkipLimits(reader, config, true, error);
			case 3: { // global
				std::uint8_t valueType = 0;
				std::uint8_t mutability = 0;
				return reader.ReadByte(valueType) && reader.ReadByte(mutability);
			}
			case 4: { // exception tag
				std::uint8_t attribute = 0;
				return reader.ReadByte(attribute) && reader.ReadLeb(ignored);
			}
			default:
				error = "unknown Wasm import kind";
				return false;
		}
	}

	bool IsDeniedImport(std::string_view module, const WasmRuntimeConfig& config)
	{
		if (module == "wasi_snapshot_preview1" || module.starts_with("wasi"))
			return !config.allowWasi;
		const bool springImport = module == "spring" || module.starts_with("spring.");
		const bool recoilImport = module == "recoil" || module.starts_with("recoil.");
		return !(springImport || recoilImport);
	}

	bool IsAllowedCoreImport(std::string_view module, std::string_view field,
		std::uint8_t kind)
	{
		// Core Wasm is retained for the tiny compatibility/host-scalar path.  It
		// has exactly one registered host function; all other core imports must
		// fail validation before Wasmtime instantiation rather than becoming
		// linker traps after untrusted code has entered the runtime.
		return kind == 0 && module == "spring" && field == "add-i32";
	}

	std::string HashModule(const std::vector<std::uint8_t>& bytes)
	{
		sha512::raw_digest digest{};
		sha512::calc_digest(bytes, digest);
		return sha512::dump_digest(digest);
	}

	bool HasComponentVersion(const std::vector<std::uint8_t>& bytes)
	{
		return bytes.size() >= 8 && bytes[4] == 0x0d && bytes[5] == 0x00 &&
			bytes[6] == 0x01 && bytes[7] == 0x00;
	}

	struct ComponentBinaryReader {
		std::span<const std::uint8_t> bytes;
		std::size_t offset = 0;

		bool ReadByte(std::uint8_t& value)
		{
			if (offset >= bytes.size())
				return false;
			value = bytes[offset++];
			return true;
		}

		bool ReadLeb(std::uint64_t& value)
		{
			value = 0;
			for (unsigned shift = 0; shift < 64; shift += 7) {
				std::uint8_t byte = 0;
				if (!ReadByte(byte))
					return false;
				if (shift == 63 && byte > 1)
					return false;
				value |= static_cast<std::uint64_t>(byte & 0x7f) << shift;
				if ((byte & 0x80) == 0)
					return true;
			}
			return false;
		}
	};

	bool ValidateComponentBinaryNesting(std::span<const std::uint8_t> bytes,
		std::uint32_t maximum, std::string& error)
	{
		std::vector<std::pair<std::span<const std::uint8_t>, std::uint32_t>> pending;
		pending.emplace_back(bytes, 0);
		while (!pending.empty()) {
			const auto [component, depth] = pending.back();
			pending.pop_back();
			if (component.size() < 8 || component[0] != 0x00 || component[1] != 'a' ||
				component[2] != 's' || component[3] != 'm' || component[4] != 0x0d ||
				component[5] != 0x00 || component[6] != 0x01 || component[7] != 0x00) {
				error = "invalid nested Wasm Component Model binary";
				return false;
			}

			ComponentBinaryReader reader{component, 8};
			while (reader.offset < component.size()) {
				std::uint8_t sectionID = 0;
				std::uint64_t sectionSize = 0;
				if (!reader.ReadByte(sectionID) || !reader.ReadLeb(sectionSize) ||
					sectionSize > component.size() - reader.offset) {
					error = "invalid nested Wasm Component Model section";
					return false;
				}
				const auto payload = component.subspan(reader.offset,
					static_cast<std::size_t>(sectionSize));
				reader.offset += static_cast<std::size_t>(sectionSize);
				if (sectionID != 4)
					continue;
				if (depth >= maximum) {
					error = "Wasm Component Model nesting exceeds configured maximum";
					return false;
				}
				pending.emplace_back(payload, depth + 1);
			}
		}
		return true;
	}

#if defined(RECOIL_WASMTIME_AVAILABLE)
	std::string WasmtimeErrorMessage(wasmtime_error_t* error)
	{
		if (error == nullptr)
			return {};
		wasm_name_t message;
		wasmtime_error_message(error, &message);
		std::string result(message.data, message.size);
		wasm_name_delete(&message);
		wasmtime_error_delete(error);
		return result;
	}

	bool IsAllowedComponentImport(std::string_view name, const WasmRuntimeConfig& config)
	{
		if (name.starts_with(RECOIL_WASM_INTERFACE_PREFIX)) {
			const std::size_t version = name.rfind('@');
			return version != std::string_view::npos &&
				name.substr(version) == RECOIL_WASM_INTERFACE_VERSION &&
				version > RECOIL_WASM_INTERFACE_PREFIX.size() &&
				name.find('/', RECOIL_WASM_INTERFACE_PREFIX.size()) == std::string_view::npos;
		}
		return config.allowWasi && name.starts_with("wasi:");
	}

	std::string ComponentImportModule(std::string_view importName)
	{
		const std::size_t slash = importName.find_last_of('/');
		std::string module(importName.substr(slash == std::string_view::npos ? 0 : slash + 1));
		const std::size_t version = module.find('@');
		if (version != std::string::npos)
			module.resize(version);
	for (char& character : module) {
		if (character == '-')
			character = '_';
	}
	return std::string(recoil::wasm::generated::CanonicalModule(module));
}

	std::string ComponentImportFunction(std::string_view moduleName,
		std::string_view importName)
	{
		// WIT uses kebab-case, while the NativeInterface keeps its historical
		// spelling, including initialisms such as COB and ID.  Resolving through
		// the generated inventory preserves that canonical native spelling
		// instead of guessing it with a lossy Pascal-case conversion.
		for (const auto& descriptor : recoil::wasm::generated::kCallouts) {
			if (descriptor.module != moduleName)
				continue;
			std::string witName;
			witName.reserve(std::char_traits<char>::length(descriptor.name) + 4);
			for (std::size_t index = 0; descriptor.name[index] != '\0'; ++index) {
				const unsigned char character = static_cast<unsigned char>(descriptor.name[index]);
				const bool uppercase = std::isupper(character) != 0;
				const bool previousUppercase = index > 0 &&
					std::isupper(static_cast<unsigned char>(descriptor.name[index - 1])) != 0;
				const bool nextLowercase = descriptor.name[index + 1] != '\0' &&
					std::islower(static_cast<unsigned char>(descriptor.name[index + 1])) != 0;
				if (uppercase && index != 0 && (!previousUppercase || nextLowercase))
					witName.push_back('-');
				witName.push_back(static_cast<char>(std::tolower(character)));
			}
			if (witName == importName)
				return descriptor.name;
		}

		std::string function;
		bool uppercase = true;
		for (const char character : importName) {
			if (character == '-' || character == '_') {
				uppercase = true;
				continue;
			}
			if (uppercase) {
				function.push_back(static_cast<char>(std::toupper(
					static_cast<unsigned char>(character))));
				uppercase = false;
			} else {
				function.push_back(character);
			}
		}
		return function;
	}

	bool IsKnownComponentModule(std::string_view moduleName)
	{
		for (const auto& descriptor : recoil::wasm::generated::kCallouts) {
			if (moduleName == descriptor.module)
				return true;
		}
		return false;
	}

	const recoil::wasm::generated::CalloutDescriptor* FindComponentCallout(
		std::string_view moduleName, std::string_view functionName)
	{
		for (const auto& descriptor : recoil::wasm::generated::kCallouts) {
			if (moduleName == descriptor.module && functionName == descriptor.name)
				return &descriptor;
		}
		return nullptr;
	}

	bool ValidateComponentInterface(const wasmtime_component_item_t& item,
		std::string_view moduleName, WasmEnvironment environment,
		const WasmRuntimeConfig& config, wasm_engine_t* engine, std::string& error)
	{
		if (item.kind != WASMTIME_COMPONENT_ITEM_COMPONENT_INSTANCE ||
			item.of.component_instance == nullptr) {
			error = "Spring Component Model import is not an interface instance: " +
				std::string(moduleName);
			return false;
		}
		const auto* instanceType = item.of.component_instance;
		const std::size_t functionCount =
			wasmtime_component_instance_type_export_count(instanceType, engine);
		if (functionCount > config.maxImports) {
			error = "Spring Component Model interface function count exceeds configured maximum: " +
				std::string(moduleName);
			return false;
		}
		for (std::size_t index = 0; index < functionCount; ++index) {
			const char* name = nullptr;
			std::size_t nameLength = 0;
			wasmtime_component_item_t child{};
			if (!wasmtime_component_instance_type_export_nth(instanceType, engine, index, &name,
				&nameLength, &child)) {
				error = "Spring Component Model interface function reflection failed: " +
					std::string(moduleName);
				return false;
			}
			// Component interfaces also export their named records, enums,
			// variants, and resources.  Those are part of the type contract, not
			// callable host capabilities; only component functions can dispatch
			// into NativeInterface and therefore need callout inventory checks.
			if (child.kind != WASMTIME_COMPONENT_ITEM_COMPONENT_FUNC) {
				wasmtime_component_item_delete(&child);
				continue;
			}
			const std::string functionName = ComponentImportFunction(moduleName,
				{name, nameLength});
			const auto* descriptor = FindComponentCallout(moduleName, functionName);
#if defined(UNIT_TEST)
			if (descriptor == nullptr && config.allowUnregisteredComponentFunctionsForTesting) {
				wasmtime_component_item_delete(&child);
				continue;
			}
#endif
			if (descriptor == nullptr) {
				wasmtime_component_item_delete(&child);
				error = "Spring Component Model import names an unknown function: " +
					std::string(moduleName) + "." + functionName;
				return false;
			}
			const std::uint32_t environmentBit = 1u << static_cast<std::uint32_t>(environment);
			if ((descriptor->environmentMask & environmentBit) == 0) {
				wasmtime_component_item_delete(&child);
				error = "Spring Component Model import function is unavailable in environment " +
					std::string(WasmEnvironmentMatrix::Name(environment)) + ": " +
					std::string(moduleName) + "." + functionName;
				return false;
			}
			wasmtime_component_item_delete(&child);
		}
		return true;
	}

	bool ValidateComponentItemNesting(const wasmtime_component_item_t& item,
		wasm_engine_t* engine, std::uint32_t maximum, std::uint32_t depth,
		std::string& error);

	bool ValidateComponentTypeNesting(const wasmtime_component_type_t* type,
		wasm_engine_t* engine, std::uint32_t maximum, std::uint32_t depth,
		std::string& error)
	{
		const std::size_t importCount = wasmtime_component_type_import_count(type, engine);
		for (std::size_t index = 0; index < importCount; ++index) {
			const char* name = nullptr;
			std::size_t nameLength = 0;
			wasmtime_component_item_t item{};
			if (!wasmtime_component_type_import_nth(type, engine, index, &name, &nameLength, &item)) {
				error = "Wasm Component Model nested import reflection failed";
				return false;
			}
			(void)name;
			(void)nameLength;
			const bool valid = ValidateComponentItemNesting(item, engine, maximum, depth, error);
			wasmtime_component_item_delete(&item);
			if (!valid)
				return false;
		}
		const std::size_t exportCount = wasmtime_component_type_export_count(type, engine);
		for (std::size_t index = 0; index < exportCount; ++index) {
			const char* name = nullptr;
			std::size_t nameLength = 0;
			wasmtime_component_item_t item{};
			if (!wasmtime_component_type_export_nth(type, engine, index, &name, &nameLength, &item)) {
				error = "Wasm Component Model nested export reflection failed";
				return false;
			}
			(void)name;
			(void)nameLength;
			const bool valid = ValidateComponentItemNesting(item, engine, maximum, depth, error);
			wasmtime_component_item_delete(&item);
			if (!valid)
				return false;
		}
		return true;
	}

	bool ValidateComponentInstanceNesting(const wasmtime_component_instance_type_t* type,
		wasm_engine_t* engine, std::uint32_t maximum, std::uint32_t depth,
		std::string& error)
	{
		const std::size_t exportCount = wasmtime_component_instance_type_export_count(type, engine);
		for (std::size_t index = 0; index < exportCount; ++index) {
			const char* name = nullptr;
			std::size_t nameLength = 0;
			wasmtime_component_item_t item{};
			if (!wasmtime_component_instance_type_export_nth(type, engine, index,
				&name, &nameLength, &item)) {
				error = "Wasm Component Model nested instance reflection failed";
				return false;
			}
			(void)name;
			(void)nameLength;
			const bool valid = ValidateComponentItemNesting(item, engine, maximum, depth, error);
			wasmtime_component_item_delete(&item);
			if (!valid)
				return false;
		}
		return true;
	}

	bool ValidateComponentItemNesting(const wasmtime_component_item_t& item,
		wasm_engine_t* engine, std::uint32_t maximum, std::uint32_t depth,
		std::string& error)
	{
		if (item.kind != WASMTIME_COMPONENT_ITEM_COMPONENT &&
			item.kind != WASMTIME_COMPONENT_ITEM_COMPONENT_INSTANCE)
			return true;
		if (depth >= maximum) {
			error = "Wasm Component Model nesting exceeds configured maximum";
			return false;
		}
		if (item.kind == WASMTIME_COMPONENT_ITEM_COMPONENT)
			return ValidateComponentTypeNesting(item.of.component, engine, maximum, depth + 1, error);
		return ValidateComponentInstanceNesting(item.of.component_instance, engine,
			maximum, depth + 1, error);
	}

	bool ValidateComponentImports(const std::vector<std::uint8_t>& bytes,
		const WasmRuntimeConfig& config, WasmEnvironment environment, wasm_engine_t* engine,
		std::vector<std::string>& imports, std::string& error)
	{
		wasmtime_component_t* component = nullptr;
		if (wasmtime_error_t* compileError = wasmtime_component_new(engine, bytes.data(),
			bytes.size(), &component); compileError != nullptr) {
			error = "invalid Wasm Component Model binary: " + WasmtimeErrorMessage(compileError);
			return false;
		}
		if (!ValidateComponentBinaryNesting(bytes, config.maxComponentNesting, error)) {
			wasmtime_component_delete(component);
			return false;
		}
		auto* type = wasmtime_component_type(component);
		if (type == nullptr) {
			wasmtime_component_delete(component);
			error = "Wasm Component Model type reflection failed";
			return false;
		}
		const std::size_t importCount = wasmtime_component_type_import_count(type, engine);
		if (importCount > config.maxImports) {
			wasmtime_component_type_delete(type);
			wasmtime_component_delete(component);
			error = "Wasm Component Model import count exceeds configured maximum";
			return false;
		}
		const std::size_t exportCount = wasmtime_component_type_export_count(type, engine);
		if (exportCount > config.maxExports) {
			wasmtime_component_type_delete(type);
			wasmtime_component_delete(component);
			error = "Wasm Component Model export count exceeds configured maximum";
			return false;
		}
		if (!ValidateComponentTypeNesting(type, engine, config.maxComponentNesting, 0, error)) {
			wasmtime_component_type_delete(type);
			wasmtime_component_delete(component);
			return false;
		}
		for (std::size_t index = 0; index < importCount; ++index) {
			const char* name = nullptr;
			std::size_t nameLength = 0;
			wasmtime_component_item_t item{};
			if (!wasmtime_component_type_import_nth(type, engine, index, &name, &nameLength,
				&item)) {
				wasmtime_component_type_delete(type);
				wasmtime_component_delete(component);
				error = "Wasm Component Model import reflection failed";
				return false;
			}
			const std::string importName(name, nameLength);
			if (!IsAllowedComponentImport(importName, config)) {
				wasmtime_component_item_delete(&item);
				wasmtime_component_type_delete(type);
				wasmtime_component_delete(component);
				error = "Wasm Component Model import is outside the Spring capability boundary: " +
					importName;
				return false;
			}
			if (importName.starts_with(RECOIL_WASM_INTERFACE_PREFIX)) {
				const std::string moduleName = ComponentImportModule(importName);
				if (!IsKnownComponentModule(moduleName)) {
					wasmtime_component_item_delete(&item);
					wasmtime_component_type_delete(type);
					wasmtime_component_delete(component);
					error = "Wasm Component Model import names an unknown Spring module: " +
						moduleName;
					return false;
				}
				if (!WasmEnvironmentMatrix::HasModule(environment, moduleName)) {
					wasmtime_component_item_delete(&item);
					wasmtime_component_type_delete(type);
					wasmtime_component_delete(component);
					error = "Wasm Component Model import is unavailable in environment " +
						std::string(WasmEnvironmentMatrix::Name(environment)) + ": " + moduleName;
					return false;
				}
				if (!ValidateComponentInterface(item, moduleName, environment, config, engine, error)) {
					wasmtime_component_item_delete(&item);
					wasmtime_component_type_delete(type);
					wasmtime_component_delete(component);
					return false;
				}
			}
			wasmtime_component_item_delete(&item);
			imports.push_back(importName);
		}
		wasmtime_component_type_delete(type);
		wasmtime_component_delete(component);
		return true;
	}
#endif
}

struct WasmRuntime::BackendState {
#if defined(RECOIL_WASMTIME_AVAILABLE)
	wasm_engine_t* engine = nullptr;

	~BackendState()
	{
		if (engine != nullptr)
			wasm_engine_delete(engine);
	}
#endif
};

WasmRuntime::WasmRuntime(WasmRuntimeConfig config)
	: config(std::move(config))
{
#if defined(RECOIL_WASMTIME_AVAILABLE)
	wasm_config_t* wasmtimeConfig = wasm_config_new();
	if (wasmtimeConfig != nullptr) {
		wasmtime_config_consume_fuel_set(wasmtimeConfig, config.instructionFuel != 0);
		wasmtime_config_wasm_component_model_set(wasmtimeConfig, true);
		// Component canonical lowering can execute guest realloc while a host
		// import is active. Keep Wasmtime's native trap handler explicit here:
		// relying on the backend default lets a guest `unreachable` escape as
		// SIGILL on some C-API builds instead of becoming a call error.
		wasmtime_config_signals_based_traps_set(wasmtimeConfig, true);
		wasmtime_config_wasm_threads_set(wasmtimeConfig, config.allowThreads);
		wasmtime_config_shared_memory_set(wasmtimeConfig, config.allowThreads);
			wasmtime_config_wasm_relaxed_simd_set(wasmtimeConfig, config.allowRelaxedSimd);
			wasmtime_config_wasm_relaxed_simd_deterministic_set(wasmtimeConfig, true);
			wasmtime_config_cranelift_nan_canonicalization_set(wasmtimeConfig, true);
			wasmtime_config_wasm_multi_value_set(wasmtimeConfig, true);
		wasmtime_config_wasm_bulk_memory_set(wasmtimeConfig, true);

		backendState = std::make_unique<BackendState>();
		backendState->engine = wasm_engine_new_with_config(wasmtimeConfig);
		if (backendState->engine != nullptr)
			backend = WasmRuntimeBackend::WasmtimeComponentModel;
		else
			backendState.reset();
	}
#else
	// The policy-only fallback is retained for source builds that deliberately
	// disable external runtime dependencies. It never executes guest code.
	backend = WasmRuntimeBackend::Unavailable;
#endif
}

WasmRuntime::~WasmRuntime() = default;

void* WasmRuntime::BackendEngine() const
{
#if defined(RECOIL_WASMTIME_AVAILABLE)
	return backendState != nullptr ? backendState->engine : nullptr;
#else
	return nullptr;
#endif
}

WasmValidationResult WasmRuntime::ValidateModule(const std::vector<std::uint8_t>& bytes,
	WasmEnvironment environment, std::string_view requestedWorld,
	std::string_view requestedInterfaceVersion) const
{
	WasmValidationResult result;
	result.identity.byteSize = bytes.size();
	result.identity.sha512 = HashModule(bytes);

	if (bytes.size() > config.maxModuleBytes) {
		result.error = "Wasm module exceeds configured byte limit";
		return result;
	}
	if (!WasmEnvironmentMatrix::IsRuntimeEnabled(environment)) {
		result.error = "requested Wasm execution environment is disabled";
		return result;
	}
	if (requestedWorld != WasmEnvironmentMatrix::Name(environment)) {
		result.error = "module world does not match its execution environment";
		return result;
	}
	if (requestedInterfaceVersion != RECOIL_WASM_INTERFACE_VERSION_NUMBER) {
		result.error = "unsupported Wasm interface version: " +
			std::string(requestedInterfaceVersion) +
			" (host supports " + std::string(RECOIL_WASM_INTERFACE_VERSION_NUMBER) + ")";
		return result;
	}
	if (bytes.size() < 8 || bytes[0] != 0x00 || bytes[1] != 'a' || bytes[2] != 's' || bytes[3] != 'm') {
		result.error = "invalid Wasm magic";
		return result;
	}
	if (!(bytes[4] == 0x01 && bytes[5] == 0x00 && bytes[6] == 0x00 && bytes[7] == 0x00) &&
		!HasComponentVersion(bytes)) {
		result.error = "unsupported Wasm core/component version";
		return result;
	}
	if (HasComponentVersion(bytes)) {
#if defined(RECOIL_WASMTIME_AVAILABLE)
		if (backendState == nullptr || backendState->engine == nullptr) {
			result.error = "Wasmtime Component Model backend is unavailable";
			return result;
		}
		if (!ValidateComponentImports(bytes, config, environment, backendState->engine, result.imports,
			result.error))
			return result;
#else
		result.error = "Wasmtime Component Model backend is unavailable";
		return result;
#endif
		result.valid = true;
		return result;
	}

	Reader moduleReader{bytes, 8};
	std::uint32_t sectionCount = 0;
	while (moduleReader.offset < bytes.size()) {
		if (++sectionCount > config.maxSections) {
			result.error = "Wasm module has too many sections";
			return result;
		}
		std::uint8_t sectionID = 0;
		std::uint64_t sectionSize = 0;
		if (!moduleReader.ReadByte(sectionID) || !moduleReader.ReadLeb(sectionSize) ||
			sectionSize > bytes.size() - moduleReader.offset) {
			result.error = "truncated Wasm section";
			return result;
		}
		const std::size_t sectionEnd = moduleReader.offset + static_cast<std::size_t>(sectionSize);
		Reader section{bytes, moduleReader.offset};
		if (sectionID == 0) {
			std::string customName;
			if (!section.ReadString(customName)) {
				result.error = "invalid Wasm custom section";
				return result;
			}
		} else if (sectionID == 2) {
			std::uint64_t importCount = 0;
			if (!section.ReadLeb(importCount) || importCount > config.maxImports) {
				result.error = "Wasm import count exceeds configured maximum";
				return result;
			}
			for (std::uint64_t index = 0; index < importCount; ++index) {
				std::string moduleName;
				std::string fieldName;
				std::uint8_t kind = 0;
				if (!section.ReadString(moduleName) || !section.ReadString(fieldName) ||
					!section.ReadByte(kind)) {
					result.error = "truncated Wasm import";
					return result;
				}
				if (IsDeniedImport(moduleName, config)) {
					result.error = "Wasm import is outside the Spring capability boundary: " + moduleName;
					return result;
				}
				if (!IsAllowedCoreImport(moduleName, fieldName, kind)) {
					result.error = "unknown or unsupported Wasm core import: " + moduleName + "." +
						fieldName;
					return result;
				}
				result.imports.push_back(moduleName + "." + fieldName);
				if (!SkipImportDescription(section, kind, config, result.error))
					return result;
			}
		} else if (sectionID == 4) {
			std::uint64_t tableCount = 0;
			if (!section.ReadLeb(tableCount)) {
				result.error = "truncated Wasm table section";
				return result;
			}
			for (std::uint64_t index = 0; index < tableCount; ++index) {
				std::uint8_t elementType = 0;
				if (!section.ReadByte(elementType) ||
					!SkipLimits(section, config, false, result.error))
					return result;
			}
		} else if (sectionID == 5) {
			std::uint64_t memoryCount = 0;
			if (!section.ReadLeb(memoryCount) || memoryCount > config.maxMemoryPages || memoryCount > 1) {
				result.error = "Wasm memory count exceeds configured policy";
				return result;
			}
			for (std::uint64_t index = 0; index < memoryCount; ++index) {
				if (!SkipLimits(section, config, true, result.error))
					return result;
			}
		} else if (sectionID == 7) {
			std::uint64_t exportCount = 0;
			if (!section.ReadLeb(exportCount) || exportCount > config.maxExports) {
				result.error = "Wasm export count exceeds configured maximum";
				return result;
			}
		}
		if (section.offset > sectionEnd) {
			result.error = "Wasm section parser overflow";
			return result;
		}
		moduleReader.offset = sectionEnd;
	}

	result.valid = true;
	return result;
}

bool WasmRuntime::CanDeserializeAot(std::string_view moduleHash,
	std::string_view runtimeConfigHash) const
{
	// The configuration flag is reserved for the future authenticated cache
	// implementation.  Accepting a hash pair alone would still permit an
	// unauthenticated native-code artifact to enter the process.
	(void)moduleHash;
	(void)runtimeConfigHash;
	return false;
}

std::string WasmRuntime::ConfigurationIdentity() const
{
	return "wasmtime=" + std::string(RECOIL_WASMTIME_VERSION) +
		";component-model=1" +
		";module-bytes=" + std::to_string(config.maxModuleBytes) +
		";memory-pages=" + std::to_string(config.maxMemoryPages) +
		";table-elements=" + std::to_string(config.maxTableElements) +
		";resources=" + std::to_string(config.maxResources) +
		";imports=" + std::to_string(config.maxImports) +
		";exports=" + std::to_string(config.maxExports) +
		";sections=" + std::to_string(config.maxSections) +
		";component-nesting=" + std::to_string(config.maxComponentNesting) +
		";value-nodes=" + std::to_string(config.maxValueNodes) +
		";fuel=" + std::to_string(config.instructionFuel) +
		";host-work=" + std::to_string(config.hostWorkLimit) +
		";result-bytes=" + std::to_string(config.resultBytesLimit) +
		";threads=" + std::to_string(config.allowThreads ? 1 : 0) +
		";relaxed-simd=" + std::to_string(config.allowRelaxedSimd ? 1 : 0) +
		";nan-canonicalization=1" +
		";wasi=" + std::to_string(config.allowWasi ? 1 : 0) +
		";aot=" + std::to_string(config.allowAotDeserialization ? 1 : 0)
#if defined(UNIT_TEST)
		+ ";test-component-functions=" + std::to_string(
			config.allowUnregisteredComponentFunctionsForTesting ? 1 : 0)
#endif
		;
}
