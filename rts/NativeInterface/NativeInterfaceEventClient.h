/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstddef>
#include <cstdint>
#include <string>
#include <vector>

#include "NativeInterface.h"
#include "NativeInterface/api/Callins.h"
#include "System/EventClient.h"

class CUnit;
class CFeature;
class CProjectile;
class CWeapon;
struct CExplosionParams;
class WeaponDef;
struct Command;
struct SCommandDescription;
struct SRectangle;
class SharedLib;

/**
 * Function pointer types for native module callbacks (using Query/Result pattern)
 */
namespace fptr {
	using InitializeNativeModuleFuncPtr = void(*)(NativeInterface*, const InitializeNativeModuleQuery*, InitializeNativeModuleResult*);
	using DownloadFailedFuncPtr = void(*)(NativeInterface*, void*, const DownloadFailedQuery*, DownloadFailedResult*);
	using DownloadFinishedFuncPtr = void(*)(NativeInterface*, void*, const DownloadFinishedQuery*, DownloadFinishedResult*);
	using DownloadProgressFuncPtr = void(*)(NativeInterface*, void*, const DownloadProgressQuery*, DownloadProgressResult*);
	using DownloadQueuedFuncPtr = void(*)(NativeInterface*, void*, const DownloadQueuedQuery*, DownloadQueuedResult*);
	using DownloadStartedFuncPtr = void(*)(NativeInterface*, void*, const DownloadStartedQuery*, DownloadStartedResult*);
	using FeatureCreatedFuncPtr = void(*)(NativeInterface*, void*, const FeatureCreatedQuery*, FeatureCreatedResult*);
	using FeatureDestroyedFuncPtr = void(*)(NativeInterface*, void*, const FeatureDestroyedQuery*, FeatureDestroyedResult*);
	using GameIDFuncPtr = void(*)(NativeInterface*, void*, const GameIDQuery*, GameIDResult*);
	using GamePausedFuncPtr = void(*)(NativeInterface*, void*, const GamePausedQuery*, GamePausedResult*);
	using GamePreloadFuncPtr = void(*)(NativeInterface*, void*, const GamePreloadQuery*, GamePreloadResult*);
	using GameStartFuncPtr = void(*)(NativeInterface*, void*, const GameStartQuery*, GameStartResult*);
	using GameOverFuncPtr = void(*)(NativeInterface*, void*, const GameOverEventQuery*, GameOverEventResult*);
	using GameFrameFuncPtr = void(*)(NativeInterface*, void*, const GameFrameQuery*, GameFrameResult*);
	using GameFramePostFuncPtr = void(*)(NativeInterface*, void*, const GameFramePostQuery*, GameFramePostResult*);
	using PlayerAddedFuncPtr = void(*)(NativeInterface*, void*, const PlayerAddedQuery*, PlayerAddedResult*);
	using PlayerChangedFuncPtr = void(*)(NativeInterface*, void*, const PlayerChangedQuery*, PlayerChangedResult*);
	using PlayerRemovedFuncPtr = void(*)(NativeInterface*, void*, const PlayerRemovedQuery*, PlayerRemovedResult*);
	using RenderUnitDestroyedFuncPtr = void(*)(NativeInterface*, void*, const RenderUnitDestroyedQuery*, RenderUnitDestroyedResult*);
	using ShutdownFuncPtr = void(*)(NativeInterface*, void*, const ShutdownQuery*, ShutdownResult*);
	using TeamChangedFuncPtr = void(*)(NativeInterface*, void*, const TeamChangedQuery*, TeamChangedResult*);
	using TeamDiedFuncPtr = void(*)(NativeInterface*, void*, const TeamDiedQuery*, TeamDiedResult*);
	using UnitCreatedFuncPtr = void(*)(NativeInterface*, void*, const UnitCreatedQuery*, UnitCreatedResult*);
	using UnitDestroyedFuncPtr = void(*)(NativeInterface*, void*, const UnitDestroyedQuery*, UnitDestroyedResult*);
	using UnitExperienceFuncPtr = void(*)(NativeInterface*, void*, const UnitExperienceQuery*, UnitExperienceResult*);
	using UnitFinishedFuncPtr = void(*)(NativeInterface*, void*, const UnitFinishedQuery*, UnitFinishedResult*);
	using UnitReverseBuiltFuncPtr = void(*)(NativeInterface*, void*, const UnitReverseBuiltQuery*, UnitReverseBuiltResult*);
	using UnitConstructionDecayedFuncPtr = void(*)(NativeInterface*, void*, const UnitConstructionDecayedQuery*, UnitConstructionDecayedResult*);
	using UnitFromFactoryFuncPtr = void(*)(NativeInterface*, void*, const UnitFromFactoryQuery*, UnitFromFactoryResult*);
	using UnitGivenFuncPtr = void(*)(NativeInterface*, void*, const UnitGivenQuery*, UnitGivenResult*);
	using UnitIdleFuncPtr = void(*)(NativeInterface*, void*, const UnitIdleQuery*, UnitIdleResult*);
	using UnitCommandFuncPtr = void(*)(NativeInterface*, void*, const UnitCommandQuery*, UnitCommandResult*);
	using CommandFallbackFuncPtr = void(*)(NativeInterface*, void*, const CommandFallbackQuery*, BoolCallinResult*);
	using AllowCommandFuncPtr = void(*)(NativeInterface*, void*, const UnitCommandQuery*, BoolCallinResult*);
	using AllowUnitCreationFuncPtr = void(*)(NativeInterface*, void*, const AllowUnitCreationQuery*, AllowUnitCreationResult*);
	using AllowUnitTransferFuncPtr = void(*)(NativeInterface*, void*, const AllowUnitTransferQuery*, BoolCallinResult*);
	using AllowUnitBuildStepFuncPtr = void(*)(NativeInterface*, void*, const AllowUnitBuildStepQuery*, BoolCallinResult*);
	using AllowUnitCaptureStepFuncPtr = AllowUnitBuildStepFuncPtr;
	using AllowUnitTransportFuncPtr = void(*)(NativeInterface*, void*, const AllowUnitTransportQuery*, BoolCallinResult*);
	using AllowUnitTransportPositionFuncPtr = void(*)(NativeInterface*, void*, const AllowUnitTransportPositionQuery*, BoolCallinResult*);
	using AllowUnitTransportLoadFuncPtr = AllowUnitTransportPositionFuncPtr;
	using AllowUnitTransportUnloadFuncPtr = AllowUnitTransportPositionFuncPtr;
	using AllowUnitCloakFuncPtr = void(*)(NativeInterface*, void*, const AllowUnitCloakQuery*, BoolCallinResult*);
	using AllowUnitDecloakFuncPtr = void(*)(NativeInterface*, void*, const AllowUnitDecloakQuery*, BoolCallinResult*);
	using AllowUnitKamikazeFuncPtr = void(*)(NativeInterface*, void*, const AllowUnitKamikazeQuery*, BoolCallinResult*);
	using AllowFeatureCreationFuncPtr = void(*)(NativeInterface*, void*, const AllowFeatureCreationQuery*, BoolCallinResult*);
	using AllowFeatureBuildStepFuncPtr = void(*)(NativeInterface*, void*, const AllowFeatureBuildStepQuery*, BoolCallinResult*);
	using AllowResourceLevelFuncPtr = void(*)(NativeInterface*, void*, const AllowResourceLevelQuery*, BoolCallinResult*);
	using AllowResourceTransferFuncPtr = void(*)(NativeInterface*, void*, const AllowResourceTransferQuery*, BoolCallinResult*);
	using ResourceExcessFuncPtr = void(*)(NativeInterface*, void*, const ResourceExcessQuery*, BoolCallinResult*);
	using AllowDirectUnitControlFuncPtr = void(*)(NativeInterface*, void*, const AllowDirectUnitControlQuery*, BoolCallinResult*);
	using AllowBuilderHoldFireFuncPtr = void(*)(NativeInterface*, void*, const AllowBuilderHoldFireQuery*, BoolCallinResult*);
	using AllowStartPositionFuncPtr = void(*)(NativeInterface*, void*, const AllowStartPositionQuery*, BoolCallinResult*);
	using TerraformCompleteFuncPtr = void(*)(NativeInterface*, void*, const TerraformCompleteQuery*, BoolCallinResult*);
	using MoveCtrlNotifyFuncPtr = void(*)(NativeInterface*, void*, const MoveCtrlNotifyQuery*, BoolCallinResult*);
	using UnitCmdDoneFuncPtr = void(*)(NativeInterface*, void*, const UnitCmdDoneQuery*, UnitCmdDoneResult*);
	using UnitDamagedFuncPtr = void(*)(NativeInterface*, void*, const UnitDamagedQuery*, UnitDamagedResult*);
	using UnitPreDamagedFuncPtr = void(*)(NativeInterface*, void*, const UnitDamagedQuery*, DamageCallinResult*);
	using UnitHarvestStorageFullFuncPtr = void(*)(NativeInterface*, void*, const UnitHarvestStorageFullQuery*, UnitHarvestStorageFullResult*);
	using UnitSeismicPingFuncPtr = void(*)(NativeInterface*, void*, const UnitSeismicPingQuery*, UnitSeismicPingResult*);
	using UnitLosEventFuncPtr = void(*)(NativeInterface*, void*, const UnitLosEventQuery*, UnitLosEventResult*);
	using UnitEnteredRadarFuncPtr = UnitLosEventFuncPtr;
	using UnitEnteredLosFuncPtr = UnitLosEventFuncPtr;
	using UnitLeftRadarFuncPtr = UnitLosEventFuncPtr;
	using UnitLeftLosFuncPtr = UnitLosEventFuncPtr;
	using UnitMovementClassEventFuncPtr = void(*)(NativeInterface*, void*, const UnitMovementClassEventQuery*, UnitMovementClassEventResult*);
	using UnitEnteredUnderwaterFuncPtr = UnitMovementClassEventFuncPtr;
	using UnitEnteredWaterFuncPtr = UnitMovementClassEventFuncPtr;
	using UnitEnteredAirFuncPtr = UnitMovementClassEventFuncPtr;
	using UnitLeftUnderwaterFuncPtr = UnitMovementClassEventFuncPtr;
	using UnitLeftWaterFuncPtr = UnitMovementClassEventFuncPtr;
	using UnitLeftAirFuncPtr = UnitMovementClassEventFuncPtr;
	using UnitLoadedFuncPtr = void(*)(NativeInterface*, void*, const UnitLoadedQuery*, UnitLoadedResult*);
	using UnitStunnedFuncPtr = void(*)(NativeInterface*, void*, const UnitStunnedQuery*, UnitStunnedResult*);
	using UnitTakenFuncPtr = void(*)(NativeInterface*, void*, const UnitTakenQuery*, UnitTakenResult*);
	using UnitUnloadedFuncPtr = void(*)(NativeInterface*, void*, const UnitUnloadedQuery*, UnitUnloadedResult*);
	using UnitCloakEventFuncPtr = void(*)(NativeInterface*, void*, const UnitCloakEventQuery*, UnitCloakEventResult*);
	using UnitCloakedFuncPtr = UnitCloakEventFuncPtr;
	using UnitDecloakedFuncPtr = UnitCloakEventFuncPtr;
	using UnitMoveEventFuncPtr = void(*)(NativeInterface*, void*, const UnitMoveEventQuery*, UnitMoveEventResult*);
	using UnitMovedFuncPtr = UnitMoveEventFuncPtr;
	using UnitMoveFailedFuncPtr = UnitMoveEventFuncPtr;
	using UnitArrivedAtGoalFuncPtr = UnitMoveEventFuncPtr;
	using HandleLuaMsgFuncPtr = void(*)(NativeInterface*, void*, const HandleLuaMsgQuery*, HandleLuaMsgResult*);
	using HandleLuaCallFuncPtr = void(*)(NativeInterface*, void*, const HandleLuaCallQuery*, HandleLuaCallResult*);
	using UpdateFuncPtr = void(*)(NativeInterface*, void*, const UpdateQuery*, UpdateResult*);
	using DrawScreenFuncPtr = void(*)(NativeInterface*, void*, const DrawScreenQuery*, DrawScreenResult*);
	using FeatureMovedFuncPtr = void(*)(NativeInterface*, void*, const FeatureMovedQuery*, FeatureMovedResult*);
	using FeatureDamagedFuncPtr = void(*)(NativeInterface*, void*, const FeatureDamagedQuery*, FeatureDamagedResult*);
	using FeaturePreDamagedFuncPtr = void(*)(NativeInterface*, void*, const FeatureDamagedQuery*, DamageCallinResult*);
	using ProjectileEventFuncPtr = void(*)(NativeInterface*, void*, const ProjectileEventQuery*, ProjectileEventResult*);
	using ProjectileCreatedFuncPtr = ProjectileEventFuncPtr;
	using ProjectileDestroyedFuncPtr = ProjectileEventFuncPtr;
	using LastMessagePositionFuncPtr = void(*)(NativeInterface*, void*, const LastMessagePositionQuery*, LastMessagePositionResult*);
	using ViewResizeFuncPtr = void(*)(NativeInterface*, void*, const ViewResizeQuery*, ViewResizeResult*);
	using SunChangedFuncPtr = void(*)(NativeInterface*, void*, const SunChangedQuery*, SunChangedResult*);
	using SimpleCallinFuncPtr = void(*)(NativeInterface*, void*, const SimpleCallinQuery*, SimpleCallinResult*);
	using DrawGenesisFuncPtr = SimpleCallinFuncPtr;
	using DrawWorldFuncPtr = SimpleCallinFuncPtr;
	using DrawWorldPreUnitFuncPtr = SimpleCallinFuncPtr;
	using DrawPreDecalsFuncPtr = SimpleCallinFuncPtr;
	using DrawWaterPostFuncPtr = SimpleCallinFuncPtr;
	using DrawWorldShadowFuncPtr = SimpleCallinFuncPtr;
	using DrawShadowPassTransparentFuncPtr = SimpleCallinFuncPtr;
	using DrawWorldReflectionFuncPtr = SimpleCallinFuncPtr;
	using DrawWorldRefractionFuncPtr = SimpleCallinFuncPtr;
	using DrawGroundPreForwardFuncPtr = SimpleCallinFuncPtr;
	using DrawGroundPostForwardFuncPtr = SimpleCallinFuncPtr;
	using DrawGroundPreDeferredFuncPtr = SimpleCallinFuncPtr;
	using DrawGroundDeferredFuncPtr = SimpleCallinFuncPtr;
	using DrawGroundPostDeferredFuncPtr = SimpleCallinFuncPtr;
	using DrawUnitsPostDeferredFuncPtr = SimpleCallinFuncPtr;
	using DrawFeaturesPostDeferredFuncPtr = SimpleCallinFuncPtr;
	using DrawScreenEffectsFuncPtr = SimpleCallinFuncPtr;
	using DrawScreenPostFuncPtr = SimpleCallinFuncPtr;
	using DrawInMiniMapFuncPtr = SimpleCallinFuncPtr;
	using DrawInMiniMapBackgroundFuncPtr = SimpleCallinFuncPtr;
	using DrawShadowUnitsLuaFuncPtr = SimpleCallinFuncPtr;
	using DrawShadowFeaturesLuaFuncPtr = SimpleCallinFuncPtr;
	using FontsChangedFuncPtr = SimpleCallinFuncPtr;
	using DrawWorldPreParticlesFuncPtr = void(*)(NativeInterface*, void*, const DrawWorldPreParticlesQuery*, DrawWorldPreParticlesResult*);
	using DrawBuildSquareFuncPtr = void(*)(NativeInterface*, void*, const DrawBuildSquareQuery*, DrawBuildSquareResult*);
	using DrawObjectsLuaFuncPtr = void(*)(NativeInterface*, void*, const DrawObjectsLuaQuery*, DrawObjectsLuaResult*);
	using DrawOpaqueUnitsLuaFuncPtr = DrawObjectsLuaFuncPtr;
	using DrawOpaqueFeaturesLuaFuncPtr = DrawObjectsLuaFuncPtr;
	using DrawAlphaObjectsLuaFuncPtr = void(*)(NativeInterface*, void*, const DrawAlphaObjectsLuaQuery*, DrawAlphaObjectsLuaResult*);
	using DrawAlphaUnitsLuaFuncPtr = DrawAlphaObjectsLuaFuncPtr;
	using DrawAlphaFeaturesLuaFuncPtr = DrawAlphaObjectsLuaFuncPtr;
	using GameProgressFuncPtr = void(*)(NativeInterface*, void*, const GameProgressQuery*, GameProgressResult*);
	using CollectGarbageFuncPtr = void(*)(NativeInterface*, void*, const CollectGarbageQuery*, CollectGarbageResult*);
	using StockpileChangedFuncPtr = void(*)(NativeInterface*, void*, const StockpileChangedQuery*, StockpileChangedResult*);
	using RectChangedFuncPtr = void(*)(NativeInterface*, void*, const RectChangedQuery*, RectChangedResult*);
	using UnsyncedHeightMapUpdateFuncPtr = RectChangedFuncPtr;
	using Float3CallinFuncPtr = void(*)(NativeInterface*, void*, const Float3CallinQuery*, Float3CallinResult*);
	using CameraRotationChangedFuncPtr = Float3CallinFuncPtr;
	using CameraPositionChangedFuncPtr = Float3CallinFuncPtr;
	using KeyMapChangedFuncPtr = void(*)(NativeInterface*, void*, const SimpleCallinQuery*, BoolCallinResult*);
	using KeyPressFuncPtr = void(*)(NativeInterface*, void*, const KeyPressQuery*, BoolCallinResult*);
	using KeyReleaseFuncPtr = void(*)(NativeInterface*, void*, const KeyReleaseQuery*, BoolCallinResult*);
	using TextInputFuncPtr = void(*)(NativeInterface*, void*, const TextInputQuery*, BoolCallinResult*);
	using TextEditingFuncPtr = void(*)(NativeInterface*, void*, const TextEditingQuery*, BoolCallinResult*);
	using MouseMoveFuncPtr = void(*)(NativeInterface*, void*, const MouseMoveQuery*, BoolCallinResult*);
	using MousePressFuncPtr = void(*)(NativeInterface*, void*, const MousePressQuery*, BoolCallinResult*);
	using MouseReleaseFuncPtr = void(*)(NativeInterface*, void*, const MouseReleaseQuery*, MouseReleaseResult*);
	using MouseWheelFuncPtr = void(*)(NativeInterface*, void*, const MouseWheelQuery*, BoolCallinResult*);
	using ScreenPositionFuncPtr = void(*)(NativeInterface*, void*, const ScreenPositionQuery*, BoolCallinResult*);
	using IsAboveFuncPtr = ScreenPositionFuncPtr;
	using MiniMapRotationChangedFuncPtr = void(*)(NativeInterface*, void*, const MiniMapRotationChangedQuery*, SimpleCallinResult*);
	using MiniMapStateChangedFuncPtr = void(*)(NativeInterface*, void*, const MiniMapStateChangedQuery*, SimpleCallinResult*);
	using MiniMapGeometryChangedFuncPtr = void(*)(NativeInterface*, void*, const MiniMapGeometryChangedQuery*, SimpleCallinResult*);
	using DrawUnitFuncPtr = void(*)(NativeInterface*, void*, const DrawUnitQuery*, BoolCallinResult*);
	using DrawFeatureFuncPtr = void(*)(NativeInterface*, void*, const DrawFeatureQuery*, BoolCallinResult*);
	using DrawShieldFuncPtr = void(*)(NativeInterface*, void*, const DrawShieldQuery*, BoolCallinResult*);
	using DrawProjectileFuncPtr = void(*)(NativeInterface*, void*, const DrawProjectileQuery*, BoolCallinResult*);
	using DrawMaterialFuncPtr = void(*)(NativeInterface*, void*, const DrawMaterialQuery*, BoolCallinResult*);
	using ActiveCommandChangedFuncPtr = void(*)(NativeInterface*, void*, const ActiveCommandChangedQuery*, ActiveCommandChangedResult*);
	using CommandNotifyFuncPtr = void(*)(NativeInterface*, void*, const CommandNotifyQuery*, BoolCallinResult*);
	using AddConsoleLineFuncPtr = void(*)(NativeInterface*, void*, const AddConsoleLineQuery*, BoolCallinResult*);
	using GroupChangedFuncPtr = void(*)(NativeInterface*, void*, const GroupChangedQuery*, BoolCallinResult*);
	using DefaultCommandFuncPtr = void(*)(NativeInterface*, void*, const DefaultCommandQuery*, DefaultCommandResult*);
	using MapDrawCmdFuncPtr = void(*)(NativeInterface*, void*, const MapDrawCmdQuery*, BoolCallinResult*);
	using ArchiveCallinFuncPtr = void(*)(NativeInterface*, void*, const ArchiveCallinQuery*, ArchiveCallinResult*);
	using LoadFuncPtr = ArchiveCallinFuncPtr;
	using SaveFuncPtr = ArchiveCallinFuncPtr;
	using UnitUnitCollisionFuncPtr = void(*)(NativeInterface*, void*, const UnitUnitCollisionQuery*, BoolCallinResult*);
	using UnitFeatureCollisionFuncPtr = void(*)(NativeInterface*, void*, const UnitFeatureCollisionQuery*, BoolCallinResult*);
	using ExplosionFuncPtr = void(*)(NativeInterface*, void*, const ExplosionQuery*, BoolCallinResult*);
	using AllowWeaponTargetCheckFuncPtr = void(*)(NativeInterface*, void*, const AllowWeaponTargetCheckQuery*, IntCallinResult*);
	using AllowWeaponTargetFuncPtr = void(*)(NativeInterface*, void*, const AllowWeaponTargetQuery*, AllowWeaponTargetResult*);
	using AllowWeaponInterceptTargetFuncPtr = void(*)(NativeInterface*, void*, const AllowWeaponInterceptTargetQuery*, BoolCallinResult*);
	using ShieldPreDamagedFuncPtr = void(*)(NativeInterface*, void*, const ShieldPreDamagedQuery*, BoolCallinResult*);
	using ScreenTooltipFuncPtr = void(*)(NativeInterface*, void*, const ScreenPositionQuery*, StringCallinResult*);
	using GetTooltipFuncPtr = ScreenTooltipFuncPtr;
	using WorldTooltipFuncPtr = void(*)(NativeInterface*, void*, const WorldTooltipQuery*, StringCallinResult*);
	using GameSetupFuncPtr = void(*)(NativeInterface*, void*, const GameSetupQuery*, GameSetupResult*);
	using PongFuncPtr = void(*)(NativeInterface*, void*, const PongQuery*, PongResult*);
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
	NativeInterfaceEventClient(NativeInterface* nativeInterface, SharedLib* sharedLib);

