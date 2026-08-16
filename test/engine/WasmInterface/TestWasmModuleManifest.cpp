/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include <catch_amalgamated.hpp>

#include <string>

#include "WasmInterface/WasmModuleManifest.h"

TEST_CASE("Wasm module manifests parse deterministic declarations")
{
	const std::string text = R"(
# game-side declaration
module(game-rules, LuaRules/wasm/game.wasm, rules-synced, 2)
module(map-gaia, LuaGaia/wasm/map.wasm, gaia-unsynced, 0)
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
	CHECK_FALSE(error.empty());
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
