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
			rows.push_back({.text = texts.back().c_str(), .muted = false, .visible = true});
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

	RmlDataModelBindTextRowsQuery bindHistory = {
		.dataModelHandle = createResult.dataModelHandle,
		.name = "history",
	};
	RmlDataModelTextRowsResult historyResult = {};
	RMLUI_API.DataModelBindTextRows(&bindHistory, &historyResult);
	REQUIRE(historyResult.error == nullptr);
	REQUIRE(historyResult.success);

	RmlDataModelBindLogRowsQuery bindLog = {
		.dataModelHandle = createResult.dataModelHandle,
		.name = "log",
	};
	RmlDataModelLogRowsResult logResult = {};
	RMLUI_API.DataModelBindLogRows(&bindLog, &logResult);
	REQUIRE(logResult.error == nullptr);
	REQUIRE(logResult.success);

	RmlDataModelBindNotificationRowsQuery bindNotifications = {
		.dataModelHandle = createResult.dataModelHandle,
		.name = "notifications",
	};
	RmlDataModelNotificationRowsResult notificationsResult = {};
	RMLUI_API.DataModelBindNotificationRows(&bindNotifications, &notificationsResult);
	REQUIRE(notificationsResult.error == nullptr);
	REQUIRE(notificationsResult.success);

	RmlDataModelBindIconRowsQuery bindIcons = {
		.dataModelHandle = createResult.dataModelHandle,
		.name = "icons",
	};
	RmlDataModelIconRowsResult iconsResult = {};
	RMLUI_API.DataModelBindIconRows(&bindIcons, &iconsResult);
	REQUIRE(iconsResult.error == nullptr);
	REQUIRE(iconsResult.success);

	RmlDataModelBindOptionRowsQuery bindOptions = {
		.dataModelHandle = createResult.dataModelHandle,
		.name = "options",
	};
	RmlDataModelOptionRowsResult optionsResult = {};
	RMLUI_API.DataModelBindOptionRows(&bindOptions, &optionsResult);
	REQUIRE(optionsResult.error == nullptr);
	REQUIRE(optionsResult.success);

	RmlDataModelBindChoiceRowsQuery bindChoices = {
		.dataModelHandle = createResult.dataModelHandle,
		.name = "choices",
	};
	RmlDataModelChoiceRowsResult choicesResult = {};
	RMLUI_API.DataModelBindChoiceRows(&bindChoices, &choicesResult);
	REQUIRE(choicesResult.error == nullptr);
	REQUIRE(choicesResult.success);

	RmlDataModelBindStatusRowsQuery bindStatuses = {
		.dataModelHandle = createResult.dataModelHandle,
		.name = "statuses",
	};
	RmlDataModelStatusRowsResult statusesResult = {};
	RMLUI_API.DataModelBindStatusRows(&bindStatuses, &statusesResult);
	REQUIRE(statusesResult.error == nullptr);
	REQUIRE(statusesResult.success);

	RmlDataModelBindSwatchRowsQuery bindSwatches = {
		.dataModelHandle = createResult.dataModelHandle,
		.name = "swatches",
	};
	RmlDataModelSwatchRowsResult swatchesResult = {};
	RMLUI_API.DataModelBindSwatchRows(&bindSwatches, &swatchesResult);
	REQUIRE(swatchesResult.error == nullptr);
	REQUIRE(swatchesResult.success);

	RmlDataModelBindGridRowsQuery bindGrid = {
		.dataModelHandle = createResult.dataModelHandle,
		.name = "grid",
	};
	RmlDataModelGridRowsResult gridResult = {};
	RMLUI_API.DataModelBindGridRows(&bindGrid, &gridResult);
	REQUIRE(gridResult.error == nullptr);
	REQUIRE(gridResult.success);

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
			<div id="bound-history"><div data-for="row : history" data-if="row.visible" data-class-undone="row.muted">{{ row.text }}</div></div>
			<div id="bound-log"><div data-for="line : log" data-if="line.visible" data-class-severity-info="line.info" data-class-severity-warning="line.warning" data-class-severity-error="line.error" data-class-selected="line.selected">{{ line.text }}</div></div>
			<div id="bound-notifications"><div data-for="notification : notifications" data-class-warning="notification.warning"><span>{{ notification.title }}</span><span>{{ notification.body }}</span><span data-if="notification.has_progress" data-style-width="notification.progress"></span></div></div>
			<div id="bound-icons"><button data-for="icon : icons" data-if="icon.visible" data-attr-title="icon.tooltip" data-class-pressed="icon.pressed" data-class-disabled="icon.disabled"><img data-attr-src="icon.icon"/><span>{{ icon.label }}</span></button></div>
			<select id="bound-options"><option data-for="option : options" data-if="option.visible" data-attr-value="option.value">{{ option.label }}</option></select>
			<div id="bound-choices"><div data-for="choice : choices" data-if="choice.visible" data-class-selected="choice.selected" data-class-highlighted="choice.highlighted"><span>{{ choice.label }}</span><span>{{ choice.detail }}</span></div></div>
			<div id="bound-statuses"><div data-for="status : statuses" data-if="status.visible" data-class-positive="status.positive">{{ status.label }}</div></div>
			<div id="bound-swatches"><div data-for="swatch : swatches" data-if="swatch.visible"><span data-style-background-color="swatch.colour"></span><span>{{ swatch.label }}</span><button data-if="swatch.actions_enabled">Edit</button></div></div>
			<div id="bound-grid"><div data-for="item : grid" data-if="item.visible" data-class-selected="item.selected" data-class-folder="item.folder" data-class-grid-filler="item.filler" data-style-flex-basis="item.cell_size"><img data-if="item.has_image &amp;&amp; !item.native_image" data-attr-src="item.image"/><texture data-if="item.has_image &amp;&amp; item.native_image" data-attr-src="item.image"/><span>{{ item.label }}</span></div></div>
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

	RmlDataTextRow historyRows[] = {
		{.text = "initial edit", .muted = false, .visible = true},
		{.text = "undone edit", .muted = true, .visible = true},
		{.text = "time={{863.446, 0.402}}ms", .muted = false, .visible = true},
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
	CHECK(history->GetChild(1)->IsClassSet("undone"));
	CHECK(history->GetChild(2)->GetInnerRML() == "time={{863.446, 0.402}}ms");

	RmlDataLogRow logRows[] = {
		{.text = "routine", .severity = 0, .selected = false},
		{.text = "careful", .severity = 1, .selected = true},
		{.text = "failed", .severity = 2, .selected = false},
		{.text = "", .severity = 0, .selected = false},
	};
	RmlDataModelSetLogRowsQuery setLog = {
		.rowsHandle = logResult.rowsHandle,
		.rows = logRows,
		.count = 4,
	};
	RmlElementBoolResult setLogResult = {};
	RMLUI_API.DataModelSetLogRows(&setLog, &setLogResult);
	REQUIRE(setLogResult.error == nullptr);
	REQUIRE(setLogResult.success);
	context->Update();
	Rml::Element* log = document->GetElementById("bound-log");
	REQUIRE(log != nullptr);
	REQUIRE(log->GetNumChildren() == 5);
	CHECK(log->GetChild(0)->IsClassSet("severity-info"));
	CHECK(log->GetChild(1)->IsClassSet("severity-warning"));
	CHECK(log->GetChild(1)->IsClassSet("selected"));
	CHECK(log->GetChild(2)->IsClassSet("severity-error"));

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
	CHECK(notifications->GetChild(0)->IsClassSet("warning"));
	CHECK(notifications->GetChild(0)->GetChild(0)->GetInnerRML() == "Warning");
	CHECK_FALSE(notifications->GetChild(0)->GetChild(2)->IsVisible());
	CHECK_FALSE(notifications->GetChild(1)->IsClassSet("warning"));
	CHECK(notifications->GetChild(1)->GetChild(1)->GetInnerRML() == "Loading");
	const Rml::Property* progressWidth = notifications->GetChild(1)->GetChild(2)->GetProperty(Rml::PropertyId::Width);
	REQUIRE(progressWidth != nullptr);
	CHECK(progressWidth->unit == Rml::Unit::PERCENT);
	CHECK(progressWidth->Get<float>() == Catch::Approx(42.0f));

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

	// Collections are kept at their previous high-water count by the Rust
	// wrapper. A blank tail row keeps RmlUi from evaluating stale data-for
	// indexes, while `visible` keeps it hidden from the rendered list.
	RmlDataChoiceRow choices[] = {
		{.label = "/airmesh", .detail = "Show or hide the air mesh", .selected = true, .highlighted = false},
		{.label = "/autocheat", .detail = "Toggle automatic cheating", .selected = false, .highlighted = true},
		{.label = "", .detail = "", .selected = false, .highlighted = false},
	};
	RmlDataModelSetChoiceRowsQuery setChoices = {
		.rowsHandle = choicesResult.rowsHandle,
		.rows = choices,
		.count = 3,
	};
	RmlElementBoolResult setChoicesResult = {};
	RMLUI_API.DataModelSetChoiceRows(&setChoices, &setChoicesResult);
	REQUIRE(setChoicesResult.error == nullptr);
	REQUIRE(setChoicesResult.success);
	context->Update();
	Rml::Element* boundChoices = document->GetElementById("bound-choices");
	REQUIRE(boundChoices != nullptr);
	CHECK(boundChoices->GetInnerRML().find("/airmesh") != Rml::String::npos);
	CHECK(boundChoices->GetInnerRML().find("Show or hide the air mesh") != Rml::String::npos);
	CHECK(boundChoices->GetInnerRML().find("/autocheat") != Rml::String::npos);
	CHECK(boundChoices->GetInnerRML().find("Toggle automatic cheating") != Rml::String::npos);
	// RmlUi retains the hidden generated row and its data-for template in the
	// DOM, so the two visible choices occupy the first two children.
	REQUIRE(boundChoices->GetNumChildren() == 4);
	CHECK(boundChoices->GetChild(0)->IsClassSet("selected"));
	CHECK_FALSE(boundChoices->GetChild(1)->IsClassSet("selected"));
	CHECK(boundChoices->GetChild(1)->IsClassSet("highlighted"));

	RmlDataIconRow icons[] = {
		{.label = "Paint", .icon = "LuaUI/images/paint.png", .tooltip = "Paint terrain", .pressed = true, .disabled = false},
		{.label = "Smooth", .icon = "LuaUI/images/smooth.png", .tooltip = "Smooth terrain", .pressed = false, .disabled = true},
	};
	RmlDataModelSetIconRowsQuery setIcons = {
		.rowsHandle = iconsResult.rowsHandle,
		.rows = icons,
		.count = 2,
	};
	RmlElementBoolResult setIconsResult = {};
	RMLUI_API.DataModelSetIconRows(&setIcons, &setIconsResult);
	REQUIRE(setIconsResult.error == nullptr);
	REQUIRE(setIconsResult.success);
	context->Update();
	Rml::Element* boundIcons = document->GetElementById("bound-icons");
	REQUIRE(boundIcons != nullptr);
	CHECK(boundIcons->GetChild(0)->IsClassSet("pressed"));
	CHECK_FALSE(boundIcons->GetChild(0)->IsClassSet("disabled"));
	CHECK(boundIcons->GetChild(1)->IsClassSet("disabled"));
	CHECK(boundIcons->GetChild(1)->GetAttribute("title", Rml::String{}) == "Smooth terrain");

	RmlDataStatusRow statuses[] = {
		{.label = "Diffuse", .positive = true},
		{.label = "Specular", .positive = false},
		{.label = "", .positive = false},
	};
	RmlDataModelSetStatusRowsQuery setStatuses = {
		.rowsHandle = statusesResult.rowsHandle,
		.rows = statuses,
		.count = 3,
	};
	RmlElementBoolResult setStatusesResult = {};
	RMLUI_API.DataModelSetStatusRows(&setStatuses, &setStatusesResult);
	REQUIRE(setStatusesResult.error == nullptr);
	REQUIRE(setStatusesResult.success);
	context->Update();
	Rml::Element* boundStatuses = document->GetElementById("bound-statuses");
	REQUIRE(boundStatuses != nullptr);
	REQUIRE(boundStatuses->GetNumChildren() == 4);
	CHECK(boundStatuses->GetChild(0)->GetInnerRML() == "Diffuse");
	CHECK(boundStatuses->GetChild(0)->IsClassSet("positive"));
	CHECK_FALSE(boundStatuses->GetChild(1)->IsClassSet("positive"));

	RmlDataSwatchRow swatches[] = {
		{.label = "Player", .red = 64, .green = 128, .blue = 255, .alpha = 255, .actionsEnabled = true},
		{.label = "Gaia", .red = 64, .green = 192, .blue = 96, .alpha = 255, .actionsEnabled = false},
		{.label = "", .red = 0, .green = 0, .blue = 0, .alpha = 0, .actionsEnabled = false},
	};
	RmlDataModelSetSwatchRowsQuery setSwatches = {
		.rowsHandle = swatchesResult.rowsHandle,
		.rows = swatches,
		.count = 3,
	};
	RmlElementBoolResult setSwatchesResult = {};
	RMLUI_API.DataModelSetSwatchRows(&setSwatches, &setSwatchesResult);
	REQUIRE(setSwatchesResult.error == nullptr);
	REQUIRE(setSwatchesResult.success);
	context->Update();
	Rml::Element* boundSwatches = document->GetElementById("bound-swatches");
	REQUIRE(boundSwatches != nullptr);
	REQUIRE(boundSwatches->GetNumChildren() == 4);
	CHECK(boundSwatches->GetChild(0)->GetChild(1)->GetInnerRML() == "Player");
	CHECK(boundSwatches->GetChild(0)->GetChild(0)->GetProperty<Rml::Colourb>("background-color") == Rml::Colourb(64, 128, 255, 255));
	CHECK(boundSwatches->GetChild(0)->GetChild(2) != nullptr);
	CHECK_FALSE(boundSwatches->GetChild(1)->GetChild(2)->IsVisible());

	RmlDataGridRow gridRows[] = {
		{.label = "Diffuse", .image = "textures/diffuse.png", .cellSize = 96.0f, .hasImage = true, .nativeImage = false, .selected = true, .folder = false, .filler = false},
		{.label = "Brushes", .image = "$native_texture", .cellSize = 96.0f, .hasImage = true, .nativeImage = true, .selected = false, .folder = true, .filler = false},
		{.label = "", .image = "", .cellSize = 96.0f, .hasImage = false, .nativeImage = false, .selected = false, .folder = false, .filler = true},
	};
	RmlDataModelSetGridRowsQuery setGrid = {
		.rowsHandle = gridResult.rowsHandle,
		.rows = gridRows,
		.count = 3,
	};
	RmlElementBoolResult setGridResult = {};
	RMLUI_API.DataModelSetGridRows(&setGrid, &setGridResult);
	REQUIRE(setGridResult.error == nullptr);
	REQUIRE(setGridResult.success);
	context->Update();
	Rml::Element* boundGrid = document->GetElementById("bound-grid");
	REQUIRE(boundGrid != nullptr);
	REQUIRE(boundGrid->GetNumChildren() == 4);
	CHECK(boundGrid->GetChild(0)->IsClassSet("selected"));
	CHECK_FALSE(boundGrid->GetChild(0)->IsClassSet("folder"));
	CHECK(boundGrid->GetChild(0)->GetProperty<float>("flex-basis") == 96.0f);
	CHECK(boundGrid->GetChild(0)->GetChild(0)->GetAttribute("src", Rml::String{}) == "textures/diffuse.png");
	CHECK(boundGrid->GetChild(0)->GetChild(0)->IsVisible());
	CHECK_FALSE(boundGrid->GetChild(0)->GetChild(1)->IsVisible());
	CHECK_FALSE(boundGrid->GetChild(1)->IsClassSet("selected"));
	CHECK(boundGrid->GetChild(2)->IsClassSet("grid-filler"));
	CHECK(boundGrid->GetChild(1)->IsClassSet("folder"));
	CHECK_FALSE(boundGrid->GetChild(1)->GetChild(0)->IsVisible());
	CHECK(boundGrid->GetChild(1)->GetChild(1)->IsVisible());

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
	document->SetInnerRML(R"(
		<div data-model="runtime">
			<div id="bound-rows"><div data-for="row : rows" data-if="row.visible" data-class-selected="row.selected">{{ row.label }}</div></div>
			<div id="bound-styled"><div data-for="row : styled" data-if="row.visible"><span data-style-background-color="row.colour" data-style-left="row.offset" data-style-width="row.progress"></span></div></div>
		</div>
	)");
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
	Rml::Element* boundStyled = document->GetElementById("bound-styled");
	REQUIRE(boundRows != nullptr);
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
