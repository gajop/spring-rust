#include "UnsyncedRead.h"
#include <cstring>
#include <algorithm>
#include <vector>

#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Misc/CustomColorPalette.h"
#include "Sim/Misc/GlobalSynced.h"
#include "Sim/Misc/TeamHandler.h"
#include "Sim/Misc/LosHandler.h"
#include "Sim/Misc/QuadField.h"
#include "Sim/Objects/WorldObject.h"
#include "Sim/Features/Feature.h"
#include "Sim/Features/FeatureHandler.h"
#include "Sim/Features/FeatureDef.h"
#include "Sim/Projectiles/Projectile.h"
#include "Rendering/Models/3DModel.hpp"
#include "Rendering/IconHandler.h"
#include "Rendering/GlobalRendering.h"
#include "Game/Camera.h"
#include "Game/GlobalUnsynced.h"
#include "Game/Game.h"
#include "Game/UI/GuiHandler.h"
#include "Game/UI/InfoConsole.h"
#include "Game/SelectedUnitsHandler.h"
#include "Sim/Units/CommandAI/CommandDescription.h"
#include "Map/ReadMap.h"
#include "System/Matrix44f.h"
#include "System/float3.h"
#include "System/float4.h"
#include "System/Platform/Clipboard.h"
#include "Sim/Projectiles/ProjectileHandler.h"
#include "Rendering/Env/Particles/Classes/NanoProjectile.h"
#include "Sim/Projectiles/PieceProjectile.h"
#include "Sim/Projectiles/WeaponProjectiles/WeaponProjectile.h"
#include "System/Math/NURBS.h"
#include "System/SpringMath.h"
#include "Lua/LuaConfig.h"
#include "Game/SelectedUnitsHandler.h"
#include "Sim/Misc/Team.h"
#include "Rendering/Models/3DModelPiece.hpp"

namespace {

// Thread-local scratch buffer for dynamic data
thread_local uint8_t scratchBuffer[4096];
thread_local size_t bufferPos = 0;

// Error messages
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Game not ready"
};

static const Error INVALID_UNIT_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid unit ID"
};

static const Error INVALID_FEATURE_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid feature ID"
};

static const Error INVALID_ARGUMENT_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "Invalid argument"
};

static const Error CAMERA_UNAVAILABLE_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Camera not available"
};

static const Error NOT_AVAILABLE_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "Not available in Native API"
};

static const Error BUFFER_OVERFLOW_ERROR = {
	.code = ERROR_BUFFER_OVERFLOW,
	.message = "Scratch buffer overflow"
};

static const char* CopyToScratch(const std::string& str) {
	const size_t len = str.size() + 1;
	if (bufferPos + len > sizeof(scratchBuffer))
		return "";

	char* out = reinterpret_cast<char*>(&scratchBuffer[bufferPos]);
	memcpy(out, str.c_str(), len);
	bufferPos += len;
	return out;
}

template<typename T>
static T* AllocateArray(size_t count) {
	const size_t needed = count * sizeof(T);
	if (bufferPos + needed > sizeof(scratchBuffer))
		return nullptr;

	T* out = reinterpret_cast<T*>(&scratchBuffer[bufferPos]);
	bufferPos += needed;
	return out;
}

static bool FillActiveCommandDescription(const SCommandDescription& in, ActiveCommandDescription& out)
{
	out.id = in.id;
	out.type = in.type;
	out.name = CopyToScratch(in.name);
	out.action = CopyToScratch(in.action);
	out.tooltip = CopyToScratch(in.tooltip);
	out.texture = CopyToScratch(in.iconname);
	out.cursor = CopyToScratch(in.mouseicon);
	out.queueing = in.queueing;
	out.hidden = in.hidden;
	out.disabled = in.disabled;
	out.showUnique = in.showUnique;
	out.onlyTexture = in.onlyTexture;
	out.params = nullptr;
	out.paramCount = 0;

	if (out.name[0] == '\0' && !in.name.empty())
		return false;
	if (out.action[0] == '\0' && !in.action.empty())
		return false;
	if (out.tooltip[0] == '\0' && !in.tooltip.empty())
		return false;
	if (out.texture[0] == '\0' && !in.iconname.empty())
		return false;
	if (out.cursor[0] == '\0' && !in.mouseicon.empty())
		return false;

	if (!in.params.empty()) {
		out.params = AllocateArray<const char*>(in.params.size());
		if (out.params == nullptr)
			return false;

		for (size_t i = 0; i < in.params.size(); ++i) {
			out.params[i] = CopyToScratch(in.params[i]);
			if (out.params[i][0] == '\0' && !in.params[i].empty())
				return false;
		}
		out.paramCount = static_cast<uint32_t>(in.params.size());
	}

	return true;
}

static bool IsReady() {
	return (gs != nullptr);
}

// Visible object iterators (ported from LuaUnsyncedRead)
template<class T> class CWorldObjectQuadDrawer: public CReadMap::IQuadDrawer {
public:
	using ObjectList = std::vector<T*>;
	using ObjectVector = std::vector<const ObjectList*>;

	void ResetState() override {
		objectLists.clear();
		objectLists.reserve(64);
		objectCount = 0;
	}

	unsigned int GetObjectCount() const { return objectCount; }
	const ObjectVector& GetObjectLists() { return objectLists; }

	void AddObjectList(const ObjectList* objects) {
		if (objects->empty())
			return;

		objectLists.push_back(objects);
		objectCount += objects->size();
	}

protected:
	ObjectVector objectLists;
	unsigned int objectCount = 0;
};

