#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Synced Control API
// @see rts/Lua/LuaSyncedCtrl.cpp
//
// Game state modification (deterministic, synced across network)
// Split into logical sub-APIs for better organization
// ============================================================================

// ============================================================================
// Team Control
// ============================================================================

// Queries - Team Control
struct SetAllyQuery { int32_t firstAllyTeamID; int32_t secondAllyTeamID; bool allied; };
struct SetAllyResult { const Error* error; bool success; };

struct SetAllyTeamStartBoxQuery { int32_t allyTeamID; float minX; float minZ; float maxX; float maxZ; };
struct SetAllyTeamStartBoxResult { const Error* error; bool success; };

struct KillTeamQuery { int32_t teamID; };
struct KillTeamResult { const Error* error; bool success; };

struct AssignPlayerToTeamQuery { int32_t playerID; int32_t teamID; };
struct AssignPlayerToTeamResult { const Error* error; bool success; };

struct GameOverQuery { const int32_t* winningAllyTeams; uint32_t count; };
struct GameOverResult { const Error* error; bool success; };

struct SetGlobalLosQuery { int32_t allyTeamID; bool enabled; };
struct SetGlobalLosResult { const Error* error; bool success; };

struct AddTeamResourceQuery { int32_t teamID; const char* resourceType; float amount; };
struct AddTeamResourceResult { const Error* error; bool success; };

struct UseTeamResourceQuery { int32_t teamID; const char* resourceType; float amount; };
struct UseTeamResourceResult { const Error* error; bool success; };

struct SetTeamResourceQuery { int32_t teamID; const char* resourceType; float amount; };
struct SetTeamResourceResult { const Error* error; bool success; };

struct SetTeamShareLevelQuery { int32_t teamID; const char* resourceType; float shareLevel; };
struct SetTeamShareLevelResult { const Error* error; bool success; };

struct ShareTeamResourceQuery { int32_t teamID; int32_t targetTeamID; const char* resourceType; float amount; };
struct ShareTeamResourceResult { const Error* error; bool success; };

// Set team start position query
struct SetTeamStartPositionQuery { int32_t teamID; Float3 pos; };
struct SetTeamStartPositionResult { const Error* error; bool success; };

// Set player ready state query
struct SetPlayerReadyStateQuery { int32_t playerID; bool ready; };
struct SetPlayerReadyStateResult { const Error* error; bool success; };

// Transfer team max units query
struct TransferTeamMaxUnitsQuery {
	int32_t fromTeamID;
	int32_t toTeamID;
	int32_t amount;
};
struct TransferTeamMaxUnitsResult { const Error* error; bool success; };

struct TeamControlApi {
	void (*SetAlly)(const SetAllyQuery* query, SetAllyResult* result);
	void (*SetAllyTeamStartBox)(const SetAllyTeamStartBoxQuery* query, SetAllyTeamStartBoxResult* result);
	void (*KillTeam)(const KillTeamQuery* query, KillTeamResult* result);
	void (*AssignPlayerToTeam)(const AssignPlayerToTeamQuery* query, AssignPlayerToTeamResult* result);
	void (*GameOver)(const GameOverQuery* query, GameOverResult* result);
	void (*SetGlobalLos)(const SetGlobalLosQuery* query, SetGlobalLosResult* result);
	void (*AddTeamResource)(const AddTeamResourceQuery* query, AddTeamResourceResult* result);
	void (*UseTeamResource)(const UseTeamResourceQuery* query, UseTeamResourceResult* result);
	void (*SetTeamResource)(const SetTeamResourceQuery* query, SetTeamResourceResult* result);
	void (*SetTeamShareLevel)(const SetTeamShareLevelQuery* query, SetTeamShareLevelResult* result);
	void (*ShareTeamResource)(const ShareTeamResourceQuery* query, ShareTeamResourceResult* result);
	void (*SetTeamStartPosition)(const SetTeamStartPositionQuery* query, SetTeamStartPositionResult* result);
	void (*SetPlayerReadyState)(const SetPlayerReadyStateQuery* query, SetPlayerReadyStateResult* result);
	void (*TransferTeamMaxUnits)(const TransferTeamMaxUnitsQuery* query, TransferTeamMaxUnitsResult* result);
};

// ============================================================================
// COB Script Control
// ============================================================================

// Call COB script query
struct CobFunctionRef {
	const char* name;
	int32_t id;              // Use id >= 0 for raw function ID, otherwise use name.
};

struct CallCOBScriptQuery {
	int32_t unitID;
	CobFunctionRef func;
	uint32_t retArgs;           // Number of return values requested
	const int32_t* args;         // Arguments array
	uint32_t argCount;           // Number of arguments
};
struct CallCOBScriptResult {
	const Error* error;
	int32_t retCode;             // Return code from COB
	int32_t* retValues;          // Return values (up to retCount)
	uint32_t retCount;           // Actual number of return values
};

// Get COB script ID query
struct GetCOBScriptIDQuery {
	int32_t unitID;
	const char* funcName;
};
struct GetCOBScriptIDResult { const Error* error; int32_t funcID; };

struct COBScriptApi {
	void (*CallCOBScript)(const CallCOBScriptQuery* query, CallCOBScriptResult* result);
	void (*GetCOBScriptID)(const GetCOBScriptIDQuery* query, GetCOBScriptIDResult* result);
};

// ============================================================================
// Unit Control
// ============================================================================

// Queries - Unit Control
struct CreateUnitQuery { DefRef unitDef; Float3 pos; int32_t facing; int32_t teamID; bool build; bool flattenGround; int32_t unitID; int32_t builderID; };
struct CreateUnitResult { const Error* error; int32_t unitID; };

struct DestroyUnitQuery { int32_t unitID; bool selfd; bool reclaimed; int32_t attackerID; bool recycleID; };
struct DestroyUnitResult { const Error* error; bool success; };

struct TransferUnitQuery { int32_t unitID; int32_t newTeamID; bool given; bool adjustUnitLimit; };
struct TransferUnitResult { const Error* error; bool success; };

struct GiveOrderToUnitQuery { int32_t unitID; int32_t cmdID; float* params; uint32_t paramCount; uint32_t options; int32_t timeout; };
struct GiveOrderToUnitResult { const Error* error; bool success; };

struct GiveOrderToUnitArrayQuery { const int32_t* unitIDs; uint32_t count; int32_t cmdID; float* params; uint32_t paramCount; uint32_t options; int32_t timeout; };
struct GiveOrderToUnitArrayResult { const Error* error; bool success; };

// Single command for order arrays
struct NativeCommand {
	int32_t cmdID;
	float* params;
	uint32_t paramCount;
	uint32_t options;
	int32_t timeout;
};

// Give multiple orders to a single unit
struct GiveOrderArrayToUnitQuery {
	int32_t unitID;
	const NativeCommand* commands;
	uint32_t commandCount;
};
struct GiveOrderArrayToUnitResult { const Error* error; bool success; };

// Give multiple orders to multiple units
struct GiveOrderArrayToUnitArrayQuery {
	const int32_t* unitIDs;
	uint32_t unitCount;
	const NativeCommand* commands;
	uint32_t commandCount;
	bool pairwise;        // When true, assign commands by index (unit[i] gets command[i])
};
struct GiveOrderArrayToUnitArrayResult { const Error* error; int32_t unitsOrdered; };

struct UnitFinishCommandQuery { int32_t unitID; };
struct UnitFinishCommandResult { const Error* error; bool success; };

struct SetUnitHealthQuery { int32_t unitID; UnitHealthValue value; };
struct SetUnitHealthResult { const Error* error; bool success; };

struct SetUnitMaxHealthQuery { int32_t unitID; float maxHealth; };
struct SetUnitMaxHealthResult { const Error* error; bool success; };

struct SetUnitExperienceQuery { int32_t unitID; float experience; };
struct SetUnitExperienceResult { const Error* error; bool success; };

struct AddUnitExperienceQuery { int32_t unitID; float experience; };
struct AddUnitExperienceResult { const Error* error; bool success; };

struct SetUnitNeutralQuery { int32_t unitID; bool neutral; };
struct SetUnitNeutralResult { const Error* error; bool success; };

