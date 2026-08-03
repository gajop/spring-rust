#include "PathFinder.h"

#include "Sim/Path/IPathManager.h"
#include "Sim/MoveTypes/MoveDefHandler.h"
#include "Sim/Misc/GlobalSynced.h"
#include <vector>
#include <cstring>
#include <algorithm>

namespace {

// Scratch buffer
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Game not ready" };
static const Error INVALID_PATH_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid path ID" };
static const Error INVALID_MOVEDEF_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid move definition" };
static const Error BUFFER_OVERFLOW_ERROR = { .code = ERROR_BUFFER_OVERFLOW, .message = "Buffer overflow" };
static const Error INVALID_OVERLAY_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid overlay index" };
static const Error OVERLAY_EXISTS_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Overlay already exists" };
static const Error OVERLAY_EMPTY_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Overlay is empty" };

// Node cost overlay structure (similar to Lua implementation)
struct NodeCostOverlay {
	std::vector<float> costs;
	unsigned int sizeX = 0;
	unsigned int sizeZ = 0;

	void Init(unsigned int sx, unsigned int sz) {
		costs.resize(sx * sz, 0.0f);
		sizeX = sx;
		sizeZ = sz;
	}

	void Clear() {
		costs.clear();
		sizeX = 0;
		sizeZ = 0;
	}

	bool Empty() const { return costs.empty(); }
	unsigned int Size() const { return costs.size(); }
};

// Cost overlays storage [0] = synced, [1] = unsynced (mirroring Lua)
static std::vector<NodeCostOverlay> costOverlays[2];

static bool IsReady() {
	return (gs != nullptr && pathManager != nullptr);
}

// Initialize cost overlays on first use
static void EnsureCostOverlaysInit() {
	static bool initialized = false;
	if (!initialized) {
		costOverlays[0].resize(4);  // synced
		costOverlays[1].resize(4);  // unsynced
		initialized = true;
	}
}

// Helper to allocate from scratch buffer
template<typename T>
static T* AllocateArray(size_t count) {
	size_t needed = count * sizeof(T);
	if (bufferPos + needed > sizeof(scratchBuffer)) {
		return nullptr;
	}
	T* ptr = reinterpret_cast<T*>(&scratchBuffer[bufferPos]);
	bufferPos += needed;
	return ptr;
}

static void NativeRequestPath(const RequestPathQuery* query, RequestPathResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->pathID = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// Get MoveDef either by ID or name
	const MoveDef* moveDef = nullptr;
	if (query->hasMoveDefName) {
		moveDef = moveDefHandler.GetMoveDefByName(query->moveDefName);
	} else {
		// Lua accepts every path type in [0, GetNumMoveDefs()).  Zero is a
		// valid path type; treating it as a sentinel made the native call
		// diverge from Spring.RequestPath for the first movement definition.
		moveDef = moveDefHandler.GetMoveDefByPathType(query->moveDefID);
	}

	if (moveDef == nullptr) {
		// Lua returns no values when a string does not resolve to a move
		// definition.  Numeric path IDs are different: Lua raises for an
		// out-of-range ID, so preserve an error for that form.
		if (!query->hasMoveDefName)
			result->error = &INVALID_MOVEDEF_ERROR;
		return;
	}

	float3 startPos(query->startPos.x, query->startPos.y, query->startPos.z);
	float3 endPos(query->endPos.x, query->endPos.y, query->endPos.z);

	// Request path (synced=true, caller=nullptr since this is from Rust)
	result->pathID = pathManager->RequestPath(nullptr, moveDef, startPos, endPos, query->radius, true);
}

static void NativeDeletePath(const DeletePathQuery* query, DeletePathResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (query->pathID == 0) {
		result->error = &INVALID_PATH_ERROR;
		return;
	}

	pathManager->DeletePath(query->pathID);
	result->success = true;
}

static void NativeGetPathWayPoints(const GetPathWayPointsQuery* query, GetPathWayPointsResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->points = nullptr;
	result->pointCount = 0;
	result->starts = nullptr;
	result->startCount = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (query->pathID == 0) {
		result->error = &INVALID_PATH_ERROR;
		return;
	}

	std::vector<float3> points;
	std::vector<int> starts;

	pathManager->GetPathWayPoints(query->pathID, points, starts);

	if (!points.empty()) {
		result->points = AllocateArray<Float3>(points.size());
		if (result->points == nullptr) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			return;
		}

		for (size_t i = 0; i < points.size(); ++i) {
			result->points[i].x = points[i].x;
			result->points[i].y = points[i].y;
			result->points[i].z = points[i].z;
		}
		result->pointCount = static_cast<uint32_t>(points.size());
	}

	if (!starts.empty()) {
		result->starts = AllocateArray<int32_t>(starts.size());
		if (result->starts == nullptr) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			result->pointCount = 0;
			return;
		}

		for (size_t i = 0; i < starts.size(); ++i) {
			result->starts[i] = starts[i];
		}
		result->startCount = static_cast<uint32_t>(starts.size());
	}
}

static void NativeGetNextWayPoint(const GetNextWayPointQuery* query, GetNextWayPointResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->hasWaypoint = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (query->pathID == 0) {
		result->error = &INVALID_PATH_ERROR;
		return;
	}

	float3 callerPos(query->callerPos.x, query->callerPos.y, query->callerPos.z);
	float3 waypoint = pathManager->NextWayPoint(nullptr, query->pathID, 0, callerPos, query->minDist, true);

	// Check if waypoint is valid (not -1,-1,-1)
	if (waypoint.x >= 0.0f || waypoint.y >= 0.0f || waypoint.z >= 0.0f) {
		result->waypoint.x = waypoint.x;
		result->waypoint.y = waypoint.y;
		result->waypoint.z = waypoint.z;
		result->hasWaypoint = true;
	}
}