	// Load symbols from DLL
	void LoadSymbols();

	// Initialize the native module
	void* Initialize();

	// Release module-owned state before its shared object is unloaded. Must be
	// called after the event client is unregistered and while m_sharedLib is valid.
	void Shutdown();

	// CEventClient interface
	bool WantsEvent(const std::string& eventName) override {
		return true;
	}
	bool GetFullRead() const override { return true; }
	int GetReadAllyTeam() const override { return AllAccessTeam; }

	// Synced events
	void Load(IArchive* archive) override;
	void GamePreload() override;
	void GameStart() override;
	void GameOver(const std::vector<unsigned char>& winningAllyTeams) override;
	void GamePaused(int playerID, bool paused) override;
	void GameFrame(int gameFrame) override;
	void GameFramePost(int gameFrame) override;
	void GameID(const unsigned char* gameID, unsigned int numBytes) override;

	void TeamDied(int teamID) override;
	void TeamChanged(int teamID) override;
	void PlayerChanged(int playerID) override;
	void PlayerAdded(int playerID) override;
	void PlayerRemoved(int playerID, int reason) override;

	void UnitCreated(const CUnit* unit, const CUnit* builder) override;
	void UnitFinished(const CUnit* unit) override;
	void UnitReverseBuilt(const CUnit* unit) override;
	void UnitConstructionDecayed(const CUnit* unit, float timeSinceLastBuild, float iterationPeriod, float part) override;
	void UnitFromFactory(const CUnit* unit, const CUnit* factory, bool userOrders) override;
	void UnitDestroyed(const CUnit* unit, const CUnit* attacker, int weaponDefID) override;
	void UnitTaken(const CUnit* unit, int oldTeam, int newTeam) override;
	void UnitGiven(const CUnit* unit, int oldTeam, int newTeam) override;
	void UnitIdle(const CUnit* unit) override;
	void UnitCommand(const CUnit* unit, const Command& command, int playerNum, bool fromSynced, bool fromLua) override;
	bool CommandFallback(const CUnit* unit, const Command& command) override;
	bool AllowCommand(const CUnit* unit, const Command& command, int playerNum, bool fromSynced, bool fromLua) override;
	std::pair<bool, bool> AllowUnitCreation(const UnitDef* unitDef, const CUnit* builder, const BuildInfo* buildInfo) override;
	bool AllowUnitTransfer(const CUnit* unit, int newTeam, bool capture) override;
	bool AllowUnitBuildStep(const CUnit* builder, const CUnit* unit, float part) override;
	bool AllowUnitCaptureStep(const CUnit* builder, const CUnit* unit, float part) override;
	bool AllowUnitTransport(const CUnit* transporter, const CUnit* transportee) override;
	bool AllowUnitTransportLoad(const CUnit* transporter, const CUnit* transportee, const float3& loadPos, bool allowed) override;
	bool AllowUnitTransportUnload(const CUnit* transporter, const CUnit* transportee, const float3& unloadPos, bool allowed) override;
	bool AllowUnitCloak(const CUnit* unit, const CUnit* enemy) override;
	bool AllowUnitDecloak(const CUnit* unit, const CSolidObject* object, const CWeapon* weapon) override;
	bool AllowUnitKamikaze(const CUnit* unit, const CUnit* target, bool allowed) override;
	bool AllowFeatureCreation(const FeatureDef* featureDef, int allyTeamID, const float3& pos) override;
	bool AllowFeatureBuildStep(const CUnit* builder, const CFeature* feature, float part) override;
	bool AllowResourceLevel(int teamID, const std::string& type, float level) override;
	bool AllowResourceTransfer(int oldTeam, int newTeam, const char* type, float amount) override;
	bool ResourceExcess(const std::map<int, SResourcePack>& excess) override;
	bool AllowDirectUnitControl(int playerID, const CUnit* unit) override;
	bool AllowBuilderHoldFire(const CUnit* unit, int action) override;
	bool AllowStartPosition(int playerID, int teamID, unsigned char readyState, const float3& clampedPos, const float3& rawPickPos) override;
	bool TerraformComplete(const CUnit* unit, const CUnit* build) override;
	bool MoveCtrlNotify(const CUnit* unit, int data) override;
	int AllowWeaponTargetCheck(unsigned int attackerID, unsigned int attackerWeaponNum, unsigned int attackerWeaponDefID) override;
	bool AllowWeaponTarget(unsigned int attackerID, unsigned int targetID, unsigned int attackerWeaponNum, unsigned int attackerWeaponDefID, float* targetPriority) override;
	bool AllowWeaponInterceptTarget(const CUnit* interceptorUnit, const CWeapon* interceptorWeapon, const CProjectile* interceptorTarget) override;
	bool UnitPreDamaged(const CUnit* unit, const CUnit* attacker, float damage, int weaponDefID, int projectileID, bool paralyzer, float* newDamage, float* impulseMult) override;
	bool FeaturePreDamaged(const CFeature* feature, const CUnit* attacker, float damage, int weaponDefID, int projectileID, float* newDamage, float* impulseMult) override;
	bool ShieldPreDamaged(const CProjectile* projectile, const CWeapon* shieldEmitter, const CUnit* shieldCarrier, bool bounceProjectile, const CWeapon* beamEmitter, const CUnit* beamCarrier, const float3& startPos, const float3& hitPos) override;
	void UnitCmdDone(const CUnit* unit, const Command& command) override;
	void UnitDamaged(const CUnit* unit, const CUnit* attacker, float damage, int weaponDefID, int projectileID, bool paralyzer) override;
	void UnitHarvestStorageFull(const CUnit* unit) override;
	void UnitSeismicPing(const CUnit* unit, int allyTeam, const float3& pos, float strength) override;
	void UnitEnteredRadar(const CUnit* unit, int allyTeam) override;
	void UnitEnteredLos(const CUnit* unit, int allyTeam) override;
	void UnitLeftRadar(const CUnit* unit, int allyTeam) override;
	void UnitLeftLos(const CUnit* unit, int allyTeam) override;
	void UnitEnteredUnderwater(const CUnit* unit) override;
	void UnitEnteredWater(const CUnit* unit) override;
	void UnitEnteredAir(const CUnit* unit) override;
	void UnitLeftUnderwater(const CUnit* unit) override;
	void UnitLeftWater(const CUnit* unit) override;
	void UnitLeftAir(const CUnit* unit) override;
	void UnitStunned(const CUnit* unit, bool stunned) override;
	void UnitExperience(const CUnit* unit, float oldExperience) override;
	void UnitLoaded(const CUnit* unit, const CUnit* transport) override;
	void UnitUnloaded(const CUnit* unit, const CUnit* transport) override;
	void UnitCloaked(const CUnit* unit) override;
	void UnitDecloaked(const CUnit* unit) override;
	void UnitMoved(const CUnit* unit) override;
	void UnitMoveFailed(const CUnit* unit) override;
	void UnitArrivedAtGoal(const CUnit* unit) override;
	bool UnitUnitCollision(const CUnit* collider, const CUnit* collidee) override;
	bool UnitFeatureCollision(const CUnit* collider, const CFeature* collidee) override;
	void RenderUnitDestroyed(const CUnit* unit) override;

