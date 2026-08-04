/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Callin Event Structures
// These define the interface for events from Spring Engine to native modules
// ============================================================================

// Initialization
struct InitializeNativeModuleQuery {
	uint32_t hostVersionMajor;  // Host's API version
	uint32_t hostVersionMinor;
	uint32_t hostVersionPatch;
};

struct InitializeNativeModuleResult {
	const Error* error;
	void* moduleData;  // Opaque pointer returned to module
	uint32_t moduleVersionMajor;  // Module's API version (for diagnostics)
	uint32_t moduleVersionMinor;
	uint32_t moduleVersionPatch;
};

// Download events
struct DownloadFailedQuery {
	int32_t downloadID;
	int32_t errorID;
};

struct DownloadFailedResult {
	const Error* error;
};

struct DownloadFinishedQuery {
	int32_t downloadID;
};

struct DownloadFinishedResult {
	const Error* error;
};

struct DownloadProgressQuery {
	int32_t downloadID;
	int64_t downloaded;
	int64_t total;
};

struct DownloadProgressResult {
	const Error* error;
};

struct DownloadQueuedQuery {
	int32_t downloadID;
	const char* archiveName;
	const char* archiveType;
};

struct DownloadQueuedResult {
	const Error* error;
};

struct DownloadStartedQuery {
	int32_t downloadID;
};

struct DownloadStartedResult {
	const Error* error;
};

// Feature events
struct FeatureCreatedQuery {
	int32_t featureID;
};

struct FeatureCreatedResult {
	const Error* error;
};

struct FeatureDestroyedQuery {
	int32_t featureID;
};

struct FeatureDestroyedResult {
	const Error* error;
};

// Game events
struct GameIDQuery {
	const uint8_t* gameID;
	uint32_t numBytes;
};

struct GameIDResult {
	const Error* error;
};

struct GamePausedQuery {
	int32_t playerID;
	bool paused;
};

struct GamePausedResult {
	const Error* error;
};

struct GamePreloadQuery {
	// No input parameters
};

struct GamePreloadResult {
	const Error* error;
};

struct GameStartQuery {
	// No input parameters
};

struct GameStartResult {
	const Error* error;
};

// The synced-control API has its own `GameOverQuery` (ends the game); rename
// the callin variant to disambiguate.
struct GameOverEventQuery {
	const uint8_t* winningAllyTeams;
	uint32_t count;
};

struct GameOverEventResult {
	const Error* error;
};

struct GameFrameQuery {
	int32_t gameFrame;
};

struct GameFrameResult {
	const Error* error;
};

struct GameFramePostQuery {
	int32_t gameFrame;
};

struct GameFramePostResult {
	const Error* error;
};

// Player events
struct PlayerAddedQuery {
	int32_t playerID;
};

struct PlayerAddedResult {
	const Error* error;
};

struct PlayerChangedQuery {
	int32_t playerID;
};

struct PlayerChangedResult {
	const Error* error;
};

struct PlayerRemovedQuery {
	int32_t playerID;
	int32_t reason;
};

struct PlayerRemovedResult {
	const Error* error;
};

// Team events
struct TeamChangedQuery {
	int32_t teamID;
};

struct TeamChangedResult {
	const Error* error;
};

struct TeamDiedQuery {
	int32_t teamID;
};

struct TeamDiedResult {
	const Error* error;
};

// Unit events
struct UnitCreatedQuery {
	int32_t unitID;
	int32_t builderID;  // -1 if none
};

struct UnitCreatedResult {
	const Error* error;
};

struct UnitDestroyedQuery {
	int32_t unitID;
	int32_t attackerID;  // -1 if none
};

struct UnitDestroyedResult {
	const Error* error;
};

struct UnitExperienceQuery {
	int32_t unitID;
	float oldExperience;
};

struct UnitExperienceResult {
	const Error* error;
};

struct UnitFinishedQuery {
	int32_t unitID;
};

struct UnitFinishedResult {
	const Error* error;
};

struct UnitReverseBuiltQuery {
	int32_t unitID;
};

