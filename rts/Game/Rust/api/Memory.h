#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Memory Management API
//
// Provides functions for Rust to deallocate memory allocated by C++ APIs
// All arrays returned by the API must be freed using these functions
// ============================================================================

struct MemoryApi {
	// Free string arrays (array of string pointers)
	// This frees both the array of pointers AND the individual strings
	void (*FreeStringArray)(const char** data, uint32_t length);

	// Free simple arrays (just the array itself, not contents)
	void (*FreeInt32Array)(int32_t* data, uint32_t length);
	void (*FreeUInt32Array)(uint32_t* data, uint32_t length);
	void (*FreeFloatArray)(float* data, uint32_t length);

	// Free structured arrays
	void (*FreeFloat2Array)(Float2* data, uint32_t length);
	void (*FreeFloat3Array)(Float3* data, uint32_t length);
	void (*FreeFloat4Array)(Float4* data, uint32_t length);
	void (*FreeInt3Array)(Int3* data, uint32_t length);

	// Generic free for opaque pointers
	void (*Free)(void* ptr);
};

extern const MemoryApi MEMORY_API;

#ifdef __cplusplus
}
#endif
