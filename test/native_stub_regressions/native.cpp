/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "NativeInterface/NativeInterface.h"
#include <SDL2/SDL_video.h>
#include <GL/glcorearb.h>
#include <jsoncpp/json/json.h>
#include <cmath>
#include <cstdio>
#include <sstream>
#include <string>
#include <set>
#include <limits>

static bool failed = false;
static unsigned checks = 0;
static int moduleData;
static bool testingMutant = false;
static bool mutantDetected = false;
static std::set<std::string> killedMutants;

static void Check(bool valid, const char* name)
{
	++checks;
	if (valid) return;
	if (testingMutant) { mutantDetected = true; return; }
	failed = true;
	std::fprintf(stderr, "NO_STUBS_FAIL %s\n", name);
}

template<typename Query, typename Result> static void Zero(const Query*, Result* result)
{
	*result = {};
}

static Json::Value Decode(const char* text)
{
	Json::Value value;
	std::istringstream input(text ? text : "");
	try { input >> value; }
	catch (...) { Check(false, "custom options must be JSON"); }
	return value;
}

// GL is supplied by the engine's current context, without linking a second
// loader or assuming that native modules share the engine's GLAD globals.
template<typename T> static T Proc(const char* name)
{
	auto function = reinterpret_cast<T>(SDL_GL_GetProcAddress(name));
	Check(function != nullptr, name);
	return function;
}

