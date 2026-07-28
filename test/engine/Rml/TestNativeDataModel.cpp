/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include <catch_amalgamated.hpp>

#include "NativeInterface/api/RmlUi.h"

#include <RmlUi/Core.h>
#include <RmlUi/Core/Context.h>
#include <RmlUi/Core/Element.h>
#include <RmlUi/Core/ElementDocument.h>
#include <RmlUi/Core/Elements/ElementFormControlSelect.h>
#include <RmlUi/Core/RenderInterface.h>
#include <RmlUi/Core/SystemInterface.h>

#include <array>
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

Rml::String SelectValueRml(Rml::ElementFormControlSelect* select)
{
	for (int childIndex = 0; childIndex < select->GetNumChildren(true); ++childIndex) {
		Rml::Element* child = select->GetChild(childIndex);
		if (child->GetTagName() == "selectvalue")
			return child->GetInnerRML();
	}
	return {};
}

Rml::String MakeLogMarkup(size_t rowCount, unsigned revision)
{
	Rml::String markup;
	markup.reserve(rowCount * 96);
	for (size_t index = 0; index < rowCount; ++index) {
		markup += "<div class=\"log-line severity-info\">line ";
		markup += std::to_string(index);
		markup += " revision ";
		markup += std::to_string(revision);
		markup += " -- representative console output</div>";
	}
	return markup;
}

struct LogRows {
	std::vector<std::string> texts;
	std::vector<RmlDataTextRow> rows;

	LogRows(size_t rowCount, unsigned revision)
	{
		texts.reserve(rowCount);
		rows.reserve(rowCount);
		for (size_t index = 0; index < rowCount; ++index) {
			texts.emplace_back("line " + std::to_string(index) + " revision " + std::to_string(revision) + " -- representative console output");
			rows.push_back({.text = texts.back().c_str(), .muted = false});
		}
	}
};

