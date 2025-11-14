#pragma once

#include <string>
#include <vector>

#include "System/EventClient.h"
#include "NativeInterface.h"

#include "System/Log/ILog.h"


class CUnit;
class CWeapon;
class CFeature;
class CProjectile;
struct Command;
struct SRectangle;

namespace fptr {
  // Native moduel initialization
  using InitializeNativeModuleFuncPtr = void* (*)(NativeInterface*);
  // Spring callins
  using DownloadFailedFuncPtr = void(*)(NativeInterface*,void*,int, int);
  using DownloadFinishedFuncPtr = void(*)(NativeInterface*,void*,int);
  using DownloadProgressFuncPtr = void(*)(NativeInterface*,void*,int, long, long);
  using DownloadQueuedFuncPtr = void(*)(NativeInterface*,void*,int, const char*, const char*);
  using DownloadStartedFuncPtr = void(*)(NativeInterface*,void*,int);
  using FeatureCreatedFuncPtr = void(*)(NativeInterface*,void*,int);
  using FeatureDestroyedFuncPtr = void(*)(NativeInterface*,void*,int);
  using GameIDFuncPtr = void(*)(NativeInterface*,void*,const unsigned char*, unsigned int);
  using GamePausedFuncPtr = void(*)(NativeInterface*,void*,int, bool);
  using GamePreloadFuncPtr = void(*)(void*);
  using GameStartFuncPtr = void(*)(void*);
  using PlayerAddedFuncPtr = void(*)(NativeInterface*,void*,int);
  using PlayerChangedFuncPtr = void(*)(NativeInterface*,void*,int);
  using PlayerRemovedFuncPtr = void(*)(NativeInterface*,void*,int, int);
  using RenderUnitDestroyedFuncPtr = void(*)(NativeInterface*,void*,int);
  using RunDelayedFunctionsFuncPtr = void(*)(NativeInterface*,void*,int);
  using ShutdownFuncPtr = void(*)(void*);
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


class RustSystem : public CEventClient, public NativeInterfaceBridge
{
public:
  RustSystem();

  void Reload();

  bool WantsEvent(const std::string &eventName) override
  {
    return true;
  }
  bool GetFullRead() const override { return true; }
  int GetReadAllyTeam() const override { return AllAccessTeam; }

  /**
   * @name Synced_events
   * @{
   */

  void GamePreload() override;
  void GameStart() override;
  // void GameOver(const std::vector<unsigned char>& winningAllyTeams) override;
  void GamePaused(int playerID, bool paused) override;
  // void GameFrame(int gameFrame) override;
  // void GameFramePost(int gameFrame) override;
  void GameID(const unsigned char* gameID, unsigned int numBytes) override;

  void TeamDied(int teamID) override;
  void TeamChanged(int teamID) override;
  void PlayerChanged(int playerID) override;
  void PlayerAdded(int playerID) override;
  void PlayerRemoved(int playerID, int reason) override;

  void UnitCreated(const CUnit* unit, const CUnit* builder) override;
  void UnitFinished(const CUnit *unit) override;
  // void UnitReverseBuilt(const CUnit *unit) override;
  void UnitFromFactory(const CUnit* unit, const CUnit* factory, bool userOrders) override;
  void UnitDestroyed(const CUnit* unit, const CUnit* attacker, int weaponDefID) override;
  void UnitTaken(const CUnit* unit, int oldTeam, int newTeam) override;
  void UnitGiven(const CUnit* unit, int oldTeam, int newTeam) override;

  // void UnitIdle(const CUnit* unit) override;
  // void UnitCommand(const CUnit* unit, const Command& command, int playerNum, bool fromSynced, bool fromLua) override;
  // void UnitCmdDone(const CUnit* unit, const Command& command                                              ) override;
  // void UnitDamaged(
  //   const CUnit* unit,
  //   const CUnit* attacker,
  //   float damage,
  //   int weaponDefID,
  //   int projectileID,
  //   bool paralyzer) override;
  void UnitStunned(const CUnit* unit, bool stunned) override;
  void UnitExperience(const CUnit* unit, float oldExperience) override;
  // void UnitHarvestStorageFull(const CUnit* unit) override;

  // void UnitSeismicPing(const CUnit* unit, int allyTeam,
  //                                const float3& pos, float strength) override;
  // void UnitEnteredRadar(const CUnit* unit, int allyTeam) override;
  // void UnitEnteredLos(const CUnit* unit, int allyTeam) override;
  // void UnitLeftRadar(const CUnit* unit, int allyTeam) override;
  // void UnitLeftLos(const CUnit* unit, int allyTeam) override;

  // void UnitEnteredUnderwater(const CUnit* unit) override;
  // void UnitEnteredWater(const CUnit* unit) override;
  // void UnitEnteredAir(const CUnit* unit) override;
  // void UnitLeftUnderwater(const CUnit* unit) override;
  // void UnitLeftWater(const CUnit* unit) override;
  // void UnitLeftAir(const CUnit* unit) override;

