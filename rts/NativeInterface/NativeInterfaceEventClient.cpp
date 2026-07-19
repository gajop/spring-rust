/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "NativeInterfaceEventClient.h"

#include <cstring>

#include "Game/GameHelper.h"
#include "Sim/Features/Feature.h"
#include "Sim/Features/FeatureDef.h"
#include "Sim/Projectiles/Projectile.h"
#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitDef.h"
#include "Sim/Units/CommandAI/Command.h"
#include "Sim/Units/CommandAI/CommandDescription.h"
#include "Sim/Weapons/Weapon.h"
#include "System/Log/ILog.h"
#include "System/Platform/SharedLib.h"
#include "System/Rectangle.h"
#ifdef SYNCCHECK
#include "System/Sync/SyncChecker.h"
#endif

#define LOAD_SYMBOL(SymbolName)                                                               \
	{                                                                                           \
		m_##SymbolName##FuncPtr = m_sharedLib->FindAddressTyped<fptr::SymbolName##FuncPtr>(      \
			#SymbolName);                                                                          \
		if (m_##SymbolName##FuncPtr == nullptr)                                                   \
			LOG_L(L_ERROR, "Failed to load native module symbol " #SymbolName);                     \
	}

namespace {
	class ScopedNativeSyncedCode {
	public:
		ScopedNativeSyncedCode(bool synced)
			: synced(synced)
		{
		#ifdef SYNCCHECK
			if (synced)
				CSyncChecker::EnterSyncedCode();
		#endif
		}

		~ScopedNativeSyncedCode()
		{
		#ifdef SYNCCHECK
			if (synced)
				CSyncChecker::LeaveSyncedCode();
		#endif
		}

	private:
		bool synced;
	};
}

NativeInterfaceEventClient::NativeInterfaceEventClient(NativeInterface* nativeInterface, SharedLib* sharedLib)
	: CEventClient("[NativeInterfaceEventClient]", 23253, false)
	, m_nativeInterface(nativeInterface)
	, m_sharedLib(sharedLib)
{
}

void NativeInterfaceEventClient::LoadSymbols() {
	LOG("Loading symbols from native module...");

	LOAD_SYMBOL(InitializeNativeModule);
	LOAD_SYMBOL(Load);
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
	LOAD_SYMBOL(GameOver);
	LOAD_SYMBOL(GameFrame);
	LOAD_SYMBOL(GameFramePost);
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
	LOAD_SYMBOL(UnitReverseBuilt);
	LOAD_SYMBOL(UnitConstructionDecayed);
	LOAD_SYMBOL(UnitFromFactory);
	LOAD_SYMBOL(UnitGiven);
	LOAD_SYMBOL(UnitIdle);
	LOAD_SYMBOL(UnitCommand);
	LOAD_SYMBOL(UnitCmdDone);
	LOAD_SYMBOL(UnitDamaged);
	LOAD_SYMBOL(UnitHarvestStorageFull);
	LOAD_SYMBOL(UnitSeismicPing);
	LOAD_SYMBOL(UnitEnteredRadar);
	LOAD_SYMBOL(UnitEnteredLos);
	LOAD_SYMBOL(UnitLeftRadar);
	LOAD_SYMBOL(UnitLeftLos);
	LOAD_SYMBOL(UnitEnteredUnderwater);
	LOAD_SYMBOL(UnitEnteredWater);
	LOAD_SYMBOL(UnitEnteredAir);
	LOAD_SYMBOL(UnitLeftUnderwater);
	LOAD_SYMBOL(UnitLeftWater);
	LOAD_SYMBOL(UnitLeftAir);
	LOAD_SYMBOL(UnitLoaded);
	LOAD_SYMBOL(UnitStunned);
	LOAD_SYMBOL(UnitTaken);
	LOAD_SYMBOL(UnitUnloaded);
	LOAD_SYMBOL(UnitCloaked);
	LOAD_SYMBOL(UnitDecloaked);
	LOAD_SYMBOL(UnitMoved);
	LOAD_SYMBOL(UnitMoveFailed);
	LOAD_SYMBOL(UnitArrivedAtGoal);
	LOAD_SYMBOL(UnitUnitCollision);
	LOAD_SYMBOL(UnitFeatureCollision);
	LOAD_SYMBOL(FeatureMoved);
	LOAD_SYMBOL(FeatureDamaged);
	LOAD_SYMBOL(ProjectileCreated);
	LOAD_SYMBOL(ProjectileDestroyed);
	LOAD_SYMBOL(Explosion);
	LOAD_SYMBOL(HandleLuaMsg);
	LOAD_SYMBOL(HandleLuaCall);
	LOAD_SYMBOL(Update);
	LOAD_SYMBOL(Save);
	LOAD_SYMBOL(DrawScreen);
	LOAD_SYMBOL(DrawGenesis);
	LOAD_SYMBOL(DrawWorld);
	LOAD_SYMBOL(DrawWorldPreUnit);
	LOAD_SYMBOL(DrawPreDecals);
	LOAD_SYMBOL(DrawWorldPreParticles);
	LOAD_SYMBOL(DrawWaterPost);
	LOAD_SYMBOL(DrawWorldShadow);
	LOAD_SYMBOL(DrawShadowPassTransparent);
	LOAD_SYMBOL(DrawWorldReflection);
	LOAD_SYMBOL(DrawWorldRefraction);
	LOAD_SYMBOL(DrawGroundPreForward);
	LOAD_SYMBOL(DrawGroundPostForward);
	LOAD_SYMBOL(DrawGroundPreDeferred);
	LOAD_SYMBOL(DrawGroundDeferred);
	LOAD_SYMBOL(DrawGroundPostDeferred);
	LOAD_SYMBOL(DrawUnitsPostDeferred);
	LOAD_SYMBOL(DrawFeaturesPostDeferred);
	LOAD_SYMBOL(DrawScreenEffects);
	LOAD_SYMBOL(DrawScreenPost);
	LOAD_SYMBOL(DrawInMiniMap);
	LOAD_SYMBOL(DrawInMiniMapBackground);
	LOAD_SYMBOL(DrawBuildSquare);
	LOAD_SYMBOL(DrawOpaqueUnitsLua);
	LOAD_SYMBOL(DrawOpaqueFeaturesLua);
	LOAD_SYMBOL(DrawAlphaUnitsLua);
	LOAD_SYMBOL(DrawAlphaFeaturesLua);
	LOAD_SYMBOL(DrawShadowUnitsLua);
	LOAD_SYMBOL(DrawShadowFeaturesLua);
	LOAD_SYMBOL(LastMessagePosition);
	LOAD_SYMBOL(UnsyncedHeightMapUpdate);
	LOAD_SYMBOL(KeyMapChanged);
	LOAD_SYMBOL(KeyPress);
	LOAD_SYMBOL(KeyRelease);
	LOAD_SYMBOL(TextInput);
	LOAD_SYMBOL(TextEditing);
	LOAD_SYMBOL(MouseMove);
	LOAD_SYMBOL(MousePress);
	LOAD_SYMBOL(MouseRelease);
	LOAD_SYMBOL(MouseWheel);
	LOAD_SYMBOL(IsAbove);
	LOAD_SYMBOL(GetTooltip);
	LOAD_SYMBOL(DefaultCommand);
	LOAD_SYMBOL(ActiveCommandChanged);
	LOAD_SYMBOL(CameraRotationChanged);
	LOAD_SYMBOL(CameraPositionChanged);
	LOAD_SYMBOL(CommandNotify);
	LOAD_SYMBOL(AddConsoleLine);
	LOAD_SYMBOL(GroupChanged);
	LOAD_SYMBOL(GameSetup);
	LOAD_SYMBOL(WorldTooltip);
	LOAD_SYMBOL(MapDrawCmd);
	LOAD_SYMBOL(ViewResize);
	LOAD_SYMBOL(SunChanged);
	LOAD_SYMBOL(FontsChanged);
	LOAD_SYMBOL(GameProgress);
	LOAD_SYMBOL(StockpileChanged);
	LOAD_SYMBOL(CollectGarbage);
	LOAD_SYMBOL(Pong);
}

void* NativeInterfaceEventClient::Initialize() {
	if (m_InitializeNativeModuleFuncPtr == nullptr) {
		LOG_L(L_ERROR, "InitializeNativeModule function not loaded");
		return nullptr;
	}

	LOG("Initializing native module...");

	// Pass host version to module
	InitializeNativeModuleQuery query = {};
	query.hostVersionMajor = NATIVE_API_MAJOR(NATIVE_API_CURRENT_VERSION);
	query.hostVersionMinor = NATIVE_API_MINOR(NATIVE_API_CURRENT_VERSION);
	query.hostVersionPatch = NATIVE_API_PATCH(NATIVE_API_CURRENT_VERSION);

	InitializeNativeModuleResult result = {};

	m_InitializeNativeModuleFuncPtr(m_nativeInterface, &query, &result);

	if (result.error != nullptr) {
		LOG_L(L_ERROR, "Failed to initialize native module: %s", result.error->message);
		return nullptr;
	}

	LOG("Native module initialized successfully (module version: %u.%u.%u)",
		result.moduleVersionMajor, result.moduleVersionMinor, result.moduleVersionPatch);

	m_moduleData = result.moduleData;
	m_initialized = true;
	return m_moduleData;
}

void NativeInterfaceEventClient::Shutdown() {
	if (!m_initialized)
		return;

	if (m_ShutdownFuncPtr != nullptr) {
		ShutdownQuery query = {};
		ShutdownResult result = {};
		m_ShutdownFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		if (result.error != nullptr)
			LOG_L(L_ERROR, "Native module shutdown failed: %s", result.error->message);
	}

	// A native module owns this pointer. Never permit a later unload path to call
	// into the same instance twice.
	m_moduleData = nullptr;
	m_initialized = false;
}

static NativeCallinCommand ToNativeCallinCommand(const Command& command)
{
	return {
		.id = command.GetID(),
		.timeOut = command.GetTimeOut(),
		.pageIndex = command.GetpageIndex(),
		.numParams = command.GetNumParams(),
		.tag = command.GetTag(),
		.options = command.GetOpts(),
		.params = command.GetParams()
	};
}

void NativeInterfaceEventClient::Load(IArchive* archive) {
	if (m_LoadFuncPtr) {
		ArchiveCallinQuery query = {.archive = archive};
		ArchiveCallinResult result = {};
		m_LoadFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
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

void NativeInterfaceEventClient::GameOver(const std::vector<unsigned char>& winningAllyTeams) {
	if (m_GameOverFuncPtr) {
		GameOverEventQuery query = {
			.winningAllyTeams = winningAllyTeams.data(),
			.count = static_cast<uint32_t>(winningAllyTeams.size())
		};
		GameOverEventResult result = {};
		m_GameOverFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::GameFrame(int gameFrame) {
	if (m_GameFrameFuncPtr) {
		GameFrameQuery query = {.gameFrame = gameFrame};
		GameFrameResult result = {};
		m_GameFrameFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::GameFramePost(int gameFrame) {
	if (m_GameFramePostFuncPtr) {
		GameFramePostQuery query = {.gameFrame = gameFrame};
		GameFramePostResult result = {};
		m_GameFramePostFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::Update() {
	if (m_UpdateFuncPtr) {
		UpdateQuery query = {};
		UpdateResult result = {};
		m_UpdateFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::DrawScreen() {
	if (m_DrawScreenFuncPtr) {
		DrawScreenQuery query = {};
		DrawScreenResult result = {};
		m_DrawScreenFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

#define DISPATCH_SIMPLE_CALLIN(EventName)                                      \
	void NativeInterfaceEventClient::EventName() {                              \
		if (m_##EventName##FuncPtr) {                                             \
			SimpleCallinQuery query = {};                                           \
			SimpleCallinResult result = {};                                         \
			m_##EventName##FuncPtr(m_nativeInterface, m_moduleData, &query, &result); \
		}                                                                          \
	}

DISPATCH_SIMPLE_CALLIN(DrawGenesis)
DISPATCH_SIMPLE_CALLIN(DrawWorld)
DISPATCH_SIMPLE_CALLIN(DrawWorldPreUnit)
DISPATCH_SIMPLE_CALLIN(DrawPreDecals)
DISPATCH_SIMPLE_CALLIN(DrawWaterPost)
DISPATCH_SIMPLE_CALLIN(DrawWorldShadow)
DISPATCH_SIMPLE_CALLIN(DrawShadowPassTransparent)
DISPATCH_SIMPLE_CALLIN(DrawWorldReflection)
DISPATCH_SIMPLE_CALLIN(DrawWorldRefraction)
DISPATCH_SIMPLE_CALLIN(DrawGroundPreForward)
DISPATCH_SIMPLE_CALLIN(DrawGroundPostForward)
DISPATCH_SIMPLE_CALLIN(DrawGroundPreDeferred)
DISPATCH_SIMPLE_CALLIN(DrawGroundDeferred)
DISPATCH_SIMPLE_CALLIN(DrawGroundPostDeferred)
DISPATCH_SIMPLE_CALLIN(DrawUnitsPostDeferred)
DISPATCH_SIMPLE_CALLIN(DrawFeaturesPostDeferred)
DISPATCH_SIMPLE_CALLIN(DrawScreenEffects)
DISPATCH_SIMPLE_CALLIN(DrawScreenPost)
DISPATCH_SIMPLE_CALLIN(DrawInMiniMap)
DISPATCH_SIMPLE_CALLIN(DrawInMiniMapBackground)
DISPATCH_SIMPLE_CALLIN(DrawShadowUnitsLua)
DISPATCH_SIMPLE_CALLIN(DrawShadowFeaturesLua)

#undef DISPATCH_SIMPLE_CALLIN

void NativeInterfaceEventClient::DrawWorldPreParticles(bool drawAboveWater, bool drawBelowWater, bool drawReflection, bool drawRefraction) {
	if (m_DrawWorldPreParticlesFuncPtr) {
		DrawWorldPreParticlesQuery query = {
			.drawAboveWater = drawAboveWater,
			.drawBelowWater = drawBelowWater,
			.drawReflection = drawReflection,
			.drawRefraction = drawRefraction
		};
		DrawWorldPreParticlesResult result = {};
		m_DrawWorldPreParticlesFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::DrawBuildSquare(int unitDefID, int x, int z, int facing, const std::vector<uint8_t>& statuses)
{
	if (m_DrawBuildSquareFuncPtr) {
		DrawBuildSquareQuery query = {
			.unitDefID = unitDefID,
			.x = x,
			.z = z,
			.facing = facing,
			.statuses = statuses.data(),
			.statusCount = static_cast<uint32_t>(statuses.size()),
		};
		DrawBuildSquareResult result = {};
		m_DrawBuildSquareFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

#define DISPATCH_DRAW_OBJECTS_LUA(EventName)                                  \
	void NativeInterfaceEventClient::EventName(bool deferredPass, bool drawReflection, bool drawRefraction) { \
		if (m_##EventName##FuncPtr) {                                             \
			DrawObjectsLuaQuery query = {                                           \
				.deferredPass = deferredPass,                                        \
				.drawReflection = drawReflection,                                    \
				.drawRefraction = drawRefraction                                     \
			};                                                                       \
			DrawObjectsLuaResult result = {};                                        \
			m_##EventName##FuncPtr(m_nativeInterface, m_moduleData, &query, &result); \
		}                                                                          \
	}

DISPATCH_DRAW_OBJECTS_LUA(DrawOpaqueUnitsLua)
DISPATCH_DRAW_OBJECTS_LUA(DrawOpaqueFeaturesLua)

#undef DISPATCH_DRAW_OBJECTS_LUA

#define DISPATCH_DRAW_ALPHA_OBJECTS_LUA(EventName)                            \
	void NativeInterfaceEventClient::EventName(bool drawReflection, bool drawRefraction) { \
		if (m_##EventName##FuncPtr) {                                             \
			DrawAlphaObjectsLuaQuery query = {                                      \
				.drawReflection = drawReflection,                                    \
				.drawRefraction = drawRefraction                                     \
			};                                                                       \
			DrawAlphaObjectsLuaResult result = {};                                   \
			m_##EventName##FuncPtr(m_nativeInterface, m_moduleData, &query, &result); \
		}                                                                          \
	}

DISPATCH_DRAW_ALPHA_OBJECTS_LUA(DrawAlphaUnitsLua)
DISPATCH_DRAW_ALPHA_OBJECTS_LUA(DrawAlphaFeaturesLua)

#undef DISPATCH_DRAW_ALPHA_OBJECTS_LUA

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

void NativeInterfaceEventClient::UnitReverseBuilt(const CUnit* unit) {
	if (m_UnitReverseBuiltFuncPtr) {
		UnitReverseBuiltQuery query = {.unitID = unit->id};
		UnitReverseBuiltResult result = {};
		m_UnitReverseBuiltFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitConstructionDecayed(const CUnit* unit, float timeSinceLastBuild, float iterationPeriod, float part) {
	if (m_UnitConstructionDecayedFuncPtr) {
		UnitConstructionDecayedQuery query = {
			.unitID = unit->id,
			.timeSinceLastBuild = timeSinceLastBuild,
			.iterationPeriod = iterationPeriod,
			.part = part
		};
		UnitConstructionDecayedResult result = {};
		m_UnitConstructionDecayedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
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

void NativeInterfaceEventClient::UnitIdle(const CUnit* unit) {
	if (m_UnitIdleFuncPtr) {
		UnitIdleQuery query = {.unitID = unit->id};
		UnitIdleResult result = {};
		m_UnitIdleFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitCommand(const CUnit* unit, const Command& command, int playerNum, bool fromSynced, bool fromLua) {
	if (m_UnitCommandFuncPtr) {
		UnitCommandQuery query = {
			.unitID = unit->id,
			.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
			.unitTeam = unit->team,
			.command = ToNativeCallinCommand(command),
			.playerNum = playerNum,
			.fromSynced = fromSynced,
			.fromLua = fromLua
		};
		UnitCommandResult result = {};
		m_UnitCommandFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitCmdDone(const CUnit* unit, const Command& command) {
	if (m_UnitCmdDoneFuncPtr) {
		UnitCmdDoneQuery query = {
			.unitID = unit->id,
			.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
			.unitTeam = unit->team,
			.command = ToNativeCallinCommand(command)
		};
		UnitCmdDoneResult result = {};
		m_UnitCmdDoneFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitDamaged(const CUnit* unit, const CUnit* attacker, float damage, int weaponDefID, int projectileID, bool paralyzer) {
	if (m_UnitDamagedFuncPtr) {
		UnitDamagedQuery query = {
			.unitID = unit->id,
			.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
			.unitTeam = unit->team,
			.damage = damage,
			.paralyzer = paralyzer,
			.weaponDefID = weaponDefID,
			.projectileID = projectileID,
			.attackerID = (attacker != nullptr) ? attacker->id : -1,
			.attackerDefID = (attacker != nullptr && attacker->unitDef != nullptr) ? attacker->unitDef->id : -1,
			.attackerTeam = (attacker != nullptr) ? attacker->team : -1
		};
		UnitDamagedResult result = {};
		m_UnitDamagedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitHarvestStorageFull(const CUnit* unit) {
	if (m_UnitHarvestStorageFullFuncPtr) {
		UnitHarvestStorageFullQuery query = {.unitID = unit->id};
		UnitHarvestStorageFullResult result = {};
		m_UnitHarvestStorageFullFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitSeismicPing(const CUnit* unit, int allyTeam, const float3& pos, float strength) {
	if (m_UnitSeismicPingFuncPtr) {
		UnitSeismicPingQuery query = {
			.pos = {.x = pos.x, .y = pos.y, .z = pos.z},
			.strength = strength,
			.allyTeam = allyTeam,
			.unitID = unit->id,
			.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1
		};
		UnitSeismicPingResult result = {};
		m_UnitSeismicPingFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

#define DISPATCH_UNIT_LOS_EVENT(EventName)                                      \
	void NativeInterfaceEventClient::EventName(const CUnit* unit, int allyTeam) { \
		if (m_##EventName##FuncPtr) {                                             \
			UnitLosEventQuery query = {.unitID = unit->id, .allyTeam = allyTeam};   \
			UnitLosEventResult result = {};                                        \
			m_##EventName##FuncPtr(m_nativeInterface, m_moduleData, &query, &result); \
		}                                                                          \
	}

DISPATCH_UNIT_LOS_EVENT(UnitEnteredRadar)
DISPATCH_UNIT_LOS_EVENT(UnitEnteredLos)
DISPATCH_UNIT_LOS_EVENT(UnitLeftRadar)
DISPATCH_UNIT_LOS_EVENT(UnitLeftLos)

#undef DISPATCH_UNIT_LOS_EVENT

#define DISPATCH_UNIT_MOVEMENT_CLASS_EVENT(EventName)                          \
	void NativeInterfaceEventClient::EventName(const CUnit* unit) {              \
		if (m_##EventName##FuncPtr) {                                             \
			UnitMovementClassEventQuery query = {.unitID = unit->id};              \
			UnitMovementClassEventResult result = {};                             \
			m_##EventName##FuncPtr(m_nativeInterface, m_moduleData, &query, &result); \
		}                                                                          \
	}

DISPATCH_UNIT_MOVEMENT_CLASS_EVENT(UnitEnteredUnderwater)
DISPATCH_UNIT_MOVEMENT_CLASS_EVENT(UnitEnteredWater)
DISPATCH_UNIT_MOVEMENT_CLASS_EVENT(UnitEnteredAir)
DISPATCH_UNIT_MOVEMENT_CLASS_EVENT(UnitLeftUnderwater)
DISPATCH_UNIT_MOVEMENT_CLASS_EVENT(UnitLeftWater)
DISPATCH_UNIT_MOVEMENT_CLASS_EVENT(UnitLeftAir)

#undef DISPATCH_UNIT_MOVEMENT_CLASS_EVENT

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

void NativeInterfaceEventClient::UnitCloaked(const CUnit* unit) {
	if (m_UnitCloakedFuncPtr) {
		UnitCloakEventQuery query = {.unitID = unit->id};
		UnitCloakEventResult result = {};
		m_UnitCloakedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitDecloaked(const CUnit* unit) {
	if (m_UnitDecloakedFuncPtr) {
		UnitCloakEventQuery query = {.unitID = unit->id};
		UnitCloakEventResult result = {};
		m_UnitDecloakedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

#define DISPATCH_UNIT_MOVE_EVENT(EventName)                                    \
	void NativeInterfaceEventClient::EventName(const CUnit* unit) {              \
		if (m_##EventName##FuncPtr) {                                             \
			UnitMoveEventQuery query = {.unitID = unit->id};                       \
			UnitMoveEventResult result = {};                                      \
			m_##EventName##FuncPtr(m_nativeInterface, m_moduleData, &query, &result); \
		}                                                                          \
	}

DISPATCH_UNIT_MOVE_EVENT(UnitMoved)
DISPATCH_UNIT_MOVE_EVENT(UnitMoveFailed)
DISPATCH_UNIT_MOVE_EVENT(UnitArrivedAtGoal)

#undef DISPATCH_UNIT_MOVE_EVENT

bool NativeInterfaceEventClient::UnitUnitCollision(const CUnit* collider, const CUnit* collidee) {
	if (m_UnitUnitCollisionFuncPtr) {
		UnitUnitCollisionQuery query = {
			.colliderID = (collider != nullptr) ? collider->id : -1,
			.collideeID = (collidee != nullptr) ? collidee->id : -1
		};
		BoolCallinResult result = {.value = false};
		m_UnitUnitCollisionFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value;
	}
	return false;
}

bool NativeInterfaceEventClient::UnitFeatureCollision(const CUnit* collider, const CFeature* collidee) {
	if (m_UnitFeatureCollisionFuncPtr) {
		UnitFeatureCollisionQuery query = {
			.colliderID = (collider != nullptr) ? collider->id : -1,
			.collideeID = (collidee != nullptr) ? collidee->id : -1
		};
		BoolCallinResult result = {.value = false};
		m_UnitFeatureCollisionFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value;
	}
	return false;
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

void NativeInterfaceEventClient::FeatureDamaged(const CFeature* feature, const CUnit* attacker, float damage, int weaponDefID, int projectileID) {
	if (m_FeatureDamagedFuncPtr) {
		FeatureDamagedQuery query = {
			.featureID = feature->id,
			.featureDefID = (feature->def != nullptr) ? feature->def->id : -1,
			.featureTeam = feature->team,
			.damage = damage,
			.weaponDefID = weaponDefID,
			.projectileID = projectileID,
			.attackerID = (attacker != nullptr) ? attacker->id : -1,
			.attackerDefID = (attacker != nullptr && attacker->unitDef != nullptr) ? attacker->unitDef->id : -1,
			.attackerTeam = (attacker != nullptr) ? attacker->team : -1
		};
		FeatureDamagedResult result = {};
		m_FeatureDamagedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::FeatureMoved(const CFeature* feature, const float3& oldpos) {
	if (m_FeatureMovedFuncPtr) {
		FeatureMovedQuery query = {
			.featureID = feature->id,
			.oldPos = {.x = oldpos.x, .y = oldpos.y, .z = oldpos.z}
		};
		FeatureMovedResult result = {};
		m_FeatureMovedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::ProjectileCreated(const CProjectile* proj) {
	if (m_ProjectileCreatedFuncPtr) {
		ProjectileEventQuery query = {.projectileID = proj->id};
		ProjectileEventResult result = {};
		m_ProjectileCreatedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::ProjectileDestroyed(const CProjectile* proj) {
	if (m_ProjectileDestroyedFuncPtr) {
		ProjectileEventQuery query = {.projectileID = proj->id};
		ProjectileEventResult result = {};
		m_ProjectileDestroyedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

bool NativeInterfaceEventClient::Explosion(int weaponID, const WeaponDef* weaponDef, const CExplosionParams& params) {
	(void)weaponDef;
	if (m_ExplosionFuncPtr) {
		ExplosionQuery query = {
			.weaponDefID = weaponID,
			.pos = {.x = params.pos.x, .y = params.pos.y, .z = params.pos.z},
			.ownerID = (params.owner != nullptr) ? params.owner->id : -1,
			.projectileID = static_cast<int32_t>(params.projectileID)
		};
		BoolCallinResult result = {.value = false};
		m_ExplosionFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value;
	}
	return false;
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

void NativeInterfaceEventClient::Save(zipFile archive) {
	if (m_SaveFuncPtr) {
		ArchiveCallinQuery query = {.archive = archive};
		ArchiveCallinResult result = {};
		m_SaveFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::LastMessagePosition(const float3& pos) {
	if (m_LastMessagePositionFuncPtr) {
		LastMessagePositionQuery query = {
			.pos = {.x = pos.x, .y = pos.y, .z = pos.z}
		};
		LastMessagePositionResult result = {};
		m_LastMessagePositionFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnsyncedHeightMapUpdate(const SRectangle& rect) {
	if (m_UnsyncedHeightMapUpdateFuncPtr) {
		RectChangedQuery query = {
			.x1 = rect.x1,
			.z1 = rect.z1,
			.x2 = rect.x2,
			.z2 = rect.z2
		};
		RectChangedResult result = {};
		m_UnsyncedHeightMapUpdateFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

bool NativeInterfaceEventClient::KeyMapChanged() {
	if (m_KeyMapChangedFuncPtr) {
		SimpleCallinQuery query = {};
		BoolCallinResult result = {.value = false};
		m_KeyMapChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value;
	}
	return false;
}

bool NativeInterfaceEventClient::KeyPress(int keyCode, int scanCode, bool isRepeat) {
	if (m_KeyPressFuncPtr) {
		KeyPressQuery query = {.keyCode = keyCode, .scanCode = scanCode, .isRepeat = isRepeat};
		BoolCallinResult result = {.value = false};
		m_KeyPressFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value;
	}
	return false;
}

bool NativeInterfaceEventClient::KeyRelease(int keyCode, int scanCode) {
	if (m_KeyReleaseFuncPtr) {
		KeyReleaseQuery query = {.keyCode = keyCode, .scanCode = scanCode};
		BoolCallinResult result = {.value = false};
		m_KeyReleaseFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value;
	}
	return false;
}

bool NativeInterfaceEventClient::TextInput(const std::string& utf8) {
	if (m_TextInputFuncPtr) {
		TextInputQuery query = {.utf8 = utf8.c_str()};
		BoolCallinResult result = {.value = false};
		m_TextInputFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value;
	}
	return false;
}

bool NativeInterfaceEventClient::TextEditing(const std::string& utf8, unsigned int start, unsigned int length) {
	if (m_TextEditingFuncPtr) {
		TextEditingQuery query = {.utf8 = utf8.c_str(), .start = start, .length = length};
		BoolCallinResult result = {.value = false};
		m_TextEditingFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value;
	}
	return false;
}

bool NativeInterfaceEventClient::MouseMove(int x, int y, int dx, int dy, int button) {
	if (m_MouseMoveFuncPtr) {
		MouseMoveQuery query = {.x = x, .y = y, .dx = dx, .dy = dy, .button = button};
		BoolCallinResult result = {.value = false};
		m_MouseMoveFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value;
	}
	return false;
}

bool NativeInterfaceEventClient::MousePress(int x, int y, int button) {
	if (m_MousePressFuncPtr) {
		MousePressQuery query = {.x = x, .y = y, .button = button};
		BoolCallinResult result = {.value = false};
		m_MousePressFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value;
	}
	return false;
}

void NativeInterfaceEventClient::MouseRelease(int x, int y, int button) {
	if (m_MouseReleaseFuncPtr) {
		MouseReleaseQuery query = {.x = x, .y = y, .button = button};
		MouseReleaseResult result = {};
		m_MouseReleaseFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

bool NativeInterfaceEventClient::MouseWheel(bool up, float value) {
	if (m_MouseWheelFuncPtr) {
		MouseWheelQuery query = {.up = up, .value = value};
		BoolCallinResult result = {.value = false};
		m_MouseWheelFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value;
	}
	return false;
}

bool NativeInterfaceEventClient::IsAbove(int x, int y) {
	if (m_IsAboveFuncPtr) {
		ScreenPositionQuery query = {.x = x, .y = y};
		BoolCallinResult result = {.value = false};
		m_IsAboveFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value;
	}
	return false;
}

std::string NativeInterfaceEventClient::GetTooltip(int x, int y) {
	if (m_GetTooltipFuncPtr) {
		ScreenPositionQuery query = {.x = x, .y = y};
		StringCallinResult result = {};
		m_GetTooltipFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return (result.value != nullptr) ? result.value : "";
	}
	return "";
}

bool NativeInterfaceEventClient::DefaultCommand(const CUnit* unit, const CFeature* feature, int& cmd) {
	if (m_DefaultCommandFuncPtr) {
		DefaultCommandQuery query = {
			.unitID = (unit != nullptr) ? unit->id : -1,
			.featureID = (feature != nullptr) ? feature->id : -1,
			.currentCommand = cmd
		};
		DefaultCommandResult result = {.value = false, .command = cmd};
		m_DefaultCommandFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		if (result.value)
			cmd = result.command;
		return result.value;
	}
	return false;
}

void NativeInterfaceEventClient::ActiveCommandChanged(const SCommandDescription* cmdDesc) {
	if (m_ActiveCommandChangedFuncPtr) {
		ActiveCommandChangedQuery query = {
			.cmdID = (cmdDesc != nullptr) ? cmdDesc->id : -1,
			.cmdType = (cmdDesc != nullptr) ? cmdDesc->type : -1,
			.name = (cmdDesc != nullptr) ? cmdDesc->name.c_str() : "",
			.action = (cmdDesc != nullptr) ? cmdDesc->action.c_str() : "",
			.tooltip = (cmdDesc != nullptr) ? cmdDesc->tooltip.c_str() : ""
		};
		ActiveCommandChangedResult result = {};
		m_ActiveCommandChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::CameraRotationChanged(const float3& rot) {
	if (m_CameraRotationChangedFuncPtr) {
		Float3CallinQuery query = {
			.value = {.x = rot.x, .y = rot.y, .z = rot.z}
		};
		Float3CallinResult result = {};
		m_CameraRotationChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::CameraPositionChanged(const float3& pos) {
	if (m_CameraPositionChangedFuncPtr) {
		Float3CallinQuery query = {
			.value = {.x = pos.x, .y = pos.y, .z = pos.z}
		};
		Float3CallinResult result = {};
		m_CameraPositionChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

bool NativeInterfaceEventClient::CommandNotify(const Command& cmd) {
	if (m_CommandNotifyFuncPtr) {
		CommandNotifyQuery query = {.command = ToNativeCallinCommand(cmd)};
		BoolCallinResult result = {.value = false};
		m_CommandNotifyFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value;
	}
	return false;
}

bool NativeInterfaceEventClient::AddConsoleLine(const std::string& msg, const std::string& section, int level) {
	if (m_AddConsoleLineFuncPtr) {
		AddConsoleLineQuery query = {.message = msg.c_str(), .section = section.c_str(), .level = level};
		BoolCallinResult result = {.value = false};
		m_AddConsoleLineFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value;
	}
	return false;
}

bool NativeInterfaceEventClient::GroupChanged(int groupID) {
	if (m_GroupChangedFuncPtr) {
		GroupChangedQuery query = {.groupID = groupID};
		BoolCallinResult result = {.value = false};
		m_GroupChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value;
	}
	return false;
}

bool NativeInterfaceEventClient::GameSetup(const std::string& state, bool& ready, const std::vector<std::pair<int, std::string>>& playerStates) {
	(void)playerStates;
	if (m_GameSetupFuncPtr) {
		GameSetupQuery query = {.state = state.c_str(), .ready = ready};
		GameSetupResult result = {.handled = false, .ready = ready};
		m_GameSetupFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		if (result.handled)
			ready = result.ready;
		return result.handled;
	}
	return false;
}

std::string NativeInterfaceEventClient::WorldTooltip(const CUnit* unit, const CFeature* feature, const float3* groundPos) {
	if (m_WorldTooltipFuncPtr) {
		WorldTooltipQuery query = {
			.kind = (unit != nullptr) ? 1 : ((feature != nullptr) ? 2 : ((groundPos != nullptr) ? 3 : 0)),
			.unitID = (unit != nullptr) ? unit->id : -1,
			.featureID = (feature != nullptr) ? feature->id : -1,
			.groundPos = (groundPos != nullptr) ? Float3{.x = groundPos->x, .y = groundPos->y, .z = groundPos->z} : Float3{}
		};
		StringCallinResult result = {};
		m_WorldTooltipFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return (result.value != nullptr) ? result.value : "";
	}
	return "";
}

bool NativeInterfaceEventClient::MapDrawCmd(int playerID, int type, const float3* pos0, const float3* pos1, const std::string* label) {
	if (m_MapDrawCmdFuncPtr) {
		MapDrawCmdQuery query = {
			.playerID = playerID,
			.type = type,
			.hasPos0 = (pos0 != nullptr),
			.pos0 = (pos0 != nullptr) ? Float3{.x = pos0->x, .y = pos0->y, .z = pos0->z} : Float3{},
			.hasPos1 = (pos1 != nullptr),
			.pos1 = (pos1 != nullptr) ? Float3{.x = pos1->x, .y = pos1->y, .z = pos1->z} : Float3{},
			.hasLabel = (label != nullptr),
			.label = (label != nullptr) ? label->c_str() : ""
		};
		BoolCallinResult result = {.value = false};
		m_MapDrawCmdFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value;
	}
	return false;
}

void NativeInterfaceEventClient::ViewResize() {
	if (m_ViewResizeFuncPtr) {
		ViewResizeQuery query = {};
		ViewResizeResult result = {};
		m_ViewResizeFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::SunChanged() {
	if (m_SunChangedFuncPtr) {
		SunChangedQuery query = {};
		SunChangedResult result = {};
		m_SunChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::FontsChanged() {
	if (m_FontsChangedFuncPtr) {
		SimpleCallinQuery query = {};
		SimpleCallinResult result = {};
		m_FontsChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::GameProgress(int gameFrame) {
	if (m_GameProgressFuncPtr) {
		GameProgressQuery query = {.gameFrame = gameFrame};
		GameProgressResult result = {};
		m_GameProgressFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::StockpileChanged(const CUnit* unit, const CWeapon* weapon, int oldCount) {
	if (m_StockpileChangedFuncPtr) {
		StockpileChangedQuery query = {
			.unitID = unit->id,
			.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
			.unitTeam = unit->team,
			.weaponNum = (weapon != nullptr) ? weapon->weaponNum + 1 : -1,
			.oldCount = oldCount,
			.newCount = (weapon != nullptr) ? weapon->numStockpiled : -1
		};
		StockpileChangedResult result = {};
		m_StockpileChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::CollectGarbage(bool forced) {
	if (m_CollectGarbageFuncPtr) {
		CollectGarbageQuery query = {.forced = forced};
		CollectGarbageResult result = {};
		m_CollectGarbageFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::Pong(uint8_t pingTag, const spring_time pktSendTime, const spring_time pktRecvTime) {
	if (m_PongFuncPtr) {
		PongQuery query = {
			.pingTag = pingTag,
			.packetSendTimeMillis = pktSendTime.toMilliSecsi(),
			.packetRecvTimeMillis = pktRecvTime.toMilliSecsi()
		};
		PongResult result = {};
		m_PongFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
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

void NativeInterfaceEventClient::HandleLuaCall(const char* msg, size_t msgLength, bool synced) {
	if (m_HandleLuaCallFuncPtr) {
		HandleLuaCallQuery query = {
			.message = msg,
			.messageLength = static_cast<uint32_t>(msgLength),
		};
		HandleLuaCallResult result = {};
		ScopedNativeSyncedCode syncedCode(synced);
		m_HandleLuaCallFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}
