#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Features API
// @see rts/Lua/LuaSyncedRead.cpp
//
// Feature queries (wrecks, rocks, trees, etc.)
// ============================================================================

// Feature health
struct FeatureHealth {
	float health;
	float maxHealth;
	float reclaimLeft;
	float resurrectProgress;
};

// Feature resources
struct FeatureResources {
	float metal;
	float energy;
	float reclaimTime;
};

// Feature resurrection
struct FeatureResurrect {
	const char* resurrectAs;  // Unit def name
	int32_t resurrectDefID;
	int32_t facingDir;
};

// Feature rotation (matrix)
struct FeatureRotation {
	Float3 col1;
	Float3 col2;
	Float3 col3;
};

// Feature blocking state
struct FeatureBlockingState {
	bool isBlocking;
	bool isSolidObjectCollidable;
	bool isProjectileCollidable;
	bool isRaySegmentCollidable;
	bool crushable;
	bool blockHeightChanges;
};

// Queries
struct ValidFeatureIDQuery { int32_t featureID; };
struct ValidFeatureIDResult { const Error* error; bool valid; };

struct GetAllFeaturesQuery { uint8_t _unused; };
struct GetAllFeaturesResult { const Error* error; int32_t* features; uint32_t count; };

struct GetFeaturesInRectangleQuery { float minX; float minZ; float maxX; float maxZ; };
struct GetFeaturesInRectangleResult { const Error* error; int32_t* features; uint32_t count; };

struct GetFeaturesInSphereQuery { Float3 center; float radius; };
struct GetFeaturesInSphereResult { const Error* error; int32_t* features; uint32_t count; };

struct GetFeaturesInCylinderQuery { Float3 center; float radius; float height; };
struct GetFeaturesInCylinderResult { const Error* error; int32_t* features; uint32_t count; };

struct GetFeatureDefIDQuery { int32_t featureID; };
struct GetFeatureDefIDResult { const Error* error; int32_t defID; };

struct GetFeatureTeamQuery { int32_t featureID; };
struct GetFeatureTeamResult { const Error* error; int32_t teamID; };

struct GetFeatureAllyTeamQuery { int32_t featureID; };
struct GetFeatureAllyTeamResult { const Error* error; int32_t allyTeamID; };

struct GetFeatureHealthQuery { int32_t featureID; };
struct GetFeatureHealthResult { const Error* error; FeatureHealth health; };

struct GetFeatureHeightQuery { int32_t featureID; };
struct GetFeatureHeightResult { const Error* error; float height; };

struct GetFeatureRadiusQuery { int32_t featureID; };
struct GetFeatureRadiusResult { const Error* error; float radius; };

struct GetFeatureMassQuery { int32_t featureID; };
struct GetFeatureMassResult { const Error* error; float mass; };

struct GetFeaturePositionQuery { int32_t featureID; };
struct GetFeaturePositionResult { const Error* error; Float3 position; };

struct GetFeatureSeparationQuery { int32_t featureID1; int32_t featureID2; bool positional; };
struct GetFeatureSeparationResult { const Error* error; float separation; };

struct GetFeatureDirectionQuery { int32_t featureID; };
struct GetFeatureDirectionResult { const Error* error; Float3 direction; };

struct GetFeatureVelocityQuery { int32_t featureID; };
struct GetFeatureVelocityResult { const Error* error; Float3 velocity; };

struct GetFeatureHeadingQuery { int32_t featureID; };
struct GetFeatureHeadingResult { const Error* error; int32_t heading; };

struct GetFeatureRotationQuery { int32_t featureID; };
struct GetFeatureRotationResult { const Error* error; FeatureRotation rotation; };

struct GetFeatureResourcesQuery { int32_t featureID; };
struct GetFeatureResourcesResult { const Error* error; FeatureResources resources; };

struct GetFeatureBlockingQuery { int32_t featureID; };
struct GetFeatureBlockingResult { const Error* error; FeatureBlockingState blockingState; };

struct GetFeatureNoSelectQuery { int32_t featureID; };
struct GetFeatureNoSelectResult { const Error* error; bool noSelect; };

