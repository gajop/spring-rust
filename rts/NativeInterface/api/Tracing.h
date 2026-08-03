#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Tracing API
// @see rts/Lua/LuaSyncedRead.cpp
//
// Ray tracing and collision testing
// ============================================================================

// Trace types
enum TraceFlags {
	TRACE_UNITS = (1 << 0),
	TRACE_FEATURES = (1 << 1),
	TRACE_GROUND = (1 << 2),
	TRACE_SKY = (1 << 3),
	TRACE_NO_ENEMY_UNITS = (1 << 4),
	TRACE_ONLY_ENEMY = (1 << 5),
};

// Ray definition
struct Ray {
	Float3 origin;
	Float3 direction;
	float length;
	uint32_t flags;  // TraceFlags bitfield
	int32_t allyTeamID;  // For LOS filtering
};

// Queries
struct TraceRayQuery {
	Ray ray;
};

struct TraceRayResult {
	const Error* error;
	bool hit;
	int32_t hitType;  // 0=none, 1=unit, 2=feature, 3=ground, 4=water
	int32_t hitID;    // Unit or feature ID
	Float3 hitPos;
	Float3 hitNormal;
};

struct TraceRayUnitsQuery {
	Ray ray;
};

struct TraceRayUnitsResult {
	const Error* error;
	bool hit;
	int32_t hitType;  // 0=none, 1=unit
	int32_t hitID;    // Unit ID
	Float3 hitPos;
	Float3 hitNormal;
};

struct TraceRayFeaturesQuery {
	Ray ray;
};

struct TraceRayFeaturesResult {
	const Error* error;
	bool hit;
	int32_t hitType;  // 0=none, 2=feature
	int32_t hitID;    // Feature ID
	Float3 hitPos;
	Float3 hitNormal;
};

// A hit returned by the multi-hit Lua ray tracing functions. objectType is
// 1 for a unit and 2 for a feature.
struct TraceRayHit {
	float hitLength;
	int32_t objectID;
	int32_t objectType;
};

struct TraceRayInDirectionQuery {
	Float3 pos;
	Float3 dir;
	float maxLength;
	bool hasMaxLength;
	const char* type;  // "unit", "feature", or "both"
};

struct TraceRayInDirectionResult {
	const Error* error;
	TraceRayHit* hits;
	uint32_t count;
};

struct TraceRayBetweenPositionsQuery {
	Float3 start;
	Float3 end;
	const char* type;  // "unit", "feature", or "both"
};

struct TraceRayBetweenPositionsResult {
	const Error* error;
	TraceRayHit* hits;
	uint32_t count;
};

struct TraceRayGroundBetweenPositionsQuery {
	Float3 start;
	Float3 end;
	bool testWater;
	bool hasTestWater;
};

struct TraceRayGroundBetweenPositionsResult {
	const Error* error;
	bool hit;
	float hitLength;
	Float3 hitPos;
	Float3 hitNormal;
};

struct TraceRayGroundInDirectionQuery {
	Float3 start;
	Float3 dir;
	float length;
	bool hasLength;
	bool testWater;
	bool hasTestWater;
};

struct TraceRayGroundInDirectionResult {
	const Error* error;
	bool hit;
	float hitLength;
	Float3 hitPos;
	Float3 hitNormal;
};

// API structure
struct TracingApi {
	void (*TraceRay)(
		const TraceRayQuery* query,
		TraceRayResult* result
	);

	void (*TraceRayUnits)(
		const TraceRayUnitsQuery* query,
		TraceRayUnitsResult* result
	);

	void (*TraceRayFeatures)(
		const TraceRayFeaturesQuery* query,
		TraceRayFeaturesResult* result
	);

	void (*TraceRayInDirection)(
		const TraceRayInDirectionQuery* query,
		TraceRayInDirectionResult* result
	);

	void (*TraceRayBetweenPositions)(
		const TraceRayBetweenPositionsQuery* query,
		TraceRayBetweenPositionsResult* result
	);

	void (*TraceRayGroundBetweenPositions)(
		const TraceRayGroundBetweenPositionsQuery* query,
		TraceRayGroundBetweenPositionsResult* result
	);

	void (*TraceRayGroundInDirection)(
		const TraceRayGroundInDirectionQuery* query,
		TraceRayGroundInDirectionResult* result
	);
};

extern const TracingApi TRACING_API;

#ifdef __cplusplus
}
#endif
