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

// Feature basic info
struct FeatureInfo {
	int32_t featureID;
	int32_t featureDefID;
	int32_t teamID;
	int32_t allyTeamID;
	const char* tooltip;
};

struct FeatureInfoResult {
	const Error* error;
	FeatureInfo info;
};

// Feature health
struct FeatureHealth {
	float health;
	float maxHealth;
	float reclaimLeft;
	float resurrectProgress;
};

struct FeatureHealthResult {
	const Error* error;
	FeatureHealth health;
};

// Feature resources
struct FeatureResources {
	float metal;
	float energy;
	float reclaimTime;
};

struct FeatureResourcesResult {
	const Error* error;
	FeatureResources resources;
};

// Feature resurrection
struct FeatureResurrect {
	const char* resurrectAs;  // Unit def name
	int32_t resurrectDefID;
	int32_t facingDir;
};

struct FeatureResurrectResult {
	const Error* error;
	FeatureResurrect resurrect;
	bool canResurrect;
};

// API structure
struct FeaturesApi {
	// Validation
	BoolResult (*ValidFeatureID)(int32_t featureID);

	// Get all features
	Int32Array (*GetAllFeatures)();

	// Spatial queries
	Int32Array (*GetFeaturesInRectangle)(float minX, float minZ, float maxX, float maxZ);
	Int32Array (*GetFeaturesInSphere)(Float3 center, float radius);
	Int32Array (*GetFeaturesInCylinder)(Float3 center, float radius, float height);

	// Basic info
	Int32Result (*GetFeatureDefID)(int32_t featureID);
	Int32Result (*GetFeatureTeam)(int32_t featureID);
	Int32Result (*GetFeatureAllyTeam)(int32_t featureID);

	// Health
	FeatureHealthResult (*GetFeatureHealth)(int32_t featureID);

	// Physical properties
	FloatResult (*GetFeatureHeight)(int32_t featureID);
	FloatResult (*GetFeatureRadius)(int32_t featureID);
	FloatResult (*GetFeatureMass)(int32_t featureID);

	// Position and orientation
	Float3Result (*GetFeaturePosition)(int32_t featureID);
	FloatResult (*GetFeatureSeparation)(int32_t featureID1, int32_t featureID2, bool positional);
	Float3Result (*GetFeatureDirection)(int32_t featureID);
	Float3Result (*GetFeatureVelocity)(int32_t featureID);
	Int32Result (*GetFeatureHeading)(int32_t featureID);

	// Rotation (matrix)
	struct {
		Float3 col1;
		Float3 col2;
		Float3 col3;
	} (*GetFeatureRotation)(int32_t featureID);

	// Resources
	FeatureResourcesResult (*GetFeatureResources)(int32_t featureID);

	// Blocking
	BoolResult (*GetFeatureBlocking)(int32_t featureID, bool* isBlocking, bool* isSolidObjectCollidable, bool* isProjectileCollidable, bool* isRaySegmentCollidable, bool* crushable, bool* blockHeightChanges);

	// No select
	BoolResult (*GetFeatureNoSelect)(int32_t featureID);

	// Resurrection
	FeatureResurrectResult (*GetFeatureResurrect)(int32_t featureID);

	// Last attacked piece
	Int32Result (*GetFeatureLastAttackedPiece)(int32_t featureID);

	// Collision volumes
	CollisionVolumeDataResult (*GetFeatureCollisionVolumeData)(int32_t featureID);
	CollisionVolumeDataResult (*GetFeaturePieceCollisionVolumeData)(int32_t featureID, int32_t pieceNum);
};

extern const FeaturesApi FEATURES_API;

#ifdef __cplusplus
}
#endif
