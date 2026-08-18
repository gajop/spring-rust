/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmTypedHost.h"

#include <algorithm>
#include <cstdlib>
#include <memory>

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

SpringTypedHostLibrary& Library()
{
	static SpringTypedHostLibrary library;
	return library;
}

std::vector<std::unique_ptr<WasmTypedHost>>& Hosts()
{
	static std::vector<std::unique_ptr<WasmTypedHost>> hosts;
	return hosts;
}

// Opens the library and resolves every entry point once. A missing symbol means
// the library and this engine were built from different revisions of the C ABI;
// fail loudly rather than dispatching into a null pointer later.
bool OpenLibrary(std::string& error)
{
	SpringTypedHostLibrary& library = Library();
	if (library.handle != nullptr)
		return true;

	const char* path = std::getenv("SPRING_WASM_TYPED_HOST_LIBRARY");
	if (path == nullptr || *path == '\0') {
		error = "SPRING_WASM_TYPED_HOST is set but SPRING_WASM_TYPED_HOST_LIBRARY is not";
		return false;
	}
	void* handle = dlopen(path, RTLD_NOW | RTLD_LOCAL);
	if (handle == nullptr) {
		error = std::string("could not load the typed Wasm host: ") + dlerror();
		return false;
	}

	const auto resolve = [&](auto& target, const char* name) {
		if (error.empty()) {
			void* symbol = dlsym(handle, name);
			if (symbol == nullptr)
				error = std::string("the typed Wasm host is missing ") + name;
			else
				target = reinterpret_cast<std::remove_reference_t<decltype(target)>>(symbol);
		}
	};
	resolve(library.hostNew, "spring_wasm_typed_host_new");
	resolve(library.hostFree, "spring_wasm_typed_host_free");
	resolve(library.stringFree, "spring_wasm_typed_string_free");
	resolve(library.callinGameFrame, "spring_wasm_typed_callin_game_frame");
	resolve(library.callinGameFramePost, "spring_wasm_typed_callin_game_frame_post");
	resolve(library.callinUpdate, "spring_wasm_typed_callin_update");
	resolve(library.callinUnitCreated, "spring_wasm_typed_callin_unit_created");
	resolve(library.callinUnitPreDamaged, "spring_wasm_typed_callin_unit_pre_damaged");
	resolve(library.callinAllowUnitCreation, "spring_wasm_typed_callin_allow_unit_creation");
	if (!error.empty()) {
		dlclose(handle);
		library = SpringTypedHostLibrary{};
		return false;
	}

	library.handle = handle;
	LOG("typed Wasm host active: %s", path);
	return true;
}

} // namespace

bool WasmTypedHost::Enabled()
{
	static const bool enabled = TruthyEnvironment("SPRING_WASM_TYPED_HOST");
	return enabled;
}

WasmTypedHost::~WasmTypedHost()
{
	if (host != nullptr && Library().hostFree != nullptr)
		Library().hostFree(host);
}

bool WasmTypedHost::Load(std::string moduleName,
	const std::vector<std::uint8_t>& componentBytes, NativeInterface* nativeInterface,
	bool synced, std::string& error)
{
	if (!OpenLibrary(error))
		return false;

	// Reloading a module replaces its host rather than accumulating one per
	// load, matching how the module list treats a duplicate name.
	Unload(moduleName);

	char* hostError = nullptr;
	void* host = Library().hostNew(componentBytes.data(), componentBytes.size(),
		nativeInterface, &TypedHostShimTable(), synced, &hostError);
	if (host == nullptr) {
		error = "the typed Wasm host could not instantiate the component";
		if (hostError != nullptr) {
			error += std::string(": ") + hostError;
			Library().stringFree(hostError);
		}
		return false;
	}

	Hosts().emplace_back(new WasmTypedHost(std::move(moduleName), host));
	return true;
}

void WasmTypedHost::Unload(std::string_view moduleName)
{
	auto& hosts = Hosts();
	hosts.erase(std::remove_if(hosts.begin(), hosts.end(), [moduleName](const auto& host) {
		return host->moduleName == moduleName;
	}), hosts.end());
}

void WasmTypedHost::UnloadAll()
{
	Hosts().clear();

	SpringTypedHostLibrary& library = Library();
	if (library.handle != nullptr)
		dlclose(library.handle);
	library = SpringTypedHostLibrary{};
}

bool WasmTypedHost::AnyActive()
{
	return !Hosts().empty();
}

bool WasmTypedHost::DispatchCallin(std::string_view name, const void* query, void* result,
	std::string& error)
{
	if (query == nullptr)
		return false;

	bool handled = false;
	for (const auto& host : Hosts()) {
		std::string hostError;
		if (!host->Invoke(name, query, result, hostError))
			return false;
		handled = true;
		// Keep fanning out on failure so one broken module does not silently
		// hide the others, but report the first error to the caller.
		if (!hostError.empty() && error.empty())
			error = hostError;
	}
	return handled;
}

bool WasmTypedHost::Invoke(std::string_view name, const void* query, void* result,
	std::string& error) const
{
	const SpringTypedHostLibrary& library = Library();

	char* hostError = nullptr;
	std::int32_t status = 0;
	if (name == "GameFrame") {
		const auto* typed = static_cast<const GameFrameQuery*>(query);
		status = library.callinGameFrame(host, typed->gameFrame, &hostError);
	} else if (name == "GameFramePost") {
		const auto* typed = static_cast<const GameFrameQuery*>(query);
		status = library.callinGameFramePost(host, typed->gameFrame, &hostError);
	} else if (name == "Update") {
		const auto* typed = static_cast<const UpdateQuery*>(query);
		status = library.callinUpdate(host, typed->deltaSeconds, &hostError);
	} else if (name == "UnitCreated") {
		const auto* typed = static_cast<const UnitCreatedQuery*>(query);
		status = library.callinUnitCreated(host, typed->unitID, typed->unitDefID,
			typed->unitTeam, typed->builderID, &hostError);
	} else if (name == "UnitPreDamaged") {
		const auto* typed = static_cast<const UnitDamagedQuery*>(query);
		auto* typedResult = static_cast<DamageCallinResult*>(result);
		float newDamage = typed->damage;
		float impulseMult = 1.0f;
		status = library.callinUnitPreDamaged(host, typed->unitID, typed->unitDefID,
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
		status = library.callinAllowUnitCreation(host, typed->unitDefID, typed->builderID,
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
		error = std::string("typed Wasm host callin failed: ") + hostError;
		library.stringFree(hostError);
		return true;
	}
	if (status != 0)
		error = "typed Wasm host callin returned error " + std::to_string(status);
	return true;
}
