/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreValidation.h"

#include <algorithm>
#include <array>
#include <cstdint>
#include <span>
#include <string>
#include <utility>
#include <vector>

#include "System/Sync/SHA512.hpp"
#include "WasmCoreRegistry.h"

namespace recoil::wasm::core {
namespace {

struct Reader {
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

	bool ReadString(std::string& value)
	{
		std::uint64_t size = 0;
		if (!ReadLeb(size) || size > bytes.size() - offset)
			return false;
		value.assign(reinterpret_cast<const char*>(bytes.data() + offset),
			static_cast<std::size_t>(size));
		offset += static_cast<std::size_t>(size);
		return true;
	}
};

struct FunctionType {
	std::vector<std::uint8_t> params;
	std::vector<std::uint8_t> results;
};

std::string HashModule(const std::vector<std::uint8_t>& bytes)
{
	sha512::raw_digest digest{};
	sha512::calc_digest(bytes, digest);
	return sha512::dump_digest(digest);
}

std::string_view ValueTypeName(std::uint8_t type)
{
	switch (type) {
		case 0x7f: return "i32";
		case 0x7e: return "i64";
		case 0x7d: return "f32";
		case 0x7c: return "f64";
		default: return {};
	}
}

bool ReadNumericTypeVector(Reader& reader, std::vector<std::uint8_t>& output,
	std::string& error)
{
	std::uint64_t count = 0;
	if (!reader.ReadLeb(count) || count > 64) {
		error = "Core Wasm function type has an invalid value count";
		return false;
	}
	output.clear();
	output.reserve(static_cast<std::size_t>(count));
	for (std::uint64_t index = 0; index < count; ++index) {
		std::uint8_t type = 0;
		if (!reader.ReadByte(type)) {
			error = "truncated Core Wasm function value type";
			return false;
		}
		if (ValueTypeName(type).empty()) {
			error = "Spring Core ABI functions may use only i32/i64/f32/f64";
			return false;
		}
		output.push_back(type);
	}
	return true;
}

bool ValidateTypeSection(Reader& section, std::vector<FunctionType>& types,
	std::string& error)
{
	std::uint64_t count = 0;
	if (!section.ReadLeb(count) || count > 65536) {
		error = "Core Wasm type count exceeds supported maximum";
		return false;
	}
	types.clear();
	types.reserve(static_cast<std::size_t>(count));
	for (std::uint64_t index = 0; index < count; ++index) {
		std::uint8_t form = 0;
		if (!section.ReadByte(form) || form != 0x60) {
			error = "Spring Core ABI profile accepts only plain function types";
			return false;
		}
		FunctionType type;
		if (!ReadNumericTypeVector(section, type.params, error) ||
			!ReadNumericTypeVector(section, type.results, error))
			return false;
		types.push_back(std::move(type));
	}
	return section.offset == section.bytes.size();
}

std::string SignatureString(const FunctionType& type)
{
	std::string result;
	for (std::size_t index = 0; index < type.params.size(); ++index) {
		if (index != 0)
			result.push_back(',');
		result += ValueTypeName(type.params[index]);
	}
	result += "->";
	for (std::size_t index = 0; index < type.results.size(); ++index) {
		if (index != 0)
			result.push_back(',');
		result += ValueTypeName(type.results[index]);
	}
	return result;
}

bool ReadLimits(Reader& reader, std::uint64_t configuredMaximum, bool memory,
	bool requireFixed, const WasmRuntimeConfig& config, std::string& error)
{
	std::uint64_t flags = 0;
	std::uint64_t minimum = 0;
	std::uint64_t maximum = 0;
	if (!reader.ReadLeb(flags) || !reader.ReadLeb(minimum)) {
		error = "truncated Core Wasm limits";
		return false;
	}

	const std::uint64_t allowedFlags = memory ? 0x03u : 0x01u;
	if ((flags & ~allowedFlags) != 0) {
		error = memory ? "Core Wasm memory64/unsupported memory flags are not allowed" :
			"unsupported Core Wasm table limits flags";
		return false;
	}
	const bool hasMaximum = (flags & 0x01u) != 0;
	const bool shared = memory && (flags & 0x02u) != 0;
	if (shared && !config.allowThreads) {
		error = "shared Core Wasm memory/threads are not allowed";
		return false;
	}
	if (hasMaximum) {
		if (!reader.ReadLeb(maximum)) {
			error = "truncated Core Wasm maximum";
			return false;
		}
		if (maximum < minimum) {
			error = "Core Wasm maximum is smaller than its minimum";
			return false;
		}
	}
	if (minimum > configuredMaximum || (hasMaximum && maximum > configuredMaximum)) {
		error = memory ? "Core Wasm memory exceeds configured maximum" :
			"Core Wasm table exceeds configured maximum";
		return false;
	}
	if (requireFixed && (!hasMaximum || minimum != maximum)) {
		error = memory ?
			"synced Core Wasm memory must declare max == min (non-growable)" :
			"synced Core Wasm table must declare max == min (non-growable)";
		return false;
	}
	return true;
}

bool ValidateImportSection(Reader& section, WasmEnvironment environment,
	const WasmRuntimeConfig& config, const std::vector<FunctionType>& types,
	std::vector<std::string>& imports, std::string& error)
{
	std::uint64_t count = 0;
	if (!section.ReadLeb(count) || count > config.maxImports) {
		error = "Core Wasm import count exceeds configured maximum";
		return false;
	}
	for (std::uint64_t index = 0; index < count; ++index) {
		std::string module;
		std::string name;
		std::uint8_t kind = 0;
		if (!section.ReadString(module) || !section.ReadString(name) || !section.ReadByte(kind)) {
			error = "truncated Core Wasm import";
			return false;
		}
		const ImportLookup descriptor = LookupImport(module, name);
		const std::uint32_t environmentBit =
			1u << static_cast<std::uint32_t>(environment);
		if (kind != 0 || !descriptor.found ||
			(descriptor.environmentMask & environmentBit) == 0) {
			error = "unknown or unavailable Core Wasm import: " + module + "." + name;
			return false;
		}
		std::uint64_t typeIndex = 0;
		if (!section.ReadLeb(typeIndex) || typeIndex >= types.size()) {
			error = "Core Wasm import references an invalid function type: " +
				module + "." + name;
			return false;
		}
		const std::string actualSignature = SignatureString(types[static_cast<std::size_t>(typeIndex)]);
		if (actualSignature != descriptor.signature) {
			error = "Core Wasm import signature mismatch for " + module + "." + name +
				": expected " + std::string(descriptor.signature) + ", got " + actualSignature;
			return false;
		}
		imports.push_back(module + "." + name);
	}
	return section.offset == section.bytes.size();
}

bool ValidateTableSection(Reader& section, const WasmRuntimeConfig& config,
	bool synced, std::string& error)
{
	std::uint64_t count = 0;
	if (!section.ReadLeb(count) || count > 1) {
		error = "Core Wasm supports at most one table";
		return false;
	}
	for (std::uint64_t index = 0; index < count; ++index) {
		std::uint8_t elementType = 0;
		if (!section.ReadByte(elementType)) {
			error = "truncated Core Wasm table type";
			return false;
		}
		if (elementType != 0x70 && elementType != 0x6f) {
			error = "unsupported Core Wasm table element type";
			return false;
		}
		if (!ReadLimits(section, config.maxTableElements, false, synced, config, error))
			return false;
	}
	return section.offset == section.bytes.size();
}

bool ValidateMemorySection(Reader& section, const WasmRuntimeConfig& config,
	bool synced, bool& hasMemory, std::string& error)
{
	std::uint64_t count = 0;
	if (!section.ReadLeb(count) || count > 1) {
		error = "Core Wasm supports exactly one linear memory";
		return false;
	}
	hasMemory = count == 1;
	for (std::uint64_t index = 0; index < count; ++index) {
		if (!ReadLimits(section, config.maxMemoryPages, true, synced, config, error))
			return false;
	}
	return section.offset == section.bytes.size();
}

bool ValidateExportSection(Reader& section, const WasmRuntimeConfig& config,
	std::vector<std::string>& exports, bool& exportsMemory, std::string& error)
{
	std::uint64_t count = 0;
	if (!section.ReadLeb(count) || count > config.maxExports) {
		error = "Core Wasm export count exceeds configured maximum";
		return false;
	}
	for (std::uint64_t index = 0; index < count; ++index) {
		std::string name;
		std::uint8_t kind = 0;
		std::uint64_t itemIndex = 0;
		if (!section.ReadString(name) || !section.ReadByte(kind) || !section.ReadLeb(itemIndex)) {
			error = "truncated Core Wasm export";
			return false;
		}
		if (name == "memory") {
			if (kind != 2) {
				error = "Core Wasm export named memory is not a memory";
				return false;
			}
			exportsMemory = true;
			continue;
		}
		if (kind == 0)
			exports.push_back(name);
	}
	return section.offset == section.bytes.size();
}

bool ValidateCodeSection(Reader& section, const WasmRuntimeConfig& config,
	std::string& error)
{
	std::uint64_t count = 0;
	if (!section.ReadLeb(count) || count > 65536) {
		error = "Core Wasm function body count exceeds supported maximum";
		return false;
	}
	for (std::uint64_t index = 0; index < count; ++index) {
		std::uint64_t bodySize = 0;
		if (!section.ReadLeb(bodySize) || bodySize > section.bytes.size() - section.offset) {
			error = "truncated Core Wasm function body";
			return false;
		}
		if (bodySize > config.maxFunctionBodyBytes) {
			error = "Core Wasm function body exceeds configured maximum";
			return false;
		}
		section.offset += static_cast<std::size_t>(bodySize);
	}
	return section.offset == section.bytes.size();
}

} // namespace

WasmValidationResult ValidateModule(const std::vector<std::uint8_t>& bytes,
	WasmEnvironment environment, std::string_view interfaceVersion,
	const WasmRuntimeConfig& config)
{
	WasmValidationResult result;
	if (interfaceVersion != RECOIL_WASM_INTERFACE_VERSION_NUMBER) {
		result.error = "unsupported Spring Core ABI interface version";
		return result;
	}
	if (bytes.size() > config.maxModuleBytes) {
		result.error = "Core Wasm module exceeds configured byte limit";
		return result;
	}
	if (bytes.size() < 8 || !std::equal(bytes.begin(), bytes.begin() + 4,
			std::array<std::uint8_t, 4>{0x00, 0x61, 0x73, 0x6d}.begin())) {
		result.error = "not a WebAssembly Core module";
		return result;
	}
	if (!std::equal(bytes.begin() + 4, bytes.begin() + 8,
			std::array<std::uint8_t, 4>{0x01, 0x00, 0x00, 0x00}.begin())) {
		result.error = "unsupported WebAssembly Core version";
		return result;
	}

	std::vector<FunctionType> types;
	std::vector<std::string> imports;
	std::vector<std::string> exports;
	bool hasMemory = false;
	bool exportsMemory = false;
	Reader module{bytes};
	module.offset = 8;
	std::uint32_t sectionCount = 0;
	while (module.offset < module.bytes.size()) {
		if (++sectionCount > config.maxSections) {
			result.error = "Core Wasm section count exceeds configured maximum";
			return result;
		}
		std::uint8_t sectionId = 0;
		std::uint64_t sectionSize = 0;
		if (!module.ReadByte(sectionId) || !module.ReadLeb(sectionSize) ||
			sectionSize > module.bytes.size() - module.offset) {
			result.error = "truncated Core Wasm section";
			return result;
		}
		Reader section{module.bytes.subspan(module.offset, static_cast<std::size_t>(sectionSize))};
		module.offset += static_cast<std::size_t>(sectionSize);
		bool valid = true;
		switch (sectionId) {
			case 0: section.offset = section.bytes.size(); break;
			case 1: valid = ValidateTypeSection(section, types, result.error); break;
			case 2: valid = ValidateImportSection(section, environment, config, types, imports, result.error); break;
			case 3: section.offset = section.bytes.size(); break;
			case 4: valid = ValidateTableSection(section, config, environment == WasmEnvironment::RulesSynced || environment == WasmEnvironment::GaiaSynced, result.error); break;
			case 5: valid = ValidateMemorySection(section, config, environment == WasmEnvironment::RulesSynced || environment == WasmEnvironment::GaiaSynced, hasMemory, result.error); break;
			case 6: section.offset = section.bytes.size(); break;
			case 7: valid = ValidateExportSection(section, config, exports, exportsMemory, result.error); break;
			case 8: section.offset = section.bytes.size(); break;
			case 9: section.offset = section.bytes.size(); break;
			case 10: valid = ValidateCodeSection(section, config, result.error); break;
			case 11: section.offset = section.bytes.size(); break;
			case 12: section.offset = section.bytes.size(); break;
			default:
				result.error = "unsupported Core Wasm section";
				valid = false;
				break;
		}
		if (!valid || section.offset != section.bytes.size()) {
			if (result.error.empty())
				result.error = "Core Wasm section was not fully consumed";
			return result;
		}
	}

	if (!hasMemory || !exportsMemory) {
		result.error = "Core Wasm module must define and export linear memory as memory";
		return result;
	}
	result.valid = true;
	result.identity.sha512 = HashModule(bytes);
	result.identity.byteSize = bytes.size();
	result.imports = std::move(imports);
	return result;
}

} // namespace recoil::wasm::core