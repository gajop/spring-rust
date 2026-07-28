/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include <catch_amalgamated.hpp>

#include "NativeInterface/api/RmlUi.h"

#include <RmlUi/Core.h>
#include <RmlUi/Core/Context.h>
#include <RmlUi/Core/Element.h>
#include <RmlUi/Core/ElementDocument.h>
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
	Rml::TextureHandle LoadTexture(Rml::Vector2i& dimensions, const Rml::String&) override { dimensions = {}; return {}; }
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

void InitialiseRml(NullRenderInterface& renderInterface)
{
	Rml::SetSystemInterface(&silentSystemInterface);
	Rml::SetRenderInterface(&renderInterface);
	REQUIRE(Rml::Initialise());
}

uint64_t CreateModel(Rml::Context* context, const char* name)
{
	RmlContextCreateDataModelQuery query = {.contextHandle = ToHandle(context), .name = name};
	RmlContextOpenDataModelResult result = {};
	RMLUI_API.ContextCreateDataModel(&query, &result);
	REQUIRE(result.error == nullptr);
	REQUIRE(result.success);
	return result.dataModelHandle;
}

uint64_t BindString(uint64_t modelHandle, const char* name)
{
	RmlDataModelBindStringQuery query = {
		.dataModelHandle = modelHandle,
		.name = name,
		.initialValue = "",
	};
	RmlDataModelBindResult result = {};
	RMLUI_API.DataModelBindString(&query, &result);
	REQUIRE(result.error == nullptr);
	REQUIRE(result.success);
	return result.variableHandle;
}

void SetString(uint64_t variableHandle, const std::string& value)
{
	RmlDataModelVariableStringQuery query = {.variableHandle = variableHandle, .value = value.c_str()};
	RmlElementBoolResult result = {};
	RMLUI_API.DataModelSetString(&query, &result);
	REQUIRE(result.error == nullptr);
	REQUIRE(result.success);
}

void CloseRml(Rml::Context* context)
{
	NativeRmlUi::ClearAllContexts(nullptr);
	REQUIRE(Rml::RemoveContext(context->GetName()));
	Rml::Shutdown();
}

Rml::String MetricMarkup(const std::array<std::string, 7>& values, size_t first, size_t count, unsigned revision)
{
	Rml::String markup;
	for (size_t offset = 0; offset < count; ++offset) {
		const size_t index = first + offset;
		markup += "<span class=\"status-metric ";
		markup += ((index + revision) % 3 == 0) ? "warning\">" : "normal\">";
		markup += "<span class=\"metric-label\">metric </span><span class=\"metric-value\">";
		markup += values[index];
		markup += "</span></span>";
	}
	return markup;
}

Rml::String NotificationMarkup(unsigned revision)
{
	Rml::String markup;
	for (size_t index = 0; index < 3; ++index) {
		const bool warning = (index + revision) % 3 == 0;
		const bool progress = (index + revision) % 2 == 0;
		markup += "<div class=\"notification\"><div class=\"notification-title";
		markup += warning ? " warning\">" : "\">";
		markup += "Notification ";
		markup += std::to_string(index);
		markup += " revision ";
		markup += std::to_string(revision);
		markup += "</div><div class=\"notification-body\">Nested native notification content ";
		markup += std::to_string(index);
		markup += "</div>";
		if (progress) {
			markup += "<div class=\"notification-progress\"><div class=\"notification-progress-fill\" style=\"width: ";
			markup += std::to_string(25 + static_cast<int>(index) * 20 + static_cast<int>(revision));
			markup += "%\"></div></div>";
		}
		markup += "</div>";
	}
	return markup;
}

struct NotificationRows {
	std::vector<std::string> text;
	std::vector<RmlDataNotificationRow> rows;

	NotificationRows(unsigned revision)
	{
		text.reserve(6);
		rows.reserve(3);
		for (size_t index = 0; index < 3; ++index) {
			const bool warning = (index + revision) % 3 == 0;
			const bool hasProgress = (index + revision) % 2 == 0;
			text.emplace_back("Notification " + std::to_string(index) + " revision " + std::to_string(revision));
			text.emplace_back("Nested native notification content " + std::to_string(index));
			rows.push_back({
				.title = text[text.size() - 2].c_str(),
				.body = text.back().c_str(),
				.warning = warning,
				.hasProgress = hasProgress,
				.progress = hasProgress ? 25.0f + static_cast<float>(index) * 20.0f + static_cast<float>(revision) : 0.0f,
			});
		}
	}
};

