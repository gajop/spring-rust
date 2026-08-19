/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <string>

#include "NativeInterface/NativeInterface.h"
#include "WasmCoreAbi.h"
#include "WasmResources.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)

struct HostState {
	NativeInterface* native = nullptr;
	Memory memory;
	WasmExecutionBudget* budget = nullptr;
	bool fixedMemory = false;
};

bool RegisterFastImports(wasmtime_linker_t* linker, HostState* state, std::string& error);
bool RegisterUnitsQueryImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool RegisterUnitDefsImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error);
bool BindGuestMemory(HostState& state, wasmtime_context_t* context,
	const wasmtime_instance_t& instance, std::string& error);

class InstanceBindings {
public:
	explicit InstanceBindings(NativeInterface* nativeInterface,
		WasmExecutionBudget* executionBudget = nullptr, bool fixedMemory = false)
	{
		host.native = nativeInterface;
		host.budget = executionBudget;
		host.fixedMemory = fixedMemory;
		if (fixedMemory)
			host.memory.MarkStable();
	}

	bool RegisterImports(wasmtime_linker_t* linker, std::string& error)
	{
		if (!RegisterFastImports(linker, &host, error))
			return false;
		if (!RegisterUnitsQueryImports(linker, &host, error))
			return false;
		return RegisterUnitDefsImports(linker, &host, error);
	}

	bool Bind(wasmtime_context_t* context, const wasmtime_instance_t& instance,
		std::string& error);

	bool GameFrame(wasmtime_context_t* context, std::int32_t frame, std::string& error) const;
	bool GameFramePost(wasmtime_context_t* context, std::int32_t frame, std::string& error) const;
	bool Update(wasmtime_context_t* context, float deltaSeconds, std::string& error) const;
	bool UnitCreated(wasmtime_context_t* context, std::int32_t unitID,
		std::int32_t unitDefID, std::int32_t unitTeam, std::int32_t builderID,
		std::string& error) const;
	bool UnitPreDamaged(wasmtime_context_t* context, std::int32_t unitID,
		std::int32_t unitDefID, std::int32_t unitTeam, float damage, bool paralyzer,
		std::int32_t weaponDefID, std::int32_t projectileID, std::int32_t attackerID,
		std::int32_t attackerDefID, std::int32_t attackerTeam,
		float& newDamage, float& impulseMult, std::string& error) const;
	bool AllowUnitCreation(wasmtime_context_t* context, std::int32_t unitDefID,
		std::int32_t builderID, std::int32_t builderTeam, bool hasBuildInfo,
		float buildX, float buildY, float buildZ, std::int32_t buildFacing,
		bool& allow, bool& dropOrder, std::string& error) const;
	bool DrawWorld(wasmtime_context_t* context, std::string& error) const;

	bool HasGameFrame() const { return gameFrame.Present(); }
	bool HasGameFramePost() const { return gameFramePost.Present(); }
	bool HasUpdate() const { return update.Present(); }
	bool HasUnitCreated() const { return unitCreated.Present(); }
	bool HasUnitPreDamaged() const { return unitPreDamaged.Present(); }
	bool HasAllowUnitCreation() const { return allowUnitCreation.Present(); }
	bool HasDrawWorld() const { return drawWorld.Present(); }

	HostState& Host() { return host; }

private:
	HostState host;
	I32ToVoidExport gameFrame;
	I32ToVoidExport gameFramePost;
	RawExport update;
	RawExport unitCreated;
	RawExport unitPreDamaged;
	RawExport allowUnitCreation;
	RawExport drawWorld;
};

#endif

} // namespace recoil::wasm::core
