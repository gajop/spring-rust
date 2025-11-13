#include "Features.h"

#include "Sim/Features/Feature.h"
#include "Sim/Features/FeatureDef.h"
#include "Sim/Units/UnitDef.h"
#include "Sim/Features/FeatureHandler.h"
#include "Sim/Misc/QuadField.h"
#include "Sim/Misc/GlobalSynced.h"
#include "System/float3.h"

namespace {

// Scratch buffer
static thread_local char scratchBuffer[8192];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Feature system not ready" };
static const Error INVALID_FEATURE_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid feature ID" };

// Helper: check if ready
static bool IsReady()
{
	return (gs != nullptr);
}

// Validation
static void NativeValidFeatureID(const ValidFeatureIDQuery* query, ValidFeatureIDResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->valid = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}
	result->valid = featureHandler.GetFeature(query->featureID) != nullptr;
}

// Get all features
static void NativeGetAllFeatures(const GetAllFeaturesQuery* query, GetAllFeaturesResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->features = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// Use scratch buffer for array
	int32_t* features = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxFeatures = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);

	for (const auto featureID : featureHandler.GetActiveFeatureIDs()) {
		if (count < maxFeatures) {
			features[count++] = featureID;
		}
	}

	result->features = features;
	result->count = count;
	bufferPos += count * sizeof(int32_t);
}

// Spatial queries
static void NativeGetFeaturesInRectangle(const GetFeaturesInRectangleQuery* query, GetFeaturesInRectangleResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->features = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const float3 mins(query->minX, 0.0f, query->minZ);
	const float3 maxs(query->maxX, 0.0f, query->maxZ);

	// Use scratch buffer for array
	int32_t* features = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxFeatures = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);

	QuadFieldQuery qfq;
	quadField.GetFeaturesExact(qfq, mins, maxs);
	if (qfq.features != nullptr) {
		for (const CFeature* feature : *(qfq.features)) {
			if (feature != nullptr && count < maxFeatures) {
				features[count++] = feature->id;
			}
		}
	}

	result->features = features;
	result->count = count;
	bufferPos += count * sizeof(int32_t);
}

static void NativeGetFeaturesInSphere(const GetFeaturesInSphereQuery* query, GetFeaturesInSphereResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->features = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const float3 pos(query->center.x, query->center.y, query->center.z);
	const float radiusSq = query->radius * query->radius;

	// Use scratch buffer for array
	int32_t* features = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxFeatures = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);

	QuadFieldQuery qfq;
	quadField.GetFeaturesExact(qfq, pos, query->radius);
	if (qfq.features != nullptr) {
		for (const CFeature* feature : *(qfq.features)) {
			if (feature != nullptr) {
				const float distSq = feature->pos.SqDistance(pos);
				if (distSq <= radiusSq && count < maxFeatures) {
					features[count++] = feature->id;
				}
			}
		}
	}

	result->features = features;
	result->count = count;
	bufferPos += count * sizeof(int32_t);
}

static void NativeGetFeaturesInCylinder(const GetFeaturesInCylinderQuery* query, GetFeaturesInCylinderResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->features = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const float3 pos(query->center.x, query->center.y, query->center.z);
	const float radiusSq = query->radius * query->radius;
	const float halfHeight = query->height * 0.5f;

	// Use scratch buffer for array
	int32_t* features = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxFeatures = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);

	QuadFieldQuery qfq;
	quadField.GetFeaturesExact(qfq, pos, query->radius);
	if (qfq.features != nullptr) {
		for (const CFeature* feature : *(qfq.features)) {
			if (feature != nullptr) {
				const float3& fpos = feature->pos;
				const float dx = fpos.x - pos.x;
				const float dz = fpos.z - pos.z;
				const float distXZSq = dx * dx + dz * dz;
				const float dy = std::abs(fpos.y - pos.y);

				if (distXZSq <= radiusSq && dy <= halfHeight && count < maxFeatures) {
					features[count++] = feature->id;
				}
			}
		}
	}

	result->features = features;
	result->count = count;
	bufferPos += count * sizeof(int32_t);
}

// Basic info
static void NativeGetFeatureDefID(const GetFeatureDefIDQuery* query, GetFeatureDefIDResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->defID = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	result->defID = feature->def->id;
}

static void NativeGetFeatureTeam(const GetFeatureTeamQuery* query, GetFeatureTeamResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->teamID = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	result->teamID = feature->team;
}