struct UnitReverseBuiltResult {
	const Error* error;
};

struct UnitConstructionDecayedQuery {
	int32_t unitID;
	float timeSinceLastBuild;
	float iterationPeriod;
	float part;
};

struct UnitConstructionDecayedResult {
	const Error* error;
};

struct UnitFromFactoryQuery {
	int32_t unitID;
	int32_t factoryID;
	bool userOrders;
};

struct UnitFromFactoryResult {
	const Error* error;
};

struct UnitGivenQuery {
	int32_t unitID;
	int32_t oldTeam;
	int32_t newTeam;
};

struct UnitGivenResult {
	const Error* error;
};

struct UnitIdleQuery {
	int32_t unitID;
};

struct UnitIdleResult {
	const Error* error;
};

struct UnitHarvestStorageFullQuery {
	int32_t unitID;
};

struct UnitHarvestStorageFullResult {
	const Error* error;
};

struct UnitLosEventQuery {
	int32_t unitID;
	int32_t allyTeam;
};

struct UnitLosEventResult {
	const Error* error;
};

struct UnitMovementClassEventQuery {
	int32_t unitID;
};

struct UnitMovementClassEventResult {
	const Error* error;
};

struct UnitLoadedQuery {
	int32_t unitID;
	int32_t transportID;
};

struct UnitLoadedResult {
	const Error* error;
};

struct UnitStunnedQuery {
	int32_t unitID;
	bool stunned;
};

struct UnitStunnedResult {
	const Error* error;
};

struct UnitTakenQuery {
	int32_t unitID;
	int32_t oldTeam;
	int32_t newTeam;
};

struct UnitTakenResult {
	const Error* error;
};

struct UnitUnloadedQuery {
	int32_t unitID;
	int32_t transportID;
};

struct UnitUnloadedResult {
	const Error* error;
};

struct UnitCloakEventQuery {
	int32_t unitID;
};

struct UnitCloakEventResult {
	const Error* error;
};

struct UnitMoveEventQuery {
	int32_t unitID;
};

struct UnitMoveEventResult {
	const Error* error;
};

struct RenderUnitDestroyedQuery {
	int32_t unitID;
};

struct RenderUnitDestroyedResult {
	const Error* error;
};

struct FeatureMovedQuery {
	int32_t featureID;
	Float3 oldPos;
};

struct FeatureMovedResult {
	const Error* error;
};

struct ProjectileEventQuery {
	int32_t projectileID;
	int32_t ownerID;
	int32_t weaponDefID;
};

struct ProjectileEventResult {
	const Error* error;
};

struct LastMessagePositionQuery {
	Float3 pos;
};

struct LastMessagePositionResult {
	const Error* error;
};

struct ViewResizeQuery {
	uint8_t _unused;
};

struct ViewResizeResult {
	const Error* error;
};

struct SunChangedQuery {
	uint8_t _unused;
};

struct SunChangedResult {
	const Error* error;
};

struct GameProgressQuery {
	int32_t gameFrame;
};

struct GameProgressResult {
	const Error* error;
};

struct PongQuery {
	uint8_t pingTag;
	int64_t packetSendTimeMillis;
	int64_t packetRecvTimeMillis;
};

struct PongResult {
	const Error* error;
};

struct NativeCallinCommand {
	int32_t id;
	int32_t timeOut;
	uint32_t pageIndex;
	uint32_t numParams;
	uint32_t tag;
	uint8_t options;
	const float* params;
};

struct UnitCommandQuery {
	int32_t unitID;
	int32_t unitDefID;
	int32_t unitTeam;
	NativeCallinCommand command;
	int32_t playerNum;
	bool fromSynced;
	bool fromLua;
};

struct UnitCommandResult {
	const Error* error;
};

struct UnitCmdDoneQuery {
	int32_t unitID;
	int32_t unitDefID;
	int32_t unitTeam;
	NativeCallinCommand command;
};

struct UnitCmdDoneResult {
	const Error* error;
};

