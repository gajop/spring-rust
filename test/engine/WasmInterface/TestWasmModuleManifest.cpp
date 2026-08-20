/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include <catch_amalgamated.hpp>

#include <cstdint>
#include <string>
#include <string_view>
#include <vector>

#include "WasmInterface/WasmCoreValidation.h"
#include "WasmInterface/WasmModuleManifest.h"

// The WasmInterface unit-test target predates the production fast-Core path and
// does not yet compile WasmCoreValidation.cpp as a separate source. Include the
// implementation here so these policy tests exercise that exact validator.
// Normalize this into test/CMakeLists.txt when the test target is next edited
// from a normal checkout.
#include "WasmInterface/WasmCoreValidation.cpp"

namespace {

void AppendCoreLeb(std::vector<std::uint8_t>& bytes, std::uint64_t value)
{
	do {
		std::uint8_t byte = static_cast<std::uint8_t>(value & 0x7f);
		value >>= 7;
		if (value != 0)
			byte |= 0x80;
		bytes.push_back(byte);
	} while (value != 0);
}

void AppendCoreString(std::vector<std::uint8_t>& bytes, std::string_view value)
{
	AppendCoreLeb(bytes, value.size());
	bytes.insert(bytes.end(), value.begin(), value.end());
}

void AddCoreSection(std::vector<std::uint8_t>& module, std::uint8_t id,
	const std::vector<std::uint8_t>& payload)
{
	module.push_back(id);
	AppendCoreLeb(module, payload.size());
	module.insert(module.end(), payload.begin(), payload.end());
}

std::vector<std::uint8_t> CoreHeader()
{
	return {0x00, 'a', 's', 'm', 0x01, 0x00, 0x00, 0x00};
}

void AddCoreFunctionType(std::vector<std::uint8_t>& module,
	const std::vector<std::uint8_t>& params,
	const std::vector<std::uint8_t>& results)
{
	std::vector<std::uint8_t> payload;
	AppendCoreLeb(payload, 1);
	payload.push_back(0x60);
	AppendCoreLeb(payload, params.size());
	payload.insert(payload.end(), params.begin(), params.end());
	AppendCoreLeb(payload, results.size());
	payload.insert(payload.end(), results.begin(), results.end());
	AddCoreSection(module, 1, payload);
}

void AddCoreFunctionImport(std::vector<std::uint8_t>& module,
	std::string_view importModule, std::string_view name)
{
	std::vector<std::uint8_t> payload;
	AppendCoreLeb(payload, 1);
	AppendCoreString(payload, importModule);
	AppendCoreString(payload, name);
	payload.push_back(0); // function
	AppendCoreLeb(payload, 0); // type index
	AddCoreSection(module, 2, payload);
}

void AddCoreMemory(std::vector<std::uint8_t>& module, bool fixed)
{
	std::vector<std::uint8_t> payload;
	AppendCoreLeb(payload, 1);
	if (fixed) {
		AppendCoreLeb(payload, 1); // has maximum
		AppendCoreLeb(payload, 1); // min pages
		AppendCoreLeb(payload, 1); // max pages
	} else {
		AppendCoreLeb(payload, 0); // no maximum
		AppendCoreLeb(payload, 1); // min pages
	}
	AddCoreSection(module, 5, payload);
}

void AddCoreMemoryExport(std::vector<std::uint8_t>& module)
{
	std::vector<std::uint8_t> payload;
	AppendCoreLeb(payload, 1);
	AppendCoreString(payload, "memory");
	payload.push_back(2); // memory
	AppendCoreLeb(payload, 0);
	AddCoreSection(module, 7, payload);
}

std::vector<std::uint8_t> MinimalCoreModule(bool fixedMemory)
{
	auto module = CoreHeader();
	AddCoreMemory(module, fixedMemory);
	AddCoreMemoryExport(module);
	return module;
}

} // namespace

TEST_CASE("Wasm module manifests parse deterministic declarations")
{
	const std::string text = R"(
# game-side declaration
	module(game-rules, LuaRules/wasm/game.wasm, rules-synced, 2)
	module(map-gaia, LuaGaia/wasm/map.wasm, gaia-unsynced, 0, 1.0.0)
      )";
	std::vector<WasmModuleDeclaration> declarations;
	std::string error;
	REQUIRE(WasmModuleManifest::Parse(text, declarations, error));
	REQUIRE(declarations.size() == 2);
	CHECK(declarations[0].name == "game-rules");
	CHECK(declarations[0].path == "LuaRules/wasm/game.wasm");
	CHECK(declarations[0].environment == WasmEnvironment::RulesSynced);
	CHECK(declarations[0].order == 2);
	CHECK(declarations[1].environment == WasmEnvironment::GaiaUnsynced);
	CHECK(declarations[0].interfaceVersion == "1.0.0");
	CHECK(declarations[1].interfaceVersion == "1.0.0");
}

