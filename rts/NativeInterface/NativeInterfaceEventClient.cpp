/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "NativeInterfaceEventClient.h"

#include <cmath>
#include <cstring>
#include <limits>
#include <utility>

#include "NativeInterface/api/Constants.h"
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
#include "System/BenchmarkCallins.h"
#include "System/Input/KeyInput.h"
#include "System/Platform/SDL1_keysym.h"
#include "System/Platform/SharedLib.h"
#include "System/Rectangle.h"
#include "System/float3.h"
#include "WasmInterface/system/WasmInterfaceSystem.h"
#include "NativeInterface/WasmUiVisibility.h"

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
	// The public native command-option ABI uses a compact bit layout which is
	// intentionally different from Command's internal simulation bitfield.
	// Constants.h exposes these names to C and bindgen, not to the C++ engine
	// build, so keep the C++ conversion beside the callin boundary explicit.
	constexpr uint8_t NATIVE_CMD_OPT_INTERNAL = (1 << 0);
	constexpr uint8_t NATIVE_CMD_OPT_RIGHT = (1 << 1);
	constexpr uint8_t NATIVE_CMD_OPT_SHIFT = (1 << 2);
	constexpr uint8_t NATIVE_CMD_OPT_CTRL = (1 << 3);
	constexpr uint8_t NATIVE_CMD_OPT_ALT = (1 << 4);
	constexpr uint8_t NATIVE_CMD_OPT_META = (1 << 5);

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

NativeInterfaceEventClient::NativeInterfaceEventClient(NativeInterface* nativeInterface,
	SharedLib* sharedLib, WasmInterfaceSystem* wasmSystem)
	: CEventClient("[NativeInterfaceEventClient]", 23253, false)
	, m_nativeInterface(nativeInterface)
	, m_sharedLib(sharedLib)
	, m_wasmSystem(wasmSystem)
{
}

bool NativeInterfaceEventClient::DispatchWasmCallin(std::string_view name,
	const void* query, bool synced, void* nativeResult)
{
	// Core is the sole Wasm transport. Supported Core callins run before the
	// native module callback so the two implementations can contribute results.
	if (m_wasmSystem != nullptr) {
		bool coreHandled = false;
		std::string coreError;
		if (!WasmInterfaceSystem::DispatchActiveCoreCallin(
				name, query, synced, nativeResult, coreHandled, coreError)) {
			if (!coreError.empty()) {
				LOG_L(L_ERROR, "Core Wasm callin %s failed: %s",
					std::string(name).c_str(), coreError.c_str());
			}
			return false;
		}
		if (coreHandled) {
			return true;
		}
	}

	return false;
}

bool NativeInterfaceEventClient::DispatchWasmBoolCallin(std::string_view name,
	const void* query, bool synced, bool& result)
{
	BoolCallinResult directResult = {.error = nullptr, .value = false};
	if (!DispatchWasmCallin(name, query, synced, &directResult))
		return false;
	if (directResult.error != nullptr) {
		LOG_L(L_WARNING, "Wasm callin %s returned a direct boolean error: %s",
			std::string(name).c_str(), directResult.error->message);
		return false;
	}
	result = directResult.value;
	return true;
}

bool NativeInterfaceEventClient::DispatchWasmStringCallin(std::string_view name,
	const void* query, bool synced, std::string& result)
{
	StringCallinResult directResult = {.error = nullptr, .value = nullptr};
	if (!DispatchWasmCallin(name, query, synced, &directResult))
		return false;
	if (directResult.error != nullptr) {
		LOG_L(L_WARNING, "Wasm callin %s returned a direct string error: %s",
			std::string(name).c_str(), directResult.error->message);
		return false;
	}
	result = directResult.value == nullptr ? std::string{} : std::string(directResult.value);
	return true;
}

bool NativeInterfaceEventClient::DispatchWasmIntegerCallin(std::string_view name,
	const void* query, bool synced, int& result)
{
	IntCallinResult directResult = {.error = nullptr, .value = 0};
	if (!DispatchWasmCallin(name, query, synced, &directResult))
		return false;
	if (directResult.error != nullptr) {
		LOG_L(L_WARNING, "Wasm callin %s returned a direct integer error: %s",
			std::string(name).c_str(), directResult.error->message);
		return false;
	}
	result = directResult.value;
	return true;
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
	if (!m_initialized) {
		return;
	}

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
	const uint8_t engineOptions = command.GetOpts();
	uint8_t nativeOptions = 0;

	// Command's simulation bitfield intentionally uses a different layout from
	// the public native-callin ABI.  Do not expose the engine-internal bit
	// positions to native modules: CMD_OPT_* is the stable public layout.
	if (engineOptions & INTERNAL_ORDER)
		nativeOptions |= NATIVE_CMD_OPT_INTERNAL;
	if (engineOptions & RIGHT_MOUSE_KEY)
		nativeOptions |= NATIVE_CMD_OPT_RIGHT;
	if (engineOptions & SHIFT_KEY)
		nativeOptions |= NATIVE_CMD_OPT_SHIFT;
	if (engineOptions & CONTROL_KEY)
		nativeOptions |= NATIVE_CMD_OPT_CTRL;
	if (engineOptions & ALT_KEY)
		nativeOptions |= NATIVE_CMD_OPT_ALT;
	if (engineOptions & META_KEY)
		nativeOptions |= NATIVE_CMD_OPT_META;

	return {
		.id = command.GetID(),
		.timeOut = command.GetTimeOut(),
		.pageIndex = command.GetpageIndex(),
		.numParams = command.GetNumParams(),
		.tag = command.GetTag(),
		.options = nativeOptions,
		.params = command.GetParams()
	};
}

