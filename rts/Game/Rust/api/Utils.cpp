#include "Utils.h"

#include "Sim/Projectiles/ExplosionGenerator.h"
#include "Sim/Units/UnitDefHandler.h"
#include "Sim/Units/BuildInfo.h"
#include "Sim/Features/Feature.h"
#include "Game/GameHelper.h"
#include "System/float3.h"

namespace {

// Error constants
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
static CEGIDResult NativeGetCEGID(const char* cegName)
{
	CEGIDResult result = {};
	if (cegName == nullptr || cegName[0] == '\0') {
		result.cegID = -1;
		return result;
	}

	result.cegID = static_cast<int32_t>(explGenHandler.LoadCustomGeneratorID(cegName));
	return result;
}

// Build testing
static TestBuildOrderResult NativeTestBuildOrder(int32_t unitDefID, Float3 pos, int32_t facing)
{
	TestBuildOrderResult result = {};
	result.canBuild = false;
	result.feature = -1;

	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const UnitDef* unitDef = unitDefHandler->GetUnitDefByID(unitDefID);
	if (unitDef == nullptr) {
		result.error = &INVALID_UNITDEF_ERROR;
		return result;
	}

	BuildInfo bi;
	bi.def = unitDef;
	bi.pos = float3(pos.x, pos.y, pos.z);
	bi.buildFacing = facing;
	bi.pos = CGameHelper::Pos2BuildPos(bi, true); // synced=true

	CFeature* feature = nullptr;

	// Negative allyTeam = full visibility
	// Return values: 0=blocked, 1=occupied, 2=reclaimable, 3=open
	const int status = CGameHelper::TestUnitBuildSquare(bi, feature, -1, true);

	// Keep backward compatibility: map OPEN to RECLAIMABLE
	const int mappedStatus = (status == CGameHelper::BUILDSQUARE_OPEN) ?
		CGameHelper::BUILDSQUARE_RECLAIMABLE : status;

	result.canBuild = (mappedStatus >= CGameHelper::BUILDSQUARE_RECLAIMABLE);
	result.feature = (feature != nullptr) ? feature->id : -1;

	return result;
}

static Float3Result NativePos2BuildPos(BuildPosQuery query)
{
	Float3Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const UnitDef* unitDef = unitDefHandler->GetUnitDefByID(query.unitDefID);
	if (unitDef == nullptr) {
		result.error = &INVALID_UNITDEF_ERROR;
		return result;
	}

	BuildInfo bi;
	bi.def = unitDef;
	bi.pos = float3(query.pos.x, query.pos.y, query.pos.z);
	bi.buildFacing = query.facing;

	const float3 buildPos = CGameHelper::Pos2BuildPos(bi, true); // synced=true

	result.value.x = buildPos.x;
	result.value.y = buildPos.y;
	result.value.z = buildPos.z;
	return result;
}

static Float3Result NativeClosestBuildPos(int32_t teamID, int32_t unitDefID, Float3 pos, float searchRadius, int32_t minDist, int32_t facing)
{
	Float3Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const UnitDef* unitDef = unitDefHandler->GetUnitDefByID(unitDefID);
	if (unitDef == nullptr) {
		result.error = &INVALID_UNITDEF_ERROR;
		return result;
	}

	const float3 worldPos(pos.x, pos.y, pos.z);
	const float3 buildPos = CGameHelper::ClosestBuildPos(teamID, unitDef, worldPos, searchRadius, minDist, facing, true);

	result.value.x = buildPos.x;
	result.value.y = buildPos.y;
	result.value.z = buildPos.z;
	return result;
}

// Move testing
static TestMoveOrderResult NativeTestMoveOrder(int32_t unitDefID, Float3 pos)
{
	TestMoveOrderResult result = {};
	result.canMove = false;

	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const UnitDef* unitDef = unitDefHandler->GetUnitDefByID(unitDefID);
	if (unitDef == nullptr || unitDef->pathType == -1u) {
		result.canMove = false;
		return result;
	}

	// Simplified: just check if unit has a valid path type
	// Full implementation would query pathfinder
	result.canMove = true;
	return result;
}

// Unit dimensions
static Float3Result NativeGetUnitDefDimensions(int32_t unitDefID)
{
	Float3Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	const UnitDef* unitDef = unitDefHandler->GetUnitDefByID(unitDefID);
	if (unitDef == nullptr) {
		result.error = &INVALID_UNITDEF_ERROR;
		return result;
	}

	// Return model dimensions
	const S3DModel* model = unitDef->model;
	if (model != nullptr) {
		const float3 mid = (model->maxs + model->mins) * 0.5f;
		result.value.x = mid.x;
		result.value.y = mid.y;
		result.value.z = mid.z;
	} else {
		result.value.x = 0.0f;
		result.value.y = 0.0f;
		result.value.z = 0.0f;
	}

	return result;
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
