/* This file is part of the Recoil engine (GPL v2 or later), see LICENSE.html */

#include "LuaDebugExtra.h"

#include "LuaInclude.h"
#include "LuaUtils.h"
#include "Lua/LuaRules.h"

#include "Game/GameController.h"
#include "Game/GameHelper.h"
#include "Game/UI/KeyBindings.h"
#include "Game/UI/KeyCodes.h"
#include "Game/UI/ScanCodes.h"
#include "Game/UI/MouseHandler.h"
#include "Lua/LuaMaterial.h"
#include "Rendering/GlobalRendering.h"
#include "Sim/Features/FeatureHandler.h"
#include "Sim/Misc/Resource.h"
#include "Sim/Projectiles/ProjectileHandler.h"
#include "Sim/Units/BuildInfo.h"
#include "Sim/Units/UnitHandler.h"
#include "Sim/Weapons/Weapon.h"
#include "Sim/Weapons/WeaponDefHandler.h"
#include "System/EventHandler.h"
#include "System/Input/KeyInput.h"
#include "System/Input/MouseInput.h"
#include "System/Platform/SDL1_keysym.h"
#include "System/Rectangle.h"
#include "Net/Protocol/NetMessageTypes.h"

#include <algorithm>
#include <map>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include <SDL_keyboard.h>
#include <SDL_keycode.h>
#include <SDL_mouse.h>


/******************************************************************************
 * debug input emulation
 *
 * Callouts that feed input to the engine as if it came from real hardware,
 * for headless regression tests. Emulated presses are held in a separate store
 * and OR'd into the real input state; an event fires only when the combined
 * (physical-or-emulated) state actually changes, so Lua never sees two Presses
 * or two Releases in a row.
 *
 * No engine-side access gate: the property that must not regress (no doubled
 * events) is structural, not access-controlled. A game that wants to restrict
 * these nils them out.
 *
 * @see rts/Lua/LuaDebugExtra.cpp
******************************************************************************/

bool LuaDebugExtra::PushEntries(lua_State* L)
{
	LuaPushNamedCFunc(L, "emulateKeyPress",     EmulateKeyPress);
	LuaPushNamedCFunc(L, "emulateKeyRelease",   EmulateKeyRelease);
	LuaPushNamedCFunc(L, "emulateMousePress",   EmulateMousePress);
	LuaPushNamedCFunc(L, "emulateMouseRelease", EmulateMouseRelease);
	LuaPushNamedCFunc(L, "emulateMouseMove",    EmulateMouseMove);
	LuaPushNamedCFunc(L, "emulateMouseWheel",   EmulateMouseWheel);
	LuaPushNamedCFunc(L, "emulateUnitMoveFailed", EmulateUnitMoveFailed);
	LuaPushNamedCFunc(L, "emulateNativeApiParityCallins", EmulateNativeApiParityCallins);
	LuaPushNamedCFunc(L, "emulateNativeApiParityUnimplementedCallin", EmulateNativeApiParityUnimplementedCallin);
	LuaPushNamedCFunc(L, "clearEmulatedInput",  ClearEmulatedInputLua);

	return true;
}


// shared body for emulateKeyPress/emulateKeyRelease; only the store update and
// the edge-fire differ, keyed on `pressed`
static int emulateKey(lua_State* L, bool pressed)
{
	if (activeController == nullptr)
		return 0;

	// Lua passes SDL1.2 keysyms; the held-state side (keyVec/IsKeyPressed) works in
	// raw SDL2, while the event side wants the normalized code like a real KEYDOWN
	const int rawKey = SDL12_keysyms(luaL_checkint(L, 1));

	// reject a junk keycode (unmapped -> SDLK_UNKNOWN). We deliberately do NOT
	// reject on an unknown scancode: headless has no keyboard layout, so
	// SDL_GetScancodeFromKey returns SDL_SCANCODE_UNKNOWN even for valid keys
	if (rawKey == SDLK_UNKNOWN)
		return 0;

	const SDL_Scancode sc = SDL_GetScancodeFromKey((SDL_Keycode)rawKey);
	const int eventKey = CKeyCodes::GetNormalizedSymbol(rawKey);
	const int scanCode = CScanCodes::GetNormalizedSymbol(sc);

	int numKeys = 0;
	const uint8_t* kbState = SDL_GetKeyboardState(&numKeys);
	const bool physicalDown = ((int)sc < numKeys && kbState[sc] != 0);

	// effective (physical-or-emulated) state before this call
	const bool wasDown = physicalDown || KeyInput::IsKeyEmulated(rawKey);

	KeyInput::SetKeyEmulated(rawKey, pressed);
	KeyInput::Update(keyBindings.GetFakeMetaKey());

	if (pressed) {
		// fire only on a false->true edge
		if (!wasDown)
			activeController->KeyPressed(eventKey, scanCode, false);
	} else {
		// effective after = physical; fire only on a true->false edge
		if (wasDown && !physicalDown)
			activeController->KeyReleased(eventKey, scanCode);
	}

	return 0;
}


