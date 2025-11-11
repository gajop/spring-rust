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

// Sound playback request
struct SoundPlayRequest {
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

// Sound stream
struct SoundStreamRequest {
	const char* oggFile;
	float volume;
};

// API structure
struct SoundApi {
	// Sound effects
	BoolResult (*PlaySoundFile)(SoundPlayRequest request);
	BoolResult (*LoadSoundDef)(const char* soundName);

	// Music streams
	BoolResult (*PlaySoundStream)(SoundStreamRequest request);
	BoolResult (*StopSoundStream)();
	BoolResult (*PauseSoundStream)();
	BoolResult (*SetSoundStreamVolume)(float volume);
	FloatResult (*GetSoundStreamTime)();
};

extern const SoundApi SOUND_API;

#ifdef __cplusplus
}
#endif
