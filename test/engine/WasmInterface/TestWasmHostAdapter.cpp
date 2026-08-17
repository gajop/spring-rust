/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include <catch_amalgamated.hpp>

#include <cstdint>
#include <string>

#include "NativeInterface/NativeInterface.h"
#include "NativeInterface/NativeInterfaceWasmAdapter.h"
#include "WasmInterface/WasmModule.h"
#include "WasmInterface/WasmRuntime.h"
#include "generated/WasmHostAdapter.h"
#include "wasm/generated/WasmCallinRegistry.h"
#include "wasm/generated/WasmCalloutRegistry.h"

namespace {
	bool listCallSeen = false;
	std::int32_t listTeamID = -1;
	std::vector<std::int32_t> listUnitDefIDs;

	void FakeGetTeamUnitsByDefs(const GetTeamUnitsByDefsQuery* query,
		GetTeamUnitsByDefsResult* result)
	{
		listCallSeen = true;
		listTeamID = query->teamID;
		listUnitDefIDs.assign(query->unitDefIDs, query->unitDefIDs + query->defCount);
		static std::int32_t units[] = {101, 202};
		result->units = units;
		result->count = 2;
	}

	void FakeGetUnitBuildParams(const GetUnitBuildParamsQuery* query,
		GetUnitBuildParamsResult* result)
	{
		REQUIRE(std::string(query->paramName) == "buildRange");
		result->value = NumberOrBool{3.5f, false, false};
		result->hasValue = true;
	}

	void FakeGetMatrixData(const GfxGetMatrixDataQuery* query,
		GfxGetMatrixDataResult* result)
	{
		REQUIRE(query->mode == 4);
		for (std::size_t index = 0; index < 16; ++index)
			result->values[index] = static_cast<float>(index) + 0.5f;
	}

	void FakeGetCommandParams(const GetCommandParamsQuery* query,
		GetCommandParamsResult* result)
	{
		REQUIRE(query != nullptr);
		REQUIRE(query->command != nullptr);
		CHECK(query->command->cmdID == 42);
		CHECK(query->command->options == 3);
		CHECK(query->command->paramCount == 2);
		CHECK(query->command->params[0] == Catch::Approx(1.5f));
		CHECK(query->command->params[1] == Catch::Approx(2.5f));
		static float values[] = {9.0f, 10.0f};
		result->params = values;
		result->count = 2;
	}

	void FakeNormalize(const NormalizeQuery* query, NormalizeResult* result)
	{
		REQUIRE(query != nullptr);
		REQUIRE(query->vec != nullptr);
		query->vec->x = 0.6f;
		query->vec->y = 0.8f;
		query->vec->z = 0.0f;
		result->length = 5.0f;
	}

	bool cobCallSeen = false;
	void FakeCallCOBScript(const CallCOBScriptQuery* query, CallCOBScriptResult* result)
	{
		cobCallSeen = true;
		result->error = nullptr;
		result->retCode = query->func.id;
		static std::int32_t values[] = {17, 19};
		result->retValues = values;
		result->retCount = std::min<std::uint32_t>(query->retArgs, 2);
	}
}

TEST_CASE("generated Wasm adapter lowers lists and owns native results")
{
	UnitsQueryApi api{};
	api.GetTeamUnitsByDefs = FakeGetTeamUnitsByDefs;
	NativeInterface native{};
	native.unitsQuery = &api;

	listCallSeen = false;
	listUnitDefIDs.clear();
	WasmValue result;
	std::string error;
	const std::vector<WasmValue> arguments = {WasmValue::Record({
		{"teamID", WasmValue::I64(7)},
		{"unitDefIDs", WasmValue::List({WasmValue::I64(3), WasmValue::I64(5)})},
	})};

	CHECK(recoil::wasm::generated::DispatchNativeCallout(
		&native, "units_query", "GetTeamUnitsByDefs", arguments, result, error) ==
		recoil::wasm::generated::NativeCalloutDispatch::handled);
	CHECK(error.empty());
	CHECK(listCallSeen);
	CHECK(listTeamID == 7);
	CHECK(listUnitDefIDs == std::vector<std::int32_t>{3, 5});
	const auto* units = std::get_if<WasmValueList>(&result.storage);
	REQUIRE(units != nullptr);
	REQUIRE(units->size() == 2);
	CHECK(std::get<std::int64_t>((*units)[0].storage) == 101);
	CHECK(std::get<std::int64_t>((*units)[1].storage) == 202);
}

