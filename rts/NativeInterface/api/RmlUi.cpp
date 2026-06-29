#include "RmlUi.h"

#include "Rml/Backends/RmlUi_Backend.h"

#include <RmlUi/Core.h>
#include <RmlUi/Core/Context.h>
#include <RmlUi/Core/ElementDocument.h>
#include <RmlUi/Core/Event.h>
#include <RmlUi/Core/EventListener.h>
#include <RmlUi/Core/Factory.h>
#include <RmlUi/Core/Elements/ElementForm.h>
#include <RmlUi/Core/Elements/ElementFormControlInput.h>
#include <RmlUi/Core/Elements/ElementFormControlSelect.h>
#include <RmlUi/Core/Elements/ElementFormControlTextArea.h>
#include <RmlUi/Core/Elements/ElementTabSet.h>

#include <cstdint>
#include <memory>
#include <sstream>
#include <unordered_map>
#include <vector>

namespace {

static const Error INVALID_ARGUMENT_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid argument" };
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "RmlUi is not ready" };

template <typename T>
static uint64_t ToHandle(T* pointer)
{
	return reinterpret_cast<uintptr_t>(pointer);
}

template <typename T>
static T* FromHandle(uint64_t handle)
{
	return reinterpret_cast<T*>(static_cast<uintptr_t>(handle));
}

static Rml::Context* FromHandle(uint64_t handle)
{
	return reinterpret_cast<Rml::Context*>(static_cast<uintptr_t>(handle));
}

static uint64_t nextElementPtrHandle = 1;
static uint64_t nextDataModelHandle = 1;
static std::unordered_map<uint64_t, Rml::ElementPtr> ownedElementPtrs;
static std::unordered_map<uint64_t, Rml::DataModelHandle> nativeDataModels;
static thread_local std::vector<uint64_t> elementHandleResults;
static thread_local std::vector<Rml::String> stringResults;
static thread_local std::vector<const char*> stringPtrResults;
static thread_local Rml::Event* currentEvent = nullptr;
static thread_local Rml::Element* currentEventElement = nullptr;
static thread_local Rml::ElementDocument* currentEventDocument = nullptr;

static uint64_t StoreElementPtr(Rml::ElementPtr&& element)
{
	if (!element)
		return 0;

	const uint64_t handle = nextElementPtrHandle++;
	ownedElementPtrs.emplace(handle, std::move(element));
	return handle;
}

static Rml::ElementPtr TakeElementPtr(uint64_t handle)
{
	auto it = ownedElementPtrs.find(handle);
	if (it == ownedElementPtrs.end())
		return nullptr;

	Rml::ElementPtr element = std::move(it->second);
	ownedElementPtrs.erase(it);
	return element;
}

static Rml::ElementDocument* FromDocumentHandle(uint64_t handle) { return FromHandle<Rml::ElementDocument>(handle); }
static Rml::Element* FromElementHandle(uint64_t handle) { return FromHandle<Rml::Element>(handle); }
static Rml::Event* FromEventHandle(uint64_t handle) { return FromHandle<Rml::Event>(handle); }
static Rml::EventListener* FromEventListenerHandle(uint64_t handle) { return FromHandle<Rml::EventListener>(handle); }

static const Rml::Variant* GetEventParameter(Rml::Event* event, const char* name)
{
	if (event == nullptr || name == nullptr)
		return nullptr;

	const auto& parameters = event->GetParameters();
	auto it = parameters.find(name);
	if (it == parameters.end())
		return nullptr;

	return &it->second;
}

class NativeRmlEventListener : public Rml::EventListener
{
public:
	NativeRmlEventListener(NativeCallback callback, void* userData, Rml::Element* element)
		: callback(callback)
		, userData(userData)
		, element(element)
	{}

	void OnAttach(Rml::Element* attachedElement) override
	{
		element = attachedElement;
	}

	void OnDetach(Rml::Element*) override
	{
		delete this;
	}

	void ProcessEvent(Rml::Event& event) override
	{
		Rml::Event* previousEvent = currentEvent;
		Rml::Element* previousElement = currentEventElement;
		Rml::ElementDocument* previousDocument = currentEventDocument;

		currentEvent = &event;
		currentEventElement = element;
		currentEventDocument = element != nullptr ? element->GetOwnerDocument() : nullptr;
		if (callback != nullptr) {
			callback(userData);
		}

		currentEvent = previousEvent;
		currentEventElement = previousElement;
		currentEventDocument = previousDocument;
	}

private:
	NativeCallback callback = nullptr;
	void* userData = nullptr;
	Rml::Element* element = nullptr;
};

static Rml::StringList SplitStringList(const char* values)
{
	Rml::StringList result;
	if (values == nullptr)
		return result;

	std::stringstream stream(values);
	Rml::String token;
	while (stream >> token) {
		result.push_back(token);
	}
	return result;
}

static bool IsReady()
{
	return RmlGui::IsInitialized();
}

