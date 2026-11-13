/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include <catch_amalgamated.hpp>

#include <array>
#include <bit>
#include <cstdint>
#include <cstring>
#include <string>
#include <vector>

#include <wasmtime.h>

#include "NativeInterface/NativeInterface.h"
#include "NativeInterface/WasmUiVisibility.h"
#include "WasmCoreAbi.h"
#include "WasmCoreBindings.h"

namespace WasmUiVisibility {
bool Active() { return false; }
ScopedContext::ScopedContext(bool) {}
ScopedContext::~ScopedContext() = default;
} // namespace WasmUiVisibility

namespace {

using recoil::wasm::core::HostState;
using recoil::wasm::core::RegisterColumnsImports;
using recoil::wasm::core::Status;
using recoil::wasm::core::UnpackI32Value;
using recoil::wasm::core::UnpackStatus;

constexpr std::int32_t COL_MASS = 22;
constexpr std::int32_t COL_CURRENT_COMMAND = 23;
constexpr std::uint8_t OPT_INTERNAL = 1u << 3;

const Error INVALID_UNIT{10, "invalid unit"};

struct MockState {
	std::array<CommandFFI, 4> queueStorage{};
	std::array<float, 5> paramsA{100.0f, 20.0f, 300.0f, 40.0f, 50.0f};
	std::array<float, 2> paramsB{42.0f, 99.0f};
};

MockState gMock;

void MockGetUnitMass(const GetUnitMassQuery* query, GetUnitMassResult* result)
{
	result->error = nullptr;
	result->mass = 0.0f;
	switch (query->unitID) {
		case 10: result->mass = 1200.0f; return;
		case 11: result->mass = 6000.0f; return;
		case 12: result->mass = 350.5f; return;
		default: result->error = &INVALID_UNIT; return;
	}
}

void MockGetCommandQueue(const GetCommandQueueQuery* query, GetCommandQueueResult* result)
{
	result->error = nullptr;
	result->commands = nullptr;
	result->count = 0;

	if (query->unitID == 10) {
		// Front is an engine-internal order; second is a player move.
		gMock.queueStorage[0] = CommandFFI{20, OPT_INTERNAL, 11, 0, 0.0f, gMock.paramsB.data(), 1};
		gMock.queueStorage[1] = CommandFFI{10, 0, 77, 0, 0.0f, gMock.paramsA.data(), 3};
		result->count = std::min<std::uint32_t>(query->maxCommands, 2u);
		result->commands = gMock.queueStorage.data();
		return;
	}
	if (query->unitID == 11) {
		// Command with more than 3 params: planes hold leading 3 plus full paramCount.
		gMock.queueStorage[0] = CommandFFI{40, 16, 88, 0, 0.0f, gMock.paramsA.data(), 5};
		result->count = query->maxCommands > 0 ? 1u : 0u;
		result->commands = gMock.queueStorage.data();
		return;
	}
	if (query->unitID == 12) {
		// Valid unit with empty queue.
		return;
	}
	if (query->unitID == 13) {
		// Valid unit with CMD_STOP (cmdID = 0, no params).
		gMock.queueStorage[0] = CommandFFI{0, 0, 5, 0, 0.0f, nullptr, 0};
		result->count = query->maxCommands > 0 ? 1u : 0u;
		result->commands = gMock.queueStorage.data();
		return;
	}
	result->error = &INVALID_UNIT;
}

struct Fixture {
	UnitsInfoApi unitsInfo{};
	UnitsQueryApi unitsQuery{};
	UnitsCommandsApi unitsCommands{};
	UnitsWeaponsApi unitsWeapons{};
	LOSApi los{};
	MoveCtrlApi moveCtrl{};
	UnitControlApi unitControl{};
	SyncedCtrlApi syncedCtrl{};
	RulesParamsApi rulesParams{};
	NativeInterface native{};
	HostState state{};