struct UnitDamagedQuery {
	int32_t unitID;
	int32_t unitDefID;
	int32_t unitTeam;
	float damage;
	bool paralyzer;
	int32_t weaponDefID;
	int32_t projectileID;
	int32_t attackerID;
	int32_t attackerDefID;
	int32_t attackerTeam;
};

struct UnitDamagedResult {
	const Error* error;
};

struct UnitSeismicPingQuery {
	Float3 pos;
	float strength;
	int32_t allyTeam;
	int32_t unitID;
	int32_t unitDefID;
};

struct UnitSeismicPingResult {
	const Error* error;
};

struct FeatureDamagedQuery {
	int32_t featureID;
	int32_t featureDefID;
	int32_t featureTeam;
	float damage;
	int32_t weaponDefID;
	int32_t projectileID;
	int32_t attackerID;
	int32_t attackerDefID;
	int32_t attackerTeam;
};

struct FeatureDamagedResult {
	const Error* error;
};

struct StockpileChangedQuery {
	int32_t unitID;
	int32_t unitDefID;
	int32_t unitTeam;
	int32_t weaponNum;
	int32_t oldCount;
	int32_t newCount;
};

struct StockpileChangedResult {
	const Error* error;
};

struct RectChangedQuery {
	int32_t x1;
	int32_t z1;
	int32_t x2;
	int32_t z2;
};

struct RectChangedResult {
	const Error* error;
};

struct Float3CallinQuery {
	Float3 value;
};

struct Float3CallinResult {
	const Error* error;
};

struct BoolCallinResult {
	const Error* error;
	bool value;
};

// Results used by synced control callins.  The initial values are supplied by
// the engine so a native module can preserve the engine default when it does
// not handle an event.
struct AllowUnitCreationResult {
	const Error* error;
	bool allow;
	bool dropOrder;
};

struct IntCallinResult {
	const Error* error;
	int32_t value;
};

struct DamageCallinResult {
	const Error* error;
	float newDamage;
	float impulseMult;
};

struct AllowWeaponTargetResult {
	const Error* error;
	bool allowed;
	float targetPriority;
};

struct CommandFallbackQuery {
	int32_t unitID;
	int32_t unitDefID;
	int32_t unitTeam;
	NativeCallinCommand command;
};

struct AllowUnitCreationQuery {
	int32_t unitDefID;
	int32_t builderID;
	int32_t builderTeam;
	bool hasBuildInfo;
	Float3 buildPos;
	int32_t buildFacing;
};

struct AllowUnitTransferQuery {
	int32_t unitID;
	int32_t unitDefID;
	int32_t oldTeam;
	int32_t newTeam;
	bool capture;
};

struct AllowUnitBuildStepQuery {
	int32_t builderID;
	int32_t builderTeam;
	int32_t unitID;
	int32_t unitDefID;
	float part;
};

struct AllowUnitTransportQuery {
	int32_t transporterID;
	int32_t transporterDefID;
	int32_t transporterTeam;
	int32_t transporteeID;
	int32_t transporteeDefID;
	int32_t transporteeTeam;
};

struct AllowUnitTransportPositionQuery {
	AllowUnitTransportQuery units;
	Float3 position;
	bool allowed;
};

struct AllowUnitCloakQuery {
	int32_t unitID;
	bool hasEnemy;
	int32_t enemyID;
};

struct AllowUnitDecloakQuery {
	int32_t unitID;
	bool hasObject;
	int32_t objectID;
	bool hasWeapon;
	int32_t weaponNum;
};

struct AllowUnitKamikazeQuery {
	int32_t unitID;
	int32_t targetID;
	bool allowed;
};

struct AllowFeatureCreationQuery {
	int32_t featureDefID;
	int32_t teamID;
	Float3 position;
};

struct AllowFeatureBuildStepQuery {
	int32_t builderID;
	int32_t builderTeam;
	int32_t featureID;
	int32_t featureDefID;
	float part;
};

struct AllowResourceLevelQuery {
	int32_t teamID;
	const char* type;
	float level;
};