	void FeatureCreated(const CFeature* feature) override;
	void FeatureDestroyed(const CFeature* feature) override;
	void FeatureDamaged(const CFeature* feature, const CUnit* attacker, float damage, int weaponDefID, int projectileID) override;
	void FeatureMoved(const CFeature* feature, const float3& oldpos) override;
	void ProjectileCreated(const CProjectile* proj) override;
	void ProjectileDestroyed(const CProjectile* proj) override;
	bool Explosion(int weaponID, const WeaponDef* weaponDef, const CExplosionParams& params) override;

	// Per-frame / draw callins (unsynced)
	void Update() override;
	void DrawGenesis() override;
	void DrawWorld() override;
	void DrawWorldPreUnit() override;
	void DrawPreDecals() override;
	void DrawWorldPreParticles(bool drawAboveWater, bool drawBelowWater, bool drawReflection, bool drawRefraction) override;
	void DrawWaterPost() override;
	void DrawWorldShadow() override;
	void DrawShadowPassTransparent() override;
	void DrawWorldReflection() override;
	void DrawWorldRefraction() override;
	void DrawGroundPreForward() override;
	void DrawGroundPostForward() override;
	void DrawGroundPreDeferred() override;
	void DrawGroundDeferred() override;
	void DrawGroundPostDeferred() override;
	void DrawUnitsPostDeferred() override;
	void DrawFeaturesPostDeferred() override;
	void DrawScreen() override;
	void DrawScreenEffects() override;
	void DrawScreenPost() override;
	void DrawInMiniMap() override;
	void DrawInMiniMapBackground() override;
	void DrawBuildSquare(int unitDefID, int x, int z, int facing, const std::vector<uint8_t>& statuses) override;
	void DrawOpaqueUnitsLua(bool deferredPass, bool drawReflection, bool drawRefraction) override;
	void DrawOpaqueFeaturesLua(bool deferredPass, bool drawReflection, bool drawRefraction) override;
	void DrawAlphaUnitsLua(bool drawReflection, bool drawRefraction) override;
	void DrawAlphaFeaturesLua(bool drawReflection, bool drawRefraction) override;
	void DrawShadowUnitsLua() override;
	void DrawShadowFeaturesLua() override;
	bool DrawUnit(const CUnit* unit) override;
	bool DrawFeature(const CFeature* feature) override;
	bool DrawShield(const CUnit* unit, const CWeapon* weapon) override;
	bool DrawProjectile(const CProjectile* projectile) override;
	bool DrawMaterial(const LuaMaterial* material) override;