TEST_CASE("every registry manual callout reaches the explicit native fallback")
{
	NativeInterface native{};
	NativeInterfaceWasmAdapter adapter(&native);
	std::size_t manualCount = 0;
	for (const auto& descriptor : recoil::wasm::generated::kCallouts) {
		if (descriptor.status != recoil::wasm::generated::LoweringStatus::manual)
			continue;
		++manualCount;
		WasmValue result;
		std::string error;
		CHECK_FALSE(adapter.Callout(descriptor.module, descriptor.name, {}, result, error));
		CHECK_FALSE(error.empty());
		// A manual callout is intentionally omitted from generated dispatch, but
		// it must be claimed by the reviewed adapter branch rather than falling
		// through to the generic "no generated adapter" error.
		CHECK(error.find("no generated NativeInterface Wasm adapter") == std::string::npos);
	}
	CHECK(manualCount == 28);
}

TEST_CASE("generated Wasm adapter lowers strings, options and fixed arrays")
{
	UnitsInfoApi unitsInfo{};
	unitsInfo.GetUnitBuildParams = FakeGetUnitBuildParams;
	GfxApi gfx{};
	gfx.GetMatrixData = FakeGetMatrixData;
	NativeInterface native{};
	native.unitsInfo = &unitsInfo;
	native.gfx = &gfx;
	std::string error;
	WasmValue result;

	CHECK(recoil::wasm::generated::DispatchNativeCallout(
		&native, "units_info", "GetUnitBuildParams",
		{WasmValue::Record({
			{"unitID", WasmValue::I64(12)},
			{"paramName", WasmValue::String("buildRange")},
		})}, result, error) == recoil::wasm::generated::NativeCalloutDispatch::handled);
	CHECK(error.empty());
	const auto* optionalValue = std::get_if<WasmValueRecord>(&result.storage);
	REQUIRE(optionalValue != nullptr);
	CHECK(std::get<double>((*optionalValue).at("number").storage) == Catch::Approx(3.5));

	error.clear();
	result = WasmValue::Unit();
	CHECK(recoil::wasm::generated::DispatchNativeCallout(
		&native, "gfx", "GetMatrixData", {WasmValue::Record({
			{"mode", WasmValue::U64(4)},
		})}, result, error) == recoil::wasm::generated::NativeCalloutDispatch::handled);
	CHECK(error.empty());
	const auto* matrix = std::get_if<WasmValueList>(&result.storage);
	REQUIRE(matrix != nullptr);
	REQUIRE(matrix->size() == 16);
	CHECK(std::get<double>((*matrix)[0].storage) == Catch::Approx(0.5));
	CHECK(std::get<double>((*matrix)[15].storage) == Catch::Approx(15.5));
}

TEST_CASE("reviewed manual adapters lower representative native shapes")
{
	UnitsCommandsApi unitsCommands{};
	unitsCommands.GetCommandParams = FakeGetCommandParams;
	MathExtraApi mathExtra{};
	mathExtra.Normalize = FakeNormalize;
	NativeInterface native{};
	native.unitsCommands = &unitsCommands;
	native.mathExtra = &mathExtra;
	NativeInterfaceWasmAdapter adapter(&native);

	WasmValue result;
	std::string error;
	CHECK(adapter.Callout("units_commands", "GetCommandParams", {
		WasmValue::Record({
			{"cmdID", WasmValue::I64(42)},
			{"options", WasmValue::U64(3)},
			{"tag", WasmValue::I64(7)},
			{"aiCommandID", WasmValue::I64(8)},
			{"timeOut", WasmValue::F64(0.25)},
			{"params", WasmValue::List({WasmValue::F64(1.5), WasmValue::F64(2.5)})},
		})
	}, result, error));
	CHECK(error.empty());
	const auto* commandParams = std::get_if<WasmValueList>(&result.storage);
	REQUIRE(commandParams != nullptr);
	REQUIRE(commandParams->size() == 2);
	CHECK(std::get<double>((*commandParams)[0].storage) == Catch::Approx(9.0));
	CHECK(std::get<double>((*commandParams)[1].storage) == Catch::Approx(10.0));

	error.clear();
	result = WasmValue::Unit();
	CHECK(adapter.Callout("math_extra", "Normalize", {
		WasmValue::Record({
			{"x", WasmValue::F64(3.0)},
			{"y", WasmValue::F64(4.0)},
			{"z", WasmValue::F64(0.0)},
		})
	}, result, error));
	CHECK(error.empty());
	const auto* normalized = std::get_if<WasmValueRecord>(&result.storage);
	REQUIRE(normalized != nullptr);
	CHECK(std::get<double>(normalized->at("length").storage) == Catch::Approx(5.0));
	const auto* normalizedVector = std::get_if<WasmValueRecord>(
		&normalized->at("vec").storage);
	REQUIRE(normalizedVector != nullptr);
	CHECK(std::get<double>(normalizedVector->at("x").storage) == Catch::Approx(0.6));
	CHECK(std::get<double>(normalizedVector->at("y").storage) == Catch::Approx(0.8));

	error.clear();
	result = WasmValue::Unit();
	CHECK_FALSE(adapter.Callout("units_commands", "GetCommandParams", {
		WasmValue::Record({
			{"cmdID", WasmValue::I64(42)},
			{"options", WasmValue::U64(3)},
			{"tag", WasmValue::I64(7)},
			{"aiCommandID", WasmValue::I64(8)},
			{"timeOut", WasmValue::F64(0.25)},
			{"params", WasmValue::String("not-a-list")},
		})
	}, result, error));
	CHECK(error.find("list") != std::string::npos);
}

