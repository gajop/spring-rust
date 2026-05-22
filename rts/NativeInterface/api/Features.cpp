#include "Features.h"

#include "Sim/Features/Feature.h"
#include "Sim/Features/FeatureDef.h"
#include "Sim/Units/UnitDef.h"
#include "Sim/Features/FeatureHandler.h"
#include "Sim/Misc/QuadField.h"
#include "Sim/Misc/GlobalSynced.h"
#include "System/float3.h"
#include "Rendering/Features/FeatureDrawer.h"
#include "Game/Game.h"

namespace {

// Scratch buffer
static thread_local char scratchBuffer[1024];
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

	const float3 pos(query->x, 0.0f, query->z);
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
	result->rotation.pitch = 0.0f;
	result->rotation.yaw = 0.0f;
	result->rotation.roll = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	const CMatrix44f& matrix = feature->GetTransformMatrix(true);
	const float3 angles = matrix.GetEulerAnglesLftHand();
	result->rotation.pitch = angles[CMatrix44f::ANGLE_P];
	result->rotation.yaw = angles[CMatrix44f::ANGLE_Y];
	result->rotation.roll = angles[CMatrix44f::ANGLE_R];
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

	result->resources.metal = feature->resources.metal;
	result->resources.defMetal = feature->defResources.metal;
	result->resources.energy = feature->resources.energy;
	result->resources.defEnergy = feature->defResources.energy;
	result->resources.reclaimLeft = feature->reclaimLeft;
	result->resources.reclaimTime = feature->reclaimTime;
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

	result->blockingState.isBlocking = feature->HasPhysicalStateBit(CSolidObject::PSTATE_BIT_BLOCKING);
	result->blockingState.isSolidObjectCollidable = feature->HasCollidableStateBit(CSolidObject::CSTATE_BIT_SOLIDOBJECTS);
	result->blockingState.isProjectileCollidable = feature->HasCollidableStateBit(CSolidObject::CSTATE_BIT_PROJECTILES);
	result->blockingState.isRaySegmentCollidable = feature->HasCollidableStateBit(CSolidObject::CSTATE_BIT_QUADMAPRAYS);
	result->blockingState.crushable = feature->crushable;
	result->blockingState.blockEnemyPushing = feature->blockEnemyPushing;
	result->blockingState.blockHeightChanges = feature->blockHeightChanges;
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

static void NativeClearFeaturesPreviousDrawFlag(const ClearFeaturesPreviousDrawFlagQuery* query, ClearFeaturesPreviousDrawFlagResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady() || featureDrawer == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	featureDrawer->ClearPreviousDrawFlags();
	result->success = true;
}

static void NativeGetFeatureNoDraw(const GetFeatureNoDrawQuery* query, GetFeatureNoDrawResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->noDraw = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	result->noDraw = feature->noDraw;
}

static void NativeGetFeatureLuaDraw(const GetFeatureLuaDrawQuery* query, GetFeatureLuaDrawResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->luaDraw = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	result->luaDraw = feature->luaDraw;
}

static void NativeGetFeatureEngineDrawMask(const GetFeatureEngineDrawMaskQuery* query, GetFeatureEngineDrawMaskResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->mask = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	result->mask = feature->engineDrawMask;
}

static void NativeGetFeatureDrawFlag(const GetFeatureDrawFlagQuery* query, GetFeatureDrawFlagResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->flag = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	result->flag = (feature->GetDrawFlag() != DrawFlags::SO_NODRAW_FLAG);
}

static void NativeGetFeatureAlwaysUpdateMatrix(const GetFeatureAlwaysUpdateMatrixQuery* query, GetFeatureAlwaysUpdateMatrixResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->update = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	result->update = feature->alwaysUpdateMat;
}

static void NativeGetFeatureTransformMatrix(const GetFeatureTransformMatrixQuery* query, GetFeatureTransformMatrixResult* result)
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

	const CMatrix44f m = feature->GetTransformMatrix(false, true);
	for (size_t i = 0; i < 16; ++i) {
		result->matrix.values[i] = m[i];
	}
}

static void NativeGetFeatureSelectionVolumeData(const GetFeatureSelectionVolumeDataQuery* query, GetFeatureSelectionVolumeDataResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->data.scales = {0.0f, 0.0f, 0.0f};
	result->data.offsets = {0.0f, 0.0f, 0.0f};
	result->data.primaryAxis = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	const CollisionVolume& vol = feature->selectionVolume;
	result->data.scales = {vol.GetScales().x, vol.GetScales().y, vol.GetScales().z};
	result->data.offsets = {vol.GetOffsets().x, vol.GetOffsets().y, vol.GetOffsets().z};
	result->data.primaryAxis = vol.GetPrimaryAxis();
}

static void NativeGetFeatureFireTime(const GetFeatureFireTimeQuery* query, GetFeatureFireTimeResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->fireTime = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	result->fireTime = feature->fireTime * (1.0f / GAME_SPEED);
}

static void NativeGetFeatureSmokeTime(const GetFeatureSmokeTimeQuery* query, GetFeatureSmokeTimeResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->smokeTime = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	result->smokeTime = feature->smokeTime * (1.0f / GAME_SPEED);
}

static void NativeGetRenderFeatures(const GetRenderFeaturesQuery* query, GetRenderFeaturesResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->features = nullptr;
	result->count = 0;

	if (!IsReady() || featureDrawer == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const auto& features = featureDrawer->GetUnsortedFeatures();
	if (features.empty())
		return;

	int32_t* out = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxCount = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);

	for (const CFeature* feature : features) {
		if (count >= maxCount)
			break;
		if ((feature->drawFlag & query->drawMask) == 0)
			continue;

		out[count++] = feature->id;
	}

	result->features = out;
	bufferPos += count * sizeof(int32_t);
	(void)query->sendMask;
	result->count = count;
}

static void NativeGetRenderFeaturesDrawFlagChanged(const GetRenderFeaturesDrawFlagChangedQuery* query, GetRenderFeaturesDrawFlagChangedResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->features = nullptr;
	result->count = 0;

	if (!IsReady() || featureDrawer == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const auto& features = featureDrawer->GetUnsortedFeatures();

	if (features.empty())
		return;

	int32_t* out = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxCount = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);

	for (const CFeature* f : features) {
		if (count >= maxCount)
			break;

		if (f->previousDrawFlag == f->drawFlag)
			continue;

		out[count++] = f->id;
	}

	result->features = out;
	bufferPos += count * sizeof(int32_t);
	(void)query->sendMask;
	result->count = count;
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
	.ClearFeaturesPreviousDrawFlag = NativeClearFeaturesPreviousDrawFlag,
	.GetFeatureNoDraw = NativeGetFeatureNoDraw,
	.GetFeatureLuaDraw = NativeGetFeatureLuaDraw,
	.GetFeatureEngineDrawMask = NativeGetFeatureEngineDrawMask,
	.GetFeatureDrawFlag = NativeGetFeatureDrawFlag,
	.GetFeatureAlwaysUpdateMatrix = NativeGetFeatureAlwaysUpdateMatrix,
	.GetFeatureTransformMatrix = NativeGetFeatureTransformMatrix,
	.GetFeatureSelectionVolumeData = NativeGetFeatureSelectionVolumeData,
	.GetFeatureFireTime = NativeGetFeatureFireTime,
	.GetFeatureSmokeTime = NativeGetFeatureSmokeTime,
	.GetRenderFeatures = NativeGetRenderFeatures,
	.GetRenderFeaturesDrawFlagChanged = NativeGetRenderFeaturesDrawFlagChanged,
};
