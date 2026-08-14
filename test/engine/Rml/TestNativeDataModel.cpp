/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include <catch_amalgamated.hpp>

#include "NativeInterface/api/RmlUi.h"

#include <RmlUi/Core.h>
#include <RmlUi/Core/Context.h>
#include <RmlUi/Core/Element.h>
#include <RmlUi/Core/ElementDocument.h>
#include <RmlUi/Core/RenderInterface.h>
#include <RmlUi/Core/SystemInterface.h>

#include <string>
#include <vector>

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

class SilentSystemInterface : public Rml::SystemInterface
{
public:
	bool LogMessage(Rml::Log::Type, const Rml::String&) override { return true; }
};

SilentSystemInterface silentSystemInterface;

uint64_t ToHandle(const void* pointer)
{
	return reinterpret_cast<uint64_t>(pointer);
}

struct DataEventCapture
{
	std::vector<int32_t> indexes;
	std::vector<uint64_t> eventHandles;
	std::vector<uint64_t> targetElementHandles;
	int destroyCount = 0;
};

void CaptureDataEvent(void* userData, const RmlDataEventArgs* args)
{
	auto& capture = *static_cast<DataEventCapture*>(userData);
	REQUIRE(args != nullptr);
	REQUIRE(args->values != nullptr);
	REQUIRE(args->count == 1);
	REQUIRE(args->values[0].type == RML_FIELD_INT);
	capture.indexes.push_back(args->values[0].intValue);
	capture.eventHandles.push_back(args->eventHandle);
	capture.targetElementHandles.push_back(args->targetElementHandle);
}

void DestroyDataEvent(void* userData)
{
	++static_cast<DataEventCapture*>(userData)->destroyCount;
}
}

// The native API translation unit links against the RmlUi backend. Typed data
// models do not reach it, so keep this test focused on the API layer.
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

