#include "Sound.h"

#include "System/Sound/ISound.h"
#include "System/Sound/ISoundChannels.h"
#include "System/Sound/IAudioChannel.h"
#include "System/float3.h"

namespace {

// Error constants
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Sound system not ready"
};

static const Error INVALID_SOUND_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid sound file or ID"
};

// Helper: check if sound system is ready
static bool IsReady()
{
	return (sound != nullptr) && ISound::IsInitialized();
}

// Sound effects
static BoolResult NativePlaySoundFile(SoundPlayRequest request)
{
	BoolResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (request.soundFile == nullptr || request.soundFile[0] == '\0') {
		result.error = &INVALID_SOUND_ERROR;
		return result;
	}

	const unsigned int soundID = sound->GetSoundId(request.soundFile);
	if (soundID == 0) {
		result.error = &INVALID_SOUND_ERROR;
		return result;
	}

	// Use General channel by default (could be extended to support channel selection)
	IAudioChannel* channel = Channels::General;

	if (request.positional) {
		const float3 pos(request.pos.x, request.pos.y, request.pos.z);
		const float3 velocity(request.velocity.x, request.velocity.y, request.velocity.z);
		channel->PlaySample(soundID, pos, velocity, request.volume);
	} else {
		channel->PlaySample(soundID, request.volume);
	}

	result.value = true;
	return result;
}

static BoolResult NativeLoadSoundDef(const char* soundName)
{
	BoolResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (soundName == nullptr || soundName[0] == '\0') {
		result.error = &INVALID_SOUND_ERROR;
		return result;
	}

	result.value = sound->PreloadSoundItem(soundName);
	return result;
}

// Music streams
static BoolResult NativePlaySoundStream(SoundStreamRequest request)
{
	BoolResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	if (request.oggFile == nullptr || request.oggFile[0] == '\0') {
		result.error = &INVALID_SOUND_ERROR;
		return result;
	}

	// BGMusic channel is used for music streams
	Channels::BGMusic->StreamPlay(request.oggFile, request.volume, false);

	result.value = true;
	return result;
}

static BoolResult NativeStopSoundStream()
{
	BoolResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	Channels::BGMusic->StreamStop();
	result.value = true;
	return result;
}

static BoolResult NativePauseSoundStream()
{
	BoolResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	Channels::BGMusic->StreamPause();
	result.value = true;
	return result;
}

static BoolResult NativeSetSoundStreamVolume(float volume)
{
	BoolResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	Channels::BGMusic->SetVolume(volume);
	result.value = true;
	return result;
}

static FloatResult NativeGetSoundStreamTime()
{
	FloatResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	result.value = Channels::BGMusic->StreamGetTime();
	return result;
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
