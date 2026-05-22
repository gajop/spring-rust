/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

struct CallAsTeamQuery {
	int32_t teamID;
	LuaFunctionRef func;
	NativeLuaArgs args;
};
struct CallAsTeamResult { const Error* error; bool success; };

struct GarbageCollectCtrlQuery {
	int32_t itersPerBatch;
	int32_t numStepsPerIter;
	int32_t minStepsPerIter;
	int32_t maxStepsPerIter;
	float minLoopRunTime;
	float maxLoopRunTime;
	float baseRunTimeMult;
	float baseMemLoadMult;
};
struct GarbageCollectCtrlResult { const Error* error; bool success; };

struct ClearWatchDogTimerQuery { const char* threadName; bool keepStopped; };
struct ClearWatchDogTimerResult { const Error* error; bool success; };

struct QuitQuery { uint8_t _unused; };
struct QuitResult { const Error* error; bool success; };

struct ReloadQuery { const char* startScript; };
struct ReloadResult { const Error* error; bool success; };

struct RestartQuery { const char* cmdArgs; const char* startScript; };
struct RestartResult { const Error* error; bool success; };

struct StartQuery { const char* cmdArgs; const char* startScript; };
struct StartResult { const Error* error; bool success; };

struct YieldQuery { uint8_t _unused; };
struct YieldResult { const Error* error; bool keepYielding; };

struct RequestStartPositionQuery { Float3 pos; bool ready; };
struct RequestStartPositionResult { const Error* error; bool success; };

struct PingQuery { uint32_t tag; };
struct PingResult { const Error* error; bool success; };

struct GetGameStateQuery { float maxLatency; };
struct GetGameStateResult { const Error* error; bool doneLoading; bool isSavedGame; bool isClientPaused; bool isSimLagging; };

struct GetGameNameQuery { uint8_t _unused; };
struct GetGameNameResult { const Error* error; const char* name; };

struct GetMenuNameQuery { uint8_t _unused; };
struct GetMenuNameResult { const Error* error; const char* name; };

struct GetReplayLengthQuery { uint8_t _unused; };
struct GetReplayLengthResult { const Error* error; float seconds; bool success; };

struct GetReplayFilePathQuery { uint8_t _unused; };
struct GetReplayFilePathResult { const Error* error; const char* path; bool success; };

struct GetReplayRecordingFilePathQuery { uint8_t _unused; };
struct GetReplayRecordingFilePathResult { const Error* error; const char* path; bool success; };

struct IsReplayQuery { uint8_t _unused; };
struct IsReplayResult { const Error* error; bool isReplay; };

struct GetVideoCapturingModeQuery { uint8_t _unused; };
struct GetVideoCapturingModeResult { const Error* error; bool allowRecord; };

struct GetWindowDisplayModeQuery { uint8_t _unused; };
struct GetWindowDisplayModeResult { const Error* error; int32_t width; int32_t height; int32_t bpp; int32_t refresh; const char* formatName; bool success; };

struct GetGatherModeQuery { uint8_t _unused; };
struct GetGatherModeResult { const Error* error; int32_t mode; };

struct SetShareLevelQuery { const char* resource; float level; };
struct SetShareLevelResult { const Error* error; bool success; };

struct ShareResourcesQuery { int32_t teamID; const char* resource; float amount; };
struct ShareResourcesResult { const Error* error; bool success; };

struct SystemControlApi {
	void (*CallAsTeam)(const CallAsTeamQuery* query, CallAsTeamResult* result);
	void (*GarbageCollectCtrl)(const GarbageCollectCtrlQuery* query, GarbageCollectCtrlResult* result);
	void (*ClearWatchDogTimer)(const ClearWatchDogTimerQuery* query, ClearWatchDogTimerResult* result);
	void (*Quit)(const QuitQuery* query, QuitResult* result);
	void (*Reload)(const ReloadQuery* query, ReloadResult* result);
	void (*Restart)(const RestartQuery* query, RestartResult* result);
	void (*Start)(const StartQuery* query, StartResult* result);
	void (*Yield)(const YieldQuery* query, YieldResult* result);
	void (*RequestStartPosition)(const RequestStartPositionQuery* query, RequestStartPositionResult* result);
	void (*Ping)(const PingQuery* query, PingResult* result);
	void (*GetGameState)(const GetGameStateQuery* query, GetGameStateResult* result);
	void (*GetGameName)(const GetGameNameQuery* query, GetGameNameResult* result);
	void (*GetMenuName)(const GetMenuNameQuery* query, GetMenuNameResult* result);
	void (*GetReplayLength)(const GetReplayLengthQuery* query, GetReplayLengthResult* result);
	void (*GetReplayFilePath)(const GetReplayFilePathQuery* query, GetReplayFilePathResult* result);
	void (*GetReplayRecordingFilePath)(const GetReplayRecordingFilePathQuery* query, GetReplayRecordingFilePathResult* result);
	void (*IsReplay)(const IsReplayQuery* query, IsReplayResult* result);
	void (*GetVideoCapturingMode)(const GetVideoCapturingModeQuery* query, GetVideoCapturingModeResult* result);
	void (*GetWindowDisplayMode)(const GetWindowDisplayModeQuery* query, GetWindowDisplayModeResult* result);
	void (*GetGatherMode)(const GetGatherModeQuery* query, GetGatherModeResult* result);
	void (*SetShareLevel)(const SetShareLevelQuery* query, SetShareLevelResult* result);
	void (*ShareResources)(const ShareResourcesQuery* query, ShareResourcesResult* result);
};

extern const SystemControlApi SYSTEM_CONTROL_API;

#ifdef __cplusplus
}
#endif