TEST_CASE("Native RmlUi data-model values stay typed and engine-owned")
{
	NullRenderInterface renderInterface;
	Rml::SetSystemInterface(&silentSystemInterface);
	Rml::SetRenderInterface(&renderInterface);
	REQUIRE(Rml::Initialise());

	Rml::Context* context = Rml::CreateContext("native-data-model", {1024, 768});
	REQUIRE(context != nullptr);

	RmlContextCreateDataModelQuery createQuery = {
		.contextHandle = ToHandle(context),
		.name = "editor",
	};
	RmlContextOpenDataModelResult createResult = {};
	RMLUI_API.ContextCreateDataModel(&createQuery, &createResult);
	REQUIRE(createResult.error == nullptr);
	REQUIRE(createResult.success);

	RmlDataModelBindBoolQuery bindBool = {
		.dataModelHandle = createResult.dataModelHandle,
		.name = "enabled",
		.initialValue = false,
	};
	RmlDataModelBindResult boolResult = {};
	RMLUI_API.DataModelBindBool(&bindBool, &boolResult);
	REQUIRE(boolResult.error == nullptr);
	REQUIRE(boolResult.success);

	RmlDataModelBindIntQuery bindInt = {
		.dataModelHandle = createResult.dataModelHandle,
		.name = "count",
		.initialValue = 3,
	};
	RmlDataModelBindResult intResult = {};
	RMLUI_API.DataModelBindInt(&bindInt, &intResult);
	REQUIRE(intResult.error == nullptr);
	REQUIRE(intResult.success);

	RmlDataModelBindFloatQuery bindFloat = {
		.dataModelHandle = createResult.dataModelHandle,
		.name = "opacity",
		.initialValue = 0.25f,
	};
	RmlDataModelBindResult floatResult = {};
	RMLUI_API.DataModelBindFloat(&bindFloat, &floatResult);
	REQUIRE(floatResult.error == nullptr);
	REQUIRE(floatResult.success);

	RmlDataModelBindPixelsQuery bindPixels = {
		.dataModelHandle = createResult.dataModelHandle,
		.name = "offset",
		.initialValue = 0.25f,
	};
	RmlDataModelBindResult pixelsResult = {};
	RMLUI_API.DataModelBindPixels(&bindPixels, &pixelsResult);
	REQUIRE(pixelsResult.error == nullptr);
	REQUIRE(pixelsResult.success);

	RmlDataModelBindPercentQuery bindPercent = {
		.dataModelHandle = createResult.dataModelHandle,
		.name = "progress",
		.initialValue = 25.0f,
	};
	RmlDataModelBindResult percentResult = {};
	RMLUI_API.DataModelBindPercent(&bindPercent, &percentResult);
	REQUIRE(percentResult.error == nullptr);
	REQUIRE(percentResult.success);

	RmlDataModelBindStringQuery bindString = {
		.dataModelHandle = createResult.dataModelHandle,
		.name = "title",
		.initialValue = "initial",
	};
	RmlDataModelBindResult stringResult = {};
	RMLUI_API.DataModelBindString(&bindString, &stringResult);
	REQUIRE(stringResult.error == nullptr);
	REQUIRE(stringResult.success);

	RmlDataModelBindColorQuery bindColour = {
		.dataModelHandle = createResult.dataModelHandle,
		.name = "accent_colour",
		.red = 16,
		.green = 64,
		.blue = 192,
		.alpha = 255,
	};
	RmlDataModelBindResult colourResult = {};
	RMLUI_API.DataModelBindColor(&bindColour, &colourResult);
	REQUIRE(colourResult.error == nullptr);
	REQUIRE(colourResult.success);

	Rml::ElementDocument* document = context->CreateDocument();
	REQUIRE(document != nullptr);
	document->SetInnerRML(R"(
		<div data-model="editor">
			<p id="bound-title">{{ title }}</p>
			<span id="bound-colour" data-style-background-color="accent_colour"></span>
			<span id="bound-left" data-style-left="offset"></span>
			<span id="bound-progress" data-style-width="progress"></span>
			<input id="bound-input" type="text" data-value="title"/>
		</div>
	)");
	document->Show();
	context->Update();
	Rml::Element* title = document->GetElementById("bound-title");
	Rml::Element* input = document->GetElementById("bound-input");
	REQUIRE(title != nullptr);
	REQUIRE(input != nullptr);
	CHECK(title->GetInnerRML() == "initial");
	Rml::Element* left = document->GetElementById("bound-left");
	REQUIRE(left != nullptr);
	REQUIRE(left->GetProperty(Rml::PropertyId::Left) != nullptr);
	CHECK(left->GetProperty(Rml::PropertyId::Left)->unit == Rml::Unit::PX);
	CHECK(left->GetProperty(Rml::PropertyId::Left)->Get<float>() == Catch::Approx(0.25f));

	RmlDataModelVariablePixelsQuery setPixels = {.variableHandle = pixelsResult.variableHandle, .value = 42.0f};
	RmlElementBoolResult setPixelsResult = {};
	RMLUI_API.DataModelSetPixels(&setPixels, &setPixelsResult);
	REQUIRE(setPixelsResult.error == nullptr);
	REQUIRE(setPixelsResult.success);
	context->Update();
	CHECK(left->GetProperty(Rml::PropertyId::Left)->unit == Rml::Unit::PX);
	CHECK(left->GetProperty(Rml::PropertyId::Left)->Get<float>() == Catch::Approx(42.0f));
	Rml::Element* progress = document->GetElementById("bound-progress");
	REQUIRE(progress != nullptr);
	CHECK(progress->GetProperty(Rml::PropertyId::Width)->unit == Rml::Unit::PERCENT);
	CHECK(progress->GetProperty(Rml::PropertyId::Width)->Get<float>() == Catch::Approx(25.0f));

	RmlDataModelVariablePercentQuery setPercent = {.variableHandle = percentResult.variableHandle, .value = 88.0f};
	RmlElementBoolResult setPercentResult = {};
	RMLUI_API.DataModelSetPercent(&setPercent, &setPercentResult);
	REQUIRE(setPercentResult.error == nullptr);
	REQUIRE(setPercentResult.success);
	context->Update();
	CHECK(progress->GetProperty(Rml::PropertyId::Width)->unit == Rml::Unit::PERCENT);
	CHECK(progress->GetProperty(Rml::PropertyId::Width)->Get<float>() == Catch::Approx(88.0f));
	Rml::Element* colour = document->GetElementById("bound-colour");
	REQUIRE(colour != nullptr);
	CHECK(colour->GetProperty<Rml::Colourb>("background-color") == Rml::Colourb(16, 64, 192, 255));

	RmlDataModelVariableColorQuery setColour = {
		.variableHandle = colourResult.variableHandle,
		.red = 255,
		.green = 96,
		.blue = 32,
		.alpha = 200,
	};
	RmlElementBoolResult setColourResult = {};
	RMLUI_API.DataModelSetColor(&setColour, &setColourResult);
	REQUIRE(setColourResult.error == nullptr);
	REQUIRE(setColourResult.success);
	context->Update();
	CHECK(colour->GetProperty<Rml::Colourb>("background-color") == Rml::Colourb(255, 96, 32, 200));

	RmlDataModelVariableBoolQuery setBool = {.variableHandle = boolResult.variableHandle, .value = true};
	RmlElementBoolResult setBoolResult = {};
	RMLUI_API.DataModelSetBool(&setBool, &setBoolResult);
	REQUIRE(setBoolResult.error == nullptr);
	REQUIRE(setBoolResult.success);

	RmlDataModelVariableIntQuery setInt = {.variableHandle = intResult.variableHandle, .value = 42};
	RmlElementBoolResult setIntResult = {};
	RMLUI_API.DataModelSetInt(&setInt, &setIntResult);
	REQUIRE(setIntResult.error == nullptr);
	REQUIRE(setIntResult.success);

	RmlDataModelVariableFloatQuery setFloat = {.variableHandle = floatResult.variableHandle, .value = 0.75f};
	RmlElementBoolResult setFloatResult = {};
	RMLUI_API.DataModelSetFloat(&setFloat, &setFloatResult);
	REQUIRE(setFloatResult.error == nullptr);
	REQUIRE(setFloatResult.success);

	RmlDataModelVariableStringQuery setString = {.variableHandle = stringResult.variableHandle, .value = "updated"};
	RmlElementBoolResult setStringResult = {};
	RMLUI_API.DataModelSetString(&setString, &setStringResult);
	REQUIRE(setStringResult.error == nullptr);
	REQUIRE(setStringResult.success);
	context->Update();
	CHECK(title->GetInnerRML() == "updated");


	// This is the same `change` event raised by an edited text input. RmlUi
	// writes directly into the engine-owned string slot; the API then reads the
	// typed value back without decoding a transport representation.
	Rml::Dictionary changedValue;
	changedValue["value"] = "from-view";
	input->DispatchEvent("change", changedValue);

	RmlDataModelVariableHandleQuery getBool = {.variableHandle = boolResult.variableHandle};
	RmlDataModelGetBoolResult getBoolResult = {};
	RMLUI_API.DataModelGetBool(&getBool, &getBoolResult);
	CHECK(getBoolResult.error == nullptr);
	CHECK(getBoolResult.success);
	CHECK(getBoolResult.value);

	RmlDataModelVariableHandleQuery getInt = {.variableHandle = intResult.variableHandle};
	RmlDataModelGetIntResult getIntResult = {};
	RMLUI_API.DataModelGetInt(&getInt, &getIntResult);
	CHECK(getIntResult.error == nullptr);
	CHECK(getIntResult.success);
	CHECK(getIntResult.value == 42);

	RmlDataModelVariableHandleQuery getFloat = {.variableHandle = floatResult.variableHandle};
	RmlDataModelGetFloatResult getFloatResult = {};
	RMLUI_API.DataModelGetFloat(&getFloat, &getFloatResult);
	CHECK(getFloatResult.error == nullptr);
	CHECK(getFloatResult.success);
	CHECK(getFloatResult.value == Catch::Approx(0.75f));

	RmlDataModelVariableHandleQuery getString = {.variableHandle = stringResult.variableHandle};
	RmlDataModelGetStringResult getStringResult = {};
	RMLUI_API.DataModelGetString(&getString, &getStringResult);
	CHECK(getStringResult.error == nullptr);
	CHECK(getStringResult.success);
	CHECK(std::string(getStringResult.value) == "from-view");

	RmlDataModelVariableHandleQuery getColour = {.variableHandle = colourResult.variableHandle};
	RmlDataModelGetColorResult getColourResult = {};
	RMLUI_API.DataModelGetColor(&getColour, &getColourResult);
	CHECK(getColourResult.error == nullptr);
	CHECK(getColourResult.success);
	CHECK(getColourResult.red == 255);
	CHECK(getColourResult.green == 96);
	CHECK(getColourResult.blue == 32);
	CHECK(getColourResult.alpha == 200);

	RmlDataModelVariableHandleQuery getPixels = {.variableHandle = pixelsResult.variableHandle};
	RmlDataModelGetPixelsResult getPixelsResult = {};
	RMLUI_API.DataModelGetPixels(&getPixels, &getPixelsResult);
	CHECK(getPixelsResult.error == nullptr);
	CHECK(getPixelsResult.success);
	CHECK(getPixelsResult.value == Catch::Approx(42.0f));

	RmlDataModelVariableHandleQuery getPercent = {.variableHandle = percentResult.variableHandle};
	RmlDataModelGetPercentResult getPercentResult = {};
	RMLUI_API.DataModelGetPercent(&getPercent, &getPercentResult);
	CHECK(getPercentResult.error == nullptr);
	CHECK(getPercentResult.success);
	CHECK(getPercentResult.value == Catch::Approx(88.0f));

	SECTION("a mismatched typed handle is rejected without touching the slot")
	{
		RmlDataModelVariableIntQuery wrongType = {.variableHandle = boolResult.variableHandle, .value = 99};
		RmlElementBoolResult wrongTypeResult = {};
		RMLUI_API.DataModelSetInt(&wrongType, &wrongTypeResult);
		CHECK(wrongTypeResult.error != nullptr);
		CHECK_FALSE(wrongTypeResult.success);
	}

	REQUIRE(Rml::RemoveContext(context->GetName()));
	Rml::Shutdown();
}