TEST_CASE("Wasm module manifests reject malformed and duplicate declarations")
{
	std::vector<WasmModuleDeclaration> declarations;
	std::string error;
	CHECK_FALSE(WasmModuleManifest::Parse(
		"module(a, a.wasm, not-an-environment, 0)\n", declarations, error));
	CHECK_FALSE(WasmModuleManifest::Parse(
		"module(a, a.wasm, rules-synced, 0)\nmodule(a, b.wasm, rules-synced, 1)\n",
		declarations, error));
	CHECK_FALSE(WasmModuleManifest::Parse(
		"module(a, ../outside.wasm, rules-synced, 0)\n", declarations, error));
	CHECK_FALSE(WasmModuleManifest::Parse(
		"module(a|unsafe, a.wasm, rules-synced, 0)\n", declarations, error));
	CHECK_FALSE(WasmModuleManifest::Parse(
		"module(a, ./inside.wasm, rules-synced, 0)\n", declarations, error));
	CHECK_FALSE(WasmModuleManifest::Parse(
		"module(a, a.wasm, rules-synced, 0, 1.0)\n", declarations, error));
	CHECK_FALSE(WasmModuleManifest::Parse(
		"module(a, a.wasm, rules-synced, 0, 1.0.0-alpha)\n", declarations, error));
	CHECK_FALSE(error.empty());
}

TEST_CASE("Wasm module manifests reject every host-path escape spelling")
{
	const std::vector<std::string> paths = {
		"/absolute.wasm",
		"\\absolute.wasm",
		"C:/absolute.wasm",
		"modules/../outside.wasm",
		"modules//inside.wasm",
		"modules/./inside.wasm",
		"modules\\inside.wasm",
	};
	for (const auto& path : paths) {
		std::vector<WasmModuleDeclaration> declarations;
		std::string error;
		CHECK_FALSE(WasmModuleManifest::Parse(
			"module(a, " + path + ", rules-synced, 0)\n", declarations, error));
		CHECK(error.find("outside the content archive") != std::string::npos);
	}
}

TEST_CASE("Wasm module manifests bound declaration count")
{
	std::string text;
	for (unsigned index = 0; index < 257; ++index)
		text += "module(module-" + std::to_string(index) +
			", modules/module.wasm, rules-synced, 0)\n";

	std::vector<WasmModuleDeclaration> declarations;
	std::string error;
	CHECK_FALSE(WasmModuleManifest::Parse(text, declarations, error));
	CHECK(error.find("more than 256") != std::string::npos);
}

TEST_CASE("Production Core ABI rejects ambient WASI even when legacy WASI is enabled")
{
	auto module = CoreHeader();
	AddCoreFunctionType(module, {}, {});
	AddCoreFunctionImport(module, "wasi_snapshot_preview1", "fd_write");
	AddCoreMemory(module, true);
	AddCoreMemoryExport(module);

	WasmRuntimeConfig config;
	config.allowWasi = true;
	const auto result = recoil::wasm::core::ValidateModule(module,
		WasmEnvironment::RulesSynced, RECOIL_WASM_INTERFACE_VERSION_NUMBER, config);
	CHECK_FALSE(result.valid);
	CHECK(result.error.find("unknown or unavailable Core Wasm import") != std::string::npos);
}

TEST_CASE("Production Core ABI requires fixed synced memory but permits unsynced growth")
{
	const auto growable = MinimalCoreModule(false);
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
	AddCoreFunctionType(module, {0x7f}, {0x7f});
	AddCoreFunctionImport(module, "spring:units-info", "get-unit-def-id");
	AddCoreMemory(module, true);
	AddCoreMemoryExport(module);

	WasmRuntimeConfig config;
	const auto result = recoil::wasm::core::ValidateModule(module,
		WasmEnvironment::RulesSynced, RECOIL_WASM_INTERFACE_VERSION_NUMBER, config);
	CHECK_FALSE(result.valid);
	CHECK(result.error.find("Core Wasm import signature mismatch") != std::string::npos);
}

TEST_CASE("Production Core ABI rejects an incompatible interface version")
{
	const auto module = MinimalCoreModule(true);
	WasmRuntimeConfig config;
	const auto result = recoil::wasm::core::ValidateModule(module,
		WasmEnvironment::RulesSynced, "999.0.0", config);
	CHECK_FALSE(result.valid);
	CHECK(result.error.find("interface version") != std::string::npos);
}

TEST_CASE("Production Core ABI records validated module identity")
{
	const auto module = MinimalCoreModule(true);
	WasmRuntimeConfig config;
	const auto result = recoil::wasm::core::ValidateModule(module,
		WasmEnvironment::RulesSynced, RECOIL_WASM_INTERFACE_VERSION_NUMBER, config);
	REQUIRE(result.valid);
	CHECK(result.identity.byteSize == module.size());
	CHECK_FALSE(result.identity.sha512.empty());
	CHECK(result.imports.empty());
}
