#include "Sound.h"

#include <vector>

#include "Lua/LuaParser.h"
#include "System/Sound/ISound.h"
#include "System/Sound/ISoundChannels.h"
#include "System/Sound/IAudioChannel.h"
#if !defined(HEADLESS) && !defined(NO_SOUND)
#include "System/Sound/OpenAL/EFX.h"
#endif
#include "System/FileSystem/VFSModes.h"
#include "System/float3.h"

namespace {

// Scratch buffer
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Sound system not ready" };
static const Error INVALID_SOUND_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid sound file or ID" };
static const Error SOUND_EFFECTS_UNSUPPORTED_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Sound effects are not supported" };

static bool IsReady() { return (sound != nullptr) && ISound::IsInitialized(); }

static void NativePlaySoundFile(const PlaySoundFileQuery* query, PlaySoundFileResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	if (query->soundFile == nullptr || query->soundFile[0] == '\0') {
		result->error = nullptr;
		result->success = false;
		return;
	}

	const unsigned int soundID = sound->GetSoundId(query->soundFile);
	if (soundID == 0) {
		result->error = nullptr;
		result->success = false;
		return;
	}

	IAudioChannel* channel = Channels::General;
	switch (query->channel) {
		case 1: channel = Channels::Battle; break;
		case 2: channel = Channels::UnitReply; break;
		case 3: channel = Channels::UserInterface; break;
		default: break;
	}

	const bool hasPos = (query->pos.x != 0.0f) || (query->pos.y != 0.0f) || (query->pos.z != 0.0f);
	const bool hasVelocity = (query->velocity.x != 0.0f) || (query->velocity.y != 0.0f) || (query->velocity.z != 0.0f);
	if (hasPos && hasVelocity) {
		channel->PlaySample(soundID, float3(query->pos.x, query->pos.y, query->pos.z), float3(query->velocity.x, query->velocity.y, query->velocity.z), query->volume);
	} else if (hasPos) {
		channel->PlaySample(soundID, float3(query->pos.x, query->pos.y, query->pos.z), query->volume);
	} else {
		channel->PlaySample(soundID, query->volume);
	}

	result->error = nullptr;
	result->success = true;
}

static void NativeLoadSoundDef(const LoadSoundDefQuery* query, LoadSoundDefResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	if (query->soundName == nullptr || query->soundName[0] == '\0') {
		result->error = nullptr;
		result->success = false;
		return;
	}

	LuaParser soundDefsParser(query->soundName, SPRING_VFS_ZIP_FIRST, SPRING_VFS_ZIP_FIRST);

	result->error = nullptr;
	result->success = sound->LoadSoundDefs(&soundDefsParser);
}

static void NativePlaySoundStream(const PlaySoundStreamQuery* query, PlaySoundStreamResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	if (query->oggFile == nullptr || query->oggFile[0] == '\0') {
		result->error = &INVALID_SOUND_ERROR;
		return;
	}

	Channels::BGMusic->StreamPlay(query->oggFile, query->volume, query->enqueue);

	result->error = nullptr;
	result->success = true;
}

static void NativeStopSoundStream(const StopSoundStreamQuery* query, StopSoundStreamResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	Channels::BGMusic->StreamStop();
	result->error = nullptr;
	result->success = true;
}

static void NativePauseSoundStream(const PauseSoundStreamQuery* query, PauseSoundStreamResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	Channels::BGMusic->StreamPause();
	result->error = nullptr;
	result->success = true;
}

static void NativeSetSoundStreamVolume(const SetSoundStreamVolumeQuery* query, SetSoundStreamVolumeResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	Channels::BGMusic->SetVolume(query->volume);
	result->error = nullptr;
	result->success = true;
}

static void NativeGetSoundStreamTime(const GetSoundStreamTimeQuery* query, GetSoundStreamTimeResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	result->error = nullptr;
	result->time = Channels::BGMusic->StreamGetTime();
}

static void NativeGetSoundDevices(const GetSoundDevicesQuery* /*query*/, GetSoundDevicesResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	static thread_local std::vector<std::string> deviceStrings;
	static thread_local std::vector<const char*> devicePtrs;

	deviceStrings = sound->GetSoundDevices();
	devicePtrs.clear();
	devicePtrs.reserve(deviceStrings.size());

	for (const auto& name : deviceStrings) {
		devicePtrs.push_back(name.c_str());
	}

	result->error = nullptr;
	result->devices = devicePtrs.data();
	result->count = static_cast<uint32_t>(devicePtrs.size());
}

static void NativeGetSoundEffectParams(const GetSoundEffectParamsQuery* /*query*/, GetSoundEffectParamsResult* result) {
	bufferPos = 0;
#if defined(HEADLESS) || defined(NO_SOUND)
	// LuaUnsyncedRead::GetSoundEffectParams returns no values when EFX is
	// unavailable; lack of support is a normal result, not a failed native
	// invocation.
	result->error = nullptr;
	result->success = false;
#else
	result->success = efx.Supported();
	result->error = nullptr;
#endif
}

static void NativeSetSoundEffectParams(const SetSoundEffectParamsQuery* query, SetSoundEffectParamsResult* result) {
	bufferPos = 0;
#if defined(HEADLESS) || defined(NO_SOUND)
	(void)query;
	// LuaUnsyncedCtrl::SetSoundEffectParams is a no-op when EFX is
	// unavailable, including in headless builds.
	result->error = nullptr;
	result->success = false;
#else
	result->success = false;

	if (!efx.Supported()) {
		result->error = nullptr;
		return;
	}

	if (query->params.preset != nullptr && query->params.preset[0] != '\0') {
		efx.SetPreset(query->params.preset, false);
		result->success = true;
	}

	result->error = nullptr;
#endif
}

static void NativePreloadSoundItem(const PreloadSoundItemQuery* query, PreloadSoundItemResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	if (query->soundName == nullptr || query->soundName[0] == '\0') {
		result->error = nullptr;
		result->success = false;
		return;
	}

	result->error = nullptr;
	result->success = sound->PreloadSoundItem(query->soundName);
}

} // namespace

const SoundApi SOUND_API = {
	.PlaySoundFile = NativePlaySoundFile,
	.LoadSoundDef = NativeLoadSoundDef,
	.PlaySoundStream = NativePlaySoundStream,
	.StopSoundStream = NativeStopSoundStream,
	.PauseSoundStream = NativePauseSoundStream,
	.SetSoundStreamVolume = NativeSetSoundStreamVolume,
	.GetSoundStreamTime = NativeGetSoundStreamTime,
	.GetSoundDevices = NativeGetSoundDevices,
	.GetSoundEffectParams = NativeGetSoundEffectParams,
	.SetSoundEffectParams = NativeSetSoundEffectParams,
	.PreloadSoundItem = NativePreloadSoundItem,
};
