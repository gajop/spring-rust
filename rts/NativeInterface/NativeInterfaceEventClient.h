/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <string>
#include <vector>

#include "NativeInterface.h"
#include "System/EventClient.h"

class CUnit;
class CFeature;

/**
 * Function pointer types for native module callbacks
 */
namespace fptr {
	using InitializeNativeModuleFuncPtr = void* (*)(NativeInterface*);
	using DownloadFailedFuncPtr = void(*)(NativeInterface*,void*,int, int);
	using DownloadFinishedFuncPtr = void(*)(NativeInterface*,void*,int);
	using DownloadProgressFuncPtr = void(*)(NativeInterface*,void*,int, long, long);
	using DownloadQueuedFuncPtr = void(*)(NativeInterface*,void*,int, const char*, const char*);
	using DownloadStartedFuncPtr = void(*)(NativeInterface*,void*,int);
	using FeatureCreatedFuncPtr = void(*)(NativeInterface*,void*,int);
	using FeatureDestroyedFuncPtr = void(*)(NativeInterface*,void*,int);
	using GameIDFuncPtr = void(*)(NativeInterface*,void*,const unsigned char*, unsigned int);
	using GamePausedFuncPtr = void(*)(NativeInterface*,void*,int, bool);
	using GamePreloadFuncPtr = void(*)(NativeInterface*, void*);
	using GameStartFuncPtr = void(*)(NativeInterface*, void*);
	using PlayerAddedFuncPtr = void(*)(NativeInterface*,void*,int);
	using PlayerChangedFuncPtr = void(*)(NativeInterface*,void*,int);
	using PlayerRemovedFuncPtr = void(*)(NativeInterface*,void*,int, int);
	using RenderUnitDestroyedFuncPtr = void(*)(NativeInterface*,void*,int);
	using RunDelayedFunctionsFuncPtr = void(*)(NativeInterface*,void*,int);
	using ShutdownFuncPtr = void(*)(NativeInterface*, void*);
	using TeamChangedFuncPtr = void(*)(NativeInterface*,void*,int);
	using TeamDiedFuncPtr = void(*)(NativeInterface*,void*,int);
	using UnitCreatedFuncPtr = void(*)(NativeInterface*,void*,int, int);
	using UnitDestroyedFuncPtr = void(*)(NativeInterface*,void*,int, int);
	using UnitExperienceFuncPtr = void(*)(NativeInterface*,void*,int, float);
	using UnitFinishedFuncPtr = void(*)(NativeInterface*,void*,int);
	using UnitFromFactoryFuncPtr = void(*)(NativeInterface*,void*,int, int, bool);
	using UnitGivenFuncPtr = void(*)(NativeInterface*,void*,int, int, int);
	using UnitLoadedFuncPtr = void(*)(NativeInterface*,void*,int, int);
	using UnitStunnedFuncPtr = void(*)(NativeInterface*,void*,int, bool);
	using UnitTakenFuncPtr = void(*)(NativeInterface*,void*,int, int, int);
	using UnitUnloadedFuncPtr = void(*)(NativeInterface*,void*,int, int);
	using HandleLuaMsgFuncPtr = void(*)(NativeInterface*,void*,int,int,int,const std::uint8_t*, int);
	using HandleLuaCallFuncPtr = void(*)(NativeInterface*,void*,const char*);
}

/**
 * NativeInterfaceEventClient - Handles DLL integration and Spring event callbacks
 *
 * This class:
 * - Inherits from CEventClient to receive Spring engine events
 * - Manages symbol loading from native module DLL
 * - Stores function pointers to native module callbacks
 * - Dispatches Spring events to native module
 *
 * Does NOT handle DLL loading - receives handle from NativeInterfaceSystem
 */
class NativeInterfaceEventClient : public CEventClient {
public:
	NativeInterfaceEventClient(NativeInterface* nativeInterface, void* dllHandle);

	// Load symbols from DLL
	void LoadSymbols();

	// Initialize the native module
	void* Initialize();

	// CEventClient interface
	bool WantsEvent(const std::string& eventName) override {
		return true;
	}
	bool GetFullRead() const override { return true; }
	int GetReadAllyTeam() const override { return AllAccessTeam; }

	// Synced events
	void GamePreload() override;
	void GameStart() override;
	void GamePaused(int playerID, bool paused) override;
	void GameID(const unsigned char* gameID, unsigned int numBytes) override;

	void TeamDied(int teamID) override;
	void TeamChanged(int teamID) override;
	void PlayerChanged(int playerID) override;
	void PlayerAdded(int playerID) override;
	void PlayerRemoved(int playerID, int reason) override;

