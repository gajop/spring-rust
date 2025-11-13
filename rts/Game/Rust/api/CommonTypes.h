#pragma once

#include <stdint.h>
#include "Common.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Common Vector Types
// ============================================================================

struct Float2 {
	float x;
	float y;
};

struct Float3 {
	float x;
	float y;
	float z;
};

struct Float4 {
	float x;
	float y;
	float z;
	float w;
};

struct Int2 {
	int32_t x;
	int32_t y;
};

struct Int3 {
	int32_t x;
	int32_t y;
	int32_t z;
};

// Collision volume data (used by units and features)
struct CollisionVolumeData {
	float scaleX;
	float scaleY;
	float scaleZ;
	float offsetX;
	float offsetY;
	float offsetZ;
	int32_t volumeType;  // 0=ellipsoid, 1=cylinder, 2=box
	int32_t testType;    // Collision test type
	int32_t primaryAxis; // For cylinders
	bool disabled;
};

// ============================================================================
// Dynamic Array Types
// ============================================================================

struct FloatArray {
	const Error* error;
	float* data;
	uint32_t length;
};

struct Int32Array {
	const Error* error;
	int32_t* data;
	uint32_t length;
};

struct UInt32Array {
	const Error* error;
	uint32_t* data;
	uint32_t length;
};

struct Float3Array {
	const Error* error;
	Float3* data;
	uint32_t length;
};

struct StringArray {
	const Error* error;
	const char** data;
	uint32_t length;
};

// ============================================================================
// String Result Type
// ============================================================================

struct StringResult {
	const Error* error;
	const char* value;
};

// ============================================================================
// Boolean Result Type
// ============================================================================

struct BoolResult {
	const Error* error;
	bool value;
};

struct Int32Result {
	const Error* error;
	int32_t value;
};

struct UInt32Result {
	const Error* error;
	uint32_t value;
};

struct FloatResult {
	const Error* error;
	float value;
};

struct Float2Result {
	const Error* error;
	Float2 value;
};

struct Float3Result {
	const Error* error;
	Float3 value;
};

struct Float4Result {
	const Error* error;
	Float4 value;
};

// ============================================================================
// Common Error Codes
// ============================================================================

enum CommonErrorCode {
	ERROR_NONE = 0,
	ERROR_INVALID_ARGUMENT = 1,
	ERROR_OUT_OF_BOUNDS = 2,
	ERROR_NOT_FOUND = 3,
	ERROR_NOT_AVAILABLE = 4,
	ERROR_INVALID_STATE = 5,
	ERROR_PERMISSION_DENIED = 6,
	ERROR_ALREADY_EXISTS = 7,
	ERROR_OPERATION_FAILED = 8,
	ERROR_BUFFER_OVERFLOW = 9,
	ERROR_INVALID_ID = 10,
	ERROR_INTERNAL = 999
};

#ifdef __cplusplus
}
#endif
