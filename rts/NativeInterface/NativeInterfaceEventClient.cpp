/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "NativeInterfaceEventClient.h"

#include <dlfcn.h>

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
	LOAD_SYMBOL(RunDelayedFunctions);
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
	m_moduleData = m_InitializeNativeModuleFuncPtr(m_nativeInterface);
	return m_moduleData;
}

void NativeInterfaceEventClient::GamePreload() {
	if (m_GamePreloadFuncPtr)
		m_GamePreloadFuncPtr(m_nativeInterface, m_moduleData);
}

void NativeInterfaceEventClient::GameStart() {
	if (m_GameStartFuncPtr)
		m_GameStartFuncPtr(m_nativeInterface, m_moduleData);
}

void NativeInterfaceEventClient::GamePaused(int playerID, bool paused) {
	if (m_GamePausedFuncPtr)
		m_GamePausedFuncPtr(m_nativeInterface, m_moduleData, playerID, paused);
}

void NativeInterfaceEventClient::GameID(const unsigned char* gameID, unsigned int numBytes) {
	if (m_GameIDFuncPtr)
		m_GameIDFuncPtr(m_nativeInterface, m_moduleData, gameID, numBytes);
}

void NativeInterfaceEventClient::TeamDied(int teamID) {
	if (m_TeamDiedFuncPtr)
		m_TeamDiedFuncPtr(m_nativeInterface, m_moduleData, teamID);
}

void NativeInterfaceEventClient::TeamChanged(int teamID) {
	if (m_TeamChangedFuncPtr)
		m_TeamChangedFuncPtr(m_nativeInterface, m_moduleData, teamID);
}

void NativeInterfaceEventClient::PlayerChanged(int playerID) {
	if (m_PlayerChangedFuncPtr)
		m_PlayerChangedFuncPtr(m_nativeInterface, m_moduleData, playerID);
}

void NativeInterfaceEventClient::PlayerAdded(int playerID) {
	if (m_PlayerAddedFuncPtr)
		m_PlayerAddedFuncPtr(m_nativeInterface, m_moduleData, playerID);
}

void NativeInterfaceEventClient::PlayerRemoved(int playerID, int reason) {
	if (m_PlayerRemovedFuncPtr)
		m_PlayerRemovedFuncPtr(m_nativeInterface, m_moduleData, playerID, reason);
}

void NativeInterfaceEventClient::UnitCreated(const CUnit* unit, const CUnit* builder) {
	if (m_UnitCreatedFuncPtr)
		m_UnitCreatedFuncPtr(m_nativeInterface, m_moduleData, unit->id, builder != nullptr ? builder->id : -1);
}

void NativeInterfaceEventClient::UnitFinished(const CUnit* unit) {
	if (m_UnitFinishedFuncPtr)
		m_UnitFinishedFuncPtr(m_nativeInterface, m_moduleData, unit->id);
}

void NativeInterfaceEventClient::UnitFromFactory(const CUnit* unit, const CUnit* factory, bool userOrders) {
	if (m_UnitFromFactoryFuncPtr)
		m_UnitFromFactoryFuncPtr(m_nativeInterface, m_moduleData, unit->id, factory->id, userOrders);
}

void NativeInterfaceEventClient::UnitDestroyed(const CUnit* unit, const CUnit* attacker, int weaponDefID) {
	if (m_UnitDestroyedFuncPtr)
		m_UnitDestroyedFuncPtr(m_nativeInterface, m_moduleData, unit->id, attacker != nullptr ? attacker->id : -1);
}

void NativeInterfaceEventClient::UnitTaken(const CUnit* unit, int oldTeam, int newTeam) {
	if (m_UnitTakenFuncPtr)
		m_UnitTakenFuncPtr(m_nativeInterface, m_moduleData, unit->id, oldTeam, newTeam);
}

