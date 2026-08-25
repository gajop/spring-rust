/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "LuaMenuController.h"

#include "Game/GlobalUnsynced.h"
#include "Game/UI/InfoConsole.h"
#include "Game/UI/MouseHandler.h"
#include "Lua/LuaInputReceiver.h"
#include "Lua/LuaMenu.h"
#include "NativeInterface/api/Callins.h"
#include "System/Config/ConfigHandler.h"
#include "System/EventHandler.h"
#include "System/FileSystem/VFSHandler.h"
#include "System/SafeUtil.h"
#include "System/Log/ILog.h"
#include "WasmInterface/core/host/WasmCoreCallinId.h"
#include "WasmInterface/runtime/WasmEnvironment.h"
#include "WasmInterface/standalone/WasmStandaloneEnvironment.h"
#include "WasmInterface/system/WasmInterfaceSystem.h"

#include "System/Misc/TracyDefs.h"

CONFIG(std::string, DefaultLuaMenu).defaultValue("").description("Sets the default menu to be used when spring is started.");

CLuaMenuController* luaMenuController = nullptr;


CLuaMenuController::CLuaMenuController(const std::string& menuName)
	: menuArchive(menuName)
	, lastDrawFrameTime(spring_gettime())
{
	if (!Valid())
		menuArchive = configHandler->GetString("DefaultLuaMenu");

	// create LuaMenu if necessary
	if (!Valid())
		return;

	Reset();
	CLuaMenu::LoadFreeHandler();
	InitWasmMenu();
}

CLuaMenuController::~CLuaMenuController()
{
	CLuaMenu::FreeHandler();
	m_wasmEnv.reset();
}


bool CLuaMenuController::Reset()
{
	if (!Valid()) {
		// if no LuaMenu, cursor will not be updated (again) until game exists so force a reset
		// calling ReloadCursors here is not possible since no archives are loaded at this point
		mouse->ResetCursor();
		return false;
	}

	LOG("[LuaMenuController::%s] using menu archive \"%s\"", __func__, menuArchive.c_str());

	// lock should not be needed here, but does no harm either
	vfsHandler->GrabLock();
	vfsHandler->SetName("LuaMenuVFS");
	vfsHandler->AddArchiveWithDeps(menuArchive, false);
	vfsHandler->SetName("SpringVFS");
	vfsHandler->FreeLock();

	mouse->ReloadCursors();
	return true;
}

bool CLuaMenuController::Activate(const std::string& msg)
{
	LOG("[LuaMenuController::%s(msg=\"%s\")] luaMenu=%p", __func__, msg.c_str(), luaMenu);

	// LuaMenu might have failed to load, making the controller deadweight
	if (luaMenu == nullptr && !HasWasmMenu())
		return false;

	assert(Valid());
	activeController = luaMenuController;

	mouse->ShowMouse();
	if (luaMenu != nullptr)
		luaMenu->ActivateMenu(msg);
	DispatchWasmActivateMenu(msg);
	return true;
}

bool CLuaMenuController::ActivateInstance(const std::string& msg)
{
	return (luaMenuController->Valid() && luaMenuController->Activate(msg));
}

void CLuaMenuController::ResizeEvent()
{
	eventHandler.ViewResize();
}

bool CLuaMenuController::Update()
{
	ZoneScoped;

	// we should not become the active controller unless this holds (see ::Activate)
	assert(luaMenu != nullptr || HasWasmMenu());

	eventHandler.CollectGarbage(false);
	infoConsole->PushNewLinesToEventHandler();
	mouse->Update();
	mouse->UpdateCursors();
	eventHandler.Update();
	if (m_wasmEnv)
		m_wasmEnv->Update();
	// calls IsAbove
	mouse->GetCurrentTooltip();

	return true;
}

