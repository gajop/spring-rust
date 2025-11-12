#include "PathFinder.h"

#include "Sim/Path/IPathManager.h"
#include <vector>

namespace {

// Scratch buffer
static thread_local char scratchBuffer[8192];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_IMPLEMENTED_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "PathFinder API not yet fully implemented" };

static void NativeRequestPath(const RequestPathQuery* query, RequestPathResult* result) {
	bufferPos = 0;
	result->pathID = 0; // No path (not implemented)
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeDeletePath(const DeletePathQuery* query, DeletePathResult* result) {
	bufferPos = 0;
	result->success = false;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetPathWayPoints(const GetPathWayPointsQuery* query, GetPathWayPointsResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->points = nullptr;
	result->pointCount = 0;
	result->starts = nullptr;
	result->startCount = 0;
}

static void NativeGetNextWayPoint(const GetNextWayPointQuery* query, GetNextWayPointResult* result) {
	bufferPos = 0;
	result->hasWaypoint = false;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeInitPathNodeCostsArray(const InitPathNodeCostsArrayQuery* query, InitPathNodeCostsArrayResult* result) {
	bufferPos = 0;
	result->success = false;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeFreePathNodeCostsArray(const FreePathNodeCostsArrayQuery* query, FreePathNodeCostsArrayResult* result) {
	bufferPos = 0;
	result->success = false;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeSetPathNodeCosts(const SetPathNodeCostsQuery* query, SetPathNodeCostsResult* result) {
	bufferPos = 0;
	result->success = false;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetPathNodeCosts(const GetPathNodeCostsQuery* query, GetPathNodeCostsResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->costs = nullptr;
	result->count = 0;
}

static void NativeSetPathNodeCost(const SetPathNodeCostQuery* query, SetPathNodeCostResult* result) {
	bufferPos = 0;
	result->success = false;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetPathNodeCost(const GetPathNodeCostQuery* query, GetPathNodeCostResult* result) {
	bufferPos = 0;
	result->cost = 0.0f;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

} // namespace

const PathFinderApi PATH_FINDER_API = {
	.RequestPath = NativeRequestPath,
	.DeletePath = NativeDeletePath,
	.GetPathWayPoints = NativeGetPathWayPoints,
	.GetNextWayPoint = NativeGetNextWayPoint,
	.InitPathNodeCostsArray = NativeInitPathNodeCostsArray,
	.FreePathNodeCostsArray = NativeFreePathNodeCostsArray,
	.SetPathNodeCosts = NativeSetPathNodeCosts,
	.GetPathNodeCosts = NativeGetPathNodeCosts,
	.SetPathNodeCost = NativeSetPathNodeCost,
	.GetPathNodeCost = NativeGetPathNodeCost,
};