class CVisUnitQuadDrawer: public CWorldObjectQuadDrawer<CUnit> {
public:
	void DrawQuad(int x, int y) override {
		const CQuadField::Quad& q = quadField.GetQuadAt(x, y);
		AddObjectList(&q.units);
	}
};

class CVisFeatureQuadDrawer: public CWorldObjectQuadDrawer<CFeature> {
public:
	void DrawQuad(int x, int y) override {
		const CQuadField::Quad& q = quadField.GetQuadAt(x, y);
		AddObjectList(&q.features);
	}
};

class CVisProjectileQuadDrawer: public CWorldObjectQuadDrawer<CProjectile> {
public:
	void DrawQuad(int x, int y) override {
		const CQuadField::Quad& q = quadField.GetQuadAt(x, y);
		AddObjectList(&q.projectiles);
	}
};

// Allegiance constants (LuaUtils::UnitAllegiance)
static constexpr int ALL_UNITS = -1;
static constexpr int MY_UNITS = -2;
static constexpr int ALLY_UNITS = -3;
static constexpr int ENEMY_UNITS = -4;

static bool UnitMatchesAllegiance(const CUnit* unit, int allegiance, int myTeam, int myAllyTeam) {
	switch (allegiance) {
		case ALL_UNITS: return true;
		case MY_UNITS:  return (myTeam >= 0 && unit->team == myTeam);
		case ALLY_UNITS: return (myAllyTeam >= 0 && unit->allyteam == myAllyTeam);
		case ENEMY_UNITS: return (myAllyTeam >= 0 && unit->allyteam != myAllyTeam);
		default: return (allegiance < 0) ? true : (unit->team == allegiance);
	}
}

static bool UnitVisibleToClient(const CUnit* unit, int readAllyTeam, bool fullView, bool checkIcon) {
	if (unit == nullptr)
		return false;

	if (unit->noDraw)
		return false;

	if (checkIcon && unit->GetIsIcon())
		return false;

	if (!fullView && readAllyTeam >= 0 && !(unit->losStatus[readAllyTeam] & LOS_INLOS))
		return false;

	return true;
}

static bool UnitVisibleForScreenRectangle(const CUnit* unit, int allegiance, int readTeam, int readAllyTeam, bool fullView)
{
	if (unit == nullptr || unit->noDraw)
		return false;

	switch (allegiance) {
		case MY_UNITS:
			return (readTeam >= 0 && unit->team == readTeam);
		case ALLY_UNITS:
			return (readAllyTeam >= 0 && unit->allyteam == readAllyTeam);
		case ENEMY_UNITS:
			return (readAllyTeam >= 0 && unit->allyteam != readAllyTeam);
		case ALL_UNITS:
			return UnitVisibleToClient(unit, readAllyTeam, fullView, false);
		default:
			if (allegiance < 0 || unit->team != allegiance)
				return false;
			if (readTeam >= 0 && teamHandler.AlliedTeams(readTeam, allegiance))
				return true;
			return UnitVisibleToClient(unit, readAllyTeam, fullView, false);
	}
}

static void SetNotAvailable(const Error** error) {
	if (error != nullptr) {
		*error = &NOT_AVAILABLE_ERROR;
	}
}

// ============================================================================
// Unit Rendering State Implementation
// ============================================================================

static void NativeGetUnitNoDraw(const GetUnitNoDrawQuery* query, GetUnitNoDrawResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->noDraw = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->noDraw = unit->noDraw;
}

static void NativeGetUnitLuaDraw(const GetUnitLuaDrawQuery* query, GetUnitLuaDrawResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->luaDraw = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->luaDraw = unit->luaDraw;
}

static void NativeGetUnitEngineDrawMask(const GetUnitEngineDrawMaskQuery* query, GetUnitEngineDrawMaskResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->engineDrawMask = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->engineDrawMask = unit->engineDrawMask;
}

static void NativeGetUnitAlwaysUpdateMatrix(const GetUnitAlwaysUpdateMatrixQuery* query, GetUnitAlwaysUpdateMatrixResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->alwaysUpdateMatrix = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->alwaysUpdateMatrix = unit->alwaysUpdateMat;
}

static void NativeGetUnitDrawFlag(const GetUnitDrawFlagQuery* query, GetUnitDrawFlagResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->drawFlag = 0;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->drawFlag = unit->drawFlag;
}

static void NativeGetUnitNoSelect(const GetUnitNoSelectQuery* query, GetUnitNoSelectResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->noSelect = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->noSelect = unit->noSelect;
}

static void NativeGetUnitNoMinimap(const GetUnitNoMinimapQuery* query, GetUnitNoMinimapResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->noMinimap = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->noMinimap = unit->noMinimap;
}

static void NativeGetUnitNoGroup(const GetUnitNoGroupQuery* query, GetUnitNoGroupResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->noGroup = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->noGroup = unit->noGroup;
}

static void NativeGetUnitViewPosition(const GetUnitViewPositionQuery* query, GetUnitViewPositionResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->position = {0.0f, 0.0f, 0.0f};

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	// Get unit position (with or without midPos adjustment)
	const float3 unitPos = query->useMidPos ? unit->GetObjDrawMidPos() : unit->drawPos;

	// Note: In Lua, this function also adds error vector for ally team visibility
	// For Native API, we return the actual position without error vector
	// If error vector is needed, it should be handled on the client side
	result->position.x = unitPos.x;
	result->position.y = unitPos.y;
	result->position.z = unitPos.z;
}

