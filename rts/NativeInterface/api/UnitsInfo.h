#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Units Info API
// @see rts/Lua/LuaSyncedRead.cpp
//
// Unit property queries (health, position, state, sensors, etc.)
// ============================================================================

// Unit basic info
struct UnitBasicInfo {
	int32_t unitID;
	int32_t unitDefID;
	int32_t teamID;
	int32_t allyTeamID;
	bool isNeutral;
	const char* tooltip;
};

// Unit health
struct UnitHealth {
	float health;
	float maxHealth;
	float paralyzeDamage;
	float captureProgress;
	float buildProgress;
};

// Unit costs
struct UnitCosts {
	float metalCost;
	float energyCost;
	float buildTime;
};

// Unit resources
struct UnitResources {
	float metalMake;
	float metalUse;
	float energyMake;
	float energyUse;
	float metalIncome;
	float energyIncome;
};

// Unit storage
struct UnitStorage {
	float metalStorage;
	float energyStorage;
};

// Unit states
struct UnitStates {
	int32_t fireState;     // 0=hold, 1=return fire, 2=fire at will
	int32_t moveState;     // 0=hold, 1=maneuver, 2=roam
	bool repeat;
	bool cloak;
	bool active;
	bool trajectory;
	bool autoLand;
};

// Unit stockpile
struct UnitStockpile {
	uint32_t stockpile;
	uint32_t stockpileQueueSize;
};

// Unit sensor radius
struct UnitSensorRadius {
	float los;
	float airLos;
	float radar;
	float sonar;
	float seismic;
	float radarJammer;
	float sonarJammer;
};

// Unit position error params (fog of war error)
struct UnitPosErrorParams {
	Float3 posError;
	Float3 nextPosError;
	float errorScale;
	float errorMult;
};

// Unit vectors (directional)
struct UnitVectors {
	Float3 frontDir;
	Float3 upDir;
	Float3 rightDir;
};

// Unit rotation (matrix representation)
struct UnitRotation {
	Float3 col1;  // Right vector
	Float3 col2;  // Up vector
	Float3 col3;  // Front vector
};

// Build params
struct UnitBuildParams {
	float buildDistance;
	float buildSpeed;
	float repairSpeed;
	float reclaimSpeed;
	float resurrectSpeed;
	float captureSpeed;
	float terraformSpeed;
};

// Shield state
struct UnitShieldState {
	bool shieldEnabled;
	float shieldPower;
	float shieldAlpha;
};

// Flanking bonus
struct UnitFlanking {
	uint32_t flankingMode;
	float minDamage;
	float maxDamage;
};

// Travel and fuel
struct UnitTravel {
	float travelPeriod;
	float travelTime;
};

struct UnitFuel {
	float fuel;
	float maxFuel;
};

// Last attacker
struct UnitLastAttacker {
	int32_t attackerID;
	int32_t attackerDefID;
	int32_t attackerTeam;
};

// LOS state
struct UnitLosState {
	bool los;
	bool prevLos;
	bool radar;
	bool sonar;
	bool seismic;
	bool jammer;
	bool typed;
};

// Blocking state
struct UnitBlockingState {
	bool isBlocking;
	bool isSolidObjectCollidable;
	bool isProjectileCollidable;
	bool isRaySegmentCollidable;
	bool crushable;
	bool blockEnemyPushing;
	bool blockHeightChanges;
};

// Armored state
struct UnitArmoredState {
	bool armored;
	float armorMultiple;
};

// Queries
struct GetUnitTooltipQuery { int32_t unitID; };
struct GetUnitTooltipResult { const Error* error; const char* tooltip; };

struct GetUnitDefIDQuery { int32_t unitID; };
struct GetUnitDefIDResult { const Error* error; int32_t unitDefID; };

struct GetUnitMoveDefIDQuery { int32_t unitID; };
struct GetUnitMoveDefIDResult { const Error* error; int32_t moveDefID; };