struct SetUnitResourcingQuery { int32_t unitID; const char* type; float amount; };
struct SetUnitResourcingResult { const Error* error; bool success; };

struct SetUnitMetalExtractionQuery { int32_t unitID; float depth; float range; };
struct SetUnitMetalExtractionResult { const Error* error; bool success; };

struct SetUnitPositionQuery { int32_t unitID; Float3 pos; };
struct SetUnitPositionResult { const Error* error; bool success; };

struct SetUnitVelocityQuery { int32_t unitID; Float3 velocity; };
struct SetUnitVelocityResult { const Error* error; bool success; };

struct SetUnitRotationQuery { int32_t unitID; Float3 rotation; };
struct SetUnitRotationResult { const Error* error; bool success; };

struct SetUnitPhysicsQuery { int32_t unitID; Float3 pos; Float3 velocity; Float3 rotation; Float3 drag; };
struct SetUnitPhysicsResult { const Error* error; bool success; };

struct AddUnitDamageQuery { int32_t unitID; float damage; float paralyzeTime; int32_t weaponDefID; int32_t attackerID; Float3 impulse; };
struct AddUnitDamageResult { const Error* error; bool success; };

struct AddUnitImpulseQuery { int32_t unitID; Float3 impulse; float decayRate; };
struct AddUnitImpulseResult { const Error* error; bool success; };

struct SetUnitCloakQuery { int32_t unitID; NumberOrBool cloak; NumberOrBool cloakArg; };
struct SetUnitCloakResult { const Error* error; bool success; };

struct SetUnitStealthQuery { int32_t unitID; bool stealth; };
struct SetUnitStealthResult { const Error* error; bool success; };

struct SetUnitSonarStealthQuery { int32_t unitID; bool sonarStealth; };
struct SetUnitSonarStealthResult { const Error* error; bool success; };

struct SetUnitSeismicSignatureQuery { int32_t unitID; float seismicSignature; };
struct SetUnitSeismicSignatureResult { const Error* error; bool success; };

struct SetUnitArmoredQuery { int32_t unitID; bool armoredState; float armoredMultiple; };
struct SetUnitArmoredResult { const Error* error; bool success; };

struct SetUnitBlockingQuery { int32_t unitID; bool blocking; bool solidObjects; bool projectiles; bool quadMapRays; bool crushable; bool blockEnemyPushing; bool blockHeightChanges; };
struct SetUnitBlockingResult { const Error* error; bool success; };

struct SetUnitMassQuery { int32_t unitID; float mass; };
struct SetUnitMassResult { const Error* error; bool success; };

struct SetUnitLeavesGhostQuery { int32_t unitID; bool leavesGhost; bool leaveDeadGhost; };
struct SetUnitLeavesGhostResult { const Error* error; bool success; };

struct SetUnitAlwaysVisibleQuery { int32_t unitID; bool alwaysVisible; };
struct SetUnitAlwaysVisibleResult { const Error* error; bool success; };

struct SetUnitUseAirLosQuery { int32_t unitID; bool useAirLos; };
struct SetUnitUseAirLosResult { const Error* error; bool success; };

// Unit read queries (synced)
struct GetUnitLeavesGhostQuery { int32_t unitID; };
struct GetUnitLeavesGhostResult { const Error* error; bool leavesGhost; };

struct GetUnitPhysicalStateQuery { int32_t unitID; };
struct GetUnitPhysicalStateResult { const Error* error; uint32_t physicalState; };

struct GetUnitFeatureSeparationQuery { int32_t unitID; int32_t featureID; bool ignoreY; };
struct GetUnitFeatureSeparationResult { const Error* error; float distance; };

// Command Description structure for native API
struct NativeCommandDescription {
	int32_t id;
	int32_t type;
	bool queueing;
	bool hidden;
	bool disabled;
	bool showUnique;
	bool onlyTexture;
	const char* name;
	const char* action;
	const char* iconname;
	const char* mouseicon;
	const char* tooltip;
	const char** params;
	uint32_t paramCount;
};

// Command Description queries
struct EditUnitCmdDescQuery { int32_t unitID; uint32_t cmdDescIndex; const NativeCommandDescription* cmdDesc; };
struct EditUnitCmdDescResult { const Error* error; bool success; };

struct InsertUnitCmdDescQuery { int32_t unitID; int32_t cmdDescIndex; const NativeCommandDescription* cmdDesc; };  // -1 for append
struct InsertUnitCmdDescResult { const Error* error; bool success; };

struct RemoveUnitCmdDescQuery { int32_t unitID; int32_t cmdDescIndex; };  // -1 for last
struct RemoveUnitCmdDescResult { const Error* error; bool success; };

// Unit costs queries
struct SetUnitCostsQuery { int32_t unitID; UnitCostOverrides costs; };
struct SetUnitCostsResult { const Error* error; bool success; };

// Unit build speed queries (for builders/factories)
struct SetUnitBuildSpeedQuery { int32_t unitID; float buildSpeed; float repairSpeed; float reclaimSpeed; float resurrectSpeed; float captureSpeed; float terraformSpeed; };
struct SetUnitBuildSpeedResult { const Error* error; bool success; };

// Unit collision volume queries
struct SetUnitCollisionVolumeDataQuery {
	int32_t unitID;
	Float3 scales;
	Float3 offsets;
	int32_t volumeType;      // COLVOL_TYPE_*
	int32_t testType;        // COLVOL_HITTEST_*
	int32_t primaryAxis;     // COLVOL_AXIS_*
};
struct SetUnitCollisionVolumeDataResult { const Error* error; bool success; };

// Unit selection volume queries
struct SetUnitSelectionVolumeDataQuery {
	int32_t unitID;
	Float3 scales;
	Float3 offsets;
	int32_t volumeType;
	int32_t testType;
	int32_t primaryAxis;
};
struct SetUnitSelectionVolumeDataResult { const Error* error; bool success; };

// Unit piece collision volume queries
struct SetUnitPieceCollisionVolumeDataQuery {
	int32_t unitID;
	int32_t pieceIndex;
	bool enable;
	Float3 scales;
	Float3 offsets;
	int32_t volumeType;
	int32_t primaryAxis;
};
struct SetUnitPieceCollisionVolumeDataResult { const Error* error; bool success; };

// Unit target queries
struct SetUnitTargetQuery {
	int32_t unitID;
	UnitTargetRef target;
	bool manualFire;
	bool userTarget;
	int32_t weaponNum;          // -1 for all weapons
};
struct SetUnitTargetResult { const Error* error; bool success; };

// Unit shield queries
struct SetUnitShieldStateQuery {
	int32_t unitID;
	int32_t weaponNum;          // -1 for default shield
	bool enabled;
	float power;
};
struct SetUnitShieldStateResult { const Error* error; bool success; };

struct SetUnitShieldRechargeDelayQuery {
	int32_t unitID;
	int32_t weaponNum;          // -1 for default shield
	float rechargeDelay;        // seconds, -1 to use default
};
struct SetUnitShieldRechargeDelayResult { const Error* error; bool success; };

// Unit flanking queries
struct SetUnitFlankingQuery {
	int32_t unitID;
	const char* type;
	Float3 args;
};
struct SetUnitFlankingResult { const Error* error; bool success; };

// Unit mid/aim position queries
struct SetUnitMidAndAimPosQuery {
	int32_t unitID;
	Float3 midPos;
	Float3 aimPos;
	bool setRelative;
};
struct SetUnitMidAndAimPosResult { const Error* error; bool success; };

// Unit radius/height queries
struct SetUnitRadiusAndHeightQuery {
	int32_t unitID;
	float radius;
	float height;
};
struct SetUnitRadiusAndHeightResult { const Error* error; bool success; };

// Unit movement queries
struct SetUnitMoveGoalQuery {
	int32_t unitID;
	Float3 pos;
	float radius;
	float speed;           // 0 for default
	bool raw;              // use raw movement (no pathfinding)
};
struct SetUnitMoveGoalResult { const Error* error; bool success; };