TEST_CASE("generated Wasm adapter serializes native callin queries")
{
	GameFrameQuery frameQuery = {.gameFrame = 1234};
	WasmValue result;
	std::string error;

	CHECK(recoil::wasm::generated::SerializeCallinQuery(
		"GameFrame", &frameQuery, result, error));
	CHECK(error.empty());
	const auto* frameRecord = std::get_if<WasmValueRecord>(&result.storage);
	REQUIRE(frameRecord != nullptr);
	CHECK(std::get<std::int64_t>(frameRecord->at("gameFrame").storage) == 1234);

	const std::vector<unsigned char> winners = {0, 2, 7};
	GameOverEventQuery gameOverQuery = {
		.winningAllyTeams = winners.data(),
		.count = static_cast<std::uint32_t>(winners.size()),
	};
	error.clear();
	result = WasmValue::Unit();
	CHECK(recoil::wasm::generated::SerializeCallinQuery(
		"GameOver", &gameOverQuery, result, error));
	CHECK(error.empty());
	const auto* gameOverRecord = std::get_if<WasmValueRecord>(&result.storage);
	REQUIRE(gameOverRecord != nullptr);
	const auto* winningTeams = std::get_if<WasmValueList>(
		&gameOverRecord->at("winningAllyTeams").storage);
	REQUIRE(winningTeams != nullptr);
	REQUIRE(winningTeams->size() == winners.size());
	CHECK(std::get<std::uint64_t>((*winningTeams)[1].storage) == 2);

	UnitCommandQuery unitCommandQuery = {};
	error.clear();
	result = WasmValue::Unit();
	CHECK(recoil::wasm::generated::SerializeCallinQuery(
		"UnitCommand", &unitCommandQuery, result, error));
	CHECK(error.empty());
	const auto* unitCommandRecord = std::get_if<WasmValueRecord>(&result.storage);
	REQUIRE(unitCommandRecord != nullptr);
	CHECK(std::get<std::int64_t>(unitCommandRecord->at("unitID").storage) == 0);

	DownloadProgressQuery downloadProgressQuery = {
		.downloadID = 9,
		.downloaded = 5'000'000'000,
		.total = 9'000'000'000,
	};
	error.clear();
	result = WasmValue::Unit();
	CHECK(recoil::wasm::generated::SerializeCallinQuery(
		"DownloadProgress", &downloadProgressQuery, result, error));
	CHECK(error.empty());
	const auto* downloadRecord = std::get_if<WasmValueRecord>(&result.storage);
	REQUIRE(downloadRecord != nullptr);
	CHECK(std::get<std::int64_t>(downloadRecord->at("downloaded").storage) == 5'000'000'000);
	CHECK(std::get<std::int64_t>(downloadRecord->at("total").storage) == 9'000'000'000);

	PongQuery pongQuery = {
		.pingTag = 7,
		.packetSendTimeMillis = 4'000'000'000,
		.packetRecvTimeMillis = 4'000'000'123,
	};
	error.clear();
	result = WasmValue::Unit();
	CHECK(recoil::wasm::generated::SerializeCallinQuery(
		"Pong", &pongQuery, result, error));
	CHECK(error.empty());
	const auto* pongRecord = std::get_if<WasmValueRecord>(&result.storage);
	REQUIRE(pongRecord != nullptr);
	CHECK(std::get<std::int64_t>(pongRecord->at("packetSendTimeMillis").storage) == 4'000'000'000);
	CHECK(std::get<std::int64_t>(pongRecord->at("packetRecvTimeMillis").storage) == 4'000'000'123);

	ArchiveCallinQuery archiveQuery = {
		.archive = reinterpret_cast<void*>(static_cast<std::uintptr_t>(0x1234)),
	};
	error.clear();
	result = WasmValue::Unit();
	CHECK(recoil::wasm::generated::SerializeCallinQuery(
		"Load", &archiveQuery, result, error));
	CHECK(error.empty());
	const auto* loadArchiveRecord = std::get_if<WasmValueRecord>(&result.storage);
	REQUIRE(loadArchiveRecord != nullptr);
	CHECK(std::get<std::uint64_t>(loadArchiveRecord->at("_unused").storage) == 0);

	error.clear();
	result = WasmValue::Unit();
	CHECK(recoil::wasm::generated::SerializeCallinQuery(
		"Save", &archiveQuery, result, error));
	CHECK(error.empty());
	const auto* archiveRecord = std::get_if<WasmValueRecord>(&result.storage);
	REQUIRE(archiveRecord != nullptr);
	CHECK(std::get<std::uint64_t>(archiveRecord->at("_unused").storage) == 0);
}

TEST_CASE("every canonical Wasm callin has a generated query serializer")
{
	for (const auto& descriptor : recoil::wasm::generated::kCallins) {
		WasmValue result;
		std::string error;
		CHECK_FALSE(recoil::wasm::generated::SerializeCallinQuery(
			descriptor.name, nullptr, result, error));
		CHECK(error == "null native callin query");
	}
}

TEST_CASE("reviewed CallCOBScript adapter checks result budget before native mutation")
{
	COBScriptApi cobScript{};
	cobScript.CallCOBScript = FakeCallCOBScript;
	SyncedCtrlApi syncedCtrl{};
	syncedCtrl.cobScript = &cobScript;
	NativeInterface native{};
	native.syncedCtrl = &syncedCtrl;
	NativeInterfaceWasmAdapter adapter(&native);
	WasmRuntimeConfig config;
	config.resultBytesLimit = sizeof(std::int32_t);
	WasmRuntime runtime(config);
	const std::vector<std::uint8_t> validCore = {
		0x00, 'a', 's', 'm', 0x01, 0x00, 0x00, 0x00,
	};
	WasmModule module(41, {
		.name = "cob-budget",
		.source = "test",
		.environment = WasmEnvironment::RulesSynced,
		.bytes = validCore,
	}, runtime, &adapter);
	std::string error;
	REQUIRE(module.Initialize(error));

	cobCallSeen = false;
	WasmValue result;
	CHECK_FALSE(adapter.Callout(module, "cob_script", "CallCOBScript", {
		WasmValue::I64(7),
		WasmValue::Record({
			{"name", WasmValue::String("Test")},
			{"id", WasmValue::I64(-1)},
		}),
		WasmValue::U64(2),
		WasmValue::List({}),
	}, result, error));
	CHECK_FALSE(cobCallSeen);
	CHECK(error == "CallCOBScript return values exceed the configured byte limit");

	config.resultBytesLimit = 16;
	// The budget is owned by the already-created module, so use a fresh module
	// with the larger limit for the successful reviewed-path check.
	WasmRuntime generousRuntime(config);
	WasmModule successfulModule(42, {
		.name = "cob-success",
		.source = "test",
		.environment = WasmEnvironment::RulesSynced,
		.bytes = validCore,
	}, generousRuntime, &adapter);
	error.clear();
	REQUIRE(successfulModule.Initialize(error));
	cobCallSeen = false;
	CHECK(adapter.Callout(successfulModule, "cob_script", "CallCOBScript", {
		WasmValue::I64(7),
		WasmValue::Record({
			{"name", WasmValue::String("Test")},
			{"id", WasmValue::I64(-1)},
		}),
		WasmValue::U64(2),
		WasmValue::List({WasmValue::I64(3), WasmValue::I64(5)}),
	}, result, error));
	CHECK(cobCallSeen);
	CHECK(error.empty());
	const auto* output = std::get_if<WasmValueRecord>(&result.storage);
	REQUIRE(output != nullptr);
	CHECK(std::get<std::int64_t>(output->at("retCode").storage) == -1);
}
