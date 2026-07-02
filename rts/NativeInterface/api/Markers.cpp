/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "Markers.h"

#include "Game/InMapDraw.h"
#include "Game/InMapDrawModel.h"
#include "Game/GlobalUnsynced.h"
#include "Game/UI/CursorIcons.h"
#include "Sim/Units/UnitDefHandler.h"
#include "Sim/Misc/TeamHandler.h"

namespace {

static const Error INVALID_PARAM_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid parameters" };
static const Error DRAWER_UNAVAILABLE_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "InMapDrawer not available" };

static void NativeAddWorldIcon(const AddWorldIconQuery* query, AddWorldIconResult* result)
{
	result->error = nullptr;
	result->success = false;

	cursorIcons.AddIcon(query->cmdID, float3(query->pos.x, query->pos.y, query->pos.z));
	result->success = true;
}

static void NativeAddWorldText(const AddWorldTextQuery* query, AddWorldTextResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (query->text == nullptr) {
		result->error = &INVALID_PARAM_ERROR;
		return;
	}

	cursorIcons.AddIconText(query->text, float3(query->pos.x, query->pos.y, query->pos.z));
	result->success = true;
}

static void NativeAddWorldUnit(const AddWorldUnitQuery* query, AddWorldUnitResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (!unitDefHandler->IsValidUnitDefID(query->unitDefID) || !teamHandler.IsValidTeam(query->teamID)) {
		result->error = &INVALID_PARAM_ERROR;
		return;
	}

	cursorIcons.AddBuildIcon(-query->unitDefID, float3(query->pos.x, query->pos.y, query->pos.z), query->teamID, query->facing);
	result->success = true;
}

static void NativeMarkerAddPoint(const MarkerAddPointQuery* query, MarkerAddPointResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (inMapDrawer == nullptr) {
		result->error = &DRAWER_UNAVAILABLE_ERROR;
		return;
	}

	const float3 pos(query->pos.x, query->pos.y, query->pos.z);
	const std::string text = (query->text != nullptr) ? query->text : "";
	const int playerID = (query->playerID >= 0) ? query->playerID : gu->myPlayerNum;

	if (query->localOnly) {
		inMapDrawerModel->AddPoint(pos, text, playerID);
	} else {
		inMapDrawer->SendPoint(pos, text, true);
	}

	result->success = true;
}

static void NativeMarkerAddLine(const MarkerAddLineQuery* query, MarkerAddLineResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (inMapDrawer == nullptr) {
		result->error = &DRAWER_UNAVAILABLE_ERROR;
		return;
	}

	const float3 pos1(query->from.x, query->from.y, query->from.z);
	const float3 pos2(query->to.x, query->to.y, query->to.z);
	const int playerID = (query->playerID >= 0) ? query->playerID : gu->myPlayerNum;

	if (query->localOnly) {
		inMapDrawerModel->AddLine(pos1, pos2, playerID);
	} else {
		inMapDrawer->SendLine(pos1, pos2, true);
	}

	result->success = true;
}

static void NativeMarkerErasePosition(const MarkerErasePositionQuery* query, MarkerErasePositionResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (inMapDrawer == nullptr) {
		result->error = &DRAWER_UNAVAILABLE_ERROR;
		return;
	}

	const float3 pos(query->pos.x, query->pos.y, query->pos.z);
	(void)query->unused;
	const int playerID = (query->playerID >= 0) ? query->playerID : gu->myPlayerNum;

	if (query->localOnly) {
		const bool alwaysErase = query->alwaysErase && gu->spectating;
		inMapDrawerModel->EraseNear(pos, playerID, alwaysErase);
	} else {
		inMapDrawer->SendErase(pos);
	}

	result->success = true;
}

} // namespace

const MarkersApi MARKERS_API = {
	.AddWorldIcon = NativeAddWorldIcon,
	.AddWorldText = NativeAddWorldText,
	.AddWorldUnit = NativeAddWorldUnit,
	.MarkerAddPoint = NativeMarkerAddPoint,
	.MarkerAddLine = NativeMarkerAddLine,
	.MarkerErasePosition = NativeMarkerErasePosition,
};