	// Unsynced events
	void DownloadFailed(int ID, int errorID) override;
	void DownloadFinished(int ID) override;
	void DownloadProgress(int ID, long downloaded, long total) override;
	void DownloadQueued(int ID, const std::string& archiveName, const std::string& archiveType) override;
	void DownloadStarted(int ID) override;
	void Save(zipFile archive) override;
	void UnsyncedHeightMapUpdate(const SRectangle& rect) override;
	void LastMessagePosition(const float3& pos) override;
	bool KeyMapChanged() override;
	bool KeyPress(int keyCode, int scanCode, bool isRepeat) override;
	bool KeyRelease(int keyCode, int scanCode) override;
	bool TextInput(const std::string& utf8) override;
	bool TextEditing(const std::string& utf8, unsigned int start, unsigned int length) override;
	bool MouseMove(int x, int y, int dx, int dy, int button) override;
	bool MousePress(int x, int y, int button) override;
	void MouseRelease(int x, int y, int button) override;
	bool MouseWheel(bool up, float value) override;
	bool IsAbove(int x, int y) override;
	std::string GetTooltip(int x, int y) override;
	bool DefaultCommand(const CUnit* unit, const CFeature* feature, int& cmd) override;
	void ActiveCommandChanged(const SCommandDescription* cmdDesc) override;
	void CameraRotationChanged(const float3& rot) override;
	void CameraPositionChanged(const float3& pos) override;
	bool CommandNotify(const Command& cmd) override;
	bool AddConsoleLine(const std::string& msg, const std::string& section, int level) override;
	bool GroupChanged(int groupID) override;
	void MiniMapRotationChanged(float newRot, float oldRot) override;
	void MiniMapStateChanged(bool isMinimized, bool isMaximized, bool isSlaved) override;
	void MiniMapGeometryChanged(int2 newPos, int2 newDim, int2 oldPos, int2 oldDim) override;
	bool GameSetup(const std::string& state, bool& ready, const std::vector<std::pair<int, std::string>>& playerStates) override;
	std::string WorldTooltip(const CUnit* unit, const CFeature* feature, const float3* groundPos) override;
	bool MapDrawCmd(int playerID, int type, const float3* pos0, const float3* pos1, const std::string* label) override;
	void ViewResize() override;
	void SunChanged() override;
	void FontsChanged() override;
	void GameProgress(int gameFrame) override;
	void StockpileChanged(const CUnit* unit, const CWeapon* weapon, int oldCount) override;
	void CollectGarbage(bool forced) override;
	void Pong(uint8_t pingTag, const spring_time pktSendTime, const spring_time pktRecvTime) override;

