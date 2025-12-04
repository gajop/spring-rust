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
};

// ============================================================================
// Unit Control
// ============================================================================

// Queries - Unit Control
struct CreateUnitQuery { int32_t unitDefID; Float3 pos; int32_t facing; int32_t teamID; bool build; int32_t builderID; };
struct CreateUnitResult { const Error* error; int32_t unitID; };

struct DestroyUnitQuery { int32_t unitID; bool selfd; bool reclaimed; };
struct DestroyUnitResult { const Error* error; bool success; };

struct TransferUnitQuery { int32_t unitID; int32_t newTeamID; bool given; };
struct TransferUnitResult { const Error* error; bool success; };

struct GiveOrderToUnitQuery { int32_t unitID; int32_t cmdID; float* params; uint32_t paramCount; uint32_t options; };
struct GiveOrderToUnitResult { const Error* error; bool success; };

struct GiveOrderToUnitArrayQuery { const int32_t* unitIDs; uint32_t count; int32_t cmdID; float* params; uint32_t paramCount; uint32_t options; };
struct GiveOrderToUnitArrayResult { const Error* error; bool success; };

struct UnitFinishCommandQuery { int32_t unitID; };
struct UnitFinishCommandResult { const Error* error; bool success; };

struct SetUnitHealthQuery { int32_t unitID; float health; bool relative; };
struct SetUnitHealthResult { const Error* error; bool success; };

struct SetUnitMaxHealthQuery { int32_t unitID; float maxHealth; };
struct SetUnitMaxHealthResult { const Error* error; bool success; };

struct SetUnitExperienceQuery { int32_t unitID; float experience; bool add; };
struct SetUnitExperienceResult { const Error* error; bool success; };

struct AddUnitExperienceQuery { int32_t unitID; float experience; };
struct AddUnitExperienceResult { const Error* error; bool success; };

struct SetUnitNeutralQuery { int32_t unitID; bool neutral; };
struct SetUnitNeutralResult { const Error* error; bool success; };

struct SetUnitResourcingQuery { int32_t unitID; const char* type; float amount; };
struct SetUnitResourcingResult { const Error* error; bool success; };

struct SetUnitMetalExtractionQuery { int32_t unitID; float amount; };
struct SetUnitMetalExtractionResult { const Error* error; bool success; };

struct SetUnitPositionQuery { int32_t unitID; Float3 pos; bool relative; };
struct SetUnitPositionResult { const Error* error; bool success; };

struct SetUnitVelocityQuery { int32_t unitID; Float3 velocity; };
struct SetUnitVelocityResult { const Error* error; bool success; };

struct SetUnitRotationQuery { int32_t unitID; Float3 rotation; };
struct SetUnitRotationResult { const Error* error; bool success; };

struct SetUnitPhysicsQuery { int32_t unitID; Float3 pos; Float3 velocity; Float3 rotation; bool setPos; bool setVel; bool setRot; };
struct SetUnitPhysicsResult { const Error* error; bool success; };

struct AddUnitDamageQuery { int32_t unitID; float damage; int32_t weaponDefID; int32_t attackerID; };
struct AddUnitDamageResult { const Error* error; bool success; };

struct AddUnitImpulseQuery { int32_t unitID; Float3 impulse; };
struct AddUnitImpulseResult { const Error* error; bool success; };

struct SetUnitCloakQuery { int32_t unitID; bool wantCloak; float decloakDistance; bool useDefaultDecloakDistance; };
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
struct GetUnitPhysicalStateResult { const Error* error; uint8_t physicalState; };

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
struct SetUnitCostsQuery { int32_t unitID; float buildTime; float metalCost; float energyCost; };
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
	int32_t primaryAxis;     // COLVOL_AXIS_*
};
struct SetUnitCollisionVolumeDataResult { const Error* error; bool success; };