/*** Emulate a keyboard key being pressed and held.
 *
 * Fires the KeyPress event and holds the key down (merged with real hardware
 * state) until released or cleared. The accompanying scancode is derived from
 * the keycode using the currently active system keyboard layout.
 *
 * @function debug.emulateKeyPress
 * @param keycode integer
 * @return nil
 */
int LuaDebugExtra::EmulateKeyPress(lua_State* L) { return emulateKey(L, true); }


/*** Emulate a held keyboard key being released.
 *
 * @function debug.emulateKeyRelease
 * @param keycode integer
 * @return nil
 */
int LuaDebugExtra::EmulateKeyRelease(lua_State* L) { return emulateKey(L, false); }


/*** Emulate a mouse button being pressed and held.
 *
 * @function debug.emulateMousePress
 * @param button integer
 * @return nil
 */
int LuaDebugExtra::EmulateMousePress(lua_State* L)
{
	if (mouse == nullptr)
		return 0;

	const int button = luaL_checkint(L, 1);

	if (button < 1 || button > NUM_BUTTONS)
		return 0;

	mouse->SetButtonEmulated(button, true);
	return 0;
}

int LuaDebugExtra::EmulateMouseWheel(lua_State* L)
{
	if (mouse == nullptr)
		return 0;

	// momentary tick, no persistent state to track; fire directly like a real wheel event
	mouse->MouseWheel((float)luaL_checknumber(L, 1));
	return 0;
}


/*** Emulate a held mouse button being released.
 *
 * @function debug.emulateMouseRelease
 * @param button integer
 * @return nil
 */
int LuaDebugExtra::EmulateMouseRelease(lua_State* L)
{
	if (mouse == nullptr)
		return 0;

	const int button = luaL_checkint(L, 1);

	if (button < 1 || button > NUM_BUTTONS)
		return 0;

	mouse->SetButtonEmulated(button, false);
	return 0;
}


/*** Emulate the cursor moving to a screen position.
 *
 * Fires a MouseMove through the normal pipeline. Coordinates use the bottom-left
 * origin like the rest of the Lua screen API. Does not move the OS cursor.
 *
 * @function debug.emulateMouseMove
 * @param x integer
 * @param y integer
 * @return nil
 */
int LuaDebugExtra::EmulateMouseMove(lua_State* L)
{
	if (mouse == nullptr || mouseInput == nullptr)
		return 0;

	const int x = luaL_checkint(L, 1);
	const int y = globalRendering->viewSizeY - luaL_checkint(L, 2) - 1;

	const int2 prev = mouseInput->GetPos();
	mouseInput->SetPos(int2(x, y));
	mouse->MouseMove(x, y, x - prev.x, y - prev.y);

	return 0;
}


/*** Exercise UnitMoveFailed from the synced Lua handle.
 *
 * @function debug.emulateUnitMoveFailed
 * @param unitID integer
 * @return nil
 */
int LuaDebugExtra::EmulateUnitMoveFailed(lua_State* L)
{
	const int unitID = luaL_checkint(L, 1);
	CUnit* unit = unitHandler.GetUnit(unitID);

	if (unit != nullptr)
		eventHandler.UnitMoveFailed(unit);

	return 0;
}


