/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "Profiling.h"

#include <cstring>
#include <string>
#include <vector>

#include "Game/Game.h"
#include "Game/GlobalUnsynced.h"
#include "Game/IVideoCapturing.h"
#include "lib/lua/include/LuaInclude.h"
#include "Lua/LuaContextData.h"
#include "Lua/LuaRules.h"
#include "System/UnorderedSet.hpp"
#include "Rendering/GlobalRendering.h"
#include "Rendering/GlobalRenderingInfo.h"
#include "Rendering/GL/myGL.h"
#include "System/Log/ILog.h"
#include "System/TimeProfiler.h"
#include "System/Platform/Watchdog.h"
#include "System/ScopedResource.h"
#include "System/SpringMath.h"
#include "lib/lua/include/LuaUser.h"

// [0] := unsynced, [1] := synced. Defined in LuaHandle.cpp.
extern const spring::unsynced_set<const luaContextData*>* LUAHANDLE_CONTEXTS[2];

namespace {

static thread_local uint8_t scratchBuffer[4096];
static thread_local size_t bufferPos = 0;

static const Error INVALID_ARGUMENT_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid argument"
};

static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Not available"
};

static void ResetBuffer()
{
	bufferPos = 0;
}

static uint64_t PackTimer(const spring_time& time, bool microseconds)
{
	if (microseconds)
		return time.toMicroSecs<uint64_t>();
	return time.toMilliSecs<uint64_t>();
}

static void NativeGetTimer(const GetTimerQuery*, GetTimerResult* result)
{
	result->error = nullptr;
	result->timer = PackTimer(spring_now(), false);
}

static void NativeGetTimerMicros(const GetTimerMicrosQuery*, GetTimerMicrosResult* result)
{
	result->error = nullptr;
	result->timer = PackTimer(spring_now(), true);
}

static void NativeDiffTimers(const DiffTimersQuery* query, DiffTimersResult* result)
{
	result->error = nullptr;

	const uint64_t t1 = query->endTimer;
	const uint64_t t2 = query->startTimer;
	if (t1 < t2) {
		result->error = &INVALID_ARGUMENT_ERROR;
		result->seconds = 0.0f;
		return;
	}

	const uint64_t delta = t1 - t2;

	if (query->options.fromMicroSecs) {
		const spring_time dt = spring_time::fromMicroSecs(delta);
		result->seconds = query->options.returnMs ? dt.toMilliSecsf() : dt.toSecsf();
	} else {
		const spring_time dt = spring_time::fromMilliSecs(delta);
		result->seconds = query->options.returnMs ? dt.toMilliSecsf() : dt.toSecsf();
	}
}

static void NativeGetFrameTimer(const GetFrameTimerQuery* query, GetFrameTimerResult* result)
{
	result->error = nullptr;
	const spring_time t = query->lastFrameTime ? game->lastFrameTime : globalRendering->lastFrameStart;
	result->timer = PackTimer(t, false);
}

static void NativeGetDrawSeconds(const GetDrawSecondsQuery*, GetDrawSecondsResult* result)
{
	result->error = nullptr;
	result->seconds = spring_tomsecs(globalRendering->grTime) * 0.001f;
}

