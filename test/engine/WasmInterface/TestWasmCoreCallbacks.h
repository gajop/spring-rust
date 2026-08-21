/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */
#pragma once

#include <catch_amalgamated.hpp>

#include <cstdint>
#include <string>
#include <tuple>
#include <vector>

#include "NativeInterface/NativeInterface.h"
#include "WasmInterface/WasmCoreBindings.h"
#include "WasmInterface/WasmCoreSystemControlBindings.h"
#include "WasmInterface/WasmCoreValidation.h"
#include "WasmInterface/WasmRuntime.h"

namespace wasm_core_callback_test {

inline int seenTeam = -1;
inline int seenUnit = -1;
inline int callbackCount = 0;

inline void CallAsTeam(const CallAsTeamQuery* query, CallAsTeamResult* result)
{
	seenTeam = query->teamID;
	++callbackCount;
	query->callback(query->userData);
	result->error = nullptr;
	result->success = true;
}

inline void GetUnitDefID(const GetUnitDefIDQuery* query, GetUnitDefIDResult* result)
{
	seenUnit = query->unitID;
	result->error = nullptr;
	result->unitDefID = query->unitID + 1000;
}

inline void Leb(std::vector<std::uint8_t>& out, std::uint64_t value)
{
	do {
		std::uint8_t byte = static_cast<std::uint8_t>(value & 0x7f);
		value >>= 7;
		if (value != 0)
			byte |= 0x80;
		out.push_back(byte);
	} while (value != 0);
}

inline void String(std::vector<std::uint8_t>& out, const char* value)
{
	const std::string text(value);
	Leb(out, text.size());
	out.insert(out.end(), text.begin(), text.end());
}

inline void Section(std::vector<std::uint8_t>& module, std::uint8_t id,
	const std::vector<std::uint8_t>& payload)
{
	module.push_back(id);
	Leb(module, payload.size());
	module.insert(module.end(), payload.begin(), payload.end());
}

inline std::vector<std::uint8_t> Module()
{
	std::vector<std::uint8_t> module = {0x00, 'a', 's', 'm', 0x01, 0x00, 0x00, 0x00};

	// type 0: call-as-team (i32,i32,i32)->i64
	// type 1: get-unit-def-id (i32)->i64
	// type 2: callback dispatch (i32,i32)->()
	// type 3: run ()->i64
	std::vector<std::uint8_t> types;
	Leb(types, 4);
	const auto type = [&types](std::initializer_list<std::uint8_t> params,
		std::initializer_list<std::uint8_t> results) {
		types.push_back(0x60);
		Leb(types, params.size());
		types.insert(types.end(), params.begin(), params.end());
		Leb(types, results.size());
		types.insert(types.end(), results.begin(), results.end());
	};
	type({0x7f, 0x7f, 0x7f}, {0x7e});
	type({0x7f}, {0x7e});
	type({0x7f, 0x7f}, {});
	type({}, {0x7e});
	Section(module, 1, types);

	std::vector<std::uint8_t> imports;
	Leb(imports, 2);
	String(imports, "spring:system-control");
	String(imports, "call-as-team");
	imports.push_back(0);
	Leb(imports, 0);
	String(imports, "spring:units-info");
	String(imports, "get-unit-def-id");
	imports.push_back(0);
	Leb(imports, 1);
	Section(module, 2, imports);

	std::vector<std::uint8_t> functions;
	Leb(functions, 2);
	Leb(functions, 2);
	Leb(functions, 3);
	Section(module, 3, functions);

	// The Core validator requires every module to define and export its linear
	// memory, with fixed limits in synced environments. This fixture never
	// touches memory, but it has to satisfy that rule to load.
	std::vector<std::uint8_t> memory = {0x01, 0x01, 0x01, 0x01};
	Section(module, 5, memory);

	// One mutable i64 observable written by the callback.
	std::vector<std::uint8_t> globals = {0x01, 0x7e, 0x01, 0x42, 0x00, 0x0b};
	Section(module, 6, globals);

	std::vector<std::uint8_t> exports;
	Leb(exports, 3);
	String(exports, "spring:callback/dispatch");
	exports.push_back(0);
	Leb(exports, 2); // first defined function after two imports
	String(exports, "run");
	exports.push_back(0);
	Leb(exports, 3);
	String(exports, "memory");
	exports.push_back(2);
	Leb(exports, 0);
	Section(module, 7, exports);

	std::vector<std::uint8_t> code;
	Leb(code, 2);
	// callback(id, user): get-unit-def-id(id + user), store packed result.
	const std::vector<std::uint8_t> callback = {
		0x00,             // local decl count
		0x20, 0x00,       // local.get 0
		0x20, 0x01,       // local.get 1
		0x6a,             // i32.add
		0x10, 0x01,       // call import #1
		0x24, 0x00,       // global.set 0
		0x0b,
	};
	Leb(code, callback.size());
	code.insert(code.end(), callback.begin(), callback.end());
	// run(): CallAsTeam(team=5, callbackID=7, userData=11), return callback result.
	const std::vector<std::uint8_t> run = {
		0x00,
		0x41, 0x05,
		0x41, 0x07,
		0x41, 0x0b,
		0x10, 0x00,
		0x1a,
		0x23, 0x00,
		0x0b,
	};
	Leb(code, run.size());
	code.insert(code.end(), run.begin(), run.end());
	Section(module, 10, code);
	return module;
}

} // namespace wasm_core_callback_test