void NativeInterfaceEventClient::UnitGiven(const CUnit* unit, int oldTeam, int newTeam) {
	if (m_UnitGivenFuncPtr)
		m_UnitGivenFuncPtr(m_nativeInterface, m_moduleData, unit->id, oldTeam, newTeam);
}

void NativeInterfaceEventClient::UnitStunned(const CUnit* unit, bool stunned) {
	if (m_UnitStunnedFuncPtr)
		m_UnitStunnedFuncPtr(m_nativeInterface, m_moduleData, unit->id, stunned);
}

void NativeInterfaceEventClient::UnitExperience(const CUnit* unit, float oldExperience) {
	if (m_UnitExperienceFuncPtr)
		m_UnitExperienceFuncPtr(m_nativeInterface, m_moduleData, unit->id, oldExperience);
}

void NativeInterfaceEventClient::UnitLoaded(const CUnit* unit, const CUnit* transport) {
	if (m_UnitLoadedFuncPtr)
		m_UnitLoadedFuncPtr(m_nativeInterface, m_moduleData, unit->id, transport->id);
}

void NativeInterfaceEventClient::UnitUnloaded(const CUnit* unit, const CUnit* transport) {
	if (m_UnitUnloadedFuncPtr)
		m_UnitUnloadedFuncPtr(m_nativeInterface, m_moduleData, unit->id, transport->id);
}

void NativeInterfaceEventClient::RenderUnitDestroyed(const CUnit* unit) {
	if (m_RenderUnitDestroyedFuncPtr)
		m_RenderUnitDestroyedFuncPtr(m_nativeInterface, m_moduleData, unit->id);
}

void NativeInterfaceEventClient::FeatureCreated(const CFeature* feature) {
	if (m_FeatureCreatedFuncPtr)
		m_FeatureCreatedFuncPtr(m_nativeInterface, m_moduleData, feature->id);
}

void NativeInterfaceEventClient::FeatureDestroyed(const CFeature* feature) {
	if (m_FeatureDestroyedFuncPtr)
		m_FeatureDestroyedFuncPtr(m_nativeInterface, m_moduleData, feature->id);
}

void NativeInterfaceEventClient::DownloadFailed(int ID, int errorID) {
	if (m_DownloadFailedFuncPtr)
		m_DownloadFailedFuncPtr(m_nativeInterface, m_moduleData, ID, errorID);
}

void NativeInterfaceEventClient::DownloadFinished(int ID) {
	if (m_DownloadFinishedFuncPtr)
		m_DownloadFinishedFuncPtr(m_nativeInterface, m_moduleData, ID);
}

void NativeInterfaceEventClient::DownloadProgress(int ID, long downloaded, long total) {
	if (m_DownloadProgressFuncPtr)
		m_DownloadProgressFuncPtr(m_nativeInterface, m_moduleData, ID, downloaded, total);
}

void NativeInterfaceEventClient::DownloadQueued(int ID, const std::string& archiveName, const std::string& archiveType) {
	if (m_DownloadQueuedFuncPtr)
		m_DownloadQueuedFuncPtr(m_nativeInterface, m_moduleData, ID, archiveName.c_str(), archiveType.c_str());
}

void NativeInterfaceEventClient::DownloadStarted(int ID) {
	if (m_DownloadStartedFuncPtr)
		m_DownloadStartedFuncPtr(m_nativeInterface, m_moduleData, ID);
}

void NativeInterfaceEventClient::HandleLuaMsg(int playerID, int script, int mode, const std::vector<std::uint8_t>& data) {
	if (m_HandleLuaMsgFuncPtr)
		m_HandleLuaMsgFuncPtr(m_nativeInterface, m_moduleData, playerID, script, mode, data.data(), data.size());
}

void NativeInterfaceEventClient::HandleLuaCall(const char* msg) {
	if (m_HandleLuaCallFuncPtr)
		m_HandleLuaCallFuncPtr(m_nativeInterface, m_moduleData, msg);
}
