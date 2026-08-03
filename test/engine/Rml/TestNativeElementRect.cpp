/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include <catch_amalgamated.hpp>

#include "NativeInterface/api/RmlUi.h"

#include <RmlUi/Core.h>
#include <RmlUi/Core/Context.h>
#include <RmlUi/Core/Element.h>
#include <RmlUi/Core/ElementDocument.h>
#include <RmlUi/Core/RenderInterface.h>

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

uint64_t ToHandle(const void* pointer)
{
	return reinterpret_cast<uint64_t>(pointer);
}
}

// The native API translation unit links against the RmlUi backend. None of it is
// reachable from ElementGetRect, so stub it out rather than pull in the engine.
namespace RmlGui
{
bool IsInitialized() { return true; }
Rml::Context* GetContext(const std::string&) { return nullptr; }
Rml::Context* GetOrCreateContext(const std::string&) { return nullptr; }
void MarkContextForRemoval(Rml::Context*) {}
void SetDebugContext(Rml::Context*) {}
void ClearDebugContext(Rml::Context*) {}
void SetMouseCursorAlias(std::string, std::string) {}
void AddTranslationString(const std::string&, const std::string&) {}
void ClearTranslations() {}
}

// A native plugin cannot map a cursor position onto an element without knowing
// where that element is: a colour picker's saturation/value square, a slider,
// any drag against a rect. Lua has `element.absolute_left` / `offset_width`;
// the native API had no equivalent, so ElementGetRect was added.
//
// This is the failing example: without the binding there is no way to ask, and
// this test does not compile.
TEST_CASE("ElementGetRect reports an element's absolute offset and border box")
{
	NullRenderInterface renderInterface;
	Rml::SetRenderInterface(&renderInterface);
	REQUIRE(Rml::Initialise());

	Rml::Context* context = Rml::CreateContext("native-element-rect", {1024, 768});
	REQUIRE(context != nullptr);

	Rml::ElementDocument* document = context->CreateDocument();
	REQUIRE(document != nullptr);
	document->SetInnerRML(R"(<div id="box"/>)");

	Rml::Element* box = document->GetElementById("box");
	REQUIRE(box != nullptr);

	// Absolute position and size, chosen so every field is distinguishable.
	box->SetProperty("position", "absolute");
	box->SetProperty("left", "40px");
	box->SetProperty("top", "70px");
	box->SetProperty("width", "180px");
	box->SetProperty("height", "120px");
	document->Show();
	context->Update();

	// Resolve the element through the native API so the test uses the same
	// validated handle path as a native plugin. Passing the raw RmlUi pointer
	// would correctly be rejected as an unregistered handle.
	RmlContextHandleQuery contextQuery = {.contextHandle = ToHandle(context)};
	RmlContextGetElementResult rootResult = {};
	RMLUI_API.ContextGetRootElement(&contextQuery, &rootResult);
	REQUIRE(rootResult.error == nullptr);
	REQUIRE(rootResult.exists);

	RmlElementGetByStringQuery elementQuery = {
		.elementHandle = rootResult.elementHandle,
		.value = "box",
	};
	RmlElementGetElementResult elementResult = {};
	RMLUI_API.ElementGetElementById(&elementQuery, &elementResult);
	REQUIRE(elementResult.error == nullptr);
	REQUIRE(elementResult.exists);

	RmlElementHandleQuery query = {.elementHandle = elementResult.elementHandle};
	RmlElementGetRectResult result = {};
	RMLUI_API.ElementGetRect(&query, &result);

	REQUIRE(result.error == nullptr);
	CHECK(result.left == Catch::Approx(40.0f));
	CHECK(result.top == Catch::Approx(70.0f));
	CHECK(result.width == Catch::Approx(180.0f));
	CHECK(result.height == Catch::Approx(120.0f));

	SECTION("a null element handle is an error, not a crash")
	{
		RmlElementHandleQuery bad = {.elementHandle = 0};
		RmlElementGetRectResult badResult = {};
		RMLUI_API.ElementGetRect(&bad, &badResult);
		CHECK(badResult.error != nullptr);
		CHECK(badResult.width == 0.0f);
	}

	REQUIRE(Rml::RemoveContext(context->GetName()));
	Rml::Shutdown();
}
