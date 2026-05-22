/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "SystemControl.h"

#include <array>
#include <cstring>
#include <fstream>
#include <string>
#include <thread>
#include <algorithm>

#include "Game/Game.h"
#include "Game/GameSetup.h"
#include "Game/GlobalUnsynced.h"
#include "Game/IVideoCapturing.h"
#include "Game/SelectedUnitsHandler.h"
#include "Game/UI/GuiHandler.h"
#include "Game/UI/KeyCodes.h"
#include "Game/UI/KeySet.h"
#include "Game/UI/MouseHandler.h"
#include "Game/UI/PlayerRoster.h"
#include "Game/UI/PlayerRosterDrawer.h"
#include "Lua/LuaHandle.h"
#include "Lua/LuaHandleSynced.h"
#include "Menu/LuaMenuController.h"
#include "Map/ReadMap.h"
#include "Net/GameServer.h"
#include "Net/Protocol/BaseNetProtocol.h"
#include "Net/Protocol/NetProtocol.h"
#include "Rendering/GlobalRendering.h"
#include "Sim/Misc/ModInfo.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Sim/Misc/TeamHandler.h"
#include "Game/Players/Player.h"
#include "Game/Players/PlayerHandler.h"
#include "System/LoadSave/DemoReader.h"
#include "System/LoadSave/DemoRecorder.h"
#include "System/LoadLock.h"
#include "System/Log/ILog.h"
#include "System/Platform/Misc.h"
#include "System/Platform/Watchdog.h"
#include "System/Platform/WindowManagerHelper.h"
#include "System/SafeUtil.h"
#include "System/FileSystem/DataDirLocater.h"

#include <SDL.h>

namespace {

static thread_local char scratchBuffer[256];

static const Error NOT_AVAILABLE_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Function not available"
};

static const Error INVALID_ARGUMENT_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid argument"
};

static const Error OPERATION_FAILED_ERROR = {
	.code = ERROR_OPERATION_FAILED,
	.message = "Operation failed"
};

static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "System not ready"
};

static bool CopyString(const std::string& src, const char** outPtr)
{
	if (src.size() + 1 > sizeof(scratchBuffer))
		return false;
	std::memcpy(scratchBuffer, src.c_str(), src.size() + 1);
	*outPtr = scratchBuffer;
	return true;
}

static void NativeCallAsTeam(const CallAsTeamQuery* query, CallAsTeamResult* result)
{
	(void)query->teamID;
	(void)query->func;
	(void)query->args;
	result->error = nullptr;
	result->success = true;
}

static void NativeGarbageCollectCtrl(const GarbageCollectCtrlQuery*, GarbageCollectCtrlResult* result)
{
	result->error = nullptr;
	result->success = true;
}

static void NativeClearWatchDogTimer(const ClearWatchDogTimerQuery* query, ClearWatchDogTimerResult* result)
{
	result->error = nullptr;
	const char* threadName = query->threadName;
	if (threadName == nullptr) {
		Watchdog::ClearTimer();
	} else {
		Watchdog::ClearTimer(threadName, query->keepStopped);
	}
	result->success = true;
}

static void NativeQuit(const QuitQuery*, QuitResult* result)
{
	result->error = nullptr;
	gu->globalQuit = true;
	result->success = true;
}

static int ReloadOrRestart(const std::string& springArgs, const std::string& scriptText, bool newProcess)
{
	const std::string springFullName = Platform::GetProcessExecutableFile();
	const std::string scriptFullName = dataDirLocater.GetWriteDirPath() + "script.txt";

	if (!newProcess) {
		if (gameSetup != nullptr) {
			gameSetup->reloadScript = scriptText;
		}
		gu->globalReload = true;

		LOG("[%s] Spring \"%s\" should be reloading", __func__, springFullName.c_str());
		return 0;
	}

	std::array<std::string, 32> processArgs;

	processArgs[0] = springFullName;
	processArgs[1] = " ";

	if (!springArgs.empty()) {
		processArgs[1] = springArgs;
	}

	if (!scriptText.empty()) {
		std::ofstream scriptFile(scriptFullName.c_str());
		scriptFile.write(scriptText.c_str(), scriptText.size());
		scriptFile.close();

		processArgs[2] = scriptFullName;
	}

	#ifdef _WIN32
	ISound::Shutdown(false);
	#endif

	spring::SafeDelete(gameServer);

	LOG("[%s] Spring \"%s\" should be restarting", __func__, springFullName.c_str());
	Platform::ExecuteProcess(processArgs, newProcess);
	return 1;
}

static void NativeReload(const ReloadQuery* query, ReloadResult* result)
{
	result->error = nullptr;
	result->success = false;

	const char* script = (query != nullptr && query->startScript != nullptr) ? query->startScript : "";
	if (ReloadOrRestart("", script, false) == 0) {
		result->success = true;
		return;
	}

	result->error = &OPERATION_FAILED_ERROR;
}

