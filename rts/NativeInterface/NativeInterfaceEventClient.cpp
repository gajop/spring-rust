/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "NativeInterfaceEventClient.h"

#include <dlfcn.h>
#include <cstring>

#include "Sim/Features/Feature.h"
#include "Sim/Units/Unit.h"
#include "System/Log/ILog.h"

#define LOAD_SYMBOL(SymbolName)                                                   \
	{                                                                               \
		m_##SymbolName##FuncPtr = reinterpret_cast<fptr::SymbolName##FuncPtr>(      \
				dlsym(m_dllHandle, #SymbolName));                                      \
		if (m_##SymbolName##FuncPtr == nullptr) {                                   \
			LOG_L(L_ERROR, "Failed to load symbol " #SymbolName ": %s", dlerror());   \
		}                                                                            \
	}

NativeInterfaceEventClient::NativeInterfaceEventClient(NativeInterface* nativeInterface, void* dllHandle)
	: CEventClient("[NativeInterfaceEventClient]", 23253, false)
	, m_nativeInterface(nativeInterface)
	, m_dllHandle(dllHandle)
{
}

void NativeInterfaceEventClient::LoadSymbols() {
	LOG("Loading symbols from native module...");

	LOAD_SYMBOL(InitializeNativeModule);
	LOAD_SYMBOL(DownloadFailed);
	LOAD_SYMBOL(DownloadFinished);
	LOAD_SYMBOL(DownloadProgress);
	LOAD_SYMBOL(DownloadQueued);
	LOAD_SYMBOL(DownloadStarted);
	LOAD_SYMBOL(FeatureCreated);
	LOAD_SYMBOL(FeatureDestroyed);
	LOAD_SYMBOL(GameID);
	LOAD_SYMBOL(GamePaused);
	LOAD_SYMBOL(GamePreload);
	LOAD_SYMBOL(GameStart);
	LOAD_SYMBOL(PlayerAdded);
	LOAD_SYMBOL(PlayerChanged);
	LOAD_SYMBOL(PlayerRemoved);
	LOAD_SYMBOL(RenderUnitDestroyed);
	LOAD_SYMBOL(Shutdown);
	LOAD_SYMBOL(TeamChanged);
	LOAD_SYMBOL(TeamDied);
	LOAD_SYMBOL(UnitCreated);
	LOAD_SYMBOL(UnitDestroyed);
	LOAD_SYMBOL(UnitExperience);
	LOAD_SYMBOL(UnitFinished);
	LOAD_SYMBOL(UnitFromFactory);
	LOAD_SYMBOL(UnitGiven);
	LOAD_SYMBOL(UnitLoaded);
	LOAD_SYMBOL(UnitStunned);
	LOAD_SYMBOL(UnitTaken);
	LOAD_SYMBOL(UnitUnloaded);
	LOAD_SYMBOL(HandleLuaMsg);
	LOAD_SYMBOL(HandleLuaCall);
}

void* NativeInterfaceEventClient::Initialize() {
	if (m_InitializeNativeModuleFuncPtr == nullptr) {
		LOG_L(L_ERROR, "InitializeNativeModule function not loaded");
		return nullptr;
	}

	LOG("Initializing native module...");
	InitializeNativeModuleQuery query = {};
	InitializeNativeModuleResult result = {};

	m_InitializeNativeModuleFuncPtr(m_nativeInterface, &query, &result);

	if (result.error != nullptr) {
		LOG_L(L_ERROR, "Failed to initialize native module: %s", result.error->message);
		return nullptr;
	}

	m_moduleData = result.moduleData;
	return m_moduleData;
}

void NativeInterfaceEventClient::GamePreload() {
	if (m_GamePreloadFuncPtr) {
		GamePreloadQuery query = {};
		GamePreloadResult result = {};
		m_GamePreloadFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::GameStart() {
	if (m_GameStartFuncPtr) {
		GameStartQuery query = {};
		GameStartResult result = {};
		m_GameStartFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::GamePaused(int playerID, bool paused) {
	if (m_GamePausedFuncPtr) {
		GamePausedQuery query = {
			.playerID = playerID,
			.paused = paused
		};
		GamePausedResult result = {};
		m_GamePausedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::GameID(const unsigned char* gameID, unsigned int numBytes) {
	if (m_GameIDFuncPtr) {
		GameIDQuery query = {
			.gameID = gameID,
			.numBytes = numBytes
		};
		GameIDResult result = {};
		m_GameIDFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::TeamDied(int teamID) {
	if (m_TeamDiedFuncPtr) {
		TeamDiedQuery query = {.teamID = teamID};
		TeamDiedResult result = {};
		m_TeamDiedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::TeamChanged(int teamID) {
	if (m_TeamChangedFuncPtr) {
		TeamChangedQuery query = {.teamID = teamID};
		TeamChangedResult result = {};
		m_TeamChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::PlayerChanged(int playerID) {
	if (m_PlayerChangedFuncPtr) {
		PlayerChangedQuery query = {.playerID = playerID};
		PlayerChangedResult result = {};
		m_PlayerChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::PlayerAdded(int playerID) {
	if (m_PlayerAddedFuncPtr) {
		PlayerAddedQuery query = {.playerID = playerID};
		PlayerAddedResult result = {};
		m_PlayerAddedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::PlayerRemoved(int playerID, int reason) {
	if (m_PlayerRemovedFuncPtr) {
		PlayerRemovedQuery query = {
			.playerID = playerID,
			.reason = reason
		};
		PlayerRemovedResult result = {};
		m_PlayerRemovedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitCreated(const CUnit* unit, const CUnit* builder) {
	if (m_UnitCreatedFuncPtr) {
		UnitCreatedQuery query = {
			.unitID = unit->id,
			.builderID = builder != nullptr ? builder->id : -1
		};
		UnitCreatedResult result = {};
		m_UnitCreatedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitFinished(const CUnit* unit) {
	if (m_UnitFinishedFuncPtr) {
		UnitFinishedQuery query = {.unitID = unit->id};
		UnitFinishedResult result = {};
		m_UnitFinishedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitFromFactory(const CUnit* unit, const CUnit* factory, bool userOrders) {
	if (m_UnitFromFactoryFuncPtr) {
		UnitFromFactoryQuery query = {
			.unitID = unit->id,
			.factoryID = factory->id,
			.userOrders = userOrders
		};
		UnitFromFactoryResult result = {};
		m_UnitFromFactoryFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitDestroyed(const CUnit* unit, const CUnit* attacker, int weaponDefID) {
	if (m_UnitDestroyedFuncPtr) {
		UnitDestroyedQuery query = {
			.unitID = unit->id,
			.attackerID = attacker != nullptr ? attacker->id : -1
		};
		UnitDestroyedResult result = {};
		m_UnitDestroyedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitTaken(const CUnit* unit, int oldTeam, int newTeam) {
	if (m_UnitTakenFuncPtr) {
		UnitTakenQuery query = {
			.unitID = unit->id,
			.oldTeam = oldTeam,
			.newTeam = newTeam
		};
		UnitTakenResult result = {};
		m_UnitTakenFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitGiven(const CUnit* unit, int oldTeam, int newTeam) {
	if (m_UnitGivenFuncPtr) {
		UnitGivenQuery query = {
			.unitID = unit->id,
			.oldTeam = oldTeam,
			.newTeam = newTeam
		};
		UnitGivenResult result = {};
		m_UnitGivenFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitStunned(const CUnit* unit, bool stunned) {
	if (m_UnitStunnedFuncPtr) {
		UnitStunnedQuery query = {
			.unitID = unit->id,
			.stunned = stunned
		};
		UnitStunnedResult result = {};
		m_UnitStunnedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitExperience(const CUnit* unit, float oldExperience) {
	if (m_UnitExperienceFuncPtr) {
		UnitExperienceQuery query = {
			.unitID = unit->id,
			.oldExperience = oldExperience
		};
		UnitExperienceResult result = {};
		m_UnitExperienceFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitLoaded(const CUnit* unit, const CUnit* transport) {
	if (m_UnitLoadedFuncPtr) {
		UnitLoadedQuery query = {
			.unitID = unit->id,
			.transportID = transport->id
		};
		UnitLoadedResult result = {};
		m_UnitLoadedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitUnloaded(const CUnit* unit, const CUnit* transport) {
	if (m_UnitUnloadedFuncPtr) {
		UnitUnloadedQuery query = {
			.unitID = unit->id,
			.transportID = transport->id
		};
		UnitUnloadedResult result = {};
		m_UnitUnloadedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::RenderUnitDestroyed(const CUnit* unit) {
	if (m_RenderUnitDestroyedFuncPtr) {
		RenderUnitDestroyedQuery query = {.unitID = unit->id};
		RenderUnitDestroyedResult result = {};
		m_RenderUnitDestroyedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::FeatureCreated(const CFeature* feature) {
	if (m_FeatureCreatedFuncPtr) {
		FeatureCreatedQuery query = {.featureID = feature->id};
		FeatureCreatedResult result = {};
		m_FeatureCreatedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::FeatureDestroyed(const CFeature* feature) {
	if (m_FeatureDestroyedFuncPtr) {
		FeatureDestroyedQuery query = {.featureID = feature->id};
		FeatureDestroyedResult result = {};
		m_FeatureDestroyedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::DownloadFailed(int ID, int errorID) {
	if (m_DownloadFailedFuncPtr) {
		DownloadFailedQuery query = {
			.downloadID = ID,
			.errorID = errorID
		};
		DownloadFailedResult result = {};
		m_DownloadFailedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::DownloadFinished(int ID) {
	if (m_DownloadFinishedFuncPtr) {
		DownloadFinishedQuery query = {.downloadID = ID};
		DownloadFinishedResult result = {};
		m_DownloadFinishedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::DownloadProgress(int ID, long downloaded, long total) {
	if (m_DownloadProgressFuncPtr) {
		DownloadProgressQuery query = {
			.downloadID = ID,
			.downloaded = downloaded,
			.total = total
		};
		DownloadProgressResult result = {};
		m_DownloadProgressFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::DownloadQueued(int ID, const std::string& archiveName, const std::string& archiveType) {
	if (m_DownloadQueuedFuncPtr) {
		DownloadQueuedQuery query = {
			.downloadID = ID,
			.archiveName = archiveName.c_str(),
			.archiveType = archiveType.c_str()
		};
		DownloadQueuedResult result = {};
		m_DownloadQueuedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::DownloadStarted(int ID) {
	if (m_DownloadStartedFuncPtr) {
		DownloadStartedQuery query = {.downloadID = ID};
		DownloadStartedResult result = {};
		m_DownloadStartedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::HandleLuaMsg(int playerID, int script, int mode, const std::vector<std::uint8_t>& data) {
	if (m_HandleLuaMsgFuncPtr) {
		HandleLuaMsgQuery query = {
			.playerID = playerID,
			.script = script,
			.mode = mode,
			.data = data.data(),
			.dataLength = static_cast<int32_t>(data.size())
		};
		HandleLuaMsgResult result = {};
		m_HandleLuaMsgFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::HandleLuaCall(const char* msg) {
	if (m_HandleLuaCallFuncPtr) {
		HandleLuaCallQuery query = {.message = msg};
		HandleLuaCallResult result = {};
		m_HandleLuaCallFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}