struct GetUnitTeamQuery { int32_t unitID; };
struct GetUnitTeamResult { const Error* error; int32_t teamID; };

struct GetUnitAllyTeamQuery { int32_t unitID; };
struct GetUnitAllyTeamResult { const Error* error; int32_t allyTeamID; };

struct GetUnitNeutralQuery { int32_t unitID; };
struct GetUnitNeutralResult { const Error* error; bool neutral; };

struct GetUnitHealthQuery { int32_t unitID; };
struct GetUnitHealthResult { const Error* error; UnitHealth health; };

struct GetUnitIsDeadQuery { int32_t unitID; };
struct GetUnitIsDeadResult { const Error* error; bool isDead; };

struct GetUnitIsStunnedQuery { int32_t unitID; };
struct GetUnitIsStunnedResult { const Error* error; bool isStunned; };

struct GetUnitIsBeingBuiltQuery { int32_t unitID; };
struct GetUnitIsBeingBuiltResult { const Error* error; bool isBeingBuilt; };

struct GetUnitCostsQuery { int32_t unitID; };
struct GetUnitCostsResult { const Error* error; UnitCosts costs; };

struct GetUnitCostTableQuery { int32_t unitID; };
struct GetUnitCostTableResult { const Error* error; UnitCosts costs; };

struct GetUnitResourcesQuery { int32_t unitID; };
struct GetUnitResourcesResult { const Error* error; UnitResources resources; };

struct GetUnitStorageQuery { int32_t unitID; };
struct GetUnitStorageResult { const Error* error; UnitStorage storage; };

struct GetUnitMetalExtractionQuery { int32_t unitID; };
struct GetUnitMetalExtractionResult { const Error* error; float metalExtraction; };

struct GetUnitExperienceQuery { int32_t unitID; };
struct GetUnitExperienceResult { const Error* error; float experience; };

struct GetUnitStatesQuery { int32_t unitID; };
struct GetUnitStatesResult { const Error* error; UnitStates states; };

struct GetUnitArmoredQuery { int32_t unitID; };
struct GetUnitArmoredResult { const Error* error; UnitArmoredState armoredState; };

struct GetUnitIsActiveQuery { int32_t unitID; };
struct GetUnitIsActiveResult { const Error* error; bool isActive; };

struct GetUnitIsCloakedQuery { int32_t unitID; };
struct GetUnitIsCloakedResult { const Error* error; bool isCloaked; };

struct GetUnitSeismicSignatureQuery { int32_t unitID; };
struct GetUnitSeismicSignatureResult { const Error* error; float seismicSignature; };

struct GetUnitSensorRadiusQuery { int32_t unitID; };
struct GetUnitSensorRadiusResult { const Error* error; UnitSensorRadius radius; };

struct GetUnitPosErrorParamsQuery { int32_t unitID; int32_t allyTeamID; };
struct GetUnitPosErrorParamsResult { const Error* error; UnitPosErrorParams params; };

struct GetUnitHeightQuery { int32_t unitID; };
struct GetUnitHeightResult { const Error* error; float height; };

struct GetUnitRadiusQuery { int32_t unitID; };
struct GetUnitRadiusResult { const Error* error; float radius; };

struct GetUnitBuildeeRadiusQuery { int32_t unitID; };
struct GetUnitBuildeeRadiusResult { const Error* error; float radius; };

struct GetUnitMassQuery { int32_t unitID; };
struct GetUnitMassResult { const Error* error; float mass; };

struct GetUnitPositionQuery { int32_t unitID; };
struct GetUnitPositionResult { const Error* error; Float3 position; };

struct GetUnitBasePositionQuery { int32_t unitID; };
struct GetUnitBasePositionResult { const Error* error; Float3 position; };

struct GetUnitVectorsQuery { int32_t unitID; };
struct GetUnitVectorsResult { const Error* error; UnitVectors vectors; };

struct GetUnitRotationQuery { int32_t unitID; };
struct GetUnitRotationResult { const Error* error; UnitRotation rotation; };