struct SetUnitLandGoalQuery {
	int32_t unitID;
	Float3 pos;
	float radiusSq;        // squared radius, -1 for default
};
struct SetUnitLandGoalResult { const Error* error; bool success; };

struct ClearUnitGoalQuery { int32_t unitID; bool cancelRaw; };
struct ClearUnitGoalResult { const Error* error; bool success; };

// Unit stockpile queries
struct SetUnitStockpileQuery {
	int32_t unitID;
	int32_t stockpile;     // -1 to not change
	float buildPercent;    // -1 to not change
};
struct SetUnitStockpileResult { const Error* error; bool success; };

struct SetUnitTravelQuery { uint8_t _unused; };
struct SetUnitTravelResult { const Error* error; bool success; };

struct SetUnitFuelQuery { uint8_t _unused; };
struct SetUnitFuelResult { const Error* error; bool success; };

// Unit direction queries
struct SetUnitDirectionQuery { int32_t unitID; Float3 frontDir; Float3 rightDir; };
struct SetUnitDirectionResult { const Error* error; bool success; };

// Unit attachment/transport queries
struct UnitAttachQuery {
	int32_t transporterID;
	int32_t transporteeID;
	int32_t pieceNum;           // 0-indexed, -1 for default
};
struct UnitAttachResult { const Error* error; bool success; };

struct UnitDetachQuery { int32_t transporteeID; };
struct UnitDetachResult { const Error* error; bool success; };

struct UnitDetachFromAirQuery {
	int32_t transporteeID;
	Float3 pos;
};
struct UnitDetachFromAirResult { const Error* error; bool success; };

struct SetUnitLoadingTransportQuery {
	int32_t unitID;
	int32_t transportID;        // -1 to clear
};
struct SetUnitLoadingTransportResult { const Error* error; bool success; };

struct SetUnitCrashingQuery {
	int32_t unitID;
	bool wantCrash;
};
struct SetUnitCrashingResult { const Error* error; bool stateChanged; };

// Unit weapon control queries
struct SetUnitWeaponStateQuery {
	int32_t unitID;
	int32_t weaponNum;          // 0-indexed
	const char* key;
	float value;
};
struct SetUnitWeaponStateResult { const Error* error; bool success; };

struct UnitWeaponFireQuery {
	int32_t unitID;
	int32_t weaponNum;          // 0-indexed
};
struct UnitWeaponFireResult { const Error* error; bool success; };

struct UnitWeaponHoldFireQuery {
	int32_t unitID;
	int32_t weaponNum;          // 0-indexed
};
struct UnitWeaponHoldFireResult { const Error* error; bool success; };

struct SetUnitUseWeaponsQuery {
	int32_t unitID;
	bool forceUseWeapons;
	bool allowUseWeapons;
};
struct SetUnitUseWeaponsResult { const Error* error; bool success; };

// Unit max range query
struct SetUnitMaxRangeQuery { int32_t unitID; float maxRange; };
struct SetUnitMaxRangeResult { const Error* error; bool success; };

// Unit physical state bit query
struct SetUnitPhysicalStateBitQuery { int32_t unitID; int32_t stateBit; };
struct SetUnitPhysicalStateBitResult { const Error* error; bool success; };

// Unit position error params query
struct SetUnitPosErrorParamsQuery {
	int32_t unitID;
	Float3 posErrorVector;
	Float3 posErrorDelta;
	int32_t nextPosErrorUpdate;
	int32_t allyTeamID;         // -1 to not set
	bool setPosErrorBit;        // whether to set the bit for allyTeamID
};
struct SetUnitPosErrorParamsResult { const Error* error; bool success; };

// Unit weapon damages query
struct SetUnitWeaponDamagesQuery {
	int32_t unitID;
	int32_t weaponNum;          // 0-indexed, -1 for "explode", -2 for "selfDestruct"
	const char* damageKey;      // damage type key (e.g., "paralyzeDamageTime", armor type index)
	float damageValue;
};
struct SetUnitWeaponDamagesResult { const Error* error; bool success; };

// Force unit collision update query
struct ForceUnitCollisionUpdateQuery { int32_t unitID; };
struct ForceUnitCollisionUpdateResult { const Error* error; bool success; };

// Set unit heading query
struct SetUnitHeadingQuery { int32_t unitID; int32_t heading; bool useSmoothing; };
struct SetUnitHeadingResult { const Error* error; bool success; };

// Set unit heading and up direction query
struct SetUnitHeadingAndUpDirQuery {
	int32_t unitID;
	int32_t heading;
	Float3 upDir;
};
struct SetUnitHeadingAndUpDirResult { const Error* error; bool success; };

// Add/remove object decal queries
struct AddObjectDecalQuery { int32_t unitID; };
struct AddObjectDecalResult { const Error* error; bool success; };

struct RemoveObjectDecalQuery { int32_t unitID; };
struct RemoveObjectDecalResult { const Error* error; bool success; };

// Set unit buildee radius query
struct SetUnitBuildeeRadiusQuery { int32_t unitID; float radius; };
struct SetUnitBuildeeRadiusResult { const Error* error; bool success; };

// Set unit sensor radius query
struct SetUnitSensorRadiusQuery {
	int32_t unitID;
	const char* sensorType;      // "los", "airLos", "radar", "sonar", "seismic", "radarJammer", "sonarJammer"
	int32_t radius;
};
struct SetUnitSensorRadiusResult { const Error* error; int32_t newRadius; };

// Set unit harvest storage query
struct SetUnitHarvestStorageQuery {
	int32_t unitID;
	float storedMetal;
	float maxStoredMetal;
	float storedEnergy;
	float maxStoredEnergy;
};
struct SetUnitHarvestStorageResult { const Error* error; bool success; };

// Set unit build params query
struct SetUnitBuildParamsQuery {
	int32_t unitID;
	const char* paramName;       // "buildRange", "buildDistance", "buildRange3D"
	NumberOrBool value;
};
struct SetUnitBuildParamsResult { const Error* error; bool success; };

// Set unit LOS mask query (mask disables engine updates for specific visibility bits)
struct SetUnitLosMaskQuery {
	int32_t unitID;
	int32_t allyTeamID;
	uint8_t losMask;             // bitmask: LOS_INLOS=1, LOS_INRADAR=2, LOS_PREVLOS=4, LOS_CONTRADAR=8
};
struct SetUnitLosMaskResult { const Error* error; bool success; };

// Set unit LOS state query (set current visibility state)
struct SetUnitLosStateQuery {
	int32_t unitID;
	int32_t allyTeamID;
	uint8_t losState;            // bitmask: LOS_INLOS=1, LOS_INRADAR=2, LOS_PREVLOS=4, LOS_CONTRADAR=8
};
struct SetUnitLosStateResult { const Error* error; bool success; };

// Set unit storage query
struct SetUnitStorageQuery {
	int32_t unitID;
	const char* resource;
	float amount;
};
struct SetUnitStorageResult { const Error* error; bool success; };

// Set unit tooltip query
struct SetUnitTooltipQuery {
	int32_t unitID;
	const char* tooltip;
};
struct SetUnitTooltipResult { const Error* error; bool success; };

// Set factory bugger off query
struct SetFactoryBuggerOffQuery {
	int32_t unitID;
	bool perform;
	float offset;
	float radius;
	int32_t relHeading;
	bool spherical;
	bool forced;
};
struct SetFactoryBuggerOffResult { const Error* error; bool perform; };

// Bugger off query (tells units to move away from position)
struct BuggerOffQuery {
	Float3 pos;
	float radius;
	int32_t teamID;
	bool spherical;
	bool forced;
	int32_t excludeUnitID;         // -1 for none
	const int32_t* excludeUnitDefIDs; // optional array of unit def IDs
	uint32_t excludeUnitDefCount;
};
struct BuggerOffResult { const Error* error; bool success; };

// Add unit seismic ping query
struct AddUnitSeismicPingQuery { int32_t unitID; float pingSize; };
struct AddUnitSeismicPingResult { const Error* error; bool success; };

// Add unit resource query
struct AddUnitResourceQuery {
	int32_t unitID;
	const char* resourceType;      // "m" or "e"
	float amount;
};
struct AddUnitResourceResult { const Error* error; bool success; };