// Unit selection volume queries
struct SetUnitSelectionVolumeDataQuery {
	int32_t unitID;
	Float3 scales;
	Float3 offsets;
	int32_t volumeType;
	int32_t primaryAxis;
	bool useContHitTest;
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
	int32_t targetID;           // target unit ID, -1 for ground target or clear
	Float3 targetPos;           // ground target position (if targetID == -1)
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
	int32_t mode;               // flanking bonus mode
	Float3 dir;                 // flanking direction
	float moveFactor;           // mobility add factor
	float minDamage;            // min damage multiplier
	float maxDamage;            // max damage multiplier
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

struct UnitControlApi {
	void (*CreateUnit)(const CreateUnitQuery* query, CreateUnitResult* result);
	void (*DestroyUnit)(const DestroyUnitQuery* query, DestroyUnitResult* result);
	void (*TransferUnit)(const TransferUnitQuery* query, TransferUnitResult* result);
	void (*GiveOrderToUnit)(const GiveOrderToUnitQuery* query, GiveOrderToUnitResult* result);
	void (*GiveOrderToUnitArray)(const GiveOrderToUnitArrayQuery* query, GiveOrderToUnitArrayResult* result);
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
};

// ============================================================================
// Feature Control
// ============================================================================

// Queries - Feature Control
struct CreateFeatureQuery { int32_t featureDefID; Float3 pos; int32_t facing; int32_t teamID; int32_t allyTeamID; };
struct CreateFeatureResult { const Error* error; int32_t featureID; };

struct DestroyFeatureQuery { int32_t featureID; };
struct DestroyFeatureResult { const Error* error; bool success; };

struct TransferFeatureQuery { int32_t featureID; int32_t newTeamID; };
struct TransferFeatureResult { const Error* error; bool success; };

struct SetFeatureHealthQuery { int32_t featureID; float health; };
struct SetFeatureHealthResult { const Error* error; bool success; };

struct SetFeaturePositionQuery { int32_t featureID; Float3 pos; };
struct SetFeaturePositionResult { const Error* error; bool success; };

struct SetFeatureDirectionQuery { int32_t featureID; Float3 dir; };
struct SetFeatureDirectionResult { const Error* error; bool success; };

struct SetFeatureVelocityQuery { int32_t featureID; Float3 velocity; };
struct SetFeatureVelocityResult { const Error* error; bool success; };

struct SetFeatureResourcesQuery { int32_t featureID; float metal; float energy; float reclaimTime; };
struct SetFeatureResourcesResult { const Error* error; bool success; };

struct AddFeatureDamageQuery { int32_t featureID; float damage; int32_t weaponDefID; int32_t attackerID; Float3 impulse; };
struct AddFeatureDamageResult { const Error* error; bool success; };

struct SetFeatureBlockingQuery { int32_t featureID; bool blocking; bool solidObjects; bool projectiles; bool quadMapRays; bool crushable; bool blockEnemyPushing; bool blockHeightChanges; };
struct SetFeatureBlockingResult { const Error* error; bool success; };

struct SetFeatureMassQuery { int32_t featureID; float mass; };
struct SetFeatureMassResult { const Error* error; bool success; };

struct SetFeatureMaxHealthQuery { int32_t featureID; float maxHealth; };
struct SetFeatureMaxHealthResult { const Error* error; bool success; };

struct SetFeatureReclaimQuery { int32_t featureID; float reclaimLeft; };
struct SetFeatureReclaimResult { const Error* error; bool success; };

struct SetFeatureResurrectQuery { int32_t featureID; int32_t unitDefID; int32_t facing; };
struct SetFeatureResurrectResult { const Error* error; bool success; };

struct SetFeaturePhysicsQuery { int32_t featureID; Float3 pos; Float3 velocity; Float3 rotation; bool setPos; bool setVel; bool setRot; };
struct SetFeaturePhysicsResult { const Error* error; bool success; };

struct SetFeatureMoveCtrlQuery { int32_t featureID; bool enable; };
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
};

// ============================================================================
// Terrain Control
// ============================================================================

// Queries - Terrain Control
struct AddHeightMapQuery { Float3 pos; float height; };
struct AddHeightMapResult { const Error* error; bool success; };

struct SetHeightMapQuery { Float3 pos; float height; };
struct SetHeightMapResult { const Error* error; bool success; };

struct RevertHeightMapQuery { Float3 pos1; Float3 pos2; float origFactor; };
struct RevertHeightMapResult { const Error* error; bool success; };

struct AddSmoothMeshQuery { Float3 pos1; Float3 pos2; float height; };
struct AddSmoothMeshResult { const Error* error; bool success; };

struct SetSmoothMeshQuery { Float3 pos1; Float3 pos2; float height; };
struct SetSmoothMeshResult { const Error* error; bool success; };

struct RevertSmoothMeshQuery { Float3 pos1; Float3 pos2; float origFactor; };
struct RevertSmoothMeshResult { const Error* error; bool success; };

struct SetMapSquareTerrainTypeQuery { int32_t x; int32_t z; int32_t terrainType; };
struct SetMapSquareTerrainTypeResult { const Error* error; bool success; };