static void CheckFramebuffers(NativeInterface* api)
{
#define GL_PROC(name, type) const auto name = Proc<type>(#name)
	GL_PROC(glGetIntegerv, PFNGLGETINTEGERVPROC);
	GL_PROC(glGetError, PFNGLGETERRORPROC);
	GL_PROC(glGenFramebuffers, PFNGLGENFRAMEBUFFERSPROC);
	GL_PROC(glBindFramebuffer, PFNGLBINDFRAMEBUFFERPROC);
	GL_PROC(glDeleteFramebuffers, PFNGLDELETEFRAMEBUFFERSPROC);
	GL_PROC(glGenTextures, PFNGLGENTEXTURESPROC);
	GL_PROC(glBindTexture, PFNGLBINDTEXTUREPROC);
	GL_PROC(glTexImage2D, PFNGLTEXIMAGE2DPROC);
	GL_PROC(glDeleteTextures, PFNGLDELETETEXTURESPROC);
	GL_PROC(glFramebufferTexture2D, PFNGLFRAMEBUFFERTEXTURE2DPROC);
	GL_PROC(glClearBufferfv, PFNGLCLEARBUFFERFVPROC);
	GL_PROC(glReadPixels, PFNGLREADPIXELSPROC);
#undef GL_PROC
	if (failed) return;
	GLint oldDraw, oldRead, oldTexture;
	glGetIntegerv(GL_DRAW_FRAMEBUFFER_BINDING, &oldDraw);
	glGetIntegerv(GL_READ_FRAMEBUFFER_BINDING, &oldRead);
	glGetIntegerv(GL_TEXTURE_BINDING_2D, &oldTexture);
	GLuint framebuffers[3], textures[3];
	glGenFramebuffers(3, framebuffers);
	glGenTextures(3, textures);
	for (int i = 0; i < 3; ++i) {
		glBindTexture(GL_TEXTURE_2D, textures[i]);
		glTexImage2D(GL_TEXTURE_2D, 0, i == 2 ? GL_R32I : GL_RGBA8, 1, 1, 0,
			i == 2 ? GL_RED_INTEGER : GL_RGBA, i == 2 ? GL_INT : GL_UNSIGNED_BYTE, nullptr);
		glBindFramebuffer(GL_FRAMEBUFFER, framebuffers[i]);
		glFramebufferTexture2D(GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, GL_TEXTURE_2D, textures[i], 0);
	}
	GfxClearAttachmentFBOQuery query{GL_FRAMEBUFFER, GL_COLOR_ATTACHMENT0, {0.25f, 0.5f, 0.75f, 1}, 4};
	GfxBoolResult result{};
	glBindFramebuffer(GL_FRAMEBUFFER, 0);
	api->gfx->ClearAttachmentFBO(&query, &result);
	Check(!result.value, "default framebuffer must not be cleared");
	glBindFramebuffer(GL_FRAMEBUFFER, framebuffers[1]);
	const float background[4] = {1, 0, 0, 1};
	glClearBufferfv(GL_COLOR, 0, background);
	glBindFramebuffer(GL_READ_FRAMEBUFFER, framebuffers[0]);
	query.target = GL_READ_FRAMEBUFFER;
	api->gfx->ClearAttachmentFBO(&query, &result);
	Check(!result.error && result.value, "clear requested read framebuffer");
	GLint bound;
	glGetIntegerv(GL_DRAW_FRAMEBUFFER_BINDING, &bound);
	Check(bound == GLint(framebuffers[1]), "restore unrelated draw binding");
	float pixel[4]{};
	glReadPixels(0, 0, 1, 1, GL_RGBA, GL_FLOAT, pixel);
	Check(std::abs(pixel[0] - .25f) < .01f && std::abs(pixel[1] - .5f) < .01f, "read target received clear");
	glBindFramebuffer(GL_READ_FRAMEBUFFER, framebuffers[1]);
	glReadPixels(0, 0, 1, 1, GL_RGBA, GL_FLOAT, pixel);
	Check(pixel[0] > .99f && pixel[1] < .01f, "unrelated draw framebuffer retained pixels");
	glBindFramebuffer(GL_FRAMEBUFFER, framebuffers[2]);
	query.target = GL_FRAMEBUFFER;
	query.values[0] = 23;
	query.count = 1;
	api->gfx->ClearAttachmentFBO(&query, &result);
	GLint integer = 0;
	glReadPixels(0, 0, 1, 1, GL_RED_INTEGER, GL_INT, &integer);
	Check(!result.error && result.value && integer == 23, "integer color attachment uses integer clear");
	query.values[0] = std::numeric_limits<float>::quiet_NaN();
	api->gfx->ClearAttachmentFBO(&query, &result);
	Check(result.error && !result.value, "non-finite integer clear rejected");
	glReadPixels(0, 0, 1, 1, GL_RED_INTEGER, GL_INT, &integer);
	Check(integer == 23, "invalid integer clear retains pixels");
	query.values[0] = 23;
	query.target = GL_TEXTURE_2D;
	api->gfx->ClearAttachmentFBO(&query, &result);
	Check(result.error && !result.value, "invalid target rejected");
	query.target = GL_FRAMEBUFFER;
	query.attachment = GL_COLOR_ATTACHMENT1;
	api->gfx->ClearAttachmentFBO(&query, &result);
	Check(!result.value, "missing attachment rejected");
	Check(glGetError() == GL_NO_ERROR, "framebuffer checks leave no GL error");
	glBindFramebuffer(GL_DRAW_FRAMEBUFFER, oldDraw);
	glBindFramebuffer(GL_READ_FRAMEBUFFER, oldRead);
	glBindTexture(GL_TEXTURE_2D, oldTexture);
	glDeleteFramebuffers(3, framebuffers);
	glDeleteTextures(3, textures);
}