bool CLuaMenuController::Draw()
{
	// we should not become the active controller unless this holds (see ::Activate)
	assert(luaMenu != nullptr || HasWasmMenu());

	// render if global rendering active + menu allows it, and at least once per 30s
	bool allowDraw = globalRendering->active;
	if (allowDraw) {
		if (luaMenu != nullptr)
			allowDraw = luaMenu->AllowDraw();
		allowDraw = allowDraw && WasmAllowDraw();
	}
	const bool forceDraw = ((spring_gettime() - lastDrawFrameTime).toSecsi() > 30);

	if (allowDraw || forceDraw) {
		globalRendering->drawFrame = std::max(1U, globalRendering->drawFrame + 1);
		ClearScreen();

		eventHandler.DrawGenesis();
		eventHandler.DrawScreen();
		mouse->DrawCursor();
		eventHandler.DrawScreenPost();

		lastDrawFrameTime = spring_gettime();
		return true;
	}

	spring_msecs(10).sleep(true); // no draw needed, sleep a bit
	return false;
}


int CLuaMenuController::KeyReleased(int keyCode, int scanCode)
{
	luaInputReceiver->KeyReleased(keyCode, scanCode);
	return 0;
}

int CLuaMenuController::KeyPressed(int keyCode, int scanCode, bool isRepeat)
{
	luaInputReceiver->KeyPressed(keyCode, scanCode, isRepeat);
	return 0;
}

int CLuaMenuController::TextInput(const std::string& utf8Text)
{
	eventHandler.TextInput(utf8Text);
	return 0;
}

int CLuaMenuController::TextEditing(const std::string& utf8Text, unsigned int start, unsigned int length)
{
	eventHandler.TextEditing(utf8Text, start, length);
	return 0;
}


// --- Wasm menu support ---

void CLuaMenuController::InitWasmMenu()
{
	m_wasmEnv = WasmStandaloneEnvironment::Create();
	m_wasmEnv->LoadManifest("WasmMenu/manifest.json");
	m_wasmEnv->TryLoadNativeDLL("WasmMenu/NativeMenu");
	m_wasmEnv->EnsureEventClient();
}

void CLuaMenuController::ReloadWasmMenu()
{
	m_wasmEnv.reset();
	InitWasmMenu();
}

bool CLuaMenuController::HasWasmMenu() const
{
	return m_wasmEnv && m_wasmEnv->HasModules(WasmEnvironment::Menu);
}

void CLuaMenuController::DispatchWasmActivateMenu(const std::string& msg)
{
	if (!HasWasmMenu())
		return;
	ActivateMenuQuery query = {
		.message = msg.c_str(),
		.messageLength = static_cast<uint32_t>(msg.size()),
	};
	bool handled = false;
	std::string error;
	if (!WasmInterfaceSystem::DispatchActiveCoreCallin(
			CoreCallinOf("ActivateMenu"), &query, false, nullptr, handled, error)) {
		if (!error.empty())
			LOG_L(L_ERROR, "WasmMenu ActivateMenu failed: %s", error.c_str());
	}
}

void CLuaMenuController::DispatchWasmActivateGame()
{
	if (!HasWasmMenu())
		return;
	SimpleCallinQuery query = {};
	bool handled = false;
	std::string error;
	if (!WasmInterfaceSystem::DispatchActiveCoreCallin(
			CoreCallinOf("ActivateGame"), &query, false, nullptr, handled, error)) {
		if (!error.empty())
			LOG_L(L_ERROR, "WasmMenu ActivateGame failed: %s", error.c_str());
	}
}

bool CLuaMenuController::WasmAllowDraw()
{
	if (!HasWasmMenu())
		return true;
	SimpleCallinQuery query = {};
	BoolCallinResult result = {.error = nullptr, .value = true};
	bool handled = false;
	std::string error;
	if (!WasmInterfaceSystem::DispatchActiveCoreCallin(
			CoreCallinOf("AllowDraw"), &query, false, &result, handled, error)) {
		if (!error.empty())
			LOG_L(L_ERROR, "WasmMenu AllowDraw failed: %s", error.c_str());
		return true;
	}
	return result.value;
}
