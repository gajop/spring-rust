/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include <catch_amalgamated.hpp>

#include "NativeInterface/api/RmlUi.h"

#include <RmlUi/Core.h>
#include <RmlUi/Core/Context.h>
#include <RmlUi/Core/ElementDocument.h>
#include <RmlUi/Core/RenderInterface.h>
#include <RmlUi/Core/SystemInterface.h>

#include <cstdint>

namespace {

class NullRenderInterface final : public Rml::RenderInterface
{
public:
	Rml::CompiledGeometryHandle CompileGeometry(Rml::Span<const Rml::Vertex>, Rml::Span<const int>) override { return {}; }
	void RenderGeometry(Rml::CompiledGeometryHandle, Rml::Vector2f, Rml::TextureHandle) override {}
	void ReleaseGeometry(Rml::CompiledGeometryHandle) override {}
	Rml::TextureHandle LoadTexture(Rml::Vector2i& dimensions, const Rml::String&) override
	{
		dimensions = {};
		return {};
	}
	Rml::TextureHandle GenerateTexture(Rml::Span<const Rml::byte>, Rml::Vector2i) override { return {}; }
	void ReleaseTexture(Rml::TextureHandle) override {}
	void EnableScissorRegion(bool) override {}
	void SetScissorRegion(Rml::Rectanglei) override {}
};

class SilentSystemInterface final : public Rml::SystemInterface
{
public:
	bool LogMessage(Rml::Log::Type, const Rml::String&) override { return true; }
};

SilentSystemInterface silentSystemInterface;

std::uint64_t ToHandle(const void* pointer)
{
	return reinterpret_cast<std::uint64_t>(pointer);
}

struct CallbackCounts {
	int eventDestroy = 0;
	int dataEventDestroy = 0;
};

void NativeEventCallback(void*) {}

void DestroyNativeEventCallback(void* userData)
{
	++static_cast<CallbackCounts*>(userData)->eventDestroy;
}

void NativeDataEventCallback(void*, const RmlDataEventArgs*) {}

void DestroyNativeDataEventCallback(void* userData)
{
	++static_cast<CallbackCounts*>(userData)->dataEventDestroy;
}

void RemoveContextImmediately(std::uint64_t handle)
{
	auto* context = reinterpret_cast<Rml::Context*>(static_cast<uintptr_t>(handle));
	if (context != nullptr)
		Rml::RemoveContext(context->GetName());
}

} // namespace

namespace RmlGui {

bool IsInitialized() { return true; }
Rml::Context* GetContext(const std::string& name) { return Rml::GetContext(name); }
Rml::Context* GetOrCreateContext(const std::string& name)
{
	if (auto* context = Rml::GetContext(name); context != nullptr)
		return context;
	return Rml::CreateContext(name, {1024, 768});
}
void MarkContextForRemoval(Rml::Context*) {}
void SetDebugContext(Rml::Context*) {}
void ClearDebugContext(Rml::Context*) {}
void SetMouseCursorAlias(std::string, std::string) {}
void AddTranslationString(const std::string&, const std::string&) {}
void ClearTranslations() {}

} // namespace RmlGui

