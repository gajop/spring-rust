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

struct FeatureControlApi {
	void (*CreateFeature)(const CreateFeatureQuery* query, CreateFeatureResult* result);
	void (*DestroyFeature)(const DestroyFeatureQuery* query, DestroyFeatureResult* result);
	void (*TransferFeature)(const TransferFeatureQuery* query, TransferFeatureResult* result);
	void (*SetFeatureHealth)(const SetFeatureHealthQuery* query, SetFeatureHealthResult* result);
	void (*SetFeaturePosition)(const SetFeaturePositionQuery* query, SetFeaturePositionResult* result);
	void (*SetFeatureDirection)(const SetFeatureDirectionQuery* query, SetFeatureDirectionResult* result);
	void (*SetFeatureVelocity)(const SetFeatureVelocityQuery* query, SetFeatureVelocityResult* result);
	void (*SetFeatureResources)(const SetFeatureResourcesQuery* query, SetFeatureResourcesResult* result);
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

struct ProjectileControlApi {
	void (*SpawnProjectile)(const SpawnProjectileQuery* query, SpawnProjectileResult* result);
	void (*DeleteProjectile)(const DeleteProjectileQuery* query, DeleteProjectileResult* result);
	void (*SetProjectilePosition)(const SetProjectilePositionQuery* query, SetProjectilePositionResult* result);
	void (*SetProjectileVelocity)(const SetProjectileVelocityQuery* query, SetProjectileVelocityResult* result);
	void (*SetProjectileGravity)(const SetProjectileGravityQuery* query, SetProjectileGravityResult* result);
	void (*SetProjectileTarget)(const SetProjectileTargetQuery* query, SetProjectileTargetResult* result);
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