struct GetFeatureResurrectQuery { int32_t featureID; };
struct GetFeatureResurrectResult { const Error* error; FeatureResurrect resurrect; bool canResurrect; };

struct GetFeatureLastAttackedPieceQuery { int32_t featureID; };
struct GetFeatureLastAttackedPieceResult { const Error* error; int32_t pieceNum; };

struct GetFeatureCollisionVolumeDataQuery { int32_t featureID; };
struct GetFeatureCollisionVolumeDataResult { const Error* error; CollisionVolumeData volume; };

struct GetFeaturePieceCollisionVolumeDataQuery { int32_t featureID; int32_t pieceNum; };
struct GetFeaturePieceCollisionVolumeDataResult { const Error* error; CollisionVolumeData volume; };

// API structure
struct FeaturesApi {
	void (*ValidFeatureID)(const ValidFeatureIDQuery* query, ValidFeatureIDResult* result);
	void (*GetAllFeatures)(const GetAllFeaturesQuery* query, GetAllFeaturesResult* result);
	void (*GetFeaturesInRectangle)(const GetFeaturesInRectangleQuery* query, GetFeaturesInRectangleResult* result);
	void (*GetFeaturesInSphere)(const GetFeaturesInSphereQuery* query, GetFeaturesInSphereResult* result);
	void (*GetFeaturesInCylinder)(const GetFeaturesInCylinderQuery* query, GetFeaturesInCylinderResult* result);
	void (*GetFeatureDefID)(const GetFeatureDefIDQuery* query, GetFeatureDefIDResult* result);
	void (*GetFeatureTeam)(const GetFeatureTeamQuery* query, GetFeatureTeamResult* result);
	void (*GetFeatureAllyTeam)(const GetFeatureAllyTeamQuery* query, GetFeatureAllyTeamResult* result);
	void (*GetFeatureHealth)(const GetFeatureHealthQuery* query, GetFeatureHealthResult* result);
	void (*GetFeatureHeight)(const GetFeatureHeightQuery* query, GetFeatureHeightResult* result);
	void (*GetFeatureRadius)(const GetFeatureRadiusQuery* query, GetFeatureRadiusResult* result);
	void (*GetFeatureMass)(const GetFeatureMassQuery* query, GetFeatureMassResult* result);
	void (*GetFeaturePosition)(const GetFeaturePositionQuery* query, GetFeaturePositionResult* result);
	void (*GetFeatureSeparation)(const GetFeatureSeparationQuery* query, GetFeatureSeparationResult* result);
	void (*GetFeatureDirection)(const GetFeatureDirectionQuery* query, GetFeatureDirectionResult* result);
	void (*GetFeatureVelocity)(const GetFeatureVelocityQuery* query, GetFeatureVelocityResult* result);
	void (*GetFeatureHeading)(const GetFeatureHeadingQuery* query, GetFeatureHeadingResult* result);
	void (*GetFeatureRotation)(const GetFeatureRotationQuery* query, GetFeatureRotationResult* result);
	void (*GetFeatureResources)(const GetFeatureResourcesQuery* query, GetFeatureResourcesResult* result);
	void (*GetFeatureBlocking)(const GetFeatureBlockingQuery* query, GetFeatureBlockingResult* result);
	void (*GetFeatureNoSelect)(const GetFeatureNoSelectQuery* query, GetFeatureNoSelectResult* result);
	void (*GetFeatureResurrect)(const GetFeatureResurrectQuery* query, GetFeatureResurrectResult* result);
	void (*GetFeatureLastAttackedPiece)(const GetFeatureLastAttackedPieceQuery* query, GetFeatureLastAttackedPieceResult* result);
	void (*GetFeatureCollisionVolumeData)(const GetFeatureCollisionVolumeDataQuery* query, GetFeatureCollisionVolumeDataResult* result);
	void (*GetFeaturePieceCollisionVolumeData)(const GetFeaturePieceCollisionVolumeDataQuery* query, GetFeaturePieceCollisionVolumeDataResult* result);
};

extern const FeaturesApi FEATURES_API;

#ifdef __cplusplus
}
#endif