void SetNotificationRows(uint64_t rowsHandle, const NotificationRows& rows)
{
	RmlDataModelSetNotificationRowsQuery query = {
		.rowsHandle = rowsHandle,
		.rows = rows.rows.data(),
		.count = rows.rows.size(),
	};
	RmlElementBoolResult result = {};
	RMLUI_API.DataModelSetNotificationRows(&query, &result);
	REQUIRE(result.error == nullptr);
	REQUIRE(result.success);
}

void SyncNotificationStyles(Rml::Element* root, const NotificationRows& rows)
{
	for (size_t index = 0; index < rows.rows.size(); ++index) {
		Rml::Element* row = root->GetChild(index);
		REQUIRE(row != nullptr);
		Rml::Element* title = row->GetChild(0);
		Rml::Element* progress = row->GetChild(2);
		REQUIRE(title != nullptr);
		REQUIRE(progress != nullptr);
		title->SetClass("warning", rows.rows[index].warning);
		progress->SetClass("hidden", !rows.rows[index].hasProgress);
		if (rows.rows[index].hasProgress) {
			Rml::Element* fill = progress->GetChild(0);
			REQUIRE(fill != nullptr);
			fill->SetAttribute("style", "width: " + std::to_string(static_cast<int>(rows.rows[index].progress)) + "%;");
		}
	}
}
}

TEST_CASE("Native scalar bindings update the fixed DevConsole status structure", "[.][benchmark]")
{
	NullRenderInterface renderInterface;
	InitialiseRml(renderInterface);
	Rml::Context* context = Rml::CreateContext("native-status-benchmark", {1024, 768});
	REQUIRE(context != nullptr);

	const std::array<std::string, 7> firstMetrics = {"60 fps", "4%", "12%", "512 MiB", "2 / 8 GiB", "5 / 16 GiB", "700 MiB"};
	const std::array<std::string, 7> secondMetrics = {"42 fps", "9%", "75%", "640 MiB", "6 / 8 GiB", "13 / 16 GiB", "900 MiB"};
	const std::string firstPosition = "X: 128, Y: 14, Z: 256";
	const std::string secondPosition = "X: 512, Y: 22, Z: 768";
	const std::string firstVersion = "SpringBoard 1.0";
	const std::string secondVersion = "SpringBoard 1.1";

	Rml::ElementDocument* innerDocument = context->LoadDocumentFromMemory(
		"<rml><body><div id=\"position\"></div><div id=\"performance\"></div><div id=\"system\"></div><div id=\"version\"></div></body></rml>",
		"status-inner-rml-benchmark.rml"
	);
	REQUIRE(innerDocument != nullptr);
	innerDocument->Show();
	Rml::Element* position = innerDocument->GetElementById("position");
	Rml::Element* performance = innerDocument->GetElementById("performance");
	Rml::Element* system = innerDocument->GetElementById("system");
	Rml::Element* version = innerDocument->GetElementById("version");
	REQUIRE(position != nullptr);
	REQUIRE(performance != nullptr);
	REQUIRE(system != nullptr);
	REQUIRE(version != nullptr);

	const uint64_t model = CreateModel(context, "native_status");
	const std::array<const char*, 9> fieldNames = {"position", "fps", "process_cpu", "system_cpu", "lua_memory", "vram", "ram", "process_memory", "version"};
	std::array<uint64_t, 9> fields = {};
	for (size_t index = 0; index < fields.size(); ++index)
		fields[index] = BindString(model, fieldNames[index]);
	Rml::ElementDocument* boundDocument = context->LoadDocumentFromMemory(
		"<rml><body data-model=\"native_status\"><div>{{ position }}</div><div><span id=\"metric-0\" class=\"status-metric normal\">{{ fps }}</span><span id=\"metric-1\" class=\"status-metric normal\">{{ process_cpu }}</span><span id=\"metric-2\" class=\"status-metric normal\">{{ system_cpu }}</span></div><div><span id=\"metric-3\" class=\"status-metric normal\">{{ lua_memory }}</span><span id=\"metric-4\" class=\"status-metric normal\">{{ vram }}</span><span id=\"metric-5\" class=\"status-metric normal\">{{ ram }}</span><span id=\"metric-6\" class=\"status-metric normal\">{{ process_memory }}</span></div><div>{{ version }}</div></body></rml>",
		"status-bound-benchmark.rml"
	);
	REQUIRE(boundDocument != nullptr);
	boundDocument->Show();
	context->Update();
	std::array<Rml::Element*, 7> metricElements = {};
	for (size_t index = 0; index < metricElements.size(); ++index) {
		metricElements[index] = boundDocument->GetElementById("metric-" + std::to_string(index));
		REQUIRE(metricElements[index] != nullptr);
	}

	bool secondInner = true;
	BENCHMARK("SetInnerRML status + Update")
	{
		const auto& metrics = secondInner ? secondMetrics : firstMetrics;
		position->SetInnerRML(secondInner ? secondPosition : firstPosition);
		performance->SetInnerRML(MetricMarkup(metrics, 0, 3, secondInner ? 2 : 1));
		system->SetInnerRML(MetricMarkup(metrics, 3, 4, secondInner ? 2 : 1));
		version->SetInnerRML(secondInner ? secondVersion : firstVersion);
		secondInner = !secondInner;
		context->Update();
	};

	bool secondBound = true;
	BENCHMARK("typed status fields + Update")
	{
		const auto& metrics = secondBound ? secondMetrics : firstMetrics;
		SetString(fields[0], secondBound ? secondPosition : firstPosition);
		for (size_t index = 0; index < metrics.size(); ++index) {
			SetString(fields[index + 1], metrics[index]);
			const bool warning = (index + (secondBound ? 2 : 1)) % 3 == 0;
			metricElements[index]->SetClass("normal", !warning);
			metricElements[index]->SetClass("healthy", false);
			metricElements[index]->SetClass("warning", warning);
			metricElements[index]->SetClass("critical", false);
		}
		SetString(fields[8], secondBound ? secondVersion : firstVersion);
		secondBound = !secondBound;
		context->Update();
	};

	CloseRml(context);
}

