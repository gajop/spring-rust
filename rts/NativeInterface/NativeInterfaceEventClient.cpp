/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "NativeInterfaceEventClient.h"

#include <cstring>

#include "Game/Game.h"
#include "Game/Action.h"
#include "Game/GameHelper.h"
#include "Game/UI/KeySet.h"
#include "Game/UI/MiniMap.h"
#include "Rendering/GlobalRendering.h"
#include "Lua/LuaConfig.h"
#include "Lua/LuaMaterial.h"
#include "Sim/Features/Feature.h"
#include "Sim/Features/FeatureDef.h"
#include "Sim/Projectiles/Projectile.h"
#include "Sim/Projectiles/WeaponProjectiles/WeaponProjectile.h"
#include "Sim/Misc/Resource.h"
#include "Sim/Objects/SolidObject.h"
#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitDef.h"
#include "Sim/Units/BuildInfo.h"
#include "Sim/Units/CommandAI/Command.h"
#include "Sim/Units/CommandAI/CommandDescription.h"
#include "Sim/Weapons/Weapon.h"
#include "System/Log/ILog.h"
#include "System/Input/KeyInput.h"
#include "System/Platform/SDL1_keysym.h"
#include "System/Platform/SharedLib.h"
#include "System/Rectangle.h"

#include <SDL_keyboard.h>
#include <SDL_keycode.h>
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
	struct LuaMousePosition {
		int x;
		int y;
	};

	LuaMousePosition ToLuaMousePosition(int x, int y)
	{
		// CMouseHandler and CEventHandler use renderer coordinates here:
		// x is screen-relative and y is top-origin.  CLuaHandle converts
		// both before invoking Lua callins, so native callbacks use the same
		// view-relative, bottom-origin contract.
		if (globalRendering == nullptr)
			return {x, y};

		return {
			x - globalRendering->viewPosX,
			globalRendering->viewSizeY - y - 1,
		};
	}

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
	LOAD_SYMBOL(CommandFallback);
	LOAD_SYMBOL(AllowCommand);
	LOAD_SYMBOL(AllowUnitCreation);
	LOAD_SYMBOL(AllowUnitTransfer);
	LOAD_SYMBOL(AllowUnitBuildStep);
	LOAD_SYMBOL(AllowUnitCaptureStep);
	LOAD_SYMBOL(AllowUnitTransport);
	LOAD_SYMBOL(AllowUnitTransportLoad);
	LOAD_SYMBOL(AllowUnitTransportUnload);
	LOAD_SYMBOL(AllowUnitCloak);
	LOAD_SYMBOL(AllowUnitDecloak);
	LOAD_SYMBOL(AllowUnitKamikaze);
	LOAD_SYMBOL(AllowFeatureCreation);
	LOAD_SYMBOL(AllowFeatureBuildStep);
	LOAD_SYMBOL(AllowResourceLevel);
	LOAD_SYMBOL(AllowResourceTransfer);
	LOAD_SYMBOL(ResourceExcess);
	LOAD_SYMBOL(AllowDirectUnitControl);
	LOAD_SYMBOL(AllowBuilderHoldFire);
	LOAD_SYMBOL(AllowStartPosition);
	LOAD_SYMBOL(TerraformComplete);
	LOAD_SYMBOL(MoveCtrlNotify);
	LOAD_SYMBOL(AllowWeaponTargetCheck);
	LOAD_SYMBOL(AllowWeaponTarget);
	LOAD_SYMBOL(AllowWeaponInterceptTarget);
	LOAD_SYMBOL(UnitPreDamaged);
	LOAD_SYMBOL(FeaturePreDamaged);
	LOAD_SYMBOL(ShieldPreDamaged);
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
	LOAD_SYMBOL(DrawUnit);
	LOAD_SYMBOL(DrawFeature);
	LOAD_SYMBOL(DrawShield);
	LOAD_SYMBOL(DrawProjectile);
	LOAD_SYMBOL(DrawMaterial);
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
	LOAD_SYMBOL(MiniMapRotationChanged);
	LOAD_SYMBOL(MiniMapStateChanged);
	LOAD_SYMBOL(MiniMapGeometryChanged);
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
		UpdateQuery query = {
			.deltaSeconds = (game != nullptr) ? game->updateDeltaSeconds : 0.0f,
		};
		UpdateResult result = {};
		m_UpdateFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::DrawScreen() {
	if (m_DrawScreenFuncPtr) {
		DrawScreenQuery query = {
			.viewSizeX = (globalRendering != nullptr) ? globalRendering->viewSizeX : 0,
			.viewSizeY = (globalRendering != nullptr) ? globalRendering->viewSizeY : 0,
		};
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
DISPATCH_SIMPLE_CALLIN(DrawShadowUnitsLua)
DISPATCH_SIMPLE_CALLIN(DrawShadowFeaturesLua)

#undef DISPATCH_SIMPLE_CALLIN

#define DISPATCH_SCREEN_CALLIN(EventName)                                      \
	void NativeInterfaceEventClient::EventName() {                               \
		if (m_##EventName##FuncPtr) {                                               \
			DrawScreenQuery query = {                                                  \
				.viewSizeX = (globalRendering != nullptr) ? globalRendering->viewSizeX : 0, \
				.viewSizeY = (globalRendering != nullptr) ? globalRendering->viewSizeY : 0, \
			};                                                                         \
			DrawScreenResult result = {};                                              \
			m_##EventName##FuncPtr(m_nativeInterface, m_moduleData, &query, &result);  \
		}                                                                            \
	}

DISPATCH_SCREEN_CALLIN(DrawScreenEffects)
DISPATCH_SCREEN_CALLIN(DrawScreenPost)

#undef DISPATCH_SCREEN_CALLIN

#define DISPATCH_MINIMAP_DRAW_CALLIN(EventName)                                \
	void NativeInterfaceEventClient::EventName() {                               \
		if (m_##EventName##FuncPtr) {                                               \
			MiniMapDrawQuery query = {                                                 \
				.sizeX = (minimap != nullptr) ? minimap->GetSizeX() : 0,                  \
				.sizeY = (minimap != nullptr) ? minimap->GetSizeY() : 0,                  \
			};                                                                         \
			SimpleCallinResult result = {};                                            \
			m_##EventName##FuncPtr(m_nativeInterface, m_moduleData, &query, &result);  \
		}                                                                            \
	}

DISPATCH_MINIMAP_DRAW_CALLIN(DrawInMiniMap)
DISPATCH_MINIMAP_DRAW_CALLIN(DrawInMiniMapBackground)

#undef DISPATCH_MINIMAP_DRAW_CALLIN

bool NativeInterfaceEventClient::DrawUnit(const CUnit* unit) {
	if (m_DrawUnitFuncPtr == nullptr)
		return false;

	DrawUnitQuery query = {
		.unitID = (unit != nullptr) ? unit->id : -1,
		.drawMode = (game != nullptr) ? static_cast<int>(game->GetDrawMode()) : 0,
	};
	BoolCallinResult result = {.value = false};
	m_DrawUnitFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

bool NativeInterfaceEventClient::DrawFeature(const CFeature* feature) {
	if (m_DrawFeatureFuncPtr == nullptr)
		return false;

	DrawFeatureQuery query = {
		.featureID = (feature != nullptr) ? feature->id : -1,
		.drawMode = (game != nullptr) ? static_cast<int>(game->GetDrawMode()) : 0,
	};
	BoolCallinResult result = {.value = false};
	m_DrawFeatureFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

bool NativeInterfaceEventClient::DrawShield(const CUnit* unit, const CWeapon* weapon) {
	if (m_DrawShieldFuncPtr == nullptr)
		return false;

	DrawShieldQuery query = {
		.unitID = (unit != nullptr) ? unit->id : -1,
		.weaponID = (weapon != nullptr) ? weapon->weaponNum + LUA_WEAPON_BASE_INDEX : -1,
		.drawMode = (game != nullptr) ? static_cast<int>(game->GetDrawMode()) : 0,
	};
	BoolCallinResult result = {.value = false};
	m_DrawShieldFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

bool NativeInterfaceEventClient::DrawProjectile(const CProjectile* projectile) {
	if (m_DrawProjectileFuncPtr == nullptr)
		return false;

	DrawProjectileQuery query = {
		.projectileID = (projectile != nullptr) ? projectile->id : -1,
		.drawMode = (game != nullptr) ? static_cast<int>(game->GetDrawMode()) : 0,
	};
	BoolCallinResult result = {.value = false};
	m_DrawProjectileFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

bool NativeInterfaceEventClient::DrawMaterial(const LuaMaterial* material) {
	if (m_DrawMaterialFuncPtr == nullptr)
		return false;

	DrawMaterialQuery query = {
		.uuid = (material != nullptr) ? material->uuid : -1,
		.drawMode = (game != nullptr) ? static_cast<int>(game->GetDrawMode()) : 0,
	};
	BoolCallinResult result = {.value = false};
	m_DrawMaterialFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

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
			.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
			.unitTeam = unit->team,
			.builderID = builder != nullptr ? builder->id : -1
		};
		UnitCreatedResult result = {};
		m_UnitCreatedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitFinished(const CUnit* unit) {
	if (m_UnitFinishedFuncPtr) {
		UnitFinishedQuery query = {
			.unitID = unit->id,
			.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
			.unitTeam = unit->team,
		};
		UnitFinishedResult result = {};
		m_UnitFinishedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitReverseBuilt(const CUnit* unit) {
	if (m_UnitReverseBuiltFuncPtr) {
		UnitReverseBuiltQuery query = {
			.unitID = unit->id,
			.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
			.unitTeam = unit->team,
		};
		UnitReverseBuiltResult result = {};
		m_UnitReverseBuiltFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitConstructionDecayed(const CUnit* unit, float timeSinceLastBuild, float iterationPeriod, float part) {
	if (m_UnitConstructionDecayedFuncPtr) {
		UnitConstructionDecayedQuery query = {
			.unitID = unit->id,
			.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
			.unitTeam = unit->team,
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
			.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
			.unitTeam = unit->team,
			.factoryID = factory->id,
			.factoryDefID = (factory->unitDef != nullptr) ? factory->unitDef->id : -1,
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
			.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
			.unitTeam = unit->team,
			.attackerID = attacker != nullptr ? attacker->id : -1,
			.attackerDefID = (attacker != nullptr && attacker->unitDef != nullptr) ? attacker->unitDef->id : -1,
			.attackerTeam = attacker != nullptr ? attacker->team : -1,
			.weaponDefID = weaponDefID,
		};
		UnitDestroyedResult result = {};
		m_UnitDestroyedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitTaken(const CUnit* unit, int oldTeam, int newTeam) {
	if (m_UnitTakenFuncPtr) {
		UnitTakenQuery query = {
			.unitID = unit->id,
			.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
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
			.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
			.oldTeam = oldTeam,
			.newTeam = newTeam
		};
		UnitGivenResult result = {};
		m_UnitGivenFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitIdle(const CUnit* unit) {
	if (m_UnitIdleFuncPtr) {
		UnitIdleQuery query = {
			.unitID = unit->id,
			.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
			.unitTeam = unit->team,
		};
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

bool NativeInterfaceEventClient::CommandFallback(const CUnit* unit, const Command& command) {
	if (m_CommandFallbackFuncPtr == nullptr)
		return false;

	CommandFallbackQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
		.command = ToNativeCallinCommand(command),
	};
	BoolCallinResult result = {.value = false};
	m_CommandFallbackFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

bool NativeInterfaceEventClient::AllowCommand(const CUnit* unit, const Command& command, int playerNum, bool fromSynced, bool fromLua) {
	if (m_AllowCommandFuncPtr == nullptr)
		return true;

	UnitCommandQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
		.command = ToNativeCallinCommand(command),
		.playerNum = playerNum,
		.fromSynced = fromSynced,
		.fromLua = fromLua,
	};
	BoolCallinResult result = {.value = true};
	m_AllowCommandFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

std::pair<bool, bool> NativeInterfaceEventClient::AllowUnitCreation(const UnitDef* unitDef, const CUnit* builder, const BuildInfo* buildInfo) {
	if (m_AllowUnitCreationFuncPtr == nullptr)
		return {true, true};

	AllowUnitCreationQuery query = {
		.unitDefID = (unitDef != nullptr) ? unitDef->id : -1,
		.builderID = (builder != nullptr) ? builder->id : -1,
		.builderTeam = (builder != nullptr) ? builder->team : -1,
		.hasBuildInfo = (buildInfo != nullptr),
		.buildPos = (buildInfo != nullptr) ? Float3{buildInfo->pos.x, buildInfo->pos.y, buildInfo->pos.z} : Float3{},
		.buildFacing = (buildInfo != nullptr) ? buildInfo->buildFacing : 0,
	};
	AllowUnitCreationResult result = {.allow = true, .dropOrder = true};
	m_AllowUnitCreationFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return {result.allow, result.dropOrder};
}

bool NativeInterfaceEventClient::AllowUnitTransfer(const CUnit* unit, int newTeam, bool capture) {
	if (m_AllowUnitTransferFuncPtr == nullptr)
		return true;

	AllowUnitTransferQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.oldTeam = unit->team,
		.newTeam = newTeam,
		.capture = capture,
	};
	BoolCallinResult result = {.value = true};
	m_AllowUnitTransferFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

bool NativeInterfaceEventClient::AllowUnitBuildStep(const CUnit* builder, const CUnit* unit, float part) {
	if (m_AllowUnitBuildStepFuncPtr == nullptr)
		return true;

	AllowUnitBuildStepQuery query = {
		.builderID = builder->id,
		.builderTeam = builder->team,
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.part = part,
	};
	BoolCallinResult result = {.value = true};
	m_AllowUnitBuildStepFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

bool NativeInterfaceEventClient::AllowUnitCaptureStep(const CUnit* builder, const CUnit* unit, float part) {
	if (m_AllowUnitCaptureStepFuncPtr == nullptr)
		return true;

	AllowUnitBuildStepQuery query = {
		.builderID = builder->id,
		.builderTeam = builder->team,
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.part = part,
	};
	BoolCallinResult result = {.value = true};
	m_AllowUnitCaptureStepFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

bool NativeInterfaceEventClient::AllowUnitTransport(const CUnit* transporter, const CUnit* transportee) {
	if (m_AllowUnitTransportFuncPtr == nullptr)
		return true;

	AllowUnitTransportQuery query = {
		.transporterID = transporter->id,
		.transporterDefID = (transporter->unitDef != nullptr) ? transporter->unitDef->id : -1,
		.transporterTeam = transporter->team,
		.transporteeID = transportee->id,
		.transporteeDefID = (transportee->unitDef != nullptr) ? transportee->unitDef->id : -1,
		.transporteeTeam = transportee->team,
	};
	BoolCallinResult result = {.value = true};
	m_AllowUnitTransportFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

bool NativeInterfaceEventClient::AllowUnitTransportLoad(const CUnit* transporter, const CUnit* transportee, const float3& loadPos, bool allowed) {
	if (m_AllowUnitTransportLoadFuncPtr == nullptr)
		return allowed;

	AllowUnitTransportPositionQuery query = {
		.units = {
			.transporterID = transporter->id,
			.transporterDefID = (transporter->unitDef != nullptr) ? transporter->unitDef->id : -1,
			.transporterTeam = transporter->team,
			.transporteeID = transportee->id,
			.transporteeDefID = (transportee->unitDef != nullptr) ? transportee->unitDef->id : -1,
			.transporteeTeam = transportee->team,
		},
		.position = {.x = loadPos.x, .y = loadPos.y, .z = loadPos.z},
		.allowed = allowed,
	};
	BoolCallinResult result = {.value = allowed};
	m_AllowUnitTransportLoadFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

bool NativeInterfaceEventClient::AllowUnitTransportUnload(const CUnit* transporter, const CUnit* transportee, const float3& unloadPos, bool allowed) {
	if (m_AllowUnitTransportUnloadFuncPtr == nullptr)
		return allowed;

	AllowUnitTransportPositionQuery query = {
		.units = {
			.transporterID = transporter->id,
			.transporterDefID = (transporter->unitDef != nullptr) ? transporter->unitDef->id : -1,
			.transporterTeam = transporter->team,
			.transporteeID = transportee->id,
			.transporteeDefID = (transportee->unitDef != nullptr) ? transportee->unitDef->id : -1,
			.transporteeTeam = transportee->team,
		},
		.position = {.x = unloadPos.x, .y = unloadPos.y, .z = unloadPos.z},
		.allowed = allowed,
	};
	BoolCallinResult result = {.value = allowed};
	m_AllowUnitTransportUnloadFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

bool NativeInterfaceEventClient::AllowUnitCloak(const CUnit* unit, const CUnit* enemy) {
	if (m_AllowUnitCloakFuncPtr == nullptr)
		return true;

	AllowUnitCloakQuery query = {
		.unitID = unit->id,
		.hasEnemy = (enemy != nullptr),
		.enemyID = (enemy != nullptr) ? enemy->id : -1,
	};
	BoolCallinResult result = {.value = true};
	m_AllowUnitCloakFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

bool NativeInterfaceEventClient::AllowUnitDecloak(const CUnit* unit, const CSolidObject* object, const CWeapon* weapon) {
	if (m_AllowUnitDecloakFuncPtr == nullptr)
		return true;

	AllowUnitDecloakQuery query = {
		.unitID = unit->id,
		.hasObject = (object != nullptr),
		.objectID = (object != nullptr) ? object->id : -1,
		.hasWeapon = (weapon != nullptr),
		.weaponNum = (weapon != nullptr) ? weapon->weaponNum : -1,
	};
	BoolCallinResult result = {.value = true};
	m_AllowUnitDecloakFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

bool NativeInterfaceEventClient::AllowUnitKamikaze(const CUnit* unit, const CUnit* target, bool allowed) {
	if (m_AllowUnitKamikazeFuncPtr == nullptr)
		return allowed;

	AllowUnitKamikazeQuery query = {
		.unitID = unit->id,
		.targetID = (target != nullptr) ? target->id : -1,
		.allowed = allowed,
	};
	BoolCallinResult result = {.value = allowed};
	m_AllowUnitKamikazeFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
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
		UnitHarvestStorageFullQuery query = {
			.unitID = unit->id,
			.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
			.unitTeam = unit->team,
		};
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
			UnitLosEventQuery query = {                                               \
				.unitID = unit->id,                                                       \
				.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,       \
				.unitTeam = unit->team,                                                  \
				.allyTeam = allyTeam,                                                     \
			};                                                                          \
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
			UnitMovementClassEventQuery query = {                                    \
				.unitID = unit->id,                                                       \
				.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,       \
				.unitTeam = unit->team,                                                  \
			};                                                                          \
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
			.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
			.unitTeam = unit->team,
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
			.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
			.unitTeam = unit->team,
			.experience = unit->experience,
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
			.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
			.unitTeam = unit->team,
			.transportID = transport->id,
			.transportTeam = transport->team,
		};
		UnitLoadedResult result = {};
		m_UnitLoadedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitUnloaded(const CUnit* unit, const CUnit* transport) {
	if (m_UnitUnloadedFuncPtr) {
		UnitUnloadedQuery query = {
			.unitID = unit->id,
			.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
			.unitTeam = unit->team,
			.transportID = transport->id,
			.transportTeam = transport->team,
		};
		UnitUnloadedResult result = {};
		m_UnitUnloadedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitCloaked(const CUnit* unit) {
	if (m_UnitCloakedFuncPtr) {
		UnitCloakEventQuery query = {
			.unitID = unit->id,
			.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
			.unitTeam = unit->team,
		};
		UnitCloakEventResult result = {};
		m_UnitCloakedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitDecloaked(const CUnit* unit) {
	if (m_UnitDecloakedFuncPtr) {
		UnitCloakEventQuery query = {
			.unitID = unit->id,
			.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
			.unitTeam = unit->team,
		};
		UnitCloakEventResult result = {};
		m_UnitDecloakedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

#define DISPATCH_UNIT_MOVE_EVENT(EventName)                                    \
	void NativeInterfaceEventClient::EventName(const CUnit* unit) {              \
		if (m_##EventName##FuncPtr) {                                             \
			UnitMoveEventQuery query = {                                            \
				.unitID = unit->id,                                                       \
				.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,       \
				.unitTeam = unit->team,                                                  \
			};                                                                          \
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
		RenderUnitDestroyedQuery query = {
			.unitID = unit->id,
			.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
			.unitTeam = unit->team,
		};
		RenderUnitDestroyedResult result = {};
		m_RenderUnitDestroyedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::FeatureCreated(const CFeature* feature) {
	if (m_FeatureCreatedFuncPtr) {
		FeatureCreatedQuery query = {
			.featureID = feature->id,
			.allyTeamID = feature->allyteam,
		};
		FeatureCreatedResult result = {};
		m_FeatureCreatedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::FeatureDestroyed(const CFeature* feature) {
	if (m_FeatureDestroyedFuncPtr) {
		FeatureDestroyedQuery query = {
			.featureID = feature->id,
			.allyTeamID = feature->allyteam,
		};
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

bool NativeInterfaceEventClient::AllowFeatureCreation(const FeatureDef* featureDef, int allyTeamID, const float3& pos) {
	if (m_AllowFeatureCreationFuncPtr == nullptr)
		return true;

	AllowFeatureCreationQuery query = {
		.featureDefID = (featureDef != nullptr) ? featureDef->id : -1,
		.teamID = allyTeamID,
		.position = {.x = pos.x, .y = pos.y, .z = pos.z},
	};
	BoolCallinResult result = {.value = true};
	m_AllowFeatureCreationFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

bool NativeInterfaceEventClient::AllowFeatureBuildStep(const CUnit* builder, const CFeature* feature, float part) {
	if (m_AllowFeatureBuildStepFuncPtr == nullptr)
		return true;

	AllowFeatureBuildStepQuery query = {
		.builderID = builder->id,
		.builderTeam = builder->team,
		.featureID = feature->id,
		.featureDefID = (feature->def != nullptr) ? feature->def->id : -1,
		.part = part,
	};
	BoolCallinResult result = {.value = true};
	m_AllowFeatureBuildStepFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

bool NativeInterfaceEventClient::AllowResourceLevel(int teamID, const std::string& type, float level) {
	if (m_AllowResourceLevelFuncPtr == nullptr)
		return true;

	AllowResourceLevelQuery query = {
		.teamID = teamID,
		.type = type.c_str(),
		.level = level,
	};
	BoolCallinResult result = {.value = true};
	m_AllowResourceLevelFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

bool NativeInterfaceEventClient::AllowResourceTransfer(int oldTeam, int newTeam, const char* type, float amount) {
	if (m_AllowResourceTransferFuncPtr == nullptr)
		return true;

	AllowResourceTransferQuery query = {
		.oldTeam = oldTeam,
		.newTeam = newTeam,
		.type = type,
		.amount = amount,
	};
	BoolCallinResult result = {.value = true};
	m_AllowResourceTransferFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

bool NativeInterfaceEventClient::ResourceExcess(const std::map<int, SResourcePack>& excess) {
	if (m_ResourceExcessFuncPtr == nullptr)
		return false;

	std::vector<ResourceExcessEntry> entries;
	entries.reserve(excess.size());
	for (const auto& [teamID, resources] : excess) {
		entries.push_back({
			.teamID = teamID,
			.resources = {resources[0], resources[1]},
		});
	}

	ResourceExcessQuery query = {
		.entries = entries.data(),
		.count = static_cast<uint32_t>(entries.size()),
	};
	BoolCallinResult result = {.value = false};
	m_ResourceExcessFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

bool NativeInterfaceEventClient::AllowDirectUnitControl(int playerID, const CUnit* unit) {
	if (m_AllowDirectUnitControlFuncPtr == nullptr)
		return true;

	AllowDirectUnitControlQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
		.playerID = playerID,
	};
	BoolCallinResult result = {.value = true};
	m_AllowDirectUnitControlFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

bool NativeInterfaceEventClient::AllowBuilderHoldFire(const CUnit* unit, int action) {
	if (m_AllowBuilderHoldFireFuncPtr == nullptr)
		return true;

	AllowBuilderHoldFireQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.action = action,
	};
	BoolCallinResult result = {.value = true};
	m_AllowBuilderHoldFireFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

bool NativeInterfaceEventClient::AllowStartPosition(int playerID, int teamID, unsigned char readyState, const float3& clampedPos, const float3& rawPickPos) {
	if (m_AllowStartPositionFuncPtr == nullptr)
		return true;

	AllowStartPositionQuery query = {
		.playerID = playerID,
		.teamID = teamID,
		.readyState = readyState,
		.clampedPos = {.x = clampedPos.x, .y = clampedPos.y, .z = clampedPos.z},
		.rawPickPos = {.x = rawPickPos.x, .y = rawPickPos.y, .z = rawPickPos.z},
	};
	BoolCallinResult result = {.value = true};
	m_AllowStartPositionFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

bool NativeInterfaceEventClient::TerraformComplete(const CUnit* unit, const CUnit* build) {
	if (m_TerraformCompleteFuncPtr == nullptr)
		return false;

	TerraformCompleteQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
		.buildUnitID = build->id,
		.buildUnitDefID = (build->unitDef != nullptr) ? build->unitDef->id : -1,
		.buildUnitTeam = build->team,
	};
	BoolCallinResult result = {.value = false};
	m_TerraformCompleteFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

bool NativeInterfaceEventClient::MoveCtrlNotify(const CUnit* unit, int data) {
	if (m_MoveCtrlNotifyFuncPtr == nullptr)
		return false;

	MoveCtrlNotifyQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
		.data = data,
	};
	BoolCallinResult result = {.value = false};
	m_MoveCtrlNotifyFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
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
		const auto* weaponProjectile = proj->weapon ? static_cast<const CWeaponProjectile*>(proj) : nullptr;
		const auto* weaponDef = (weaponProjectile != nullptr) ? weaponProjectile->GetWeaponDef() : nullptr;
		ProjectileEventQuery query = {
			.projectileID = proj->id,
			.ownerID = static_cast<int32_t>(proj->GetOwnerID()),
			.weaponDefID = (weaponDef != nullptr) ? weaponDef->id : -1,
		};
		ProjectileEventResult result = {};
		m_ProjectileCreatedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::ProjectileDestroyed(const CProjectile* proj) {
	if (m_ProjectileDestroyedFuncPtr) {
		const auto* weaponProjectile = proj->weapon ? static_cast<const CWeaponProjectile*>(proj) : nullptr;
		const auto* weaponDef = (weaponProjectile != nullptr) ? weaponProjectile->GetWeaponDef() : nullptr;
		ProjectileEventQuery query = {
			.projectileID = proj->id,
			.ownerID = static_cast<int32_t>(proj->GetOwnerID()),
			.weaponDefID = (weaponDef != nullptr) ? weaponDef->id : -1,
		};
		ProjectileEventResult result = {};
		m_ProjectileDestroyedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

bool NativeInterfaceEventClient::Explosion(int weaponID, const WeaponDef* weaponDef, const CExplosionParams& params) {
	(void)weaponDef;
	// Lua's CLuaHandle::Explosion does not dispatch piece-projectile
	// explosions, which are represented by a negative weapon definition ID.
	if (weaponID < 0)
		return false;
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

int NativeInterfaceEventClient::AllowWeaponTargetCheck(unsigned int attackerID, unsigned int attackerWeaponNum, unsigned int attackerWeaponDefID) {
	if (m_AllowWeaponTargetCheckFuncPtr == nullptr)
		return -1;

	AllowWeaponTargetCheckQuery query = {
		.attackerID = static_cast<int32_t>(attackerID),
		.attackerWeaponNum = static_cast<int32_t>(attackerWeaponNum + LUA_WEAPON_BASE_INDEX),
		.attackerWeaponDefID = static_cast<int32_t>(attackerWeaponDefID),
	};
	IntCallinResult result = {.value = -1};
	m_AllowWeaponTargetCheckFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

bool NativeInterfaceEventClient::AllowWeaponTarget(unsigned int attackerID, unsigned int targetID, unsigned int attackerWeaponNum, unsigned int attackerWeaponDefID, float* targetPriority) {
	if (m_AllowWeaponTargetFuncPtr == nullptr)
		return true;

	const int attackerWeaponNumber = static_cast<int>(attackerWeaponNum);
	AllowWeaponTargetQuery query = {
		.attackerID = static_cast<int32_t>(attackerID),
		.targetID = static_cast<int32_t>(targetID),
		.attackerWeaponNum = attackerWeaponNumber + LUA_WEAPON_BASE_INDEX * (attackerWeaponNumber >= 0),
		.attackerWeaponDefID = static_cast<int32_t>(attackerWeaponDefID),
		.hasTargetPriority = (targetPriority != nullptr),
		.targetPriority = (targetPriority != nullptr) ? *targetPriority : 0.0f,
	};
	AllowWeaponTargetResult result = {
		.allowed = true,
		.targetPriority = query.targetPriority,
	};
	m_AllowWeaponTargetFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	if (targetPriority != nullptr)
		*targetPriority = result.targetPriority;
	return result.allowed;
}

bool NativeInterfaceEventClient::AllowWeaponInterceptTarget(const CUnit* interceptorUnit, const CWeapon* interceptorWeapon, const CProjectile* interceptorTarget) {
	if (m_AllowWeaponInterceptTargetFuncPtr == nullptr)
		return true;

	AllowWeaponInterceptTargetQuery query = {
		.interceptorUnitID = (interceptorUnit != nullptr) ? interceptorUnit->id : -1,
		.interceptorWeaponID = (interceptorWeapon != nullptr) ? interceptorWeapon->weaponNum + LUA_WEAPON_BASE_INDEX : -1,
		.interceptorTargetID = (interceptorTarget != nullptr) ? interceptorTarget->id : -1,
	};
	BoolCallinResult result = {.value = true};
	m_AllowWeaponInterceptTargetFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
}

bool NativeInterfaceEventClient::UnitPreDamaged(const CUnit* unit, const CUnit* attacker, float damage, int weaponDefID, int projectileID, bool paralyzer, float* newDamage, float* impulseMult) {
	if (m_UnitPreDamagedFuncPtr == nullptr)
		return false;

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
		.attackerTeam = (attacker != nullptr) ? attacker->team : -1,
	};
	DamageCallinResult result = {
		.newDamage = (newDamage != nullptr) ? *newDamage : damage,
		.impulseMult = (impulseMult != nullptr) ? *impulseMult : 1.0f,
	};
	m_UnitPreDamagedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	if (newDamage != nullptr)
		*newDamage = result.newDamage;
	if (impulseMult != nullptr)
		*impulseMult = result.impulseMult;
	return result.newDamage == 0.0f && result.impulseMult == 0.0f;
}

bool NativeInterfaceEventClient::FeaturePreDamaged(const CFeature* feature, const CUnit* attacker, float damage, int weaponDefID, int projectileID, float* newDamage, float* impulseMult) {
	if (m_FeaturePreDamagedFuncPtr == nullptr)
		return false;

	FeatureDamagedQuery query = {
		.featureID = feature->id,
		.featureDefID = (feature->def != nullptr) ? feature->def->id : -1,
		.featureTeam = feature->team,
		.damage = damage,
		.weaponDefID = weaponDefID,
		.projectileID = projectileID,
		.attackerID = (attacker != nullptr) ? attacker->id : -1,
		.attackerDefID = (attacker != nullptr && attacker->unitDef != nullptr) ? attacker->unitDef->id : -1,
		.attackerTeam = (attacker != nullptr) ? attacker->team : -1,
	};
	DamageCallinResult result = {
		.newDamage = (newDamage != nullptr) ? *newDamage : damage,
		.impulseMult = (impulseMult != nullptr) ? *impulseMult : 1.0f,
	};
	m_FeaturePreDamagedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	if (newDamage != nullptr)
		*newDamage = result.newDamage;
	if (impulseMult != nullptr)
		*impulseMult = result.impulseMult;
	return result.newDamage == 0.0f && result.impulseMult == 0.0f;
}

bool NativeInterfaceEventClient::ShieldPreDamaged(const CProjectile* projectile, const CWeapon* shieldEmitter, const CUnit* shieldCarrier, bool bounceProjectile, const CWeapon* beamEmitter, const CUnit* beamCarrier, const float3& startPos, const float3& hitPos) {
	if (m_ShieldPreDamagedFuncPtr == nullptr)
		return false;

	ShieldPreDamagedQuery query = {
		.projectileID = (projectile != nullptr) ? projectile->id : -1,
		.projectileOwnerID = (projectile != nullptr) ? static_cast<int32_t>(projectile->GetOwnerID()) : -1,
		.shieldWeaponNum = (shieldEmitter != nullptr) ? shieldEmitter->weaponNum + LUA_WEAPON_BASE_INDEX : -1,
		.shieldCarrierID = (shieldCarrier != nullptr) ? shieldCarrier->id : -1,
		.bounceProjectile = bounceProjectile,
		.beamEmitterWeaponNum = (beamEmitter != nullptr) ? beamEmitter->weaponNum + LUA_WEAPON_BASE_INDEX : -1,
		.beamEmitterUnitID = (beamCarrier != nullptr) ? beamCarrier->id : -1,
		.startPos = {.x = startPos.x, .y = startPos.y, .z = startPos.z},
		.hitPos = {.x = hitPos.x, .y = hitPos.y, .z = hitPos.z},
	};
	BoolCallinResult result = {.value = false};
	m_ShieldPreDamagedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value;
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
		const ActionList& actionList = (game != nullptr) ? game->GetLastActionList() : ActionList{};
		std::vector<KeyAction> actions;
		actions.reserve(actionList.size());
		for (const Action& action : actionList) {
			actions.push_back({
				.command = action.command.c_str(),
				.extra = action.extra.c_str(),
				.boundWith = action.boundWith.c_str(),
			});
		}
		const CKeySet keySet(keyCode);
		const std::string label = keySet.GetString(true);
		KeyPressQuery query = {
			.keyCode = SDL21_keysyms(keyCode),
			.alt = !!KeyInput::GetKeyModState(KMOD_ALT),
			.ctrl = !!KeyInput::GetKeyModState(KMOD_CTRL),
			.meta = !!KeyInput::GetKeyModState(KMOD_GUI),
			.shift = !!KeyInput::GetKeyModState(KMOD_SHIFT),
			.isRepeat = isRepeat,
			.label = label.c_str(),
			.utf32Char = 0,
			.scanCode = scanCode,
			.actionList = actions.data(),
			.actionCount = static_cast<uint32_t>(actions.size()),
		};
		BoolCallinResult result = {.value = false};
		m_KeyPressFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value;
	}
	return false;
}

bool NativeInterfaceEventClient::KeyRelease(int keyCode, int scanCode) {
	if (m_KeyReleaseFuncPtr) {
		const ActionList& actionList = (game != nullptr) ? game->GetLastActionList() : ActionList{};
		std::vector<KeyAction> actions;
		actions.reserve(actionList.size());
		for (const Action& action : actionList) {
			actions.push_back({
				.command = action.command.c_str(),
				.extra = action.extra.c_str(),
				.boundWith = action.boundWith.c_str(),
			});
		}
		const CKeySet keySet(keyCode);
		const std::string label = keySet.GetString(true);
		KeyReleaseQuery query = {
			.keyCode = SDL21_keysyms(keyCode),
			.alt = !!KeyInput::GetKeyModState(KMOD_ALT),
			.ctrl = !!KeyInput::GetKeyModState(KMOD_CTRL),
			.meta = !!KeyInput::GetKeyModState(KMOD_GUI),
			.shift = !!KeyInput::GetKeyModState(KMOD_SHIFT),
			.label = label.c_str(),
			.utf32Char = 0,
			.scanCode = scanCode,
			.actionList = actions.data(),
			.actionCount = static_cast<uint32_t>(actions.size()),
		};
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
		const LuaMousePosition position = ToLuaMousePosition(x, y);
		MouseMoveQuery query = {
			.x = position.x,
			.y = position.y,
			.dx = dx,
			.dy = -dy,
			.button = button,
		};
		BoolCallinResult result = {.value = false};
		m_MouseMoveFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value;
	}
	return false;
}

bool NativeInterfaceEventClient::MousePress(int x, int y, int button) {
	if (m_MousePressFuncPtr) {
		const LuaMousePosition position = ToLuaMousePosition(x, y);
		MousePressQuery query = {.x = position.x, .y = position.y, .button = button};
		BoolCallinResult result = {.value = false};
		m_MousePressFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value;
	}
	return false;
}

void NativeInterfaceEventClient::MouseRelease(int x, int y, int button) {
	if (m_MouseReleaseFuncPtr) {
		const LuaMousePosition position = ToLuaMousePosition(x, y);
		MouseReleaseQuery query = {.x = position.x, .y = position.y, .button = button};
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
		const LuaMousePosition position = ToLuaMousePosition(x, y);
		ScreenPositionQuery query = {.x = position.x, .y = position.y};
		BoolCallinResult result = {.value = false};
		m_IsAboveFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value;
	}
	return false;
}

std::string NativeInterfaceEventClient::GetTooltip(int x, int y) {
	if (m_GetTooltipFuncPtr) {
		const LuaMousePosition position = ToLuaMousePosition(x, y);
		ScreenPositionQuery query = {.x = position.x, .y = position.y};
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

void NativeInterfaceEventClient::MiniMapRotationChanged(float newRot, float oldRot) {
	if (m_MiniMapRotationChangedFuncPtr == nullptr)
		return;

	MiniMapRotationChangedQuery query = {.newRot = newRot, .oldRot = oldRot};
	SimpleCallinResult result = {};
	m_MiniMapRotationChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
}

void NativeInterfaceEventClient::MiniMapStateChanged(bool isMinimized, bool isMaximized, bool isSlaved) {
	if (m_MiniMapStateChangedFuncPtr == nullptr)
		return;

	MiniMapStateChangedQuery query = {
		.isMinimized = isMinimized,
		.isMaximized = isMaximized,
		.isSlaved = isSlaved,
	};
	SimpleCallinResult result = {};
	m_MiniMapStateChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
}

void NativeInterfaceEventClient::MiniMapGeometryChanged(int2 newPos, int2 newDim, int2 oldPos, int2 oldDim) {
	if (m_MiniMapGeometryChangedFuncPtr == nullptr)
		return;

	MiniMapGeometryChangedQuery query = {
		.newPosX = newPos.x,
		.newPosY = newPos.y,
		.newDimX = newDim.x,
		.newDimY = newDim.y,
		.oldPosX = oldPos.x,
		.oldPosY = oldPos.y,
		.oldDimX = oldDim.x,
		.oldDimY = oldDim.y,
	};
	SimpleCallinResult result = {};
	m_MiniMapGeometryChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
}

bool NativeInterfaceEventClient::GameSetup(const std::string& state, bool& ready, const std::vector<std::pair<int, std::string>>& playerStates) {
	if (m_GameSetupFuncPtr) {
		std::vector<GameSetupPlayerState> states;
		states.reserve(playerStates.size());
		for (const auto& [playerID, playerState] : playerStates) {
			states.push_back({
				.playerID = playerID,
				.state = playerState.c_str(),
			});
		}
		GameSetupQuery query = {
			.state = state.c_str(),
			.ready = ready,
			.playerStates = states.data(),
			.playerStateCount = static_cast<uint32_t>(states.size()),
		};
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
		const int winPosY_bl = (globalRendering != nullptr)
			? globalRendering->screenSizeY - globalRendering->winSizeY - globalRendering->winPosY
			: 0;
		ViewResizeQuery query = {
			.screenSizeX = (globalRendering != nullptr) ? globalRendering->screenSizeX : 0,
			.screenSizeY = (globalRendering != nullptr) ? globalRendering->screenSizeY : 0,
			.screenPosX = (globalRendering != nullptr) ? globalRendering->screenPosX : 0,
			.screenPosY = (globalRendering != nullptr) ? globalRendering->screenPosY : 0,
			.windowSizeX = (globalRendering != nullptr) ? globalRendering->winSizeX : 0,
			.windowSizeY = (globalRendering != nullptr) ? globalRendering->winSizeY : 0,
			.windowPosX = (globalRendering != nullptr) ? globalRendering->winPosX : 0,
			.windowPosY = winPosY_bl,
			.windowBorderTop = (globalRendering != nullptr) ? globalRendering->winBorder[0] : 0,
			.windowBorderLeft = (globalRendering != nullptr) ? globalRendering->winBorder[1] : 0,
			.windowBorderBottom = (globalRendering != nullptr) ? globalRendering->winBorder[2] : 0,
			.windowBorderRight = (globalRendering != nullptr) ? globalRendering->winBorder[3] : 0,
			.viewSizeX = (globalRendering != nullptr) ? globalRendering->viewSizeX : 0,
			.viewSizeY = (globalRendering != nullptr) ? globalRendering->viewSizeY : 0,
			.viewPosX = (globalRendering != nullptr) ? globalRendering->viewPosX : 0,
			.viewPosY = (globalRendering != nullptr) ? globalRendering->viewPosY : 0,
		};
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
