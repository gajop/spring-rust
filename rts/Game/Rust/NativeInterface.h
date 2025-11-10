#pragma once

#include <stdint.h>

#include "Game/Rust/api/Common.h"
#include "Game/Rust/api/MetalMap.h"


#ifdef __cplusplus
extern "C" {
#endif

struct IsPosInMapReturn {
  bool inPlayArea;
  bool inMap;
};

struct GetGroundNormalReturn {
  float x;
  float y;
  float z;
  float slope;
};

struct GetGroundInfoReturn {
  float ix;
  float iz;
  float terrainTypeIndex;
  const char *name;
  float metalExtraction;
  float hardness;
  float tankSpeed;
  float kbotSpeed;
  float hoverSpeed;
  float shipSpeed;
  bool receiveTracks;
};

struct GetGroundExtremesReturn {
  float initMinHeight;
  float initMaxHeight;
  float currMinHeight;
  float currMaxHeight;
};

struct GetTerrainTypeDataReturn {
  int index;
  const char *name;
  float hardness;
  float tankSpeed;
  float kbotSpeed;
  float hoverSpeed;
  float shipSpeed;
  bool receiveTracks;
};


#ifdef __cplusplus
}
#endif

struct NativeInterfaceBridge {
  // System
  virtual void HandleRustPanic() = 0;

  //
  virtual IsPosInMapReturn IsPosInMap(float x, float z) = 0;
  virtual float GetGroundHeight(float x, float z) = 0;
  virtual float GetGroundOrigHeight(float x, float z) = 0;
  virtual GetGroundNormalReturn GetGroundNormal(const float x, const float z, bool *normal) = 0;
  virtual GetGroundInfoReturn GetGroundInfo(float x, float z) = 0;
  virtual bool GetGroundBlocked(float fx1, float fz1, const float *fx2, const float *fz2) = 0;
  virtual GetGroundExtremesReturn GetGroundExtremes() = 0;
  virtual GetTerrainTypeDataReturn GetTerrainTypeData(int terrainTypeInfo) = 0;
  virtual float GetGrass(float x, float z) = 0;
  virtual float GetSmoothMeshHeight(float x, float z) = 0;

  // SyncedCtrl START
  virtual float AddHeightMap(float xl, float zl, float height) = 0;
  virtual float SetHeightMap(float xl, float zl, float height, float* scalingFactor) = 0;
  virtual float SetHeightMapFunc(void (*function)(void* data), void* data) = 0;

  virtual void RevertHeightMap(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height, float origFactor) = 0;

  // virtual void SetHeightMap(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height) = 0;
  // virtual void AddHeightMap(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height) = 0;
  // virtual void SetHeightMapArray(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height) = 0;
  // virtual void AddHeightMapArray(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height) = 0;

  // virtual void SetOriginalHeightMap(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height) = 0;
  // virtual void AddOriginalHeightMap(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height) = 0;
  // virtual void RevertOriginalHeightMap(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float origFactor) = 0;
  // virtual void SetOriginalHeightMapArray(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height) = 0;
  // virtual void AddOriginalHeightMapArray(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height) = 0;

  virtual void SetSmoothMesh(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height) = 0;
  virtual void AddSmoothMesh(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height) = 0;
  virtual void RevertSmoothMesh(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float origFactor) = 0;
  virtual void SetSmoothMeshArray(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height) = 0;
  virtual void AddSmoothMeshArray(uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height) = 0;

  virtual void SetMapSquareTerrainType(uint32_t x, uint32_t z, uint32_t newType) = 0;
  virtual bool SetTerrainTypeData(uint32_t typeIndex, float* tankSpeed, float* kbotSpeed, float* hoverSpeed, float* shipSpeed,
	  float* hardness, bool* receiveTracks, const char* name) = 0;
// SyncedCtrl END

// UnsyncedCtrl START
	virtual void Echo(const char* msg) const = 0;
// UnsyncedCtrl END

// Game constants STAT
  virtual float GameMapX() const = 0;
  virtual float GameMapY() const = 0;
  virtual float GameMapSizeX() const = 0;
  virtual float GameMapSizeZ() const = 0;
// Game constants END
};