/*** Exercise the engine-to-Lua/native event surface with a deterministic fixture.
 *
 * This is test infrastructure, not a gameplay API.  It enters the same
 * CEventHandler dispatch methods used by the engine so the Lua and native
 * consumers observe identical C++-constructed payloads.  The object IDs are
 * supplied by the parity fixture and are only used to provide valid object
 * pointers for callbacks whose public contract contains an object ID.
 *
 * @function debug.emulateNativeApiParityCallins
 * @param unitID integer
 * @param featureID integer
 * @param projectileID integer
 * @param benchmarkOnly boolean? If true, exercise only the non-rendering benchmark events.
 * @return nil
 */
int LuaDebugExtra::EmulateNativeApiParityCallins(lua_State* L)
{
	const int unitID = luaL_checkint(L, 1);
	const bool benchmarkOnly = lua_toboolean(L, 4);

	CUnit* unit = unitHandler.GetUnit(unitID);
	if (unit == nullptr)
		return 0;

	if (benchmarkOnly) {
		// Keep the bounded benchmark independent of the simulation clock while
		// still entering the same event-handler dispatches as a real frame.
		eventHandler.GameFrame(7);
		eventHandler.Update();
		BuildInfo buildInfo(unit->unitDef, unit->pos, 0);
		eventHandler.UnitCreated(unit, unit);
		eventHandler.AllowUnitCreation(unit->unitDef, unit, &buildInfo);
		float newDamage = 12.0f;
		float impulseMult = 1.0f;
		eventHandler.UnitPreDamaged(unit, unit, 12.0f, -1, -1, false, &newDamage, &impulseMult);
		Command command(CMD_MOVE, SHIFT_KEY, unit->pos);
		eventHandler.CommandNotify(command);
		eventHandler.AddConsoleLine("wasm_benchmark_callin", "WasmBenchmark", 3);
		return 0;
	}

	const int featureID = luaL_checkint(L, 2);
	const int projectileID = luaL_checkint(L, 3);
	CFeature* feature = featureHandler.GetFeature(featureID);
	CProjectile* projectile = projectileHandler.GetProjectileBySyncedID(projectileID);

	if (feature == nullptr || projectile == nullptr)
		return 0;

	// AddConsoleLine is intentionally not used as a delimiter here: console
	// output is also produced by the parity module itself and the engine may
	// decorate or coalesce those messages.  GameFrame has a stable integer
	// payload, so these test-only sentinels remain unambiguous in both traces.
	constexpr int parityDriverStart = -1001;
	constexpr int parityDriverEnd = -1002;
	eventHandler.GameFrame(parityDriverStart);

	CWeapon* weapon = unit->weapons.empty() ? nullptr : unit->weapons.front();
	const WeaponDef* weaponDef = (weapon != nullptr) ? weapon->weaponDef : nullptr;
	const int unitDefID = (unit->unitDef != nullptr) ? unit->unitDef->id : -1;
	const int featureDefID = (feature->def != nullptr) ? feature->def->id : -1;
	const int weaponDefID = (weaponDef != nullptr) ? weaponDef->id : -1;
	const float3 position = unit->pos;
	const float3 featurePosition = feature->pos;
	const float3 direction = {0.0f, 1.0f, 0.0f};

	// Synced callins.
	eventHandler.GamePreload();
	eventHandler.Load(nullptr);
	eventHandler.GameStart();
	eventHandler.GameOver(std::vector<unsigned char>{0});
	eventHandler.GamePaused(0, true);
	eventHandler.GameFrame(7);
	eventHandler.GameFramePost(7);
	const unsigned char gameID[16] = {
		1, 2, 3, 4, 5, 6, 7, 8,
		9, 10, 11, 12, 13, 14, 15, 16,
	};
	eventHandler.GameID(gameID, sizeof(gameID));

	eventHandler.TeamDied(0);
	eventHandler.TeamChanged(0);
	eventHandler.ResourceExcess(std::map<int, SResourcePack>{{0, SResourcePack(1.5f, 2.5f)}});
	eventHandler.PlayerChanged(0);
	eventHandler.PlayerAdded(0);
	eventHandler.PlayerRemoved(0, 2);

	eventHandler.UnitCreated(unit, unit);
	eventHandler.UnitFinished(unit);
	eventHandler.UnitReverseBuilt(unit);
	eventHandler.UnitConstructionDecayed(unit, 1.0f, 2.0f, 0.5f);
	eventHandler.UnitFromFactory(unit, unit, true);
	eventHandler.UnitDestroyed(unit, unit, weaponDefID);
	eventHandler.UnitTaken(unit, 0, 1);
	eventHandler.UnitGiven(unit, 1, 0);
	eventHandler.RenderUnitDestroyed(unit);

	eventHandler.UnitIdle(unit);
	Command command(CMD_MOVE, SHIFT_KEY, position);
	eventHandler.UnitCommand(unit, command, 0, true, true);
	eventHandler.AllowCommand(unit, command, 0, true, false);
	eventHandler.CommandFallback(unit, command);
	eventHandler.UnitCmdDone(unit, command);
	eventHandler.UnitDamaged(unit, unit, 12.0f, weaponDefID, projectile->id, false);
	eventHandler.UnitStunned(unit, true);
	eventHandler.UnitExperience(unit, 0.25f);
	eventHandler.UnitHarvestStorageFull(unit);

	eventHandler.UnitSeismicPing(unit, 0, position, 3.0f);
	eventHandler.UnitEnteredRadar(unit, 0);
	eventHandler.UnitEnteredLos(unit, 0);
	eventHandler.UnitLeftRadar(unit, 0);
	eventHandler.UnitLeftLos(unit, 0);

	eventHandler.UnitEnteredUnderwater(unit);
	eventHandler.UnitEnteredWater(unit);
	eventHandler.UnitEnteredAir(unit);
	eventHandler.UnitLeftUnderwater(unit);
	eventHandler.UnitLeftWater(unit);
	eventHandler.UnitLeftAir(unit);

	eventHandler.UnitLoaded(unit, unit);
	eventHandler.UnitUnloaded(unit, unit);
	eventHandler.UnitMoveFailed(unit);
	eventHandler.UnitCloaked(unit);
	eventHandler.UnitDecloaked(unit);
	eventHandler.UnitUnitCollision(unit, unit);
	eventHandler.UnitFeatureCollision(unit, feature);
	eventHandler.UnitArrivedAtGoal(unit);

	eventHandler.FeatureCreated(feature);
	eventHandler.FeatureDestroyed(feature);
	eventHandler.FeatureDamaged(feature, unit, 7.0f, weaponDefID, projectile->id);
	eventHandler.ProjectileCreated(projectile, 0);
	eventHandler.ProjectileDestroyed(projectile, 0);

	DamageArray damages(5.0f);
	CExplosionParams explosionParams{
		position,
		direction,
		damages,
		weaponDef,
		unit,
		ExplosionHitObject{},
		0.0f,
		32.0f,
		0.5f,
		1.0f,
		1.0f,
		1.0f,
		false,
		false,
		true,
		static_cast<uint32_t>(projectile->id),
	};
	eventHandler.Explosion(weaponDefID, weaponDef, explosionParams);
	eventHandler.StockpileChanged(unit, weapon, 1);

	BuildInfo buildInfo(unit->unitDef, position, 0);
	eventHandler.AllowUnitCreation(unit->unitDef, unit, &buildInfo);
	eventHandler.AllowUnitTransfer(unit, 1, false);
	eventHandler.AllowUnitBuildStep(unit, unit, 0.5f);
	eventHandler.AllowUnitCaptureStep(unit, unit, 0.5f);
	eventHandler.AllowUnitTransport(unit, unit);
	eventHandler.AllowUnitTransportLoad(unit, unit, position, true);
	eventHandler.AllowUnitTransportUnload(unit, unit, featurePosition, true);
	eventHandler.AllowUnitCloak(unit, unit);
	eventHandler.AllowUnitDecloak(unit, unit, weapon);
	eventHandler.AllowUnitKamikaze(unit, unit, true);
	eventHandler.AllowFeatureCreation(feature->def, 0, featurePosition);
	eventHandler.AllowFeatureBuildStep(unit, feature, 0.5f);
	eventHandler.AllowResourceLevel(0, "metal", 4.0f);
	eventHandler.AllowResourceTransfer(0, 1, "metal", 2.0f);
	eventHandler.AllowDirectUnitControl(0, unit);
	eventHandler.AllowBuilderHoldFire(unit, 1);
	eventHandler.AllowStartPosition(0, 0, 1, position, featurePosition);
	eventHandler.TerraformComplete(unit, unit);
	eventHandler.MoveCtrlNotify(unit, 3);

	if (weaponDefID >= 0) {
		eventHandler.AllowWeaponTargetCheck(unitID, 0, weaponDefID);
		// LuaRules' AllowWeaponTarget dispatcher starts with the neutral
		// priority of 1.0 and passes that value on to later event clients.
		// Seed the deterministic cross-consumer trace with the value native
		// receives after the Lua consumer has run.
		float targetPriority = 1.0f;
		eventHandler.AllowWeaponTarget(unitID, unitID, 0, weaponDefID, &targetPriority);
	}
	eventHandler.AllowWeaponInterceptTarget(unit, weapon, projectile);

	float newDamage = 12.0f;
	float impulseMult = 1.0f;
	eventHandler.UnitPreDamaged(unit, unit, 12.0f, weaponDefID, projectile->id, false, &newDamage, &impulseMult);
	newDamage = 7.0f;
	impulseMult = 1.0f;
	eventHandler.FeaturePreDamaged(feature, unit, 7.0f, weaponDefID, projectile->id, &newDamage, &impulseMult);
	eventHandler.ShieldPreDamaged(projectile, weapon, unit, false, weapon, unit, position, featurePosition);

	// Unsynced callins.
	eventHandler.Save(nullptr);
	eventHandler.UnsyncedHeightMapUpdate(SRectangle(1, 2, 3, 4));
	eventHandler.Update();
	eventHandler.KeyMapChanged();
	eventHandler.KeyPress(SDLK_a, SDL_SCANCODE_A, false);
	eventHandler.KeyRelease(SDLK_a, SDL_SCANCODE_A);
	eventHandler.TextInput("native_api_parity");
	eventHandler.TextEditing("native_api_parity", 1, 3);
	eventHandler.MousePress(100, 200, 1);
	eventHandler.MouseMove(100, 200, 3, -4, 1);
	eventHandler.MouseRelease(100, 200, 1);
	eventHandler.MouseWheel(true, 1.5f);
	eventHandler.IsAbove(100, 200);
	eventHandler.GetTooltip(100, 200);

	int defaultCommand = CMD_MOVE;
	eventHandler.DefaultCommand(unit, nullptr, defaultCommand);
	SCommandDescription commandDescription;
	commandDescription.id = CMD_MOVE;
	commandDescription.type = CMDTYPE_ICON;
	commandDescription.name = "move";
	commandDescription.action = "move";
	commandDescription.tooltip = "Native API parity";
	eventHandler.ActiveCommandChanged(&commandDescription);
	eventHandler.CameraRotationChanged(direction);
	eventHandler.CameraPositionChanged(position);
	eventHandler.MiniMapRotationChanged(1.0f, 0.5f);
	eventHandler.MiniMapStateChanged(false, false, false);
	eventHandler.MiniMapGeometryChanged(int2(1, 2), int2(3, 4), int2(5, 6), int2(7, 8));
	eventHandler.CommandNotify(command);
	eventHandler.AddConsoleLine("native_api_parity_callin", "NativeApiParity", 3);
	eventHandler.GroupChanged(1);

	bool ready = false;
	eventHandler.GameSetup("parity", ready, {{0, "ready"}});
	eventHandler.DownloadQueued(1, "native_api_parity", "sdd");
	eventHandler.DownloadStarted(1);
	eventHandler.DownloadProgress(1, 5, 10);
	eventHandler.DownloadFinished(1);
	eventHandler.DownloadFailed(1, 2);
	const float3 groundPosition = {position.x, position.y, position.z};
	eventHandler.WorldTooltip(nullptr, nullptr, &groundPosition);
	const std::string label = "native_api_parity";
	eventHandler.MapDrawCmd(0, MAPDRAW_POINT, &position, nullptr, &label);
	eventHandler.SunChanged();
	eventHandler.ViewResize();

	eventHandler.DrawGenesis();
	eventHandler.DrawWorld();
	eventHandler.DrawWorldPreUnit();
	eventHandler.DrawPreDecals();
	eventHandler.DrawWorldPreParticles(true, false, true, false);
	eventHandler.DrawWaterPost();
	eventHandler.DrawWorldShadow();
	eventHandler.DrawShadowUnitsLua();
	eventHandler.DrawShadowFeaturesLua();
	eventHandler.DrawShadowPassTransparent();
	eventHandler.DrawWorldReflection();
	eventHandler.DrawWorldRefraction();
	eventHandler.DrawGroundPreForward();
	eventHandler.DrawGroundPostForward();
	eventHandler.DrawGroundPreDeferred();
	eventHandler.DrawGroundDeferred();
	eventHandler.DrawGroundPostDeferred();
	eventHandler.DrawUnitsPostDeferred();
	eventHandler.DrawFeaturesPostDeferred();
	eventHandler.DrawScreenEffects();
	eventHandler.DrawScreenPost();
	eventHandler.DrawScreen();
	eventHandler.DrawInMiniMap();
	eventHandler.DrawInMiniMapBackground();
	eventHandler.DrawBuildSquare(unitDefID, 10, 20, 1, std::vector<uint8_t>{0, 1, 2, 3});
	eventHandler.FontsChanged();
	eventHandler.GameProgress(7);
	eventHandler.DrawUnit(unit);
	eventHandler.DrawFeature(feature);
	if (weapon != nullptr)
		eventHandler.DrawShield(unit, weapon);
	eventHandler.DrawProjectile(projectile);
	eventHandler.DrawMaterial(&LuaMaterial::defMat);
	eventHandler.GameFrame(parityDriverEnd);

	return 0;
}