  void UnitLoaded(const CUnit* unit, const CUnit* transport) override;
  void UnitUnloaded(const CUnit* unit, const CUnit* transport) override;

  // void UnitCloaked(const CUnit* unit) override;
  // void UnitDecloaked(const CUnit* unit) override;

  // void RenderUnitPreCreated(const CUnit* unit) override;
  // void RenderUnitCreated(const CUnit* unit, int cloaked) override;
  void RenderUnitDestroyed(const CUnit* unit) override;

  // bool UnitUnitCollision(const CUnit* collider, const CUnit* collidee) { return false; }
  // bool UnitFeatureCollision(const CUnit* collider, const CFeature* collidee) { return false; }
  // void UnitMoved(const CUnit* unit) override;
  // void UnitMoveFailed(const CUnit* unit) override;

  void FeatureCreated(const CFeature* feature) override;
  void FeatureDestroyed(const CFeature* feature) override;
  // void FeatureDamaged(
  //   const CFeature* feature,
  //   const CUnit* attacker,
  //   float damage,
  //   int weaponDefID,
  //   int projectileID) override;
  // void FeatureMoved(const CFeature* feature, const float3& oldpos) override;

  // void RenderFeaturePreCreated(const CFeature* feature) override;
  // void RenderFeatureCreated(const CFeature* feature) override;
  // void RenderFeatureDestroyed(const CFeature* feature) override;

  // void ProjectileCreated(const CProjectile* proj) override;
  // void ProjectileDestroyed(const CProjectile* proj) override;

  // void RenderProjectileCreated(const CProjectile* proj) override;
  // void RenderProjectileDestroyed(const CProjectile* proj) override;

  // void StockpileChanged(const CUnit* unit,
  //                               const CWeapon* weapon, int oldCount) override;

  // bool Explosion(int weaponID, int projectileID, const float3& pos, const CUnit* owner) override  { return false; }


  // bool CommandFallback(const CUnit* unit, const Command& cmd) override  { return false; }
  // bool AllowCommand(const CUnit* unit, const Command& cmd, int playerNum, bool fromSynced, bool fromLua) override  { return true; }

  // std::pair <bool, bool> AllowUnitCreation(const UnitDef* unitDef, const CUnit* builder, const BuildInfo* buildInfo) override  { return {true, true}; }
  // bool AllowUnitTransfer(const CUnit* unit, int newTeam, bool capture) override { return true; }
  // bool AllowUnitBuildStep(const CUnit* builder, const CUnit* unit, float part) override { return true; }
  // bool AllowUnitCaptureStep(const CUnit* builder, const CUnit* unit, float part) override { return true; }
  // bool AllowUnitTransport(const CUnit* transporter, const CUnit* transportee) override  { return true; }
  // bool AllowUnitTransportLoad(const CUnit* transporter, const CUnit* transportee, const float3& loadPos, bool allowed) override  { return true; }
  // bool AllowUnitTransportUnload(const CUnit* transporter, const CUnit* transportee, const float3& unloadPos, bool allowed) override { return true; }
  // bool AllowUnitCloak(const CUnit* unit, const CUnit* enemy) override  { return true; }
  // bool AllowUnitDecloak(const CUnit* unit, const CSolidObject* object, const CWeapon* weapon) override { return true; }
  // bool AllowUnitKamikaze(const CUnit* unit, const CUnit* target, bool allowed) override  { return true; }
  // bool AllowFeatureCreation(const FeatureDef* featureDef, int allyTeamID, const float3& pos) override  { return true; }
  // bool AllowFeatureBuildStep(const CUnit* builder, const CFeature* feature, float part) override { return true; }
  // bool AllowResourceLevel(int teamID, const string& type, float level) override { return true; }
  // bool AllowResourceTransfer(int oldTeam, int newTeam, const char* type, float amount)override  { return true; }
  // bool AllowDirectUnitControl(int playerID, const CUnit* unit) override { return true; }
  // bool AllowBuilderHoldFire(const CUnit* unit, int action) override { return true; }
  // bool AllowStartPosition(int playerID, int teamID, unsigned char readyState, const float3& clampedPos, const float3& rawPickPos) override { return true; }

  // bool TerraformComplete(const CUnit* unit, const CUnit* build)override  { return false; }
  // bool MoveCtrlNotify(const CUnit* unit, int data) override { return false; }

  // int AllowWeaponTargetCheck(unsigned int attackerID, unsigned int attackerWeaponNum, unsigned int attackerWeaponDefID) override { return -1; }
  // bool AllowWeaponTarget(
  //   unsigned int attackerID,
  //   unsigned int targetID,
  //   unsigned int attackerWeaponNum,
  //   unsigned int attackerWeaponDefID,
  //   float* targetPriority
  // ) override { return true; }
  // bool AllowWeaponInterceptTarget(const CUnit* interceptorUnit, const CWeapon* interceptorWeapon, const CProjectile* interceptorTarget) override { return true; }