TEST_CASE("Native RmlUi callbacks are destroyed when a module context is removed")
{
	NullRenderInterface renderInterface;
	Rml::SetSystemInterface(&silentSystemInterface);
	Rml::SetRenderInterface(&renderInterface);
	REQUIRE(Rml::Initialise());

	RmlCreateContextQuery createQuery{.name = "native-callback-lifetime"};
	RmlCreateContextResult createResult{};
	RMLUI_API.CreateContext(&createQuery, &createResult);
	REQUIRE(createResult.error == nullptr);
	REQUIRE(createResult.success);
	REQUIRE(createResult.contextHandle != 0);

	auto* context = reinterpret_cast<Rml::Context*>(static_cast<uintptr_t>(createResult.contextHandle));
	auto* document = context->CreateDocument();
	REQUIRE(document != nullptr);
	document->SetInnerRML("<button id=callback-target>callback</button>");
	document->Show();
	context->Update();

	RmlContextHandleQuery rootQuery{.contextHandle = createResult.contextHandle};
	RmlContextGetElementResult rootResult{};
	RMLUI_API.ContextGetRootElement(&rootQuery, &rootResult);
	REQUIRE(rootResult.error == nullptr);
	REQUIRE(rootResult.exists);

	RmlElementGetByStringQuery targetQuery{
		.elementHandle = rootResult.elementHandle,
		.value = "callback-target",
	};
	RmlElementGetElementResult targetResult{};
	RMLUI_API.ElementGetElementById(&targetQuery, &targetResult);
	REQUIRE(targetResult.error == nullptr);
	REQUIRE(targetResult.exists);

	CallbackCounts counts;
	RmlContextEventListenerCallbackQuery contextListenerQuery{
		.contextHandle = createResult.contextHandle,
		.event = "click",
		.inCapturePhase = false,
		.callback = NativeEventCallback,
		.userData = &counts,
		.destroyCallback = DestroyNativeEventCallback,
	};
	RmlEventListenerCallbackResult contextListenerResult{};
	RMLUI_API.ContextAddEventListener(&contextListenerQuery, &contextListenerResult);
	REQUIRE(contextListenerResult.error == nullptr);
	REQUIRE(contextListenerResult.success);

	RmlEventListenerCallbackQuery elementListenerQuery{
		.elementHandle = targetResult.elementHandle,
		.event = "click",
		.inCapturePhase = false,
		.callback = NativeEventCallback,
		.userData = &counts,
		.destroyCallback = DestroyNativeEventCallback,
	};
	RmlEventListenerCallbackResult elementListenerResult{};
	RMLUI_API.ElementAddEventListener(&elementListenerQuery, &elementListenerResult);
	REQUIRE(elementListenerResult.error == nullptr);
	REQUIRE(elementListenerResult.success);

	RmlContextCreateDataModelQuery modelQuery{
		.contextHandle = createResult.contextHandle,
		.name = "callbacks",
	};
	RmlContextOpenDataModelResult modelResult{};
	RMLUI_API.ContextCreateDataModel(&modelQuery, &modelResult);
	REQUIRE(modelResult.error == nullptr);
	REQUIRE(modelResult.success);

	const std::uint8_t fieldTypes[] = {RML_FIELD_INT};
	RmlDataModelBindEventQuery dataEventQuery{
		.dataModelHandle = modelResult.dataModelHandle,
		.name = "on_callback",
		.callback = NativeDataEventCallback,
		.userData = &counts,
		.destroyCallback = DestroyNativeDataEventCallback,
		.fieldTypes = fieldTypes,
		.fieldCount = 1,
	};
	RmlDataModelBindEventResult dataEventResult{};
	RMLUI_API.DataModelBindEvent(&dataEventQuery, &dataEventResult);
	REQUIRE(dataEventResult.error == nullptr);
	REQUIRE(dataEventResult.success);
	CHECK(counts.eventDestroy == 0);
	CHECK(counts.dataEventDestroy == 0);

	NativeRmlUi::ClearAllContexts(RemoveContextImmediately);

	CHECK(Rml::GetContext("native-callback-lifetime") == nullptr);
	CHECK(counts.eventDestroy == 2);
	CHECK(counts.dataEventDestroy == 1);

	RmlContextEventListenerRemoveQuery staleContextListener{
		.contextHandle = createResult.contextHandle,
		.eventListenerHandle = contextListenerResult.eventListenerHandle,
		.event = "click",
		.inCapturePhase = false,
	};
	RmlElementBoolResult staleContextResult{};
	RMLUI_API.ContextRemoveEventListener(&staleContextListener, &staleContextResult);
	CHECK_FALSE(staleContextResult.success);

	RmlElementEventListenerRemoveQuery staleElementListener{
		.elementHandle = targetResult.elementHandle,
		.eventListenerHandle = elementListenerResult.eventListenerHandle,
		.event = "click",
		.inCapturePhase = false,
	};
	RmlElementBoolResult staleElementResult{};
	RMLUI_API.ElementRemoveEventListener(&staleElementListener, &staleElementResult);
	CHECK_FALSE(staleElementResult.success);

	RmlDataModelEventHandleQuery staleDataEvent{.eventHandle = dataEventResult.eventHandle};
	RmlElementBoolResult staleDataEventResult{};
	RMLUI_API.DataModelUnbindEvent(&staleDataEvent, &staleDataEventResult);
	CHECK_FALSE(staleDataEventResult.success);

	Rml::Shutdown();
}
