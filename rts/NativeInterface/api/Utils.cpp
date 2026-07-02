#include "Utils.h"

#include "Sim/Projectiles/ExplosionGenerator.h"
#include "Sim/Units/UnitDefHandler.h"
#include "Sim/Units/BuildInfo.h"
#include "Sim/Features/Feature.h"
#include "Game/GameHelper.h"
#include "Rendering/Models/3DModel.hpp"
#include "System/float3.h"

namespace {

// Scratch buffer for dynamic data
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "System not ready"
};

static const Error INVALID_UNITDEF_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid unit definition ID"
};

// Helper: check if ready
static bool IsReady()
{
	return (unitDefHandler != nullptr);
}

// CEG
static void NativeGetCEGID(const GetCEGIDQuery* query, GetCEGIDResult* result)
{
	bufferPos = 0;

	if (query->cegName == nullptr || query->cegName[0] == '\0') {
		result->error = nullptr;
		result->cegID = -1;
		return;
	}

	result->error = nullptr;
	result->cegID = static_cast<int32_t>(explGenHandler.LoadCustomGeneratorID(query->cegName));
}

// Build testing
static void NativeTestBuildOrder(const TestBuildOrderQuery* query, TestBuildOrderResult* result)
{
	bufferPos = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const UnitDef* unitDef = unitDefHandler->GetUnitDefByID(query->unitDefID);
	if (unitDef == nullptr) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}

	BuildInfo bi;
	bi.def = unitDef;
	bi.pos = float3(query->pos.x, query->pos.y, query->pos.z);
	bi.buildFacing = query->facing;
	bi.pos = CGameHelper::Pos2BuildPos(bi, true); // synced=true

	CFeature* feature = nullptr;

	// Return values: 0=blocked, 1=occupied, 2=reclaimable, 3=open
	const int status = CGameHelper::TestUnitBuildSquare(bi, feature, 0, true);

	// Keep backward compatibility: map OPEN to RECLAIMABLE
	const int mappedStatus = (status == CGameHelper::BUILDSQUARE_OPEN) ?
		CGameHelper::BUILDSQUARE_RECLAIMABLE : status;

	result->error = nullptr;
	result->status = mappedStatus;
	result->canBuild = (mappedStatus >= CGameHelper::BUILDSQUARE_RECLAIMABLE);
	result->feature = (feature != nullptr) ? feature->id : -1;
}

static void NativePos2BuildPos(const Pos2BuildPosQuery* query, Pos2BuildPosResult* result)
{
	bufferPos = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const UnitDef* unitDef = unitDefHandler->GetUnitDefByID(query->unitDefID);
	if (unitDef == nullptr) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}

	BuildInfo bi;
	bi.def = unitDef;
	bi.pos = float3(query->pos.x, query->pos.y, query->pos.z);
	bi.buildFacing = query->facing;

	const float3 buildPos = CGameHelper::Pos2BuildPos(bi, true); // synced=true

	result->error = nullptr;
	result->buildPos.x = buildPos.x;
	result->buildPos.y = buildPos.y;
	result->buildPos.z = buildPos.z;
}

static void NativeClosestBuildPos(const ClosestBuildPosQuery* query, ClosestBuildPosResult* result)
{
	bufferPos = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const UnitDef* unitDef = unitDefHandler->GetUnitDefByID(query->unitDefID);
	if (unitDef == nullptr) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}

	const float3 worldPos(query->pos.x, query->pos.y, query->pos.z);
	const float3 buildPos = CGameHelper::ClosestBuildPos(query->teamID, unitDef, worldPos, query->searchRadius, query->minDist, query->facing, true);

	result->error = nullptr;
	result->buildPos.x = buildPos.x;
	result->buildPos.y = buildPos.y;
	result->buildPos.z = buildPos.z;
}

// Move testing
static void NativeTestMoveOrder(const TestMoveOrderQuery* query, TestMoveOrderResult* result)
{
	bufferPos = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const UnitDef* unitDef = unitDefHandler->GetUnitDefByID(query->unitDefID);
	if (unitDef == nullptr || unitDef->pathType == -1u) {
		result->error = nullptr;
		result->canMove = false;
		return;
	}

	// Simplified: just check if unit has a valid path type
	// Full implementation would query pathfinder
	result->error = nullptr;
	result->canMove = true;
}

// Unit dimensions
static void NativeGetUnitDefDimensions(const GetUnitDefDimensionsQuery* query, GetUnitDefDimensionsResult* result)
{
	bufferPos = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const UnitDef* unitDef = unitDefHandler->GetUnitDefByID(query->unitDefID);
	if (unitDef == nullptr) {
		result->error = &INVALID_UNITDEF_ERROR;
		return;
	}

	const S3DModel* model = unitDef->LoadModel();
	result->error = nullptr;
	if (model != nullptr) {
		const float3& mid = model->relMidPos;
		result->dimensions.height = model->height;
		result->dimensions.radius = model->radius;
		result->dimensions.midx = mid.x;
		result->dimensions.minx = model->mins.x;
		result->dimensions.maxx = model->maxs.x;
		result->dimensions.midy = mid.y;
		result->dimensions.miny = model->mins.y;
		result->dimensions.maxy = model->maxs.y;
		result->dimensions.midz = mid.z;
		result->dimensions.minz = model->mins.z;
		result->dimensions.maxz = model->maxs.z;
	} else {
		result->dimensions = {};
	}
}

} // namespace

const UtilsApi UTILS_API = {
	.GetCEGID = NativeGetCEGID,

	.TestBuildOrder = NativeTestBuildOrder,
	.Pos2BuildPos = NativePos2BuildPos,
	.ClosestBuildPos = NativeClosestBuildPos,

	.TestMoveOrder = NativeTestMoveOrder,

	.GetUnitDefDimensions = NativeGetUnitDefDimensions,
};