static void NativeInitPathNodeCostsArray(const InitPathNodeCostsArrayQuery* query, InitPathNodeCostsArrayResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	EnsureCostOverlaysInit();

	// Disallow creating empty overlays
	if (query->sizeX == 0 || query->sizeZ == 0) {
		result->error = &INVALID_OVERLAY_ERROR;
		return;
	}

	// For FFI we always use synced mode (true)
	const unsigned int syncedIdx = 0;
	std::vector<NodeCostOverlay>& overlays = costOverlays[syncedIdx];

	// Expand overlays array if needed
	if (query->overlayIndex >= overlays.size()) {
		overlays.resize(std::max(overlays.size() * 2, static_cast<size_t>(query->overlayIndex + 1)));
	}

	NodeCostOverlay& overlay = overlays[query->overlayIndex];

	// Disallow resizing existing overlays
	if (!overlay.Empty()) {
		result->error = &OVERLAY_EXISTS_ERROR;
		return;
	}

	overlay.Init(query->sizeX, query->sizeZ);
	result->success = true;
}

static void NativeFreePathNodeCostsArray(const FreePathNodeCostsArrayQuery* query, FreePathNodeCostsArrayResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	EnsureCostOverlaysInit();

	const unsigned int syncedIdx = 0;
	std::vector<NodeCostOverlay>& overlays = costOverlays[syncedIdx];

	// Not an existing overlay
	if (query->overlayIndex >= overlays.size()) {
		result->error = &INVALID_OVERLAY_ERROR;
		return;
	}

	NodeCostOverlay& overlay = overlays[query->overlayIndex];

	// Not an initialized overlay (already freed)
	if (overlay.Empty()) {
		result->error = &OVERLAY_EMPTY_ERROR;
		return;
	}

	// Nullify the active cost-overlay if we are freeing it
	if (pathManager->GetNodeExtraCosts(true) == &overlay.costs[0]) {
		pathManager->SetNodeExtraCosts(nullptr, 1, 1, true);
	}

	overlay.Clear();
	result->success = true;
}

static void NativeSetPathNodeCosts(const SetPathNodeCostsQuery* query, SetPathNodeCostsResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	EnsureCostOverlaysInit();

	const unsigned int syncedIdx = 0;
	std::vector<NodeCostOverlay>& overlays = costOverlays[syncedIdx];

	if (query->overlayIndex >= overlays.size()) {
		result->error = &INVALID_OVERLAY_ERROR;
		return;
	}

	NodeCostOverlay& overlay = overlays[query->overlayIndex];

	if (overlay.Empty()) {
		result->error = &OVERLAY_EMPTY_ERROR;
		return;
	}

	// Set the active cost-overlay to this overlay
	result->success = pathManager->SetNodeExtraCosts(&overlay.costs[0], overlay.sizeX, overlay.sizeZ, true);
}

static void NativeGetPathNodeCosts(const GetPathNodeCostsQuery* query, GetPathNodeCostsResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->costs = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	EnsureCostOverlaysInit();

	const unsigned int syncedIdx = 0;
	std::vector<NodeCostOverlay>& overlays = costOverlays[syncedIdx];

	if (query->overlayIndex >= overlays.size()) {
		result->error = &INVALID_OVERLAY_ERROR;
		return;
	}

	NodeCostOverlay& overlay = overlays[query->overlayIndex];

	if (overlay.Empty()) {
		result->error = &OVERLAY_EMPTY_ERROR;
		return;
	}

	// Copy costs to scratch buffer
	const size_t totalCosts = overlay.Size();
	const size_t bytesNeeded = totalCosts * sizeof(float);

	if (bufferPos + bytesNeeded > sizeof(scratchBuffer)) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	float* costsBuf = reinterpret_cast<float*>(scratchBuffer + bufferPos);
	memcpy(costsBuf, &overlay.costs[0], bytesNeeded);
	bufferPos += bytesNeeded;

	result->costs = costsBuf;
	result->count = static_cast<uint32_t>(totalCosts);
}

static void NativeSetPathNodeCost(const SetPathNodeCostQuery* query, SetPathNodeCostResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	EnsureCostOverlaysInit();

	const unsigned int syncedIdx = 0;
	std::vector<NodeCostOverlay>& overlays = costOverlays[syncedIdx];

	if (query->overlayIndex >= overlays.size()) {
		result->error = &INVALID_OVERLAY_ERROR;
		return;
	}

	NodeCostOverlay& overlay = overlays[query->overlayIndex];

	// Non-initialized array
	if (overlay.Empty()) {
		result->error = &OVERLAY_EMPTY_ERROR;
		return;
	}

	// Modify the cost-overlay at the specified index
	if (query->costIndex < overlay.Size()) {
		overlay.costs[query->costIndex] = query->cost;
		result->success = true;
	} else {
		result->error = &INVALID_OVERLAY_ERROR;
	}
}

static void NativeGetPathNodeCost(const GetPathNodeCostQuery* query, GetPathNodeCostResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->cost = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// Get cost from the ACTIVE overlay (reads from pathManager)
	// This retrieves from the currently set extra costs overlay
	result->cost = pathManager->GetNodeExtraCost(query->x, query->z, true);
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