void NativeInterfaceEventClient::Load(IArchive* archive) {
	ArchiveCallinQuery query = {.archive = archive};
	DispatchWasmCallin("Load", &query, true);
	if (m_LoadFuncPtr) {
		ArchiveCallinResult result = {};
		m_LoadFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::GamePreload() {
	GamePreloadQuery query = {};
	DispatchWasmCallin("GamePreload", &query, true);
	if (m_GamePreloadFuncPtr) {
		GamePreloadResult result = {};
		m_GamePreloadFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::GameStart() {
	GameStartQuery query = {};
	DispatchWasmCallin("GameStart", &query, true);
	if (m_GameStartFuncPtr) {
		GameStartResult result = {};
		m_GameStartFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::GameOver(const std::vector<unsigned char>& winningAllyTeams) {
	GameOverEventQuery query = {
		.winningAllyTeams = winningAllyTeams.data(),
		.count = static_cast<uint32_t>(winningAllyTeams.size())
	};
	DispatchWasmCallin("GameOver", &query, true);
	if (m_GameOverFuncPtr) {
		GameOverEventResult result = {};
		m_GameOverFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::GameFrame(int gameFrame) {
	GameFrameQuery query = {.gameFrame = gameFrame};
	const auto wasmToken = spring::benchmark_callins::Begin(
		"wasm", spring::benchmark_callins::GameFrameTestName());
	DispatchWasmCallin("GameFrame", &query, true);
	spring::benchmark_callins::End(wasmToken);
	const bool benchmarkUnimplemented = spring::benchmark_callins::IsCase("callins") &&
		spring::benchmark_callins::IsVariant("unimplemented");
	const auto nativeToken = benchmarkUnimplemented
		? spring::benchmark_callins::Begin("native", "callin_unimplemented")
		: spring::benchmark_callins::Token{};
	if (!benchmarkUnimplemented && m_GameFrameFuncPtr) {
		const auto nativeToken = spring::benchmark_callins::Begin(
			"native", spring::benchmark_callins::GameFrameTestName());
		GameFrameResult result = {};
		m_GameFrameFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		spring::benchmark_callins::End(nativeToken);
	}
	spring::benchmark_callins::End(nativeToken);
}

void NativeInterfaceEventClient::GameFramePost(int gameFrame) {
	GameFramePostQuery query = {.gameFrame = gameFrame};
	DispatchWasmCallin("GameFramePost", &query, true);
	if (m_GameFramePostFuncPtr) {
		GameFramePostResult result = {};
		m_GameFramePostFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::Update() {
	UpdateQuery query = {
		.deltaSeconds = (game != nullptr) ? game->updateDeltaSeconds : 0.0f,
	};
	const auto wasmToken = spring::benchmark_callins::Begin("wasm", "callin_update");
	DispatchWasmCallin("Update", &query, false);
	spring::benchmark_callins::End(wasmToken);
	if (m_UpdateFuncPtr) {
		const auto nativeToken = spring::benchmark_callins::Begin("native", "callin_update");
		UpdateResult result = {};
		m_UpdateFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		spring::benchmark_callins::End(nativeToken);
	}
}

void NativeInterfaceEventClient::DrawScreen() {
	DrawScreenQuery query = {
		.viewSizeX = (globalRendering != nullptr) ? globalRendering->viewSizeX : 0,
		.viewSizeY = (globalRendering != nullptr) ? globalRendering->viewSizeY : 0,
	};
	DispatchWasmCallin("DrawScreen", &query, false);
	if (m_DrawScreenFuncPtr) {
		DrawScreenResult result = {};
		m_DrawScreenFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

#define DISPATCH_SIMPLE_CALLIN(EventName)                                      \
	void NativeInterfaceEventClient::EventName() {                              \
		SimpleCallinQuery query = {};                                           \
		const auto wasmToken = spring::benchmark_callins::Begin(                 \
			"wasm", spring::benchmark_callins::EventTestName(#EventName));       \
		const auto dispatchStage =                                            \
			strcmp(#EventName, "DrawWorld") == 0                                \
			? spring::benchmark_callins::BeginStage(                              \
				"wasm", "callin_drawworld_native_dispatch")                      \
			: spring::benchmark_callins::Token{};                                 \
		DispatchWasmCallin(#EventName, &query, false);                           \
		spring::benchmark_callins::End(dispatchStage);                          \
		spring::benchmark_callins::End(wasmToken);                               \
		if (m_##EventName##FuncPtr) {                                             \
			const auto nativeToken = spring::benchmark_callins::Begin(              \
				"native", spring::benchmark_callins::EventTestName(#EventName));      \
			SimpleCallinResult result = {};                                         \
			m_##EventName##FuncPtr(m_nativeInterface, m_moduleData, &query, &result); \
			spring::benchmark_callins::End(nativeToken);                            \
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
		DrawScreenQuery query = {                                                  \
			.viewSizeX = (globalRendering != nullptr) ? globalRendering->viewSizeX : 0, \
			.viewSizeY = (globalRendering != nullptr) ? globalRendering->viewSizeY : 0, \
		};                                                                         \
		DispatchWasmCallin(#EventName, &query, false);                              \
		if (m_##EventName##FuncPtr) {                                               \
			DrawScreenResult result = {};                                              \
			m_##EventName##FuncPtr(m_nativeInterface, m_moduleData, &query, &result);  \
		}                                                                            \
	}

DISPATCH_SCREEN_CALLIN(DrawScreenEffects)
DISPATCH_SCREEN_CALLIN(DrawScreenPost)

#undef DISPATCH_SCREEN_CALLIN

#define DISPATCH_MINIMAP_DRAW_CALLIN(EventName)                                \
	void NativeInterfaceEventClient::EventName() {                               \
		MiniMapDrawQuery query = {                                                 \
			.sizeX = (minimap != nullptr) ? minimap->GetSizeX() : 0,                  \
			.sizeY = (minimap != nullptr) ? minimap->GetSizeY() : 0,                  \
		};                                                                         \
		DispatchWasmCallin(#EventName, &query, false);                              \
		if (m_##EventName##FuncPtr) {                                               \
			SimpleCallinResult result = {};                                            \
			m_##EventName##FuncPtr(m_nativeInterface, m_moduleData, &query, &result);  \
		}                                                                            \
	}

DISPATCH_MINIMAP_DRAW_CALLIN(DrawInMiniMap)
DISPATCH_MINIMAP_DRAW_CALLIN(DrawInMiniMapBackground)

#undef DISPATCH_MINIMAP_DRAW_CALLIN

bool NativeInterfaceEventClient::DrawUnit(const CUnit* unit) {
	DrawUnitQuery query = {
		.unitID = (unit != nullptr) ? unit->id : -1,
		.drawMode = (game != nullptr) ? static_cast<int>(game->GetDrawMode()) : 0,
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("DrawUnit", &query, false, wasmValue);
	if (m_DrawUnitFuncPtr == nullptr)
		return hasWasmValue && wasmValue;

	BoolCallinResult result = {.value = false};
	m_DrawUnitFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value || (hasWasmValue && wasmValue);
}

bool NativeInterfaceEventClient::DrawFeature(const CFeature* feature) {
	DrawFeatureQuery query = {
		.featureID = (feature != nullptr) ? feature->id : -1,
		.drawMode = (game != nullptr) ? static_cast<int>(game->GetDrawMode()) : 0,
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("DrawFeature", &query, false, wasmValue);
	if (m_DrawFeatureFuncPtr == nullptr)
		return hasWasmValue && wasmValue;

	BoolCallinResult result = {.value = false};
	m_DrawFeatureFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value || (hasWasmValue && wasmValue);
}

bool NativeInterfaceEventClient::DrawShield(const CUnit* unit, const CWeapon* weapon) {
	DrawShieldQuery query = {
		.unitID = (unit != nullptr) ? unit->id : -1,
		.weaponID = (weapon != nullptr) ? weapon->weaponNum + LUA_WEAPON_BASE_INDEX : -1,
		.drawMode = (game != nullptr) ? static_cast<int>(game->GetDrawMode()) : 0,
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("DrawShield", &query, false, wasmValue);
	if (m_DrawShieldFuncPtr == nullptr)
		return hasWasmValue && wasmValue;

	BoolCallinResult result = {.value = false};
	m_DrawShieldFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value || (hasWasmValue && wasmValue);
}

bool NativeInterfaceEventClient::DrawProjectile(const CProjectile* projectile) {
	DrawProjectileQuery query = {
		.projectileID = (projectile != nullptr) ? projectile->id : -1,
		.drawMode = (game != nullptr) ? static_cast<int>(game->GetDrawMode()) : 0,
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("DrawProjectile", &query, false, wasmValue);
	if (m_DrawProjectileFuncPtr == nullptr)
		return hasWasmValue && wasmValue;

	BoolCallinResult result = {.value = false};
	m_DrawProjectileFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value || (hasWasmValue && wasmValue);
}

bool NativeInterfaceEventClient::DrawMaterial(const LuaMaterial* material) {
	DrawMaterialQuery query = {
		.uuid = (material != nullptr) ? material->uuid : -1,
		.drawMode = (game != nullptr) ? static_cast<int>(game->GetDrawMode()) : 0,
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("DrawMaterial", &query, false, wasmValue);
	if (m_DrawMaterialFuncPtr == nullptr)
		return hasWasmValue && wasmValue;

	BoolCallinResult result = {.value = false};
	m_DrawMaterialFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value || (hasWasmValue && wasmValue);
}

void NativeInterfaceEventClient::DrawWorldPreParticles(bool drawAboveWater, bool drawBelowWater, bool drawReflection, bool drawRefraction) {
	DrawWorldPreParticlesQuery query = {
		.drawAboveWater = drawAboveWater,
		.drawBelowWater = drawBelowWater,
		.drawReflection = drawReflection,
		.drawRefraction = drawRefraction
	};
	DispatchWasmCallin("DrawWorldPreParticles", &query, false);
	if (m_DrawWorldPreParticlesFuncPtr) {
		DrawWorldPreParticlesResult result = {};
		m_DrawWorldPreParticlesFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::DrawBuildSquare(int unitDefID, int x, int z, int facing, const std::vector<uint8_t>& statuses)
{
	DrawBuildSquareQuery query = {
		.unitDefID = unitDefID,
		.x = x,
		.z = z,
		.facing = facing,
		.statuses = statuses.data(),
		.statusCount = static_cast<uint32_t>(statuses.size()),
	};
	DispatchWasmCallin("DrawBuildSquare", &query, false);
	if (m_DrawBuildSquareFuncPtr) {
		DrawBuildSquareResult result = {};
		m_DrawBuildSquareFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

#define DISPATCH_DRAW_OBJECTS_LUA(EventName)                                  \
	void NativeInterfaceEventClient::EventName(bool deferredPass, bool drawReflection, bool drawRefraction) { \
		DrawObjectsLuaQuery query = {                                           \
			.deferredPass = deferredPass,                                        \
			.drawReflection = drawReflection,                                    \
			.drawRefraction = drawRefraction                                     \
		};                                                                       \
		DispatchWasmCallin(#EventName, &query, false);                           \
		if (m_##EventName##FuncPtr) {                                             \
			DrawObjectsLuaResult result = {};                                        \
			m_##EventName##FuncPtr(m_nativeInterface, m_moduleData, &query, &result); \
		}                                                                          \
	}

DISPATCH_DRAW_OBJECTS_LUA(DrawOpaqueUnitsLua)
DISPATCH_DRAW_OBJECTS_LUA(DrawOpaqueFeaturesLua)

#undef DISPATCH_DRAW_OBJECTS_LUA

#define DISPATCH_DRAW_ALPHA_OBJECTS_LUA(EventName)                            \
	void NativeInterfaceEventClient::EventName(bool drawReflection, bool drawRefraction) { \
		DrawAlphaObjectsLuaQuery query = {                                      \
			.drawReflection = drawReflection,                                    \
			.drawRefraction = drawRefraction                                     \
		};                                                                       \
		DispatchWasmCallin(#EventName, &query, false);                           \
		if (m_##EventName##FuncPtr) {                                             \
			DrawAlphaObjectsLuaResult result = {};                                   \
			m_##EventName##FuncPtr(m_nativeInterface, m_moduleData, &query, &result); \
		}                                                                          \
	}

DISPATCH_DRAW_ALPHA_OBJECTS_LUA(DrawAlphaUnitsLua)
DISPATCH_DRAW_ALPHA_OBJECTS_LUA(DrawAlphaFeaturesLua)

#undef DISPATCH_DRAW_ALPHA_OBJECTS_LUA

void NativeInterfaceEventClient::GamePaused(int playerID, bool paused) {
	GamePausedQuery query = {
		.playerID = playerID,
		.paused = paused
	};
	DispatchWasmCallin("GamePaused", &query, true);
	if (m_GamePausedFuncPtr) {
		GamePausedResult result = {};
		m_GamePausedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::GameID(const unsigned char* gameID, unsigned int numBytes) {
	GameIDQuery query = {
		.gameID = gameID,
		.numBytes = numBytes
	};
	DispatchWasmCallin("GameID", &query, true);
	if (m_GameIDFuncPtr) {
		GameIDResult result = {};
		m_GameIDFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::TeamDied(int teamID) {
	TeamDiedQuery query = {.teamID = teamID};
	DispatchWasmCallin("TeamDied", &query, true);
	if (m_TeamDiedFuncPtr) {
		TeamDiedResult result = {};
		m_TeamDiedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::TeamChanged(int teamID) {
	TeamChangedQuery query = {.teamID = teamID};
	DispatchWasmCallin("TeamChanged", &query, true);
	if (m_TeamChangedFuncPtr) {
		TeamChangedResult result = {};
		m_TeamChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::PlayerChanged(int playerID) {
	PlayerChangedQuery query = {.playerID = playerID};
	DispatchWasmCallin("PlayerChanged", &query, true);
	if (m_PlayerChangedFuncPtr) {
		PlayerChangedResult result = {};
		m_PlayerChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::PlayerAdded(int playerID) {
	PlayerAddedQuery query = {.playerID = playerID};
	DispatchWasmCallin("PlayerAdded", &query, true);
	if (m_PlayerAddedFuncPtr) {
		PlayerAddedResult result = {};
		m_PlayerAddedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::PlayerRemoved(int playerID, int reason) {
	PlayerRemovedQuery query = {
		.playerID = playerID,
		.reason = reason
	};
	DispatchWasmCallin("PlayerRemoved", &query, true);
	if (m_PlayerRemovedFuncPtr) {
		PlayerRemovedResult result = {};
		m_PlayerRemovedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitCreated(const CUnit* unit, const CUnit* builder) {
	UnitCreatedQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
		.builderID = builder != nullptr ? builder->id : -1
	};
	const auto wasmToken = spring::benchmark_callins::Begin("wasm", "callin_unitcreated");
	DispatchWasmCallin("UnitCreated", &query, true);
	spring::benchmark_callins::End(wasmToken);
	if (m_UnitCreatedFuncPtr) {
		const auto nativeToken = spring::benchmark_callins::Begin("native", "callin_unitcreated");
		UnitCreatedResult result = {};
		m_UnitCreatedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		spring::benchmark_callins::End(nativeToken);
	}
}

void NativeInterfaceEventClient::UnitFinished(const CUnit* unit) {
	UnitFinishedQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
	};
	DispatchWasmCallin("UnitFinished", &query, true);
	if (m_UnitFinishedFuncPtr) {
		UnitFinishedResult result = {};
		m_UnitFinishedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitReverseBuilt(const CUnit* unit) {
	UnitReverseBuiltQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
	};
	DispatchWasmCallin("UnitReverseBuilt", &query, true);
	if (m_UnitReverseBuiltFuncPtr) {
		UnitReverseBuiltResult result = {};
		m_UnitReverseBuiltFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitConstructionDecayed(const CUnit* unit, float timeSinceLastBuild, float iterationPeriod, float part) {
	UnitConstructionDecayedQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
		.timeSinceLastBuild = timeSinceLastBuild,
		.iterationPeriod = iterationPeriod,
		.part = part
	};
	DispatchWasmCallin("UnitConstructionDecayed", &query, true);
	if (m_UnitConstructionDecayedFuncPtr) {
		UnitConstructionDecayedResult result = {};
		m_UnitConstructionDecayedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitFromFactory(const CUnit* unit, const CUnit* factory, bool userOrders) {
	UnitFromFactoryQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
		.factoryID = factory->id,
		.factoryDefID = (factory->unitDef != nullptr) ? factory->unitDef->id : -1,
		.userOrders = userOrders
	};
	DispatchWasmCallin("UnitFromFactory", &query, true);
	if (m_UnitFromFactoryFuncPtr) {
		UnitFromFactoryResult result = {};
		m_UnitFromFactoryFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitDestroyed(const CUnit* unit, const CUnit* attacker, int weaponDefID) {
	UnitDestroyedQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
		.attackerID = attacker != nullptr ? attacker->id : -1,
		.attackerDefID = (attacker != nullptr && attacker->unitDef != nullptr) ? attacker->unitDef->id : -1,
		.attackerTeam = attacker != nullptr ? attacker->team : -1,
		.weaponDefID = weaponDefID,
	};
	DispatchWasmCallin("UnitDestroyed", &query, true);
	if (m_UnitDestroyedFuncPtr) {
		UnitDestroyedResult result = {};
		m_UnitDestroyedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitTaken(const CUnit* unit, int oldTeam, int newTeam) {
	UnitTakenQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.oldTeam = oldTeam,
		.newTeam = newTeam
	};
	DispatchWasmCallin("UnitTaken", &query, true);
	if (m_UnitTakenFuncPtr) {
		UnitTakenResult result = {};
		m_UnitTakenFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitGiven(const CUnit* unit, int oldTeam, int newTeam) {
	UnitGivenQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.oldTeam = oldTeam,
		.newTeam = newTeam
	};
	DispatchWasmCallin("UnitGiven", &query, true);
	if (m_UnitGivenFuncPtr) {
		UnitGivenResult result = {};
		m_UnitGivenFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitIdle(const CUnit* unit) {
	UnitIdleQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
	};
	DispatchWasmCallin("UnitIdle", &query, true);
	if (m_UnitIdleFuncPtr) {
		UnitIdleResult result = {};
		m_UnitIdleFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitCommand(const CUnit* unit, const Command& command, int playerNum, bool fromSynced, bool fromLua) {
	UnitCommandQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
		.command = ToNativeCallinCommand(command),
		.playerNum = playerNum,
		.fromSynced = fromSynced,
		.fromLua = fromLua
	};
	DispatchWasmCallin("UnitCommand", &query, fromSynced);
	if (m_UnitCommandFuncPtr) {
		UnitCommandResult result = {};
		m_UnitCommandFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

bool NativeInterfaceEventClient::CommandFallback(const CUnit* unit, const Command& command) {
	CommandFallbackQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
		.command = ToNativeCallinCommand(command),
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("CommandFallback", &query, true, wasmValue);
	if (m_CommandFallbackFuncPtr == nullptr)
		return hasWasmValue && wasmValue;

	BoolCallinResult result = {.value = false};
	m_CommandFallbackFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value || (hasWasmValue && wasmValue);
}

bool NativeInterfaceEventClient::AllowCommand(const CUnit* unit, const Command& command, int playerNum, bool fromSynced, bool fromLua) {
	UnitCommandQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
		.command = ToNativeCallinCommand(command),
		.playerNum = playerNum,
		.fromSynced = fromSynced,
		.fromLua = fromLua,
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("AllowCommand", &query, fromSynced, wasmValue);
	if (m_AllowCommandFuncPtr == nullptr)
		return hasWasmValue ? wasmValue : true;

	BoolCallinResult result = {.value = true};
	m_AllowCommandFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value && (!hasWasmValue || wasmValue);
}

std::pair<bool, bool> NativeInterfaceEventClient::AllowUnitCreation(const UnitDef* unitDef, const CUnit* builder, const BuildInfo* buildInfo) {
	AllowUnitCreationQuery query = {
		.unitDefID = (unitDef != nullptr) ? unitDef->id : -1,
		.builderID = (builder != nullptr) ? builder->id : -1,
		.builderTeam = (builder != nullptr) ? builder->team : -1,
		.hasBuildInfo = (buildInfo != nullptr),
		.buildPos = (buildInfo != nullptr) ? Float3{buildInfo->pos.x, buildInfo->pos.y, buildInfo->pos.z} : Float3{},
		.buildFacing = (buildInfo != nullptr) ? buildInfo->buildFacing : 0,
	};
	const auto wasmToken = spring::benchmark_callins::Begin("wasm", "callin_allowunitcreation");
	AllowUnitCreationResult directResult = {.allow = true, .dropOrder = true};
	const bool hasWasmResult = DispatchWasmCallin("AllowUnitCreation", &query, true,
		&directResult);
	spring::benchmark_callins::End(wasmToken);
	const bool hasWasmFields = hasWasmResult && directResult.error == nullptr;
	if (m_AllowUnitCreationFuncPtr == nullptr)
		return hasWasmFields ? std::pair{directResult.allow, directResult.dropOrder} :
			std::pair{true, true};

	const auto nativeToken = spring::benchmark_callins::Begin("native", "callin_allowunitcreation");
	AllowUnitCreationResult result = {.allow = true, .dropOrder = true};
	m_AllowUnitCreationFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	spring::benchmark_callins::End(nativeToken);
	return {result.allow, result.dropOrder};
}

bool NativeInterfaceEventClient::AllowUnitTransfer(const CUnit* unit, int newTeam, bool capture) {
	AllowUnitTransferQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.oldTeam = unit->team,
		.newTeam = newTeam,
		.capture = capture,
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("AllowUnitTransfer", &query, true, wasmValue);
	if (m_AllowUnitTransferFuncPtr == nullptr)
		return hasWasmValue ? wasmValue : true;

	BoolCallinResult result = {.value = true};
	m_AllowUnitTransferFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value && (!hasWasmValue || wasmValue);
}

bool NativeInterfaceEventClient::AllowUnitBuildStep(const CUnit* builder, const CUnit* unit, float part) {
	AllowUnitBuildStepQuery query = {
		.builderID = builder->id,
		.builderTeam = builder->team,
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.part = part,
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("AllowUnitBuildStep", &query, true, wasmValue);
	if (m_AllowUnitBuildStepFuncPtr == nullptr)
		return hasWasmValue ? wasmValue : true;

	BoolCallinResult result = {.value = true};
	m_AllowUnitBuildStepFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value && (!hasWasmValue || wasmValue);
}

bool NativeInterfaceEventClient::AllowUnitCaptureStep(const CUnit* builder, const CUnit* unit, float part) {
	AllowUnitBuildStepQuery query = {
		.builderID = builder->id,
		.builderTeam = builder->team,
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.part = part,
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("AllowUnitCaptureStep", &query, true, wasmValue);
	if (m_AllowUnitCaptureStepFuncPtr == nullptr)
		return hasWasmValue ? wasmValue : true;

	BoolCallinResult result = {.value = true};
	m_AllowUnitCaptureStepFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value && (!hasWasmValue || wasmValue);
}

bool NativeInterfaceEventClient::AllowUnitTransport(const CUnit* transporter, const CUnit* transportee) {
	AllowUnitTransportQuery query = {
		.transporterID = transporter->id,
		.transporterDefID = (transporter->unitDef != nullptr) ? transporter->unitDef->id : -1,
		.transporterTeam = transporter->team,
		.transporteeID = transportee->id,
		.transporteeDefID = (transportee->unitDef != nullptr) ? transportee->unitDef->id : -1,
		.transporteeTeam = transportee->team,
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("AllowUnitTransport", &query, true, wasmValue);
	if (m_AllowUnitTransportFuncPtr == nullptr)
		return hasWasmValue ? wasmValue : true;

	BoolCallinResult result = {.value = true};
	m_AllowUnitTransportFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value && (!hasWasmValue || wasmValue);
}

bool NativeInterfaceEventClient::AllowUnitTransportLoad(const CUnit* transporter, const CUnit* transportee, const float3& loadPos, bool allowed) {
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
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("AllowUnitTransportLoad", &query, true, wasmValue);
	if (m_AllowUnitTransportLoadFuncPtr == nullptr)
		return hasWasmValue ? wasmValue : allowed;

	BoolCallinResult result = {.value = allowed};
	m_AllowUnitTransportLoadFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value && (!hasWasmValue || wasmValue);
}

bool NativeInterfaceEventClient::AllowUnitTransportUnload(const CUnit* transporter, const CUnit* transportee, const float3& unloadPos, bool allowed) {
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
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("AllowUnitTransportUnload", &query, true, wasmValue);
	if (m_AllowUnitTransportUnloadFuncPtr == nullptr)
		return hasWasmValue ? wasmValue : allowed;

	BoolCallinResult result = {.value = allowed};
	m_AllowUnitTransportUnloadFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value && (!hasWasmValue || wasmValue);
}

bool NativeInterfaceEventClient::AllowUnitCloak(const CUnit* unit, const CUnit* enemy) {
	AllowUnitCloakQuery query = {
		.unitID = unit->id,
		.hasEnemy = (enemy != nullptr),
		.enemyID = (enemy != nullptr) ? enemy->id : -1,
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("AllowUnitCloak", &query, true, wasmValue);
	if (m_AllowUnitCloakFuncPtr == nullptr)
		return hasWasmValue ? wasmValue : true;

	BoolCallinResult result = {.value = true};
	m_AllowUnitCloakFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value && (!hasWasmValue || wasmValue);
}

bool NativeInterfaceEventClient::AllowUnitDecloak(const CUnit* unit, const CSolidObject* object, const CWeapon* weapon) {
	AllowUnitDecloakQuery query = {
		.unitID = unit->id,
		.hasObject = (object != nullptr),
		.objectID = (object != nullptr) ? object->id : -1,
		.hasWeapon = (weapon != nullptr),
		.weaponNum = (weapon != nullptr) ? weapon->weaponNum : -1,
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("AllowUnitDecloak", &query, true, wasmValue);
	if (m_AllowUnitDecloakFuncPtr == nullptr)
		return hasWasmValue ? wasmValue : true;

	BoolCallinResult result = {.value = true};
	m_AllowUnitDecloakFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value && (!hasWasmValue || wasmValue);
}

bool NativeInterfaceEventClient::AllowUnitKamikaze(const CUnit* unit, const CUnit* target, bool allowed) {
	AllowUnitKamikazeQuery query = {
		.unitID = unit->id,
		.targetID = (target != nullptr) ? target->id : -1,
		.allowed = allowed,
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("AllowUnitKamikaze", &query, true, wasmValue);
	if (m_AllowUnitKamikazeFuncPtr == nullptr)
		return hasWasmValue ? wasmValue : allowed;

	BoolCallinResult result = {.value = allowed};
	m_AllowUnitKamikazeFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value && (!hasWasmValue || wasmValue);
}

void NativeInterfaceEventClient::UnitCmdDone(const CUnit* unit, const Command& command) {
	UnitCmdDoneQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
		.command = ToNativeCallinCommand(command)
	};
	DispatchWasmCallin("UnitCmdDone", &query, true);
	if (m_UnitCmdDoneFuncPtr) {
		UnitCmdDoneResult result = {};
		m_UnitCmdDoneFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitDamaged(const CUnit* unit, const CUnit* attacker, float damage, int weaponDefID, int projectileID, bool paralyzer) {
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
	DispatchWasmCallin("UnitDamaged", &query, true);
	if (m_UnitDamagedFuncPtr) {
		UnitDamagedResult result = {};
		m_UnitDamagedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitHarvestStorageFull(const CUnit* unit) {
	UnitHarvestStorageFullQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
	};
	DispatchWasmCallin("UnitHarvestStorageFull", &query, true);
	if (m_UnitHarvestStorageFullFuncPtr) {
		UnitHarvestStorageFullResult result = {};
		m_UnitHarvestStorageFullFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitSeismicPing(const CUnit* unit, int allyTeam, const float3& pos, float strength) {
	UnitSeismicPingQuery query = {
		.pos = {.x = pos.x, .y = pos.y, .z = pos.z},
		.strength = strength,
		.allyTeam = allyTeam,
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1
	};
	DispatchWasmCallin("UnitSeismicPing", &query, true);
	if (m_UnitSeismicPingFuncPtr) {
		UnitSeismicPingResult result = {};
		m_UnitSeismicPingFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

#define DISPATCH_UNIT_LOS_EVENT(EventName)                                      \
	void NativeInterfaceEventClient::EventName(const CUnit* unit, int allyTeam) { \
		UnitLosEventQuery query = {                                               \
			.unitID = unit->id,                                                       \
			.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,       \
			.unitTeam = unit->team,                                                  \
			.allyTeam = allyTeam,                                                     \
		};                                                                          \
		DispatchWasmCallin(#EventName, &query, true);                              \
		if (m_##EventName##FuncPtr) {                                             \
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
		UnitMovementClassEventQuery query = {                                    \
			.unitID = unit->id,                                                       \
			.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,       \
			.unitTeam = unit->team,                                                  \
		};                                                                          \
		DispatchWasmCallin(#EventName, &query, true);                              \
		if (m_##EventName##FuncPtr) {                                             \
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
	UnitStunnedQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
		.stunned = stunned
	};
	DispatchWasmCallin("UnitStunned", &query, true);
	if (m_UnitStunnedFuncPtr) {
		UnitStunnedResult result = {};
		m_UnitStunnedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitExperience(const CUnit* unit, float oldExperience) {
	UnitExperienceQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
		.experience = unit->experience,
		.oldExperience = oldExperience
	};
	DispatchWasmCallin("UnitExperience", &query, true);
	if (m_UnitExperienceFuncPtr) {
		UnitExperienceResult result = {};
		m_UnitExperienceFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitLoaded(const CUnit* unit, const CUnit* transport) {
	UnitLoadedQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
		.transportID = transport->id,
		.transportTeam = transport->team,
	};
	DispatchWasmCallin("UnitLoaded", &query, true);
	if (m_UnitLoadedFuncPtr) {
		UnitLoadedResult result = {};
		m_UnitLoadedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitUnloaded(const CUnit* unit, const CUnit* transport) {
	UnitUnloadedQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
		.transportID = transport->id,
		.transportTeam = transport->team,
	};
	DispatchWasmCallin("UnitUnloaded", &query, true);
	if (m_UnitUnloadedFuncPtr) {
		UnitUnloadedResult result = {};
		m_UnitUnloadedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitCloaked(const CUnit* unit) {
	UnitCloakEventQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
	};
	DispatchWasmCallin("UnitCloaked", &query, true);
	if (m_UnitCloakedFuncPtr) {
		UnitCloakEventResult result = {};
		m_UnitCloakedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnitDecloaked(const CUnit* unit) {
	UnitCloakEventQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
	};
	DispatchWasmCallin("UnitDecloaked", &query, true);
	if (m_UnitDecloakedFuncPtr) {
		UnitCloakEventResult result = {};
		m_UnitDecloakedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

#define DISPATCH_UNIT_MOVE_EVENT(EventName)                                    \
	void NativeInterfaceEventClient::EventName(const CUnit* unit) {              \
		UnitMoveEventQuery query = {                                            \
			.unitID = unit->id,                                                       \
			.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,       \
			.unitTeam = unit->team,                                                  \
		};                                                                          \
		DispatchWasmCallin(#EventName, &query, true);                              \
		if (m_##EventName##FuncPtr) {                                             \
			UnitMoveEventResult result = {};                                      \
			m_##EventName##FuncPtr(m_nativeInterface, m_moduleData, &query, &result); \
		}                                                                          \
	}

DISPATCH_UNIT_MOVE_EVENT(UnitMoved)
DISPATCH_UNIT_MOVE_EVENT(UnitMoveFailed)
DISPATCH_UNIT_MOVE_EVENT(UnitArrivedAtGoal)

#undef DISPATCH_UNIT_MOVE_EVENT

bool NativeInterfaceEventClient::UnitUnitCollision(const CUnit* collider, const CUnit* collidee) {
	UnitUnitCollisionQuery query = {
		.colliderID = (collider != nullptr) ? collider->id : -1,
		.collideeID = (collidee != nullptr) ? collidee->id : -1
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("UnitUnitCollision", &query, true, wasmValue);
	if (m_UnitUnitCollisionFuncPtr) {
		BoolCallinResult result = {.value = false};
		m_UnitUnitCollisionFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value || (hasWasmValue && wasmValue);
	}
	return hasWasmValue && wasmValue;
}

bool NativeInterfaceEventClient::UnitFeatureCollision(const CUnit* collider, const CFeature* collidee) {
	UnitFeatureCollisionQuery query = {
		.colliderID = (collider != nullptr) ? collider->id : -1,
		.collideeID = (collidee != nullptr) ? collidee->id : -1
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("UnitFeatureCollision", &query, true, wasmValue);
	if (m_UnitFeatureCollisionFuncPtr) {
		BoolCallinResult result = {.value = false};
		m_UnitFeatureCollisionFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value || (hasWasmValue && wasmValue);
	}
	return hasWasmValue && wasmValue;
}

void NativeInterfaceEventClient::RenderUnitDestroyed(const CUnit* unit) {
	RenderUnitDestroyedQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
	};
	DispatchWasmCallin("RenderUnitDestroyed", &query, false);
	if (m_RenderUnitDestroyedFuncPtr) {
		RenderUnitDestroyedResult result = {};
		m_RenderUnitDestroyedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::FeatureCreated(const CFeature* feature) {
	FeatureCreatedQuery query = {
		.featureID = feature->id,
		.allyTeamID = feature->allyteam,
	};
	DispatchWasmCallin("FeatureCreated", &query, true);
	if (m_FeatureCreatedFuncPtr) {
		FeatureCreatedResult result = {};
		m_FeatureCreatedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::FeatureDestroyed(const CFeature* feature) {
	FeatureDestroyedQuery query = {
		.featureID = feature->id,
		.allyTeamID = feature->allyteam,
	};
	DispatchWasmCallin("FeatureDestroyed", &query, true);
	if (m_FeatureDestroyedFuncPtr) {
		FeatureDestroyedResult result = {};
		m_FeatureDestroyedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::FeatureDamaged(const CFeature* feature, const CUnit* attacker, float damage, int weaponDefID, int projectileID) {
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
	DispatchWasmCallin("FeatureDamaged", &query, true);
	if (m_FeatureDamagedFuncPtr) {
		FeatureDamagedResult result = {};
		m_FeatureDamagedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

bool NativeInterfaceEventClient::AllowFeatureCreation(const FeatureDef* featureDef, int allyTeamID, const float3& pos) {
	AllowFeatureCreationQuery query = {
		.featureDefID = (featureDef != nullptr) ? featureDef->id : -1,
		.teamID = allyTeamID,
		.position = {.x = pos.x, .y = pos.y, .z = pos.z},
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("AllowFeatureCreation", &query, true, wasmValue);
	if (m_AllowFeatureCreationFuncPtr == nullptr)
		return hasWasmValue ? wasmValue : true;

	BoolCallinResult result = {.value = true};
	m_AllowFeatureCreationFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value || (hasWasmValue && wasmValue);
}

bool NativeInterfaceEventClient::AllowFeatureBuildStep(const CUnit* builder, const CFeature* feature, float part) {
	AllowFeatureBuildStepQuery query = {
		.builderID = builder->id,
		.builderTeam = builder->team,
		.featureID = feature->id,
		.featureDefID = (feature->def != nullptr) ? feature->def->id : -1,
		.part = part,
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("AllowFeatureBuildStep", &query, true, wasmValue);
	if (m_AllowFeatureBuildStepFuncPtr == nullptr)
		return hasWasmValue ? wasmValue : true;

	BoolCallinResult result = {.value = true};
	m_AllowFeatureBuildStepFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value || (hasWasmValue && wasmValue);
}

bool NativeInterfaceEventClient::AllowResourceLevel(int teamID, const std::string& type, float level) {
	AllowResourceLevelQuery query = {
		.teamID = teamID,
		.type = type.c_str(),
		.level = level,
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("AllowResourceLevel", &query, true, wasmValue);
	if (m_AllowResourceLevelFuncPtr == nullptr)
		return hasWasmValue ? wasmValue : true;

	BoolCallinResult result = {.value = true};
	m_AllowResourceLevelFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value || (hasWasmValue && wasmValue);
}

bool NativeInterfaceEventClient::AllowResourceTransfer(int oldTeam, int newTeam, const char* type, float amount) {
	AllowResourceTransferQuery query = {
		.oldTeam = oldTeam,
		.newTeam = newTeam,
		.type = type,
		.amount = amount,
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("AllowResourceTransfer", &query, true, wasmValue);
	if (m_AllowResourceTransferFuncPtr == nullptr)
		return hasWasmValue ? wasmValue : true;

	BoolCallinResult result = {.value = true};
	m_AllowResourceTransferFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value || (hasWasmValue && wasmValue);
}

bool NativeInterfaceEventClient::ResourceExcess(const std::map<int, SResourcePack>& excess) {
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
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("ResourceExcess", &query, true, wasmValue);
	if (m_ResourceExcessFuncPtr == nullptr)
		return hasWasmValue && wasmValue;

	BoolCallinResult result = {.value = false};
	m_ResourceExcessFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value || (hasWasmValue && wasmValue);
}

bool NativeInterfaceEventClient::AllowDirectUnitControl(int playerID, const CUnit* unit) {
	AllowDirectUnitControlQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
		.playerID = playerID,
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("AllowDirectUnitControl", &query, true, wasmValue);
	if (m_AllowDirectUnitControlFuncPtr == nullptr)
		return hasWasmValue ? wasmValue : true;

	BoolCallinResult result = {.value = true};
	m_AllowDirectUnitControlFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value || (hasWasmValue && wasmValue);
}

bool NativeInterfaceEventClient::AllowBuilderHoldFire(const CUnit* unit, int action) {
	AllowBuilderHoldFireQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.action = action,
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("AllowBuilderHoldFire", &query, true, wasmValue);
	if (m_AllowBuilderHoldFireFuncPtr == nullptr)
		return hasWasmValue ? wasmValue : true;

	BoolCallinResult result = {.value = true};
	m_AllowBuilderHoldFireFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value || (hasWasmValue && wasmValue);
}

bool NativeInterfaceEventClient::AllowStartPosition(int playerID, int teamID, unsigned char readyState, const float3& clampedPos, const float3& rawPickPos) {
	AllowStartPositionQuery query = {
		.playerID = playerID,
		.teamID = teamID,
		.readyState = readyState,
		.clampedPos = {.x = clampedPos.x, .y = clampedPos.y, .z = clampedPos.z},
		.rawPickPos = {.x = rawPickPos.x, .y = rawPickPos.y, .z = rawPickPos.z},
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("AllowStartPosition", &query, true, wasmValue);
	if (m_AllowStartPositionFuncPtr == nullptr)
		return hasWasmValue ? wasmValue : true;

	BoolCallinResult result = {.value = true};
	m_AllowStartPositionFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value && (!hasWasmValue || wasmValue);
}

bool NativeInterfaceEventClient::TerraformComplete(const CUnit* unit, const CUnit* build) {
	TerraformCompleteQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
		.buildUnitID = build->id,
		.buildUnitDefID = (build->unitDef != nullptr) ? build->unitDef->id : -1,
		.buildUnitTeam = build->team,
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("TerraformComplete", &query, true, wasmValue);
	if (m_TerraformCompleteFuncPtr == nullptr)
		return hasWasmValue && wasmValue;

	BoolCallinResult result = {.value = false};
	m_TerraformCompleteFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value || (hasWasmValue && wasmValue);
}

bool NativeInterfaceEventClient::MoveCtrlNotify(const CUnit* unit, int data) {
	MoveCtrlNotifyQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
		.data = data,
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("MoveCtrlNotify", &query, true, wasmValue);
	if (m_MoveCtrlNotifyFuncPtr == nullptr)
		return hasWasmValue && wasmValue;

	BoolCallinResult result = {.value = false};
	m_MoveCtrlNotifyFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value || (hasWasmValue && wasmValue);
}

void NativeInterfaceEventClient::FeatureMoved(const CFeature* feature, const float3& oldpos) {
	FeatureMovedQuery query = {
		.featureID = feature->id,
		.oldPos = {.x = oldpos.x, .y = oldpos.y, .z = oldpos.z}
	};
	DispatchWasmCallin("FeatureMoved", &query, true);
	if (m_FeatureMovedFuncPtr) {
		FeatureMovedResult result = {};
		m_FeatureMovedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::ProjectileCreated(const CProjectile* proj) {
	const auto* weaponProjectile = proj->weapon ? static_cast<const CWeaponProjectile*>(proj) : nullptr;
	const auto* weaponDef = (weaponProjectile != nullptr) ? weaponProjectile->GetWeaponDef() : nullptr;
	ProjectileEventQuery query = {
		.projectileID = proj->id,
		.ownerID = static_cast<int32_t>(proj->GetOwnerID()),
		.weaponDefID = (weaponDef != nullptr) ? weaponDef->id : -1,
	};
	DispatchWasmCallin("ProjectileCreated", &query, true);
	if (m_ProjectileCreatedFuncPtr) {
		ProjectileEventResult result = {};
		m_ProjectileCreatedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::ProjectileDestroyed(const CProjectile* proj) {
	const auto* weaponProjectile = proj->weapon ? static_cast<const CWeaponProjectile*>(proj) : nullptr;
	const auto* weaponDef = (weaponProjectile != nullptr) ? weaponProjectile->GetWeaponDef() : nullptr;
	ProjectileEventQuery query = {
		.projectileID = proj->id,
		.ownerID = static_cast<int32_t>(proj->GetOwnerID()),
		.weaponDefID = (weaponDef != nullptr) ? weaponDef->id : -1,
	};
	DispatchWasmCallin("ProjectileDestroyed", &query, true);
	if (m_ProjectileDestroyedFuncPtr) {
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
	ExplosionQuery query = {
		.weaponDefID = weaponID,
		.pos = {.x = params.pos.x, .y = params.pos.y, .z = params.pos.z},
		.ownerID = (params.owner != nullptr) ? params.owner->id : -1,
		.projectileID = static_cast<int32_t>(params.projectileID)
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("Explosion", &query, true, wasmValue);
	if (m_ExplosionFuncPtr) {
		BoolCallinResult result = {.value = false};
		m_ExplosionFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value || (hasWasmValue && wasmValue);
	}
	return hasWasmValue && wasmValue;
}

int NativeInterfaceEventClient::AllowWeaponTargetCheck(unsigned int attackerID, unsigned int attackerWeaponNum, unsigned int attackerWeaponDefID) {
	AllowWeaponTargetCheckQuery query = {
		.attackerID = static_cast<int32_t>(attackerID),
		.attackerWeaponNum = static_cast<int32_t>(attackerWeaponNum + LUA_WEAPON_BASE_INDEX),
		.attackerWeaponDefID = static_cast<int32_t>(attackerWeaponDefID),
	};
	int wasmValue = -1;
	const bool hasWasmValue = DispatchWasmIntegerCallin(
		"AllowWeaponTargetCheck", &query, true, wasmValue);
	if (m_AllowWeaponTargetCheckFuncPtr == nullptr)
		return hasWasmValue ? wasmValue : -1;

	IntCallinResult result = {.value = -1};
	m_AllowWeaponTargetCheckFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return hasWasmValue && wasmValue != -1 ? wasmValue : result.value;
}

bool NativeInterfaceEventClient::AllowWeaponTarget(unsigned int attackerID, unsigned int targetID, unsigned int attackerWeaponNum, unsigned int attackerWeaponDefID, float* targetPriority) {
	const int attackerWeaponNumber = static_cast<int>(attackerWeaponNum);
	AllowWeaponTargetQuery query = {
		.attackerID = static_cast<int32_t>(attackerID),
		.targetID = static_cast<int32_t>(targetID),
		.attackerWeaponNum = attackerWeaponNumber + LUA_WEAPON_BASE_INDEX * (attackerWeaponNumber >= 0),
		.attackerWeaponDefID = static_cast<int32_t>(attackerWeaponDefID),
		.hasTargetPriority = (targetPriority != nullptr),
		.targetPriority = (targetPriority != nullptr) ? *targetPriority : 0.0f,
	};
	AllowWeaponTargetResult directResult = {
		.allowed = true,
		.targetPriority = query.targetPriority,
	};
	const bool hasWasmResult = DispatchWasmCallin("AllowWeaponTarget", &query, true,
		&directResult);
	const bool hasWasmFields = hasWasmResult && directResult.error == nullptr;
	if (m_AllowWeaponTargetFuncPtr == nullptr) {
		if (hasWasmFields && targetPriority != nullptr)
			*targetPriority = directResult.targetPriority;
		return hasWasmFields ? directResult.allowed : true;
	}

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
	AllowWeaponInterceptTargetQuery query = {
		.interceptorUnitID = (interceptorUnit != nullptr) ? interceptorUnit->id : -1,
		.interceptorWeaponID = (interceptorWeapon != nullptr) ? interceptorWeapon->weaponNum + LUA_WEAPON_BASE_INDEX : -1,
		.interceptorTargetID = (interceptorTarget != nullptr) ? interceptorTarget->id : -1,
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("AllowWeaponInterceptTarget", &query, true, wasmValue);
	if (m_AllowWeaponInterceptTargetFuncPtr == nullptr)
		return hasWasmValue ? wasmValue : true;

	BoolCallinResult result = {.value = true};
	m_AllowWeaponInterceptTargetFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value || (hasWasmValue && wasmValue);
}

bool NativeInterfaceEventClient::UnitPreDamaged(const CUnit* unit, const CUnit* attacker, float damage, int weaponDefID, int projectileID, bool paralyzer, float* newDamage, float* impulseMult) {
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
	const auto wasmToken = spring::benchmark_callins::Begin("wasm", "callin_unitpredamaged");
	DamageCallinResult directResult = {
		.newDamage = (newDamage != nullptr) ? *newDamage : damage,
		.impulseMult = (impulseMult != nullptr) ? *impulseMult : 1.0f,
	};
	const bool hasWasmResult = DispatchWasmCallin("UnitPreDamaged", &query, true,
		&directResult);
	spring::benchmark_callins::End(wasmToken);
	const bool hasWasmFields = hasWasmResult && directResult.error == nullptr;
	if (m_UnitPreDamagedFuncPtr == nullptr) {
		if (hasWasmFields) {
			if (newDamage != nullptr)
				*newDamage = directResult.newDamage;
			if (impulseMult != nullptr)
				*impulseMult = directResult.impulseMult;
			return directResult.newDamage == 0.0f && directResult.impulseMult == 0.0f;
		}
		return false;
	}
	const auto nativeToken = spring::benchmark_callins::Begin("native", "callin_unitpredamaged");
	DamageCallinResult result = {
		.newDamage = (newDamage != nullptr) ? *newDamage : damage,
		.impulseMult = (impulseMult != nullptr) ? *impulseMult : 1.0f,
	};
	m_UnitPreDamagedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	spring::benchmark_callins::End(nativeToken);
	if (newDamage != nullptr)
		*newDamage = result.newDamage;
	if (impulseMult != nullptr)
		*impulseMult = result.impulseMult;
	return result.newDamage == 0.0f && result.impulseMult == 0.0f;
}

bool NativeInterfaceEventClient::FeaturePreDamaged(const CFeature* feature, const CUnit* attacker, float damage, int weaponDefID, int projectileID, float* newDamage, float* impulseMult) {
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
	DamageCallinResult directResult = {
		.newDamage = (newDamage != nullptr) ? *newDamage : damage,
		.impulseMult = (impulseMult != nullptr) ? *impulseMult : 1.0f,
	};
	const bool hasWasmResult = DispatchWasmCallin("FeaturePreDamaged", &query, true,
		&directResult);
	const bool hasWasmFields = hasWasmResult && directResult.error == nullptr;
	if (m_FeaturePreDamagedFuncPtr == nullptr) {
		if (hasWasmFields) {
			if (newDamage != nullptr)
				*newDamage = directResult.newDamage;
			if (impulseMult != nullptr)
				*impulseMult = directResult.impulseMult;
			return directResult.newDamage == 0.0f && directResult.impulseMult == 0.0f;
		}
		return false;
	}
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
	ShieldPreDamagedQuery query = {
		.projectileID = (projectile != nullptr) ? projectile->id : -1,
		.projectileOwnerID = (projectile != nullptr) ? static_cast<int32_t>(projectile->GetOwnerID()) : -1,
		.shieldWeaponNum = (shieldEmitter != nullptr) ? shieldEmitter->weaponNum + LUA_WEAPON_BASE_INDEX : -1,
		.shieldCarrierID = (shieldCarrier != nullptr) ? shieldCarrier->id : -1,
		.bounceProjectile = bounceProjectile,
		// Lua exposes these two fields only for beam/ lightning events.  A
		// regular projectile has nil there even if the engine supplied stale or
		// auxiliary beam pointers to the event dispatch.
		.beamEmitterWeaponNum = (projectile == nullptr && beamEmitter != nullptr) ? beamEmitter->weaponNum + LUA_WEAPON_BASE_INDEX : -1,
		.beamEmitterUnitID = (projectile == nullptr && beamCarrier != nullptr) ? beamCarrier->id : -1,
		.startPos = {.x = startPos.x, .y = startPos.y, .z = startPos.z},
		.hitPos = {.x = hitPos.x, .y = hitPos.y, .z = hitPos.z},
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("ShieldPreDamaged", &query, true, wasmValue);
	if (m_ShieldPreDamagedFuncPtr == nullptr)
		return hasWasmValue && wasmValue;

	BoolCallinResult result = {.value = false};
	m_ShieldPreDamagedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	return result.value || (hasWasmValue && wasmValue);
}

void NativeInterfaceEventClient::DownloadFailed(int ID, int errorID) {
	DownloadFailedQuery query = {
		.downloadID = ID,
		.errorID = errorID
	};
	DispatchWasmCallin("DownloadFailed", &query, false);
	if (m_DownloadFailedFuncPtr) {
		DownloadFailedResult result = {};
		m_DownloadFailedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::DownloadFinished(int ID) {
	DownloadFinishedQuery query = {.downloadID = ID};
	DispatchWasmCallin("DownloadFinished", &query, false);
	if (m_DownloadFinishedFuncPtr) {
		DownloadFinishedResult result = {};
		m_DownloadFinishedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::DownloadProgress(int ID, long downloaded, long total) {
	DownloadProgressQuery query = {
		.downloadID = ID,
		.downloaded = downloaded,
		.total = total
	};
	DispatchWasmCallin("DownloadProgress", &query, false);
	if (m_DownloadProgressFuncPtr) {
		DownloadProgressResult result = {};
		m_DownloadProgressFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::DownloadQueued(int ID, const std::string& archiveName, const std::string& archiveType) {
	DownloadQueuedQuery query = {
		.downloadID = ID,
		.archiveName = archiveName.c_str(),
		.archiveType = archiveType.c_str()
	};
	DispatchWasmCallin("DownloadQueued", &query, false);
	if (m_DownloadQueuedFuncPtr) {
		DownloadQueuedResult result = {};
		m_DownloadQueuedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::DownloadStarted(int ID) {
	DownloadStartedQuery query = {.downloadID = ID};
	DispatchWasmCallin("DownloadStarted", &query, false);
	if (m_DownloadStartedFuncPtr) {
		DownloadStartedResult result = {};
		m_DownloadStartedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::Save(zipFile archive) {
	ArchiveCallinQuery query = {.archive = archive};
	DispatchWasmCallin("Save", &query, false);
	if (m_SaveFuncPtr) {
		ArchiveCallinResult result = {};
		m_SaveFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::LastMessagePosition(const float3& pos) {
	LastMessagePositionQuery query = {
		.pos = {.x = pos.x, .y = pos.y, .z = pos.z}
	};
	DispatchWasmCallin("LastMessagePosition", &query, false);
	if (m_LastMessagePositionFuncPtr) {
		LastMessagePositionResult result = {};
		m_LastMessagePositionFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::UnsyncedHeightMapUpdate(const SRectangle& rect) {
	RectChangedQuery query = {
		.x1 = rect.x1,
		.z1 = rect.z1,
		.x2 = rect.x2,
		.z2 = rect.z2
	};
	DispatchWasmCallin("UnsyncedHeightMapUpdate", &query, false);
	if (m_UnsyncedHeightMapUpdateFuncPtr) {
		RectChangedResult result = {};
		m_UnsyncedHeightMapUpdateFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

bool NativeInterfaceEventClient::KeyMapChanged() {
	SimpleCallinQuery query = {};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("KeyMapChanged", &query, false, wasmValue);
	if (m_KeyMapChangedFuncPtr) {
		BoolCallinResult result = {.value = false};
		m_KeyMapChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value || (hasWasmValue && wasmValue);
	}
	return hasWasmValue && wasmValue;
}

bool NativeInterfaceEventClient::KeyPress(int keyCode, int scanCode, bool isRepeat) {
	if (suppressNextKeyPress) {
		suppressNextKeyPress = false;
		return false;
	}

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
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("KeyPress", &query, false, wasmValue);
	if (m_KeyPressFuncPtr) {
		BoolCallinResult result = {.value = false};
		m_KeyPressFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value || (hasWasmValue && wasmValue);
	}
	return hasWasmValue && wasmValue;
}

bool NativeInterfaceEventClient::KeyRelease(int keyCode, int scanCode) {
	if (suppressNextKeyRelease) {
		suppressNextKeyRelease = false;
		return false;
	}

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
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("KeyRelease", &query, false, wasmValue);
	if (m_KeyReleaseFuncPtr) {
		BoolCallinResult result = {.value = false};
		m_KeyReleaseFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value || (hasWasmValue && wasmValue);
	}
	return hasWasmValue && wasmValue;
}

bool NativeInterfaceEventClient::TextInput(const std::string& utf8) {
	TextInputQuery query = {.utf8 = utf8.c_str()};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("TextInput", &query, false, wasmValue);
	if (m_TextInputFuncPtr) {
		BoolCallinResult result = {.value = false};
		m_TextInputFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value || (hasWasmValue && wasmValue);
	}
	return hasWasmValue && wasmValue;
}

bool NativeInterfaceEventClient::TextEditing(const std::string& utf8, unsigned int start, unsigned int length) {
	TextEditingQuery query = {.utf8 = utf8.c_str(), .start = start, .length = length};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("TextEditing", &query, false, wasmValue);
	if (m_TextEditingFuncPtr) {
		BoolCallinResult result = {.value = false};
		m_TextEditingFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value || (hasWasmValue && wasmValue);
	}
	return hasWasmValue && wasmValue;
}

bool NativeInterfaceEventClient::MouseMove(int x, int y, int dx, int dy, int button) {
	const LuaMousePosition position = ToLuaMousePosition(x, y);
	MouseMoveQuery query = {
		.x = position.x,
		.y = position.y,
		.dx = dx,
		.dy = -dy,
		.button = button,
	};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("MouseMove", &query, false, wasmValue);
	if (m_MouseMoveFuncPtr) {
		BoolCallinResult result = {.value = false};
		m_MouseMoveFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value || (hasWasmValue && wasmValue);
	}
	return hasWasmValue && wasmValue;
}

bool NativeInterfaceEventClient::MousePress(int x, int y, int button) {
	const LuaMousePosition position = ToLuaMousePosition(x, y);
	MousePressQuery query = {.x = position.x, .y = position.y, .button = button};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("MousePress", &query, false, wasmValue);
	if (m_MousePressFuncPtr) {
		BoolCallinResult result = {.value = false};
		m_MousePressFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value || (hasWasmValue && wasmValue);
	}
	return hasWasmValue && wasmValue;
}

void NativeInterfaceEventClient::MouseRelease(int x, int y, int button) {
	const LuaMousePosition position = ToLuaMousePosition(x, y);
	MouseReleaseQuery query = {.x = position.x, .y = position.y, .button = button};
	DispatchWasmCallin("MouseRelease", &query, false);
	if (m_MouseReleaseFuncPtr) {
		MouseReleaseResult result = {};
		m_MouseReleaseFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

bool NativeInterfaceEventClient::MouseWheel(bool up, float value) {
	MouseWheelQuery query = {.up = up, .value = value};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("MouseWheel", &query, false, wasmValue);
	if (m_MouseWheelFuncPtr) {
		BoolCallinResult result = {.value = false};
		m_MouseWheelFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value || (hasWasmValue && wasmValue);
	}
	return hasWasmValue && wasmValue;
}

bool NativeInterfaceEventClient::IsAbove(int x, int y) {
	const LuaMousePosition position = ToLuaMousePosition(x, y);
	ScreenPositionQuery query = {.x = position.x, .y = position.y};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("IsAbove", &query, false, wasmValue);
	if (m_IsAboveFuncPtr) {
		BoolCallinResult result = {.value = false};
		m_IsAboveFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value || (hasWasmValue && wasmValue);
	}
	return hasWasmValue && wasmValue;
}

std::string NativeInterfaceEventClient::GetTooltip(int x, int y) {
	const LuaMousePosition position = ToLuaMousePosition(x, y);
	ScreenPositionQuery query = {.x = position.x, .y = position.y};
	std::string wasmValue;
	const bool hasWasmValue = DispatchWasmStringCallin("GetTooltip", &query, false, wasmValue);
	if (m_GetTooltipFuncPtr) {
		StringCallinResult result = {};
		m_GetTooltipFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		if (result.value != nullptr && result.value[0] != '\0')
			return result.value;
	}
	return hasWasmValue ? wasmValue : "";
}

bool NativeInterfaceEventClient::DefaultCommand(const CUnit* unit, const CFeature* feature, int& cmd) {
	DefaultCommandQuery query = {
		.unitID = (unit != nullptr) ? unit->id : -1,
		.featureID = (feature != nullptr) ? feature->id : -1,
		.currentCommand = cmd
	};
	DefaultCommandResult directResult = {.error = nullptr, .value = false, .command = cmd};
	const bool hasWasmResult = DispatchWasmCallin("DefaultCommand", &query, false,
		&directResult);
	const bool hasWasmFields = hasWasmResult && directResult.error == nullptr;
	if (m_DefaultCommandFuncPtr) {
		DefaultCommandResult result = {.value = false, .command = cmd};
		m_DefaultCommandFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		if (result.value)
			cmd = result.command;
		return result.value;
	}
	if (hasWasmFields && directResult.value)
		cmd = directResult.command;
	return hasWasmFields && directResult.value;
}

void NativeInterfaceEventClient::ActiveCommandChanged(const SCommandDescription* cmdDesc) {
	ActiveCommandChangedQuery query = {
		.cmdID = (cmdDesc != nullptr) ? cmdDesc->id : -1,
		.cmdType = (cmdDesc != nullptr) ? cmdDesc->type : -1,
		.name = (cmdDesc != nullptr) ? cmdDesc->name.c_str() : "",
		.action = (cmdDesc != nullptr) ? cmdDesc->action.c_str() : "",
		.tooltip = (cmdDesc != nullptr) ? cmdDesc->tooltip.c_str() : ""
	};
	DispatchWasmCallin("ActiveCommandChanged", &query, false);
	if (m_ActiveCommandChangedFuncPtr) {
		ActiveCommandChangedResult result = {};
		m_ActiveCommandChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::CameraRotationChanged(const float3& rot) {
	Float3CallinQuery query = {
		.value = {.x = rot.x, .y = rot.y, .z = rot.z}
	};
	DispatchWasmCallin("CameraRotationChanged", &query, false);
	if (m_CameraRotationChangedFuncPtr) {
		Float3CallinResult result = {};
		m_CameraRotationChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::CameraPositionChanged(const float3& pos) {
	Float3CallinQuery query = {
		.value = {.x = pos.x, .y = pos.y, .z = pos.z}
	};
	DispatchWasmCallin("CameraPositionChanged", &query, false);
	if (m_CameraPositionChangedFuncPtr) {
		Float3CallinResult result = {};
		m_CameraPositionChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

bool NativeInterfaceEventClient::CommandNotify(const Command& cmd) {
	CommandNotifyQuery query = {.command = ToNativeCallinCommand(cmd)};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("CommandNotify", &query, false, wasmValue);
	if (m_CommandNotifyFuncPtr) {
		BoolCallinResult result = {.value = false};
		m_CommandNotifyFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value || (hasWasmValue && wasmValue);
	}
	return hasWasmValue && wasmValue;
}

bool NativeInterfaceEventClient::AddConsoleLine(const std::string& msg, const std::string& section, int level) {
	AddConsoleLineQuery query = {.message = msg.c_str(), .section = section.c_str(), .level = level};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("AddConsoleLine", &query, false, wasmValue);
	if (m_AddConsoleLineFuncPtr) {
		BoolCallinResult result = {.value = false};
		m_AddConsoleLineFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value || (hasWasmValue && wasmValue);
	}
	return hasWasmValue && wasmValue;
}

bool NativeInterfaceEventClient::GroupChanged(int groupID) {
	GroupChangedQuery query = {.groupID = groupID};
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("GroupChanged", &query, false, wasmValue);
	if (m_GroupChangedFuncPtr) {
		BoolCallinResult result = {.value = false};
		m_GroupChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value || (hasWasmValue && wasmValue);
	}
	return hasWasmValue && wasmValue;
}

void NativeInterfaceEventClient::MiniMapRotationChanged(float newRot, float oldRot) {
	MiniMapRotationChangedQuery query = {.newRot = newRot, .oldRot = oldRot};
	DispatchWasmCallin("MiniMapRotationChanged", &query, false);
	if (m_MiniMapRotationChangedFuncPtr == nullptr)
		return;

	SimpleCallinResult result = {};
	m_MiniMapRotationChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
}

void NativeInterfaceEventClient::MiniMapStateChanged(bool isMinimized, bool isMaximized, bool isSlaved) {
	MiniMapStateChangedQuery query = {
		.isMinimized = isMinimized,
		.isMaximized = isMaximized,
		.isSlaved = isSlaved,
	};
	DispatchWasmCallin("MiniMapStateChanged", &query, false);
	if (m_MiniMapStateChangedFuncPtr == nullptr)
		return;

	SimpleCallinResult result = {};
	m_MiniMapStateChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
}

void NativeInterfaceEventClient::MiniMapGeometryChanged(int2 newPos, int2 newDim, int2 oldPos, int2 oldDim) {
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
	DispatchWasmCallin("MiniMapGeometryChanged", &query, false);
	if (m_MiniMapGeometryChangedFuncPtr == nullptr)
		return;

	SimpleCallinResult result = {};
	m_MiniMapGeometryChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
}

bool NativeInterfaceEventClient::GameSetup(const std::string& state, bool& ready, const std::vector<std::pair<int, std::string>>& playerStates) {
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
	DispatchWasmCallin("GameSetup", &query, false);
	if (m_GameSetupFuncPtr) {
		GameSetupResult result = {.handled = false, .ready = ready};
		m_GameSetupFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		if (result.handled)
			ready = result.ready;
		return result.handled;
	}
	return false;
}

std::string NativeInterfaceEventClient::WorldTooltip(const CUnit* unit, const CFeature* feature, const float3* groundPos) {
	WorldTooltipQuery query = {
		.kind = (unit != nullptr) ? 1 : ((feature != nullptr) ? 2 : ((groundPos != nullptr) ? 3 : 0)),
		.unitID = (unit != nullptr) ? unit->id : -1,
		.featureID = (feature != nullptr) ? feature->id : -1,
		.groundPos = (groundPos != nullptr) ? Float3{.x = groundPos->x, .y = groundPos->y, .z = groundPos->z} : Float3{}
	};
	std::string wasmValue;
	const bool hasWasmValue = DispatchWasmStringCallin("WorldTooltip", &query, false, wasmValue);
	if (m_WorldTooltipFuncPtr) {
		StringCallinResult result = {};
		m_WorldTooltipFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		if (result.value != nullptr && result.value[0] != '\0')
			return result.value;
	}
	return hasWasmValue ? wasmValue : "";
}

bool NativeInterfaceEventClient::MapDrawCmd(int playerID, int type, const float3* pos0, const float3* pos1, const std::string* label) {
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
	bool wasmValue = false;
	const bool hasWasmValue = DispatchWasmBoolCallin("MapDrawCmd", &query, false, wasmValue);
	if (m_MapDrawCmdFuncPtr) {
		BoolCallinResult result = {.value = false};
		m_MapDrawCmdFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
		return result.value || (hasWasmValue && wasmValue);
	}
	return hasWasmValue && wasmValue;
}

void NativeInterfaceEventClient::ViewResize() {
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
	DispatchWasmCallin("ViewResize", &query, false);
	if (m_ViewResizeFuncPtr) {
		ViewResizeResult result = {};
		m_ViewResizeFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::SunChanged() {
	SunChangedQuery query = {};
	DispatchWasmCallin("SunChanged", &query, false);
	if (m_SunChangedFuncPtr) {
		SunChangedResult result = {};
		m_SunChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::FontsChanged() {
	SimpleCallinQuery query = {};
	DispatchWasmCallin("FontsChanged", &query, false);
	if (m_FontsChangedFuncPtr) {
		SimpleCallinResult result = {};
		m_FontsChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::GameProgress(int gameFrame) {
	GameProgressQuery query = {.gameFrame = gameFrame};
	DispatchWasmCallin("GameProgress", &query, false);
	if (m_GameProgressFuncPtr) {
		GameProgressResult result = {};
		m_GameProgressFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::StockpileChanged(const CUnit* unit, const CWeapon* weapon, int oldCount) {
	StockpileChangedQuery query = {
		.unitID = unit->id,
		.unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1,
		.unitTeam = unit->team,
		.weaponNum = (weapon != nullptr) ? weapon->weaponNum + 1 : -1,
		.oldCount = oldCount,
		.newCount = (weapon != nullptr) ? weapon->numStockpiled : -1
	};
	DispatchWasmCallin("StockpileChanged", &query, true);
	if (m_StockpileChangedFuncPtr) {
		StockpileChangedResult result = {};
		m_StockpileChangedFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::CollectGarbage(bool forced) {
	CollectGarbageQuery query = {.forced = forced};
	DispatchWasmCallin("CollectGarbage", &query, false);
	if (m_CollectGarbageFuncPtr) {
		CollectGarbageResult result = {};
		m_CollectGarbageFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::Pong(uint8_t pingTag, const spring_time pktSendTime, const spring_time pktRecvTime) {
	PongQuery query = {
		.pingTag = pingTag,
		.packetSendTimeMillis = pktSendTime.toMilliSecsi(),
		.packetRecvTimeMillis = pktRecvTime.toMilliSecsi()
	};
	DispatchWasmCallin("Pong", &query, false);
	if (m_PongFuncPtr) {
		PongResult result = {};
		m_PongFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::HandleLuaMsg(int playerID, int script, int mode, const std::vector<std::uint8_t>& data) {
	HandleLuaMsgQuery query = {
		.playerID = playerID,
		.script = script,
		.mode = mode,
		.data = data.data(),
		.dataLength = static_cast<int32_t>(data.size())
	};
	DispatchWasmCallin("HandleLuaMsg", &query, false);
	if (m_HandleLuaMsgFuncPtr) {
		HandleLuaMsgResult result = {};
		m_HandleLuaMsgFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}

void NativeInterfaceEventClient::HandleLuaCall(const char* msg, size_t msgLength, bool synced) {
	HandleLuaCallQuery query = {
		.message = msg,
		.messageLength = static_cast<uint32_t>(msgLength),
	};
	DispatchWasmCallin("HandleLuaCall", &query, synced);
	if (m_HandleLuaCallFuncPtr) {
		HandleLuaCallResult result = {};
		ScopedNativeSyncedCode syncedCode(synced);
		m_HandleLuaCallFuncPtr(m_nativeInterface, m_moduleData, &query, &result);
	}
}