// Use unit resource query
struct UseUnitResourceQuery {
	int32_t unitID;
	const char* resourceType;      // "m" or "e"
	float amount;
};
struct UseUnitResourceResult { const Error* error; bool success; };

// Set unit piece visible query
struct SetUnitPieceVisibleQuery {
	int32_t unitID;
	int32_t pieceIndex;      // 0-indexed
	bool visible;
};
struct SetUnitPieceVisibleResult { const Error* error; bool success; };

// Set unit piece parent query
struct SetUnitPieceParentQuery {
	int32_t unitID;
	int32_t childPieceIndex;    // 0-indexed
	int32_t parentPieceIndex;   // 0-indexed
};
struct SetUnitPieceParentResult { const Error* error; bool success; };

// Set unit piece matrix query
struct SetUnitPieceMatrixQuery {
	int32_t unitID;
	int32_t pieceIndex;      // 0-indexed
	float matrix[16];        // 4x4 transformation matrix (column-major)
};
struct SetUnitPieceMatrixResult { const Error* error; bool blockScriptAnims; };

// Set unit nano pieces query
struct SetUnitNanoPiecesQuery {
	int32_t unitID;
	const int32_t* pieceIndices;   // 0-indexed piece indices
	uint32_t pieceCount;
};
struct SetUnitNanoPiecesResult { const Error* error; bool success; };

struct UnitControlApi {
	void (*CreateUnit)(const CreateUnitQuery* query, CreateUnitResult* result);
	void (*DestroyUnit)(const DestroyUnitQuery* query, DestroyUnitResult* result);
	void (*TransferUnit)(const TransferUnitQuery* query, TransferUnitResult* result);
	void (*GiveOrderToUnit)(const GiveOrderToUnitQuery* query, GiveOrderToUnitResult* result);
	void (*GiveOrderToUnitArray)(const GiveOrderToUnitArrayQuery* query, GiveOrderToUnitArrayResult* result);
	void (*GiveOrderArrayToUnit)(const GiveOrderArrayToUnitQuery* query, GiveOrderArrayToUnitResult* result);
	void (*GiveOrderArrayToUnitArray)(const GiveOrderArrayToUnitArrayQuery* query, GiveOrderArrayToUnitArrayResult* result);
	void (*UnitFinishCommand)(const UnitFinishCommandQuery* query, UnitFinishCommandResult* result);
	void (*SetUnitHealth)(const SetUnitHealthQuery* query, SetUnitHealthResult* result);
	void (*SetUnitMaxHealth)(const SetUnitMaxHealthQuery* query, SetUnitMaxHealthResult* result);
	void (*SetUnitExperience)(const SetUnitExperienceQuery* query, SetUnitExperienceResult* result);
	void (*AddUnitExperience)(const AddUnitExperienceQuery* query, AddUnitExperienceResult* result);
	void (*SetUnitNeutral)(const SetUnitNeutralQuery* query, SetUnitNeutralResult* result);
	void (*SetUnitResourcing)(const SetUnitResourcingQuery* query, SetUnitResourcingResult* result);
	void (*SetUnitMetalExtraction)(const SetUnitMetalExtractionQuery* query, SetUnitMetalExtractionResult* result);
	void (*SetUnitPosition)(const SetUnitPositionQuery* query, SetUnitPositionResult* result);
	void (*SetUnitVelocity)(const SetUnitVelocityQuery* query, SetUnitVelocityResult* result);
	void (*SetUnitRotation)(const SetUnitRotationQuery* query, SetUnitRotationResult* result);
	void (*SetUnitPhysics)(const SetUnitPhysicsQuery* query, SetUnitPhysicsResult* result);
	void (*AddUnitDamage)(const AddUnitDamageQuery* query, AddUnitDamageResult* result);
	void (*AddUnitImpulse)(const AddUnitImpulseQuery* query, AddUnitImpulseResult* result);
	void (*SetUnitCloak)(const SetUnitCloakQuery* query, SetUnitCloakResult* result);
	void (*SetUnitStealth)(const SetUnitStealthQuery* query, SetUnitStealthResult* result);
	void (*SetUnitSonarStealth)(const SetUnitSonarStealthQuery* query, SetUnitSonarStealthResult* result);
	void (*SetUnitSeismicSignature)(const SetUnitSeismicSignatureQuery* query, SetUnitSeismicSignatureResult* result);
	void (*SetUnitArmored)(const SetUnitArmoredQuery* query, SetUnitArmoredResult* result);
	void (*SetUnitBlocking)(const SetUnitBlockingQuery* query, SetUnitBlockingResult* result);
	void (*SetUnitMass)(const SetUnitMassQuery* query, SetUnitMassResult* result);
	void (*SetUnitLeavesGhost)(const SetUnitLeavesGhostQuery* query, SetUnitLeavesGhostResult* result);
	void (*SetUnitAlwaysVisible)(const SetUnitAlwaysVisibleQuery* query, SetUnitAlwaysVisibleResult* result);
	void (*SetUnitUseAirLos)(const SetUnitUseAirLosQuery* query, SetUnitUseAirLosResult* result);
	void (*GetUnitLeavesGhost)(const GetUnitLeavesGhostQuery* query, GetUnitLeavesGhostResult* result);
	void (*GetUnitPhysicalState)(const GetUnitPhysicalStateQuery* query, GetUnitPhysicalStateResult* result);
	void (*GetUnitFeatureSeparation)(const GetUnitFeatureSeparationQuery* query, GetUnitFeatureSeparationResult* result);
	void (*EditUnitCmdDesc)(const EditUnitCmdDescQuery* query, EditUnitCmdDescResult* result);
	void (*InsertUnitCmdDesc)(const InsertUnitCmdDescQuery* query, InsertUnitCmdDescResult* result);
	void (*RemoveUnitCmdDesc)(const RemoveUnitCmdDescQuery* query, RemoveUnitCmdDescResult* result);
	void (*SetUnitCosts)(const SetUnitCostsQuery* query, SetUnitCostsResult* result);
	void (*SetUnitBuildSpeed)(const SetUnitBuildSpeedQuery* query, SetUnitBuildSpeedResult* result);
	void (*SetUnitCollisionVolumeData)(const SetUnitCollisionVolumeDataQuery* query, SetUnitCollisionVolumeDataResult* result);
	void (*SetUnitSelectionVolumeData)(const SetUnitSelectionVolumeDataQuery* query, SetUnitSelectionVolumeDataResult* result);
	void (*SetUnitPieceCollisionVolumeData)(const SetUnitPieceCollisionVolumeDataQuery* query, SetUnitPieceCollisionVolumeDataResult* result);
	void (*SetUnitTarget)(const SetUnitTargetQuery* query, SetUnitTargetResult* result);
	void (*SetUnitShieldState)(const SetUnitShieldStateQuery* query, SetUnitShieldStateResult* result);
	void (*SetUnitShieldRechargeDelay)(const SetUnitShieldRechargeDelayQuery* query, SetUnitShieldRechargeDelayResult* result);
	void (*SetUnitFlanking)(const SetUnitFlankingQuery* query, SetUnitFlankingResult* result);
	void (*SetUnitMidAndAimPos)(const SetUnitMidAndAimPosQuery* query, SetUnitMidAndAimPosResult* result);
	void (*SetUnitRadiusAndHeight)(const SetUnitRadiusAndHeightQuery* query, SetUnitRadiusAndHeightResult* result);
	void (*SetUnitMoveGoal)(const SetUnitMoveGoalQuery* query, SetUnitMoveGoalResult* result);
	void (*SetUnitLandGoal)(const SetUnitLandGoalQuery* query, SetUnitLandGoalResult* result);
	void (*ClearUnitGoal)(const ClearUnitGoalQuery* query, ClearUnitGoalResult* result);
	void (*SetUnitStockpile)(const SetUnitStockpileQuery* query, SetUnitStockpileResult* result);
	void (*SetUnitTravel)(const SetUnitTravelQuery* query, SetUnitTravelResult* result);
	void (*SetUnitFuel)(const SetUnitFuelQuery* query, SetUnitFuelResult* result);
	void (*SetUnitDirection)(const SetUnitDirectionQuery* query, SetUnitDirectionResult* result);
	void (*UnitAttach)(const UnitAttachQuery* query, UnitAttachResult* result);
	void (*UnitDetach)(const UnitDetachQuery* query, UnitDetachResult* result);
	void (*UnitDetachFromAir)(const UnitDetachFromAirQuery* query, UnitDetachFromAirResult* result);
	void (*SetUnitLoadingTransport)(const SetUnitLoadingTransportQuery* query, SetUnitLoadingTransportResult* result);
	void (*SetUnitCrashing)(const SetUnitCrashingQuery* query, SetUnitCrashingResult* result);
	void (*SetUnitWeaponState)(const SetUnitWeaponStateQuery* query, SetUnitWeaponStateResult* result);
	void (*UnitWeaponFire)(const UnitWeaponFireQuery* query, UnitWeaponFireResult* result);
	void (*UnitWeaponHoldFire)(const UnitWeaponHoldFireQuery* query, UnitWeaponHoldFireResult* result);
	void (*SetUnitUseWeapons)(const SetUnitUseWeaponsQuery* query, SetUnitUseWeaponsResult* result);
	void (*SetUnitMaxRange)(const SetUnitMaxRangeQuery* query, SetUnitMaxRangeResult* result);
	void (*SetUnitPhysicalStateBit)(const SetUnitPhysicalStateBitQuery* query, SetUnitPhysicalStateBitResult* result);
	void (*SetUnitPosErrorParams)(const SetUnitPosErrorParamsQuery* query, SetUnitPosErrorParamsResult* result);
	void (*SetUnitWeaponDamages)(const SetUnitWeaponDamagesQuery* query, SetUnitWeaponDamagesResult* result);
	void (*ForceUnitCollisionUpdate)(const ForceUnitCollisionUpdateQuery* query, ForceUnitCollisionUpdateResult* result);
	void (*SetUnitHeading)(const SetUnitHeadingQuery* query, SetUnitHeadingResult* result);
	void (*SetUnitHeadingAndUpDir)(const SetUnitHeadingAndUpDirQuery* query, SetUnitHeadingAndUpDirResult* result);
	void (*AddObjectDecal)(const AddObjectDecalQuery* query, AddObjectDecalResult* result);
	void (*RemoveObjectDecal)(const RemoveObjectDecalQuery* query, RemoveObjectDecalResult* result);
	void (*SetUnitBuildeeRadius)(const SetUnitBuildeeRadiusQuery* query, SetUnitBuildeeRadiusResult* result);
	void (*SetUnitSensorRadius)(const SetUnitSensorRadiusQuery* query, SetUnitSensorRadiusResult* result);
	void (*SetUnitHarvestStorage)(const SetUnitHarvestStorageQuery* query, SetUnitHarvestStorageResult* result);
	void (*SetUnitBuildParams)(const SetUnitBuildParamsQuery* query, SetUnitBuildParamsResult* result);
	void (*SetUnitLosMask)(const SetUnitLosMaskQuery* query, SetUnitLosMaskResult* result);
	void (*SetUnitLosState)(const SetUnitLosStateQuery* query, SetUnitLosStateResult* result);
	void (*SetUnitStorage)(const SetUnitStorageQuery* query, SetUnitStorageResult* result);
	void (*SetUnitTooltip)(const SetUnitTooltipQuery* query, SetUnitTooltipResult* result);
	void (*SetFactoryBuggerOff)(const SetFactoryBuggerOffQuery* query, SetFactoryBuggerOffResult* result);
	void (*BuggerOff)(const BuggerOffQuery* query, BuggerOffResult* result);
	void (*AddUnitSeismicPing)(const AddUnitSeismicPingQuery* query, AddUnitSeismicPingResult* result);
	void (*AddUnitResource)(const AddUnitResourceQuery* query, AddUnitResourceResult* result);
	void (*UseUnitResource)(const UseUnitResourceQuery* query, UseUnitResourceResult* result);
	void (*SetUnitPieceVisible)(const SetUnitPieceVisibleQuery* query, SetUnitPieceVisibleResult* result);
	void (*SetUnitPieceParent)(const SetUnitPieceParentQuery* query, SetUnitPieceParentResult* result);
	void (*SetUnitPieceMatrix)(const SetUnitPieceMatrixQuery* query, SetUnitPieceMatrixResult* result);
	void (*SetUnitNanoPieces)(const SetUnitNanoPiecesQuery* query, SetUnitNanoPiecesResult* result);
};