static void NativeGetUnitTransformMatrix(const GetUnitTransformMatrixQuery* query, GetUnitTransformMatrixResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	memset(result->matrix, 0, sizeof(result->matrix));

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	// Get the transform matrix
	const CMatrix44f& mat = unit->GetTransformMatrix(false);

	// Copy matrix data (CMatrix44f stores in column-major order)
	for (int i = 0; i < 16; ++i) {
		result->matrix[i] = mat[i];
	}
}

static void NativeGetUnitSelectionVolumeData(const GetUnitSelectionVolumeDataQuery* query, GetUnitSelectionVolumeDataResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->scales = {0.0f, 0.0f, 0.0f};
	result->offsets = {0.0f, 0.0f, 0.0f};
	result->volumeType = 0;
	result->useContHitTest = false;
	result->primaryAxis = 0;
	result->ignoreHits = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const CollisionVolume& vol = unit->selectionVolume;

	result->scales.x = vol.GetScales().x;
	result->scales.y = vol.GetScales().y;
	result->scales.z = vol.GetScales().z;

	result->offsets.x = vol.GetOffsets().x;
	result->offsets.y = vol.GetOffsets().y;
	result->offsets.z = vol.GetOffsets().z;

	result->volumeType = vol.GetVolumeType();
	result->useContHitTest = vol.UseContHitTest();
	result->primaryAxis = vol.GetPrimaryAxis();
	result->ignoreHits = vol.IgnoreHits();
}

static void NativeGetUnitIconData(const GetUnitIconDataQuery* query, GetUnitIconDataResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->iconName = "";
	result->atlasTexCoords[0] = 0.0f;
	result->atlasTexCoords[1] = 0.0f;
	result->atlasTexCoords[2] = 0.0f;
	result->atlasTexCoords[3] = 0.0f;
	result->size = 0.0f;
	result->distance = 0.0f;
	result->radiusAdjust = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const auto iconIdx = unit->currentIconIndex;

	if (iconIdx < 0 || iconIdx >= icon::iconHandler.GetIconsData().size()) {
		// Invalid icon index, return empty data
		return;
	}

	const auto& iconData = icon::iconHandler.GetIconsData()[iconIdx];

	// Allocate space for icon name in scratch buffer
	const std::string& iconName = iconData.GetName();
	const size_t nameLen = iconName.length();
	if (bufferPos + nameLen + 1 > sizeof(scratchBuffer)) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	char* nameBuf = reinterpret_cast<char*>(scratchBuffer + bufferPos);
	memcpy(nameBuf, iconName.c_str(), nameLen + 1);
	bufferPos += nameLen + 1;

	result->iconName = nameBuf;
	const auto& texCoords = iconData.GetSrcTexCoords();
	result->atlasTexCoords[0] = texCoords.x1;
	result->atlasTexCoords[1] = texCoords.y1;
	result->atlasTexCoords[2] = texCoords.x2;
	result->atlasTexCoords[3] = texCoords.y2;

	if (query->fullData) {
		result->size = iconData.GetSize();
		result->distance = iconData.GetDistance();
		result->radiusAdjust = iconData.GetRadiusAdjust();
	}
}

static void NativeGetUnitIcon(const GetUnitIconQuery* query, GetUnitIconResult* result)
{
	GetUnitIconDataQuery dataQuery = {
		.unitID = query->unitID,
		.fullData = false,
	};
	GetUnitIconDataResult dataResult = {};
	NativeGetUnitIconData(&dataQuery, &dataResult);

	result->error = dataResult.error;
	result->iconName = dataResult.iconName;
	result->atlasTexCoords[0] = dataResult.atlasTexCoords[0];
	result->atlasTexCoords[1] = dataResult.atlasTexCoords[1];
	result->atlasTexCoords[2] = dataResult.atlasTexCoords[2];
	result->atlasTexCoords[3] = dataResult.atlasTexCoords[3];
	result->size = dataResult.size;
	result->distance = dataResult.distance;
	result->radiusAdjust = dataResult.radiusAdjust;
}

static void NativeGetClipboard(const GetClipboardQuery* query, GetClipboardResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->text = "";

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const std::string text = CClipboard().GetContents();
	const size_t len = text.size();

	if (bufferPos + len + 1 > sizeof(scratchBuffer)) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	char* buf = reinterpret_cast<char*>(scratchBuffer + bufferPos);
	memcpy(buf, text.c_str(), len + 1);
	bufferPos += len + 1;

	result->text = buf;
}

