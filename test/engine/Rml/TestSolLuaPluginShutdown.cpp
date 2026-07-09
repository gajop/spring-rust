/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include <catch_amalgamated.hpp>

#include "Rml/SolLua/plugin/SolLuaPlugin.h"
#include "Rml/SolLua/plugin/SolLuaEventListener.h"

#include <RmlUi/Core.h>
#include <RmlUi/Core/Context.h>
#include <RmlUi/Core/ElementDocument.h>
#include <RmlUi/Core/RenderInterface.h>

#include <sol2/sol.hpp>

namespace
{
class NullRenderInterface : public Rml::RenderInterface
{
public:
	Rml::CompiledGeometryHandle CompileGeometry(Rml::Span<const Rml::Vertex>, Rml::Span<const int>) override { return {}; }
	void RenderGeometry(Rml::CompiledGeometryHandle, Rml::Vector2f, Rml::TextureHandle) override {}
	void ReleaseGeometry(Rml::CompiledGeometryHandle) override {}

	Rml::TextureHandle LoadTexture(Rml::Vector2i& textureDimensions, const Rml::String&) override
	{
		textureDimensions = {};
		return {};
	}

	Rml::TextureHandle GenerateTexture(Rml::Span<const Rml::byte>, Rml::Vector2i) override { return {}; }
	void ReleaseTexture(Rml::TextureHandle) override {}
	void EnableScissorRegion(bool) override {}
	void SetScissorRegion(Rml::Rectanglei) override {}
};
}

namespace RmlGui
{
void MarkContextForRemoval(Rml::Context*) {}
}

TEST_CASE("SolLuaPlugin removes all tracked documents before plugin shutdown")
{
	NullRenderInterface renderInterface;
	Rml::SetRenderInterface(&renderInterface);
	REQUIRE(Rml::Initialise());

	sol::state lua;
	auto* luaPlugin = new Rml::SolLua::SolLuaPlugin(lua, "rmlDocumentId");
	Rml::RegisterPlugin(luaPlugin);

	Rml::Context* context = Rml::CreateContext("sol-lua-shutdown", {1024, 768});
	REQUIRE(context != nullptr);

	Rml::ElementDocument* firstDocument = context->CreateDocument();
	Rml::ElementDocument* secondDocument = context->CreateDocument();
	REQUIRE(firstDocument != nullptr);
	REQUIRE(secondDocument != nullptr);

	luaPlugin->AddDocumentTracking(firstDocument);
	luaPlugin->AddDocumentTracking(secondDocument);

	luaPlugin->RemoveLuaItems();
	context->Update();

	REQUIRE(context->GetNumDocuments() == 0);

	REQUIRE(Rml::RemoveContext(context->GetName()));
	Rml::UnregisterPlugin(luaPlugin);
	Rml::Shutdown();
}

// SBC's Lua keeps element references across DOM rebuilds and then calls
// Element:SetClass / element.inner_rml / Element:SetAttribute on them. Once RmlUi
// has destroyed the element that is a use-after-free (ASAN: use-after-poison in
// ElementStyle::SetClass). Those bindings now consult IsSolLuaElementAlive first.
//
// This is the failing example: before element lifetime was tracked there was no
// way to tell a freed Element* from a live one, so the check below could not be
// expressed at all and the bindings dereferenced blindly.
TEST_CASE("SolLuaPlugin tracks element lifetime so stale Lua handles are detectable")
{
	NullRenderInterface renderInterface;
	Rml::SetRenderInterface(&renderInterface);
	REQUIRE(Rml::Initialise());

	sol::state lua;
	auto* luaPlugin = new Rml::SolLua::SolLuaPlugin(lua, "rmlDocumentId");
	Rml::RegisterPlugin(luaPlugin);

	Rml::Context* context = Rml::CreateContext("sol-lua-element-lifetime", {1024, 768});
	REQUIRE(context != nullptr);

	Rml::ElementDocument* document = context->CreateDocument();
	REQUIRE(document != nullptr);

	// A null pointer is never alive, and the lookup must not dereference it.
	REQUIRE_FALSE(Rml::SolLua::IsSolLuaElementAlive(nullptr));

	Rml::ElementPtr owned = document->CreateElement("div");
	REQUIRE(owned != nullptr);
	Rml::Element* element = owned.get();

	// Created through the factory, so the plugin's OnElementCreate hook saw it.
	REQUIRE(Rml::SolLua::IsSolLuaElementAlive(element));

	REQUIRE(document->AppendChild(std::move(owned)) == element);
	REQUIRE(Rml::SolLua::IsSolLuaElementAlive(element));

	// Detaching alone must not invalidate it: the caller still owns the element.
	Rml::ElementPtr detached = document->RemoveChild(element);
	REQUIRE(detached != nullptr);
	REQUIRE(Rml::SolLua::IsSolLuaElementAlive(element));

	// Destroying it must. `element` is now exactly the dangling pointer Lua may
	// still be holding; the lookup compares pointer values and never dereferences,
	// so it is safe to ask.
	detached.reset();
	REQUIRE_FALSE(Rml::SolLua::IsSolLuaElementAlive(element));

	REQUIRE(Rml::RemoveContext(context->GetName()));
	Rml::UnregisterPlugin(luaPlugin);
	Rml::Shutdown();
}

