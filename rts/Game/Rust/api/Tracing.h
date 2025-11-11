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

// Trace result
struct TraceResult {
	const Error* error;
	bool hit;
	int32_t hitType;  // 0=none, 1=unit, 2=feature, 3=ground, 4=water
	int32_t hitID;    // Unit or feature ID
	Float3 hitPos;
	Float3 hitNormal;
};

// Ray definition
struct Ray {
	Float3 origin;
	Float3 direction;
	float length;
	uint32_t flags;  // TraceFlags bitfield
	int32_t allyTeamID;  // For LOS filtering
};

// Ground trace
struct GroundTraceQuery {
	Float3 start;
	Float3 end;
};

// API structure
struct TracingApi {
	// General ray trace (not yet implemented in Lua)
	TraceResult (*TraceRay)(Ray ray);

	// Unit-only trace (not yet implemented)
	TraceResult (*TraceRayUnits)(Ray ray);

	// Feature-only trace (not yet implemented)
	TraceResult (*TraceRayFeatures)(Ray ray);

	// Ground trace between positions
	TraceResult (*TraceRayGroundBetweenPositions)(GroundTraceQuery query);

	// Ground trace in direction
	TraceResult (*TraceRayGroundInDirection)(Float3 start, Float3 dir, float length);
};

extern const TracingApi TRACING_API;

#ifdef __cplusplus
}
#endif