static void NativeGetFeatureAllyTeam(const GetFeatureAllyTeamQuery* query, GetFeatureAllyTeamResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->allyTeamID = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	result->allyTeamID = feature->allyteam;
}

// Health
static void NativeGetFeatureHealth(const GetFeatureHealthQuery* query, GetFeatureHealthResult* result)
{
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	result->health.health = feature->health;
	result->health.maxHealth = feature->maxHealth;
	result->health.reclaimLeft = feature->reclaimLeft;
	result->health.resurrectProgress = feature->resurrectProgress;
}

// Physical properties
static void NativeGetFeatureHeight(const GetFeatureHeightQuery* query, GetFeatureHeightResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->height = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	result->height = feature->height;
}

static void NativeGetFeatureRadius(const GetFeatureRadiusQuery* query, GetFeatureRadiusResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->radius = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	result->radius = feature->radius;
}

static void NativeGetFeatureMass(const GetFeatureMassQuery* query, GetFeatureMassResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->mass = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	result->mass = feature->mass;
}

// Position and orientation
static void NativeGetFeaturePosition(const GetFeaturePositionQuery* query, GetFeaturePositionResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->position.x = 0.0f;
	result->position.y = 0.0f;
	result->position.z = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	result->position.x = feature->pos.x;
	result->position.y = feature->pos.y;
	result->position.z = feature->pos.z;
}

static void NativeGetFeatureSeparation(const GetFeatureSeparationQuery* query, GetFeatureSeparationResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->separation = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature1 = featureHandler.GetFeature(query->featureID1);
	const CFeature* feature2 = featureHandler.GetFeature(query->featureID2);

	if (feature1 == nullptr || feature2 == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	if (query->positional) {
		result->separation = feature1->pos.distance(feature2->pos);
	} else {
		const float radSum = feature1->radius + feature2->radius;
		const float dist = feature1->pos.distance(feature2->pos);
		result->separation = std::max(0.0f, dist - radSum);
	}
}

static void NativeGetFeatureDirection(const GetFeatureDirectionQuery* query, GetFeatureDirectionResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->direction.x = 0.0f;
	result->direction.y = 0.0f;
	result->direction.z = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	const float3 dir = feature->frontdir;
	result->direction.x = dir.x;
	result->direction.y = dir.y;
	result->direction.z = dir.z;
}

static void NativeGetFeatureVelocity(const GetFeatureVelocityQuery* query, GetFeatureVelocityResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->velocity.x = 0.0f;
	result->velocity.y = 0.0f;
	result->velocity.z = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	// Features typically don't move
}

static void NativeGetFeatureHeading(const GetFeatureHeadingQuery* query, GetFeatureHeadingResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->heading = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	result->heading = feature->heading;
}

static void NativeGetFeatureRotation(const GetFeatureRotationQuery* query, GetFeatureRotationResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->rotation.col1 = {1.0f, 0.0f, 0.0f};
	result->rotation.col2 = {0.0f, 1.0f, 0.0f};
	result->rotation.col3 = {0.0f, 0.0f, 1.0f};

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	// Return identity for now
}

// Resources
static void NativeGetFeatureResources(const GetFeatureResourcesQuery* query, GetFeatureResourcesResult* result)
{
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	result->resources.metal = feature->def->metal * feature->reclaimLeft;
	result->resources.energy = feature->def->energy * feature->reclaimLeft;
	result->resources.reclaimTime = feature->def->reclaimTime;
}

// Blocking
static void NativeGetFeatureBlocking(const GetFeatureBlockingQuery* query, GetFeatureBlockingResult* result)
{
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	result->blockingState.isBlocking = feature->IsBlocking();
	result->blockingState.isSolidObjectCollidable = !feature->collisionVolume.IgnoreHits();
	result->blockingState.isProjectileCollidable = !feature->collisionVolume.IgnoreHits();
	result->blockingState.isRaySegmentCollidable = !feature->collisionVolume.IgnoreHits();
	result->blockingState.crushable = false;
	result->blockingState.blockHeightChanges = feature->def->floating;
}

// No select
static void NativeGetFeatureNoSelect(const GetFeatureNoSelectQuery* query, GetFeatureNoSelectResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->noSelect = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	result->noSelect = feature->noSelect;
}

// Resurrection
static void NativeGetFeatureResurrect(const GetFeatureResurrectQuery* query, GetFeatureResurrectResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->canResurrect = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	if (feature->udef != nullptr) {
		result->canResurrect = true;
		result->resurrect.resurrectAs = feature->udef->name.c_str();
		result->resurrect.resurrectDefID = feature->udef->id;
		result->resurrect.facingDir = feature->buildFacing;
	}
}

// Last attacked piece
static void NativeGetFeatureLastAttackedPiece(const GetFeatureLastAttackedPieceQuery* query, GetFeatureLastAttackedPieceResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->pieceNum = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	result->pieceNum = -1; // lastAttackedPiece not available
}

// Collision volumes
static void NativeGetFeatureCollisionVolumeData(const GetFeatureCollisionVolumeDataQuery* query, GetFeatureCollisionVolumeDataResult* result)
{
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	const CollisionVolume& cv = feature->collisionVolume;
	result->volume.scaleX = cv.GetScales().x;
	result->volume.scaleY = cv.GetScales().y;
	result->volume.scaleZ = cv.GetScales().z;
	result->volume.offsetX = cv.GetOffsets().x;
	result->volume.offsetY = cv.GetOffsets().y;
	result->volume.offsetZ = cv.GetOffsets().z;
	result->volume.volumeType = cv.GetVolumeType();
	result->volume.testType = cv.UseContHitTest() ? 1 : 0;  // 1=continuous, 0=discrete
	result->volume.primaryAxis = cv.GetPrimaryAxis();
	result->volume.disabled = cv.IgnoreHits();
}

static void NativeGetFeaturePieceCollisionVolumeData(const GetFeaturePieceCollisionVolumeDataQuery* query, GetFeaturePieceCollisionVolumeDataResult* result)
{
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	// Features don't have piece-specific collision volumes in Spring
	// Return the main collision volume
	const CollisionVolume& cv = feature->collisionVolume;
	result->volume.scaleX = cv.GetScales().x;
	result->volume.scaleY = cv.GetScales().y;
	result->volume.scaleZ = cv.GetScales().z;
	result->volume.offsetX = cv.GetOffsets().x;
	result->volume.offsetY = cv.GetOffsets().y;
	result->volume.offsetZ = cv.GetOffsets().z;
	result->volume.volumeType = cv.GetVolumeType();
	result->volume.testType = cv.UseContHitTest() ? 1 : 0;  // 1=continuous, 0=discrete
	result->volume.primaryAxis = cv.GetPrimaryAxis();
	result->volume.disabled = cv.IgnoreHits();
}

} // namespace

