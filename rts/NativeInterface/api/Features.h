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
	float defMetal;
	float energy;
	float defEnergy;
	float reclaimLeft;
	float reclaimTime;
};

// Feature resurrection
struct FeatureResurrect {
	const char* resurrectAs;  // Unit def name
	int32_t resurrectDefID;
	int32_t facingDir;
};

struct FeatureLastHitPiece {
	const char* name;
	int32_t pieceNum;
	int32_t frame;
	bool wasHit;
};

// Feature rotation (pitch, yaw, roll)
struct FeatureRotation {
	float pitch;
	float yaw;
	float roll;
};

// Feature blocking state
struct FeatureBlockingState {
	bool isBlocking;
	bool isSolidObjectCollidable;
	bool isProjectileCollidable;
	bool isRaySegmentCollidable;
	bool crushable;
	bool blockEnemyPushing;
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

struct GetFeaturesInCylinderQuery { float x; float z; float radius; float height; };
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

struct FeaturePositionExt {
	Float3 position;
	Float3 midPosition;
	Float3 aimPosition;
};

struct GetFeaturePositionExtQuery { int32_t featureID; };
struct GetFeaturePositionExtResult { const Error* error; FeaturePositionExt position; };

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
struct GetFeatureLastAttackedPieceResult { const Error* error; FeatureLastHitPiece piece; };

struct GetFeatureCollisionVolumeDataQuery { int32_t featureID; };
struct GetFeatureCollisionVolumeDataResult { const Error* error; CollisionVolumeData volume; };

struct GetFeaturePieceCollisionVolumeDataQuery { int32_t featureID; };
struct GetFeaturePieceCollisionVolumeDataResult { const Error* error; CollisionVolumeData volume; };

struct ClearFeaturesPreviousDrawFlagQuery { uint8_t _unused; };
struct ClearFeaturesPreviousDrawFlagResult { const Error* error; bool success; };

struct GetFeatureNoDrawQuery { int32_t featureID; };
struct GetFeatureNoDrawResult { const Error* error; bool noDraw; };

struct GetFeatureLuaDrawQuery { int32_t featureID; };
struct GetFeatureLuaDrawResult { const Error* error; bool luaDraw; };

struct GetFeatureEngineDrawMaskQuery { int32_t featureID; };
struct GetFeatureEngineDrawMaskResult { const Error* error; uint32_t mask; };

struct GetFeatureDrawFlagQuery { int32_t featureID; };
struct GetFeatureDrawFlagResult { const Error* error; uint8_t flag; };

struct GetFeatureAlwaysUpdateMatrixQuery { int32_t featureID; };
struct GetFeatureAlwaysUpdateMatrixResult { const Error* error; bool update; };

struct FeatureTransformMatrix { float values[16]; };
struct GetFeatureTransformMatrixQuery { int32_t featureID; };
struct GetFeatureTransformMatrixResult { const Error* error; FeatureTransformMatrix matrix; };

struct FeatureSelectionVolumeData {
	Float3 scales;
	Float3 offsets;
	int32_t primaryAxis;
};

struct GetFeatureSelectionVolumeDataQuery { int32_t featureID; };
struct GetFeatureSelectionVolumeDataResult { const Error* error; FeatureSelectionVolumeData data; };

struct GetFeatureFireTimeQuery { int32_t featureID; };
struct GetFeatureFireTimeResult { const Error* error; float fireTime; };

struct GetFeatureSmokeTimeQuery { int32_t featureID; };
struct GetFeatureSmokeTimeResult { const Error* error; float smokeTime; };

struct GetRenderFeaturesQuery { int32_t drawMask; bool sendMask; };
struct GetRenderFeaturesResult { const Error* error; int32_t* features; uint32_t count; };

struct GetRenderFeaturesDrawFlagChangedQuery { bool sendMask; };
struct GetRenderFeaturesDrawFlagChangedResult { const Error* error; int32_t* features; uint32_t count; };

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
	void (*GetFeaturePositionExt)(const GetFeaturePositionExtQuery* query, GetFeaturePositionExtResult* result);
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
	void (*ClearFeaturesPreviousDrawFlag)(const ClearFeaturesPreviousDrawFlagQuery* query, ClearFeaturesPreviousDrawFlagResult* result);
	void (*GetFeatureNoDraw)(const GetFeatureNoDrawQuery* query, GetFeatureNoDrawResult* result);
	void (*GetFeatureLuaDraw)(const GetFeatureLuaDrawQuery* query, GetFeatureLuaDrawResult* result);
	void (*GetFeatureEngineDrawMask)(const GetFeatureEngineDrawMaskQuery* query, GetFeatureEngineDrawMaskResult* result);
	void (*GetFeatureDrawFlag)(const GetFeatureDrawFlagQuery* query, GetFeatureDrawFlagResult* result);
	void (*GetFeatureAlwaysUpdateMatrix)(const GetFeatureAlwaysUpdateMatrixQuery* query, GetFeatureAlwaysUpdateMatrixResult* result);
	void (*GetFeatureTransformMatrix)(const GetFeatureTransformMatrixQuery* query, GetFeatureTransformMatrixResult* result);
	void (*GetFeatureSelectionVolumeData)(const GetFeatureSelectionVolumeDataQuery* query, GetFeatureSelectionVolumeDataResult* result);
	void (*GetFeatureFireTime)(const GetFeatureFireTimeQuery* query, GetFeatureFireTimeResult* result);
	void (*GetFeatureSmokeTime)(const GetFeatureSmokeTimeQuery* query, GetFeatureSmokeTimeResult* result);
	void (*GetRenderFeatures)(const GetRenderFeaturesQuery* query, GetRenderFeaturesResult* result);
	void (*GetRenderFeaturesDrawFlagChanged)(const GetRenderFeaturesDrawFlagChangedQuery* query, GetRenderFeaturesDrawFlagChangedResult* result);
};

extern const FeaturesApi FEATURES_API;

#ifdef __cplusplus
}
#endif