// ============================================================================
// Feature Control
// ============================================================================

// Queries - Feature Control
struct CreateFeatureQuery { DefRef featureDef; Float3 pos; int32_t facing; int32_t teamID; int32_t featureID; };
struct CreateFeatureResult { const Error* error; int32_t featureID; };

struct DestroyFeatureQuery { int32_t featureID; };
struct DestroyFeatureResult { const Error* error; bool success; };

struct TransferFeatureQuery { int32_t featureID; int32_t newTeamID; };
struct TransferFeatureResult { const Error* error; bool success; };

struct SetFeatureHealthQuery { int32_t featureID; float health; bool checkDestruction; };
struct SetFeatureHealthResult { const Error* error; bool success; };

struct SetFeaturePositionQuery { int32_t featureID; Float3 pos; bool snapToGround; };
struct SetFeaturePositionResult { const Error* error; bool success; };

struct SetFeatureDirectionQuery { int32_t featureID; Float3 frontDir; Float3 rightDir; };
struct SetFeatureDirectionResult { const Error* error; bool success; };

struct SetFeatureVelocityQuery { int32_t featureID; Float3 velocity; };
struct SetFeatureVelocityResult { const Error* error; bool success; };

struct SetFeatureResourcesQuery { int32_t featureID; float metal; float energy; float reclaimTime; float reclaimLeft; float featureDefMetal; float featureDefEnergy; };
struct SetFeatureResourcesResult { const Error* error; bool success; };

struct AddFeatureDamageQuery { int32_t featureID; float damage; float paralyzeTime; int32_t weaponDefID; int32_t attackerID; Float3 impulse; };
struct AddFeatureDamageResult { const Error* error; bool success; };

struct SetFeatureBlockingQuery { int32_t featureID; bool blocking; bool solidObjects; bool projectiles; bool quadMapRays; bool crushable; bool blockEnemyPushing; bool blockHeightChanges; };
struct SetFeatureBlockingResult { const Error* error; bool success; };

struct SetFeatureMassQuery { int32_t featureID; float mass; };
struct SetFeatureMassResult { const Error* error; bool success; };

struct SetFeatureMaxHealthQuery { int32_t featureID; float maxHealth; };
struct SetFeatureMaxHealthResult { const Error* error; bool success; };

struct SetFeatureReclaimQuery { int32_t featureID; float reclaimLeft; };
struct SetFeatureReclaimResult { const Error* error; bool success; };

struct SetFeatureResurrectQuery { int32_t featureID; DefRef unitDef; int32_t facing; float progress; };
struct SetFeatureResurrectResult { const Error* error; bool success; };

struct SetFeaturePhysicsQuery { int32_t featureID; Float3 pos; Float3 velocity; Float3 rotation; Float3 drag; };
struct SetFeaturePhysicsResult { const Error* error; bool success; };

struct SetFeatureMoveCtrlQuery { int32_t featureID; bool enable; Float3 velocityOrMask; Float3 accelerationOrImpulseMask; Float3 movementMask; };
struct SetFeatureMoveCtrlResult { const Error* error; bool success; };

struct SetFeatureHeadingAndUpDirQuery { int32_t featureID; int32_t heading; Float3 upDir; };
struct SetFeatureHeadingAndUpDirResult { const Error* error; bool success; };

struct SetFeatureRotationQuery { int32_t featureID; Float3 rotation; };
struct SetFeatureRotationResult { const Error* error; bool success; };

struct SetFeatureAlwaysVisibleQuery { int32_t featureID; bool alwaysVisible; };
struct SetFeatureAlwaysVisibleResult { const Error* error; bool success; };

