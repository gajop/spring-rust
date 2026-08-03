#pragma once

#include <stdint.h>
#include "Common.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Common Vector Types
// ============================================================================

struct Float2 {
	float x;
	float y;
};

struct Float3 {
	float x;
	float y;
	float z;
};

struct Float4 {
	float x;
	float y;
	float z;
	float w;
};

struct Int2 {
	int32_t x;
	int32_t y;
};

struct Int3 {
	int32_t x;
	int32_t y;
	int32_t z;
};

struct DefRef {
	const char* name;
	int32_t id;              // Use id >= 0 for numeric lookups, otherwise use name.
};

struct NumberOrBool {
	float number;
	bool boolean;
	bool useBoolean;
};

struct UnitHealthValue {
	float health;
	float capture;
	float paralyze;
	float build;
	bool useAmounts;
};

struct UnitCostOverrides {
	float buildTime;
	float metalCost;
	float energyCost;
};

struct UnitTargetRef {
	int32_t targetID;
	Float3 pos;
	bool isGroundTarget;
};

struct ProjectileTargetRef {
	int32_t targetID;
	int32_t targetType;
	Float3 pos;
	bool isGroundTarget;
};

struct NativeProjectileParams {
	Float3 pos;
	Float3 speed;
	Float3 spread;
	Float3 error;
	Float3 end;
	int32_t owner;
	int32_t team;
	// Zero-based weapon number used to inherit a live weapon's mutable
	// projectile state. Use -1 when there is no firing weapon instance.
	int32_t weaponNum;
	float ttl;
	float gravity;
	float tracking;
	float maxRange;
	float upTime;
	float startAlpha;
	float endAlpha;
	const char* model;
	const char* cegTag;
};

struct NativeExplosionParams {
	float damages;
	int32_t weaponDefID;
	int32_t ownerID;
	int32_t hitUnitID;
	int32_t hitFeatureID;
	float craterAreaOfEffect;
	float damageAreaOfEffect;
	float edgeEffectiveness;
	float explosionSpeed;
	float gfxMod;
	bool impactOnly;
	bool ignoreOwner;
	bool damageGround;
	int32_t projectileID;
};

struct RgbColor {
	float r;
	float g;
	float b;
};

struct ResourcePack {
	float metal;
	float energy;
};

// Generic native callback the engine invokes back into the module. Used for
// batched operations where the engine must set up state before invoking module
// code and restore/finalize afterwards. `userData` is opaque to the engine and
// passed straight back to the callback.
typedef void (*NativeCallback)(void* userData);

// Map-rendering parameter structs. Each carries every field the corresponding
// Lua setter (SetAtmosphere/SetSunLighting/SetWaterParams/SetMapRenderingParams)
// understands, plus a `has<Field>` flag so partial updates work like the Lua
// key/value tables: only fields with their flag set are applied.

struct AtmosphereParams {
	float fogColor[4];     bool hasFogColor;
	float skyColor[4];     bool hasSkyColor;
	float sunColor[4];     bool hasSunColor;
	float cloudColor[4];   bool hasCloudColor;
	float skyAxisAngle[4]; bool hasSkyAxisAngle;
	float fogStart;        bool hasFogStart;
	float fogEnd;          bool hasFogEnd;
};

struct SunLightingParams {
	float groundAmbientColor[4];  bool hasGroundAmbientColor;
	float groundDiffuseColor[4];  bool hasGroundDiffuseColor;
	float groundSpecularColor[4]; bool hasGroundSpecularColor;
	float modelAmbientColor[4];   bool hasModelAmbientColor;
	float modelDiffuseColor[4];   bool hasModelDiffuseColor;
	float modelSpecularColor[4];  bool hasModelSpecularColor;
	float specularExponent;       bool hasSpecularExponent;
	float groundShadowDensity;    bool hasGroundShadowDensity;
	float modelShadowDensity;     bool hasModelShadowDensity;
};

struct WaterParams {
	float absorb[3];        bool hasAbsorb;
	float baseColor[3];     bool hasBaseColor;
	float minColor[3];      bool hasMinColor;
	float surfaceColor[3];  bool hasSurfaceColor;
	float diffuseColor[3];  bool hasDiffuseColor;
	float specularColor[3]; bool hasSpecularColor;
	float planeColor[3];    bool hasPlaneColor;

