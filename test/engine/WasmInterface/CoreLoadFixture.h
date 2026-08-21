/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */
#pragma once

#include <cstdint>
#include <string>
#include <string_view>
#include <vector>

#include "NativeInterface/NativeInterface.h"
#include "WasmInterface/WasmHost.h"

// Core is the production transport, so WasmInterfaceSystem::LoadModule now takes
// the Core path for every module: it demands a host adapter with a
// NativeInterface, and the reviewed handwritten binding groups refuse to
// register against a missing API table. Tests that only exercise system
// bookkeeping (load ordering, duplicate names, manifest atomicity) still have to
// get past that, so this fixture supplies a fully wired but inert API table and
// the smallest module the Core validator accepts.
namespace wasm_core_load_fixture {

inline NativeInterface* StubNativeInterface()
{
	// Only the groups a handwritten Register*Imports dereferences need to exist;
	// the generated registrations require nothing beyond a non-null table. The
	// function pointers stay null because nothing here is ever called: each
	// binding re-checks its own entry point before use.
	static const UnitControlApi unitControl{};
	static const TerrainControlApi terrainControl{};
	static const COBScriptApi cobScript{};
	static const SyncedCtrlApi syncedCtrl{
		.unit = &unitControl,
		.terrain = &terrainControl,
		.cobScript = &cobScript,
	};

	static const ConfigApi config{};
	static const GfxApi gfx{};
	static const MathExtraApi mathExtra{};
	static const MessagesApi messages{};
	static const ProfilingApi profiling{};
	static const RmlUiApi rmlUi{};
	static const RulesParamsApi rulesParams{};
	static const SystemControlApi systemControl{};
	static const TerrainApi terrain{};
	static const UnitDefsApi unitDefs{};
	static const UnitsCommandsApi unitsCommands{};
	static const UnitsInfoApi unitsInfo{};
	static const UnitsPiecesApi unitsPieces{};
	static const UnitsQueryApi unitsQuery{};
	static const VFSApi vfs{};

	static NativeInterface native = [] {
		NativeInterface table{};
		table.config = &config;
		table.gfx = &gfx;
		table.mathExtra = &mathExtra;
		table.messages = &messages;
		table.profiling = &profiling;
		table.rmlUi = &rmlUi;
		table.rulesParams = &rulesParams;
		table.syncedCtrl = &syncedCtrl;
		table.systemControl = &systemControl;
		table.terrain = &terrain;
		table.unitDefs = &unitDefs;
		table.unitsCommands = &unitsCommands;
		table.unitsInfo = &unitsInfo;
		table.unitsPieces = &unitsPieces;
		table.unitsQuery = &unitsQuery;
		table.vfs = &vfs;
		return table;
	}();
	return &native;
}

class StubAdapter final : public WasmHostAdapter {
public:
	bool Callout(std::string_view /*module*/, std::string_view /*function*/,
		const std::vector<WasmValue>& /*arguments*/, WasmValue& /*result*/,
		std::string& error) override
	{
		error = "the Core load fixture does not serve component callouts";
		return false;
	}

	void* NativeInterfaceHandle() override { return StubNativeInterface(); }
};

inline void AppendLeb(std::vector<std::uint8_t>& bytes, std::uint64_t value)
{
	do {
		std::uint8_t byte = static_cast<std::uint8_t>(value & 0x7f);
		value >>= 7;
		if (value != 0)
			byte |= 0x80;
		bytes.push_back(byte);
	} while (value != 0);
}

inline void AppendString(std::vector<std::uint8_t>& bytes, std::string_view text)
{
	AppendLeb(bytes, text.size());
	bytes.insert(bytes.end(), text.begin(), text.end());
}

inline void AppendSection(std::vector<std::uint8_t>& module, std::uint8_t id,
	const std::vector<std::uint8_t>& payload)
{
	module.push_back(id);
	AppendLeb(module, payload.size());
	module.insert(module.end(), payload.begin(), payload.end());
}

// Synced environments require fixed memory limits, so the shared fixture always
// declares a maximum: growth must not depend on local host resources.
inline std::vector<std::uint8_t> MinimalCoreModule()
{
	std::vector<std::uint8_t> module{0x00, 'a', 's', 'm', 0x01, 0x00, 0x00, 0x00};

	std::vector<std::uint8_t> memory;
	AppendLeb(memory, 1); // one memory
	AppendLeb(memory, 1); // limits: has maximum
	AppendLeb(memory, 1); // minimum pages
	AppendLeb(memory, 1); // maximum pages
	AppendSection(module, 5, memory);

	std::vector<std::uint8_t> exports;
	AppendLeb(exports, 1); // one export
	AppendString(exports, "memory");
	exports.push_back(2); // memory kind
	AppendLeb(exports, 0); // memory index
	AppendSection(module, 7, exports);

	return module;
}

} // namespace wasm_core_load_fixture
