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

struct UnitBasicInfoResult {
	const Error* error;
	UnitBasicInfo info;
};

// Unit health
struct UnitHealth {
	float health;
	float maxHealth;
	float paralyzeDamage;
	float captureProgress;
	float buildProgress;
};

struct UnitHealthResult {
	const Error* error;
	UnitHealth health;
};

// Unit costs
struct UnitCosts {
	float metalCost;
	float energyCost;
	float buildTime;
};

struct UnitCostsResult {
	const Error* error;
	UnitCosts costs;
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

struct UnitResourcesResult {
	const Error* error;
	UnitResources resources;
};

// Unit storage
struct UnitStorage {
	float metalStorage;
	float energyStorage;
};

struct UnitStorageResult {
	const Error* error;
	UnitStorage storage;
};

// Unit states
struct UnitStates {
	bool fireState;        // 0=hold, 1=return fire, 2=fire at will
	bool moveState;        // 0=hold, 1=maneuver, 2=roam
	bool repeat;
	bool cloak;
	bool active;
	bool trajectory;
	bool autoLand;
};

struct UnitStatesResult {
	const Error* error;
	UnitStates states;
};

// Unit stockpile
struct UnitStockpile {
	uint32_t stockpile;
	uint32_t stockpileQueueSize;
};

struct UnitStockpileResult {
	const Error* error;
	UnitStockpile stockpile;
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

struct UnitSensorRadiusResult {
	const Error* error;
	UnitSensorRadius radius;
};

// Unit position error params (fog of war error)
struct UnitPosErrorParams {
	Float3 posError;
	Float3 nextPosError;
	float errorScale;
	float errorMult;
};

struct UnitPosErrorParamsResult {
	const Error* error;
	UnitPosErrorParams params;
};

// Unit vectors (directional)
struct UnitVectors {
	Float3 frontDir;
	Float3 upDir;
	Float3 rightDir;
};

struct UnitVectorsResult {
	const Error* error;
	UnitVectors vectors;
};

// Unit rotation (matrix representation)
struct UnitRotation {
	Float3 col1;  // Right vector
	Float3 col2;  // Up vector
	Float3 col3;  // Front vector
};

struct UnitRotationResult {
	const Error* error;
	UnitRotation rotation;
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

struct UnitBuildParamsResult {
	const Error* error;
	UnitBuildParams params;
};

// Shield state
struct UnitShieldState {
	bool shieldEnabled;
	float shieldPower;
	float shieldAlpha;
};

struct UnitShieldStateResult {
	const Error* error;
	UnitShieldState shield;
	bool hasShield;
};

// Flanking bonus
struct UnitFlanking {
	uint32_t flankingMode;
	float minDamage;
	float maxDamage;
};

struct UnitFlankingResult {
	const Error* error;
	UnitFlanking flanking;
};

// Travel and fuel
struct UnitTravel {
	float travelPeriod;
	float travelTime;
};

struct UnitTravelResult {
	const Error* error;
	UnitTravel travel;
};

struct UnitFuel {
	float fuel;
	float maxFuel;
};

struct UnitFuelResult {
	const Error* error;
	UnitFuel fuel;
};

// Last attacker
struct UnitLastAttacker {
	int32_t attackerID;
	int32_t attackerDefID;
	int32_t attackerTeam;
};

struct UnitLastAttackerResult {
	const Error* error;
	UnitLastAttacker attacker;
	bool hasAttacker;
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

struct UnitLosStateResult {
	const Error* error;
	UnitLosState losState;
};

// Collision volume
struct CollisionVolumeData {
	float scaleX;
	float scaleY;
	float scaleZ;
	float offsetX;
	float offsetY;
	float offsetZ;
	int32_t volumeType;  // 0=ellipsoid, 1=cylinder, 2=box
	int32_t testType;    // Collision test type
	int32_t primaryAxis; // For cylinders
	bool disabled;
};

struct CollisionVolumeDataResult {
	const Error* error;
	CollisionVolumeData volume;
};

// API structure (part 1 - basic info)
struct UnitsInfoApi {
	// Basic info
	StringResult (*GetUnitTooltip)(int32_t unitID);
	Int32Result (*GetUnitDefID)(int32_t unitID);
	Int32Result (*GetUnitTeam)(int32_t unitID);
	Int32Result (*GetUnitAllyTeam)(int32_t unitID);
	BoolResult (*GetUnitNeutral)(int32_t unitID);

