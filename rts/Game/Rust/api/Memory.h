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
// Note: With scratch buffer pattern, most arrays don't need explicit freeing
// as they're reused per-call. These functions are for special cases.
// ============================================================================

// Queries (for consistency, though most don't need results)
struct FreeStringArrayQuery {
	const char** data;
	uint32_t length;
};

struct FreeStringArrayResult {
	const Error* error;
};

struct FreeInt32ArrayQuery {
	int32_t* data;
	uint32_t length;
};

struct FreeInt32ArrayResult {
	const Error* error;
};

struct FreeUInt32ArrayQuery {
	uint32_t* data;
	uint32_t length;
};

struct FreeUInt32ArrayResult {
	const Error* error;
};

struct FreeFloatArrayQuery {
	float* data;
	uint32_t length;
};

struct FreeFloatArrayResult {
	const Error* error;
};

struct FreeFloat2ArrayQuery {
	Float2* data;
	uint32_t length;
};

struct FreeFloat2ArrayResult {
	const Error* error;
};

struct FreeFloat3ArrayQuery {
	Float3* data;
	uint32_t length;
};

struct FreeFloat3ArrayResult {
	const Error* error;
};

struct FreeFloat4ArrayQuery {
	Float4* data;
	uint32_t length;
};

struct FreeFloat4ArrayResult {
	const Error* error;
};

struct FreeInt3ArrayQuery {
	Int3* data;
	uint32_t length;
};

struct FreeInt3ArrayResult {
	const Error* error;
};

struct FreeQuery {
	void* ptr;
};

struct FreeResult {
	const Error* error;
};

struct MemoryApi {
	void (*FreeStringArray)(
		const FreeStringArrayQuery* query,
		FreeStringArrayResult* result
	);

	void (*FreeInt32Array)(
		const FreeInt32ArrayQuery* query,
		FreeInt32ArrayResult* result
	);

	void (*FreeUInt32Array)(
		const FreeUInt32ArrayQuery* query,
		FreeUInt32ArrayResult* result
	);

	void (*FreeFloatArray)(
		const FreeFloatArrayQuery* query,
		FreeFloatArrayResult* result
	);

	void (*FreeFloat2Array)(
		const FreeFloat2ArrayQuery* query,
		FreeFloat2ArrayResult* result
	);

	void (*FreeFloat3Array)(
		const FreeFloat3ArrayQuery* query,
		FreeFloat3ArrayResult* result
	);

	void (*FreeFloat4Array)(
		const FreeFloat4ArrayQuery* query,
		FreeFloat4ArrayResult* result
	);

	void (*FreeInt3Array)(
		const FreeInt3ArrayQuery* query,
		FreeInt3ArrayResult* result
	);

	void (*Free)(
		const FreeQuery* query,
		FreeResult* result
	);
};

extern const MemoryApi MEMORY_API;

#ifdef __cplusplus
}
#endif
