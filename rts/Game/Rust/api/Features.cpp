#include "Features.h"

#include "Sim/Features/Feature.h"
#include "Sim/Features/FeatureHandler.h"
#include "Sim/Misc/QuadField.h"
#include "Sim/Misc/GlobalSynced.h"
#include "System/float3.h"

namespace {

// Error constants
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Feature system not ready"
};

static const Error INVALID_FEATURE_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid feature ID"
};

// Helper: check if ready
static bool IsReady()
{
	return (gs != nullptr);
}

// Validation
static BoolResult NativeValidFeatureID(int32_t featureID)
{
	BoolResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}
	result.value = featureHandler.GetFeature(featureID) != nullptr;
	return result;
}

// Get all features
static Int32Array NativeGetAllFeatures()
{
	Int32Array result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Use static storage - valid for call duration only
	static thread_local std::vector<int32_t> features;
	features.clear();

	for (const auto& pair : featureHandler.GetActiveFeatures()) {
		if (pair.second != nullptr) {
			features.push_back(pair.first);
		}
	}

	result.data = features.data();
	result.length = static_cast<uint32_t>(features.size());
	return result;
}

// Spatial queries
static Int32Array NativeGetFeaturesInRectangle(float minX, float minZ, float maxX, float maxZ)
{
	Int32Array result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Use static storage - valid for call duration only
	static thread_local std::vector<int32_t> features;
	features.clear();

	const float3 mins(minX, 0.0f, minZ);
	const float3 maxs(maxX, 0.0f, maxZ);

	const auto& foundFeatures = quadField.GetFeaturesExact(mins, maxs);
	for (const CFeature* feature : foundFeatures) {
		if (feature != nullptr) {
			features.push_back(feature->id);
		}
	}

	result.data = features.data();
	result.length = static_cast<uint32_t>(features.size());
	return result;
}

static Int32Array NativeGetFeaturesInSphere(Float3 center, float radius)
{
	Int32Array result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Use static storage - valid for call duration only
	static thread_local std::vector<int32_t> features;
	features.clear();

	const float3 pos(center.x, center.y, center.z);
	const float radiusSq = radius * radius;

	const auto& foundFeatures = quadField.GetFeaturesExact(pos, radius);
	for (const CFeature* feature : foundFeatures) {
		if (feature != nullptr) {
			const float distSq = feature->pos.SqDistance(pos);
			if (distSq <= radiusSq) {
				features.push_back(feature->id);
			}
		}
	}

	result.data = features.data();
	result.length = static_cast<uint32_t>(features.size());
	return result;
}

static Int32Array NativeGetFeaturesInCylinder(Float3 center, float radius, float height)
{
	Int32Array result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Use static storage - valid for call duration only
	static thread_local std::vector<int32_t> features;
	features.clear();

	const float3 pos(center.x, center.y, center.z);
	const float radiusSq = radius * radius;
	const float halfHeight = height * 0.5f;

	const auto& foundFeatures = quadField.GetFeaturesExact(pos, radius);
	for (const CFeature* feature : foundFeatures) {
		if (feature != nullptr) {
			const float3& fpos = feature->pos;
			const float dx = fpos.x - pos.x;
			const float dz = fpos.z - pos.z;
			const float distXZSq = dx * dx + dz * dz;
			const float dy = std::abs(fpos.y - pos.y);

			if (distXZSq <= radiusSq && dy <= halfHeight) {
				features.push_back(feature->id);
			}
		}
	}

	result.data = features.data();
	result.length = static_cast<uint32_t>(features.size());
	return result;
}

// Basic info
static Int32Result NativeGetFeatureDefID(int32_t featureID)
{
	Int32Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CFeature* feature = featureHandler.GetFeature(featureID);
	if (feature == nullptr) {
		result.error = &INVALID_FEATURE_ERROR;
		return result;
	}

	result.value = feature->def->id;
	return result;
}

static Int32Result NativeGetFeatureTeam(int32_t featureID)
{
	Int32Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CFeature* feature = featureHandler.GetFeature(featureID);
	if (feature == nullptr) {
		result.error = &INVALID_FEATURE_ERROR;
		return result;
	}

	result.value = feature->team;
	return result;
}