struct AllowResourceTransferQuery {
	int32_t oldTeam;
	int32_t newTeam;
	const char* type;
	float amount;
};

struct ResourceExcessEntry {
	int32_t teamID;
	float resources[2];
};

struct ResourceExcessQuery {
	const ResourceExcessEntry* entries;
	uint32_t count;
};

struct AllowDirectUnitControlQuery {
	int32_t unitID;
	int32_t unitDefID;
	int32_t unitTeam;
	int32_t playerID;
};

struct AllowBuilderHoldFireQuery {
	int32_t unitID;
	int32_t unitDefID;
	int32_t action;
};

struct AllowStartPositionQuery {
	int32_t playerID;
	int32_t teamID;
	uint8_t readyState;
	Float3 clampedPos;
	Float3 rawPickPos;
};

struct TerraformCompleteQuery {
	int32_t unitID;
	int32_t unitDefID;
	int32_t unitTeam;
	int32_t buildUnitID;
	int32_t buildUnitDefID;
	int32_t buildUnitTeam;
};

struct MoveCtrlNotifyQuery {
	int32_t unitID;
	int32_t unitDefID;
	int32_t unitTeam;
	int32_t data;
};

struct AllowWeaponTargetCheckQuery {
	uint32_t attackerID;
	int32_t attackerWeaponNum;
	uint32_t attackerWeaponDefID;
};

struct AllowWeaponTargetQuery {
	uint32_t attackerID;
	uint32_t targetID;
	int32_t attackerWeaponNum;
	uint32_t attackerWeaponDefID;
	bool hasTargetPriority;
	float targetPriority;
};

struct AllowWeaponInterceptTargetQuery {
	int32_t interceptorUnitID;
	int32_t interceptorWeaponID;
	int32_t interceptorTargetID;
};

struct ShieldPreDamagedQuery {
	int32_t projectileID;
	int32_t projectileOwnerID;
	int32_t shieldWeaponNum;
	int32_t shieldCarrierID;
	bool bounceProjectile;
	int32_t beamEmitterWeaponNum;
	int32_t beamEmitterUnitID;
	Float3 startPos;
	Float3 hitPos;
};

struct KeyPressQuery {
	int32_t keyCode;
	int32_t scanCode;
	bool isRepeat;
};

struct KeyReleaseQuery {
	int32_t keyCode;
	int32_t scanCode;
};

struct TextInputQuery {
	const char* utf8;
};

struct TextEditingQuery {
	const char* utf8;
	uint32_t start;
	uint32_t length;
};

struct MouseMoveQuery {
	int32_t x;
	int32_t y;
	int32_t dx;
	int32_t dy;
	int32_t button;
};

struct MousePressQuery {
	int32_t x;
	int32_t y;
	int32_t button;
};

struct MouseReleaseQuery {
	int32_t x;
	int32_t y;
	int32_t button;
};

struct MouseReleaseResult {
	const Error* error;
};

struct MouseWheelQuery {
	bool up;
	float value;
};

struct ScreenPositionQuery {
	int32_t x;
	int32_t y;
};

struct MiniMapRotationChangedQuery {
	float newRot;
	float oldRot;
};

struct MiniMapStateChangedQuery {
	bool isMinimized;
	bool isMaximized;
	bool isSlaved;
};

struct MiniMapGeometryChangedQuery {
	int32_t newPosX;
	int32_t newPosY;
	int32_t newDimX;
	int32_t newDimY;
	int32_t oldPosX;
	int32_t oldPosY;
	int32_t oldDimX;
	int32_t oldDimY;
};

struct DrawUnitQuery {
	int32_t unitID;
	int32_t drawMode;
};

struct DrawFeatureQuery {
	int32_t featureID;
	int32_t drawMode;
};

struct DrawShieldQuery {
	int32_t unitID;
	int32_t weaponID;
	int32_t drawMode;
};

struct DrawProjectileQuery {
	int32_t projectileID;
	int32_t drawMode;
};

struct DrawMaterialQuery {
	int32_t uuid;
	int32_t drawMode;
};