	wasm_engine_t* engine = nullptr;
	wasmtime_store_t* store = nullptr;
	wasmtime_context_t* ctx = nullptr;
	wasmtime_module_t* module = nullptr;
	wasmtime_linker_t* linker = nullptr;
	wasmtime_instance_t instance{};
	wasmtime_memory_t memory{};
	wasmtime_func_t callGetUnits{};

	Fixture()
	{
		unitsInfo.GetUnitMass = MockGetUnitMass;
		unitsCommands.GetCommandQueue = MockGetCommandQueue;
		syncedCtrl.unit = &unitControl;

		native.unitsInfo = &unitsInfo;
		native.unitsQuery = &unitsQuery;
		native.unitsCommands = &unitsCommands;
		native.unitsWeapons = &unitsWeapons;
		native.los = &los;
		native.moveCtrl = &moveCtrl;
		native.syncedCtrl = &syncedCtrl;
		native.rulesParams = &rulesParams;

		state.native = &native;
		state.environment = WasmEnvironment::RulesSynced;

		const std::string wat = R"(
			(module
				(import "spring:columns" "get-units"
					(func $get_units (param i32 i32 i32 i32 i32 i32 i32 i32) (result i64)))
				(memory (export "memory") 1)
				(func (export "call") (param i32 i32 i32 i32 i32 i32 i32 i32) (result i64)
					local.get 0
					local.get 1
					local.get 2
					local.get 3
					local.get 4
					local.get 5
					local.get 6
					local.get 7
					call $get_units)
			)
		)";
		wasm_byte_vec_t wasmBytes{};
		REQUIRE(wasmtime_wat2wasm(wat.data(), wat.size(), &wasmBytes) == nullptr);

		engine = wasm_engine_new();
		REQUIRE(engine != nullptr);
		store = wasmtime_store_new(engine, nullptr, nullptr);
		ctx = wasmtime_store_context(store);
		REQUIRE(wasmtime_module_new(
			engine, reinterpret_cast<const std::uint8_t*>(wasmBytes.data), wasmBytes.size, &module) == nullptr);
		wasm_byte_vec_delete(&wasmBytes);

		linker = wasmtime_linker_new(engine);
		std::string error;
		REQUIRE(RegisterColumnsImports(linker, &state, error));

		wasm_trap_t* trap = nullptr;
		REQUIRE(wasmtime_linker_instantiate(linker, ctx, module, &instance, &trap) == nullptr);
		REQUIRE(trap == nullptr);

		wasmtime_extern_t ext{};
		REQUIRE(wasmtime_instance_export_get(ctx, &instance, "memory", 6, &ext));
		REQUIRE(ext.kind == WASMTIME_EXTERN_MEMORY);
		memory = ext.of.memory;

		REQUIRE(wasmtime_instance_export_get(ctx, &instance, "call", 4, &ext));
		REQUIRE(ext.kind == WASMTIME_EXTERN_FUNC);
		callGetUnits = ext.of.func;
	}

	~Fixture()
	{
		if (linker) wasmtime_linker_delete(linker);
		if (module) wasmtime_module_delete(module);
		if (store) wasmtime_store_delete(store);
		if (engine) wasm_engine_delete(engine);
	}

	std::uint8_t* Mem() { return wasmtime_memory_data(ctx, &memory); }

	void WriteI32s(std::uint32_t addr, const std::vector<std::int32_t>& values)
	{
		if (!values.empty())
			std::memcpy(Mem() + addr, values.data(), values.size() * 4);
	}

	std::int32_t ReadI32(std::uint32_t addr)
	{
		std::int32_t v = 0;
		std::memcpy(&v, Mem() + addr, 4);
		return v;
	}

	float ReadF32(std::uint32_t addr)
	{
		float v = 0.0f;
		std::memcpy(&v, Mem() + addr, 4);
		return v;
	}