struct GetUnitDirectionQuery { int32_t unitID; };
struct GetUnitDirectionResult { const Error* error; Float3 direction; };

struct GetUnitHeadingQuery { int32_t unitID; };
struct GetUnitHeadingResult { const Error* error; int32_t heading; };

struct GetUnitVelocityQuery { int32_t unitID; };
struct GetUnitVelocityResult { const Error* error; Float3 velocity; };

struct GetUnitBuildFacingQuery { int32_t unitID; };
struct GetUnitBuildFacingResult { const Error* error; int32_t facing; };

struct GetUnitIsBuildingQuery { int32_t unitID; };
struct GetUnitIsBuildingResult { const Error* error; int32_t buildeeID; };

struct GetUnitWorkerTaskQuery { int32_t unitID; };
struct GetUnitWorkerTaskResult { const Error* error; const char* task; };

struct GetUnitEffectiveBuildRangeQuery { int32_t unitID; };
struct GetUnitEffectiveBuildRangeResult { const Error* error; float range; };

struct GetUnitCurrentBuildPowerQuery { int32_t unitID; };
struct GetUnitCurrentBuildPowerResult { const Error* error; float buildPower; };

struct GetUnitBuildParamsQuery { int32_t unitID; };
struct GetUnitBuildParamsResult { const Error* error; UnitBuildParams params; };

struct GetUnitInBuildStanceQuery { int32_t unitID; };
struct GetUnitInBuildStanceResult { const Error* error; bool inBuildStance; };

struct GetUnitNanoPiecesQuery { int32_t unitID; };
struct GetUnitNanoPiecesResult { const Error* error; int32_t* pieces; uint32_t count; };

struct GetUnitTransporterQuery { int32_t unitID; };
struct GetUnitTransporterResult { const Error* error; int32_t transporterID; };

struct GetUnitIsTransportingQuery { int32_t unitID; };
struct GetUnitIsTransportingResult { const Error* error; bool isTransporting; };

struct GetUnitStockpileQuery { int32_t unitID; };
struct GetUnitStockpileResult { const Error* error; UnitStockpile stockpile; };

struct GetUnitSelfDTimeQuery { int32_t unitID; };
struct GetUnitSelfDTimeResult { const Error* error; float selfDTime; };

struct GetUnitShieldStateQuery { int32_t unitID; int32_t weaponNum; };
struct GetUnitShieldStateResult { const Error* error; UnitShieldState shield; bool hasShield; };

struct GetUnitFlankingQuery { int32_t unitID; };
struct GetUnitFlankingResult { const Error* error; UnitFlanking flanking; };

struct GetUnitTravelQuery { int32_t unitID; };
struct GetUnitTravelResult { const Error* error; UnitTravel travel; };

struct GetUnitFuelQuery { int32_t unitID; };
struct GetUnitFuelResult { const Error* error; UnitFuel fuel; };

struct GetUnitLastAttackerQuery { int32_t unitID; };
struct GetUnitLastAttackerResult { const Error* error; UnitLastAttacker attacker; bool hasAttacker; };

struct GetUnitLastAttackedPieceQuery { int32_t unitID; };
struct GetUnitLastAttackedPieceResult { const Error* error; int32_t pieceNum; };

struct GetUnitLosStateQuery { int32_t unitID; int32_t allyTeamID; };
struct GetUnitLosStateResult { const Error* error; UnitLosState losState; };

struct GetUnitCollisionVolumeDataQuery { int32_t unitID; };
struct GetUnitCollisionVolumeDataResult { const Error* error; CollisionVolumeData volume; };

struct GetUnitPieceCollisionVolumeDataQuery { int32_t unitID; int32_t pieceNum; };
struct GetUnitPieceCollisionVolumeDataResult { const Error* error; CollisionVolumeData volume; };

struct GetUnitBlockingQuery { int32_t unitID; };
struct GetUnitBlockingResult { const Error* error; UnitBlockingState blockingState; };