	// Health and state
	UnitHealthResult (*GetUnitHealth)(int32_t unitID);
	BoolResult (*GetUnitIsDead)(int32_t unitID);
	BoolResult (*GetUnitIsStunned)(int32_t unitID);
	BoolResult (*GetUnitIsBeingBuilt)(int32_t unitID);

	// Costs
	UnitCostsResult (*GetUnitCosts)(int32_t unitID);
	UnitCostsResult (*GetUnitCostTable)(int32_t unitID);

	// Resources
	UnitResourcesResult (*GetUnitResources)(int32_t unitID);
	UnitStorageResult (*GetUnitStorage)(int32_t unitID);
	FloatResult (*GetUnitMetalExtraction)(int32_t unitID);

	// Experience
	FloatResult (*GetUnitExperience)(int32_t unitID);

	// States
	UnitStatesResult (*GetUnitStates)(int32_t unitID);
	BoolResult (*GetUnitArmored)(int32_t unitID, float* armorMultiple);
	BoolResult (*GetUnitIsActive)(int32_t unitID);
	BoolResult (*GetUnitIsCloaked)(int32_t unitID);

	// Sensors
	FloatResult (*GetUnitSeismicSignature)(int32_t unitID);
	UnitSensorRadiusResult (*GetUnitSensorRadius)(int32_t unitID);
	UnitPosErrorParamsResult (*GetUnitPosErrorParams)(int32_t unitID, int32_t allyTeamID);

	// Physical properties
	FloatResult (*GetUnitHeight)(int32_t unitID);
	FloatResult (*GetUnitRadius)(int32_t unitID);
	FloatResult (*GetUnitBuildeeRadius)(int32_t unitID);
	FloatResult (*GetUnitMass)(int32_t unitID);

	// Position and orientation
	Float3Result (*GetUnitPosition)(int32_t unitID);
	Float3Result (*GetUnitBasePosition)(int32_t unitID);
	UnitVectorsResult (*GetUnitVectors)(int32_t unitID);
	UnitRotationResult (*GetUnitRotation)(int32_t unitID);
	Float3Result (*GetUnitDirection)(int32_t unitID);
	Int32Result (*GetUnitHeading)(int32_t unitID);
	Float3Result (*GetUnitVelocity)(int32_t unitID);
	Int32Result (*GetUnitBuildFacing)(int32_t unitID);

	// Building
	Int32Result (*GetUnitIsBuilding)(int32_t unitID);  // Returns unitID being built
	StringResult (*GetUnitWorkerTask)(int32_t unitID);  // "build", "reclaim", etc.
	FloatResult (*GetUnitEffectiveBuildRange)(int32_t unitID);
	FloatResult (*GetUnitCurrentBuildPower)(int32_t unitID);
	UnitBuildParamsResult (*GetUnitBuildParams)(int32_t unitID);
	BoolResult (*GetUnitInBuildStance)(int32_t unitID);
	Int32Array (*GetUnitNanoPieces)(int32_t unitID);

	// Transport
	Int32Result (*GetUnitTransporter)(int32_t unitID);
	BoolResult (*GetUnitIsTransporting)(int32_t unitID);

	// Special states
	UnitStockpileResult (*GetUnitStockpile)(int32_t unitID);
	FloatResult (*GetUnitSelfDTime)(int32_t unitID);
	UnitShieldStateResult (*GetUnitShieldState)(int32_t unitID, int32_t weaponNum);
	UnitFlankingResult (*GetUnitFlanking)(int32_t unitID);
	UnitTravelResult (*GetUnitTravel)(int32_t unitID);
	UnitFuelResult (*GetUnitFuel)(int32_t unitID);

	// Combat
	UnitLastAttackerResult (*GetUnitLastAttacker)(int32_t unitID);
	Int32Result (*GetUnitLastAttackedPiece)(int32_t unitID);

	// LOS
	UnitLosStateResult (*GetUnitLosState)(int32_t unitID, int32_t allyTeamID);

	// Collision volumes
	CollisionVolumeDataResult (*GetUnitCollisionVolumeData)(int32_t unitID);
	CollisionVolumeDataResult (*GetUnitPieceCollisionVolumeData)(int32_t unitID, int32_t pieceNum);

	// Blocking state
	BoolResult (*GetUnitBlocking)(int32_t unitID, bool* isBlocking, bool* isSolidObjectCollidable, bool* isProjectileCollidable, bool* isRaySegmentCollidable, bool* crushable, bool* blockEnemyPushing, bool* blockHeightChanges);

	// Harvest storage
	FloatResult (*GetUnitHarvestStorage)(int32_t unitID);
};

extern const UnitsInfoApi UNITS_INFO_API;

#ifdef __cplusplus
}
#endif
