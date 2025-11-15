#include "FeatureDefs.h"

#include "Sim/Features/FeatureDef.h"
#include "Sim/Features/FeatureDefHandler.h"

namespace {

// Scratch buffer
static thread_local char scratchBuffer[8192];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "FeatureDef system not ready" };
static const Error INVALID_FEATUREDEF_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid feature def ID" };

static bool IsReady() {
	return (featureDefHandler != nullptr);
}

static void NativeGetFeatureDefIDs(const GetFeatureDefIDsQuery* query, GetFeatureDefIDsResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->ids = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	// Use scratch buffer for array
	int32_t* ids = reinterpret_cast<int32_t*>(scratchBuffer + bufferPos);
	uint32_t count = 0;
	const size_t maxIds = (sizeof(scratchBuffer) - bufferPos) / sizeof(int32_t);

	const auto& defsVec = featureDefHandler->GetFeatureDefsVec();
	for (size_t i = 1; i < defsVec.size() && count < maxIds; i++) { // Start at 1, 0 is invalid
		ids[count++] = defsVec[i].id;
	}

	result->ids = ids;
	result->count = count;
	bufferPos += count * sizeof(int32_t);
}

static void NativeGetFeatureDefCount(const GetFeatureDefCountQuery* query, GetFeatureDefCountResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->count = featureDefHandler->NumFeatureDefs();
}

static void NativeGetFeatureDefByID(const GetFeatureDefByIDQuery* query, GetFeatureDefByIDResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->exists = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const FeatureDef* def = featureDefHandler->GetFeatureDefByID(query->featureDefID);
	if (def == nullptr) {
		return; // exists = false
	}

	result->exists = true;
	result->info.id = def->id;
	result->info.name = def->name.c_str();
	result->info.description = def->description.c_str();
	result->info.tooltip = def->description.c_str(); // FeatureDef doesn't have separate tooltip
	result->info.metal = def->cost.metal;
	result->info.energy = def->cost.energy;
	result->info.maxHealth = def->health;
	result->info.reclaimTime = def->reclaimTime;
	result->info.mass = def->mass;
	result->info.destructable = def->destructable;
	result->info.reclaimable = def->reclaimable;
	result->info.blocking = def->collidable;
	result->info.burnable = def->burnable;
	result->info.floating = def->floating;
	result->info.geoThermal = def->geoThermal;
	result->info.modelName = def->modelName.c_str();
	result->info.resurrectAs = ""; // Would need to look up unit def by ID
}

static void NativeGetFeatureDefIDByName(const GetFeatureDefIDByNameQuery* query, GetFeatureDefIDByNameResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->id = -1;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const FeatureDef* def = featureDefHandler->GetFeatureDef(query->featureDefName, false);
	if (def != nullptr) {
		result->id = def->id;
	}
}

static void NativeValidFeatureDefID(const ValidFeatureDefIDQuery* query, ValidFeatureDefIDResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->valid = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->valid = featureDefHandler->IsValidFeatureDefID(query->featureDefID);
}

static void NativeGetFeatureDefName(const GetFeatureDefNameQuery* query, GetFeatureDefNameResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->name = "";

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const FeatureDef* def = featureDefHandler->GetFeatureDefByID(query->featureDefID);
	if (def == nullptr) {
		result->error = &INVALID_FEATUREDEF_ERROR;
		return;
	}

	result->name = def->name.c_str();
}

static void NativeGetFeatureDefMetal(const GetFeatureDefMetalQuery* query, GetFeatureDefMetalResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->metal = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const FeatureDef* def = featureDefHandler->GetFeatureDefByID(query->featureDefID);
	if (def == nullptr) {
		result->error = &INVALID_FEATUREDEF_ERROR;
		return;
	}

	result->metal = def->cost.metal;
}

static void NativeGetFeatureDefEnergy(const GetFeatureDefEnergyQuery* query, GetFeatureDefEnergyResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->energy = 0.0f;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const FeatureDef* def = featureDefHandler->GetFeatureDefByID(query->featureDefID);
	if (def == nullptr) {
		result->error = &INVALID_FEATUREDEF_ERROR;
		return;
	}

	result->energy = def->cost.energy;
}

static void NativeGetFeatureDefCustomParam(const GetFeatureDefCustomParamQuery* query, GetFeatureDefCustomParamResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->value = "";

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const FeatureDef* def = featureDefHandler->GetFeatureDefByID(query->featureDefID);
	if (def == nullptr) {
		result->error = &INVALID_FEATUREDEF_ERROR;
		return;
	}

	const auto& params = def->customParams;
	auto it = params.find(query->key);
	if (it != params.end()) {
		result->value = it->second.c_str();
	}
}

static void NativeGetFeatureDefCustomParamKeys(const GetFeatureDefCustomParamKeysQuery* query, GetFeatureDefCustomParamKeysResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->keys = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const FeatureDef* def = featureDefHandler->GetFeatureDefByID(query->featureDefID);
	if (def == nullptr) {
		result->error = &INVALID_FEATUREDEF_ERROR;
		return;
	}

	const auto& params = def->customParams;
	const size_t maxKeys = (sizeof(scratchBuffer) - bufferPos) / sizeof(const char*);

	const char** keys = reinterpret_cast<const char**>(scratchBuffer + bufferPos);
	uint32_t count = 0;

	for (const auto& [key, value] : params) {
		if (count < maxKeys) {
			keys[count++] = key.c_str();
		}
	}

	result->keys = keys;
	result->count = count;
	bufferPos += count * sizeof(const char*);
}

} // namespace

const FeatureDefsApi FEATURE_DEFS_API = {
	.GetFeatureDefIDs = NativeGetFeatureDefIDs,
	.GetFeatureDefCount = NativeGetFeatureDefCount,
	.GetFeatureDefByID = NativeGetFeatureDefByID,
	.GetFeatureDefIDByName = NativeGetFeatureDefIDByName,
	.ValidFeatureDefID = NativeValidFeatureDefID,
	.GetFeatureDefName = NativeGetFeatureDefName,
	.GetFeatureDefMetal = NativeGetFeatureDefMetal,
	.GetFeatureDefEnergy = NativeGetFeatureDefEnergy,
	.GetFeatureDefCustomParam = NativeGetFeatureDefCustomParam,
	.GetFeatureDefCustomParamKeys = NativeGetFeatureDefCustomParamKeys,
};