struct GetUnitHarvestStorageQuery { int32_t unitID; };
struct GetUnitHarvestStorageResult { const Error* error; float harvestStorage; };

struct ClearUnitsPreviousDrawFlagQuery { uint8_t _unused; };
struct ClearUnitsPreviousDrawFlagResult { const Error* error; bool success; };

// API structure
struct UnitsInfoApi {
	void (*GetUnitTooltip)(const GetUnitTooltipQuery* query, GetUnitTooltipResult* result);
	void (*GetUnitDefID)(const GetUnitDefIDQuery* query, GetUnitDefIDResult* result);
	void (*GetUnitMoveDefID)(const GetUnitMoveDefIDQuery* query, GetUnitMoveDefIDResult* result);
	void (*GetUnitTeam)(const GetUnitTeamQuery* query, GetUnitTeamResult* result);
	void (*GetUnitAllyTeam)(const GetUnitAllyTeamQuery* query, GetUnitAllyTeamResult* result);
	void (*GetUnitNeutral)(const GetUnitNeutralQuery* query, GetUnitNeutralResult* result);
	void (*GetUnitHealth)(const GetUnitHealthQuery* query, GetUnitHealthResult* result);
	void (*GetUnitIsDead)(const GetUnitIsDeadQuery* query, GetUnitIsDeadResult* result);
	void (*GetUnitIsStunned)(const GetUnitIsStunnedQuery* query, GetUnitIsStunnedResult* result);
	void (*GetUnitIsBeingBuilt)(const GetUnitIsBeingBuiltQuery* query, GetUnitIsBeingBuiltResult* result);
	void (*GetUnitCosts)(const GetUnitCostsQuery* query, GetUnitCostsResult* result);
	void (*GetUnitCostTable)(const GetUnitCostTableQuery* query, GetUnitCostTableResult* result);
	void (*GetUnitResources)(const GetUnitResourcesQuery* query, GetUnitResourcesResult* result);
	void (*GetUnitStorage)(const GetUnitStorageQuery* query, GetUnitStorageResult* result);
	void (*GetUnitMetalExtraction)(const GetUnitMetalExtractionQuery* query, GetUnitMetalExtractionResult* result);
	void (*GetUnitExperience)(const GetUnitExperienceQuery* query, GetUnitExperienceResult* result);
	void (*GetUnitStates)(const GetUnitStatesQuery* query, GetUnitStatesResult* result);
	void (*GetUnitArmored)(const GetUnitArmoredQuery* query, GetUnitArmoredResult* result);
	void (*GetUnitIsActive)(const GetUnitIsActiveQuery* query, GetUnitIsActiveResult* result);
	void (*GetUnitIsCloaked)(const GetUnitIsCloakedQuery* query, GetUnitIsCloakedResult* result);
	void (*GetUnitSeismicSignature)(const GetUnitSeismicSignatureQuery* query, GetUnitSeismicSignatureResult* result);
	void (*GetUnitSensorRadius)(const GetUnitSensorRadiusQuery* query, GetUnitSensorRadiusResult* result);
	void (*GetUnitPosErrorParams)(const GetUnitPosErrorParamsQuery* query, GetUnitPosErrorParamsResult* result);
	void (*GetUnitHeight)(const GetUnitHeightQuery* query, GetUnitHeightResult* result);
	void (*GetUnitRadius)(const GetUnitRadiusQuery* query, GetUnitRadiusResult* result);
	void (*GetUnitBuildeeRadius)(const GetUnitBuildeeRadiusQuery* query, GetUnitBuildeeRadiusResult* result);
	void (*GetUnitMass)(const GetUnitMassQuery* query, GetUnitMassResult* result);
	void (*GetUnitPosition)(const GetUnitPositionQuery* query, GetUnitPositionResult* result);
	void (*GetUnitBasePosition)(const GetUnitBasePositionQuery* query, GetUnitBasePositionResult* result);
	void (*GetUnitVectors)(const GetUnitVectorsQuery* query, GetUnitVectorsResult* result);
	void (*GetUnitRotation)(const GetUnitRotationQuery* query, GetUnitRotationResult* result);
	void (*GetUnitDirection)(const GetUnitDirectionQuery* query, GetUnitDirectionResult* result);
	void (*GetUnitHeading)(const GetUnitHeadingQuery* query, GetUnitHeadingResult* result);
	void (*GetUnitVelocity)(const GetUnitVelocityQuery* query, GetUnitVelocityResult* result);
	void (*GetUnitBuildFacing)(const GetUnitBuildFacingQuery* query, GetUnitBuildFacingResult* result);
	void (*GetUnitIsBuilding)(const GetUnitIsBuildingQuery* query, GetUnitIsBuildingResult* result);
	void (*GetUnitWorkerTask)(const GetUnitWorkerTaskQuery* query, GetUnitWorkerTaskResult* result);
	void (*GetUnitEffectiveBuildRange)(const GetUnitEffectiveBuildRangeQuery* query, GetUnitEffectiveBuildRangeResult* result);
	void (*GetUnitCurrentBuildPower)(const GetUnitCurrentBuildPowerQuery* query, GetUnitCurrentBuildPowerResult* result);
	void (*GetUnitBuildParams)(const GetUnitBuildParamsQuery* query, GetUnitBuildParamsResult* result);
	void (*GetUnitInBuildStance)(const GetUnitInBuildStanceQuery* query, GetUnitInBuildStanceResult* result);
	void (*GetUnitNanoPieces)(const GetUnitNanoPiecesQuery* query, GetUnitNanoPiecesResult* result);
	void (*GetUnitTransporter)(const GetUnitTransporterQuery* query, GetUnitTransporterResult* result);
	void (*GetUnitIsTransporting)(const GetUnitIsTransportingQuery* query, GetUnitIsTransportingResult* result);
	void (*GetUnitStockpile)(const GetUnitStockpileQuery* query, GetUnitStockpileResult* result);
	void (*GetUnitSelfDTime)(const GetUnitSelfDTimeQuery* query, GetUnitSelfDTimeResult* result);
	void (*GetUnitShieldState)(const GetUnitShieldStateQuery* query, GetUnitShieldStateResult* result);
	void (*GetUnitFlanking)(const GetUnitFlankingQuery* query, GetUnitFlankingResult* result);
	void (*GetUnitTravel)(const GetUnitTravelQuery* query, GetUnitTravelResult* result);
	void (*GetUnitFuel)(const GetUnitFuelQuery* query, GetUnitFuelResult* result);
	void (*GetUnitLastAttacker)(const GetUnitLastAttackerQuery* query, GetUnitLastAttackerResult* result);
	void (*GetUnitLastAttackedPiece)(const GetUnitLastAttackedPieceQuery* query, GetUnitLastAttackedPieceResult* result);
	void (*GetUnitLosState)(const GetUnitLosStateQuery* query, GetUnitLosStateResult* result);
	void (*GetUnitCollisionVolumeData)(const GetUnitCollisionVolumeDataQuery* query, GetUnitCollisionVolumeDataResult* result);
	void (*GetUnitPieceCollisionVolumeData)(const GetUnitPieceCollisionVolumeDataQuery* query, GetUnitPieceCollisionVolumeDataResult* result);
	void (*GetUnitBlocking)(const GetUnitBlockingQuery* query, GetUnitBlockingResult* result);
	void (*GetUnitHarvestStorage)(const GetUnitHarvestStorageQuery* query, GetUnitHarvestStorageResult* result);
	void (*ClearUnitsPreviousDrawFlag)(const ClearUnitsPreviousDrawFlagQuery* query, ClearUnitsPreviousDrawFlagResult* result);
};

extern const UnitsInfoApi UNITS_INFO_API;

#ifdef __cplusplus
}
#endif