	float repeatX;          bool hasRepeatX;
	float repeatY;          bool hasRepeatY;
	float surfaceAlpha;     bool hasSurfaceAlpha;
	float ambientFactor;    bool hasAmbientFactor;
	float diffuseFactor;    bool hasDiffuseFactor;
	float specularFactor;   bool hasSpecularFactor;
	float specularPower;    bool hasSpecularPower;
	float fresnelMin;       bool hasFresnelMin;
	float fresnelMax;       bool hasFresnelMax;
	float fresnelPower;     bool hasFresnelPower;
	float reflectionDistortion; bool hasReflectionDistortion;
	float blurBase;         bool hasBlurBase;
	float blurExponent;     bool hasBlurExponent;
	float perlinStartFreq;  bool hasPerlinStartFreq;
	float perlinLacunarity; bool hasPerlinLacunarity;
	float perlinAmplitude;  bool hasPerlinAmplitude;
	float windSpeed;        bool hasWindSpeed;
	float waveOffsetFactor; bool hasWaveOffsetFactor;
	float waveLength;       bool hasWaveLength;
	float waveFoamDistortion; bool hasWaveFoamDistortion;
	float waveFoamIntensity; bool hasWaveFoamIntensity;
	float causticsResolution; bool hasCausticsResolution;
	float causticsStrength; bool hasCausticsStrength;
	float numTiles;         bool hasNumTiles;

	bool shoreWaves;        bool hasShoreWaves;
	bool forceRendering;    bool hasForceRendering;
	bool hasWaterPlane;     bool hasHasWaterPlane;
};

struct MapRenderingParams {
	float splatTexScales[4]; bool hasSplatTexScales;
	float splatTexMults[4];  bool hasSplatTexMults;
	bool voidWater;          bool hasVoidWater;
	bool voidGround;         bool hasVoidGround;
	bool splatDetailNormalDiffuseAlpha; bool hasSplatDetailNormalDiffuseAlpha;
};
struct SoundEffectParams { const char* preset; };

// Collision volume data (used by units and features)
struct CollisionVolumeData {
	float scaleX;
	float scaleY;
	float scaleZ;
	float offsetX;
	float offsetY;
	float offsetZ;
	int32_t volumeType;  // 0=ellipsoid, 1=cylinder, 2=box
	int32_t testType;    // Collision test type
	int32_t primaryAxis; // For cylinders
	bool disabled;
};

// ============================================================================
// Dynamic Array Types
// ============================================================================

struct FloatArray {
	const Error* error;
	float* data;
	uint32_t length;
};

struct Int32Array {
	const Error* error;
	int32_t* data;
	uint32_t length;
};

struct UInt32Array {
	const Error* error;
	uint32_t* data;
	uint32_t length;
};

struct Float3Array {
	const Error* error;
	Float3* data;
	uint32_t length;
};

struct StringArray {
	const Error* error;
	const char** data;
	uint32_t length;
};

// ============================================================================
// String Result Type
// ============================================================================

struct StringResult {
	const Error* error;
	const char* value;
};

// ============================================================================
// Boolean Result Type
// ============================================================================

struct BoolResult {
	const Error* error;
	bool value;
};

struct Int32Result {
	const Error* error;
	int32_t value;
};

struct UInt32Result {
	const Error* error;
	uint32_t value;
};

struct FloatResult {
	const Error* error;
	float value;
};

struct Float2Result {
	const Error* error;
	Float2 value;
};

struct Float3Result {
	const Error* error;
	Float3 value;
};

struct Float4Result {
	const Error* error;
	Float4 value;
};

// ============================================================================
// Common Error Codes
// ============================================================================

enum CommonErrorCode {
	ERROR_NONE = 0,
	ERROR_INVALID_ARGUMENT = 1,
	ERROR_OUT_OF_BOUNDS = 2,
	ERROR_NOT_FOUND = 3,
	ERROR_NOT_AVAILABLE = 4,
	ERROR_INVALID_STATE = 5,
	ERROR_PERMISSION_DENIED = 6,
	ERROR_ALREADY_EXISTS = 7,
	ERROR_OPERATION_FAILED = 8,
	ERROR_BUFFER_OVERFLOW = 9,
	ERROR_INVALID_ID = 10,
	ERROR_INTERNAL = 999
};

#ifdef __cplusplus
}
#endif