struct ActiveCommandChangedQuery {
	int32_t cmdID;
	int32_t cmdType;
	const char* name;
	const char* action;
	const char* tooltip;
};

struct ActiveCommandChangedResult {
	const Error* error;
};

struct CommandNotifyQuery {
	NativeCallinCommand command;
};

struct AddConsoleLineQuery {
	const char* message;
	const char* section;
	int32_t level;
};

struct GroupChangedQuery {
	int32_t groupID;
};

struct DefaultCommandQuery {
	int32_t unitID;
	int32_t featureID;
	int32_t currentCommand;
};

struct DefaultCommandResult {
	const Error* error;
	bool value;
	int32_t command;
};

struct MapDrawCmdQuery {
	int32_t playerID;
	int32_t type;
	bool hasPos0;
	Float3 pos0;
	bool hasPos1;
	Float3 pos1;
	bool hasLabel;
	const char* label;
};

struct ArchiveCallinQuery {
	void* archive;
};

struct ArchiveCallinResult {
	const Error* error;
};

struct UnitUnitCollisionQuery {
	int32_t colliderID;
	int32_t collideeID;
};

struct UnitFeatureCollisionQuery {
	int32_t colliderID;
	int32_t collideeID;
};

struct ExplosionQuery {
	int32_t weaponDefID;
	Float3 pos;
	int32_t ownerID;
	int32_t projectileID;
};

struct StringCallinResult {
	const Error* error;
	const char* value;
};

struct GameSetupQuery {
	const char* state;
	bool ready;
};

struct GameSetupResult {
	const Error* error;
	bool handled;
	bool ready;
};

struct WorldTooltipQuery {
	int32_t kind; // 0 selection, 1 unit, 2 feature, 3 ground
	int32_t unitID;
	int32_t featureID;
	Float3 groundPos;
};

// Special events
struct HandleLuaMsgQuery {
	int32_t playerID;
	int32_t script;
	int32_t mode;
	const uint8_t* data;
	int32_t dataLength;
};

struct HandleLuaMsgResult {
	const Error* error;
};

struct HandleLuaCallQuery {
	const char* message;
	uint32_t messageLength;
};

struct HandleLuaCallResult {
	const Error* error;
};

struct ShutdownQuery {
	// No input parameters
};

struct ShutdownResult {
	const Error* error;
};

// Per-render-frame callin (unsynced). Fires once per drawn frame even while the
// sim is paused — the native equivalent of widget:Update.
struct UpdateQuery {
	// No input parameters
};

struct UpdateResult {
	const Error* error;
};

// Draw-pass callin (unsynced). Runs inside the screen draw pass where a GL
// context is valid — the native equivalent of SpringBoard's delayGL. GfxApi
// operations are only valid when called from here.
struct DrawScreenQuery {
	// No input parameters
};

struct DrawScreenResult {
	const Error* error;
};

struct SimpleCallinQuery {
	uint8_t _unused;
};

struct SimpleCallinResult {
	const Error* error;
};

struct DrawWorldPreParticlesQuery {
	bool drawAboveWater;
	bool drawBelowWater;
	bool drawReflection;
	bool drawRefraction;
};

struct DrawWorldPreParticlesResult {
	const Error* error;
};

struct DrawBuildSquareQuery {
	int32_t unitDefID;
	int32_t x;
	int32_t z;
	int32_t facing;
	const uint8_t* statuses;
	uint32_t statusCount;
};

struct DrawBuildSquareResult {
	const Error* error;
};

struct DrawObjectsLuaQuery {
	bool deferredPass;
	bool drawReflection;
	bool drawRefraction;
};

struct DrawObjectsLuaResult {
	const Error* error;
};

struct DrawAlphaObjectsLuaQuery {
	bool drawReflection;
	bool drawRefraction;
};

struct DrawAlphaObjectsLuaResult {
	const Error* error;
};

struct CollectGarbageQuery {
	bool forced;
};

struct CollectGarbageResult {
	const Error* error;
};

#ifdef __cplusplus
}
#endif