	void UnitCreated(const CUnit* unit, const CUnit* builder) override;
	void UnitFinished(const CUnit* unit) override;
	void UnitFromFactory(const CUnit* unit, const CUnit* factory, bool userOrders) override;
	void UnitDestroyed(const CUnit* unit, const CUnit* attacker, int weaponDefID) override;
	void UnitTaken(const CUnit* unit, int oldTeam, int newTeam) override;
	void UnitGiven(const CUnit* unit, int oldTeam, int newTeam) override;
	void UnitStunned(const CUnit* unit, bool stunned) override;
	void UnitExperience(const CUnit* unit, float oldExperience) override;
	void UnitLoaded(const CUnit* unit, const CUnit* transport) override;
	void UnitUnloaded(const CUnit* unit, const CUnit* transport) override;
	void RenderUnitDestroyed(const CUnit* unit) override;

	void FeatureCreated(const CFeature* feature) override;
	void FeatureDestroyed(const CFeature* feature) override;

	// Unsynced events
	void DownloadFailed(int ID, int errorID) override;
	void DownloadFinished(int ID) override;
	void DownloadProgress(int ID, long downloaded, long total) override;
	void DownloadQueued(int ID, const std::string& archiveName, const std::string& archiveType) override;
	void DownloadStarted(int ID) override;

	// Special events (not part of CEventClient)
	void HandleLuaMsg(int playerID, int script, int mode, const std::vector<std::uint8_t>& data);
	void HandleLuaCall(const char* msg);

private:
	NativeInterface* m_nativeInterface;
	void* m_dllHandle;
	void* m_moduleData = nullptr;

	// Function pointers to native module
	fptr::InitializeNativeModuleFuncPtr m_InitializeNativeModuleFuncPtr = nullptr;
	fptr::DownloadFailedFuncPtr m_DownloadFailedFuncPtr = nullptr;
	fptr::DownloadFinishedFuncPtr m_DownloadFinishedFuncPtr = nullptr;
	fptr::DownloadProgressFuncPtr m_DownloadProgressFuncPtr = nullptr;
	fptr::DownloadQueuedFuncPtr m_DownloadQueuedFuncPtr = nullptr;
	fptr::DownloadStartedFuncPtr m_DownloadStartedFuncPtr = nullptr;
	fptr::FeatureCreatedFuncPtr m_FeatureCreatedFuncPtr = nullptr;
	fptr::FeatureDestroyedFuncPtr m_FeatureDestroyedFuncPtr = nullptr;
	fptr::GameIDFuncPtr m_GameIDFuncPtr = nullptr;
	fptr::GamePausedFuncPtr m_GamePausedFuncPtr = nullptr;
	fptr::GamePreloadFuncPtr m_GamePreloadFuncPtr = nullptr;
	fptr::GameStartFuncPtr m_GameStartFuncPtr = nullptr;
	fptr::PlayerAddedFuncPtr m_PlayerAddedFuncPtr = nullptr;
	fptr::PlayerChangedFuncPtr m_PlayerChangedFuncPtr = nullptr;
	fptr::PlayerRemovedFuncPtr m_PlayerRemovedFuncPtr = nullptr;
	fptr::RenderUnitDestroyedFuncPtr m_RenderUnitDestroyedFuncPtr = nullptr;
	fptr::RunDelayedFunctionsFuncPtr m_RunDelayedFunctionsFuncPtr = nullptr;
	fptr::ShutdownFuncPtr m_ShutdownFuncPtr = nullptr;
	fptr::TeamChangedFuncPtr m_TeamChangedFuncPtr = nullptr;
	fptr::TeamDiedFuncPtr m_TeamDiedFuncPtr = nullptr;
	fptr::UnitCreatedFuncPtr m_UnitCreatedFuncPtr = nullptr;
	fptr::UnitDestroyedFuncPtr m_UnitDestroyedFuncPtr = nullptr;
	fptr::UnitExperienceFuncPtr m_UnitExperienceFuncPtr = nullptr;
	fptr::UnitFinishedFuncPtr m_UnitFinishedFuncPtr = nullptr;
	fptr::UnitFromFactoryFuncPtr m_UnitFromFactoryFuncPtr = nullptr;
	fptr::UnitGivenFuncPtr m_UnitGivenFuncPtr = nullptr;
	fptr::UnitLoadedFuncPtr m_UnitLoadedFuncPtr = nullptr;
	fptr::UnitStunnedFuncPtr m_UnitStunnedFuncPtr = nullptr;
	fptr::UnitTakenFuncPtr m_UnitTakenFuncPtr = nullptr;
	fptr::UnitUnloadedFuncPtr m_UnitUnloadedFuncPtr = nullptr;
	fptr::HandleLuaMsgFuncPtr m_HandleLuaMsgFuncPtr = nullptr;
	fptr::HandleLuaCallFuncPtr m_HandleLuaCallFuncPtr = nullptr;
};