struct SetFeatureUseAirLosQuery { int32_t featureID; bool useAirLos; };
struct SetFeatureUseAirLosResult { const Error* error; bool success; };

struct SetFeatureNoSelectQuery { int32_t featureID; bool noSelect; };
struct SetFeatureNoSelectResult { const Error* error; bool success; };

struct SetFeatureMidAndAimPosQuery { int32_t featureID; Float3 midPos; Float3 aimPos; bool setRelative; };
struct SetFeatureMidAndAimPosResult { const Error* error; bool success; };

struct SetFeatureRadiusAndHeightQuery { int32_t featureID; float radius; float height; };
struct SetFeatureRadiusAndHeightResult { const Error* error; bool success; };

struct SetFeatureCollisionVolumeDataQuery {
	int32_t featureID;
	Float3 scales;
	Float3 offsets;
	int32_t volumeType;
	int32_t testType;
	int32_t primaryAxis;
};
struct SetFeatureCollisionVolumeDataResult { const Error* error; bool success; };

struct SetFeatureSelectionVolumeDataQuery {
	int32_t featureID;
	Float3 scales;
	Float3 offsets;
	int32_t volumeType;
	int32_t primaryAxis;
	bool useContHitTest;
};
struct SetFeatureSelectionVolumeDataResult { const Error* error; bool success; };

// Set feature fire time query
struct SetFeatureFireTimeQuery { int32_t featureID; float fireTime; };  // fireTime in seconds
struct SetFeatureFireTimeResult { const Error* error; bool success; };

// Set feature smoke time query
struct SetFeatureSmokeTimeQuery { int32_t featureID; float smokeTime; };  // smokeTime in seconds
struct SetFeatureSmokeTimeResult { const Error* error; bool success; };

// Create unit wreck query
struct CreateUnitWreckQuery {
	int32_t unitID;
	int32_t wreckLevel;       // 1-indexed wreck level (default 1)
	bool doSmoke;             // emit smoke (default true)
};
struct CreateUnitWreckResult { const Error* error; int32_t featureID; };

// Create feature wreck query
struct CreateFeatureWreckQuery {
	int32_t featureID;
	int32_t wreckLevel;       // 1-indexed wreck level (default 1)
	bool doSmoke;             // emit smoke (default false)
};
struct CreateFeatureWreckResult { const Error* error; int32_t featureID; };

// Set feature piece visible query
struct SetFeaturePieceVisibleQuery {
	int32_t featureID;
	int32_t pieceIndex;      // 0-indexed
	bool visible;
};
struct SetFeaturePieceVisibleResult { const Error* error; bool success; };

struct SetFeaturePieceMatrixQuery {
	int32_t featureID;
	int32_t pieceIndex;      // 0-indexed
	float matrix[16];        // 4x4 transformation matrix (column-major)
};
struct SetFeaturePieceMatrixResult { const Error* error; bool blockScriptAnims; };

// Set feature piece collision volume data query
struct SetFeaturePieceCollisionVolumeDataQuery {
	int32_t featureID;
	int32_t pieceIndex;      // 0-indexed
	bool enable;
	Float3 scales;
	Float3 offsets;
	int32_t volumeType;
	int32_t primaryAxis;
};
struct SetFeaturePieceCollisionVolumeDataResult { const Error* error; bool success; };

struct FeatureControlApi {
	void (*CreateFeature)(const CreateFeatureQuery* query, CreateFeatureResult* result);
	void (*DestroyFeature)(const DestroyFeatureQuery* query, DestroyFeatureResult* result);
	void (*TransferFeature)(const TransferFeatureQuery* query, TransferFeatureResult* result);
	void (*SetFeatureHealth)(const SetFeatureHealthQuery* query, SetFeatureHealthResult* result);
	void (*SetFeaturePosition)(const SetFeaturePositionQuery* query, SetFeaturePositionResult* result);
	void (*SetFeatureDirection)(const SetFeatureDirectionQuery* query, SetFeatureDirectionResult* result);
	void (*SetFeatureVelocity)(const SetFeatureVelocityQuery* query, SetFeatureVelocityResult* result);
	void (*SetFeatureResources)(const SetFeatureResourcesQuery* query, SetFeatureResourcesResult* result);
	void (*AddFeatureDamage)(const AddFeatureDamageQuery* query, AddFeatureDamageResult* result);
	void (*SetFeatureBlocking)(const SetFeatureBlockingQuery* query, SetFeatureBlockingResult* result);
	void (*SetFeatureMass)(const SetFeatureMassQuery* query, SetFeatureMassResult* result);
	void (*SetFeatureMaxHealth)(const SetFeatureMaxHealthQuery* query, SetFeatureMaxHealthResult* result);
	void (*SetFeatureReclaim)(const SetFeatureReclaimQuery* query, SetFeatureReclaimResult* result);
	void (*SetFeatureResurrect)(const SetFeatureResurrectQuery* query, SetFeatureResurrectResult* result);
	void (*SetFeaturePhysics)(const SetFeaturePhysicsQuery* query, SetFeaturePhysicsResult* result);
	void (*SetFeatureMoveCtrl)(const SetFeatureMoveCtrlQuery* query, SetFeatureMoveCtrlResult* result);
	void (*SetFeatureHeadingAndUpDir)(const SetFeatureHeadingAndUpDirQuery* query, SetFeatureHeadingAndUpDirResult* result);
	void (*SetFeatureRotation)(const SetFeatureRotationQuery* query, SetFeatureRotationResult* result);
	void (*SetFeatureAlwaysVisible)(const SetFeatureAlwaysVisibleQuery* query, SetFeatureAlwaysVisibleResult* result);
	void (*SetFeatureUseAirLos)(const SetFeatureUseAirLosQuery* query, SetFeatureUseAirLosResult* result);
	void (*SetFeatureNoSelect)(const SetFeatureNoSelectQuery* query, SetFeatureNoSelectResult* result);
	void (*SetFeatureMidAndAimPos)(const SetFeatureMidAndAimPosQuery* query, SetFeatureMidAndAimPosResult* result);
	void (*SetFeatureRadiusAndHeight)(const SetFeatureRadiusAndHeightQuery* query, SetFeatureRadiusAndHeightResult* result);
	void (*SetFeatureCollisionVolumeData)(const SetFeatureCollisionVolumeDataQuery* query, SetFeatureCollisionVolumeDataResult* result);
	void (*SetFeatureSelectionVolumeData)(const SetFeatureSelectionVolumeDataQuery* query, SetFeatureSelectionVolumeDataResult* result);
	void (*SetFeatureFireTime)(const SetFeatureFireTimeQuery* query, SetFeatureFireTimeResult* result);
	void (*SetFeatureSmokeTime)(const SetFeatureSmokeTimeQuery* query, SetFeatureSmokeTimeResult* result);
	void (*CreateUnitWreck)(const CreateUnitWreckQuery* query, CreateUnitWreckResult* result);
	void (*CreateFeatureWreck)(const CreateFeatureWreckQuery* query, CreateFeatureWreckResult* result);
	void (*SetFeaturePieceVisible)(const SetFeaturePieceVisibleQuery* query, SetFeaturePieceVisibleResult* result);
	void (*SetFeaturePieceMatrix)(const SetFeaturePieceMatrixQuery* query, SetFeaturePieceMatrixResult* result);
	void (*SetFeaturePieceCollisionVolumeData)(const SetFeaturePieceCollisionVolumeDataQuery* query, SetFeaturePieceCollisionVolumeDataResult* result);
};

// ============================================================================
// Terrain Control
// ============================================================================

// Queries - Terrain Control
struct AddHeightMapQuery { float x; float z; float height; };
struct AddHeightMapResult { const Error* error; bool success; };

struct SetHeightMapQuery { float x; float z; float height; float terraform; };
struct SetHeightMapResult { const Error* error; bool success; };

struct RevertHeightMapQuery { float x1; float z1; float x2; float z2; float origFactor; };
struct RevertHeightMapResult { const Error* error; bool success; };

