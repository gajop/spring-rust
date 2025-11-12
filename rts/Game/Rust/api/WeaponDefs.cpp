#include "WeaponDefs.h"

namespace {

// Scratch buffer
static thread_local char scratchBuffer[8192];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_IMPLEMENTED_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "WeaponDefs API not yet fully implemented - stubs only" };

static void NativeGetWeaponDefIDs(const GetWeaponDefIDsQuery* query, GetWeaponDefIDsResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->ids = nullptr;
	result->count = 0;
}

static void NativeGetWeaponDefCount(const GetWeaponDefCountQuery* query, GetWeaponDefCountResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->count = 0;
}

static void NativeGetWeaponDefByID(const GetWeaponDefByIDQuery* query, GetWeaponDefByIDResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->exists = false;
}

static void NativeGetWeaponDefID(const GetWeaponDefIDQuery* query, GetWeaponDefIDResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->id = -1;
}

static void NativeValidWeaponDefID(const ValidWeaponDefIDQuery* query, ValidWeaponDefIDResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->valid = false;
}

static void NativeGetWeaponDefName(const GetWeaponDefNameQuery* query, GetWeaponDefNameResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->name = "";
}

static void NativeGetWeaponDefRange(const GetWeaponDefRangeQuery* query, GetWeaponDefRangeResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->range = 0.0f;
}

static void NativeGetWeaponDefDamage(const GetWeaponDefDamageQuery* query, GetWeaponDefDamageResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->damage = 0.0f;
}

static void NativeGetWeaponDefCustomParam(const GetWeaponDefCustomParamQuery* query, GetWeaponDefCustomParamResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->value = "";
}

static void NativeGetWeaponDefCustomParamKeys(const GetWeaponDefCustomParamKeysQuery* query, GetWeaponDefCustomParamKeysResult* result) {
	bufferPos = 0;
	result->error = &NOT_IMPLEMENTED_ERROR;
	result->keys = nullptr;
	result->count = 0;
}

} // namespace

const WeaponDefsApi WEAPON_DEFS_API = {
	.GetWeaponDefIDs = NativeGetWeaponDefIDs,
	.GetWeaponDefCount = NativeGetWeaponDefCount,
	.GetWeaponDefByID = NativeGetWeaponDefByID,
	.GetWeaponDefID = NativeGetWeaponDefID,
	.ValidWeaponDefID = NativeValidWeaponDefID,
	.GetWeaponDefName = NativeGetWeaponDefName,
	.GetWeaponDefRange = NativeGetWeaponDefRange,
	.GetWeaponDefDamage = NativeGetWeaponDefDamage,
	.GetWeaponDefCustomParam = NativeGetWeaponDefCustomParam,
	.GetWeaponDefCustomParamKeys = NativeGetWeaponDefCustomParamKeys,
};
