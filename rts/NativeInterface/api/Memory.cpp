#include "Memory.h"
#include <cstdlib>

namespace {

// NOTE: With scratch buffer pattern, these free functions are mostly unused.
// The scratch buffer is reused per-call, so no explicit freeing needed.
// These are kept for compatibility with any APIs that might allocate separately.

static void NativeFreeStringArray(const FreeStringArrayQuery* query, FreeStringArrayResult* result)
{
	if (query->data == nullptr) {
		result->error = nullptr;
		return;
	}

	// Free each individual string
	for (uint32_t i = 0; i < query->length; i++) {
		if (query->data[i] != nullptr) {
			std::free(const_cast<char*>(query->data[i]));
		}
	}

	// Free the array of pointers
	std::free(const_cast<char**>(query->data));
	result->error = nullptr;
}

static void NativeFreeInt32Array(const FreeInt32ArrayQuery* query, FreeInt32ArrayResult* result)
{
	std::free(query->data);
	result->error = nullptr;
}

static void NativeFreeUInt32Array(const FreeUInt32ArrayQuery* query, FreeUInt32ArrayResult* result)
{
	std::free(query->data);
	result->error = nullptr;
}

static void NativeFreeFloatArray(const FreeFloatArrayQuery* query, FreeFloatArrayResult* result)
{
	std::free(query->data);
	result->error = nullptr;
}

static void NativeFreeFloat2Array(const FreeFloat2ArrayQuery* query, FreeFloat2ArrayResult* result)
{
	std::free(query->data);
	result->error = nullptr;
}

static void NativeFreeFloat3Array(const FreeFloat3ArrayQuery* query, FreeFloat3ArrayResult* result)
{
	std::free(query->data);
	result->error = nullptr;
}

static void NativeFreeFloat4Array(const FreeFloat4ArrayQuery* query, FreeFloat4ArrayResult* result)
{
	std::free(query->data);
	result->error = nullptr;
}

static void NativeFreeInt3Array(const FreeInt3ArrayQuery* query, FreeInt3ArrayResult* result)
{
	std::free(query->data);
	result->error = nullptr;
}

static void NativeFree(const FreeQuery* query, FreeResult* result)
{
	std::free(query->ptr);
	result->error = nullptr;
}

} // namespace

const MemoryApi MEMORY_API = {
	.FreeStringArray = NativeFreeStringArray,
	.FreeInt32Array = NativeFreeInt32Array,
	.FreeUInt32Array = NativeFreeUInt32Array,
	.FreeFloatArray = NativeFreeFloatArray,
	.FreeFloat2Array = NativeFreeFloat2Array,
	.FreeFloat3Array = NativeFreeFloat3Array,
	.FreeFloat4Array = NativeFreeFloat4Array,
	.FreeInt3Array = NativeFreeInt3Array,
	.Free = NativeFree,
};
