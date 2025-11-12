#include "Sound.h"

#include "System/Sound/ISound.h"
#include "System/Sound/ISoundChannels.h"
#include "System/Sound/IAudioChannel.h"
#include "System/float3.h"

namespace {

// Scratch buffer
static thread_local char scratchBuffer[8192];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Sound system not ready" };
static const Error INVALID_SOUND_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid sound file or ID" };

static bool IsReady() { return (sound != nullptr) && ISound::IsInitialized(); }

static void NativePlaySoundFile(const PlaySoundFileQuery* query, PlaySoundFileResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	if (query->soundFile == nullptr || query->soundFile[0] == '\0') {
		result->error = &INVALID_SOUND_ERROR;
		return;
	}

	const unsigned int soundID = sound->GetSoundId(query->soundFile);
	if (soundID == 0) {
		result->error = &INVALID_SOUND_ERROR;
		return;
	}

	IAudioChannel* channel = Channels::General;

	if (query->positional) {
		const float3 pos(query->pos.x, query->pos.y, query->pos.z);
		const float3 velocity(query->velocity.x, query->velocity.y, query->velocity.z);
		channel->PlaySample(soundID, pos, velocity, query->volume);
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
		result->error = &INVALID_SOUND_ERROR;
		return;
	}

	result->error = nullptr;
	result->success = sound->PreloadSoundItem(query->soundName);
}

static void NativePlaySoundStream(const PlaySoundStreamQuery* query, PlaySoundStreamResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	if (query->oggFile == nullptr || query->oggFile[0] == '\0') {
		result->error = &INVALID_SOUND_ERROR;
		return;
	}

	Channels::BGMusic->StreamPlay(query->oggFile, query->volume, false);

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

} // namespace

const SoundApi SOUND_API = {
	.PlaySoundFile = NativePlaySoundFile,
	.LoadSoundDef = NativeLoadSoundDef,
	.PlaySoundStream = NativePlaySoundStream,
	.StopSoundStream = NativeStopSoundStream,
	.PauseSoundStream = NativePauseSoundStream,
	.SetSoundStreamVolume = NativeSetSoundStreamVolume,
	.GetSoundStreamTime = NativeGetSoundStreamTime,
};