// A Lua event handler is allowed to destroy the element it is attached to --
// rebuilding a parent's inner_rml does exactly that -- which detaches and
// deletes the listener while it is still on the stack. sol2 reads the
// function's lua_state() again *after* the call returns (invoke() computes
// poststacksize = lua_gettop(lua_state())), so SolLuaEventListener::ProcessEvent
// must not touch any member once it has called into Lua.
//
// This is the failing example: before ProcessEvent copied the function onto the
// stack, this test aborted under ASAN with
//   use-after-poison ... in sol::basic_reference<false>::lua_state()
//   #3 Rml::SolLua::SolLuaEventListener::ProcessEvent
TEST_CASE("SolLuaEventListener survives a handler that destroys its own element")
{
	NullRenderInterface renderInterface;
	Rml::SetRenderInterface(&renderInterface);
	REQUIRE(Rml::Initialise());

	sol::state lua;
	lua.open_libraries(sol::lib::base);
	auto* luaPlugin = new Rml::SolLua::SolLuaPlugin(lua, "rmlDocumentId");
	Rml::RegisterPlugin(luaPlugin);

	Rml::Context* context = Rml::CreateContext("sol-lua-listener-suicide", {1024, 768});
	REQUIRE(context != nullptr);

	// The plugin registers the "body" instancer, so this is a SolLuaDocument.
	Rml::ElementDocument* document = context->CreateDocument();
	REQUIRE(document != nullptr);

	Rml::Element* element = document->AppendChild(document->CreateElement("div"));
	REQUIRE(element != nullptr);

	// The handler destroys the element the listener is attached to.
	bool handlerRan = false;
	lua.set_function("destroyElement", [&]() {
		handlerRan = true;
		Rml::ElementPtr owned = document->RemoveChild(element);
		owned.reset();
	});

	auto chunk = lua.load("return function(event, element, document) destroyElement() end");
	REQUIRE(chunk.valid());
	sol::protected_function factory = chunk.get<sol::protected_function>();
	sol::protected_function handler = factory().get<sol::protected_function>();
	REQUIRE(handler.valid());

	// Owned by the element: OnDetach deletes it, which is what happens mid-call.
	auto* listener = new Rml::SolLua::SolLuaEventListener(handler, element);
	element->AddEventListener(Rml::EventId::Click, listener, false);
	REQUIRE(Rml::SolLua::IsSolLuaElementAlive(element));

	element->DispatchEvent(Rml::EventId::Click, Rml::Dictionary());

	REQUIRE(handlerRan);
	REQUIRE_FALSE(Rml::SolLua::IsSolLuaElementAlive(element));
	REQUIRE(document->GetNumChildren() == 0);

	REQUIRE(Rml::RemoveContext(context->GetName()));
	Rml::UnregisterPlugin(luaPlugin);
	Rml::Shutdown();
}
