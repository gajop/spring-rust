#include "FeatureDefs.h"

namespace {

// Scratch buffer
static thread_local char scratchBuffer[8192];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_IMPLEMENTED_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "FeatureDefs API not yet fully implemented - stubs only" };

static void NativeGetFeatureDefIDs(const GetFeatureDefIDsQuery* query, GetFeatureDefIDsResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->ids = nullptr;
	result->count = 0;
}

static void NativeGetFeatureDefCount(const GetFeatureDefCountQuery* query, GetFeatureDefCountResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->count = 0;
}

static void NativeGetFeatureDefByID(const GetFeatureDefByIDQuery* query, GetFeatureDefByIDResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->exists = false;
}

static void NativeGetFeatureDefID(const GetFeatureDefIDQuery* query, GetFeatureDefIDResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->id = -1;
}

static void NativeValidFeatureDefID(const ValidFeatureDefIDQuery* query, ValidFeatureDefIDResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->valid = false;
}

static void NativeGetFeatureDefName(const GetFeatureDefNameQuery* query, GetFeatureDefNameResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->name = "";
}

static void NativeGetFeatureDefMetal(const GetFeatureDefMetalQuery* query, GetFeatureDefMetalResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->metal = 0.0f;
}

static void NativeGetFeatureDefEnergy(const GetFeatureDefEnergyQuery* query, GetFeatureDefEnergyResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->energy = 0.0f;
}

static void NativeGetFeatureDefCustomParam(const GetFeatureDefCustomParamQuery* query, GetFeatureDefCustomParamResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->value = "";
}

static void NativeGetFeatureDefCustomParamKeys(const GetFeatureDefCustomParamKeysQuery* query, GetFeatureDefCustomParamKeysResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->keys = nullptr;
	result->count = 0;
}

} // namespace

const FeatureDefsApi FEATURE_DEFS_API = {
	.GetFeatureDefIDs = NativeGetFeatureDefIDs,
	.GetFeatureDefCount = NativeGetFeatureDefCount,
	.GetFeatureDefByID = NativeGetFeatureDefByID,
	.GetFeatureDefID = NativeGetFeatureDefID,
	.ValidFeatureDefID = NativeValidFeatureDefID,
	.GetFeatureDefName = NativeGetFeatureDefName,
	.GetFeatureDefMetal = NativeGetFeatureDefMetal,
	.GetFeatureDefEnergy = NativeGetFeatureDefEnergy,
	.GetFeatureDefCustomParam = NativeGetFeatureDefCustomParam,
	.GetFeatureDefCustomParamKeys = NativeGetFeatureDefCustomParamKeys,
};
