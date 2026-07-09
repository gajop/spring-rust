/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include <catch_amalgamated.hpp>

#include "Rml/SolLua/plugin/SolLuaPlugin.h"

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