void SetTextRows(uint64_t rowsHandle, const LogRows& rows)
{
	RmlDataModelSetTextRowsQuery query = {
		.rowsHandle = rowsHandle,
		.rows = rows.rows.data(),
		.count = rows.rows.size(),
	};
	RmlElementBoolResult result = {};
	RMLUI_API.DataModelSetTextRows(&query, &result);
	REQUIRE(result.error == nullptr);
	REQUIRE(result.success);
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

	RmlDataModelBindStringQuery bindString = {
		.dataModelHandle = createResult.dataModelHandle,
		.name = "title",
		.initialValue = "initial",
	};
	RmlDataModelBindResult stringResult = {};
	RMLUI_API.DataModelBindString(&bindString, &stringResult);
	REQUIRE(stringResult.error == nullptr);
	REQUIRE(stringResult.success);

	RmlDataModelBindTextRowsQuery bindHistory = {
		.dataModelHandle = createResult.dataModelHandle,
		.name = "history",
	};
	RmlDataModelTextRowsResult historyResult = {};
	RMLUI_API.DataModelBindTextRows(&bindHistory, &historyResult);
	REQUIRE(historyResult.error == nullptr);
	REQUIRE(historyResult.success);

	RmlDataModelBindNotificationRowsQuery bindNotifications = {
		.dataModelHandle = createResult.dataModelHandle,
		.name = "notifications",
	};
	RmlDataModelNotificationRowsResult notificationsResult = {};
	RMLUI_API.DataModelBindNotificationRows(&bindNotifications, &notificationsResult);
	REQUIRE(notificationsResult.error == nullptr);
	REQUIRE(notificationsResult.success);

	RmlDataModelBindOptionRowsQuery bindOptions = {
		.dataModelHandle = createResult.dataModelHandle,
		.name = "options",
	};
	RmlDataModelOptionRowsResult optionsResult = {};
	RMLUI_API.DataModelBindOptionRows(&bindOptions, &optionsResult);
	REQUIRE(optionsResult.error == nullptr);
	REQUIRE(optionsResult.success);

	Rml::ElementDocument* document = context->CreateDocument();
	REQUIRE(document != nullptr);
	document->SetInnerRML(R"(
		<div data-model="editor">
			<p id="bound-title">{{ title }}</p>
			<input id="bound-input" type="text" data-value="title"/>
			<div id="bound-history"><div data-for="row : history">{{ row.text }}</div></div>
			<div id="bound-notifications"><div data-for="notification : notifications"><span>{{ notification.title }}</span><span>{{ notification.body }}</span><span>{{ notification.progress }}</span></div></div>
			<select id="bound-options"><option data-for="option : options" data-if="option.visible" data-attr-value="option.value">{{ option.label }}</option></select>
		</div>
	)");
	document->Show();
	context->Update();
	Rml::Element* title = document->GetElementById("bound-title");
	Rml::Element* input = document->GetElementById("bound-input");
	REQUIRE(title != nullptr);
	REQUIRE(input != nullptr);
	CHECK(title->GetInnerRML() == "initial");

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

	RmlDataTextRow historyRows[] = {
		{.text = "initial edit", .muted = false},
		{.text = "undone edit", .muted = true},
		{.text = "time={{863.446, 0.402}}ms", .muted = false},
	};
	RmlDataModelSetTextRowsQuery setHistory = {
		.rowsHandle = historyResult.rowsHandle,
		.rows = historyRows,
		.count = 3,
	};
	RmlElementBoolResult setHistoryResult = {};
	RMLUI_API.DataModelSetTextRows(&setHistory, &setHistoryResult);
	REQUIRE(setHistoryResult.error == nullptr);
	REQUIRE(setHistoryResult.success);
	context->Update();
	Rml::Element* history = document->GetElementById("bound-history");
	REQUIRE(history != nullptr);
	// `data-for` retains its hidden template element after the generated rows.
	REQUIRE(history->GetNumChildren() == 4);
	CHECK(history->GetChild(0)->GetInnerRML() == "initial edit");
	CHECK(history->GetChild(1)->GetInnerRML() == "undone edit");
	CHECK(history->GetChild(2)->GetInnerRML() == "time={{863.446, 0.402}}ms");

	RmlDataNotificationRow notificationRows[] = {
		{.title = "Warning", .body = "Actual lines are native data", .warning = true, .hasProgress = false, .progress = 0.0f},
		{.title = "Progress", .body = "Loading", .warning = false, .hasProgress = true, .progress = 42.0f},
	};
	RmlDataModelSetNotificationRowsQuery setNotifications = {
		.rowsHandle = notificationsResult.rowsHandle,
		.rows = notificationRows,
		.count = 2,
	};
	RmlElementBoolResult setNotificationsResult = {};
	RMLUI_API.DataModelSetNotificationRows(&setNotifications, &setNotificationsResult);
	REQUIRE(setNotificationsResult.error == nullptr);
	REQUIRE(setNotificationsResult.success);
	context->Update();
	Rml::Element* notifications = document->GetElementById("bound-notifications");
	REQUIRE(notifications != nullptr);
	REQUIRE(notifications->GetNumChildren() == 3);
	CHECK(notifications->GetChild(0)->GetInnerRML() == "<span>Warning</span><span>Actual lines are native data</span><span>0</span>");
	CHECK(notifications->GetChild(1)->GetInnerRML() == "<span>Progress</span><span>Loading</span><span>42</span>");

	RmlDataOptionRow options[] = {
		{.value = "diffuse", .label = "Diffuse map"},
		{.value = "height", .label = "Heightmap"},
	};
	RmlDataModelSetOptionRowsQuery setOptions = {
		.rowsHandle = optionsResult.rowsHandle,
		.rows = options,
		.count = 2,
	};
	RmlElementBoolResult setOptionsResult = {};
	RMLUI_API.DataModelSetOptionRows(&setOptions, &setOptionsResult);
	REQUIRE(setOptionsResult.error == nullptr);
	REQUIRE(setOptionsResult.success);
	// The panel writes the field value before its next Context::Update grows the
	// `data-for` options. RmlUi must retain that requested value as the options
	// materialise, rather than silently selecting the first row.
	document->GetElementById("bound-options")->SetAttribute("value", "height");
	context->Update();
	auto* select = rmlui_dynamic_cast<Rml::ElementFormControlSelect*>(document->GetElementById("bound-options"));
	REQUIRE(select != nullptr);
	CHECK(select->GetValue() == "height");
	CHECK(SelectValueRml(select) == "Heightmap");
	select->SetValue("height");
	CHECK(select->GetValue() == "height");
	CHECK(SelectValueRml(select) == "Heightmap");

	RmlDataModelSetTextRowsQuery clearHistory = {
		.rowsHandle = historyResult.rowsHandle,
		.rows = nullptr,
		.count = 0,
	};
	RmlElementBoolResult clearHistoryResult = {};
	RMLUI_API.DataModelSetTextRows(&clearHistory, &clearHistoryResult);
	REQUIRE(clearHistoryResult.error == nullptr);
	REQUIRE(clearHistoryResult.success);
	context->Update();
	CHECK(history->GetNumChildren() == 1);

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

TEST_CASE("Native text rows avoid reparsing equivalent console markup", "[.][benchmark]")
{
	constexpr std::array<size_t, 3> rowCounts = {12, 200, 500};
	NullRenderInterface renderInterface;
	Rml::SetSystemInterface(&silentSystemInterface);
	Rml::SetRenderInterface(&renderInterface);
	REQUIRE(Rml::Initialise());
	std::vector<Rml::String> contextNames;

	for (const size_t rowCount: rowCounts) {
		const std::string benchmarkName = "native-data-model-benchmark-" + std::to_string(rowCount);
		const std::string modelName = "benchmark-" + std::to_string(rowCount);

		Rml::Context* context = Rml::CreateContext(benchmarkName, {1024, 768});
		REQUIRE(context != nullptr);
		contextNames.emplace_back(context->GetName());

		const Rml::String firstMarkup = MakeLogMarkup(rowCount, 1);
		const Rml::String secondMarkup = MakeLogMarkup(rowCount, 2);
		const LogRows firstRows(rowCount, 1);
		const LogRows secondRows(rowCount, 2);

		Rml::ElementDocument* innerRmlDocument = context->LoadDocumentFromMemory(
			"<rml><body><div id=\"inner-rml-lines\"></div></body></rml>",
			"inner-rml-benchmark.rml"
		);
		REQUIRE(innerRmlDocument != nullptr);
		innerRmlDocument->Show();
		Rml::Element* innerRmlLines = innerRmlDocument->GetElementById("inner-rml-lines");
		REQUIRE(innerRmlLines != nullptr);
		innerRmlLines->SetInnerRML(firstMarkup);
		context->Update();
		REQUIRE(innerRmlLines->GetNumChildren() == rowCount);

		RmlContextCreateDataModelQuery createQuery = {
			.contextHandle = ToHandle(context),
			.name = modelName.c_str(),
		};
		RmlContextOpenDataModelResult createResult = {};
		RMLUI_API.ContextCreateDataModel(&createQuery, &createResult);
		REQUIRE(createResult.error == nullptr);
		REQUIRE(createResult.success);

		RmlDataModelBindTextRowsQuery bindQuery = {
			.dataModelHandle = createResult.dataModelHandle,
			.name = "log_rows",
		};
		RmlDataModelTextRowsResult rowsResult = {};
		RMLUI_API.DataModelBindTextRows(&bindQuery, &rowsResult);
		REQUIRE(rowsResult.error == nullptr);
		REQUIRE(rowsResult.success);

		const Rml::String dataModelRml = "<rml><body data-model=\"" + modelName + "\"><div id=\"data-model-lines\"><div data-for=\"line : log_rows\" class=\"log-line severity-info\">{{line.text}}</div></div></body></rml>";
		Rml::ElementDocument* dataModelDocument = context->LoadDocumentFromMemory(
			dataModelRml,
			"data-model-benchmark.rml"
		);
		REQUIRE(dataModelDocument != nullptr);
		dataModelDocument->Show();
		SetTextRows(rowsResult.rowsHandle, firstRows);
		context->Update();
		Rml::Element* dataModelLines = dataModelDocument->GetElementById("data-model-lines");
		REQUIRE(dataModelLines != nullptr);
		// RmlUi keeps the data-for template in the DOM in addition to the
		// generated rows. It is non-rendered, unlike the rows we compare.
		REQUIRE(dataModelLines->GetNumChildren() == rowCount + 1);

		bool useSecondInnerRml = true;
		BENCHMARK("SetInnerRML + Update (" + std::to_string(rowCount) + " rows)")
		{
			innerRmlLines->SetInnerRML(useSecondInnerRml ? secondMarkup : firstMarkup);
			useSecondInnerRml = !useSecondInnerRml;
			context->Update();
		};

		bool useSecondDataModel = true;
		BENCHMARK("typed text rows + Update (" + std::to_string(rowCount) + " rows)")
		{
			SetTextRows(rowsResult.rowsHandle, useSecondDataModel ? secondRows : firstRows);
			useSecondDataModel = !useSecondDataModel;
			context->Update();
		};
	}

	NativeRmlUi::ClearAllContexts(nullptr);
	for (const Rml::String& contextName : contextNames) {
		REQUIRE(Rml::RemoveContext(contextName));
	}
	Rml::Shutdown();
}