static Int32Result NativeGetFeatureAllyTeam(int32_t featureID)
{
	Int32Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CFeature* feature = featureHandler.GetFeature(featureID);
	if (feature == nullptr) {
		result.error = &INVALID_FEATURE_ERROR;
		return result;
	}

	result.value = feature->allyteam;
	return result;
}

// Health
static FeatureHealthResult NativeGetFeatureHealth(int32_t featureID)
{
	FeatureHealthResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CFeature* feature = featureHandler.GetFeature(featureID);
	if (feature == nullptr) {
		result.error = &INVALID_FEATURE_ERROR;
		return result;
	}

	result.health.health = feature->health;
	result.health.maxHealth = feature->maxHealth;
	result.health.reclaimLeft = feature->reclaimLeft;
	result.health.resurrectProgress = feature->resurrectProgress;

	return result;
}

// Physical properties
static FloatResult NativeGetFeatureHeight(int32_t featureID)
{
	FloatResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CFeature* feature = featureHandler.GetFeature(featureID);
	if (feature == nullptr) {
		result.error = &INVALID_FEATURE_ERROR;
		return result;
	}

	result.value = feature->height;
	return result;
}

static FloatResult NativeGetFeatureRadius(int32_t featureID)
{
	FloatResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CFeature* feature = featureHandler.GetFeature(featureID);
	if (feature == nullptr) {
		result.error = &INVALID_FEATURE_ERROR;
		return result;
	}

	result.value = feature->radius;
	return result;
}

static FloatResult NativeGetFeatureMass(int32_t featureID)
{
	FloatResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CFeature* feature = featureHandler.GetFeature(featureID);
	if (feature == nullptr) {
		result.error = &INVALID_FEATURE_ERROR;
		return result;
	}

	result.value = feature->mass;
	return result;
}

// Position and orientation
static Float3Result NativeGetFeaturePosition(int32_t featureID)
{
	Float3Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CFeature* feature = featureHandler.GetFeature(featureID);
	if (feature == nullptr) {
		result.error = &INVALID_FEATURE_ERROR;
		return result;
	}

	result.value.x = feature->pos.x;
	result.value.y = feature->pos.y;
	result.value.z = feature->pos.z;
	return result;
}

static FloatResult NativeGetFeatureSeparation(int32_t featureID1, int32_t featureID2, bool positional)
{
	FloatResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CFeature* feature1 = featureHandler.GetFeature(featureID1);
	const CFeature* feature2 = featureHandler.GetFeature(featureID2);

	if (feature1 == nullptr || feature2 == nullptr) {
		result.error = &INVALID_FEATURE_ERROR;
		return result;
	}

	if (positional) {
		result.value = feature1->pos.distance(feature2->pos);
	} else {
		const float radSum = feature1->radius + feature2->radius;
		const float dist = feature1->pos.distance(feature2->pos);
		result.value = std::max(0.0f, dist - radSum);
	}

	return result;
}

static Float3Result NativeGetFeatureDirection(int32_t featureID)
{
	Float3Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CFeature* feature = featureHandler.GetFeature(featureID);
	if (feature == nullptr) {
		result.error = &INVALID_FEATURE_ERROR;
		return result;
	}

	// Direction is the forward vector (heading)
	const float3 dir = feature->frontdir;
	result.value.x = dir.x;
	result.value.y = dir.y;
	result.value.z = dir.z;
	return result;
}

static Float3Result NativeGetFeatureVelocity(int32_t featureID)
{
	Float3Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CFeature* feature = featureHandler.GetFeature(featureID);
	if (feature == nullptr) {
		result.error = &INVALID_FEATURE_ERROR;
		return result;
	}

	// Features typically don't move
	result.value.x = 0.0f;
	result.value.y = 0.0f;
	result.value.z = 0.0f;
	return result;
}

static Int32Result NativeGetFeatureHeading(int32_t featureID)
{
	Int32Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CFeature* feature = featureHandler.GetFeature(featureID);
	if (feature == nullptr) {
		result.error = &INVALID_FEATURE_ERROR;
		return result;
	}

	result.value = feature->heading;
	return result;
}