struct SetTerrainTypeDataQuery { int32_t typeIndex; const char* name; float hardness; float tankSpeed; float kbotSpeed; };
struct SetTerrainTypeDataResult { const Error* error; bool success; };

struct SetTidalQuery { float tidal; };
struct SetTidalResult { const Error* error; bool success; };

struct SetWindQuery { float minWind; float maxWind; };
struct SetWindResult { const Error* error; bool success; };

// Grass queries
struct AddGrassQuery { float x; float z; uint8_t grassValue; };
struct AddGrassResult { const Error* error; bool success; };

struct RemoveGrassQuery { float x; float z; };
struct RemoveGrassResult { const Error* error; bool success; };

// Advanced height map queries
struct AdjustHeightMapQuery { float x1; float z1; float x2; float z2; float height; };
struct AdjustHeightMapResult { const Error* error; bool success; };

struct LevelHeightMapQuery { float x1; float z1; float x2; float z2; float height; };
struct LevelHeightMapResult { const Error* error; bool success; };

// Original height map queries
struct AddOriginalHeightMapQuery { float x1; float z1; float x2; float z2; float height; };
struct AddOriginalHeightMapResult { const Error* error; bool success; };

struct SetOriginalHeightMapQuery { float x1; float z1; float x2; float z2; float height; };
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

struct RebuildSmoothMeshQuery { float x1; float z1; float x2; float z2; };
struct RebuildSmoothMeshResult { const Error* error; bool success; };

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
};

// ============================================================================
// Projectile Control
// ============================================================================

// Queries - Projectile Control
struct SpawnProjectileQuery { int32_t weaponDefID; Float3 pos; Float3 velocity; Float3 target; int32_t ownerID; int32_t teamID; float ttl; float gravity; };
struct SpawnProjectileResult { const Error* error; int32_t projectileID; };

struct DeleteProjectileQuery { int32_t projectileID; };
struct DeleteProjectileResult { const Error* error; bool success; };

struct SetProjectilePositionQuery { int32_t projectileID; Float3 pos; };
struct SetProjectilePositionResult { const Error* error; bool success; };

struct SetProjectileVelocityQuery { int32_t projectileID; Float3 velocity; };
struct SetProjectileVelocityResult { const Error* error; bool success; };

struct SetProjectileGravityQuery { int32_t projectileID; float gravity; };
struct SetProjectileGravityResult { const Error* error; bool success; };

struct SetProjectileTargetQuery { int32_t projectileID; int32_t targetID; Float3 targetPos; bool isGroundTarget; };
struct SetProjectileTargetResult { const Error* error; bool success; };

struct SetProjectileDamagesQuery { int32_t projectileID; const char* damageKey; float damageValue; };
struct SetProjectileDamagesResult { const Error* error; bool success; };

struct SetProjectileTimeToLiveQuery { int32_t projectileID; int32_t timeToLive; };
struct SetProjectileTimeToLiveResult { const Error* error; bool success; };

struct SetProjectileIsInterceptedQuery { int32_t projectileID; bool intercepted; };
struct SetProjectileIsInterceptedResult { const Error* error; bool success; };

struct SetProjectileCollisionQuery { int32_t projectileID; bool collide; };
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
};

// ============================================================================
// Effects Control
// ============================================================================

// Spawn explosion query
struct SpawnExplosionQuery {
	Float3 pos;
	Float3 dir;
	float damages;              // base damage
	float craterAreaOfEffect;
	float damageAreaOfEffect;
	float edgeEffectiveness;
	float explosionSpeed;
	float gfxMod;
	bool impactOnly;
	bool ignoreOwner;
	bool damageGround;
	int32_t weaponDefID;        // -1 for none
	int32_t ownerID;            // -1 for none
	int32_t projectileID;       // -1 for none
};
struct SpawnExplosionResult { const Error* error; bool success; };

// Spawn CEG (Custom Explosion Generator) query
struct SpawnCEGQuery {
	const char* cegName;        // CEG name or NULL to use cegID
	int32_t cegID;              // CEG ID, used if cegName is NULL
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
// Combined API
// ============================================================================

struct SyncedCtrlApi {
	const TeamControlApi* team;
	const UnitControlApi* unit;
	const FeatureControlApi* feature;
	const TerrainControlApi* terrain;
	const ProjectileControlApi* projectile;
	const EffectsControlApi* effects;
};

extern const SyncedCtrlApi SYNCED_CTRL_API;

#ifdef __cplusplus
}
#endif
