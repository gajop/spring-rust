/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreValidation.h"

#include <algorithm>
#include <cstdint>
#include <span>
#include <string>
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

	// Bit 0 means a maximum is present. Bit 1 is shared memory. Bit 2 is
	// memory64 and is deliberately rejected by the wasm32 Spring ABI.
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
		const ImportDescriptor* descriptor = FindImport(module, name);
		if (kind != 0 || descriptor == nullptr || !ImportAllowed(module, name, environment)) {
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
		if (actualSignature != descriptor->signature) {
			error = "Core Wasm import signature mismatch for " + module + "." + name +
				": expected " + std::string(descriptor->signature) + ", got " + actualSignature;
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
		error = "Core Wasm supports exactly one linear-memory address space";
		return false;
	}
	if (count == 1) {
		hasMemory = true;
		if (!ReadLimits(section, config.maxMemoryPages, true, synced, config, error))
			return false;
	}
	return section.offset == section.bytes.size();
}

bool ValidateExportSection(Reader& section, const WasmRuntimeConfig& config,
	bool& exportsMemory, std::string& error)
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
		if (name == "memory" && kind == 2)
			exportsMemory = true;
	}
	return section.offset == section.bytes.size();
}

} // namespace

WasmValidationResult ValidateModule(const std::vector<std::uint8_t>& bytes,
	WasmEnvironment environment, std::string_view interfaceVersion,
	const WasmRuntimeConfig& config)
{
	WasmValidationResult result;
	result.identity.byteSize = bytes.size();
	result.identity.sha512 = HashModule(bytes);

	if (bytes.size() > config.maxModuleBytes) {
		result.error = "Core Wasm module exceeds configured byte limit";
		return result;
	}
	if (!WasmEnvironmentMatrix::IsRuntimeEnabled(environment)) {
		result.error = "requested Core Wasm execution environment is disabled";
		return result;
	}
	if (interfaceVersion != RECOIL_WASM_INTERFACE_VERSION_NUMBER) {
		result.error = "unsupported Core Wasm interface version: " +
			std::string(interfaceVersion);
		return result;
	}
	if (bytes.size() < 8 || bytes[0] != 0x00 || bytes[1] != 'a' ||
		bytes[2] != 's' || bytes[3] != 'm' || bytes[4] != 0x01 ||
		bytes[5] != 0x00 || bytes[6] != 0x00 || bytes[7] != 0x00) {
		result.error = "invalid or non-Core Wasm binary";
		return result;
	}

	const bool synced = WasmEnvironmentMatrix::Policy(environment).synced;
	bool hasMemory = false;
	bool exportsMemory = false;
	std::vector<FunctionType> types;
	std::span<const std::uint8_t> module(bytes.data(), bytes.size());
	Reader reader{module.subspan(8)};
	std::uint32_t sectionCount = 0;
	while (reader.offset < reader.bytes.size()) {
		if (++sectionCount > config.maxSections) {
			result.error = "Core Wasm module has too many sections";
			return result;
		}
		std::uint8_t sectionID = 0;
		std::uint64_t sectionSize = 0;
		if (!reader.ReadByte(sectionID) || !reader.ReadLeb(sectionSize) ||
			sectionSize > reader.bytes.size() - reader.offset) {
			result.error = "truncated Core Wasm section";
			return result;
		}
		Reader section{reader.bytes.subspan(reader.offset, static_cast<std::size_t>(sectionSize))};
		reader.offset += static_cast<std::size_t>(sectionSize);

		if (sectionID == 1) {
			if (!ValidateTypeSection(section, types, result.error))
				return result;
		} else if (sectionID == 2) {
			if (!ValidateImportSection(section, environment, config, types,
				result.imports, result.error))
				return result;
		} else if (sectionID == 4) {
			if (!ValidateTableSection(section, config, synced, result.error))
				return result;
		} else if (sectionID == 5) {
			if (!ValidateMemorySection(section, config, synced, hasMemory, result.error))
				return result;
		} else if (sectionID == 7) {
			if (!ValidateExportSection(section, config, exportsMemory, result.error))
				return result;
		}
	}

	// The Core ABI uses caller-owned buffers and therefore requires a directly
	// addressable exported memory even if this particular module currently uses
	// only scalar imports. Keeping this invariant removes a conditional from all
	// generated aggregate bindings.
	if (!hasMemory || !exportsMemory) {
		result.error = "Core Wasm module must define and export linear memory as `memory`";
		return result;
	}

	result.valid = true;
	return result;
}

} // namespace recoil::wasm::core