static void NativeGetProfilerTimeRecord(const GetProfilerTimeRecordQuery* query, GetProfilerTimeRecordResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->totalMs = 0.0f;
	result->currentMs = 0.0f;
	result->maxDt = 0.0f;
	result->timePct = 0.0f;
	result->peakPct = 0.0f;
	result->frameData = nullptr;
	result->frameCount = 0;

	if (query->name == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const CTimeProfiler::TimeRecord& record = CTimeProfiler::GetInstance().GetTimeRecord(query->name);

	result->totalMs = record.total.toMilliSecsf();
	result->currentMs = record.current.toMilliSecsf();
	result->maxDt = record.stats.x;
	result->timePct = record.stats.y;
	result->peakPct = record.stats.z;

	if (!query->includeFrameData || record.frames.empty())
		return;

	const size_t bytesNeeded = record.frames.size() * sizeof(float);
	if (bufferPos + bytesNeeded > sizeof(scratchBuffer)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	float* out = reinterpret_cast<float*>(scratchBuffer + bufferPos);
	bufferPos += bytesNeeded;

	for (size_t i = 0; i < record.frames.size(); ++i) {
		out[i] = record.frames[i].toMilliSecsf();
	}

	result->frameData = out;
	result->frameCount = record.frames.size();
}

static void NativeGetProfilerRecordNames(const GetProfilerRecordNamesQuery*, GetProfilerRecordNamesResult* result)
{
	ResetBuffer();
	result->error = nullptr;
	result->names = nullptr;
	result->count = 0;

	const auto& sortedProfiles = CTimeProfiler::GetInstance().GetSortedProfiles();
	const size_t count = sortedProfiles.size();
	if (count == 0)
		return;

	const size_t pointerBytes = count * sizeof(const char*);
	if (bufferPos + pointerBytes > sizeof(scratchBuffer)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const char** names = reinterpret_cast<const char**>(scratchBuffer + bufferPos);
	bufferPos += pointerBytes;

	for (size_t i = 0; i < count; ++i) {
		const std::string& name = sortedProfiles[i].first;
		if (bufferPos + name.size() + 1 > sizeof(scratchBuffer)) {
			result->error = &INVALID_ARGUMENT_ERROR;
			return;
		}
		char* dest = reinterpret_cast<char*>(scratchBuffer + bufferPos);
		std::memcpy(dest, name.c_str(), name.size() + 1);
		names[i] = dest;
		bufferPos += name.size() + 1;
	}

	result->names = names;
	result->count = static_cast<uint32_t>(count);
}

static void NativeGetLuaMemUsage(const GetLuaMemUsageQuery*, GetLuaMemUsageResult* result)
{
	result->error = nullptr;
	result->handleAllocedKB = 0.0f;
	result->handleAllocsK = 0.0f;
	result->globalAllocedKB = 0.0f;
	result->globalAllocsK = 0.0f;
	result->unsyncedAllocedKB = 0.0f;
	result->unsyncedAllocsK = 0.0f;
	result->syncedAllocedKB = 0.0f;
	result->syncedAllocsK = 0.0f;

	SLuaAllocState globalState;
	spring_lua_alloc_get_stats(&globalState);

	result->globalAllocedKB = globalState.allocedBytes / 1024.0f;
	result->globalAllocsK = globalState.numLuaAllocs / 1000.0f;

	for (bool synced: {false, true}) {
		SLuaAllocState state;
		state.allocedBytes = {0};
		state.numLuaAllocs = {0};
		state.luaAllocTime = {0};
		state.numLuaStates = {0};

		for (const luaContextData* lcd: *LUAHANDLE_CONTEXTS[synced]) {
			state.allocedBytes += lcd->allocState.allocedBytes;
			state.numLuaAllocs += lcd->allocState.numLuaAllocs;
		}

		if (synced) {
			result->syncedAllocedKB = state.allocedBytes / 1024.0f;
			result->syncedAllocsK = state.numLuaAllocs / 1000.0f;
		} else {
			result->unsyncedAllocedKB = state.allocedBytes / 1024.0f;
			result->unsyncedAllocsK = state.numLuaAllocs / 1000.0f;
		}
	}
}

static void NativeGetVidMemUsage(const GetVidMemUsageQuery*, GetVidMemUsageResult* result)
{
	result->error = nullptr;
	int2 vidMemInfo;
	GetAvailableVideoRAM(&vidMemInfo.x, globalRenderingInfo.glVendor);

	result->usedMB = (vidMemInfo.x - vidMemInfo.y) / 1024.0f;
	result->availableMB = (vidMemInfo.x) / 1024.0f;
}

static void NativeGetSyncedGCInfo(const GetSyncedGCInfoQuery* query, GetSyncedGCInfoResult* result)
{
	result->gcKB = 0.0f;
	result->error = nullptr;

	if (luaRules == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	lua_State* syncedL = luaRules->syncedLuaHandle.GetLuaGCState();
	if (syncedL == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	auto luaLock = spring::ScopedNullResource([syncedL]() { lua_lock(syncedL); }, [syncedL]() { lua_unlock(syncedL); });

	if (query->collect) {
		lua_gc(syncedL, LUA_GCCOLLECT, 0);
		lua_gc(syncedL, LUA_GCCOLLECT, 0);
		lua_gc(syncedL, LUA_GCSTOP, 0);
	}

	result->gcKB = lua_gc(syncedL, LUA_GCCOUNT, 0);
}

} // namespace

const ProfilingApi PROFILING_API = {
	.GetTimer = NativeGetTimer,
	.GetTimerMicros = NativeGetTimerMicros,
	.DiffTimers = NativeDiffTimers,
	.GetFrameTimer = NativeGetFrameTimer,
	.GetDrawSeconds = NativeGetDrawSeconds,
	.GetProfilerTimeRecord = NativeGetProfilerTimeRecord,
	.GetProfilerRecordNames = NativeGetProfilerRecordNames,
	.GetLuaMemUsage = NativeGetLuaMemUsage,
	.GetVidMemUsage = NativeGetVidMemUsage,
	.GetSyncedGCInfo = NativeGetSyncedGCInfo,
};
