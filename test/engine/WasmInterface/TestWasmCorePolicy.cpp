/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include <catch_amalgamated.hpp>

#include <cstdint>
#include <string_view>
#include <vector>

#include "WasmInterface/WasmCoreValidation.h"
#include "WasmInterface/WasmRuntime.h"

namespace {

void AppendLeb(std::vector<std::uint8_t>& bytes, std::uint64_t value)
{
	do {
		std::uint8_t byte = static_cast<std::uint8_t>(value & 0x7f);
		value >>= 7;
		if (value != 0)
			byte |= 0x80;
		bytes.push_back(byte);
	} while (value != 0);
}

void AppendString(std::vector<std::uint8_t>& bytes, std::string_view value)
{
	AppendLeb(bytes, value.size());
	bytes.insert(bytes.end(), value.begin(), value.end());
}

void AddSection(std::vector<std::uint8_t>& module, std::uint8_t id,
	const std::vector<std::uint8_t>& payload)
{
	module.push_back(id);
	AppendLeb(module, payload.size());
	module.insert(module.end(), payload.begin(), payload.end());
}

std::vector<std::uint8_t> CoreHeader()
{
	return {0x00, 'a', 's', 'm', 0x01, 0x00, 0x00, 0x00};
}

void AddFunctionType(std::vector<std::uint8_t>& module,
	const std::vector<std::uint8_t>& params,
	const std::vector<std::uint8_t>& results)
{
	std::vector<std::uint8_t> payload;
	AppendLeb(payload, 1);
	payload.push_back(0x60);
	AppendLeb(payload, params.size());
	payload.insert(payload.end(), params.begin(), params.end());
	AppendLeb(payload, results.size());
	payload.insert(payload.end(), results.begin(), results.end());
	AddSection(module, 1, payload);
}

void AddFunctionImport(std::vector<std::uint8_t>& module, std::string_view importModule,
	std::string_view name)
{
	std::vector<std::uint8_t> payload;
	AppendLeb(payload, 1);
	AppendString(payload, importModule);
	AppendString(payload, name);
	payload.push_back(0); // function
	AppendLeb(payload, 0); // type index
	AddSection(module, 2, payload);
}

void AddMemory(std::vector<std::uint8_t>& module, bool fixed)
{
	std::vector<std::uint8_t> payload;
	AppendLeb(payload, 1);
	if (fixed) {
		AppendLeb(payload, 1); // has maximum
		AppendLeb(payload, 1); // min pages
		AppendLeb(payload, 1); // max pages
	} else {
		AppendLeb(payload, 0); // no maximum
		AppendLeb(payload, 1); // min pages
	}
	AddSection(module, 5, payload);
}

void AddMemoryExport(std::vector<std::uint8_t>& module)
{
	std::vector<std::uint8_t> payload;
	AppendLeb(payload, 1);
	AppendString(payload, "memory");
	payload.push_back(2); // memory
	AppendLeb(payload, 0);
	AddSection(module, 7, payload);
}

std::vector<std::uint8_t> MinimalModule(bool fixedMemory)
{
	auto module = CoreHeader();
	AddMemory(module, fixedMemory);
	AddMemoryExport(module);
	return module;
}

} // namespace

TEST_CASE("Production Core ABI rejects ambient WASI even when legacy WASI is enabled")
{
	auto module = CoreHeader();
	AddFunctionType(module, {}, {});
	AddFunctionImport(module, "wasi_snapshot_preview1", "fd_write");
	AddMemory(module, true);
	AddMemoryExport(module);

	WasmRuntimeConfig config;
	config.allowWasi = true;
	const auto result = recoil::wasm::core::ValidateModule(module,
		WasmEnvironment::RulesSynced, RECOIL_WASM_INTERFACE_VERSION_NUMBER, config);
	CHECK_FALSE(result.valid);
	CHECK(result.error.find("unknown or unavailable Core Wasm import") != std::string::npos);
}

TEST_CASE("Production Core ABI requires fixed synced memory but permits unsynced growth")
{
	const auto growable = MinimalModule(false);
	WasmRuntimeConfig config;

	const auto synced = recoil::wasm::core::ValidateModule(growable,
		WasmEnvironment::RulesSynced, RECOIL_WASM_INTERFACE_VERSION_NUMBER, config);
	CHECK_FALSE(synced.valid);
	CHECK(synced.error.find("synced Core Wasm memory must declare max == min") !=
		std::string::npos);

	const auto unsynced = recoil::wasm::core::ValidateModule(growable,
		WasmEnvironment::RulesUnsynced, RECOIL_WASM_INTERFACE_VERSION_NUMBER, config);
	CHECK(unsynced.valid);
}

TEST_CASE("Production Core ABI validates exact import signatures")
{
	// get-unit-def-id is (i32)->i64. Deliberately declare (i32)->i32.
	auto module = CoreHeader();
	AddFunctionType(module, {0x7f}, {0x7f});
	AddFunctionImport(module, "spring:units-info", "get-unit-def-id");
	AddMemory(module, true);
	AddMemoryExport(module);

	WasmRuntimeConfig config;
	const auto result = recoil::wasm::core::ValidateModule(module,
		WasmEnvironment::RulesSynced, RECOIL_WASM_INTERFACE_VERSION_NUMBER, config);
	CHECK_FALSE(result.valid);
	CHECK(result.error.find("Core Wasm import signature mismatch") != std::string::npos);
}

TEST_CASE("Production Core ABI rejects an incompatible interface version")
{
	const auto module = MinimalModule(true);
	WasmRuntimeConfig config;
	const auto result = recoil::wasm::core::ValidateModule(module,
		WasmEnvironment::RulesSynced, "999.0.0", config);
	CHECK_FALSE(result.valid);
	CHECK(result.error.find("interface version") != std::string::npos);
}

TEST_CASE("Production Core ABI records validated module identity")
{
	const auto module = MinimalModule(true);
	WasmRuntimeConfig config;
	const auto result = recoil::wasm::core::ValidateModule(module,
		WasmEnvironment::RulesSynced, RECOIL_WASM_INTERFACE_VERSION_NUMBER, config);
	REQUIRE(result.valid);
	CHECK(result.identity.byteSize == module.size());
	CHECK_FALSE(result.identity.sha512.empty());
	CHECK(result.imports.empty());
}