TEST_CASE("Native notification rows update nested cards with their required style sync", "[.][benchmark]")
{
	NullRenderInterface renderInterface;
	InitialiseRml(renderInterface);
	Rml::Context* context = Rml::CreateContext("native-notification-benchmark", {1024, 768});
	REQUIRE(context != nullptr);
	const NotificationRows firstRows(1);
	const NotificationRows secondRows(2);

	Rml::ElementDocument* innerDocument = context->LoadDocumentFromMemory(
		"<rml><body><div id=\"inner-notifications\"></div></body></rml>",
		"notification-inner-rml-benchmark.rml"
	);
	REQUIRE(innerDocument != nullptr);
	innerDocument->Show();
	Rml::Element* innerRoot = innerDocument->GetElementById("inner-notifications");
	REQUIRE(innerRoot != nullptr);

	const uint64_t model = CreateModel(context, "native_notifications");
	RmlDataModelBindNotificationRowsQuery bindQuery = {.dataModelHandle = model, .name = "notifications"};
	RmlDataModelNotificationRowsResult bindResult = {};
	RMLUI_API.DataModelBindNotificationRows(&bindQuery, &bindResult);
	REQUIRE(bindResult.error == nullptr);
	REQUIRE(bindResult.success);
	Rml::ElementDocument* boundDocument = context->LoadDocumentFromMemory(
		"<rml><body data-model=\"native_notifications\"><div id=\"bound-notifications\"><div data-for=\"notification : notifications\" class=\"notification\"><div class=\"notification-title\">{{ notification.title }}</div><div class=\"notification-body\">{{ notification.body }}</div><div class=\"notification-progress\"><div class=\"notification-progress-fill\"></div></div></div></div></body></rml>",
		"notification-bound-benchmark.rml"
	);
	REQUIRE(boundDocument != nullptr);
	boundDocument->Show();
	SetNotificationRows(bindResult.rowsHandle, firstRows);
	context->Update();
	Rml::Element* boundRoot = boundDocument->GetElementById("bound-notifications");
	REQUIRE(boundRoot != nullptr);
	REQUIRE(boundRoot->GetNumChildren() == firstRows.rows.size() + 1);
	SyncNotificationStyles(boundRoot, firstRows);

	bool secondInner = true;
	BENCHMARK("SetInnerRML notifications + Update")
	{
		innerRoot->SetInnerRML(NotificationMarkup(secondInner ? 2 : 1));
		secondInner = !secondInner;
		context->Update();
	};

	bool secondBound = true;
	BENCHMARK("typed notifications + Update + style sync")
	{
		const NotificationRows& rows = secondBound ? secondRows : firstRows;
		SetNotificationRows(bindResult.rowsHandle, rows);
		secondBound = !secondBound;
		context->Update();
		SyncNotificationStyles(boundRoot, rows);
	};

	CloseRml(context);
}
