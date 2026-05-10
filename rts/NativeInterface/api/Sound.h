#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Sound API
// @see rts/Lua/LuaUnsyncedCtrl.cpp
//
// Sound playback control (unsynced)
// ============================================================================

// Queries
struct PlaySoundFileQuery {
	const char* soundFile;
	float volume;
	Float3 pos;
	Float3 velocity;
	bool positional;  // false for ambient sound
	bool inCone;
	float coneAngle;
	float coneInnerAngle;
	float coneOuterAngle;
	Float3 coneDir;
};

struct PlaySoundFileResult {
	const Error* error;
	bool success;
};

struct LoadSoundDefQuery {
	const char* soundName;
};

struct LoadSoundDefResult {
	const Error* error;
	bool success;
};

struct PlaySoundStreamQuery {
	const char* oggFile;
	float volume;
};

struct PlaySoundStreamResult {
	const Error* error;
	bool success;
};

struct StopSoundStreamQuery { uint8_t _unused; };
struct StopSoundStreamResult { const Error* error; bool success; };

struct PauseSoundStreamQuery { uint8_t _unused; };
struct PauseSoundStreamResult { const Error* error; bool success; };

struct SetSoundStreamVolumeQuery { float volume; };
struct SetSoundStreamVolumeResult { const Error* error; bool success; };

struct GetSoundStreamTimeQuery { uint8_t _unused; };
struct GetSoundStreamTimeResult { const Error* error; float time; };

struct GetSoundDevicesQuery { uint8_t _unused; };
struct GetSoundDevicesResult { const Error* error; const char** devices; uint32_t count; };

struct GetSoundEffectParamsQuery { uint8_t _unused; };
struct GetSoundEffectParamsResult { const Error* error; bool success; };

struct SetSoundEffectParamsQuery { const char* preset; };
struct SetSoundEffectParamsResult { const Error* error; bool success; };

struct PreloadSoundItemQuery { const char* soundName; };
struct PreloadSoundItemResult { const Error* error; bool success; };

// API structure
struct SoundApi {
	void (*PlaySoundFile)(const PlaySoundFileQuery* query, PlaySoundFileResult* result);
	void (*LoadSoundDef)(const LoadSoundDefQuery* query, LoadSoundDefResult* result);
	void (*PlaySoundStream)(const PlaySoundStreamQuery* query, PlaySoundStreamResult* result);
	void (*StopSoundStream)(const StopSoundStreamQuery* query, StopSoundStreamResult* result);
	void (*PauseSoundStream)(const PauseSoundStreamQuery* query, PauseSoundStreamResult* result);
	void (*SetSoundStreamVolume)(const SetSoundStreamVolumeQuery* query, SetSoundStreamVolumeResult* result);
	void (*GetSoundStreamTime)(const GetSoundStreamTimeQuery* query, GetSoundStreamTimeResult* result);
	void (*GetSoundDevices)(const GetSoundDevicesQuery* query, GetSoundDevicesResult* result);
	void (*GetSoundEffectParams)(const GetSoundEffectParamsQuery* query, GetSoundEffectParamsResult* result);
	void (*SetSoundEffectParams)(const SetSoundEffectParamsQuery* query, SetSoundEffectParamsResult* result);
	void (*PreloadSoundItem)(const PreloadSoundItemQuery* query, PreloadSoundItemResult* result);
};

extern const SoundApi SOUND_API;

#ifdef __cplusplus
}
#endif
