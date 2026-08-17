/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include <catch_amalgamated.hpp>

#include <cmath>
#include <cstdint>
#include <limits>
#include <string>
#include <vector>

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
	bool buildParamsCallSeen = false;
	bool matrixCallSeen = false;
	bool normalizeCallSeen = false;

	void FakeGetTeamUnitsByDefs(const GetTeamUnitsByDefsQuery* query,
		GetTeamUnitsByDefsResult* result)
	{
		listCallSeen = true;
		listTeamID = query->teamID;
		if (query->defCount == 0)
			listUnitDefIDs.clear();
		else
			listUnitDefIDs.assign(query->unitDefIDs, query->unitDefIDs + query->defCount);
		result->error = nullptr;
		static std::int32_t units[] = {101, 202};
		result->units = units;
		result->count = 2;
	}

	bool ReferenceI32(const WasmValue& value, std::int32_t& output)
	{
		if (const auto* signedValue = std::get_if<std::int64_t>(&value.storage)) {
			if (*signedValue < std::numeric_limits<std::int32_t>::lowest() ||
				*signedValue > std::numeric_limits<std::int32_t>::max())
				return false;
			output = static_cast<std::int32_t>(*signedValue);
			return true;
		}
		if (const auto* unsignedValue = std::get_if<std::uint64_t>(&value.storage)) {
			if (*unsignedValue > static_cast<std::uint64_t>(
				std::numeric_limits<std::int32_t>::max()))
				return false;
			output = static_cast<std::int32_t>(*unsignedValue);
			return true;
		}
		return false;
	}

	bool ReferenceGetTeamUnitsByDefs(const std::vector<WasmValue>& arguments,
		std::int32_t& teamID, std::vector<std::int32_t>& unitDefIDs)
	{
		if (arguments.size() != 1)
			return false;
		const auto* record = std::get_if<WasmValueRecord>(&arguments.front().storage);
		if (record == nullptr)
			return false;
		const auto team = record->find("team-id");
		const auto defs = record->find("unit-def-i-ds");
		if (team == record->end() || defs == record->end() ||
			!ReferenceI32(team->second, teamID))
			return false;
		const auto* list = std::get_if<WasmValueList>(&defs->second.storage);
		if (list == nullptr)
			return false;
		unitDefIDs.clear();
		unitDefIDs.reserve(list->size());
		for (const auto& value : *list) {
			std::int32_t unitDefID = 0;
			if (!ReferenceI32(value, unitDefID))
				return false;
			unitDefIDs.push_back(unitDefID);
		}
		return true;
	}

	void FakeGetUnitBuildParams(const GetUnitBuildParamsQuery* query,
		GetUnitBuildParamsResult* result)
	{
		buildParamsCallSeen = true;
		REQUIRE(std::string(query->paramName) == "buildRange");
		result->value = NumberOrBool{3.5f, false, false};
		result->hasValue = true;
	}

	void FakeGetMatrixData(const GfxGetMatrixDataQuery* query,
		GfxGetMatrixDataResult* result)
	{
		matrixCallSeen = true;
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
		normalizeCallSeen = true;
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

	bool ReferenceF64(const WasmValue& value, double& output)
	{
		if (const auto* floating = std::get_if<double>(&value.storage)) {
			if (!std::isfinite(*floating))
				return false;
			output = *floating;
			return true;
		}
		if (const auto* signedValue = std::get_if<std::int64_t>(&value.storage)) {
			output = static_cast<double>(*signedValue);
			return true;
		}
		if (const auto* unsignedValue = std::get_if<std::uint64_t>(&value.storage)) {
			output = static_cast<double>(*unsignedValue);
			return true;
		}
		return false;
	}

	bool ReferenceGetUnitBuildParams(const std::vector<WasmValue>& arguments)
	{
		if (arguments.size() != 1)
			return false;
		const auto* record = std::get_if<WasmValueRecord>(&arguments.front().storage);
		if (record == nullptr)
			return false;
		const auto unit = record->find("unit-id");
		const auto name = record->find("param-name");
		std::int32_t unitID = 0;
		return unit != record->end() && name != record->end() &&
			ReferenceI32(unit->second, unitID) &&
			std::get_if<std::string>(&name->second.storage) != nullptr;
	}

	bool ReferenceGetMatrixData(const std::vector<WasmValue>& arguments)
	{
		if (arguments.size() != 1)
			return false;
		const auto* record = std::get_if<WasmValueRecord>(&arguments.front().storage);
		if (record == nullptr)
			return false;
		const auto mode = record->find("mode");
		std::int32_t modeValue = 0;
		return mode != record->end() && ReferenceI32(mode->second, modeValue) &&
			modeValue == 4;
	}

	bool ReferenceNormalize(const std::vector<WasmValue>& arguments)
	{
		if (arguments.size() != 1)
			return false;
		const auto* record = std::get_if<WasmValueRecord>(&arguments.front().storage);
		if (record == nullptr)
			return false;
		for (const char* name : {"x", "y", "z"}) {
			const auto value = record->find(name);
			double numericValue = 0.0;
			if (value == record->end() || !ReferenceF64(value->second, numericValue))
				return false;
		}
		return true;
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
		{"team-id", WasmValue::I64(7)},
		{"unit-def-i-ds", WasmValue::List({WasmValue::I64(3), WasmValue::I64(5)})},
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

TEST_CASE("generated Wasm conversion agrees with an independent bounded corpus")
{
	UnitsQueryApi api{};
	api.GetTeamUnitsByDefs = FakeGetTeamUnitsByDefs;
	NativeInterface native{};
	native.unitsQuery = &api;

	std::uint32_t state = 0x9e3779b9u;
	const auto next = [&state]() {
		state = state * 1664525u + 1013904223u;
		return state;
	};
	for (std::size_t index = 0; index < 128; ++index) {
		const bool malformedTeam = index % 11 == 0;
		const bool malformedList = index % 13 == 0;
		const std::int32_t expectedTeam = static_cast<std::int32_t>(next() % 8) - 2;
		WasmValue team = index % 3 == 0
			? WasmValue::U64(static_cast<std::uint64_t>(expectedTeam))
			: WasmValue::I64(expectedTeam);
		if (malformedTeam)
			team = WasmValue::F64(static_cast<double>(expectedTeam));

		WasmValueList definitions;
		std::vector<std::int32_t> expectedDefinitions;
		const std::size_t definitionCount = next() % 5;
		for (std::size_t definitionIndex = 0; definitionIndex < definitionCount;
			++definitionIndex) {
			const auto definition = static_cast<std::int32_t>(next() % 100) - 50;
			expectedDefinitions.push_back(definition);
			definitions.push_back(definitionIndex % 2 == 0
				? WasmValue::I64(definition)
				: WasmValue::U64(static_cast<std::uint64_t>(definition)));
		}
		if (malformedList)
			definitions.push_back(WasmValue::String("not-an-id"));

		std::vector<WasmValue> arguments = {WasmValue::Record({
			{"team-id", std::move(team)},
			{"unit-def-i-ds", WasmValue::List(std::move(definitions))},
		})};
		std::int32_t referenceTeam = 0;
		std::vector<std::int32_t> referenceDefinitions;
		const bool referenceAccepted = ReferenceGetTeamUnitsByDefs(arguments,
			referenceTeam, referenceDefinitions);
		WasmValue result;
		std::string error;
		listCallSeen = false;
		listUnitDefIDs.clear();
		const auto dispatch = recoil::wasm::generated::DispatchNativeCallout(
			&native, "units_query", "GetTeamUnitsByDefs", arguments, result, error);
		CHECK(dispatch == recoil::wasm::generated::NativeCalloutDispatch::handled);
		CHECK(listCallSeen == referenceAccepted);
		if (referenceAccepted) {
			CHECK(error.empty());
			CHECK(listTeamID == referenceTeam);
			CHECK(listUnitDefIDs == referenceDefinitions);
		} else {
			CHECK_FALSE(error.empty());
		}
	}
}

TEST_CASE("generated Wasm conversion covers independent record and array corpus")
{
	UnitsInfoApi unitsInfo{};
	unitsInfo.GetUnitBuildParams = FakeGetUnitBuildParams;
	GfxApi gfx{};
	gfx.GetMatrixData = FakeGetMatrixData;
	MathExtraApi mathExtra{};
	mathExtra.Normalize = FakeNormalize;
	NativeInterface native{};
	native.unitsInfo = &unitsInfo;
	native.gfx = &gfx;
	native.mathExtra = &mathExtra;
	NativeInterfaceWasmAdapter adapter(&native);

	std::uint32_t state = 0x243f6a88u;
	const auto next = [&state]() {
		state = state * 1103515245u + 12345u;
		return state;
	};
	for (std::size_t index = 0; index < 96; ++index) {
		const auto numeric = static_cast<std::int64_t>(next() % 31) - 15;
		const auto numericValue = index % 2 == 0
			? WasmValue::I64(numeric)
			: WasmValue::F64(static_cast<double>(numeric) + 0.25);
		const bool malformed = index % 9 == 0;
		const std::vector<WasmValue> buildArguments = {WasmValue::Record({
			{"unit-id", malformed ? WasmValue::String("not-an-id") : numericValue},
			{"param-name", WasmValue::String("buildRange")},
		})};
		const bool buildAccepted = ReferenceGetUnitBuildParams(buildArguments);
		buildParamsCallSeen = false;
		WasmValue result;
		std::string error;
		CHECK(recoil::wasm::generated::DispatchNativeCallout(
			&native, "units_info", "GetUnitBuildParams", buildArguments,
			result, error) == recoil::wasm::generated::NativeCalloutDispatch::handled);
		CHECK(buildParamsCallSeen == buildAccepted);
		CHECK(error.empty() == buildAccepted);

		const std::vector<WasmValue> matrixArguments = {WasmValue::Record({
			{"mode", index % 7 == 0 ? WasmValue::String("not-a-mode") :
				(index % 2 == 0 ? WasmValue::I64(4) : WasmValue::U64(4))},
		})};
		const bool matrixAccepted = ReferenceGetMatrixData(matrixArguments);
		matrixCallSeen = false;
		error.clear();
		result = WasmValue::Unit();
		CHECK(recoil::wasm::generated::DispatchNativeCallout(
			&native, "gfx", "GetMatrixData", matrixArguments, result, error) ==
			recoil::wasm::generated::NativeCalloutDispatch::handled);
		CHECK(matrixCallSeen == matrixAccepted);
		CHECK(error.empty() == matrixAccepted);

		WasmValueList vectorValues = {
			index % 11 == 0 ? WasmValue::String("not-a-number") :
				WasmValue::F64(static_cast<double>(numeric) + 0.25),
			WasmValue::F64(1.5),
			WasmValue::F64(-2.0),
		};
		const std::vector<WasmValue> normalizeArguments = {WasmValue::Record({
			{"x", vectorValues[0]},
			{"y", vectorValues[1]},
			{"z", vectorValues[2]},
		})};
		const bool normalizeAccepted = ReferenceNormalize(normalizeArguments);
		normalizeCallSeen = false;
		error.clear();
		result = WasmValue::Unit();
		CHECK(adapter.Callout("math_extra", "Normalize", normalizeArguments,
			result, error) == normalizeAccepted);
		CHECK(normalizeCallSeen == normalizeAccepted);
		CHECK(error.empty() == normalizeAccepted);
	}
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
			{"unit-id", WasmValue::I64(12)},
			{"param-name", WasmValue::String("buildRange")},
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
	CHECK(std::get<std::int64_t>(frameRecord->at("game-frame").storage) == 1234);

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
		&gameOverRecord->at("winning-ally-teams").storage);
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
	CHECK(std::get<std::int64_t>(unitCommandRecord->at("unit-id").storage) == 0);

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
	CHECK(std::get<std::int64_t>(pongRecord->at("packet-send-time-millis").storage) == 4'000'000'000);
	CHECK(std::get<std::int64_t>(pongRecord->at("packet-recv-time-millis").storage) == 4'000'000'123);

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
	CHECK(std::get<std::uint64_t>(loadArchiveRecord->at("unused").storage) == 0);

	error.clear();
	result = WasmValue::Unit();
	CHECK(recoil::wasm::generated::SerializeCallinQuery(
		"Save", &archiveQuery, result, error));
	CHECK(error.empty());
	const auto* archiveRecord = std::get_if<WasmValueRecord>(&result.storage);
	REQUIRE(archiveRecord != nullptr);
	CHECK(std::get<std::uint64_t>(archiveRecord->at("unused").storage) == 0);
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
