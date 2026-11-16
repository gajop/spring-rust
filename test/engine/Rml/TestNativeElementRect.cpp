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

namespace
{
float GetFloat(void (*call)(const RmlElementHandleQuery*, RmlElementGetFloatResult*), uint64_t element)
{
	RmlElementHandleQuery query = {.elementHandle = element};
	RmlElementGetFloatResult result = {};
	call(&query, &result);
	REQUIRE(result.error == nullptr);
	return result.value;
}

int32_t GetScrollTop(uint64_t element)
{
	RmlElementHandleQuery query = {.elementHandle = element};
	RmlElementGetIntResult result = {};
	RMLUI_API.ElementGetScrollTop(&query, &result);
	REQUIRE(result.error == nullptr);
	return result.value;
}
}

// A scrolling list that follows its newest entry must know whether it is at the
// end: scroll_top + client_height against scroll_height, as Lua reads them from
// element.scroll_top, client_height and scroll_height. The native API had the
// scroll offsets but not the sizes, so ElementGetScrollWidth/Height and
// ElementGetClientWidth/Height were added.
TEST_CASE("ElementGetScroll and ElementGetClient sizes report an element's scrolling as RmlUi does")
{
	NullRenderInterface renderInterface;
	Rml::SetRenderInterface(&renderInterface);
	REQUIRE(Rml::Initialise());

	Rml::Context* context = Rml::CreateContext("native-element-scroll", {1024, 768});
	REQUIRE(context != nullptr);

	Rml::ElementDocument* document = context->CreateDocument();
	REQUIRE(document != nullptr);
	// A 100x80 box scrolling vertically over 60x500 of content, with 10px of
	// padding (scroll and client sizes include it) and a 2px border (neither does).
	document->SetInnerRML(R"(<div id="host"><div id="content"/></div>)");
	Rml::Element* host = document->GetElementById("host");
	Rml::Element* content = document->GetElementById("content");
	REQUIRE(host != nullptr);
	REQUIRE(content != nullptr);
	host->SetProperty("display", "block");
	host->SetProperty("position", "absolute");
	host->SetProperty("width", "100px");
	host->SetProperty("height", "80px");
	host->SetProperty("padding", "10px");
	host->SetProperty("border-width", "2px");
	host->SetProperty("overflow-x", "hidden");
	host->SetProperty("overflow-y", "scroll");
	content->SetProperty("display", "block");
	content->SetProperty("width", "60px");
	content->SetProperty("height", "500px");
	document->Show();
	context->Update();

	RmlContextHandleQuery contextQuery = {.contextHandle = ToHandle(context)};
	RmlContextGetElementResult rootResult = {};
	RMLUI_API.ContextGetRootElement(&contextQuery, &rootResult);
	REQUIRE(rootResult.error == nullptr);
	REQUIRE(rootResult.exists);
	RmlElementGetByStringQuery elementQuery = {.elementHandle = rootResult.elementHandle, .value = "host"};
	RmlElementGetElementResult elementResult = {};
	RMLUI_API.ElementGetElementById(&elementQuery, &elementResult);
	REQUIRE(elementResult.error == nullptr);
	REQUIRE(elementResult.exists);
	const uint64_t handle = elementResult.elementHandle;

	const float scrollWidth = GetFloat(RMLUI_API.ElementGetScrollWidth, handle);
	const float scrollHeight = GetFloat(RMLUI_API.ElementGetScrollHeight, handle);
	const float clientWidth = GetFloat(RMLUI_API.ElementGetClientWidth, handle);
	const float clientHeight = GetFloat(RMLUI_API.ElementGetClientHeight, handle);
	CHECK(scrollWidth == Catch::Approx(host->GetScrollWidth()));
	CHECK(scrollHeight == Catch::Approx(host->GetScrollHeight()));
	CHECK(clientWidth == Catch::Approx(host->GetClientWidth()));
	CHECK(clientHeight == Catch::Approx(host->GetClientHeight()));
	// RmlUi's scrollable overflow runs from the padding edge to the content's end:
	// the top padding and the content (10 + 500), not the bottom padding. The client
	// height is the padding box (10 + 80 + 10), no border.
	CHECK(scrollHeight == Catch::Approx(510.0f));
	CHECK(clientHeight == Catch::Approx(100.0f));
	CHECK(clientWidth <= 120.0f);

	SECTION("scrolled to the end, scroll_top + client_height reaches scroll_height")
	{
		CHECK(GetScrollTop(handle) == 0);
		RmlElementSetIntQuery scroll = {.elementHandle = handle, .value = static_cast<int32_t>(scrollHeight)};
		RmlElementBoolResult scrolled = {};
		RMLUI_API.ElementSetScrollTop(&scroll, &scrolled);
		REQUIRE(scrolled.error == nullptr);
		context->Update();
		const int32_t top = GetScrollTop(handle);
		CHECK(top == static_cast<int32_t>(scrollHeight - clientHeight));
		CHECK(static_cast<float>(top) + GetFloat(RMLUI_API.ElementGetClientHeight, handle) >= GetFloat(RMLUI_API.ElementGetScrollHeight, handle) - 1.0f);
	}

	SECTION("a null element handle is an error, not a crash")
	{
		for (auto call : {RMLUI_API.ElementGetScrollWidth, RMLUI_API.ElementGetScrollHeight,
				RMLUI_API.ElementGetClientWidth, RMLUI_API.ElementGetClientHeight}) {
			RmlElementHandleQuery bad = {.elementHandle = 0};
			RmlElementGetFloatResult badResult = {.error = nullptr, .value = 1.0f};
			call(&bad, &badResult);
			CHECK(badResult.error != nullptr);
			CHECK(badResult.value == 0.0f);
		}
	}

	REQUIRE(Rml::RemoveContext(context->GetName()));
	Rml::Shutdown();
}
