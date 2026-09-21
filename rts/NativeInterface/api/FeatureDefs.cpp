#include "FeatureDefs.h"

#include "Sim/Features/FeatureDef.h"
#include "Sim/Features/FeatureDefHandler.h"

#include <vector>

namespace {

static thread_local std::vector<int32_t> featureDefIDs;
static thread_local std::vector<const char*> customParamKeys;

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "FeatureDef system not ready" };
static const Error INVALID_FEATUREDEF_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid feature def ID" };

static bool IsReady() {
	return (featureDefHandler != nullptr);
}

static void NativeGetFeatureDefIDs(const GetFeatureDefIDsQuery* query, GetFeatureDefIDsResult* result) {
	result->error = nullptr;
	result->ids = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const auto& defsVec = featureDefHandler->GetFeatureDefsVec();
	featureDefIDs.clear();
	featureDefIDs.reserve(defsVec.size() > 0 ? defsVec.size() - 1 : 0);
	for (size_t i = 1; i < defsVec.size(); i++) // Start at 1, 0 is invalid
		featureDefIDs.push_back(defsVec[i].id);

	result->ids = featureDefIDs.empty() ? nullptr : featureDefIDs.data();
	result->count = featureDefIDs.size();
}

static void NativeGetFeatureDefCount(const GetFeatureDefCountQuery* query, GetFeatureDefCountResult* result) {
	result->error = nullptr;
	result->count = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->count = featureDefHandler->NumFeatureDefs();
}

static void NativeGetFeatureDefByID(const GetFeatureDefByIDQuery* query, GetFeatureDefByIDResult* result) {
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
	result->error = nullptr;
	result->valid = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->valid = featureDefHandler->IsValidFeatureDefID(query->featureDefID);
}

static void NativeGetFeatureDefName(const GetFeatureDefNameQuery* query, GetFeatureDefNameResult* result) {
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

	customParamKeys.clear();
	customParamKeys.reserve(def->customParams.size());
	for (const auto& [key, value] : def->customParams)
		customParamKeys.push_back(key.c_str());

	result->keys = customParamKeys.empty() ? nullptr : customParamKeys.data();
	result->count = customParamKeys.size();
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