	// Special events (not part of CEventClient)
	void HandleLuaMsg(int playerID, int script, int mode, const std::vector<std::uint8_t>& data);
	void HandleLuaCall(const char* msg, size_t msgLength, bool synced);

private:
	NativeInterface* m_nativeInterface;
	SharedLib* m_sharedLib;
	void* m_moduleData = nullptr;
	bool m_initialized = false;

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
	fptr::GameOverFuncPtr m_GameOverFuncPtr = nullptr;
	fptr::GameFrameFuncPtr m_GameFrameFuncPtr = nullptr;
	fptr::GameFramePostFuncPtr m_GameFramePostFuncPtr = nullptr;
	fptr::PlayerAddedFuncPtr m_PlayerAddedFuncPtr = nullptr;
	fptr::PlayerChangedFuncPtr m_PlayerChangedFuncPtr = nullptr;
	fptr::PlayerRemovedFuncPtr m_PlayerRemovedFuncPtr = nullptr;
	fptr::RenderUnitDestroyedFuncPtr m_RenderUnitDestroyedFuncPtr = nullptr;
	fptr::ShutdownFuncPtr m_ShutdownFuncPtr = nullptr;
	fptr::TeamChangedFuncPtr m_TeamChangedFuncPtr = nullptr;
	fptr::TeamDiedFuncPtr m_TeamDiedFuncPtr = nullptr;
	fptr::UnitCreatedFuncPtr m_UnitCreatedFuncPtr = nullptr;
	fptr::UnitDestroyedFuncPtr m_UnitDestroyedFuncPtr = nullptr;
	fptr::UnitExperienceFuncPtr m_UnitExperienceFuncPtr = nullptr;
	fptr::UnitFinishedFuncPtr m_UnitFinishedFuncPtr = nullptr;
	fptr::UnitReverseBuiltFuncPtr m_UnitReverseBuiltFuncPtr = nullptr;
	fptr::UnitConstructionDecayedFuncPtr m_UnitConstructionDecayedFuncPtr = nullptr;
	fptr::UnitFromFactoryFuncPtr m_UnitFromFactoryFuncPtr = nullptr;
	fptr::UnitGivenFuncPtr m_UnitGivenFuncPtr = nullptr;
	fptr::UnitIdleFuncPtr m_UnitIdleFuncPtr = nullptr;
	fptr::UnitCommandFuncPtr m_UnitCommandFuncPtr = nullptr;
	fptr::CommandFallbackFuncPtr m_CommandFallbackFuncPtr = nullptr;
	fptr::AllowCommandFuncPtr m_AllowCommandFuncPtr = nullptr;
	fptr::AllowUnitCreationFuncPtr m_AllowUnitCreationFuncPtr = nullptr;
	fptr::AllowUnitTransferFuncPtr m_AllowUnitTransferFuncPtr = nullptr;
	fptr::AllowUnitBuildStepFuncPtr m_AllowUnitBuildStepFuncPtr = nullptr;
	fptr::AllowUnitCaptureStepFuncPtr m_AllowUnitCaptureStepFuncPtr = nullptr;
	fptr::AllowUnitTransportFuncPtr m_AllowUnitTransportFuncPtr = nullptr;
	fptr::AllowUnitTransportPositionFuncPtr m_AllowUnitTransportLoadFuncPtr = nullptr;
	fptr::AllowUnitTransportPositionFuncPtr m_AllowUnitTransportUnloadFuncPtr = nullptr;
	fptr::AllowUnitCloakFuncPtr m_AllowUnitCloakFuncPtr = nullptr;
	fptr::AllowUnitDecloakFuncPtr m_AllowUnitDecloakFuncPtr = nullptr;
	fptr::AllowUnitKamikazeFuncPtr m_AllowUnitKamikazeFuncPtr = nullptr;
	fptr::AllowFeatureCreationFuncPtr m_AllowFeatureCreationFuncPtr = nullptr;
	fptr::AllowFeatureBuildStepFuncPtr m_AllowFeatureBuildStepFuncPtr = nullptr;
	fptr::AllowResourceLevelFuncPtr m_AllowResourceLevelFuncPtr = nullptr;
	fptr::AllowResourceTransferFuncPtr m_AllowResourceTransferFuncPtr = nullptr;
	fptr::ResourceExcessFuncPtr m_ResourceExcessFuncPtr = nullptr;
	fptr::AllowDirectUnitControlFuncPtr m_AllowDirectUnitControlFuncPtr = nullptr;
	fptr::AllowBuilderHoldFireFuncPtr m_AllowBuilderHoldFireFuncPtr = nullptr;
	fptr::AllowStartPositionFuncPtr m_AllowStartPositionFuncPtr = nullptr;
	fptr::TerraformCompleteFuncPtr m_TerraformCompleteFuncPtr = nullptr;
	fptr::MoveCtrlNotifyFuncPtr m_MoveCtrlNotifyFuncPtr = nullptr;
	fptr::AllowWeaponTargetCheckFuncPtr m_AllowWeaponTargetCheckFuncPtr = nullptr;
	fptr::AllowWeaponTargetFuncPtr m_AllowWeaponTargetFuncPtr = nullptr;
	fptr::AllowWeaponInterceptTargetFuncPtr m_AllowWeaponInterceptTargetFuncPtr = nullptr;
	fptr::UnitPreDamagedFuncPtr m_UnitPreDamagedFuncPtr = nullptr;
	fptr::FeaturePreDamagedFuncPtr m_FeaturePreDamagedFuncPtr = nullptr;
	fptr::ShieldPreDamagedFuncPtr m_ShieldPreDamagedFuncPtr = nullptr;
	fptr::UnitCmdDoneFuncPtr m_UnitCmdDoneFuncPtr = nullptr;
	fptr::UnitDamagedFuncPtr m_UnitDamagedFuncPtr = nullptr;
	fptr::UnitHarvestStorageFullFuncPtr m_UnitHarvestStorageFullFuncPtr = nullptr;
	fptr::UnitSeismicPingFuncPtr m_UnitSeismicPingFuncPtr = nullptr;
	fptr::UnitLosEventFuncPtr m_UnitEnteredRadarFuncPtr = nullptr;
	fptr::UnitLosEventFuncPtr m_UnitEnteredLosFuncPtr = nullptr;
	fptr::UnitLosEventFuncPtr m_UnitLeftRadarFuncPtr = nullptr;
	fptr::UnitLosEventFuncPtr m_UnitLeftLosFuncPtr = nullptr;
	fptr::UnitMovementClassEventFuncPtr m_UnitEnteredUnderwaterFuncPtr = nullptr;
	fptr::UnitMovementClassEventFuncPtr m_UnitEnteredWaterFuncPtr = nullptr;
	fptr::UnitMovementClassEventFuncPtr m_UnitEnteredAirFuncPtr = nullptr;
	fptr::UnitMovementClassEventFuncPtr m_UnitLeftUnderwaterFuncPtr = nullptr;
	fptr::UnitMovementClassEventFuncPtr m_UnitLeftWaterFuncPtr = nullptr;
	fptr::UnitMovementClassEventFuncPtr m_UnitLeftAirFuncPtr = nullptr;
	fptr::UnitLoadedFuncPtr m_UnitLoadedFuncPtr = nullptr;
	fptr::UnitStunnedFuncPtr m_UnitStunnedFuncPtr = nullptr;
	fptr::UnitTakenFuncPtr m_UnitTakenFuncPtr = nullptr;
	fptr::UnitUnloadedFuncPtr m_UnitUnloadedFuncPtr = nullptr;
	fptr::UnitCloakEventFuncPtr m_UnitCloakedFuncPtr = nullptr;
	fptr::UnitCloakEventFuncPtr m_UnitDecloakedFuncPtr = nullptr;
	fptr::UnitMoveEventFuncPtr m_UnitMovedFuncPtr = nullptr;
	fptr::UnitMoveEventFuncPtr m_UnitMoveFailedFuncPtr = nullptr;
	fptr::UnitMoveEventFuncPtr m_UnitArrivedAtGoalFuncPtr = nullptr;
	fptr::UnitUnitCollisionFuncPtr m_UnitUnitCollisionFuncPtr = nullptr;
	fptr::UnitFeatureCollisionFuncPtr m_UnitFeatureCollisionFuncPtr = nullptr;
	fptr::HandleLuaMsgFuncPtr m_HandleLuaMsgFuncPtr = nullptr;
	fptr::HandleLuaCallFuncPtr m_HandleLuaCallFuncPtr = nullptr;
	fptr::UpdateFuncPtr m_UpdateFuncPtr = nullptr;
	fptr::DrawScreenFuncPtr m_DrawScreenFuncPtr = nullptr;
	fptr::FeatureMovedFuncPtr m_FeatureMovedFuncPtr = nullptr;
	fptr::ProjectileEventFuncPtr m_ProjectileCreatedFuncPtr = nullptr;
	fptr::ProjectileEventFuncPtr m_ProjectileDestroyedFuncPtr = nullptr;
	fptr::FeatureDamagedFuncPtr m_FeatureDamagedFuncPtr = nullptr;
	fptr::ExplosionFuncPtr m_ExplosionFuncPtr = nullptr;
	fptr::LastMessagePositionFuncPtr m_LastMessagePositionFuncPtr = nullptr;
	fptr::RectChangedFuncPtr m_UnsyncedHeightMapUpdateFuncPtr = nullptr;
	fptr::Float3CallinFuncPtr m_CameraRotationChangedFuncPtr = nullptr;
	fptr::Float3CallinFuncPtr m_CameraPositionChangedFuncPtr = nullptr;
	fptr::KeyMapChangedFuncPtr m_KeyMapChangedFuncPtr = nullptr;
	fptr::KeyPressFuncPtr m_KeyPressFuncPtr = nullptr;
	fptr::KeyReleaseFuncPtr m_KeyReleaseFuncPtr = nullptr;
	fptr::TextInputFuncPtr m_TextInputFuncPtr = nullptr;
	fptr::TextEditingFuncPtr m_TextEditingFuncPtr = nullptr;
	fptr::MouseMoveFuncPtr m_MouseMoveFuncPtr = nullptr;
	fptr::MousePressFuncPtr m_MousePressFuncPtr = nullptr;
	fptr::MouseReleaseFuncPtr m_MouseReleaseFuncPtr = nullptr;
	fptr::MouseWheelFuncPtr m_MouseWheelFuncPtr = nullptr;
	fptr::ScreenPositionFuncPtr m_IsAboveFuncPtr = nullptr;
	fptr::MiniMapRotationChangedFuncPtr m_MiniMapRotationChangedFuncPtr = nullptr;
	fptr::MiniMapStateChangedFuncPtr m_MiniMapStateChangedFuncPtr = nullptr;
	fptr::MiniMapGeometryChangedFuncPtr m_MiniMapGeometryChangedFuncPtr = nullptr;
	fptr::DrawUnitFuncPtr m_DrawUnitFuncPtr = nullptr;
	fptr::DrawFeatureFuncPtr m_DrawFeatureFuncPtr = nullptr;
	fptr::DrawShieldFuncPtr m_DrawShieldFuncPtr = nullptr;
	fptr::DrawProjectileFuncPtr m_DrawProjectileFuncPtr = nullptr;
	fptr::DrawMaterialFuncPtr m_DrawMaterialFuncPtr = nullptr;
	fptr::DefaultCommandFuncPtr m_DefaultCommandFuncPtr = nullptr;
	fptr::ActiveCommandChangedFuncPtr m_ActiveCommandChangedFuncPtr = nullptr;
	fptr::CommandNotifyFuncPtr m_CommandNotifyFuncPtr = nullptr;
	fptr::AddConsoleLineFuncPtr m_AddConsoleLineFuncPtr = nullptr;
	fptr::GroupChangedFuncPtr m_GroupChangedFuncPtr = nullptr;
	fptr::MapDrawCmdFuncPtr m_MapDrawCmdFuncPtr = nullptr;
	fptr::ArchiveCallinFuncPtr m_LoadFuncPtr = nullptr;
	fptr::ArchiveCallinFuncPtr m_SaveFuncPtr = nullptr;
	fptr::ScreenTooltipFuncPtr m_GetTooltipFuncPtr = nullptr;
	fptr::WorldTooltipFuncPtr m_WorldTooltipFuncPtr = nullptr;
	fptr::GameSetupFuncPtr m_GameSetupFuncPtr = nullptr;
	fptr::ViewResizeFuncPtr m_ViewResizeFuncPtr = nullptr;
	fptr::SunChangedFuncPtr m_SunChangedFuncPtr = nullptr;
	fptr::SimpleCallinFuncPtr m_DrawGenesisFuncPtr = nullptr;
	fptr::SimpleCallinFuncPtr m_DrawWorldFuncPtr = nullptr;
	fptr::SimpleCallinFuncPtr m_DrawWorldPreUnitFuncPtr = nullptr;
	fptr::SimpleCallinFuncPtr m_DrawPreDecalsFuncPtr = nullptr;
	fptr::DrawWorldPreParticlesFuncPtr m_DrawWorldPreParticlesFuncPtr = nullptr;
	fptr::SimpleCallinFuncPtr m_DrawWaterPostFuncPtr = nullptr;
	fptr::SimpleCallinFuncPtr m_DrawWorldShadowFuncPtr = nullptr;
	fptr::SimpleCallinFuncPtr m_DrawShadowPassTransparentFuncPtr = nullptr;
	fptr::SimpleCallinFuncPtr m_DrawWorldReflectionFuncPtr = nullptr;
	fptr::SimpleCallinFuncPtr m_DrawWorldRefractionFuncPtr = nullptr;
	fptr::SimpleCallinFuncPtr m_DrawGroundPreForwardFuncPtr = nullptr;
	fptr::SimpleCallinFuncPtr m_DrawGroundPostForwardFuncPtr = nullptr;
	fptr::SimpleCallinFuncPtr m_DrawGroundPreDeferredFuncPtr = nullptr;
	fptr::SimpleCallinFuncPtr m_DrawGroundDeferredFuncPtr = nullptr;
	fptr::SimpleCallinFuncPtr m_DrawGroundPostDeferredFuncPtr = nullptr;
	fptr::SimpleCallinFuncPtr m_DrawUnitsPostDeferredFuncPtr = nullptr;
	fptr::SimpleCallinFuncPtr m_DrawFeaturesPostDeferredFuncPtr = nullptr;
	fptr::SimpleCallinFuncPtr m_DrawScreenEffectsFuncPtr = nullptr;
	fptr::SimpleCallinFuncPtr m_DrawScreenPostFuncPtr = nullptr;
	fptr::SimpleCallinFuncPtr m_DrawInMiniMapFuncPtr = nullptr;
	fptr::SimpleCallinFuncPtr m_DrawInMiniMapBackgroundFuncPtr = nullptr;
	fptr::DrawBuildSquareFuncPtr m_DrawBuildSquareFuncPtr = nullptr;
	fptr::DrawObjectsLuaFuncPtr m_DrawOpaqueUnitsLuaFuncPtr = nullptr;
	fptr::DrawObjectsLuaFuncPtr m_DrawOpaqueFeaturesLuaFuncPtr = nullptr;
	fptr::DrawAlphaObjectsLuaFuncPtr m_DrawAlphaUnitsLuaFuncPtr = nullptr;
	fptr::DrawAlphaObjectsLuaFuncPtr m_DrawAlphaFeaturesLuaFuncPtr = nullptr;
	fptr::SimpleCallinFuncPtr m_DrawShadowUnitsLuaFuncPtr = nullptr;
	fptr::SimpleCallinFuncPtr m_DrawShadowFeaturesLuaFuncPtr = nullptr;
	fptr::SimpleCallinFuncPtr m_FontsChangedFuncPtr = nullptr;
	fptr::GameProgressFuncPtr m_GameProgressFuncPtr = nullptr;
	fptr::StockpileChangedFuncPtr m_StockpileChangedFuncPtr = nullptr;
	fptr::CollectGarbageFuncPtr m_CollectGarbageFuncPtr = nullptr;
	fptr::PongFuncPtr m_PongFuncPtr = nullptr;
};
