/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmTypedHost.h"

#include <cstdlib>

#include <dlfcn.h>

#include "NativeInterface/api/Callins.h"
#include "System/Log/ILog.h"

namespace {

bool TruthyEnvironment(const char* name)
{
	const char* value = std::getenv(name);
	if (value == nullptr)
		return false;
	const std::string_view setting(value);
	return setting == "1" || setting == "true" || setting == "TRUE" ||
		setting == "yes" || setting == "YES" || setting == "on" || setting == "ON";
}

} // namespace

bool WasmTypedHost::Enabled()
{
	static const bool enabled = TruthyEnvironment("SPRING_WASM_TYPED_HOST");
	return enabled;
}

WasmTypedHost& WasmTypedHost::Instance()
{
	static WasmTypedHost instance;
	return instance;
}

WasmTypedHost::~WasmTypedHost()
{
	Unload();
}

bool WasmTypedHost::Load(const std::vector<std::uint8_t>& componentBytes,
	NativeInterface* nativeInterface, bool synced, std::string& error)
{
	if (host != nullptr)
		return true;

	const char* path = std::getenv("SPRING_WASM_TYPED_HOST_LIBRARY");
	if (path == nullptr || *path == '\0') {
		error = "SPRING_WASM_TYPED_HOST is set but SPRING_WASM_TYPED_HOST_LIBRARY is not";
		return false;
	}
	if (library == nullptr) {
		library = dlopen(path, RTLD_NOW | RTLD_LOCAL);
		if (library == nullptr) {
			error = std::string("could not load the Rust Wasm host: ") + dlerror();
			return false;
		}
	}

	// A missing symbol means the library and this engine were built from
	// different revisions of the C ABI; fail loudly rather than dispatching
	// into a null pointer later.
	const auto resolve = [&](auto& target, const char* name) {
		if (error.empty()) {
			void* symbol = dlsym(library, name);
			if (symbol == nullptr)
				error = std::string("the Rust Wasm host is missing ") + name;
			else
				target = reinterpret_cast<std::remove_reference_t<decltype(target)>>(symbol);
		}
	};
	resolve(hostNew, "spring_wasm_typed_host_new");
	resolve(hostFree, "spring_wasm_typed_host_free");
	resolve(stringFree, "spring_wasm_typed_string_free");
	resolve(callinGameFrame, "spring_wasm_typed_callin_game_frame");
	resolve(callinGameFramePost, "spring_wasm_typed_callin_game_frame_post");
	resolve(callinUpdate, "spring_wasm_typed_callin_update");
	resolve(callinUnitCreated, "spring_wasm_typed_callin_unit_created");
	resolve(callinUnitPreDamaged, "spring_wasm_typed_callin_unit_pre_damaged");
	resolve(callinAllowUnitCreation, "spring_wasm_typed_callin_allow_unit_creation");
	if (!error.empty()) {
		Unload();
		return false;
	}

	char* hostError = nullptr;
	host = hostNew(componentBytes.data(), componentBytes.size(), nativeInterface,
		&TypedHostShimTable(), synced, &hostError);
	if (host == nullptr) {
		error = "the Rust Wasm host could not instantiate the component";
		if (hostError != nullptr) {
			error += std::string(": ") + hostError;
			stringFree(hostError);
		}
		return false;
	}
	LOG("Rust Wasm host active: %s", path);
	return true;
}

void WasmTypedHost::Unload()
{
	if (host != nullptr && hostFree != nullptr) {
		hostFree(host);
		host = nullptr;
	}
	if (library != nullptr) {
		dlclose(library);
		library = nullptr;
	}
	hostNew = nullptr;
	hostFree = nullptr;
	stringFree = nullptr;
	callinGameFrame = nullptr;
	callinGameFramePost = nullptr;
	callinUpdate = nullptr;
	callinUnitCreated = nullptr;
	callinUnitPreDamaged = nullptr;
	callinAllowUnitCreation = nullptr;
}

bool WasmTypedHost::DispatchCallin(std::string_view name, const void* query,
	void* result, std::string& error)
{
	if (host == nullptr || query == nullptr)
		return false;

	char* hostError = nullptr;
	std::int32_t status = 0;
	if (name == "GameFrame") {
		const auto* typed = static_cast<const GameFrameQuery*>(query);
		status = callinGameFrame(host, typed->gameFrame, &hostError);
	} else if (name == "GameFramePost") {
		const auto* typed = static_cast<const GameFrameQuery*>(query);
		status = callinGameFramePost(host, typed->gameFrame, &hostError);
	} else if (name == "Update") {
		const auto* typed = static_cast<const UpdateQuery*>(query);
		status = callinUpdate(host, typed->deltaSeconds, &hostError);
	} else if (name == "UnitCreated") {
		const auto* typed = static_cast<const UnitCreatedQuery*>(query);
		status = callinUnitCreated(host, typed->unitID, typed->unitDefID,
			typed->unitTeam, typed->builderID, &hostError);
	} else if (name == "UnitPreDamaged") {
		const auto* typed = static_cast<const UnitDamagedQuery*>(query);
		auto* typedResult = static_cast<DamageCallinResult*>(result);
		float newDamage = typed->damage;
		float impulseMult = 1.0f;
		status = callinUnitPreDamaged(host, typed->unitID, typed->unitDefID,
			typed->unitTeam, typed->damage, typed->paralyzer, typed->weaponDefID,
			typed->projectileID, typed->attackerID, typed->attackerDefID,
			typed->attackerTeam, &newDamage, &impulseMult, &hostError);
		if (status == 0 && typedResult != nullptr) {
			typedResult->newDamage = newDamage;
			typedResult->impulseMult = impulseMult;
		}
	} else if (name == "AllowUnitCreation") {
		const auto* typed = static_cast<const AllowUnitCreationQuery*>(query);
		auto* typedResult = static_cast<AllowUnitCreationResult*>(result);
		bool allow = true;
		bool dropOrder = false;
		status = callinAllowUnitCreation(host, typed->unitDefID, typed->builderID,
			typed->builderTeam, typed->hasBuildInfo, typed->buildPos.x,
			typed->buildPos.y, typed->buildPos.z, typed->buildFacing, &allow,
			&dropOrder, &hostError);
		if (status == 0 && typedResult != nullptr) {
			typedResult->allow = allow;
			typedResult->dropOrder = dropOrder;
		}
	} else {
		// Deliberately scoped to the callins the benchmark table needs; anything
		// else is not handled here and the caller keeps its existing path.
		return false;
	}

	if (hostError != nullptr) {
		error = std::string("Rust Wasm host callin failed: ") + hostError;
		stringFree(hostError);
		return true;
	}
	if (status != 0)
		error = "Rust Wasm host callin returned error " + std::to_string(status);
	return true;
}
