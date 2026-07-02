/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

struct GetTimerQuery { uint8_t _unused; };
struct GetTimerResult { const Error* error; uint64_t timer; };

struct GetTimerMicrosQuery { uint8_t _unused; };
struct GetTimerMicrosResult { const Error* error; uint64_t timer; };

struct DiffTimersQuery { uint64_t endTimer; uint64_t startTimer; bool returnMs; bool fromMicroSecs; };
struct DiffTimersResult { const Error* error; float seconds; };

struct GetFrameTimerQuery { bool lastFrameTime; };
struct GetFrameTimerResult { const Error* error; uint64_t timer; };

struct GetDrawSecondsQuery { uint8_t _unused; };
struct GetDrawSecondsResult { const Error* error; float seconds; };

struct GetProfilerTimeRecordQuery { const char* name; bool includeFrameData; };
struct GetProfilerTimeRecordResult { const Error* error; float totalMs; float currentMs; float maxDt; float timePct; float peakPct; const float* frameData; uint32_t frameCount; };

struct GetProfilerRecordNamesQuery { uint8_t _unused; };
struct GetProfilerRecordNamesResult { const Error* error; const char** names; uint32_t count; };

struct GetLuaMemUsageQuery { uint8_t _unused; };
struct GetLuaMemUsageResult { const Error* error; float handleAllocedKB; float handleAllocsK; float globalAllocedKB; float globalAllocsK; float unsyncedAllocedKB; float unsyncedAllocsK; float syncedAllocedKB; float syncedAllocsK; };

struct GetVidMemUsageQuery { uint8_t _unused; };
struct GetVidMemUsageResult { const Error* error; float usedMB; float availableMB; };

struct GetSyncedGCInfoQuery { bool collect; };
struct GetSyncedGCInfoResult { const Error* error; float gcKB; };

struct ProfilingApi {
	void (*GetTimer)(const GetTimerQuery* query, GetTimerResult* result);
	void (*GetTimerMicros)(const GetTimerMicrosQuery* query, GetTimerMicrosResult* result);
	void (*DiffTimers)(const DiffTimersQuery* query, DiffTimersResult* result);
	void (*GetFrameTimer)(const GetFrameTimerQuery* query, GetFrameTimerResult* result);
	void (*GetDrawSeconds)(const GetDrawSecondsQuery* query, GetDrawSecondsResult* result);
	void (*GetProfilerTimeRecord)(const GetProfilerTimeRecordQuery* query, GetProfilerTimeRecordResult* result);
	void (*GetProfilerRecordNames)(const GetProfilerRecordNamesQuery* query, GetProfilerRecordNamesResult* result);
	void (*GetLuaMemUsage)(const GetLuaMemUsageQuery* query, GetLuaMemUsageResult* result);
	void (*GetVidMemUsage)(const GetVidMemUsageQuery* query, GetVidMemUsageResult* result);
	void (*GetSyncedGCInfo)(const GetSyncedGCInfoQuery* query, GetSyncedGCInfoResult* result);
};

extern const ProfilingApi PROFILING_API;

#ifdef __cplusplus
}
#endif