	std::uint64_t CallGetUnits(std::int32_t prop, std::uint32_t idsAddr, std::uint32_t count,
		std::uint32_t argsAddr, std::int32_t arg, std::uint32_t outAddr, std::uint32_t outBytes,
		std::uint32_t foundAddr)
	{
		wasmtime_val_t params[8]{};
		params[0].kind = WASMTIME_I32;
		params[0].of.i32 = prop;
		params[1].kind = WASMTIME_I32;
		params[1].of.i32 = static_cast<std::int32_t>(idsAddr);
		params[2].kind = WASMTIME_I32;
		params[2].of.i32 = static_cast<std::int32_t>(count);
		params[3].kind = WASMTIME_I32;
		params[3].of.i32 = static_cast<std::int32_t>(argsAddr);
		params[4].kind = WASMTIME_I32;
		params[4].of.i32 = arg;
		params[5].kind = WASMTIME_I32;
		params[5].of.i32 = static_cast<std::int32_t>(outAddr);
		params[6].kind = WASMTIME_I32;
		params[6].of.i32 = static_cast<std::int32_t>(outBytes);
		params[7].kind = WASMTIME_I32;
		params[7].of.i32 = static_cast<std::int32_t>(foundAddr);

		wasmtime_val_t results[1]{};
		wasm_trap_t* trap = nullptr;
		REQUIRE(wasmtime_func_call(ctx, &callGetUnits, params, 8, results, 1, &trap) == nullptr);
		REQUIRE(trap == nullptr);
		REQUIRE(results[0].kind == WASMTIME_I64);
		return static_cast<std::uint64_t>(results[0].of.i64);
	}
};

} // namespace

TEST_CASE("Wasm columns read unit mass and zero refused units", "[wasm][columns]")
{
	Fixture fx;
	const std::uint32_t idsAddr = 0x100;
	const std::uint32_t outAddr = 0x200;
	const std::uint32_t foundAddr = 0x300;
	fx.WriteI32s(idsAddr, {10, 999, 11, 12});

	const std::uint64_t packed = fx.CallGetUnits(COL_MASS, idsAddr, 4, 0, 0, outAddr, 16, foundAddr);
	REQUIRE(UnpackStatus(packed) == static_cast<std::int32_t>(Status::Ok));
	REQUIRE(UnpackI32Value(packed) == 3);

	CHECK(fx.ReadF32(outAddr + 0) == 1200.0f);
	CHECK(fx.ReadF32(outAddr + 4) == 0.0f);
	CHECK(fx.ReadF32(outAddr + 8) == 6000.0f);
	CHECK(fx.ReadF32(outAddr + 12) == 350.5f);
	CHECK(fx.Mem()[foundAddr + 0] == 1);
	CHECK(fx.Mem()[foundAddr + 1] == 0);
	CHECK(fx.Mem()[foundAddr + 2] == 1);
	CHECK(fx.Mem()[foundAddr + 3] == 1);
}

