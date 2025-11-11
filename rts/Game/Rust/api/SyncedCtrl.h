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

struct AllyTeamStartBoxSet {
	int32_t allyTeamID;
	float minX;
	float minZ;
	float maxX;
	float maxZ;
};

struct TeamResourceSet {
	int32_t teamID;
	const char* resourceType;  // "m" or "e"
	float amount;
};

struct TeamShareLevelSet {
	int32_t teamID;
	const char* resourceType;
	float shareLevel;
};

struct TeamShareRequest {
	int32_t teamID;
	int32_t targetTeamID;
	const char* resourceType;
	float amount;
};

struct TeamControlApi {
	// Alliance
	BoolResult (*SetAlly)(int32_t firstAllyTeamID, int32_t secondAllyTeamID, bool allied);
	BoolResult (*SetAllyTeamStartBox)(AllyTeamStartBoxSet request);

	// Team management
	BoolResult (*KillTeam)(int32_t teamID);
	BoolResult (*AssignPlayerToTeam)(int32_t playerID, int32_t teamID);
	BoolResult (*GameOver)(const int32_t* winningAllyTeams, uint32_t count);
	BoolResult (*SetGlobalLos)(int32_t allyTeamID, bool enabled);

	// Resources
	BoolResult (*AddTeamResource)(TeamResourceSet request);
	BoolResult (*UseTeamResource)(TeamResourceSet request);
	BoolResult (*SetTeamResource)(TeamResourceSet request);
	BoolResult (*SetTeamShareLevel)(TeamShareLevelSet request);
	BoolResult (*ShareTeamResource)(TeamShareRequest request);
};

// ============================================================================
// Unit Control
// ============================================================================

struct UnitCreateRequest {
	int32_t unitDefID;
	Float3 pos;
	int32_t facing;  // 0-3
	int32_t teamID;
	bool build;  // false for instant creation
	int32_t builderID;  // -1 for no builder
};

struct UnitOrderRequest {
	int32_t unitID;
	int32_t cmdID;
	float* params;
	uint32_t paramCount;
	uint32_t options;  // Bitfield
};

struct UnitHealthSet {
	int32_t unitID;
	float health;
	bool relative;  // true to add/subtract, false to set
};

struct UnitExperienceSet {
	int32_t unitID;
	float experience;
	bool add;  // true to add, false to set
};

struct UnitPhysicsSet {
	int32_t unitID;
	Float3 pos;
	Float3 velocity;
	Float3 rotation;  // Euler angles or quaternion
	bool setPos;
	bool setVel;
	bool setRot;
};

struct UnitControlApi {
	// Creation/destruction
	Int32Result (*CreateUnit)(UnitCreateRequest request);
	BoolResult (*DestroyUnit)(int32_t unitID, bool selfd, bool reclaimed);
	BoolResult (*TransferUnit)(int32_t unitID, int32_t newTeamID, bool given);

	// Orders
	BoolResult (*GiveOrderToUnit)(UnitOrderRequest order);
	BoolResult (*GiveOrderToUnitArray)(const int32_t* unitIDs, uint32_t count, UnitOrderRequest order);
	BoolResult (*UnitFinishCommand)(int32_t unitID);

	// Health and state
	BoolResult (*SetUnitHealth)(UnitHealthSet request);
	BoolResult (*SetUnitMaxHealth)(int32_t unitID, float maxHealth);
	BoolResult (*SetUnitExperience)(UnitExperienceSet request);
	BoolResult (*AddUnitExperience)(int32_t unitID, float experience);
	BoolResult (*SetUnitNeutral)(int32_t unitID, bool neutral);

	// Resources
	BoolResult (*SetUnitResourcing)(int32_t unitID, const char* type, float amount);
	BoolResult (*SetUnitMetalExtraction)(int32_t unitID, float amount);

	// Physics
	BoolResult (*SetUnitPosition)(int32_t unitID, Float3 pos, bool relative);
	BoolResult (*SetUnitVelocity)(int32_t unitID, Float3 velocity);
	BoolResult (*SetUnitRotation)(int32_t unitID, Float3 rotation);
	BoolResult (*SetUnitPhysics)(UnitPhysicsSet request);

	// Damage and impulse
	BoolResult (*AddUnitDamage)(int32_t unitID, float damage, int32_t weaponDefID, int32_t attackerID);
	BoolResult (*AddUnitImpulse)(int32_t unitID, Float3 impulse);
};

// ============================================================================
// Feature Control
// ============================================================================

struct FeatureCreateRequest {
	int32_t featureDefID;
	Float3 pos;
	int32_t facing;
	int32_t teamID;
	int32_t allyTeamID;
};

struct FeatureControlApi {
	// Creation/destruction
	Int32Result (*CreateFeature)(FeatureCreateRequest request);
	BoolResult (*DestroyFeature)(int32_t featureID);
	BoolResult (*TransferFeature)(int32_t featureID, int32_t newTeamID);

	// Properties
	BoolResult (*SetFeatureHealth)(int32_t featureID, float health);
	BoolResult (*SetFeaturePosition)(int32_t featureID, Float3 pos);
	BoolResult (*SetFeatureDirection)(int32_t featureID, Float3 dir);
	BoolResult (*SetFeatureVelocity)(int32_t featureID, Float3 velocity);
	BoolResult (*SetFeatureResources)(int32_t featureID, float metal, float energy, float reclaimTime);
};

// ============================================================================
// Terrain Control
// ============================================================================

struct TerrainModifyRequest {
	Float3 pos1;
	Float3 pos2;  // For rectangular operations
	float amount;
	bool hasPos2;
};

struct TerrainControlApi {
	// Height map
	BoolResult (*AddHeightMap)(Float3 pos, float height);
	BoolResult (*SetHeightMap)(Float3 pos, float height);
	BoolResult (*RevertHeightMap)(Float3 pos1, Float3 pos2, float origFactor);

	// Smooth mesh
	BoolResult (*AddSmoothMesh)(Float3 pos1, Float3 pos2, float height);
	BoolResult (*SetSmoothMesh)(Float3 pos1, Float3 pos2, float height);
	BoolResult (*RevertSmoothMesh)(Float3 pos1, Float3 pos2, float origFactor);

	// Terrain type
	BoolResult (*SetMapSquareTerrainType)(int32_t x, int32_t z, int32_t terrainType);
	BoolResult (*SetTerrainTypeData)(int32_t typeIndex, const char* name, float hardness, float tankSpeed, float kbotSpeed);

	// Environmental
	BoolResult (*SetTidal)(float tidal);
	BoolResult (*SetWind)(float minWind, float maxWind);
};

// ============================================================================
// Projectile Control
// ============================================================================

struct ProjectileSpawnRequest {
	int32_t weaponDefID;
	Float3 pos;
	Float3 velocity;
	Float3 target;
	int32_t ownerID;
	int32_t teamID;
	float ttl;
	float gravity;
};

struct ProjectileControlApi {
	// Creation/destruction
	Int32Result (*SpawnProjectile)(ProjectileSpawnRequest request);
	BoolResult (*DeleteProjectile)(int32_t projectileID);

	// Properties
	BoolResult (*SetProjectilePosition)(int32_t projectileID, Float3 pos);
	BoolResult (*SetProjectileVelocity)(int32_t projectileID, Float3 velocity);
	BoolResult (*SetProjectileGravity)(int32_t projectileID, float gravity);
	BoolResult (*SetProjectileTarget)(int32_t projectileID, int32_t targetID, Float3 targetPos, bool isGroundTarget);
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
};

extern const SyncedCtrlApi SYNCED_CTRL_API;

#ifdef __cplusplus
}
#endif