static void NativeCreateContext(const RmlCreateContextQuery* query, RmlCreateContextResult* result)
{
	result->error = nullptr;
	result->contextHandle = 0;
	result->success = false;

	if (query->name == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	Rml::Context* context = RmlGui::GetOrCreateContext(query->name);
	result->contextHandle = ToHandle(context);
	result->success = (context != nullptr);
	if (!result->success) {
		result->error = &NOT_READY_ERROR;
	}
}

static void NativeGetContext(const RmlGetContextQuery* query, RmlGetContextResult* result)
{
	result->error = nullptr;
	result->contextHandle = 0;
	result->exists = false;

	if (query->name == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	Rml::Context* context = RmlGui::GetContext(query->name);
	result->contextHandle = ToHandle(context);
	result->exists = (context != nullptr);
}

static void NativeRemoveContext(const RmlRemoveContextQuery* query, RmlRemoveContextResult* result)
{
	result->error = nullptr;
	Rml::Context* context = FromHandle(query->contextHandle);
	result->success = (context != nullptr);
	if (context != nullptr) {
		RmlGui::MarkContextForRemoval(context);
	}
}

static void NativeRemoveContextByName(const RmlRemoveContextByNameQuery* query, RmlRemoveContextByNameResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (query->name == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	Rml::Context* context = RmlGui::GetContext(query->name);
	result->success = (context != nullptr);
	if (context != nullptr) {
		RmlGui::MarkContextForRemoval(context);
	}
}

static void NativeSetDebugContext(const RmlSetDebugContextQuery* query, RmlSetDebugContextResult* result)
{
	result->error = nullptr;
	Rml::Context* context = FromHandle(query->contextHandle);
	result->success = (context != nullptr);
	RmlGui::SetDebugContext(context);
}

static void NativeSetDebugContextByName(const RmlSetDebugContextByNameQuery* query, RmlSetDebugContextByNameResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (query->name == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	Rml::Context* context = RmlGui::GetContext(query->name);
	result->success = (context != nullptr);
	RmlGui::SetDebugContext(context);
}

static void NativeLoadFontFace(const RmlLoadFontFaceQuery* query, RmlLoadFontFaceResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (query->filePath == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	if (query->hasWeight) {
		result->success = Rml::LoadFontFace(query->filePath, query->fallback, static_cast<Rml::Style::FontWeight>(query->weight));
	} else {
		result->success = Rml::LoadFontFace(query->filePath, query->fallback);
	}
}

static void NativeRegisterEventType(const RmlRegisterEventTypeQuery* query, RmlRegisterEventTypeResult* result)
{
	result->error = nullptr;
	result->eventID = 0;

	if (query->eventType == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const auto phase = query->hasDefaultPhase ? static_cast<Rml::DefaultActionPhase>(query->defaultPhase) : Rml::DefaultActionPhase::None;
	result->eventID = static_cast<int32_t>(Rml::RegisterEventType(query->eventType, query->interruptible, query->bubbles, phase));
}

static void NativeAddTranslationString(const RmlAddTranslationStringQuery* query, RmlAddTranslationStringResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (query->key == nullptr || query->translation == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	result->success = RmlGui::AddTranslationString(query->key, query->translation);
}

static void NativeClearTranslations(const RmlClearTranslationsQuery*, RmlClearTranslationsResult* result)
{
	result->error = nullptr;
	result->success = RmlGui::ClearTranslations();
}

static void NativeSetMouseCursorAlias(const RmlSetMouseCursorAliasQuery* query, RmlSetMouseCursorAliasResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (query->rmlName == nullptr || query->recoilName == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	RmlGui::SetMouseCursorAlias(query->rmlName, query->recoilName);
	result->success = true;
}

static void NativeGetVersion(const RmlGetVersionQuery*, RmlGetVersionResult* result)
{
	static thread_local Rml::String version;
	version = Rml::GetVersion();

	result->error = nullptr;
	result->version = version.c_str();
}

static void NativeIsReady(const RmlIsReadyQuery*, RmlIsReadyResult* result)
{
	result->error = nullptr;
	result->ready = IsReady();
}

static void NativeContextCreateDocument(const RmlContextCreateDocumentQuery* query, RmlContextCreateDocumentResult* result)
{
	result->error = nullptr;
	result->documentHandle = 0;
	result->success = false;

	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	Rml::ElementDocument* document = (query->tag != nullptr) ? context->CreateDocument(query->tag) : context->CreateDocument();
	result->documentHandle = ToHandle(document);
	result->success = (document != nullptr);
}

static void NativeContextLoadDocument(const RmlContextLoadDocumentQuery* query, RmlContextLoadDocumentResult* result)
{
	result->error = nullptr;
	result->documentHandle = 0;
	result->success = false;

	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr || query->documentPath == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	Rml::ElementDocument* document = context->LoadDocument(query->documentPath);
	result->documentHandle = ToHandle(document);
	result->success = (document != nullptr);
}

static void NativeContextGetDocument(const RmlContextGetDocumentQuery* query, RmlContextGetDocumentResult* result)
{
	result->error = nullptr;
	result->documentHandle = 0;
	result->exists = false;

	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr || query->name == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	Rml::ElementDocument* document = context->GetDocument(query->name);
	result->documentHandle = ToHandle(document);
	result->exists = (document != nullptr);
}

static void NativeContextAddEventListener(const RmlContextEventListenerCallbackQuery* query, RmlEventListenerCallbackResult* result)
{
	result->error = nullptr;
	result->eventListenerHandle = 0;
	result->success = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr || query->event == nullptr || query->callback == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	auto* listener = new NativeRmlEventListener(query->callback, query->userData, context->GetRootElement());
	context->AddEventListener(query->event, listener, query->inCapturePhase);
	result->eventListenerHandle = ToHandle(listener);
	result->success = true;
}

static void NativeContextRender(const RmlContextHandleQuery* query, RmlContextBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->success = context->Render();
}

static void NativeContextUnloadAllDocuments(const RmlContextHandleQuery* query, RmlContextBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	context->UnloadAllDocuments();
	result->success = true;
}

static void NativeContextUnloadDocument(const RmlContextDocumentQuery* query, RmlContextBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	Rml::ElementDocument* document = FromDocumentHandle(query->documentHandle);
	if (context == nullptr || document == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	context->UnloadDocument(document);
	result->success = true;
}

static void NativeContextUpdate(const RmlContextHandleQuery* query, RmlContextBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->success = context->Update();
}

static void NativeContextRemoveDataModel(const RmlContextStringQuery* query, RmlContextBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr || query->name == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->success = context->RemoveDataModel(query->name);
}

static void NativeContextOpenDataModel(const RmlContextOpenDataModelQuery* query, RmlContextOpenDataModelResult* result)
{
	result->error = nullptr;
	result->dataModelHandle = 0;
	result->success = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr || query->name == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	Rml::DataModelConstructor constructor = context->CreateDataModel(query->name);
	if (!constructor) {
		constructor = context->GetDataModel(query->name);
	}
	if (!constructor) {
		return;
	}

	const uint64_t handle = nextDataModelHandle++;
	nativeDataModels.emplace(handle, constructor.GetModelHandle());
	result->dataModelHandle = handle;
	result->success = true;
}

static void NativeContextProcessMouseMove(const RmlContextMouseMoveQuery* query, RmlContextBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->success = context->ProcessMouseMove(static_cast<int>(query->x), static_cast<int>(query->y), query->keyModifierState);
}

static void NativeContextProcessMouseButtonDown(const RmlContextMouseButtonQuery* query, RmlContextBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->success = context->ProcessMouseButtonDown(query->button, query->keyModifierState);
}

static void NativeContextProcessMouseButtonUp(const RmlContextMouseButtonQuery* query, RmlContextBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->success = context->ProcessMouseButtonUp(query->button, query->keyModifierState);
}

static void NativeContextProcessMouseWheel(const RmlContextMouseWheelQuery* query, RmlContextBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->success = context->ProcessMouseWheel(Rml::Vector2f(query->x, query->y), query->keyModifierState);
}

static void NativeContextProcessMouseLeave(const RmlContextHandleQuery* query, RmlContextBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->success = context->ProcessMouseLeave();
}

static void NativeContextIsMouseInteracting(const RmlContextHandleQuery* query, RmlContextBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->success = context->IsMouseInteracting();
}

static void NativeContextProcessKeyDown(const RmlContextKeyQuery* query, RmlContextBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->success = context->ProcessKeyDown(static_cast<Rml::Input::KeyIdentifier>(query->key), query->keyModifierState);
}

static void NativeContextProcessKeyUp(const RmlContextKeyQuery* query, RmlContextBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->success = context->ProcessKeyUp(static_cast<Rml::Input::KeyIdentifier>(query->key), query->keyModifierState);
}

static void NativeContextProcessTextInput(const RmlContextTextInputQuery* query, RmlContextBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr || query->text == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->success = context->ProcessTextInput(query->text);
}

static void NativeContextEnableMouseCursor(const RmlContextBoolQuery* query, RmlContextBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	context->EnableMouseCursor(query->value);
	result->success = true;
}

static void NativeContextActivateTheme(const RmlContextStringBoolQuery* query, RmlContextBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr || query->name == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	context->ActivateTheme(query->name, query->value);
	result->success = true;
}

static void NativeContextIsThemeActive(const RmlContextStringQuery* query, RmlContextBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr || query->name == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->success = context->IsThemeActive(query->name);
}

static void NativeContextGetElementAtPoint(const RmlContextGetElementAtPointQuery* query, RmlContextGetElementAtPointResult* result)
{
	result->error = nullptr;
	result->elementHandle = 0;
	result->exists = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	Rml::Element* element = context->GetElementAtPoint(Rml::Vector2f(query->x, query->y), FromElementHandle(query->ignoreElementHandle));
	result->elementHandle = ToHandle(element);
	result->exists = (element != nullptr);
}

static void NativeContextPullDocumentToFront(const RmlContextDocumentQuery* query, RmlContextBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	Rml::ElementDocument* document = FromDocumentHandle(query->documentHandle);
	if (context == nullptr || document == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	context->PullDocumentToFront(document);
	result->success = true;
}

static void NativeContextPushDocumentToBack(const RmlContextDocumentQuery* query, RmlContextBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	Rml::ElementDocument* document = FromDocumentHandle(query->documentHandle);
	if (context == nullptr || document == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	context->PushDocumentToBack(document);
	result->success = true;
}

static void NativeContextGetName(const RmlContextHandleQuery* query, RmlContextGetNameResult* result)
{
	result->error = nullptr;
	result->name = nullptr;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->name = context->GetName().c_str();
}

static void NativeContextGetDimensions(const RmlContextHandleQuery* query, RmlContextGetDimensionsResult* result)
{
	result->error = nullptr;
	result->x = 0;
	result->y = 0;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	const auto dims = context->GetDimensions();
	result->x = dims.x;
	result->y = dims.y;
}

static void NativeContextSetDimensions(const RmlContextSetDimensionsQuery* query, RmlContextBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	context->SetDimensions(Rml::Vector2i(query->x, query->y));
	result->success = true;
}

static void NativeContextGetDensityIndependentPixelRatio(const RmlContextHandleQuery* query, RmlContextGetFloatResult* result)
{
	result->error = nullptr;
	result->value = 0.0f;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->value = context->GetDensityIndependentPixelRatio();
}

static void NativeContextSetDensityIndependentPixelRatio(const RmlContextSetFloatQuery* query, RmlContextBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	context->SetDensityIndependentPixelRatio(query->value);
	result->success = true;
}

static void FillContextElementResult(Rml::Element* element, RmlContextGetElementResult* result)
{
	result->error = nullptr;
	result->elementHandle = ToHandle(element);
	result->exists = (element != nullptr);
}

static void NativeContextGetFocusElement(const RmlContextHandleQuery* query, RmlContextGetElementResult* result)
{
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		result->elementHandle = 0;
		result->exists = false;
		return;
	}
	FillContextElementResult(context->GetFocusElement(), result);
}

static void NativeContextGetHoverElement(const RmlContextHandleQuery* query, RmlContextGetElementResult* result)
{
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		result->elementHandle = 0;
		result->exists = false;
		return;
	}
	FillContextElementResult(context->GetHoverElement(), result);
}

static void NativeContextGetRootElement(const RmlContextHandleQuery* query, RmlContextGetElementResult* result)
{
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		result->elementHandle = 0;
		result->exists = false;
		return;
	}
	FillContextElementResult(context->GetRootElement(), result);
}

static void NativeDocumentPullToFront(const RmlDocumentHandleQuery* query, RmlDocumentBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::ElementDocument* document = FromDocumentHandle(query->documentHandle);
	if (document == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	document->PullToFront();
	result->success = true;
}

static void NativeDocumentPushToBack(const RmlDocumentHandleQuery* query, RmlDocumentBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::ElementDocument* document = FromDocumentHandle(query->documentHandle);
	if (document == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	document->PushToBack();
	result->success = true;
}

static void NativeDocumentShow(const RmlDocumentShowQuery* query, RmlDocumentBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::ElementDocument* document = FromDocumentHandle(query->documentHandle);
	if (document == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	const auto modal = query->hasModal ? static_cast<Rml::ModalFlag>(query->modal) : Rml::ModalFlag::None;
	const auto focus = query->hasFocus ? static_cast<Rml::FocusFlag>(query->focus) : Rml::FocusFlag::Auto;
	document->Show(modal, focus);
	result->success = true;
}

static void NativeDocumentHide(const RmlDocumentHandleQuery* query, RmlDocumentBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::ElementDocument* document = FromDocumentHandle(query->documentHandle);
	if (document == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	document->Hide();
	result->success = true;
}

static void NativeDocumentClose(const RmlDocumentHandleQuery* query, RmlDocumentBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::ElementDocument* document = FromDocumentHandle(query->documentHandle);
	if (document == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	document->Close();
	result->success = true;
}

static void NativeDocumentCreateElement(const RmlDocumentCreateElementQuery* query, RmlDocumentCreateElementResult* result)
{
	result->error = nullptr;
	result->elementPtrHandle = 0;
	result->success = false;
	Rml::ElementDocument* document = FromDocumentHandle(query->documentHandle);
	if (document == nullptr || query->tagName == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->elementPtrHandle = StoreElementPtr(document->CreateElement(query->tagName));
	result->success = (result->elementPtrHandle != 0);
}

static void NativeDocumentCreateTextNode(const RmlDocumentStringQuery* query, RmlDocumentCreateElementResult* result)
{
	result->error = nullptr;
	result->elementPtrHandle = 0;
	result->success = false;
	Rml::ElementDocument* document = FromDocumentHandle(query->documentHandle);
	if (document == nullptr || query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->elementPtrHandle = StoreElementPtr(document->CreateTextNode(query->value));
	result->success = (result->elementPtrHandle != 0);
}

static void NativeDocumentReloadStyleSheet(const RmlDocumentHandleQuery* query, RmlDocumentBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::ElementDocument* document = FromDocumentHandle(query->documentHandle);
	if (document == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	document->ReloadStyleSheet();
	result->success = true;
}

static void NativeDocumentLoadInlineScript(const RmlDocumentInlineScriptQuery* query, RmlDocumentBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::ElementDocument* document = FromDocumentHandle(query->documentHandle);
	if (document == nullptr || query->content == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	const char* sourcePath = query->sourcePath != nullptr ? query->sourcePath : document->GetSourceURL().c_str();
	document->LoadInlineScript(query->content, sourcePath, query->sourceLine);
	result->success = true;
}

static void NativeDocumentLoadExternalScript(const RmlDocumentStringQuery* query, RmlDocumentBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::ElementDocument* document = FromDocumentHandle(query->documentHandle);
	if (document == nullptr || query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	document->LoadExternalScript(query->value);
	result->success = true;
}

static void NativeDocumentUpdateDocument(const RmlDocumentHandleQuery* query, RmlDocumentBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::ElementDocument* document = FromDocumentHandle(query->documentHandle);
	if (document == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	document->UpdateDocument();
	result->success = true;
}

static void NativeDocumentAppendToStyleSheet(const RmlDocumentStringQuery* query, RmlDocumentBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::ElementDocument* document = FromDocumentHandle(query->documentHandle);
	if (document == nullptr || query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	auto styleSheet = Rml::Factory::InstanceStyleSheetString(query->value);
	auto combined = styleSheet->CombineStyleSheetContainer(*document->GetStyleSheetContainer());
	document->SetStyleSheetContainer(std::move(combined));
	result->success = true;
}

static void NativeDocumentGetTitle(const RmlDocumentHandleQuery* query, RmlDocumentGetStringResult* result)
{
	result->error = nullptr;
	result->value = nullptr;
	Rml::ElementDocument* document = FromDocumentHandle(query->documentHandle);
	if (document == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->value = document->GetTitle().c_str();
}

static void NativeDocumentSetTitle(const RmlDocumentSetTitleQuery* query, RmlDocumentBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::ElementDocument* document = FromDocumentHandle(query->documentHandle);
	if (document == nullptr || query->title == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	document->SetTitle(query->title);
	result->success = true;
}

static void NativeDocumentGetContext(const RmlDocumentHandleQuery* query, RmlDocumentGetContextResult* result)
{
	result->error = nullptr;
	result->contextHandle = 0;
	result->exists = false;
	Rml::ElementDocument* document = FromDocumentHandle(query->documentHandle);
	if (document == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	Rml::Context* context = document->GetContext();
	result->contextHandle = ToHandle(context);
	result->exists = (context != nullptr);
}

static void NativeDocumentGetUrl(const RmlDocumentHandleQuery* query, RmlDocumentGetStringResult* result)
{
	result->error = nullptr;
	result->value = nullptr;
	Rml::ElementDocument* document = FromDocumentHandle(query->documentHandle);
	if (document == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->value = document->GetSourceURL().c_str();
}

static void NativeDocumentIsModal(const RmlDocumentHandleQuery* query, RmlDocumentBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::ElementDocument* document = FromDocumentHandle(query->documentHandle);
	if (document == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->success = document->IsModal();
}

static void FillElementResult(Rml::Element* element, RmlElementGetElementResult* result)
{
	result->error = nullptr;
	result->elementHandle = ToHandle(element);
	result->exists = (element != nullptr);
}

static void NativeElementAppendChild(const RmlElementAppendChildQuery* query, RmlElementGetElementResult* result)
{
	Rml::Element* element = FromElementHandle(query->elementHandle);
	Rml::ElementPtr child = TakeElementPtr(query->elementPtrHandle);
	if (element == nullptr || child == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		result->elementHandle = 0;
		result->exists = false;
		return;
	}
	FillElementResult(element->AppendChild(std::move(child)), result);
}

static void NativeElementAddEventListener(const RmlEventListenerCallbackQuery* query, RmlEventListenerCallbackResult* result)
{
	result->error = nullptr;
	result->eventListenerHandle = 0;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr || query->event == nullptr || query->callback == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	auto* listener = new NativeRmlEventListener(query->callback, query->userData, element);
	element->AddEventListener(query->event, listener, query->inCapturePhase);
	result->eventListenerHandle = ToHandle(listener);
	result->success = true;
}

static void NativeElementBlur(const RmlElementHandleQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	element->Blur();
	result->success = true;
}

static void NativeElementClick(const RmlElementHandleQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	element->Click();
	result->success = true;
}

static void NativeElementFocus(const RmlElementHandleQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	element->Focus();
	result->success = true;
}

static void NativeElementGetAttribute(const RmlElementGetAttributeQuery* query, RmlElementGetAttributeResult* result)
{
	static thread_local Rml::String value;
	result->error = nullptr;
	result->value = nullptr;
	result->exists = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr || query->name == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	if (!element->HasAttribute(query->name))
		return;
	value = element->GetAttribute<Rml::String>(query->name, "");
	result->value = value.c_str();
	result->exists = true;
}

static void NativeElementGetElementById(const RmlElementGetByStringQuery* query, RmlElementGetElementResult* result)
{
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr || query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		result->elementHandle = 0;
		result->exists = false;
		return;
	}
	FillElementResult(element->GetElementById(query->value), result);
}

static void NativeElementQuerySelector(const RmlElementGetByStringQuery* query, RmlElementGetElementResult* result)
{
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr || query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		result->elementHandle = 0;
		result->exists = false;
		return;
	}
	FillElementResult(element->QuerySelector(query->value), result);
}

static void NativeElementHasAttribute(const RmlElementGetByStringQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr || query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->success = element->HasAttribute(query->value);
}

static void NativeElementHasChildNodes(const RmlElementHandleQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->success = element->HasChildNodes();
}

static void NativeElementInsertBefore(const RmlElementInsertBeforeQuery* query, RmlElementGetElementResult* result)
{
	Rml::Element* element = FromElementHandle(query->elementHandle);
	Rml::ElementPtr inserted = TakeElementPtr(query->elementPtrHandle);
	Rml::Element* adjacent = FromElementHandle(query->adjacentElementHandle);
	if (element == nullptr || inserted == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		result->elementHandle = 0;
		result->exists = false;
		return;
	}
	FillElementResult(element->InsertBefore(std::move(inserted), adjacent), result);
}

static void NativeElementIsClassSet(const RmlElementGetByStringQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr || query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->success = element->IsClassSet(query->value);
}

static void NativeElementRemoveAttribute(const RmlElementGetByStringQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr || query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	element->RemoveAttribute(query->value);
	result->success = true;
}

static void NativeElementRemoveChild(const RmlElementChildQuery* query, RmlDocumentCreateElementResult* result)
{
	result->error = nullptr;
	result->elementPtrHandle = 0;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	Rml::Element* child = FromElementHandle(query->childElementHandle);
	if (element == nullptr || child == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->elementPtrHandle = StoreElementPtr(element->RemoveChild(child));
	result->success = (result->elementPtrHandle != 0);
}

static void NativeElementReplaceChild(const RmlElementReplaceChildQuery* query, RmlDocumentCreateElementResult* result)
{
	result->error = nullptr;
	result->elementPtrHandle = 0;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	Rml::ElementPtr inserted = TakeElementPtr(query->elementPtrHandle);
	Rml::Element* replaced = FromElementHandle(query->replacedElementHandle);
	if (element == nullptr || inserted == nullptr || replaced == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->elementPtrHandle = StoreElementPtr(element->ReplaceChild(std::move(inserted), replaced));
	result->success = (result->elementPtrHandle != 0);
}

static void NativeElementScrollIntoView(const RmlElementScrollIntoViewQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	element->ScrollIntoView(query->alignWithTop);
	result->success = true;
}

static void NativeElementSetAttribute(const RmlElementSetAttributeQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr || query->name == nullptr || query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	element->SetAttribute(query->name, query->value);
	result->success = true;
}

static void NativeElementSetClass(const RmlElementStringBoolQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr || query->name == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	element->SetClass(query->name, query->value);
	result->success = true;
}

static void NativeElementGetElementsByClassNameCount(const RmlElementGetByStringQuery* query, RmlElementGetIntResult* result)
{
	result->error = nullptr;
	result->value = 0;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr || query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	Rml::ElementList elements;
	element->GetElementsByClassName(elements, query->value);
	result->value = static_cast<int32_t>(elements.size());
}

static void NativeElementGetElementsByTagNameCount(const RmlElementGetByStringQuery* query, RmlElementGetIntResult* result)
{
	result->error = nullptr;
	result->value = 0;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr || query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	Rml::ElementList elements;
	element->GetElementsByTagName(elements, query->value);
	result->value = static_cast<int32_t>(elements.size());
}

static void NativeElementQuerySelectorAllCount(const RmlElementGetByStringQuery* query, RmlElementGetIntResult* result)
{
	result->error = nullptr;
	result->value = 0;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr || query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	Rml::ElementList elements;
	element->QuerySelectorAll(elements, query->value);
	result->value = static_cast<int32_t>(elements.size());
}

static void FillElementHandleList(const Rml::ElementList& elements, RmlElementHandleListResult* result)
{
	elementHandleResults.clear();
	elementHandleResults.reserve(elements.size());
	for (Rml::Element* element : elements) {
		elementHandleResults.push_back(ToHandle(element));
	}

	result->error = nullptr;
	result->elementHandles = elementHandleResults.data();
	result->elementHandleCount = static_cast<uint32_t>(elementHandleResults.size());
}

static void NativeElementGetElementsByClassName(const RmlElementGetByStringQuery* query, RmlElementHandleListResult* result)
{
	result->error = nullptr;
	result->elementHandles = nullptr;
	result->elementHandleCount = 0;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr || query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	Rml::ElementList elements;
	element->GetElementsByClassName(elements, query->value);
	FillElementHandleList(elements, result);
}

static void NativeElementGetElementsByTagName(const RmlElementGetByStringQuery* query, RmlElementHandleListResult* result)
{
	result->error = nullptr;
	result->elementHandles = nullptr;
	result->elementHandleCount = 0;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr || query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	Rml::ElementList elements;
	element->GetElementsByTagName(elements, query->value);
	FillElementHandleList(elements, result);
}

static void NativeElementQuerySelectorAll(const RmlElementGetByStringQuery* query, RmlElementHandleListResult* result)
{
	result->error = nullptr;
	result->elementHandles = nullptr;
	result->elementHandleCount = 0;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr || query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	Rml::ElementList elements;
	element->QuerySelectorAll(elements, query->value);
	FillElementHandleList(elements, result);
}

static void NativeElementClone(const RmlElementHandleQuery* query, RmlDocumentCreateElementResult* result)
{
	result->error = nullptr;
	result->elementPtrHandle = 0;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->elementPtrHandle = StoreElementPtr(element->Clone());
	result->success = (result->elementPtrHandle != 0);
}

static void NativeElementClosest(const RmlElementGetByStringQuery* query, RmlElementGetElementResult* result)
{
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr || query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		result->elementHandle = 0;
		result->exists = false;
		return;
	}
	FillElementResult(element->Closest(query->value), result);
}

static void NativeElementSetPseudoClass(const RmlElementStringBoolQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr || query->name == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	element->SetPseudoClass(query->name, query->value);
	result->success = true;
}

static void NativeElementIsPseudoClassSet(const RmlElementGetByStringQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr || query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->success = element->IsPseudoClassSet(query->value);
}

static void NativeElementArePseudoClassesSet(const RmlElementGetByStringQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr || query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->success = element->ArePseudoClassesSet(SplitStringList(query->value));
}

static void NativeElementGetActivePseudoClasses(const RmlElementHandleQuery* query, RmlElementStringListResult* result)
{
	result->error = nullptr;
	result->values = nullptr;
	result->valueCount = 0;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	stringResults = element->GetActivePseudoClasses();
	stringPtrResults.clear();
	stringPtrResults.reserve(stringResults.size());
	for (const Rml::String& value : stringResults) {
		stringPtrResults.push_back(value.c_str());
	}
	result->values = stringPtrResults.data();
	result->valueCount = static_cast<uint32_t>(stringPtrResults.size());
}

static void NativeElementIsPointWithinElement(const RmlElementPointQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->success = element->IsPointWithinElement(Rml::Vector2f(query->x, query->y));
}

static void NativeElementMatches(const RmlElementGetByStringQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr || query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->success = element->Matches(query->value);
}

static void NativeElementDispatchEvent(const RmlElementDispatchEventQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr || query->event == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->success = element->DispatchEvent(query->event, Rml::Dictionary());
}

static void NativeElementProcessDefaultAction(const RmlElementProcessDefaultActionQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	Rml::Event* event = query->eventHandle != 0 ? FromEventHandle(query->eventHandle) : currentEvent;
	if (element == nullptr || event == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	element->ProcessDefaultAction(*event);
	result->success = true;
}

static void NativeElementGetValue(const RmlElementHandleQuery* query, RmlElementGetStringResult* result)
{
	static thread_local Rml::String value;
	result->error = nullptr;
	result->value = nullptr;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	if (auto* input = dynamic_cast<Rml::ElementFormControlInput*>(element)) {
		value = input->GetValue();
	} else if (auto* textarea = dynamic_cast<Rml::ElementFormControlTextArea*>(element)) {
		value = textarea->GetValue();
	} else {
		value.clear();
	}
	result->value = value.c_str();
}

static void NativeElementGetChild(const RmlElementGetChildQuery* query, RmlElementGetElementResult* result)
{
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		result->elementHandle = 0;
		result->exists = false;
		return;
	}
	FillElementResult(element->GetChild(query->index), result);
}

static void FillElementString(const Rml::String& value, RmlElementGetStringResult* result)
{
	result->error = nullptr;
	result->value = value.c_str();
}

static void NativeElementGetClassName(const RmlElementHandleQuery* query, RmlElementGetStringResult* result)
{
	static thread_local Rml::String value;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		result->value = nullptr;
		return;
	}
	value = element->GetClassNames();
	FillElementString(value, result);
}

static void NativeElementSetClassName(const RmlElementSetStringQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr || query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	element->SetClassNames(query->value);
	result->success = true;
}

static void NativeElementGetId(const RmlElementHandleQuery* query, RmlElementGetStringResult* result)
{
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		result->value = nullptr;
		return;
	}
	FillElementString(element->GetId(), result);
}

static void NativeElementSetId(const RmlElementSetStringQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr || query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	element->SetId(query->value);
	result->success = true;
}

static void NativeElementGetInnerRml(const RmlElementHandleQuery* query, RmlElementGetStringResult* result)
{
	static thread_local Rml::String value;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		result->value = nullptr;
		return;
	}
	value = element->GetInnerRML();
	FillElementString(value, result);
}

static void NativeElementSetInnerRml(const RmlElementSetStringQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr || query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	element->SetInnerRML(query->value);
	result->success = true;
}

static void NativeElementGetScrollLeft(const RmlElementHandleQuery* query, RmlElementGetIntResult* result)
{
	result->error = nullptr;
	result->value = 0;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->value = static_cast<int32_t>(element->GetScrollLeft());
}

static void NativeElementSetScrollLeft(const RmlElementSetIntQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	element->SetScrollLeft(static_cast<float>(query->value));
	result->success = true;
}

static void NativeElementGetScrollTop(const RmlElementHandleQuery* query, RmlElementGetIntResult* result)
{
	result->error = nullptr;
	result->value = 0;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->value = static_cast<int32_t>(element->GetScrollTop());
}

static void NativeElementSetScrollTop(const RmlElementSetIntQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	element->SetScrollTop(static_cast<float>(query->value));
	result->success = true;
}

static void NativeElementGetTagName(const RmlElementHandleQuery* query, RmlElementGetStringResult* result)
{
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		result->value = nullptr;
		return;
	}
	FillElementString(element->GetTagName(), result);
}

static void NativeElementIsVisible(const RmlElementHandleQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->success = element->IsVisible();
}

static void NativeElementFormSubmit(const RmlElementFormSubmitQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	auto* form = dynamic_cast<Rml::ElementForm*>(FromElementHandle(query->elementHandle));
	if (form == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	if (query->name != nullptr && query->value != nullptr) {
		form->Submit(query->name, query->value);
	} else if (query->name != nullptr) {
		form->Submit(query->name);
	} else {
		form->Submit();
	}
	result->success = true;
}

static void NativeElementFormControlSelectAdd(const RmlElementFormControlSelectAddQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	auto* select = dynamic_cast<Rml::ElementFormControlSelect*>(FromElementHandle(query->elementHandle));
	Rml::ElementPtr option = TakeElementPtr(query->elementPtrHandle);
	if (select == nullptr || option == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	select->Add(std::move(option), query->before);
	result->success = true;
}

static void NativeElementFormControlSelectRemove(const RmlElementFormControlSelectRemoveQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	auto* select = dynamic_cast<Rml::ElementFormControlSelect*>(FromElementHandle(query->elementHandle));
	if (select == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	select->Remove(query->index);
	result->success = true;
}

static void NativeElementFormControlSelectRemoveAll(const RmlElementHandleQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	auto* select = dynamic_cast<Rml::ElementFormControlSelect*>(FromElementHandle(query->elementHandle));
	if (select == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	select->RemoveAll();
	result->success = true;
}

static void NativeElementFormControlInputSelect(const RmlElementHandleQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	auto* input = dynamic_cast<Rml::ElementFormControlInput*>(FromElementHandle(query->elementHandle));
	if (input == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	input->Select();
	result->success = true;
}

static void NativeElementFormControlInputSetSelection(const RmlElementFormControlSelectionQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	auto* input = dynamic_cast<Rml::ElementFormControlInput*>(FromElementHandle(query->elementHandle));
	if (input == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	input->SetSelectionRange(query->start, query->end);
	result->success = true;
}

static void NativeElementFormControlInputGetSelection(const RmlElementHandleQuery* query, RmlElementFormControlSelectionResult* result)
{
	result->error = nullptr;
	result->start = 0;
	result->end = 0;
	result->success = false;
	auto* input = dynamic_cast<Rml::ElementFormControlInput*>(FromElementHandle(query->elementHandle));
	if (input == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	Rml::String selected;
	input->GetSelection(&result->start, &result->end, &selected);
	result->success = true;
}

static void NativeElementFormControlTextAreaSelect(const RmlElementHandleQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	auto* textarea = dynamic_cast<Rml::ElementFormControlTextArea*>(FromElementHandle(query->elementHandle));
	if (textarea == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	textarea->Select();
	result->success = true;
}

static void NativeElementFormControlTextAreaSetSelection(const RmlElementFormControlSelectionQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	auto* textarea = dynamic_cast<Rml::ElementFormControlTextArea*>(FromElementHandle(query->elementHandle));
	if (textarea == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	textarea->SetSelectionRange(query->start, query->end);
	result->success = true;
}

static void NativeElementFormControlTextAreaGetSelection(const RmlElementHandleQuery* query, RmlElementFormControlSelectionResult* result)
{
	result->error = nullptr;
	result->start = 0;
	result->end = 0;
	result->success = false;
	auto* textarea = dynamic_cast<Rml::ElementFormControlTextArea*>(FromElementHandle(query->elementHandle));
	if (textarea == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	Rml::String selected;
	textarea->GetSelection(&result->start, &result->end, &selected);
	result->success = true;
}

static void NativeElementTabSetSetPanel(const RmlElementTabSetIndexStringQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	auto* tabSet = dynamic_cast<Rml::ElementTabSet*>(FromElementHandle(query->elementHandle));
	if (tabSet == nullptr || query->rml == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	tabSet->SetPanel(query->index, query->rml);
	result->success = true;
}

static void NativeElementTabSetSetTab(const RmlElementTabSetIndexStringQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	auto* tabSet = dynamic_cast<Rml::ElementTabSet*>(FromElementHandle(query->elementHandle));
	if (tabSet == nullptr || query->rml == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	tabSet->SetTab(query->index, query->rml);
	result->success = true;
}

static void NativeElementTabSetRemoveTab(const RmlElementTabSetIndexQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	auto* tabSet = dynamic_cast<Rml::ElementTabSet*>(FromElementHandle(query->elementHandle));
	if (tabSet == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	tabSet->RemoveTab(query->index);
	result->success = true;
}

static void NativeEventListenerOnAttach(const RmlEventListenerElementQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::EventListener* listener = FromEventListenerHandle(query->eventListenerHandle);
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (listener == nullptr || element == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	listener->OnAttach(element);
	result->success = true;
}

static void NativeEventListenerOnDetach(const RmlEventListenerElementQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::EventListener* listener = FromEventListenerHandle(query->eventListenerHandle);
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (listener == nullptr || element == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	listener->OnDetach(element);
	result->success = true;
}

static void NativeEventListenerProcessEvent(const RmlEventListenerEventQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::EventListener* listener = FromEventListenerHandle(query->eventListenerHandle);
	Rml::Event* event = query->eventHandle != 0 ? FromEventHandle(query->eventHandle) : currentEvent;
	if (listener == nullptr || event == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	listener->ProcessEvent(*event);
	result->success = true;
}

static void NativeEventGetCurrent(const RmlEventCurrentQuery*, RmlEventCurrentResult* result)
{
	result->error = nullptr;
	result->eventHandle = ToHandle(currentEvent);
	result->elementHandle = ToHandle(currentEventElement);
	result->documentHandle = ToHandle(currentEventDocument);
	result->exists = (currentEvent != nullptr);
}

static void NativeEventStopPropagation(const RmlEventHandleQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Event* event = FromEventHandle(query->eventHandle);
	if (event == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	event->StopPropagation();
	result->success = true;
}

static void NativeEventStopImmediatePropagation(const RmlEventHandleQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Event* event = FromEventHandle(query->eventHandle);
	if (event == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	event->StopImmediatePropagation();
	result->success = true;
}

static void NativeEventGetCurrentElement(const RmlEventHandleQuery* query, RmlElementGetElementResult* result)
{
	result->error = nullptr;
	result->elementHandle = 0;
	result->exists = false;
	Rml::Event* event = FromEventHandle(query->eventHandle);
	if (event == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->elementHandle = ToHandle(event->GetCurrentElement());
	result->exists = (result->elementHandle != 0);
}

static void NativeEventGetTargetElement(const RmlEventHandleQuery* query, RmlElementGetElementResult* result)
{
	result->error = nullptr;
	result->elementHandle = 0;
	result->exists = false;
	Rml::Event* event = FromEventHandle(query->eventHandle);
	if (event == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->elementHandle = ToHandle(event->GetTargetElement());
	result->exists = (result->elementHandle != 0);
}

static void NativeEventGetType(const RmlEventHandleQuery* query, RmlEventGetStringResult* result)
{
	result->error = nullptr;
	result->value = nullptr;
	result->exists = false;
	Rml::Event* event = FromEventHandle(query->eventHandle);
	if (event == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->value = event->GetType().c_str();
	result->exists = true;
}

static void NativeEventGetPhase(const RmlEventHandleQuery* query, RmlEventGetIntResult* result)
{
	result->error = nullptr;
	result->value = 0;
	result->exists = false;
	Rml::Event* event = FromEventHandle(query->eventHandle);
	if (event == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->value = static_cast<int32_t>(event->GetPhase());
	result->exists = true;
}

static void NativeEventIsInterruptible(const RmlEventHandleQuery* query, RmlEventGetBoolResult* result)
{
	result->error = nullptr;
	result->value = false;
	result->exists = false;
	Rml::Event* event = FromEventHandle(query->eventHandle);
	if (event == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->value = event->IsInterruptible();
	result->exists = true;
}

static void NativeEventIsPropagating(const RmlEventHandleQuery* query, RmlEventGetBoolResult* result)
{
	result->error = nullptr;
	result->value = false;
	result->exists = false;
	Rml::Event* event = FromEventHandle(query->eventHandle);
	if (event == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->value = event->IsPropagating();
	result->exists = true;
}

static void NativeEventIsImmediatePropagating(const RmlEventHandleQuery* query, RmlEventGetBoolResult* result)
{
	result->error = nullptr;
	result->value = false;
	result->exists = false;
	Rml::Event* event = FromEventHandle(query->eventHandle);
	if (event == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->value = event->IsImmediatePropagating();
	result->exists = true;
}

static void NativeEventGetParameterType(const RmlEventParameterQuery* query, RmlEventGetIntResult* result)
{
	result->error = nullptr;
	result->value = static_cast<int32_t>(Rml::Variant::NONE);
	result->exists = false;
	const Rml::Variant* value = GetEventParameter(FromEventHandle(query->eventHandle), query->name);
	if (value == nullptr)
		return;

	result->value = static_cast<int32_t>(value->GetType());
	result->exists = true;
}

static void NativeEventGetParameterBool(const RmlEventParameterQuery* query, RmlEventGetBoolResult* result)
{
	result->error = nullptr;
	result->value = false;
	result->exists = false;
	const Rml::Variant* value = GetEventParameter(FromEventHandle(query->eventHandle), query->name);
	if (value == nullptr)
		return;

	result->value = value->Get(false);
	result->exists = true;
}

static void NativeEventGetParameterInt(const RmlEventParameterQuery* query, RmlEventGetIntResult* result)
{
	result->error = nullptr;
	result->value = 0;
	result->exists = false;
	const Rml::Variant* value = GetEventParameter(FromEventHandle(query->eventHandle), query->name);
	if (value == nullptr)
		return;

	result->value = value->Get(0);
	result->exists = true;
}

static void NativeEventGetParameterFloat(const RmlEventParameterQuery* query, RmlEventGetFloatResult* result)
{
	result->error = nullptr;
	result->value = 0.0f;
	result->exists = false;
	const Rml::Variant* value = GetEventParameter(FromEventHandle(query->eventHandle), query->name);
	if (value == nullptr)
		return;

	result->value = value->Get(0.0f);
	result->exists = true;
}

static void NativeEventGetParameterString(const RmlEventParameterQuery* query, RmlEventGetStringResult* result)
{
	static thread_local Rml::String valueString;

	result->error = nullptr;
	result->value = nullptr;
	result->exists = false;
	const Rml::Variant* value = GetEventParameter(FromEventHandle(query->eventHandle), query->name);
	if (value == nullptr)
		return;

	valueString = value->Get(Rml::String());
	result->value = valueString.c_str();
	result->exists = true;
}

static void NativeSolLuaDataModelSetDirty(const RmlSolLuaDataModelSetDirtyQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	if (query->property == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	auto it = nativeDataModels.find(query->dataModelHandle);
	if (it == nativeDataModels.end()) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	it->second.DirtyVariable(query->property);
	result->success = true;
}

static void NativeVector2fNew(const RmlVector2fNewQuery* query, RmlVector2fNewResult* result)
{
	result->error = nullptr;
	result->x = query->x;
	result->y = query->y;
}

static void NativeVector2iNew(const RmlVector2iNewQuery* query, RmlVector2iNewResult* result)
{
	result->error = nullptr;
	result->x = query->x;
	result->y = query->y;
}

} // namespace

const RmlUiApi RMLUI_API = {
	.CreateContext = NativeCreateContext,
	.GetContext = NativeGetContext,
	.RemoveContext = NativeRemoveContext,
	.RemoveContextByName = NativeRemoveContextByName,
	.SetDebugContext = NativeSetDebugContext,
	.SetDebugContextByName = NativeSetDebugContextByName,
	.LoadFontFace = NativeLoadFontFace,
	.RegisterEventType = NativeRegisterEventType,
	.RegiserEventType = NativeRegisterEventType,
	.AddTranslationString = NativeAddTranslationString,
	.ClearTranslations = NativeClearTranslations,
	.SetMouseCursorAlias = NativeSetMouseCursorAlias,
	.GetVersion = NativeGetVersion,
	.IsReady = NativeIsReady,
	.ContextCreateDocument = NativeContextCreateDocument,
	.ContextLoadDocument = NativeContextLoadDocument,
	.ContextGetDocument = NativeContextGetDocument,
	.ContextAddEventListener = NativeContextAddEventListener,
	.ContextRender = NativeContextRender,
	.ContextUnloadAllDocuments = NativeContextUnloadAllDocuments,
	.ContextUnloadDocument = NativeContextUnloadDocument,
	.ContextUpdate = NativeContextUpdate,
	.ContextOpenDataModel = NativeContextOpenDataModel,
	.ContextRemoveDataModel = NativeContextRemoveDataModel,
	.ContextProcessMouseMove = NativeContextProcessMouseMove,
	.ContextProcessMouseButtonDown = NativeContextProcessMouseButtonDown,
	.ContextProcessMouseButtonUp = NativeContextProcessMouseButtonUp,
	.ContextProcessMouseWheel = NativeContextProcessMouseWheel,
	.ContextProcessMouseLeave = NativeContextProcessMouseLeave,
	.ContextIsMouseInteracting = NativeContextIsMouseInteracting,
	.ContextProcessKeyDown = NativeContextProcessKeyDown,
	.ContextProcessKeyUp = NativeContextProcessKeyUp,
	.ContextProcessTextInput = NativeContextProcessTextInput,
	.ContextEnableMouseCursor = NativeContextEnableMouseCursor,
	.ContextActivateTheme = NativeContextActivateTheme,
	.ContextIsThemeActive = NativeContextIsThemeActive,
	.ContextGetElementAtPoint = NativeContextGetElementAtPoint,
	.ContextPullDocumentToFront = NativeContextPullDocumentToFront,
	.ContextPushDocumentToBack = NativeContextPushDocumentToBack,
	.ContextGetName = NativeContextGetName,
	.ContextGetDimensions = NativeContextGetDimensions,
	.ContextSetDimensions = NativeContextSetDimensions,
	.ContextGetDensityIndependentPixelRatio = NativeContextGetDensityIndependentPixelRatio,
	.ContextSetDensityIndependentPixelRatio = NativeContextSetDensityIndependentPixelRatio,
	.ContextGetFocusElement = NativeContextGetFocusElement,
	.ContextGetHoverElement = NativeContextGetHoverElement,
	.ContextGetRootElement = NativeContextGetRootElement,
	.DocumentPullToFront = NativeDocumentPullToFront,
	.DocumentPushToBack = NativeDocumentPushToBack,
	.DocumentShow = NativeDocumentShow,
	.DocumentHide = NativeDocumentHide,
	.DocumentClose = NativeDocumentClose,
	.DocumentCreateElement = NativeDocumentCreateElement,
	.DocumentCreateTextNode = NativeDocumentCreateTextNode,
	.DocumentReloadStyleSheet = NativeDocumentReloadStyleSheet,
	.DocumentLoadInlineScript = NativeDocumentLoadInlineScript,
	.DocumentLoadExternalScript = NativeDocumentLoadExternalScript,
	.DocumentUpdateDocument = NativeDocumentUpdateDocument,
	.DocumentAppendToStyleSheet = NativeDocumentAppendToStyleSheet,
	.DocumentGetTitle = NativeDocumentGetTitle,
	.DocumentSetTitle = NativeDocumentSetTitle,
	.DocumentGetContext = NativeDocumentGetContext,
	.DocumentGetUrl = NativeDocumentGetUrl,
	.DocumentIsModal = NativeDocumentIsModal,
	.ElementAppendChild = NativeElementAppendChild,
	.ElementAddEventListener = NativeElementAddEventListener,
	.ElementBlur = NativeElementBlur,
	.ElementClick = NativeElementClick,
	.ElementFocus = NativeElementFocus,
	.ElementGetAttribute = NativeElementGetAttribute,
	.ElementGetElementById = NativeElementGetElementById,
	.ElementQuerySelector = NativeElementQuerySelector,
	.ElementHasAttribute = NativeElementHasAttribute,
	.ElementHasChildNodes = NativeElementHasChildNodes,
	.ElementInsertBefore = NativeElementInsertBefore,
	.ElementIsClassSet = NativeElementIsClassSet,
	.ElementRemoveAttribute = NativeElementRemoveAttribute,
	.ElementRemoveChild = NativeElementRemoveChild,
	.ElementReplaceChild = NativeElementReplaceChild,
	.ElementScrollIntoView = NativeElementScrollIntoView,
	.ElementSetAttribute = NativeElementSetAttribute,
	.ElementSetClass = NativeElementSetClass,
	.ElementGetElementsByClassNameCount = NativeElementGetElementsByClassNameCount,
	.ElementGetElementsByTagNameCount = NativeElementGetElementsByTagNameCount,
	.ElementQuerySelectorAllCount = NativeElementQuerySelectorAllCount,
	.ElementGetElementsByClassName = NativeElementGetElementsByClassName,
	.ElementGetElementsByTagName = NativeElementGetElementsByTagName,
	.ElementQuerySelectorAll = NativeElementQuerySelectorAll,
	.ElementClone = NativeElementClone,
	.ElementClosest = NativeElementClosest,
	.ElementSetPseudoClass = NativeElementSetPseudoClass,
	.ElementIsPseudoClassSet = NativeElementIsPseudoClassSet,
	.ElementArePseudoClassesSet = NativeElementArePseudoClassesSet,
	.ElementGetActivePseudoClasses = NativeElementGetActivePseudoClasses,
	.ElementIsPointWithinElement = NativeElementIsPointWithinElement,
	.ElementMatches = NativeElementMatches,
	.ElementDispatchEvent = NativeElementDispatchEvent,
	.ElementProcessDefaultAction = NativeElementProcessDefaultAction,
	.ElementGetValue = NativeElementGetValue,
	.ElementGetChild = NativeElementGetChild,
	.ElementGetClassName = NativeElementGetClassName,
	.ElementSetClassName = NativeElementSetClassName,
	.ElementGetId = NativeElementGetId,
	.ElementSetId = NativeElementSetId,
	.ElementGetInnerRml = NativeElementGetInnerRml,
	.ElementSetInnerRml = NativeElementSetInnerRml,
	.ElementGetScrollLeft = NativeElementGetScrollLeft,
	.ElementSetScrollLeft = NativeElementSetScrollLeft,
	.ElementGetScrollTop = NativeElementGetScrollTop,
	.ElementSetScrollTop = NativeElementSetScrollTop,
	.ElementGetTagName = NativeElementGetTagName,
	.ElementIsVisible = NativeElementIsVisible,
	.ElementFormSubmit = NativeElementFormSubmit,
	.ElementFormControlSelectAdd = NativeElementFormControlSelectAdd,
	.ElementFormControlSelectRemove = NativeElementFormControlSelectRemove,
	.ElementFormControlSelectRemoveAll = NativeElementFormControlSelectRemoveAll,
	.ElementFormControlInputSelect = NativeElementFormControlInputSelect,
	.ElementFormControlInputSetSelection = NativeElementFormControlInputSetSelection,
	.ElementFormControlInputGetSelection = NativeElementFormControlInputGetSelection,
	.ElementFormControlTextAreaSelect = NativeElementFormControlTextAreaSelect,
	.ElementFormControlTextAreaSetSelection = NativeElementFormControlTextAreaSetSelection,
	.ElementFormControlTextAreaGetSelection = NativeElementFormControlTextAreaGetSelection,
	.ElementTabSetSetPanel = NativeElementTabSetSetPanel,
	.ElementTabSetSetTab = NativeElementTabSetSetTab,
	.ElementTabSetRemoveTab = NativeElementTabSetRemoveTab,
	.EventListenerOnAttach = NativeEventListenerOnAttach,
	.EventListenerOnDetach = NativeEventListenerOnDetach,
	.EventListenerProcessEvent = NativeEventListenerProcessEvent,
	.EventGetCurrent = NativeEventGetCurrent,
	.EventStopPropagation = NativeEventStopPropagation,
	.EventStopImmediatePropagation = NativeEventStopImmediatePropagation,
	.EventGetCurrentElement = NativeEventGetCurrentElement,
	.EventGetTargetElement = NativeEventGetTargetElement,
	.EventGetType = NativeEventGetType,
	.EventGetPhase = NativeEventGetPhase,
	.EventIsInterruptible = NativeEventIsInterruptible,
	.EventIsPropagating = NativeEventIsPropagating,
	.EventIsImmediatePropagating = NativeEventIsImmediatePropagating,
	.EventGetParameterType = NativeEventGetParameterType,
	.EventGetParameterBool = NativeEventGetParameterBool,
	.EventGetParameterInt = NativeEventGetParameterInt,
	.EventGetParameterFloat = NativeEventGetParameterFloat,
	.EventGetParameterString = NativeEventGetParameterString,
	.SolLuaDataModelSetDirty = NativeSolLuaDataModelSetDirty,
	.Vector2fNew = NativeVector2fNew,
	.Vector2iNew = NativeVector2iNew,
};