// Resources
static FeatureResourcesResult NativeGetFeatureResources(int32_t featureID)
{
	FeatureResourcesResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CFeature* feature = featureHandler.GetFeature(featureID);
	if (feature == nullptr) {
		result.error = &INVALID_FEATURE_ERROR;
		return result;
	}

	result.resources.metal = feature->def->metal * feature->reclaimLeft;
	result.resources.energy = feature->def->energy * feature->reclaimLeft;
	result.resources.reclaimTime = feature->def->reclaimTime;

	return result;
}

// Blocking
static BoolResult NativeGetFeatureBlocking(int32_t featureID, bool* isBlocking, bool* isSolidObjectCollidable,
	bool* isProjectileCollidable, bool* isRaySegmentCollidable, bool* crushable, bool* blockHeightChanges)
{
	BoolResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CFeature* feature = featureHandler.GetFeature(featureID);
	if (feature == nullptr) {
		result.error = &INVALID_FEATURE_ERROR;
		return result;
	}

	if (isBlocking) *isBlocking = feature->blocking;
	if (isSolidObjectCollidable) *isSolidObjectCollidable = feature->collisionVolume->DefaultToFeature();
	if (isProjectileCollidable) *isProjectileCollidable = feature->collisionVolume->DefaultToFeature();
	if (isRaySegmentCollidable) *isRaySegmentCollidable = feature->collisionVolume->DefaultToFeature();
	if (crushable) *crushable = false; // Features generally not crushable
	if (blockHeightChanges) *blockHeightChanges = feature->def->floating;

	result.value = feature->blocking;
	return result;
}

// No select
static BoolResult NativeGetFeatureNoSelect(int32_t featureID)
{
	BoolResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CFeature* feature = featureHandler.GetFeature(featureID);
	if (feature == nullptr) {
		result.error = &INVALID_FEATURE_ERROR;
		return result;
	}

	result.value = feature->noSelect;
	return result;
}

// Resurrection
static FeatureResurrectResult NativeGetFeatureResurrect(int32_t featureID)
{
	FeatureResurrectResult result = {};
	result.canResurrect = false;

	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CFeature* feature = featureHandler.GetFeature(featureID);
	if (feature == nullptr) {
		result.error = &INVALID_FEATURE_ERROR;
		return result;
	}

	if (feature->udef != nullptr) {
		result.canResurrect = true;
		result.resurrect.resurrectAs = feature->udef->name.c_str();
		result.resurrect.resurrectDefID = feature->udef->id;
		result.resurrect.facingDir = feature->buildFacing;
	}

	return result;
}

// Last attacked piece
static Int32Result NativeGetFeatureLastAttackedPiece(int32_t featureID)
{
	Int32Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const CFeature* feature = featureHandler.GetFeature(featureID);
	if (feature == nullptr) {
		result.error = &INVALID_FEATURE_ERROR;
		return result;
	}

	result.value = feature->lastAttackedPiece;
	return result;
}

// Stub rotation function (returning identity for now)
static struct { Float3 col1; Float3 col2; Float3 col3; } NativeGetFeatureRotation(int32_t featureID)
{
	struct { Float3 col1; Float3 col2; Float3 col3; } result;
	result.col1 = {1.0f, 0.0f, 0.0f};
	result.col2 = {0.0f, 1.0f, 0.0f};
	result.col3 = {0.0f, 0.0f, 1.0f};
	return result;
}

// Stub collision volume functions
static CollisionVolumeDataResult NativeGetFeatureCollisionVolumeData(int32_t featureID)
{
	CollisionVolumeDataResult result = {};
	static const Error NOT_IMPLEMENTED = {
		.code = ERROR_NOT_AVAILABLE,
		.message = "Collision volume data not yet implemented"
	};
	result.error = &NOT_IMPLEMENTED;
	return result;
}

static CollisionVolumeDataResult NativeGetFeaturePieceCollisionVolumeData(int32_t featureID, int32_t pieceNum)
{
	CollisionVolumeDataResult result = {};
	static const Error NOT_IMPLEMENTED = {
		.code = ERROR_NOT_AVAILABLE,
		.message = "Collision volume data not yet implemented"
	};
	result.error = &NOT_IMPLEMENTED;
	return result;
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
