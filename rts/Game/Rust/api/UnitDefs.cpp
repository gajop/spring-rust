#include "UnitDefs.h"

#include "Sim/Units/UnitDefHandler.h"

namespace {

// Scratch buffer
static thread_local char scratchBuffer[8192];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_IMPLEMENTED_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "UnitDefs API not yet fully implemented - stubs only" };

static void NativeGetUnitDefIDs(const GetUnitDefIDsQuery* query, GetUnitDefIDsResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->ids = nullptr;
	result->count = 0;
}

static void NativeGetUnitDefCount(const GetUnitDefCountQuery* query, GetUnitDefCountResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->count = static_cast<uint32_t>(unitDefHandler->NumUnitDefs());
}

static void NativeGetUnitDefByID(const GetUnitDefByIDQuery* query, GetUnitDefByIDResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->exists = false;
}

static void NativeGetUnitDefID(const GetUnitDefIDQuery* query, GetUnitDefIDResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->id = -1;
}

static void NativeValidUnitDefID(const ValidUnitDefIDQuery* query, ValidUnitDefIDResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->valid = false;
}

static void NativeGetUnitDefName(const GetUnitDefNameQuery* query, GetUnitDefNameResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->name = "";
}

static void NativeGetUnitDefHumanName(const GetUnitDefHumanNameQuery* query, GetUnitDefHumanNameResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->humanName = "";
}

static void NativeGetUnitDefCosts(const GetUnitDefCostsQuery* query, GetUnitDefCostsResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
}

static void NativeGetUnitDefSpeed(const GetUnitDefSpeedQuery* query, GetUnitDefSpeedResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->speed = 0.0f;
}

static void NativeGetUnitDefHealth(const GetUnitDefHealthQuery* query, GetUnitDefHealthResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->health = 0.0f;
}

static void NativeGetUnitDefCustomParam(const GetUnitDefCustomParamQuery* query, GetUnitDefCustomParamResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->value = "";
}

static void NativeGetUnitDefCustomParamKeys(const GetUnitDefCustomParamKeysQuery* query, GetUnitDefCustomParamKeysResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->keys = nullptr;
	result->count = 0;
}

} // namespace

const UnitDefsApi UNIT_DEFS_API = {
	.GetUnitDefIDs = NativeGetUnitDefIDs,
	.GetUnitDefCount = NativeGetUnitDefCount,
	.GetUnitDefByID = NativeGetUnitDefByID,
	.GetUnitDefID = NativeGetUnitDefID,
	.ValidUnitDefID = NativeValidUnitDefID,
	.GetUnitDefName = NativeGetUnitDefName,
	.GetUnitDefHumanName = NativeGetUnitDefHumanName,
	.GetUnitDefCosts = NativeGetUnitDefCosts,
	.GetUnitDefSpeed = NativeGetUnitDefSpeed,
	.GetUnitDefHealth = NativeGetUnitDefHealth,
	.GetUnitDefCustomParam = NativeGetUnitDefCustomParam,
	.GetUnitDefCustomParamKeys = NativeGetUnitDefCustomParamKeys,
};