static void NativeRestart(const RestartQuery* query, RestartResult* result)
{
	result->error = nullptr;
	result->success = false;

	const char* args = (query != nullptr && query->cmdArgs != nullptr) ? query->cmdArgs : "";
	const char* script = (query != nullptr && query->startScript != nullptr) ? query->startScript : "";

	if (ReloadOrRestart(args, script, false) == 0) {
		result->success = true;
		return;
	}

	result->error = &OPERATION_FAILED_ERROR;
}

static void NativeStart(const StartQuery* query, StartResult* result)
{
	result->error = nullptr;
	result->success = false;

	const char* args = (query != nullptr && query->cmdArgs != nullptr) ? query->cmdArgs : "";
	const char* script = (query != nullptr && query->startScript != nullptr) ? query->startScript : "";

	if (ReloadOrRestart(args, script, true) == 0) {
		result->success = true;
		return;
	}

	result->error = &OPERATION_FAILED_ERROR;
}

static void NativeYield(const YieldQuery*, YieldResult* result)
{
	if (CLoadLock::GetThreadSafety() == false) {
		result->error = nullptr;
		result->keepYielding = false;
		return;
	}

	auto& mtx = CLoadLock::GetMutex();
	mtx.unlock();
	std::this_thread::yield();
	mtx.lock();
	Watchdog::ClearTimer(WDT_LOAD);

	result->error = nullptr;
	result->keepYielding = true;
}

static void NativeRequestStartPosition(const RequestStartPositionQuery* query, RequestStartPositionResult* result)
{
	result->error = nullptr;
	result->success = false;

	const float3 pickPos(query->pos.x, query->pos.y, query->pos.z);
	const bool isReady = query->ready;

	clientNet->Send(CBaseNetProtocol::Get().SendStartPos(
		gu->myPlayerNum,
		gu->myTeam,
		isReady ? CPlayer::PLAYER_RDYSTATE_READIED : CPlayer::PLAYER_RDYSTATE_UPDATED,
		pickPos.x,
		pickPos.y,
		pickPos.z
	));

	result->success = true;
}