extern "C" {
uint32_t NativeModuleApiVersion[] = {
	NATIVE_API_MAJOR(NATIVE_API_CURRENT_VERSION), NATIVE_API_MINOR(NATIVE_API_CURRENT_VERSION), NATIVE_API_PATCH(NATIVE_API_CURRENT_VERSION)
};

void InitializeNativeModule(NativeInterface*, const InitializeNativeModuleQuery* query, InitializeNativeModuleResult* result)
{
	*result = {nullptr, &moduleData, query->hostVersionMajor, query->hostVersionMinor, query->hostVersionPatch};
}

void HandleLuaCall(NativeInterface* api, void*, const HandleLuaCallQuery* query, HandleLuaCallResult* result)
{
	result->error = nullptr;
	std::istringstream input(std::string(query->message, query->messageLength));
	std::string command;
	input >> command;
	if (command == "move") {
		TestMoveOrderQuery q{};
		int expected;
		input >> q.unitDefID >> q.pos.x >> q.pos.y >> q.pos.z >> q.dir.x >> q.dir.y >> q.dir.z
			>> q.options.testTerrain >> q.options.testObjects >> q.options.centerOnly >> expected;
		TestMoveOrderResult r{};
		api->utils->TestMoveOrder(&q, &r);
		Check(!r.error && r.canMove == bool(expected), "movement square equals Lua");
	} else if (command == "shield") {
		GetUnitShieldStateQuery q{};
		int present, enabled;
		float power, alpha;
		input >> q.unitID >> q.weaponNum >> present >> enabled >> power >> alpha;
		GetUnitShieldStateResult r{};
		api->unitsInfo->GetUnitShieldState(&q, &r);
		Check(!r.error && r.hasShield == bool(present), "selected shield presence equals Lua");
		if (present) Check(r.shield.shieldEnabled == bool(enabled) && std::abs(r.shield.shieldPower-power) < .01f &&
			std::abs(r.shield.shieldAlpha-alpha) < .01f, "selected shield fields are real values");
	} else if (command == "hover") {
		GetUnitDefByIDQuery q{};
		bool expected;
		input >> q.unitDefID >> expected;
		GetUnitDefByIDResult r{};
		api->unitDefs->GetUnitDefByID(&q, &r);
		Check(!r.error && r.exists && r.physics.canHover == expected, "hovercraft differs from hovering aircraft");
	} else if (command == "turn") {
		GetUnitWeaponStateQuery q{};
		float expected;
		input >> q.unitID >> q.weaponNum >> expected;
		GetUnitWeaponStateResult r{};
		api->unitsWeapons->GetUnitWeaponState(&q, &r);
		Check(!r.error && expected != 0 && std::abs(r.state.turnRate-expected) < 1e-5f, "nonzero weapon turn rate");
	} else if (command == "factory") {
		GetFactoryCountsQuery q{};
		unsigned expected;
		input >> q.unitID >> expected;
		q.count = -1;
		q.addCmds = true;
		GetFactoryCountsResult r{};
		api->unitsCommands->GetFactoryCounts(&q, &r);
		Check(!r.error && r.info.currentCount == expected, "actual factory construction count");
	} else if (command == "options") {
		GetTeamInfoQuery team{0, true};
		GetTeamInfoResult tr{};
		api->teams->GetTeamInfo(&team, &tr);
		Check(!tr.error && Decode(tr.info.customKeys)["audit_key"] == "team-nonempty", "team custom keys are not empty");
		team.getTeamKeys = false;
		api->teams->GetTeamInfo(&team, &tr);
		Check(!tr.error && tr.info.customKeys == nullptr, "unrequested team keys remain absent");
		GetPlayerInfoQuery player{0, true};
		GetPlayerInfoResult pr{};
		api->teams->GetPlayerInfo(&player, &pr);
		Check(!pr.error && Decode(pr.info.customKeys)["audit_key"] == "player-nonempty", "player custom options are not empty");
	} else if (command == "piece") {
		GetPieceProjectileParamsQuery q{};
		int expectedPiece;
		input >> q.projectileID >> expectedPiece;
		GetPieceProjectileParamsResult r{};
		api->projectiles->GetPieceProjectileParams(&q, &r);
		Check(!r.error && r.isPieceProjectile && r.params.modelPieceNum == expectedPiece && r.params.modelObjectType > 0 &&
			r.params.modelName && std::string(r.params.modelName).find("fir_tree_small") != std::string::npos, "piece projectile carries its actual model");
	} else if (command == "log") {
		LogQuery q{"NativeStubAudit", 40, "audit section/level 100%"};
		LogResult r{};
		api->messages->Log(&q, &r);
		Check(!r.error && r.success, "section/level log accepted");
	} else if (command == "draw") {
		CheckFramebuffers(api);
	} else if (command == "finish") {
		for (const auto& name: killedMutants) std::fprintf(stderr,"NO_STUBS_MUTANT_KILLED %s\n",name.c_str());
		Check(killedMutants.size() == 10, "all audited zero-return mutants were killed");
		if (!failed && checks >= 30) std::fprintf(stderr, "NO_STUBS_PASS %u checks, %zu killed mutants\n", checks, killedMutants.size());
		else Check(false, "all audit cases executed");
	} else Check(false, "known audit command");

	// Mutate module-local API-table copies. The engine's production tables and
	// other callers are untouched. Replaying the same Lua observation must
	// expose a zero-return stub in at least one case for each audited function.
	if (testingMutant || command == "finish") return;
	NativeInterface copy = *api;
	UtilsApi utils = *api->utils;
	UnitsInfoApi unitsInfo = *api->unitsInfo;
	UnitsWeaponsApi weapons = *api->unitsWeapons;
	UnitDefsApi defs = *api->unitDefs;
	UnitsCommandsApi commands = *api->unitsCommands;
	MessagesApi messages = *api->messages;
	TeamsApi teams = *api->teams;
	ProjectilesApi projectiles = *api->projectiles;
	GfxApi gfx = *api->gfx;
	copy.utils=&utils;copy.unitsInfo=&unitsInfo;copy.unitsWeapons=&weapons;
	copy.unitDefs=&defs;copy.unitsCommands=&commands;copy.messages=&messages;
	copy.teams=&teams;copy.projectiles=&projectiles;copy.gfx=&gfx;
	std::string mutant;
	if (command == "move") { utils.TestMoveOrder=Zero<TestMoveOrderQuery,TestMoveOrderResult>;mutant="TestMoveOrder"; }
	if (command == "shield") { unitsInfo.GetUnitShieldState=Zero<GetUnitShieldStateQuery,GetUnitShieldStateResult>;mutant="GetUnitShieldState"; }
	if (command == "turn") { weapons.GetUnitWeaponState=Zero<GetUnitWeaponStateQuery,GetUnitWeaponStateResult>;mutant="GetUnitWeaponState"; }
	if (command == "hover") { defs.GetUnitDefByID=Zero<GetUnitDefByIDQuery,GetUnitDefByIDResult>;mutant="GetUnitDefByID"; }
	if (command == "factory") { commands.GetFactoryCounts=Zero<GetFactoryCountsQuery,GetFactoryCountsResult>;mutant="GetFactoryCounts"; }
	if (command == "piece") { projectiles.GetPieceProjectileParams=Zero<GetPieceProjectileParamsQuery,GetPieceProjectileParamsResult>;mutant="GetPieceProjectileParams"; }
	if (command == "log") { messages.Log=Zero<LogQuery,LogResult>;mutant="Log"; }
	if (command == "draw") { gfx.ClearAttachmentFBO=Zero<GfxClearAttachmentFBOQuery,GfxBoolResult>;mutant="ClearAttachmentFBO"; }
	if (command == "options") { teams.GetTeamInfo=Zero<GetTeamInfoQuery,GetTeamInfoResult>;mutant="GetTeamInfo"; }
	if (mutant.empty()) return;
	testingMutant=true;mutantDetected=false;
	HandleLuaCall(&copy,nullptr,query,result);
	if (mutantDetected) killedMutants.insert(mutant);
	if (command == "options") {
		teams=*api->teams;
		teams.GetPlayerInfo=Zero<GetPlayerInfoQuery,GetPlayerInfoResult>;
		mutantDetected=false;
		HandleLuaCall(&copy,nullptr,query,result);
		if (mutantDetected) killedMutants.insert("GetPlayerInfo");
	}
	testingMutant=false;
}
} // extern C