/*** Exercise a missing GameFrame implementation for the benchmark fixture.
 *
 * The LuaRules handle intentionally has no GameFrame function in this mode, so
 * the direct handle call measures the real Lua global lookup/missing-callin
 * path. The event-handler call immediately afterwards drives the native and
 * Wasm clients through the same engine event boundary without adding a Lua
 * callback to the event list.
 *
 * @function debug.emulateNativeApiParityUnimplementedCallin
 * @param iterations integer
 * @return nil
 */
int LuaDebugExtra::EmulateNativeApiParityUnimplementedCallin(lua_State* L)
{
	const int iterations = std::max(1, luaL_checkint(L, 1));
	if (luaRules == nullptr)
		return 0;

	for (int frame = 1; frame <= iterations; ++frame) {
		luaRules->syncedLuaHandle.GameFrame(frame);
		eventHandler.GameFrame(frame);
	}

	return 0;
}


/*** Release everything currently held via emulation.
 *
 * @function debug.clearEmulatedInput
 * @return nil
 */
int LuaDebugExtra::ClearEmulatedInputLua(lua_State* L)
{
	ClearEmulatedInput();
	return 0;
}


void LuaDebugExtra::ClearEmulatedInput(bool fireReleases)
{
	// snapshot the emulated keys before clearing, so a fired release can't walk
	// the store we are emptying
	const std::set<int> keyCodes = KeyInput::GetEmulatedKeys();

	KeyInput::ClearEmulatedKeys();
	KeyInput::Update(keyBindings.GetFakeMetaKey());

	if (fireReleases && activeController != nullptr) {
		int numKeys = 0;
		const uint8_t* kbState = SDL_GetKeyboardState(&numKeys);

		// the store holds raw SDL2 keycodes; the event side wants the normalized code
		for (const int rawKey: keyCodes) {
			const SDL_Scancode sc = SDL_GetScancodeFromKey((SDL_Keycode)rawKey);

			if ((int)sc < numKeys && kbState[sc] != 0)
				continue;

			activeController->KeyReleased(CKeyCodes::GetNormalizedSymbol(rawKey), CScanCodes::GetNormalizedSymbol(sc));
		}
	}

	if (mouse == nullptr)
		return;

	// no-fire path (game teardown): drop the flags without dispatching into
	// handles that are already being destroyed
	if (!fireReleases) {
		mouse->ClearEmulatedButtons();
		return;
	}

	// SetButtonEmulated fires the release itself if the button ends up effectively up
	for (int button = 1; button <= NUM_BUTTONS; ++button) {
		if (mouse->IsButtonEmulated(button))
			mouse->SetButtonEmulated(button, false);
	}
}