static void NativePing(const PingQuery* query, PingResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (guihandler == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	guihandler->RunCustomCommands({"@@netping " + IntToString(query->tag, "%u")}, false);
	result->success = true;
}

static void NativeGetGameState(const GetGameStateQuery* query, GetGameStateResult* result)
{
	result->error = nullptr;
	const float maxLatency = (query->maxLatency != 0.0f) ? query->maxLatency : 500.0f;

	result->doneLoading = game->IsDoneLoading();
	result->isSavedGame = game->IsSavedGame();
	result->isClientPaused = game->IsClientPaused();
	result->isSimLagging = game->IsSimLagging(maxLatency);
}

static void NativeGetGameName(const GetGameNameQuery*, GetGameNameResult* result)
{
	result->error = nullptr;
	if (!CopyString(modInfo.humanNameVersioned, &result->name))
		result->error = &INVALID_ARGUMENT_ERROR;
}

static void NativeGetMenuName(const GetMenuNameQuery*, GetMenuNameResult* result)
{
	result->error = nullptr;
	if (luaMenuController == nullptr) {
		result->error = &NOT_READY_ERROR;
		result->name = nullptr;
		return;
	}

	if (!CopyString(luaMenuController->GetMenuName(), &result->name))
		result->error = &INVALID_ARGUMENT_ERROR;
}

static void NativeGetReplayLength(const GetReplayLengthQuery*, GetReplayLengthResult* result)
{
	result->error = nullptr;
	result->success = false;
	result->seconds = 0.0f;

	if (gameServer != nullptr && gameServer->GetDemoReader()) {
		result->seconds = gameServer->GetDemoReader()->GetFileHeader().gameTime;
		result->success = true;
		return;
	}
}

static void NativeGetReplayFilePath(const GetReplayFilePathQuery*, GetReplayFilePathResult* result)
{
	result->error = nullptr;
	result->path = nullptr;
	result->success = false;

	if (gameServer != nullptr && gameServer->GetDemoReader()) {
		if (CopyString(gameServer->GetDemoReader()->GetName(), &result->path))
			result->success = true;
		else
			result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	result->error = &NOT_AVAILABLE_ERROR;
}

static void NativeGetReplayRecordingFilePath(const GetReplayRecordingFilePathQuery*, GetReplayRecordingFilePathResult* result)
{
	result->error = nullptr;
	result->path = nullptr;
	result->success = false;

	CDemoRecorder* demoRecorder = (clientNet != nullptr) ? clientNet->GetDemoRecorder() : nullptr;
	if (demoRecorder == nullptr && gameServer != nullptr && gameServer->GetDemoRecorder() != nullptr)
		demoRecorder = gameServer->GetDemoRecorder().get();

	if (demoRecorder != nullptr && demoRecorder->IsValid()) {
		if (CopyString(demoRecorder->GetName(), &result->path))
			result->success = true;
		else
			result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	result->error = &NOT_AVAILABLE_ERROR;
}

static void NativeIsReplay(const IsReplayQuery*, IsReplayResult* result)
{
	result->error = nullptr;
	result->isReplay = gameSetup != nullptr && gameSetup->hostDemo;
}

static void NativeGetVideoCapturingMode(const GetVideoCapturingModeQuery*, GetVideoCapturingModeResult* result)
{
	result->error = nullptr;
	result->allowRecord = (videoCapturing != nullptr) ? videoCapturing->AllowRecord() : false;
}

static void NativeGetWindowDisplayMode(const GetWindowDisplayModeQuery*, GetWindowDisplayModeResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (globalRendering == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	SDL_DisplayMode dmode;
	if (!SDL_GetWindowDisplayMode(globalRendering->GetWindow(), &dmode)) {
		result->width = dmode.w;
		result->height = dmode.h;
		result->bpp = SDL_BITSPERPIXEL(dmode.format);
		result->refresh = dmode.refresh_rate;
		result->success = CopyString(SDL_GetPixelFormatName(dmode.format), &result->formatName);
		if (!result->success)
			result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	result->error = &NOT_AVAILABLE_ERROR;
}

static void NativeGetGatherMode(const GetGatherModeQuery*, GetGatherModeResult* result)
{
	result->error = nullptr;
	result->mode = (guihandler != nullptr) ? guihandler->GetGatherMode() : -1;
}

static void NativeSetShareLevel(const SetShareLevelQuery* query, SetShareLevelResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (gu->spectating || gs->noHelperAIs || gs->PreSimFrame()) {
		result->error = &NOT_AVAILABLE_ERROR;
		return;
	}

	if (query->resource == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const float shareLevel = std::clamp(query->level, 0.0f, 1.0f);

	if (query->resource[0] == 'm') {
		clientNet->Send(CBaseNetProtocol::Get().SendSetShare(
			gu->myPlayerNum,
			gu->myTeam,
			shareLevel,
			teamHandler.Team(gu->myTeam)->resShare.energy
		));
		result->success = true;
		return;
	}

	if (query->resource[0] == 'e') {
		clientNet->Send(CBaseNetProtocol::Get().SendSetShare(
			gu->myPlayerNum,
			gu->myTeam,
			teamHandler.Team(gu->myTeam)->resShare.metal,
			shareLevel
		));
		result->success = true;
		return;
	}

	result->error = &INVALID_ARGUMENT_ERROR;
}

static void NativeShareResources(const ShareResourcesQuery* query, ShareResourcesResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (gu->spectating || gs->noHelperAIs || gs->PreSimFrame()) {
		result->error = &NOT_AVAILABLE_ERROR;
		return;
	}

	if (!teamHandler.IsValidTeam(query->teamID)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const CTeam* team = teamHandler.Team(query->teamID);
	if ((team == nullptr) || team->isDead) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	if (query->resource == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const char type = query->resource[0];
	if (type == 'u') {
		selectedUnitsHandler.SendSelect();
		clientNet->Send(CBaseNetProtocol::Get().SendShare(gu->myPlayerNum, query->teamID, 1, 0.0f, 0.0f));
		selectedUnitsHandler.ClearSelected();
		result->success = true;
		return;
	}

	if (type == 'm') {
		clientNet->Send(CBaseNetProtocol::Get().SendShare(gu->myPlayerNum, query->teamID, 0, query->amount, 0.0f));
		result->success = true;
		return;
	}

	if (type == 'e') {
		clientNet->Send(CBaseNetProtocol::Get().SendShare(gu->myPlayerNum, query->teamID, 0, 0.0f, query->amount));
		result->success = true;
		return;
	}

	result->error = &INVALID_ARGUMENT_ERROR;
}

} // namespace

const SystemControlApi SYSTEM_CONTROL_API = {
	.CallAsTeam = NativeCallAsTeam,
	.GarbageCollectCtrl = NativeGarbageCollectCtrl,
	.ClearWatchDogTimer = NativeClearWatchDogTimer,
	.Quit = NativeQuit,
	.Reload = NativeReload,
	.Restart = NativeRestart,
	.Start = NativeStart,
	.Yield = NativeYield,
	.RequestStartPosition = NativeRequestStartPosition,
	.Ping = NativePing,
	.GetGameState = NativeGetGameState,
	.GetGameName = NativeGetGameName,
	.GetMenuName = NativeGetMenuName,
	.GetReplayLength = NativeGetReplayLength,
	.GetReplayFilePath = NativeGetReplayFilePath,
	.GetReplayRecordingFilePath = NativeGetReplayRecordingFilePath,
	.IsReplay = NativeIsReplay,
	.GetVideoCapturingMode = NativeGetVideoCapturingMode,
	.GetWindowDisplayMode = NativeGetWindowDisplayMode,
	.GetGatherMode = NativeGetGatherMode,
	.SetShareLevel = NativeSetShareLevel,
	.ShareResources = NativeShareResources,
};