TEST_CASE("Wasm columns read current command with lookahead and leading parameter subset", "[wasm][columns]")
{
	Fixture fx;
	const std::uint32_t idsAddr = 0x100;
	const std::uint32_t argsAddr = 0x180;
	const std::uint32_t outAddr = 0x400;
	const std::uint32_t foundAddr = 0x800;
	// Units: 10 (internal + move), 11 (5-param cmd), 12 (empty queue), 13 (CMD_STOP), 999 (invalid)
	fx.WriteI32s(idsAddr, {10, 11, 12, 13, 999});

	SECTION("arg = 4 skips engine-internal orders")
	{
		const std::uint64_t packed = fx.CallGetUnits(
			COL_CURRENT_COMMAND, idsAddr, 5, 0, 4, outAddr, 5 * 8 * 4, foundAddr);
		REQUIRE(UnpackStatus(packed) == static_cast<std::int32_t>(Status::Ok));
		REQUIRE(UnpackI32Value(packed) == 4);

		auto planeI32 = [&](std::uint32_t plane, std::uint32_t i) {
			return fx.ReadI32(outAddr + (plane * 5 + i) * 4);
		};
		auto planeF32 = [&](std::uint32_t plane, std::uint32_t i) {
			return fx.ReadF32(outAddr + (plane * 5 + i) * 4);
		};

		// Unit 10: skipped internal order, took move
		CHECK(fx.Mem()[foundAddr + 0] == 1);
		CHECK(planeI32(0, 0) == 1);
		CHECK(planeI32(1, 0) == 10);
		CHECK(planeI32(2, 0) == 0);
		CHECK(planeI32(3, 0) == 77);
		CHECK(planeI32(4, 0) == 3);
		CHECK(planeF32(5, 0) == 100.0f);
		CHECK(planeF32(6, 0) == 20.0f);
		CHECK(planeF32(7, 0) == 300.0f);

		// Unit 11: leading 3 parameters plus full count 5
		CHECK(fx.Mem()[foundAddr + 1] == 1);
		CHECK(planeI32(0, 1) == 1);
		CHECK(planeI32(1, 1) == 40);
		CHECK(planeI32(2, 1) == 16);
		CHECK(planeI32(3, 1) == 88);
		CHECK(planeI32(4, 1) == 5);
		CHECK(planeF32(5, 1) == 100.0f);
		CHECK(planeF32(6, 1) == 20.0f);
		CHECK(planeF32(7, 1) == 300.0f);

		// Unit 12: empty queue -> found=1, has_command=0
		CHECK(fx.Mem()[foundAddr + 2] == 1);
		CHECK(planeI32(0, 2) == 0);
		CHECK(planeI32(1, 2) == 0);
		CHECK(planeI32(4, 2) == 0);

		// Unit 13: CMD_STOP -> found=1, has_command=1, cmd_id=0
		CHECK(fx.Mem()[foundAddr + 3] == 1);
		CHECK(planeI32(0, 3) == 1);
		CHECK(planeI32(1, 3) == 0);
		CHECK(planeI32(3, 3) == 5);
		CHECK(planeI32(4, 3) == 0);

		// Unit 999: invalid -> found=0, zeroed
		CHECK(fx.Mem()[foundAddr + 4] == 0);
		CHECK(planeI32(0, 4) == 0);
	}

	SECTION("per-entry args allow slot-0 verbatim when <= 0")
	{
		fx.WriteI32s(idsAddr, {10, 10});
		fx.WriteI32s(argsAddr, {0, 4});
		const std::uint64_t packed = fx.CallGetUnits(
			COL_CURRENT_COMMAND, idsAddr, 2, argsAddr, 0, outAddr, 2 * 8 * 4, foundAddr);
		REQUIRE(UnpackStatus(packed) == static_cast<std::int32_t>(Status::Ok));
		REQUIRE(UnpackI32Value(packed) == 2);
		CHECK(fx.ReadI32(outAddr + (1 * 2 + 0) * 4) == 20);
		CHECK(fx.ReadI32(outAddr + (2 * 2 + 0) * 4) == OPT_INTERNAL);
		CHECK(fx.ReadI32(outAddr + (1 * 2 + 1) * 4) == 10);
	}

	SECTION("buffer overflow and out of bounds return expected statuses")
	{
		const std::uint64_t overflow = fx.CallGetUnits(
			COL_CURRENT_COMMAND, idsAddr, 2, 0, 4, outAddr, 2 * 8 * 4 - 4, 0);
		CHECK(UnpackStatus(overflow) == static_cast<std::int32_t>(Status::BufferOverflow));
		CHECK(UnpackI32Value(overflow) == 2 * 8 * 4);

		const std::uint64_t oob = fx.CallGetUnits(
			COL_MASS, 0xFFF8, 4, 0, 0, outAddr, 16, 0);
		CHECK(UnpackStatus(oob) == static_cast<std::int32_t>(Status::OutOfBounds));
	}
}