TEST_CASE("Core synchronous callback propagates identity and permits reviewed re-entry")
{
#if defined(RECOIL_WASMTIME_AVAILABLE)
	using namespace wasm_core_callback_test;
	seenTeam = -1;
	seenUnit = -1;
	callbackCount = 0;

	SystemControlApi systemControl{};
	systemControl.CallAsTeam = CallAsTeam;
	UnitsInfoApi unitsInfo{};
	unitsInfo.GetUnitDefID = GetUnitDefID;
	NativeInterface native{};
	native.systemControl = &systemControl;
	native.unitsInfo = &unitsInfo;

	WasmRuntime runtime;
	REQUIRE(runtime.IsAvailable());
	// The runtime keeps its backend engine private to WasmModule/WasmCoreHost.
	// This test drives the linker directly, so it owns a plain engine of its own
	// and uses the runtime only for the validation config.
	wasm_engine_t* engine = wasm_engine_new();
	REQUIRE(engine != nullptr);

	const auto bytes = Module();
	const auto validation = recoil::wasm::core::ValidateModule(bytes,
		WasmEnvironment::RulesSynced, RECOIL_WASM_INTERFACE_VERSION_NUMBER, runtime.Config());
	INFO(validation.error);
	REQUIRE(validation.valid);

	wasmtime_store_t* store = wasmtime_store_new(engine, nullptr, nullptr);
	REQUIRE(store != nullptr);
	wasmtime_module_t* module = nullptr;
	REQUIRE(wasmtime_module_new(engine, bytes.data(), bytes.size(), &module) == nullptr);
	REQUIRE(module != nullptr);
	wasmtime_linker_t* linker = wasmtime_linker_new(engine);
	REQUIRE(linker != nullptr);

	WasmExecutionBudget budget(0, 1000, 1024);
	recoil::wasm::core::HostState state{};
	state.native = &native;
	state.budget = &budget;
	state.environment = WasmEnvironment::RulesSynced;
	std::string error;
	REQUIRE(recoil::wasm::core::RegisterSystemControlImports(linker, &state, error));
	REQUIRE(recoil::wasm::core::RegisterFastImports(linker, &state, error));

	wasmtime_instance_t instance{};
	wasm_trap_t* trap = nullptr;
	REQUIRE(wasmtime_linker_instantiate(linker, wasmtime_store_context(store), module,
		&instance, &trap) == nullptr);
	REQUIRE(trap == nullptr);

	wasmtime_extern_t run{};
	REQUIRE(wasmtime_instance_export_get(wasmtime_store_context(store), &instance,
		"run", 3, &run));
	REQUIRE(run.kind == WASMTIME_EXTERN_FUNC);
	wasmtime_val_raw_t slot{};
	REQUIRE(wasmtime_func_call_unchecked(wasmtime_store_context(store), &run.of.func,
		&slot, 1, &trap) == nullptr);
	REQUIRE(trap == nullptr);

	CHECK(seenTeam == 5);
	CHECK(callbackCount == 1);
	CHECK(seenUnit == 18); // callbackID 7 + userData 11
	CHECK(static_cast<std::uint32_t>(slot.i64) == 1018u);
	CHECK(static_cast<std::uint32_t>(static_cast<std::uint64_t>(slot.i64) >> 32) == 0u);

	wasmtime_extern_delete(&run);
	wasmtime_linker_delete(linker);
	wasmtime_module_delete(module);
	wasmtime_store_delete(store);
	wasm_engine_delete(engine);
#else
	SUCCEED("Wasmtime unavailable in this build");
#endif
}