struct AddSmoothMeshQuery { float x; float z; float height; };
struct AddSmoothMeshResult { const Error* error; bool success; };

struct SetSmoothMeshQuery { float x; float z; float height; float terraform; };
struct SetSmoothMeshResult { const Error* error; bool success; };

struct RevertSmoothMeshQuery { float x1; float z1; float x2; float z2; float origFactor; };
struct RevertSmoothMeshResult { const Error* error; bool success; };

struct SetMapSquareTerrainTypeQuery { int32_t x; int32_t z; int32_t terrainType; };
struct SetMapSquareTerrainTypeResult { const Error* error; bool success; };

struct SetTerrainTypeDataQuery { int32_t typeIndex; float tankSpeed; float kbotSpeed; float hoverSpeed; float shipSpeed; float hardness; bool receiveTracks; const char* name; };
struct SetTerrainTypeDataResult { const Error* error; bool success; };

struct SetTidalQuery { float tidal; };
struct SetTidalResult { const Error* error; bool success; };

struct SetWindQuery { float minWind; float maxWind; };
struct SetWindResult { const Error* error; bool success; };

// Grass queries
struct AddGrassQuery { float x; float z; };
struct AddGrassResult { const Error* error; bool success; };

struct RemoveGrassQuery { float x; float z; };
struct RemoveGrassResult { const Error* error; bool success; };

// Advanced height map queries
struct AdjustHeightMapQuery { float x1; float z1; float x2; float z2; float height; };
struct AdjustHeightMapResult { const Error* error; bool success; };

struct LevelHeightMapQuery { float x1; float z1; float x2; float z2; float height; };
struct LevelHeightMapResult { const Error* error; bool success; };

// Original height map queries
struct AddOriginalHeightMapQuery { float x; float z; float height; };
struct AddOriginalHeightMapResult { const Error* error; bool success; };

struct SetOriginalHeightMapQuery { float x; float z; float height; float factor; };
struct SetOriginalHeightMapResult { const Error* error; bool success; };

struct RevertOriginalHeightMapQuery { float x1; float z1; float x2; float z2; float origFactor; };
struct RevertOriginalHeightMapResult { const Error* error; bool success; };

struct AdjustOriginalHeightMapQuery { float x1; float z1; float x2; float z2; float height; };
struct AdjustOriginalHeightMapResult { const Error* error; bool success; };

struct LevelOriginalHeightMapQuery { float x1; float z1; float x2; float z2; float height; };
struct LevelOriginalHeightMapResult { const Error* error; bool success; };

// Smooth mesh advanced queries
struct AdjustSmoothMeshQuery { float x1; float z1; float x2; float z2; float height; };
struct AdjustSmoothMeshResult { const Error* error; bool success; };

struct LevelSmoothMeshQuery { float x1; float z1; float x2; float z2; float height; };
struct LevelSmoothMeshResult { const Error* error; bool success; };

struct RebuildSmoothMeshQuery { uint8_t _unused; };
struct RebuildSmoothMeshResult { const Error* error; bool success; };

// Native counterparts of Lua's terrain edit callback wrappers. The engine sets
// callback-local edit state, invokes callback(userData), then finalizes the
// edited data before returning.
struct SetHeightMapFuncQuery { NativeCallback callback; void* userData; };
struct SetHeightMapFuncResult { const Error* error; bool success; };

struct SetOriginalHeightMapFuncQuery { NativeCallback callback; void* userData; };
struct SetOriginalHeightMapFuncResult { const Error* error; bool success; };

struct SetSmoothMeshFuncQuery { NativeCallback callback; void* userData; };
struct SetSmoothMeshFuncResult { const Error* error; bool success; };

struct TerrainControlApi {
	void (*AddHeightMap)(const AddHeightMapQuery* query, AddHeightMapResult* result);
	void (*SetHeightMap)(const SetHeightMapQuery* query, SetHeightMapResult* result);
	void (*RevertHeightMap)(const RevertHeightMapQuery* query, RevertHeightMapResult* result);
	void (*AddSmoothMesh)(const AddSmoothMeshQuery* query, AddSmoothMeshResult* result);
	void (*SetSmoothMesh)(const SetSmoothMeshQuery* query, SetSmoothMeshResult* result);
	void (*RevertSmoothMesh)(const RevertSmoothMeshQuery* query, RevertSmoothMeshResult* result);
	void (*SetMapSquareTerrainType)(const SetMapSquareTerrainTypeQuery* query, SetMapSquareTerrainTypeResult* result);
	void (*SetTerrainTypeData)(const SetTerrainTypeDataQuery* query, SetTerrainTypeDataResult* result);
	void (*SetTidal)(const SetTidalQuery* query, SetTidalResult* result);
	void (*SetWind)(const SetWindQuery* query, SetWindResult* result);
	void (*AddGrass)(const AddGrassQuery* query, AddGrassResult* result);
	void (*RemoveGrass)(const RemoveGrassQuery* query, RemoveGrassResult* result);
	void (*AdjustHeightMap)(const AdjustHeightMapQuery* query, AdjustHeightMapResult* result);
	void (*LevelHeightMap)(const LevelHeightMapQuery* query, LevelHeightMapResult* result);
	void (*AddOriginalHeightMap)(const AddOriginalHeightMapQuery* query, AddOriginalHeightMapResult* result);
	void (*SetOriginalHeightMap)(const SetOriginalHeightMapQuery* query, SetOriginalHeightMapResult* result);
	void (*RevertOriginalHeightMap)(const RevertOriginalHeightMapQuery* query, RevertOriginalHeightMapResult* result);
	void (*AdjustOriginalHeightMap)(const AdjustOriginalHeightMapQuery* query, AdjustOriginalHeightMapResult* result);
	void (*LevelOriginalHeightMap)(const LevelOriginalHeightMapQuery* query, LevelOriginalHeightMapResult* result);
	void (*AdjustSmoothMesh)(const AdjustSmoothMeshQuery* query, AdjustSmoothMeshResult* result);
	void (*LevelSmoothMesh)(const LevelSmoothMeshQuery* query, LevelSmoothMeshResult* result);
	void (*RebuildSmoothMesh)(const RebuildSmoothMeshQuery* query, RebuildSmoothMeshResult* result);
	void (*SetHeightMapFunc)(const SetHeightMapFuncQuery* query, SetHeightMapFuncResult* result);
	void (*SetOriginalHeightMapFunc)(const SetOriginalHeightMapFuncQuery* query, SetOriginalHeightMapFuncResult* result);
	void (*SetSmoothMeshFunc)(const SetSmoothMeshFuncQuery* query, SetSmoothMeshFuncResult* result);
};

// ============================================================================
// Projectile Control
// ============================================================================

// Queries - Projectile Control
struct SpawnProjectileQuery { int32_t weaponDefID; NativeProjectileParams projectileParams; };
struct SpawnProjectileResult { const Error* error; int32_t projectileID; };

struct DeleteProjectileQuery { int32_t projectileID; };
struct DeleteProjectileResult { const Error* error; bool success; };

struct SetProjectilePositionQuery { int32_t projectileID; Float3 pos; };
struct SetProjectilePositionResult { const Error* error; bool success; };

struct SetProjectileVelocityQuery { int32_t projectileID; Float3 velocity; };
struct SetProjectileVelocityResult { const Error* error; bool success; };

struct SetProjectileGravityQuery { int32_t projectileID; float gravity; };
struct SetProjectileGravityResult { const Error* error; bool success; };

struct SetProjectileTargetQuery { int32_t projectileID; ProjectileTargetRef target; };
struct SetProjectileTargetResult { const Error* error; bool success; };

struct SetProjectileDamagesQuery { int32_t projectileID; int32_t unused; const char* damageKey; float damageValue; };
struct SetProjectileDamagesResult { const Error* error; bool success; };

struct SetProjectileTimeToLiveQuery { int32_t projectileID; int32_t timeToLive; };
struct SetProjectileTimeToLiveResult { const Error* error; bool success; };

struct SetProjectileIsInterceptedQuery { int32_t projectileID; bool intercepted; };
struct SetProjectileIsInterceptedResult { const Error* error; bool success; };