  // bool UnitPreDamaged(
  //   const CUnit* unit,
  //   const CUnit* attacker,
  //   float damage,
  //   int weaponDefID,
  //   int projectileID,
  //   bool paralyzer,
  //   float* newDamage,
  //   float* impulseMult
  // ) override { return false; }

  // bool FeaturePreDamaged(
  //   const CFeature* feature,
  //   const CUnit* attacker,
  //   float damage,
  //   int weaponDefID,
  //   int projectileID,
  //   float* newDamage,
  //   float* impulseMult
  // ) override { return false; }

  // bool ShieldPreDamaged(
  //   const CProjectile* projectile,
  //   const CWeapon* shieldEmitter,
  //   const CUnit* shieldCarrier,
  //   bool bounceProjectile,
  //   const CWeapon* beamEmitter,
  //   const CUnit* beamCarrier,
  //   const float3& startPos,
  //   const float3& hitPos
  // ) override { return false; }

  // bool SyncedActionFallback(const string& line, int playerID) override { return false; }


public:
  void HandleRustPanic() override;

// Unsyced Event START
public:
  void DownloadFailed(int ID, int errorID) override;
  void DownloadFinished(int ID) override;
  void DownloadProgress(int ID, long downloaded, long total) override;
  void DownloadQueued(int ID, const std::string& archiveName, const std::string& archiveType) override;
  void DownloadStarted(int ID) override;
// Unsynced Event END

// Special
public:
  void HandleLuaMsg(int playerID, int script, int mode, const std::vector<std::uint8_t>& data);
  void HandleLuaCall(const char* msg);

// SyncedRead START
public:
  IsPosInMapReturn IsPosInMap(float x, float z) override;
  float GetGroundHeight(float x, float z) override;
  float GetGroundOrigHeight(float x, float z) override;
  GetGroundNormalReturn GetGroundNormal(const float x, const float z, bool *normal) override;
  GetGroundInfoReturn GetGroundInfo(float x, float z) override;
  bool GetGroundBlocked(float fx1, float fz1, const float *fx2, const float *fz2) override;
  GetGroundExtremesReturn GetGroundExtremes() override;
  GetTerrainTypeDataReturn GetTerrainTypeData(int terrainTypeInfo) override;
  float GetGrass(float x, float z) override;
  float GetSmoothMeshHeight(float x, float z) override;
// SyncedRead END

// SyncedCtrl START
public:
  float AddHeightMap(float xl, float zl, float height) override;
  float SetHeightMap(float xl, float zl, float height, float* scalingFactor) override;
  float SetHeightMapFunc(void (*function)(void* data), void* data) override;

  void RevertHeightMap(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height, float origFactor) override;
  // void RevertOriginalHeightMap(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float origFactor) override;

  // New API?
  // void SetHeightMap(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height) override;
  // void AddHeightMap(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height) override;
  // void SetHeightMapArray(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height) override;
  // void AddHeightMapArray(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height) override;
  // void SetOriginalHeightMap(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height) override;
  // void AddOriginalHeightMap(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height) override;
  // void SetOriginalHeightMapArray(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height) override;
  // void AddOriginalHeightMapArray(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height) override;

  void SetSmoothMesh(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height) override;
  void AddSmoothMesh(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height) override;
  void RevertSmoothMesh(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float origFactor) override;
  void SetSmoothMeshArray(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height) override;
  void AddSmoothMeshArray(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height) override;

  void SetMapSquareTerrainType(uint32_t x, uint32_t z, uint32_t newType) override;
  bool SetTerrainTypeData(uint32_t typeIndex, float* tankSpeed, float* kbotSpeed, float* hoverSpeed, float* shipSpeed,
    float* hardness, bool* receiveTracks, const char* name) override;

// SyncedCtrl END

// TODO: Should figure out what stuff to add
// UnsyncedCtrl START
// Adding a only a couple of things for now...
	void Echo(const char* msg) const;
// UnsyncedCtrl END


// Game constants STAT
  float GameMapX() const;
  float GameMapY() const;
  float GameMapSizeX() const;
  float GameMapSizeZ() const;
// Game constants END

public:
  static RustSystem* s_instance;

private:
  template<typename Func, typename... Args>
  auto CallNativeFunction(Func func, const char* funcName, Args&&... args)
      -> std::enable_if_t<std::is_member_function_pointer_v<Func>, void>
  {
      LOG("NativeModule::%s", funcName);
      (this->*func)(&m_NativeInterface, std::forward<Args>(args)...);
  }

  struct HeightMapChange {
    int x1 = 0;
    int x2 = 0;
    int z1 = 0;
    int z2 = 0;
    float amountChanged = 0.0f;
    bool inHeightMap = false;
  };
  HeightMapChange m_heightMapChange = {};

private:
  NativeInterface m_NativeInterface;
  void *m_handle = nullptr;
  void *m_data = nullptr;

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