#ifdef __cplusplus
extern "C" {
#endif

struct NativeInterface {
  using HandleRustPanic = void(*)(NativeInterface* ptr);

  using IsPosInMap = IsPosInMapReturn(*)(NativeInterface* ptr, float x, float z);
  using GetGroundHeight = float(*)(NativeInterface* ptr, float x, float z);
  using GetGroundOrigHeight = float(*)(NativeInterface* ptr, float x, float z);
  using GetGroundNormal = GetGroundNormalReturn(*)(NativeInterface* ptr, const float x, const float z, bool *normal);
  using GetGroundInfo = GetGroundInfoReturn(*)(NativeInterface* ptr, float x, float z);
  using GetGroundBlocked = bool(*)(NativeInterface* ptr, float fx1, float fz1, const float *fx2, const float *fz2);
  using GetGroundExtremes = GetGroundExtremesReturn(*)(NativeInterface* ptr);
  using GetTerrainTypeData = GetTerrainTypeDataReturn(*)(NativeInterface* ptr, int terrainTypeInfo);
  using GetGrass = float(*)(NativeInterface* ptr, float x, float z);
  using GetSmoothMeshHeight = float(*)(NativeInterface* ptr, float x, float z);

  // SyncedCtrl START
  using AddHeightMap = float(*)(NativeInterface* ptr, float xl, float zl, float height);
  using SetHeightMap = float(*)(NativeInterface* ptr, float xl, float zl, float height, float* scalingFactor);
  using SetHeightMapFunc = float(*)(NativeInterface* ptr, void (*function)(void* data), void* data);

  using RevertHeightMap = void(*)(NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height, float origFactor);

  // using SetHeightMap = void(*)(NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height);
  // using AddHeightMap = void(*)(NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height);
  // using SetHeightMapArray = void(*)(NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height);
  // using AddHeightMapArray = void(*)(NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height);

  // using SetOriginalHeightMap = void(*)(NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height);
  // using AddOriginalHeightMap = void(*)(NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height);
  // using RevertOriginalHeightMap = void(*)(NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float origFactor);
  // using SetOriginalHeightMapArray = void(*)(NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height);
  // using AddOriginalHeightMapArray = void(*)(NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height);

  using SetSmoothMesh = void(*)(NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height);
  using AddSmoothMesh = void(*)(NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float height);
  using RevertSmoothMesh = void(*)(NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, float origFactor);
  using SetSmoothMeshArray = void(*)(NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height);
  using AddSmoothMeshArray = void(*)(NativeInterface* ptr, uint32_t x1, uint32_t z1, uint32_t x2, uint32_t z2, const float* height);

  using SetMapSquareTerrainType = void(*)(NativeInterface* ptr, uint32_t x, uint32_t z, uint32_t newType);
  using SetTerrainTypeData = bool(*)(NativeInterface* ptr, uint32_t typeIndex, float* tankSpeed, float* kbotSpeed, float* hoverSpeed, float* shipSpeed,
	  float* hardness, bool* receiveTracks, const char* name);

  using GetMetalAmount = float(*)(NativeInterface* ptr, uint32_t x, uint32_t z);
  using SetMetalAmount = void(*)(NativeInterface* ptr, uint32_t x, uint32_t z, float amount);
// UnsyncedCtrl START
  using Echo = void(*)(NativeInterface* ptr, const char* msg);
// UnsyncedCtrl END

  // SyncedCtrl END
  using GameMapX = float(*)(NativeInterface* ptr);
  using GameMapY = float(*)(NativeInterface* ptr);
  using GameMapSizeX = float(*)(NativeInterface* ptr);
  using GameMapSizeZ = float(*)(NativeInterface* ptr);

  HandleRustPanic f_HandleRustPanic;

	IsPosInMap f_IsPosInMap;
	GetGroundHeight f_GetGroundHeight;
	GetGroundOrigHeight f_GetGroundOrigHeight;
	GetGroundNormal f_GetGroundNormal;
	GetGroundInfo f_GetGroundInfo;
	GetGroundBlocked f_GetGroundBlocked;
	GetGroundExtremes f_GetGroundExtremes;
	GetTerrainTypeData f_GetTerrainTypeData;
	GetGrass f_GetGrass;
	GetSmoothMeshHeight f_GetSmoothMeshHeight;

// SyncedCtrl START
  AddHeightMap f_AddHeightMap;
  SetHeightMap f_SetHeightMap;
  SetHeightMapFunc f_SetHeightMapFunc;

  RevertHeightMap f_RevertHeightMap;

  // SetHeightMapArray f_SetHeightMapArray;
  // AddHeightMapArray f_AddHeightMapArray;

  // SetOriginalHeightMap f_SetOriginalHeightMap;
  // AddOriginalHeightMap f_AddOriginalHeightMap;
  // RevertOriginalHeightMap f_RevertOriginalHeightMap;
  // SetOriginalHeightMapArray f_SetOriginalHeightMapArray;
  // AddOriginalHeightMapArray f_AddOriginalHeightMapArray;

  SetSmoothMesh f_SetSmoothMesh;
  AddSmoothMesh f_AddSmoothMesh;
  RevertSmoothMesh f_RevertSmoothMesh;
  SetSmoothMeshArray f_SetSmoothMeshArray;
  AddSmoothMeshArray f_AddSmoothMeshArray;

  SetMapSquareTerrainType f_SetMapSquareTerrainType;
  SetTerrainTypeData f_SetTerrainTypeData;


  // SyncedCtrl END

  // UnsyncedCtrl START
  Echo f_Echo;
  // UnsyncedCtrl END


	GameMapX f_GameMapX;
	GameMapY f_GameMapY;
	GameMapSizeX f_GameMapSizeX;
	GameMapSizeZ f_GameMapSizeZ;

	const MetalMapApi* metalMap;


	NativeInterfaceBridge *bridge;
};


#ifdef __cplusplus
}
#endif
