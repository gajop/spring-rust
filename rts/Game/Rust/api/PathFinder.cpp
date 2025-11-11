#include "PathFinder.h"

#include "Sim/Path/IPathManager.h"
#include <vector>

namespace {

// Error constants
static const Error NOT_IMPLEMENTED_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "PathFinder API not yet fully implemented"
};

// Request a path
static PathResult NativeRequestPath(PathRequest request)
{
	PathResult result = {};
	result.pathID = 0; // No path (not implemented)
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

// Delete a path
static BoolResult NativeDeletePath(uint32_t pathID)
{
	BoolResult result = {};
	result.value = false;
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

// Get all waypoints for a path
static PathWayPointsResult NativeGetPathWayPoints(uint32_t pathID)
{
	PathWayPointsResult result = {};
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

// Get next waypoint from current position
static NextWayPointResult NativeGetNextWayPoint(NextWayPointRequest request)
{
	NextWayPointResult result = {};
	result.hasWaypoint = false;
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

// Initialize a node cost overlay array
static BoolResult NativeInitPathNodeCostsArray(NodeCostOverlayInit init)
{
	BoolResult result = {};
	result.value = false;
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

// Free a node cost overlay array
static BoolResult NativeFreePathNodeCostsArray(uint32_t overlayIndex)
{
	BoolResult result = {};
	result.value = false;
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

// Set the active node cost overlay
static BoolResult NativeSetPathNodeCosts(NodeCostOverlaySet set)
{
	BoolResult result = {};
	result.value = false;
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

// Get all costs from an overlay
static FloatArray NativeGetPathNodeCosts(uint32_t overlayIndex)
{
	FloatArray result = {};
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

// Set a specific node cost in an overlay
static BoolResult NativeSetPathNodeCost(NodeCostSet set)
{
	BoolResult result = {};
	result.value = false;
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
}

// Get the cost of a specific node from the active overlay
static FloatResult NativeGetPathNodeCost(NodeCostGet get)
{
	FloatResult result = {};
	result.value = 0.0f;
	result.error = &NOT_IMPLEMENTED_ERROR;
	return result;
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