static void NativeGetGameSecondsInterpolated(const GetGameSecondsInterpolatedQuery* query, GetGameSecondsInterpolatedResult* result)
{
	(void)query;
	result->error = nullptr;
	result->seconds = 0.0f;

	if (gs == nullptr || globalRendering == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->seconds = (gs->GetLuaSimFrame() + globalRendering->timeOffset) / GAME_SPEED;
}

// ============================================================================
// Camera queries
// ============================================================================

static void NativeGetCameraRotation(const GetCameraRotationQuery* /*query*/, GetCameraRotationResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->rotX = result->rotY = result->rotZ = 0.0f;

	if (!IsReady() || camera == nullptr) {
		result->error = &CAMERA_UNAVAILABLE_ERROR;
		return;
	}

	const float3& rot = camera->GetRot();
	result->rotX = rot.x;
	result->rotY = rot.y;
	result->rotZ = rot.z;
}

static void NativeGetCameraVectors(const GetCameraVectorsQuery* /*query*/, GetCameraVectorsResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->forward = {0.0f, 0.0f, 0.0f};
	result->up = {0.0f, 0.0f, 0.0f};
	result->right = {0.0f, 0.0f, 0.0f};

	if (!IsReady() || camera == nullptr) {
		result->error = &CAMERA_UNAVAILABLE_ERROR;
		return;
	}

	const float3 dir = camera->GetDir();
	const float3 up = camera->GetUp();
	const float3 right = camera->GetRight();

	result->forward = {dir.x, dir.y, dir.z};
	result->up = {up.x, up.y, up.z};
	result->right = {right.x, right.y, right.z};
}

static void NativeGetFrustumPlanes(const GetFrustumPlanesQuery* /*query*/, GetFrustumPlanesResult* result)
{
	bufferPos = 0;
	result->error = nullptr;

	if (!IsReady() || camera == nullptr) {
		result->error = &CAMERA_UNAVAILABLE_ERROR;
		return;
	}

	const float4 planes[4] = {
		camera->GetFrustumPlane(CCamera::FRUSTUM_PLANE_TOP),
		camera->GetFrustumPlane(CCamera::FRUSTUM_PLANE_BOT),
		camera->GetFrustumPlane(CCamera::FRUSTUM_PLANE_LFT),
		camera->GetFrustumPlane(CCamera::FRUSTUM_PLANE_RGT)
	};

	for (int i = 0; i < 4; ++i) {
		result->planes[i * 4 + 0] = planes[i].x;
		result->planes[i * 4 + 1] = planes[i].y;
		result->planes[i * 4 + 2] = planes[i].z;
		result->planes[i * 4 + 3] = planes[i].w;
	}
}

// ============================================================================
// Visibility queries
// ============================================================================

static void NativeGetVisibleUnits(const GetVisibleUnitsQuery* query, GetVisibleUnitsResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->unitIDs = nullptr;
	result->count = 0;

	if (!IsReady() || camera == nullptr || readMap == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const bool includeIcons = query->includeIcons;
	const float testRadius = std::max(query->radius, -query->radius);
	const bool useUnitRadius = (query->radius >= 0.0f);

	const bool fullView = (gu != nullptr) && gu->spectatingFullView;
	const int readAllyTeam = (gu != nullptr) ? gu->myAllyTeam : -1;
	const int readTeam = (gu != nullptr) ? gu->myTeam : -1;

	static CVisUnitQuadDrawer unitQuadIter;
	unitQuadIter.ResetState();
	readMap->GridVisibility(nullptr, &unitQuadIter, 1e9, CQuadField::BASE_QUAD_SIZE / SQUARE_SIZE);

	int32_t* ids = reinterpret_cast<int32_t*>(&scratchBuffer[bufferPos]);
	uint32_t count = 0;
	const int tempNum = gs != nullptr ? gs->GetTempNum() : 0;

	for (auto visUnitList : unitQuadIter.GetObjectLists()) {
		for (CUnit* unit : *visUnitList) {
			if (unit->tempNum == tempNum)
				continue;
			unit->tempNum = tempNum;

			if (!UnitMatchesAllegiance(unit, query->teamID, readTeam, readAllyTeam))
				continue;

			if (!UnitVisibleToClient(unit, readAllyTeam, fullView, !includeIcons))
				continue;

			if (!camera->InView(unit->drawMidPos, testRadius + (useUnitRadius ? unit->GetDrawRadius() : 0.0f)))
				continue;

			if (bufferPos + sizeof(int32_t) > sizeof(scratchBuffer)) {
				result->error = &BUFFER_OVERFLOW_ERROR;
				return;
			}

			ids[count++] = unit->id;
			bufferPos += sizeof(int32_t);
		}
	}

	result->unitIDs = ids;
	result->count = count;
}

static void NativeGetVisibleFeatures(const GetVisibleFeaturesQuery* query, GetVisibleFeaturesResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->featureIDs = nullptr;
	result->count = 0;

	if (!IsReady() || camera == nullptr || readMap == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const bool includeIcons = query->includeIcons;
	const bool includeGeos = query->includeGeos;
	const float testRadius = std::max(query->radius, -query->radius);
	const bool useFeatureRadius = (query->radius >= 0.0f);
	const bool fullView = (gu != nullptr) && gu->spectatingFullView;
	const int readAllyTeam = (gu != nullptr) ? gu->myAllyTeam : query->allyTeamID;
	const int allyTeamID = (query->allyTeamID >= 0) ? query->allyTeamID : readAllyTeam;

	static CVisFeatureQuadDrawer featureIter;
	featureIter.ResetState();
	readMap->GridVisibility(nullptr, &featureIter, 1e9, CQuadField::BASE_QUAD_SIZE / SQUARE_SIZE);

	int32_t* ids = reinterpret_cast<int32_t*>(&scratchBuffer[bufferPos]);
	uint32_t count = 0;
	const int tempNum = gs != nullptr ? gs->GetTempNum() : 0;

	for (auto visFeatureList : featureIter.GetObjectLists()) {
		for (CFeature* feature : *visFeatureList) {
			if (feature->tempNum == tempNum)
				continue;
			feature->tempNum = tempNum;

			if (feature->noDraw)
				continue;

			if (!includeIcons && feature->drawFlag == DrawFlags::SO_DRICON_FLAG)
				continue;

			if (!includeGeos && feature->def != nullptr && feature->def->geoThermal)
				continue;

			if (!fullView && allyTeamID >= 0 && !feature->IsInLosForAllyTeam(allyTeamID))
				continue;

			if (!camera->InView(feature->drawMidPos, testRadius + (useFeatureRadius ? feature->GetDrawRadius() : 0.0f)))
				continue;

			if (bufferPos + sizeof(int32_t) > sizeof(scratchBuffer)) {
				result->error = &BUFFER_OVERFLOW_ERROR;
				return;
			}

			ids[count++] = feature->id;
			bufferPos += sizeof(int32_t);
		}
	}

	result->featureIDs = ids;
	result->count = count;
}

static void NativeGetVisibleProjectiles(const GetVisibleProjectilesQuery* query, GetVisibleProjectilesResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->projectileIDs = nullptr;
	result->count = 0;

	if (!IsReady() || camera == nullptr || readMap == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const bool includeWeapon = query->includeWeaponProjectiles;
	const bool includePiece = query->includePieceProjectiles;
	(void)query->includeSyncedProjectiles;
	const bool fullView = (gu != nullptr) && gu->spectatingFullView;
	const int readAllyTeam = (gu != nullptr) ? gu->myAllyTeam : query->allyTeamID;
	const int allyTeamID = (query->allyTeamID >= 0) ? query->allyTeamID : readAllyTeam;

	static CVisProjectileQuadDrawer projIter;
	projIter.ResetState();
	readMap->GridVisibility(nullptr, &projIter, 1e9, CQuadField::BASE_QUAD_SIZE / SQUARE_SIZE);

	int32_t* ids = reinterpret_cast<int32_t*>(&scratchBuffer[bufferPos]);
	uint32_t count = 0;
	const int tempNum = gs != nullptr ? gs->GetTempNum() : 0;

	for (auto visProjectileList : projIter.GetObjectLists()) {
		for (CProjectile* p : *visProjectileList) {
			if (p->tempNum == tempNum)
				continue;
			p->tempNum = tempNum;

			if (!p->synced)
				continue;

			if (!fullView && allyTeamID >= 0 && !losHandler->InLos(p, allyTeamID))
				continue;

			if (!camera->InView(p->pos, p->GetDrawRadius()))
				continue;

			if (!includeWeapon && p->weapon)
				continue;
			if (!includePiece && p->piece)
				continue;

			if (bufferPos + sizeof(int32_t) > sizeof(scratchBuffer)) {
				result->error = &BUFFER_OVERFLOW_ERROR;
				return;
			}

			ids[count++] = p->id;
			bufferPos += sizeof(int32_t);
		}
	}

	result->projectileIDs = ids;
	result->count = count;
}

static void NativeGetUnitsInScreenRectangle(const GetUnitsInScreenRectangleQuery* query, GetUnitsInScreenRectangleResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->unitIDs = nullptr;
	result->count = 0;

	if (!IsReady() || camera == nullptr || readMap == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	float l = query->left;
	float t = query->top;
	float r = query->right;
	float b = query->bottom;

	if (l > r) std::swap(l, r);
	if (t > b) std::swap(t, b);

	const bool fullView = (gu != nullptr) && gu->spectatingFullView;
	const int readAllyTeam = (gu != nullptr) ? gu->myAllyTeam : -1;
	const int readTeam = (gu != nullptr) ? gu->myTeam : -1;

	static CVisUnitQuadDrawer unitIter;
	unitIter.ResetState();
	readMap->GridVisibility(nullptr, &unitIter, 1e9, CQuadField::BASE_QUAD_SIZE / SQUARE_SIZE);

	int32_t* ids = reinterpret_cast<int32_t*>(&scratchBuffer[bufferPos]);
	uint32_t count = 0;
	const int tempNum = gs != nullptr ? gs->GetTempNum() : 0;

	for (auto visUnitList : unitIter.GetObjectLists()) {
		for (CUnit* unit : *visUnitList) {
			if (unit->tempNum == tempNum)
				continue;
			unit->tempNum = tempNum;

			if (!UnitVisibleForScreenRectangle(unit, query->allegiance, readTeam, readAllyTeam, fullView))
				continue;

			const float3 vpPos = camera->CalcViewPortCoordinates(unit->drawPos);

			if (vpPos.x > r || vpPos.x < l)
				continue;
			if (vpPos.y > b || vpPos.y < t)
				continue;
			if (vpPos.z > 1.0f || vpPos.z < 0.0f)
				continue;

			if (bufferPos + sizeof(int32_t) > sizeof(scratchBuffer)) {
				result->error = &BUFFER_OVERFLOW_ERROR;
				return;
			}

			ids[count++] = unit->id;
			bufferPos += sizeof(int32_t);
		}
	}

	result->unitIDs = ids;
	result->count = count;
}

static void NativeGetFeaturesInScreenRectangle(const GetFeaturesInScreenRectangleQuery* query, GetFeaturesInScreenRectangleResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->featureIDs = nullptr;
	result->count = 0;

	if (!IsReady() || camera == nullptr || readMap == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	float l = query->left;
	float t = query->top;
	float r = query->right;
	float b = query->bottom;

	if (l > r) std::swap(l, r);
	if (t > b) std::swap(t, b);

	static CVisFeatureQuadDrawer featureIter;
	featureIter.ResetState();
	readMap->GridVisibility(nullptr, &featureIter, 1e9, CQuadField::BASE_QUAD_SIZE / SQUARE_SIZE);

	int32_t* ids = reinterpret_cast<int32_t*>(&scratchBuffer[bufferPos]);
	uint32_t count = 0;
	const int tempNum = gs != nullptr ? gs->GetTempNum() : 0;

	for (auto visFeatureList : featureIter.GetObjectLists()) {
		for (CFeature* feature : *visFeatureList) {
			if (feature->tempNum == tempNum)
				continue;
			feature->tempNum = tempNum;

			if (featureHandler.GetFeature(feature->id) != feature)
				continue;

			const float3 vpPos = camera->CalcViewPortCoordinates(feature->drawPos);

			if (vpPos.x > r || vpPos.x < l)
				continue;
			if (vpPos.y > b || vpPos.y < t)
				continue;
			if (vpPos.z > 1.0f || vpPos.z < 0.0f)
				continue;

			if (bufferPos + sizeof(int32_t) > sizeof(scratchBuffer)) {
				result->error = &BUFFER_OVERFLOW_ERROR;
				return;
			}

			ids[count++] = feature->id;
			bufferPos += sizeof(int32_t);
		}
	}

	result->featureIDs = ids;
	result->count = count;
}

static void NativeIsUnitVisible(const IsUnitVisibleQuery* query, IsUnitVisibleResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->visible = false;

	if (!IsReady() || camera == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const bool fullView = (gu != nullptr) && gu->spectatingFullView;
	const int readAllyTeam = (gu != nullptr) ? gu->myAllyTeam : -1;

	if (!UnitVisibleToClient(unit, readAllyTeam, fullView, query->checkIcon))
		return;

	const float radius = (query->radius == 0.0f) ? unit->radius : query->radius;
	result->visible = camera->InView(unit->midPos, radius);
}

static void NativeIsUnitInView(const IsUnitInViewQuery* query, IsUnitInViewResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->inView = false;

	if (!IsReady() || camera == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->inView = camera->InView(unit->midPos, unit->radius);
}

static void NativeIsUnitIcon(const IsUnitIconQuery* query, IsUnitIconResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->isIcon = false;

	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	result->isIcon = unit->GetIsIcon();
}

// ====================================================================
// Additional stubs
// ====================================================================

static void NativeGetActiveCmdDesc(const GetActiveCmdDescQuery* query, GetActiveCmdDescResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->cmdDesc = {};
	result->hasCommand = false;

	if (guihandler == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const int cmdIndex = query->cmdIndex - CMD_INDEX_OFFSET;
	const auto& cmdDescs = guihandler->commands;
	if (cmdIndex < 0 || cmdIndex >= static_cast<int>(cmdDescs.size()))
		return;

	if (!FillActiveCommandDescription(cmdDescs[cmdIndex], result->cmdDesc)) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	result->hasCommand = true;
}

static void NativeGetActiveCmdDescs(const GetActiveCmdDescsQuery* /*query*/, GetActiveCmdDescsResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->cmdDescs = nullptr;
	result->count = 0;

	if (guihandler == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const auto& cmdDescs = guihandler->commands;
	if (cmdDescs.empty())
		return;

	result->cmdDescs = AllocateArray<ActiveCommandDescription>(cmdDescs.size());
	if (result->cmdDescs == nullptr) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		return;
	}

	for (size_t i = 0; i < cmdDescs.size(); ++i) {
		if (!FillActiveCommandDescription(cmdDescs[i], result->cmdDescs[i])) {
			result->error = &BUFFER_OVERFLOW_ERROR;
			result->count = static_cast<uint32_t>(i);
			return;
		}
	}

	result->count = static_cast<uint32_t>(cmdDescs.size());
}

static void NativeGetCmdDescIndex(const GetCmdDescIndexQuery* query, GetCmdDescIndexResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->index = -1;

	if (guihandler == nullptr) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const auto& cmdDescs = guihandler->commands;
	for (size_t i = 0; i < cmdDescs.size(); ++i) {
		if (cmdDescs[i].id == query->cmdID) {
			result->index = static_cast<int32_t>(i + CMD_INDEX_OFFSET);
			return;
		}
	}
}

static void NativeGetBoxSelectionByEngine(const GetBoxSelectionByEngineQuery* /*query*/, GetBoxSelectionByEngineResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->enabled = selectedUnitsHandler.GetBoxSelectionHandledByEngine();
}

static void NativeGetBuildFacing(const GetBuildFacingQuery* /*query*/, GetBuildFacingResult* result) {
	bufferPos = 0;
	if (guihandler == nullptr) {
		result->error = &NOT_READY_ERROR;
		result->facing = -1;
		return;
	}
	result->error = nullptr;
	result->facing = guihandler->buildFacing;
}

static void NativeGetBuildSpacing(const GetBuildSpacingQuery* /*query*/, GetBuildSpacingResult* result) {
	bufferPos = 0;
	if (guihandler == nullptr) {
		result->error = &NOT_READY_ERROR;
		result->spacing = -1;
		return;
	}
	result->error = nullptr;
	result->spacing = guihandler->buildSpacing;
}

static void NativeGetDrawSelectionInfo(const GetDrawSelectionInfoQuery* /*query*/, GetDrawSelectionInfoResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->draw = (guihandler != nullptr) && guihandler->GetDrawSelectionInfo();
}

static void NativeGetNanoProjectileParams(const GetNanoProjectileParamsQuery* /*query*/, GetNanoProjectileParamsResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->r = CNanoProjectile::rotVal0 * math::RAD_TO_DEG;
	result->v = CNanoProjectile::rotVel0 * (math::RAD_TO_DEG * GAME_SPEED);
	result->a = CNanoProjectile::rotAcc0 * (math::RAD_TO_DEG * (GAME_SPEED * GAME_SPEED));
	result->randR = CNanoProjectile::rotValRng0 * math::RAD_TO_DEG;
	result->randV = CNanoProjectile::rotVelRng0 * (math::RAD_TO_DEG * GAME_SPEED);
	result->randA = CNanoProjectile::rotAccRng0 * (math::RAD_TO_DEG * (GAME_SPEED * GAME_SPEED));
}

static void NativeGetPieceProjectileName(const GetPieceProjectileNameQuery* query, GetPieceProjectileNameResult* result) {
	bufferPos = 0;
	const CProjectile* proj = projectileHandler.GetProjectileByUnsyncedID(query->projectileID);
	if (proj == nullptr || !proj->piece) {
		result->error = nullptr;
		result->name = "";
		return;
	}

	const CPieceProjectile* pproj = dynamic_cast<const CPieceProjectile*>(proj);
	if (pproj == nullptr || pproj->omp == nullptr) {
		result->error = nullptr;
		result->name = "";
		return;
	}

	result->error = nullptr;
	result->name = CopyToScratch(pproj->omp->name);
}

static void NativeGetTeamDamageStats(const GetTeamDamageStatsQuery* query, GetTeamDamageStatsResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;
	result->damageDealt = 0.0f;
	result->damageReceived = 0.0f;

	if (game == nullptr || !teamHandler.IsValidTeam(query->teamID)) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	const CTeam* team = teamHandler.Team(query->teamID);
	const int myAlly = (gu != nullptr) ? gu->myAllyTeam : -1;
	const int teamAlly = teamHandler.AllyTeam(query->teamID);

	if (myAlly >= 0 && !teamHandler.Ally(teamAlly, myAlly) && !game->IsGameOver())
		return;

	const TeamStatistics& stats = team->GetCurrentStats();
	result->damageDealt = stats.damageDealt;
	result->damageReceived = stats.damageReceived;
	result->success = true;
}

static void NativeGetLastMessagePositions(const GetLastMessagePositionsQuery* /*query*/, GetLastMessagePositionsResult* result) {
	bufferPos = 0;
	if (infoConsole == nullptr) {
		result->error = &NOT_READY_ERROR;
		result->positions = nullptr;
		result->count = 0;
		return;
	}

	const unsigned int count = infoConsole->GetMsgPosCount();
	const size_t needed = count * sizeof(Float3);
	if (bufferPos + needed > sizeof(scratchBuffer)) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		result->positions = nullptr;
		result->count = 0;
		return;
	}

	Float3* positions = reinterpret_cast<Float3*>(&scratchBuffer[bufferPos]);
	for (unsigned int i = 0; i < count; ++i) {
		const float3 msg = infoConsole->GetMsgPos();
		positions[i].x = msg.x;
		positions[i].y = msg.y;
		positions[i].z = msg.z;
	}
	bufferPos += needed;

	result->error = nullptr;
	result->positions = positions;
	result->count = count;
}

static void NativeSolveNURBSCurve(const SolveNURBSCurveQuery* query, SolveNURBSCurveResult* result) {
	bufferPos = 0;
	if (query->points == nullptr || query->knots == nullptr || query->pointCount == 0 || query->knotCount == 0 || query->segments <= 0) {
		result->error = &INVALID_UNIT_ERROR;
		result->success = false;
		result->points = nullptr;
		result->count = 0;
		return;
	}

	std::vector<float4> cpoints;
	cpoints.reserve(query->pointCount);
	for (uint32_t i = 0; i < query->pointCount; ++i) {
		const Float4& p = query->points[i];
		cpoints.emplace_back(p.x, p.y, p.z, p.w);
	}

	std::vector<float> knots(query->knots, query->knots + query->knotCount);
	const auto solved = NURBS::SolveNURBSCurve(query->degree, cpoints, knots, query->segments);

	const size_t needed = solved.size() * sizeof(Float3);
	if (bufferPos + needed > sizeof(scratchBuffer)) {
		result->error = &BUFFER_OVERFLOW_ERROR;
		result->success = false;
		result->points = nullptr;
		result->count = 0;
		return;
	}

	Float3* out = reinterpret_cast<Float3*>(&scratchBuffer[bufferPos]);
	for (size_t i = 0; i < solved.size(); ++i) {
		out[i].x = solved[i].x;
		out[i].y = solved[i].y;
		out[i].z = solved[i].z;
	}
	bufferPos += needed;

	result->error = nullptr;
	result->points = out;
	result->count = static_cast<uint32_t>(solved.size());
	result->success = true;
}

static void NativeIsUnitSelected(const IsUnitSelectedQuery* query, IsUnitSelectedResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->selected = false;

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const auto& selUnits = selectedUnitsHandler.selectedUnits;
	result->selected = (selUnits.find(unit->id) != selUnits.end());
}

static void NativeIsUnitAllied(const IsUnitAlliedQuery* query, IsUnitAlliedResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->allied = false;

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	const int myAlly = (gu != nullptr) ? gu->myAllyTeam : -1;
	if (myAlly >= 0) {
		result->allied = (unit->allyteam == myAlly);
	}
}

static void NativeGetCustomPaletteColor(const GetCustomPaletteColorQuery* query, GetCustomPaletteColorResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->r = 0.0f;
	result->g = 0.0f;
	result->b = 0.0f;
	result->success = false;

	if (query->index < 0 || query->index >= MAX_CUSTOM_COLORS) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const float4 color = customColorPalette.GetColor(static_cast<uint16_t>(query->index));
	result->r = color.x;
	result->g = color.y;
	result->b = color.z;
	result->success = true;
}

static void NativeGetUnitPaletteIndex(const GetUnitPaletteIndexQuery* query, GetUnitPaletteIndexResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->customIndex = -1;
	result->usingCustomColor = false;

	const CUnit* unit = unitHandler.GetUnit(query->unitID);
	if (unit == nullptr) {
		result->error = &INVALID_UNIT_ERROR;
		return;
	}

	if (CCustomColorPalette::IsCustomPaletteIndex(unit->paletteIndex)) {
		result->customIndex = CCustomColorPalette::DecodePaletteIndex(unit->paletteIndex);
		result->usingCustomColor = true;
	}
}

static void NativeGetFeaturePaletteIndex(const GetFeaturePaletteIndexQuery* query, GetFeaturePaletteIndexResult* result)
{
	bufferPos = 0;
	result->error = nullptr;
	result->customIndex = -1;
	result->usingCustomColor = false;

	const CFeature* feature = featureHandler.GetFeature(query->featureID);
	if (feature == nullptr) {
		result->error = &INVALID_FEATURE_ERROR;
		return;
	}

	if (CCustomColorPalette::IsCustomPaletteIndex(feature->paletteIndex)) {
		result->customIndex = CCustomColorPalette::DecodePaletteIndex(feature->paletteIndex);
		result->usingCustomColor = true;
	}
}

static const UnitRenderingApi UNIT_RENDERING_API = {
	.GetUnitNoDraw = NativeGetUnitNoDraw,
	.GetUnitLuaDraw = NativeGetUnitLuaDraw,
	.GetUnitEngineDrawMask = NativeGetUnitEngineDrawMask,
	.GetUnitAlwaysUpdateMatrix = NativeGetUnitAlwaysUpdateMatrix,
	.GetUnitDrawFlag = NativeGetUnitDrawFlag,
	.GetUnitNoSelect = NativeGetUnitNoSelect,
	.GetUnitNoMinimap = NativeGetUnitNoMinimap,
	.GetUnitNoGroup = NativeGetUnitNoGroup,
	.GetUnitViewPosition = NativeGetUnitViewPosition,
	.GetUnitTransformMatrix = NativeGetUnitTransformMatrix,
	.GetUnitSelectionVolumeData = NativeGetUnitSelectionVolumeData,
	.GetUnitIconData = NativeGetUnitIconData,
	.GetUnitIcon = NativeGetUnitIcon,
	.GetCameraRotation = NativeGetCameraRotation,
	.GetCameraVectors = NativeGetCameraVectors,
	.GetFrustumPlanes = NativeGetFrustumPlanes,
	.GetVisibleUnits = NativeGetVisibleUnits,
	.GetVisibleFeatures = NativeGetVisibleFeatures,
	.GetVisibleProjectiles = NativeGetVisibleProjectiles,
	.GetUnitsInScreenRectangle = NativeGetUnitsInScreenRectangle,
	.GetFeaturesInScreenRectangle = NativeGetFeaturesInScreenRectangle,
	.IsUnitVisible = NativeIsUnitVisible,
	.IsUnitInView = NativeIsUnitInView,
	.IsUnitIcon = NativeIsUnitIcon
};

} // namespace

// ============================================================================
// Public API Export
// ============================================================================

const UnsyncedReadApi UNSYNCED_READ_API = {
	.unitRendering = &UNIT_RENDERING_API,
	.GetClipboard = NativeGetClipboard,
	.GetActiveCmdDesc = NativeGetActiveCmdDesc,
	.GetActiveCmdDescs = NativeGetActiveCmdDescs,
	.GetCmdDescIndex = NativeGetCmdDescIndex,
	.GetBoxSelectionByEngine = NativeGetBoxSelectionByEngine,
	.GetBuildFacing = NativeGetBuildFacing,
	.GetBuildSpacing = NativeGetBuildSpacing,
	.GetDrawSelectionInfo = NativeGetDrawSelectionInfo,
	.GetNanoProjectileParams = NativeGetNanoProjectileParams,
	.GetPieceProjectileName = NativeGetPieceProjectileName,
	.GetTeamDamageStats = NativeGetTeamDamageStats,
	.GetLastMessagePositions = NativeGetLastMessagePositions,
	.SolveNURBSCurve = NativeSolveNURBSCurve,
	.IsUnitSelected = NativeIsUnitSelected,
	.IsUnitAllied = NativeIsUnitAllied,
	.GetCustomPaletteColor = NativeGetCustomPaletteColor,
	.GetUnitPaletteIndex = NativeGetUnitPaletteIndex,
	.GetFeaturePaletteIndex = NativeGetFeaturePaletteIndex,
	.GetGameSecondsInterpolated = NativeGetGameSecondsInterpolated,
};