const FeaturesApi FEATURES_API = {
	.ValidFeatureID = NativeValidFeatureID,
	.GetAllFeatures = NativeGetAllFeatures,

	.GetFeaturesInRectangle = NativeGetFeaturesInRectangle,
	.GetFeaturesInSphere = NativeGetFeaturesInSphere,
	.GetFeaturesInCylinder = NativeGetFeaturesInCylinder,

	.GetFeatureDefID = NativeGetFeatureDefID,
	.GetFeatureTeam = NativeGetFeatureTeam,
	.GetFeatureAllyTeam = NativeGetFeatureAllyTeam,

	.GetFeatureHealth = NativeGetFeatureHealth,

	.GetFeatureHeight = NativeGetFeatureHeight,
	.GetFeatureRadius = NativeGetFeatureRadius,
	.GetFeatureMass = NativeGetFeatureMass,

	.GetFeaturePosition = NativeGetFeaturePosition,
	.GetFeatureSeparation = NativeGetFeatureSeparation,
	.GetFeatureDirection = NativeGetFeatureDirection,
	.GetFeatureVelocity = NativeGetFeatureVelocity,
	.GetFeatureHeading = NativeGetFeatureHeading,

	.GetFeatureRotation = NativeGetFeatureRotation,

	.GetFeatureResources = NativeGetFeatureResources,

	.GetFeatureBlocking = NativeGetFeatureBlocking,
	.GetFeatureNoSelect = NativeGetFeatureNoSelect,

	.GetFeatureResurrect = NativeGetFeatureResurrect,
	.GetFeatureLastAttackedPiece = NativeGetFeatureLastAttackedPiece,

	.GetFeatureCollisionVolumeData = NativeGetFeatureCollisionVolumeData,
	.GetFeaturePieceCollisionVolumeData = NativeGetFeaturePieceCollisionVolumeData,
};
