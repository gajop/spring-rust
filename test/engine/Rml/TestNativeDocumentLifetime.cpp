/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include <catch_amalgamated.hpp>

#include "NativeInterface/api/RmlUi.h"

#include <RmlUi/Core.h>
#include <RmlUi/Core/Context.h>
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

TEST_CASE("Native document handles reject documents destroyed with their context")
{
	NullRenderInterface renderInterface;
	Rml::SetRenderInterface(&renderInterface);
	REQUIRE(Rml::Initialise());

	Rml::Context* context = Rml::CreateContext("native-document-lifetime", {1024, 768});
	REQUIRE(context != nullptr);

	RmlContextCreateDocumentQuery createQuery = {
		.contextHandle = ToHandle(context),
		.tag = nullptr,
	};
	RmlContextCreateDocumentResult createResult = {};
	RMLUI_API.ContextCreateDocument(&createQuery, &createResult);
	REQUIRE(createResult.error == nullptr);
	REQUIRE(createResult.success);

	// Native module teardown removes the context before dropping its document
	// views. The document handle must become invalid instead of being cast back
	// to the freed ElementDocument.
	REQUIRE(Rml::RemoveContext(context->GetName()));

	RmlRemoveContextQuery staleRemoveQuery = {.contextHandle = ToHandle(context)};
	RmlRemoveContextResult staleRemoveResult = {};
	RMLUI_API.RemoveContext(&staleRemoveQuery, &staleRemoveResult);
	CHECK_FALSE(staleRemoveResult.success);

	RmlDocumentHandleQuery closeQuery = {.documentHandle = createResult.documentHandle};
	RmlDocumentBoolResult closeResult = {};
	RMLUI_API.DocumentClose(&closeQuery, &closeResult);
	CHECK(closeResult.error != nullptr);
	CHECK_FALSE(closeResult.success);

	Rml::Shutdown();
}