TEST_CASE("Native runtime data rows resolve fields and retain their high-water count")
{
	NullRenderInterface renderInterface;
	Rml::SetSystemInterface(&silentSystemInterface);
	Rml::SetRenderInterface(&renderInterface);
	REQUIRE(Rml::Initialise());

	Rml::Context* context = Rml::CreateContext("native-runtime-data-rows", {1024, 768});
	REQUIRE(context != nullptr);

	RmlContextCreateDataModelQuery createQuery = {
		.contextHandle = ToHandle(context),
		.name = "runtime",
	};
	RmlContextOpenDataModelResult createResult = {};
	RMLUI_API.ContextCreateDataModel(&createQuery, &createResult);
	REQUIRE(createResult.error == nullptr);
	REQUIRE(createResult.success);

	RmlDataFieldDef rowFields[] = {
		{.name = "label", .type = RML_FIELD_STRING},
		{.name = "selected", .type = RML_FIELD_BOOL},
	};
	RmlDataModelBindRowsQuery bindRows = {
		.dataModelHandle = createResult.dataModelHandle,
		.name = "rows",
		.fields = rowFields,
		.fieldCount = std::size(rowFields),
	};
	RmlDataModelRowsResult rowsResult = {};
	RMLUI_API.DataModelBindRows(&bindRows, &rowsResult);
	REQUIRE(rowsResult.error == nullptr);
	REQUIRE(rowsResult.success);

	DataEventCapture eventCapture;
	const uint8_t eventFieldTypes[] = {RML_FIELD_INT};
	RmlDataModelBindEventQuery bindEvent = {
		.dataModelHandle = createResult.dataModelHandle,
		.name = "select",
		.callback = CaptureDataEvent,
		.userData = &eventCapture,
		.destroyCallback = DestroyDataEvent,
		.fieldTypes = eventFieldTypes,
		.fieldCount = std::size(eventFieldTypes),
	};
	RmlDataModelBindEventResult eventResult = {};
	RMLUI_API.DataModelBindEvent(&bindEvent, &eventResult);
	REQUIRE(eventResult.error == nullptr);
	REQUIRE(eventResult.success);
	REQUIRE(eventResult.eventHandle != 0);
	CHECK(eventCapture.destroyCount == 0);

	RmlDataFieldDef styledFields[] = {
		{.name = "colour", .type = RML_FIELD_COLOR},
		{.name = "offset", .type = RML_FIELD_PIXELS},
		{.name = "progress", .type = RML_FIELD_PERCENT},
	};
	RmlDataModelBindRowsQuery bindStyledRows = {
		.dataModelHandle = createResult.dataModelHandle,
		.name = "styled",
		.fields = styledFields,
		.fieldCount = std::size(styledFields),
	};
	RmlDataModelRowsResult styledRowsResult = {};
	RMLUI_API.DataModelBindRows(&bindStyledRows, &styledRowsResult);
	REQUIRE(styledRowsResult.error == nullptr);
	REQUIRE(styledRowsResult.success);

	Rml::ElementDocument* document = context->CreateDocument();
	REQUIRE(document != nullptr);
	document->SetInnerRML(R"RML(
		<div data-model="runtime">
			<div id="bound-rows"><div data-for="row : rows" data-if="row.visible" data-class-selected="row.selected" data-event-click="select(it_index)">{{ row.label }}</div></div>
			<div id="wrong-arity" data-event-click="select()"></div>
			<div id="bound-styled"><div data-for="row : styled" data-if="row.visible"><span data-style-background-color="row.colour" data-style-left="row.offset" data-style-width="row.progress"></span></div></div>
		</div>
	)RML");
	document->Show();

	RmlDataValue fiveRows[] = {
		{.type = RML_FIELD_STRING, .stringValue = "first"},
		{.type = RML_FIELD_BOOL, .boolValue = true},
		{.type = RML_FIELD_STRING, .stringValue = "second"},
		{.type = RML_FIELD_BOOL, .boolValue = false},
		{.type = RML_FIELD_STRING, .stringValue = "third"},
		{.type = RML_FIELD_BOOL, .boolValue = false},
		{.type = RML_FIELD_STRING, .stringValue = "fourth"},
		{.type = RML_FIELD_BOOL, .boolValue = true},
		{.type = RML_FIELD_STRING, .stringValue = "fifth"},
		{.type = RML_FIELD_BOOL, .boolValue = false},
	};
	RmlDataModelSetRowsQuery setFiveRows = {
		.rowsHandle = rowsResult.rowsHandle,
		.values = fiveRows,
		.rowCount = 5,
	};
	RmlElementBoolResult setFiveRowsResult = {};
	RMLUI_API.DataModelSetRows(&setFiveRows, &setFiveRowsResult);
	REQUIRE(setFiveRowsResult.error == nullptr);
	REQUIRE(setFiveRowsResult.success);

	RmlDataValue styledValues[] = {
		{.type = RML_FIELD_COLOR, .red = 16, .green = 64, .blue = 192, .alpha = 255},
		{.type = RML_FIELD_PIXELS, .floatValue = 12.5f},
		{.type = RML_FIELD_PERCENT, .floatValue = 35.0f},
	};
	RmlDataModelSetRowsQuery setStyledRows = {
		.rowsHandle = styledRowsResult.rowsHandle,
		.values = styledValues,
		.rowCount = 1,
	};
	RmlElementBoolResult setStyledRowsResult = {};
	RMLUI_API.DataModelSetRows(&setStyledRows, &setStyledRowsResult);
	REQUIRE(setStyledRowsResult.error == nullptr);
	REQUIRE(setStyledRowsResult.success);

	context->Update();
	Rml::Element* boundRows = document->GetElementById("bound-rows");
	Rml::Element* wrongArity = document->GetElementById("wrong-arity");
	Rml::Element* boundStyled = document->GetElementById("bound-styled");
	REQUIRE(boundRows != nullptr);
	REQUIRE(wrongArity != nullptr);
	REQUIRE(boundStyled != nullptr);
	REQUIRE(boundRows->GetNumChildren() == 6);
	CHECK(boundRows->GetChild(0)->GetInnerRML() == "first");
	CHECK(boundRows->GetChild(0)->IsClassSet("selected"));
	CHECK(boundRows->GetChild(1)->GetInnerRML() == "second");
	CHECK_FALSE(boundRows->GetChild(1)->IsClassSet("selected"));
	CHECK(boundRows->GetChild(4)->GetInnerRML() == "fifth");
	REQUIRE(boundStyled->GetNumChildren() == 2);
	Rml::Element* styledElement = boundStyled->GetChild(0)->GetChild(0);
	REQUIRE(styledElement != nullptr);
	CHECK(styledElement->GetProperty<Rml::Colourb>("background-color") == Rml::Colourb(16, 64, 192, 255));
	REQUIRE(styledElement->GetProperty(Rml::PropertyId::Left) != nullptr);
	CHECK(styledElement->GetProperty(Rml::PropertyId::Left)->unit == Rml::Unit::PX);
	CHECK(styledElement->GetProperty(Rml::PropertyId::Left)->Get<float>() == Catch::Approx(12.5f));
	REQUIRE(styledElement->GetProperty(Rml::PropertyId::Width) != nullptr);
	CHECK(styledElement->GetProperty(Rml::PropertyId::Width)->unit == Rml::Unit::PERCENT);
	CHECK(styledElement->GetProperty(Rml::PropertyId::Width)->Get<float>() == Catch::Approx(35.0f));

	wrongArity->DispatchEvent("click", Rml::Dictionary{});
	CHECK(eventCapture.indexes.empty());

	boundRows->GetChild(3)->DispatchEvent("click", Rml::Dictionary{});
	REQUIRE(eventCapture.indexes.size() == 1);
	CHECK(eventCapture.indexes[0] == 3);
	CHECK(eventCapture.eventHandles[0] != 0);
	CHECK(eventCapture.targetElementHandles[0] == ToHandle(boundRows->GetChild(3)));

	RmlDataValue twoRows[] = {
		{.type = RML_FIELD_STRING, .stringValue = "replacement"},
		{.type = RML_FIELD_BOOL, .boolValue = false},
		{.type = RML_FIELD_STRING, .stringValue = "last visible"},
		{.type = RML_FIELD_BOOL, .boolValue = true},
	};
	RmlDataModelSetRowsQuery setTwoRows = {
		.rowsHandle = rowsResult.rowsHandle,
		.values = twoRows,
		.rowCount = 2,
	};
	RmlElementBoolResult setTwoRowsResult = {};
	RMLUI_API.DataModelSetRows(&setTwoRows, &setTwoRowsResult);
	REQUIRE(setTwoRowsResult.error == nullptr);
	REQUIRE(setTwoRowsResult.success);
	context->Update();

	// The engine owns the data-for padding: after five rows, assigning two
	// leaves five addressable rows, with the final three explicitly invisible.
	REQUIRE(boundRows->GetNumChildren() == 6);
	CHECK(boundRows->GetChild(0)->GetInnerRML() == "replacement");
	CHECK_FALSE(boundRows->GetChild(0)->IsClassSet("selected"));
	CHECK(boundRows->GetChild(1)->GetInnerRML() == "last visible");
	CHECK(boundRows->GetChild(1)->IsClassSet("selected"));
	CHECK_FALSE(boundRows->GetChild(2)->IsVisible());
	CHECK_FALSE(boundRows->GetChild(3)->IsVisible());
	CHECK_FALSE(boundRows->GetChild(4)->IsVisible());

	boundRows->GetChild(1)->DispatchEvent("click", Rml::Dictionary{});
	REQUIRE(eventCapture.indexes.size() == 2);
	CHECK(eventCapture.indexes[1] == 1);
	CHECK(eventCapture.targetElementHandles[1] == ToHandle(boundRows->GetChild(1)));

	RmlDataModelEventHandleQuery unbindEvent = {
		.eventHandle = eventResult.eventHandle,
	};
	RmlElementBoolResult unbindEventResult = {};
	RMLUI_API.DataModelUnbindEvent(&unbindEvent, &unbindEventResult);
	REQUIRE(unbindEventResult.error == nullptr);
	REQUIRE(unbindEventResult.success);
	CHECK(eventCapture.destroyCount == 1);
	boundRows->GetChild(1)->DispatchEvent("click", Rml::Dictionary{});
	CHECK(eventCapture.indexes.size() == 2);

	eventResult = {};
	RMLUI_API.DataModelBindEvent(&bindEvent, &eventResult);
	REQUIRE(eventResult.error == nullptr);
	REQUIRE(eventResult.success);
	REQUIRE(eventResult.eventHandle != 0);
	CHECK(eventCapture.destroyCount == 1);
	boundRows->GetChild(1)->DispatchEvent("click", Rml::Dictionary{});
	REQUIRE(eventCapture.indexes.size() == 3);
	CHECK(eventCapture.indexes[2] == 1);

	RmlContextStringQuery removeModel = {
		.contextHandle = ToHandle(context),
		.name = "runtime",
	};
	RmlContextBoolResult removeModelResult = {};
	RMLUI_API.ContextRemoveDataModel(&removeModel, &removeModelResult);
	REQUIRE(removeModelResult.error == nullptr);
	REQUIRE(removeModelResult.success);
	CHECK(eventCapture.destroyCount == 2);

	REQUIRE(Rml::RemoveContext(context->GetName()));
	Rml::Shutdown();
}