struct SetProjectileCollisionQuery { int32_t projectileID; };
struct SetProjectileCollisionResult { const Error* error; bool success; };

struct SetProjectileCEGQuery { int32_t projectileID; const char* cegName; };
struct SetProjectileCEGResult { const Error* error; bool success; };

struct SetProjectileAlwaysVisibleQuery { int32_t projectileID; bool alwaysVisible; };
struct SetProjectileAlwaysVisibleResult { const Error* error; bool success; };

struct SetProjectileUseAirLosQuery { int32_t projectileID; bool useAirLos; };
struct SetProjectileUseAirLosResult { const Error* error; bool success; };

struct SetProjectileMoveControlQuery { int32_t projectileID; bool enable; };
struct SetProjectileMoveControlResult { const Error* error; bool success; };

struct SetProjectileIgnoreTrackingErrorQuery { int32_t projectileID; bool ignore; };
struct SetProjectileIgnoreTrackingErrorResult { const Error* error; bool success; };

struct SetProjectileSpinAngleQuery { int32_t projectileID; float angle; };
struct SetProjectileSpinAngleResult { const Error* error; bool success; };

struct SetProjectileSpinSpeedQuery { int32_t projectileID; float speed; };
struct SetProjectileSpinSpeedResult { const Error* error; bool success; };

struct SetProjectileSpinVecQuery { int32_t projectileID; Float3 spinVec; };
struct SetProjectileSpinVecResult { const Error* error; bool success; };

// Set piece projectile params query
struct SetPieceProjectileParamsQuery {
	int32_t projectileID;
	int32_t explFlags;
	float spinAngle;
	float spinSpeed;
	Float3 spinVec;
};
struct SetPieceProjectileParamsResult { const Error* error; bool success; };

struct ProjectileControlApi {
	void (*SpawnProjectile)(const SpawnProjectileQuery* query, SpawnProjectileResult* result);
	void (*DeleteProjectile)(const DeleteProjectileQuery* query, DeleteProjectileResult* result);
	void (*SetProjectilePosition)(const SetProjectilePositionQuery* query, SetProjectilePositionResult* result);
	void (*SetProjectileVelocity)(const SetProjectileVelocityQuery* query, SetProjectileVelocityResult* result);
	void (*SetProjectileGravity)(const SetProjectileGravityQuery* query, SetProjectileGravityResult* result);
	void (*SetProjectileTarget)(const SetProjectileTargetQuery* query, SetProjectileTargetResult* result);
	void (*SetProjectileDamages)(const SetProjectileDamagesQuery* query, SetProjectileDamagesResult* result);
	void (*SetProjectileTimeToLive)(const SetProjectileTimeToLiveQuery* query, SetProjectileTimeToLiveResult* result);
	void (*SetProjectileIsIntercepted)(const SetProjectileIsInterceptedQuery* query, SetProjectileIsInterceptedResult* result);
	void (*SetProjectileCollision)(const SetProjectileCollisionQuery* query, SetProjectileCollisionResult* result);
	void (*SetProjectileCEG)(const SetProjectileCEGQuery* query, SetProjectileCEGResult* result);
	void (*SetProjectileAlwaysVisible)(const SetProjectileAlwaysVisibleQuery* query, SetProjectileAlwaysVisibleResult* result);
	void (*SetProjectileUseAirLos)(const SetProjectileUseAirLosQuery* query, SetProjectileUseAirLosResult* result);
	void (*SetProjectileMoveControl)(const SetProjectileMoveControlQuery* query, SetProjectileMoveControlResult* result);
	void (*SetProjectileIgnoreTrackingError)(const SetProjectileIgnoreTrackingErrorQuery* query, SetProjectileIgnoreTrackingErrorResult* result);
	void (*SetProjectileSpinAngle)(const SetProjectileSpinAngleQuery* query, SetProjectileSpinAngleResult* result);
	void (*SetProjectileSpinSpeed)(const SetProjectileSpinSpeedQuery* query, SetProjectileSpinSpeedResult* result);
	void (*SetProjectileSpinVec)(const SetProjectileSpinVecQuery* query, SetProjectileSpinVecResult* result);
	void (*SetPieceProjectileParams)(const SetPieceProjectileParamsQuery* query, SetPieceProjectileParamsResult* result);
};

// ============================================================================
// Effects Control
// ============================================================================

// Spawn explosion query
struct SpawnExplosionQuery {
	Float3 pos;
	Float3 dir;
	NativeExplosionParams explosionParams;
};
struct SpawnExplosionResult { const Error* error; bool success; };

// Spawn CEG (Custom Explosion Generator) query
struct SpawnCEGQuery {
	DefRef ceg;
	Float3 pos;
	Float3 dir;
	float radius;
	float damage;
	float dmgMod;
};
struct SpawnCEGResult { const Error* error; bool success; int32_t cegID; };

// Spawn SFX query
struct SpawnSFXQuery {
	int32_t unitID;
	int32_t sfxID;
	Float3 pos;
	Float3 dir;
	float radius;
	float damage;
	bool absolute;              // absolute or unit-relative coordinates
};
struct SpawnSFXResult { const Error* error; bool success; };

struct EffectsControlApi {
	void (*SpawnExplosion)(const SpawnExplosionQuery* query, SpawnExplosionResult* result);
	void (*SpawnCEG)(const SpawnCEGQuery* query, SpawnCEGResult* result);
	void (*SpawnSFX)(const SpawnSFXQuery* query, SpawnSFXResult* result);
};

// ============================================================================
// Game Config Control
// ============================================================================

// Set no pause query (disables game pausing)
struct SetNoPauseQuery { bool noPause; };
struct SetNoPauseResult { const Error* error; bool success; };

struct SetCheatingEnabledQuery { bool enabled; };
struct SetCheatingEnabledResult { const Error* error; bool success; };

struct SetGodModeQuery { bool controlAllies; bool controlEnemies; };
struct SetGodModeResult { const Error* error; bool success; };

// Set experience grade query (controls UnitExperience callin frequency)
struct SetExperienceGradeQuery {
	float expGrade;
	float expPowerScale;      // only applies if cheats enabled, use -1 to not set
	float expHealthScale;     // only applies if cheats enabled, use -1 to not set
	float expReloadScale;     // only applies if cheats enabled, use -1 to not set
};
struct SetExperienceGradeResult { const Error* error; bool success; };

// Set radar error params query
struct SetRadarErrorParamsQuery {
	int32_t allyTeamID;
	float allyTeamErrorSize;
	float baseErrorSize;      // use -1 to not set
	float baseErrorMult;      // use -1 to not set
};
struct SetRadarErrorParamsResult { const Error* error; bool success; };

// Set square building mask query
struct SetSquareBuildingMaskQuery {
	int32_t x;
	int32_t z;
	uint16_t mask;
};
struct SetSquareBuildingMaskResult { const Error* error; bool success; };

struct GameConfigApi {
	void (*SetNoPause)(const SetNoPauseQuery* query, SetNoPauseResult* result);
	void (*SetCheatingEnabled)(const SetCheatingEnabledQuery* query, SetCheatingEnabledResult* result);
	void (*SetGodMode)(const SetGodModeQuery* query, SetGodModeResult* result);
	void (*SetExperienceGrade)(const SetExperienceGradeQuery* query, SetExperienceGradeResult* result);
	void (*SetRadarErrorParams)(const SetRadarErrorParamsQuery* query, SetRadarErrorParamsResult* result);
	void (*SetSquareBuildingMask)(const SetSquareBuildingMaskQuery* query, SetSquareBuildingMaskResult* result);
};

// ============================================================================
// Combined API
// ============================================================================

struct SyncedCtrlApi {
	const TeamControlApi* team;
	const UnitControlApi* unit;
	const FeatureControlApi* feature;
	const TerrainControlApi* terrain;
	const ProjectileControlApi* projectile;
	const EffectsControlApi* effects;
	const GameConfigApi* gameConfig;
	const COBScriptApi* cobScript;
};

extern const SyncedCtrlApi SYNCED_CTRL_API;

#ifdef __cplusplus
}
#endif
