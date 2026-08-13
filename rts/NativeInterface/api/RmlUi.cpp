#include "RmlUi.h"

#include "Rml/Backends/RmlUi_Backend.h"

#include <RmlUi/Core.h>
#include <RmlUi/Core/Context.h>
#include <RmlUi/Core/ElementDocument.h>
#include <RmlUi/Core/Event.h>
#include <RmlUi/Core/EventListener.h>
#include <RmlUi/Core/Factory.h>
#include <RmlUi/Core/StyleSheetContainer.h>
#include <RmlUi/Core/Elements/ElementForm.h>
#include <RmlUi/Core/Elements/ElementFormControl.h>
#include <RmlUi/Core/Elements/ElementFormControlInput.h>
#include <RmlUi/Core/Elements/ElementFormControlSelect.h>
#include <RmlUi/Core/Elements/ElementFormControlTextArea.h>
#include <RmlUi/Core/Elements/ElementTabSet.h>

#include <cstdint>
#include <memory>
#include <sstream>
#include <unordered_map>
#include <unordered_set>
#include <vector>

namespace {

static const Error INVALID_ARGUMENT_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid argument" };
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "RmlUi is not ready" };

template <typename T>
static uint64_t ToHandle(T* pointer)
{
	return reinterpret_cast<uintptr_t>(pointer);
}

// Element handles must be validated before they are used.
//
// A handle is the element's address, and a native plugin holds on to it across
// frames. RmlUi destroys elements whenever a subtree is rebuilt (setting
// inner_rml, say), which leaves the plugin holding a dangling handle -- and
// every call below dereferenced it without checking, so the next SetAttribute
// wrote through freed memory.
//
// Rml::Element derives from EnableObserverPtr, so RmlUi already knows whether an
// element is still alive. Every handle handed out is remembered with an observer
// pointer, and resolving one that has since died returns null instead of a wild
// pointer.
static std::unordered_map<uint64_t, Rml::ObserverPtr<Rml::Element>> liveElements;

static uint64_t ToElementHandle(Rml::Element* element)
{
	if (element == nullptr)
		return 0;

	// Elements die constantly -- every rebuilt subtree -- and a dead entry is
	// only noticed when its handle is next resolved, which may be never. Sweep
	// them rather than grow without bound.
	if (liveElements.size() > 4096) {
		for (auto it = liveElements.begin(); it != liveElements.end(); ) {
			it = (it->second.get() == nullptr) ? liveElements.erase(it) : std::next(it);
		}
	}

	const uint64_t handle = ToHandle(element);
	liveElements.insert_or_assign(handle, element->GetObserverPtr());
	return handle;
}

template <typename T>
static T* FromHandle(uint64_t handle)
{
	return reinterpret_cast<T*>(static_cast<uintptr_t>(handle));
}

static Rml::Context* FromHandle(uint64_t handle)
{
	if (handle == 0 || !RmlGui::IsInitialized())
		return nullptr;

	// Contexts do not expose observer pointers. Resolve the opaque handle by
	// comparing it with the currently-owned contexts without ever dereferencing
	// the caller's possibly stale pointer.
	for (int index = 0; index < Rml::GetNumContexts(); ++index) {
		Rml::Context* context = Rml::GetContext(index);
		if (ToHandle(context) == handle)
			return context;
	}

	return nullptr;
}

static uint64_t nextElementPtrHandle = 1;
static uint64_t nextDataModelHandle = 1;
static uint64_t nextDataModelVariableHandle = 1;
static std::unordered_map<uint64_t, Rml::ElementPtr> ownedElementPtrs;
static std::unordered_set<Rml::String> nativeContextNames;

enum class NativeDataValueType { Bool, Int, Float, String, Color, Pixels, Percent };

struct NativeDataPixels {
	float value = 0.0f;
};

struct NativeDataPercent {
	float value = 0.0f;
};

struct NativeDataVariable
{
	NativeDataValueType type;
	Rml::String name;
	bool boolValue = false;
	int32_t intValue = 0;
	float floatValue = 0.0f;
	Rml::String stringValue;
	Rml::Colourb colourValue;
	NativeDataPixels pixelsValue;
	NativeDataPercent percentValue;
};

struct NativeDataTextRow
{
	Rml::String text;
	bool muted = false;
	bool visible = false;
};

struct NativeDataTextRows
{
	Rml::String name;
	std::vector<NativeDataTextRow> rows;
};

struct NativeDataLogRow
{
	Rml::String text;
	bool info = true;
	bool warning = false;
	bool error = false;
	bool selected = false;
	bool visible = false;
};

struct NativeDataLogRows
{
	Rml::String name;
	std::vector<NativeDataLogRow> rows;
};

struct NativeDataNotificationRow
{
	Rml::String title;
	Rml::String body;
	bool warning = false;
	bool hasProgress = false;
	NativeDataPercent progress;
};

struct NativeDataNotificationRows
{
	Rml::String name;
	std::vector<NativeDataNotificationRow> rows;
};

struct NativeDataIconRow
{
	Rml::String label;
	Rml::String icon;
	Rml::String tooltip;
	bool pressed = false;
	bool disabled = false;
	bool visible = false;
};

struct NativeDataIconRows
{
	Rml::String name;
	std::vector<NativeDataIconRow> rows;
};

struct NativeDataOptionRow
{
	Rml::String value;
	Rml::String label;
	bool visible = false;
};

struct NativeDataOptionRows
{
	Rml::String name;
	std::vector<NativeDataOptionRow> rows;
};

struct NativeDataChoiceRow
{
	Rml::String label;
	Rml::String detail;
	bool selected = false;
	bool highlighted = false;
	bool visible = false;
};

struct NativeDataChoiceRows
{
	Rml::String name;
	std::vector<NativeDataChoiceRow> rows;
};

struct NativeDataStatusRow
{
	Rml::String label;
	bool positive = false;
	bool visible = false;
};

struct NativeDataStatusRows
{
	Rml::String name;
	std::vector<NativeDataStatusRow> rows;
};

struct NativeDataSwatchRow
{
	Rml::String label;
	Rml::Colourb colour;
	bool actionsEnabled = false;
	bool visible = false;
};

struct NativeDataSwatchRows
{
	Rml::String name;
	std::vector<NativeDataSwatchRow> rows;
};

struct NativeDataGridRow
{
	Rml::String label;
	Rml::String image;
	NativeDataPixels cellSize;
	bool hasImage = false;
	bool nativeImage = false;
	bool selected = false;
	bool folder = false;
	bool filler = false;
	bool visible = false;
};

struct NativeDataGridRows
{
	Rml::String name;
	std::vector<NativeDataGridRow> rows;
};

struct NativeDataModel
{
	NativeDataModel(Rml::DataModelConstructor constructor, Rml::Context* context)
		: constructor(std::move(constructor))
		, handle(this->constructor.GetModelHandle())
		, context(context)
	{}

	Rml::DataModelConstructor constructor;
	Rml::DataModelHandle handle;
	Rml::Context* context;
	std::unordered_map<uint64_t, std::unique_ptr<NativeDataVariable>> variables;
	std::unordered_map<uint64_t, std::unique_ptr<NativeDataTextRows>> textRows;
	std::unordered_map<uint64_t, std::unique_ptr<NativeDataLogRows>> logRows;
	std::unordered_map<uint64_t, std::unique_ptr<NativeDataNotificationRows>> notificationRows;
	std::unordered_map<uint64_t, std::unique_ptr<NativeDataIconRows>> iconRows;
	std::unordered_map<uint64_t, std::unique_ptr<NativeDataOptionRows>> optionRows;
	std::unordered_map<uint64_t, std::unique_ptr<NativeDataChoiceRows>> choiceRows;
	std::unordered_map<uint64_t, std::unique_ptr<NativeDataStatusRows>> statusRows;
	std::unordered_map<uint64_t, std::unique_ptr<NativeDataSwatchRows>> swatchRows;
	std::unordered_map<uint64_t, std::unique_ptr<NativeDataGridRows>> gridRows;
};

struct NativeDataModelRecord
{
	Rml::Context* context = nullptr;
	Rml::String name;
	Rml::DataModelHandle handle;
	std::unique_ptr<NativeDataModel> native;
};
static std::unordered_map<uint64_t, NativeDataModelRecord> nativeDataModels;
static std::unordered_map<uint64_t, uint64_t> nativeDataVariableModels;
static std::unordered_map<uint64_t, uint64_t> nativeDataTextRowsModels;
static std::unordered_map<uint64_t, uint64_t> nativeDataLogRowsModels;
static std::unordered_map<uint64_t, uint64_t> nativeDataNotificationRowsModels;
static std::unordered_map<uint64_t, uint64_t> nativeDataIconRowsModels;
static std::unordered_map<uint64_t, uint64_t> nativeDataOptionRowsModels;
static std::unordered_map<uint64_t, uint64_t> nativeDataChoiceRowsModels;
static std::unordered_map<uint64_t, uint64_t> nativeDataStatusRowsModels;
static std::unordered_map<uint64_t, uint64_t> nativeDataSwatchRowsModels;
static std::unordered_map<uint64_t, uint64_t> nativeDataGridRowsModels;
static std::unordered_set<Rml::Context*> nativeTextRowTypes;
static std::unordered_set<Rml::Context*> nativeLogRowTypes;
static std::unordered_set<Rml::Context*> nativeNotificationRowTypes;
static std::unordered_set<Rml::Context*> nativeIconRowTypes;
static std::unordered_set<Rml::Context*> nativeOptionRowTypes;
static std::unordered_set<Rml::Context*> nativeChoiceRowTypes;
static std::unordered_set<Rml::Context*> nativeStatusRowTypes;
static std::unordered_set<Rml::Context*> nativeSwatchRowTypes;
static std::unordered_set<Rml::Context*> nativeGridRowTypes;
static std::unordered_set<Rml::Context*> nativeColourTypes;
static std::unordered_set<Rml::Context*> nativePixelTypes;
static std::unordered_set<Rml::Context*> nativePercentTypes;
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

// Null if the element behind this handle has been destroyed since it was handed
// out, rather than a pointer into freed memory.
static Rml::Element* FromElementHandle(uint64_t handle)
{
	if (handle == 0)
		return nullptr;

	const auto it = liveElements.find(handle);
	if (it == liveElements.end()) {
		// Never handed out by us (or already reaped). Refuse it: the alternative
		// is trusting an address the caller made up.
		return nullptr;
	}
	if (Rml::Element* element = it->second.get())
		return element;

	liveElements.erase(it);
	return nullptr;
}

// Documents are elements too, so resolve their handles through the same
// observer-backed lookup. Native modules can outlive the contexts and
// documents they created during a hot reload.
static Rml::ElementDocument* FromDocumentHandle(uint64_t handle)
{
	return rmlui_dynamic_cast<Rml::ElementDocument*>(FromElementHandle(handle));
}

static Rml::Event* FromEventHandle(uint64_t handle) { return FromHandle<Rml::Event>(handle); }
static Rml::EventListener* FromEventListenerHandle(uint64_t handle) { return FromHandle<Rml::EventListener>(handle); }

static void EraseNativeDataModelHandles(Rml::Context* context)
{
	for (auto it = nativeDataModels.begin(); it != nativeDataModels.end(); ) {
		if (it->second.context == context) {
			if (it->second.native != nullptr) {
				for (const auto& [variableHandle, _] : it->second.native->variables)
					nativeDataVariableModels.erase(variableHandle);
				for (const auto& [rowsHandle, _] : it->second.native->textRows)
					nativeDataTextRowsModels.erase(rowsHandle);
				for (const auto& [rowsHandle, _] : it->second.native->logRows)
					nativeDataLogRowsModels.erase(rowsHandle);
				for (const auto& [rowsHandle, _] : it->second.native->notificationRows)
					nativeDataNotificationRowsModels.erase(rowsHandle);
				for (const auto& [rowsHandle, _] : it->second.native->iconRows)
					nativeDataIconRowsModels.erase(rowsHandle);
				for (const auto& [rowsHandle, _] : it->second.native->optionRows)
					nativeDataOptionRowsModels.erase(rowsHandle);
				for (const auto& [rowsHandle, _] : it->second.native->choiceRows)
					nativeDataChoiceRowsModels.erase(rowsHandle);
				for (const auto& [rowsHandle, _] : it->second.native->statusRows)
					nativeDataStatusRowsModels.erase(rowsHandle);
				for (const auto& [rowsHandle, _] : it->second.native->swatchRows)
					nativeDataSwatchRowsModels.erase(rowsHandle);
				for (const auto& [rowsHandle, _] : it->second.native->gridRows)
					nativeDataGridRowsModels.erase(rowsHandle);
			}
			it = nativeDataModels.erase(it);
		} else {
			++it;
		}
	}
	nativeTextRowTypes.erase(context);
	nativeLogRowTypes.erase(context);
	nativeNotificationRowTypes.erase(context);
	nativeIconRowTypes.erase(context);
	nativeOptionRowTypes.erase(context);
	nativeChoiceRowTypes.erase(context);
	nativeStatusRowTypes.erase(context);
	nativeSwatchRowTypes.erase(context);
	nativeGridRowTypes.erase(context);
	nativeColourTypes.erase(context);
	nativePixelTypes.erase(context);
	nativePercentTypes.erase(context);
}

static void EraseNativeDataModelHandles(Rml::Context* context, const Rml::String& name)
{
	for (auto it = nativeDataModels.begin(); it != nativeDataModels.end(); ) {
		if (it->second.context == context && it->second.name == name) {
			if (it->second.native != nullptr) {
				for (const auto& [variableHandle, _] : it->second.native->variables)
					nativeDataVariableModels.erase(variableHandle);
				for (const auto& [rowsHandle, _] : it->second.native->textRows)
					nativeDataTextRowsModels.erase(rowsHandle);
				for (const auto& [rowsHandle, _] : it->second.native->logRows)
					nativeDataLogRowsModels.erase(rowsHandle);
				for (const auto& [rowsHandle, _] : it->second.native->notificationRows)
					nativeDataNotificationRowsModels.erase(rowsHandle);
				for (const auto& [rowsHandle, _] : it->second.native->iconRows)
					nativeDataIconRowsModels.erase(rowsHandle);
				for (const auto& [rowsHandle, _] : it->second.native->optionRows)
					nativeDataOptionRowsModels.erase(rowsHandle);
				for (const auto& [rowsHandle, _] : it->second.native->choiceRows)
					nativeDataChoiceRowsModels.erase(rowsHandle);
				for (const auto& [rowsHandle, _] : it->second.native->statusRows)
					nativeDataStatusRowsModels.erase(rowsHandle);
				for (const auto& [rowsHandle, _] : it->second.native->swatchRows)
					nativeDataSwatchRowsModels.erase(rowsHandle);
				for (const auto& [rowsHandle, _] : it->second.native->gridRows)
					nativeDataGridRowsModels.erase(rowsHandle);
			}
			it = nativeDataModels.erase(it);
		} else {
			++it;
		}
	}
}

static NativeDataModel* GetNativeDataModel(uint64_t handle)
{
	auto it = nativeDataModels.find(handle);
	return (it == nativeDataModels.end()) ? nullptr : it->second.native.get();
}

static NativeDataVariable* GetNativeDataVariable(uint64_t variableHandle, NativeDataValueType expectedType, NativeDataModel** outModel = nullptr)
{
	auto ownerIt = nativeDataVariableModels.find(variableHandle);
	if (ownerIt == nativeDataVariableModels.end())
		return nullptr;
	NativeDataModel* model = GetNativeDataModel(ownerIt->second);
	if (model == nullptr)
		return nullptr;
	auto variableIt = model->variables.find(variableHandle);
	if (variableIt == model->variables.end() || variableIt->second->type != expectedType)
		return nullptr;
	if (outModel != nullptr)
		*outModel = model;
	return variableIt->second.get();
}

static NativeDataTextRows* GetNativeDataTextRows(uint64_t rowsHandle, NativeDataModel** outModel = nullptr)
{
	auto ownerIt = nativeDataTextRowsModels.find(rowsHandle);
	if (ownerIt == nativeDataTextRowsModels.end())
		return nullptr;
	NativeDataModel* model = GetNativeDataModel(ownerIt->second);
	if (model == nullptr)
		return nullptr;
	auto rowsIt = model->textRows.find(rowsHandle);
	if (rowsIt == model->textRows.end())
		return nullptr;
	if (outModel != nullptr)
		*outModel = model;
	return rowsIt->second.get();
}

static NativeDataLogRows* GetNativeDataLogRows(uint64_t rowsHandle, NativeDataModel** outModel = nullptr)
{
	auto ownerIt = nativeDataLogRowsModels.find(rowsHandle);
	if (ownerIt == nativeDataLogRowsModels.end())
		return nullptr;
	NativeDataModel* model = GetNativeDataModel(ownerIt->second);
	if (model == nullptr)
		return nullptr;
	auto rowsIt = model->logRows.find(rowsHandle);
	if (rowsIt == model->logRows.end())
		return nullptr;
	if (outModel != nullptr)
		*outModel = model;
	return rowsIt->second.get();
}

static NativeDataNotificationRows* GetNativeDataNotificationRows(uint64_t rowsHandle, NativeDataModel** outModel = nullptr)
{
	auto ownerIt = nativeDataNotificationRowsModels.find(rowsHandle);
	if (ownerIt == nativeDataNotificationRowsModels.end())
		return nullptr;
	NativeDataModel* model = GetNativeDataModel(ownerIt->second);
	if (model == nullptr)
		return nullptr;
	auto rowsIt = model->notificationRows.find(rowsHandle);
	if (rowsIt == model->notificationRows.end())
		return nullptr;
	if (outModel != nullptr)
		*outModel = model;
	return rowsIt->second.get();
}

static NativeDataIconRows* GetNativeDataIconRows(uint64_t rowsHandle, NativeDataModel** outModel = nullptr)
{
	auto ownerIt = nativeDataIconRowsModels.find(rowsHandle);
	if (ownerIt == nativeDataIconRowsModels.end())
		return nullptr;
	NativeDataModel* model = GetNativeDataModel(ownerIt->second);
	if (model == nullptr)
		return nullptr;
	auto rowsIt = model->iconRows.find(rowsHandle);
	if (rowsIt == model->iconRows.end())
		return nullptr;
	if (outModel != nullptr)
		*outModel = model;
	return rowsIt->second.get();
}

static NativeDataOptionRows* GetNativeDataOptionRows(uint64_t rowsHandle, NativeDataModel** outModel = nullptr)
{
	auto ownerIt = nativeDataOptionRowsModels.find(rowsHandle);
	if (ownerIt == nativeDataOptionRowsModels.end())
		return nullptr;
	NativeDataModel* model = GetNativeDataModel(ownerIt->second);
	if (model == nullptr)
		return nullptr;
	auto rowsIt = model->optionRows.find(rowsHandle);
	if (rowsIt == model->optionRows.end())
		return nullptr;
	if (outModel != nullptr)
		*outModel = model;
	return rowsIt->second.get();
}

static NativeDataChoiceRows* GetNativeDataChoiceRows(uint64_t rowsHandle, NativeDataModel** outModel = nullptr)
{
	auto ownerIt = nativeDataChoiceRowsModels.find(rowsHandle);
	if (ownerIt == nativeDataChoiceRowsModels.end())
		return nullptr;
	NativeDataModel* model = GetNativeDataModel(ownerIt->second);
	if (model == nullptr)
		return nullptr;
	auto rowsIt = model->choiceRows.find(rowsHandle);
	if (rowsIt == model->choiceRows.end())
		return nullptr;
	if (outModel != nullptr)
		*outModel = model;
	return rowsIt->second.get();
}

static NativeDataStatusRows* GetNativeDataStatusRows(uint64_t rowsHandle, NativeDataModel** outModel = nullptr)
{
	auto ownerIt = nativeDataStatusRowsModels.find(rowsHandle);
	if (ownerIt == nativeDataStatusRowsModels.end())
		return nullptr;
	NativeDataModel* model = GetNativeDataModel(ownerIt->second);
	if (model == nullptr)
		return nullptr;
	auto rowsIt = model->statusRows.find(rowsHandle);
	if (rowsIt == model->statusRows.end())
		return nullptr;
	if (outModel != nullptr)
		*outModel = model;
	return rowsIt->second.get();
}

static NativeDataSwatchRows* GetNativeDataSwatchRows(uint64_t rowsHandle, NativeDataModel** outModel = nullptr)
{
	auto ownerIt = nativeDataSwatchRowsModels.find(rowsHandle);
	if (ownerIt == nativeDataSwatchRowsModels.end())
		return nullptr;
	NativeDataModel* model = GetNativeDataModel(ownerIt->second);
	if (model == nullptr)
		return nullptr;
	auto rowsIt = model->swatchRows.find(rowsHandle);
	if (rowsIt == model->swatchRows.end())
		return nullptr;
	if (outModel != nullptr)
		*outModel = model;
	return rowsIt->second.get();
}

static NativeDataGridRows* GetNativeDataGridRows(uint64_t rowsHandle, NativeDataModel** outModel = nullptr)
{
	auto ownerIt = nativeDataGridRowsModels.find(rowsHandle);
	if (ownerIt == nativeDataGridRowsModels.end())
		return nullptr;
	NativeDataModel* model = GetNativeDataModel(ownerIt->second);
	if (model == nullptr)
		return nullptr;
	auto rowsIt = model->gridRows.find(rowsHandle);
	if (rowsIt == model->gridRows.end())
		return nullptr;
	if (outModel != nullptr)
		*outModel = model;
	return rowsIt->second.get();
}


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
	NativeRmlEventListener(NativeCallback callback, void* userData, NativeCallback destroyCallback, Rml::Element* element)
		: callback(callback)
		, userData(userData)
		, destroyCallback(destroyCallback)
		, element(element)
	{}

	void OnAttach(Rml::Element* attachedElement) override
	{
		element = attachedElement;
	}

	void OnDetach(Rml::Element*) override
	{
		if (destroyCallback != nullptr) {
			destroyCallback(userData);
		}
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
	NativeCallback destroyCallback = nullptr;
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
	if (result->success) {
		nativeContextNames.emplace(query->name);
	}
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

static void NativeContextPullToFront(const RmlContextHandleQuery* query, RmlContextBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->success = RmlGui::PullContextToFront(context);
	if (!result->success)
		result->error = &INVALID_ARGUMENT_ERROR;
}

static void NativeContextSetPointerCapture(const RmlContextPointerCaptureQuery* query, RmlContextBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->success = RmlGui::SetPointerCapture(
		context, query->anchorX, query->anchorY, query->active);
	if (!result->success)
		result->error = &INVALID_ARGUMENT_ERROR;
}

static void NativeContextTakePointerCaptureDelta(const RmlContextHandleQuery* query, RmlContextPointerDeltaResult* result)
{
	result->error = nullptr;
	result->deltaX = 0;
	result->deltaY = 0;
	result->status = 0;
	Rml::Context* context = FromHandle(query->contextHandle);
	if (context == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	if (!RmlGui::TakePointerCaptureDelta(context, result->deltaX, result->deltaY, result->status))
		result->error = &INVALID_ARGUMENT_ERROR;
}

static void NativeRemoveContext(const RmlRemoveContextQuery* query, RmlRemoveContextResult* result)
{
	result->error = nullptr;
	Rml::Context* context = FromHandle(query->contextHandle);
	result->success = (context != nullptr);
	if (context != nullptr) {
		EraseNativeDataModelHandles(context);
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
		EraseNativeDataModelHandles(context);
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

	const auto phase = query->options.hasDefaultPhase ? static_cast<Rml::DefaultActionPhase>(query->options.defaultPhase) : Rml::DefaultActionPhase::None;
	result->eventID = static_cast<int32_t>(Rml::RegisterEventType(query->eventType, query->options.interruptible, query->options.bubbles, phase));
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

static void NativeGetDocumentPathRequests(const RmlGetDocumentPathRequestsQuery* query, RmlGetDocumentPathRequestsResult* result)
{
	result->error = nullptr;
	result->paths = nullptr;
	result->count = 0;

	if (query->documentPath == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	static thread_local std::vector<std::string> pathStorage;
	static thread_local std::vector<const char*> pathPointers;
	pathStorage = RmlGui::GetDocumentPathRequests(query->documentPath);
	pathPointers.clear();
	pathPointers.reserve(pathStorage.size());
	for (const auto& path : pathStorage)
		pathPointers.push_back(path.c_str());

	result->paths = pathPointers.data();
	result->count = static_cast<uint32_t>(pathPointers.size());
}

static void NativeClearDocumentPathRequests(const RmlClearDocumentPathRequestsQuery* query, RmlClearDocumentPathRequestsResult* result)
{
	result->error = nullptr;
	result->success = false;

	if (query->documentPath == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	if (!IsReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	result->success = RmlGui::ClearDocumentPathRequests(query->documentPath);
	if (!result->success)
		result->error = &NOT_READY_ERROR;
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
	result->documentHandle = ToElementHandle(document);
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
	result->documentHandle = ToElementHandle(document);
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
	result->documentHandle = ToElementHandle(document);
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

	auto* listener = new NativeRmlEventListener(query->callback, query->userData, query->destroyCallback, context->GetRootElement());
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
	RmlGui::ClearDebugContext(context);
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
	if (result->success) {
		EraseNativeDataModelHandles(context, query->name);
	}
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
	nativeDataModels.emplace(handle, NativeDataModelRecord{
		.context = context,
		.name = query->name,
		.handle = constructor.GetModelHandle(),
	});
	result->dataModelHandle = handle;
	result->success = true;
}

static void NativeContextCreateDataModel(const RmlContextCreateDataModelQuery* query, RmlContextOpenDataModelResult* result)
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
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const uint64_t dataModelHandle = nextDataModelHandle++;
	nativeDataModels.emplace(dataModelHandle, NativeDataModelRecord{
		.context = context,
		.name = query->name,
		.handle = constructor.GetModelHandle(),
		.native = std::make_unique<NativeDataModel>(std::move(constructor), context),
	});
	result->dataModelHandle = dataModelHandle;
	result->success = true;
}

static void NativeDataModelBindBool(const RmlDataModelBindBoolQuery* query, RmlDataModelBindResult* result)
{
	result->error = nullptr;
	result->variableHandle = 0;
	result->success = false;
	NativeDataModel* model = GetNativeDataModel(query->dataModelHandle);
	if (model == nullptr || query->name == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	auto variable = std::make_unique<NativeDataVariable>();
	variable->type = NativeDataValueType::Bool;
	variable->name = query->name;
	variable->boolValue = query->initialValue;
	if (!model->constructor.Bind(variable->name, &variable->boolValue)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	const uint64_t variableHandle = nextDataModelVariableHandle++;
	model->variables.emplace(variableHandle, std::move(variable));
	nativeDataVariableModels.emplace(variableHandle, query->dataModelHandle);
	result->variableHandle = variableHandle;
	result->success = true;
}

static void NativeDataModelBindInt(const RmlDataModelBindIntQuery* query, RmlDataModelBindResult* result)
{
	result->error = nullptr;
	result->variableHandle = 0;
	result->success = false;
	NativeDataModel* model = GetNativeDataModel(query->dataModelHandle);
	if (model == nullptr || query->name == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	auto variable = std::make_unique<NativeDataVariable>();
	variable->type = NativeDataValueType::Int;
	variable->name = query->name;
	variable->intValue = query->initialValue;
	if (!model->constructor.Bind(variable->name, &variable->intValue)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	const uint64_t variableHandle = nextDataModelVariableHandle++;
	model->variables.emplace(variableHandle, std::move(variable));
	nativeDataVariableModels.emplace(variableHandle, query->dataModelHandle);
	result->variableHandle = variableHandle;
	result->success = true;
}

static void NativeDataModelBindFloat(const RmlDataModelBindFloatQuery* query, RmlDataModelBindResult* result)
{
	result->error = nullptr;
	result->variableHandle = 0;
	result->success = false;
	NativeDataModel* model = GetNativeDataModel(query->dataModelHandle);
	if (model == nullptr || query->name == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	auto variable = std::make_unique<NativeDataVariable>();
	variable->type = NativeDataValueType::Float;
	variable->name = query->name;
	variable->floatValue = query->initialValue;
	if (!model->constructor.Bind(variable->name, &variable->floatValue)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	const uint64_t variableHandle = nextDataModelVariableHandle++;
	model->variables.emplace(variableHandle, std::move(variable));
	nativeDataVariableModels.emplace(variableHandle, query->dataModelHandle);
	result->variableHandle = variableHandle;
	result->success = true;
}

static void NativeDataModelBindString(const RmlDataModelBindStringQuery* query, RmlDataModelBindResult* result)
{
	result->error = nullptr;
	result->variableHandle = 0;
	result->success = false;
	NativeDataModel* model = GetNativeDataModel(query->dataModelHandle);
	if (model == nullptr || query->name == nullptr || query->initialValue == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	auto variable = std::make_unique<NativeDataVariable>();
	variable->type = NativeDataValueType::String;
	variable->name = query->name;
	variable->stringValue = query->initialValue;
	if (!model->constructor.Bind(variable->name, &variable->stringValue)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	const uint64_t variableHandle = nextDataModelVariableHandle++;
	model->variables.emplace(variableHandle, std::move(variable));
	nativeDataVariableModels.emplace(variableHandle, query->dataModelHandle);
	result->variableHandle = variableHandle;
	result->success = true;
}

static bool RegisterNativeColourType(NativeDataModel* model)
{
	if (!nativeColourTypes.insert(model->context).second)
		return true;
	if (!model->constructor.RegisterScalar<Rml::Colourb>([](const Rml::Colourb& colour, Rml::Variant& value) {
		value = Rml::ToString(colour);
	})) {
		nativeColourTypes.erase(model->context);
		return false;
	}
	return true;
}

static void NativeDataModelBindColor(const RmlDataModelBindColorQuery* query, RmlDataModelBindResult* result)
{
	result->error = nullptr;
	result->variableHandle = 0;
	result->success = false;
	NativeDataModel* model = GetNativeDataModel(query->dataModelHandle);
	if (model == nullptr || query->name == nullptr || !RegisterNativeColourType(model)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	auto variable = std::make_unique<NativeDataVariable>();
	variable->type = NativeDataValueType::Color;
	variable->name = query->name;
	variable->colourValue = Rml::Colourb(query->red, query->green, query->blue, query->alpha);
	if (!model->constructor.Bind(variable->name, &variable->colourValue)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	const uint64_t variableHandle = nextDataModelVariableHandle++;
	model->variables.emplace(variableHandle, std::move(variable));
	nativeDataVariableModels.emplace(variableHandle, query->dataModelHandle);
	result->variableHandle = variableHandle;
	result->success = true;
}

static bool RegisterNativePixelType(NativeDataModel* model)
{
	if (!nativePixelTypes.insert(model->context).second)
		return true;
	if (!model->constructor.RegisterScalar<NativeDataPixels>([](const NativeDataPixels& pixels, Rml::Variant& value) {
		value = Rml::ToString(pixels.value) + "px";
	})) {
		nativePixelTypes.erase(model->context);
		return false;
	}
	return true;
}

static bool RegisterNativePercentType(NativeDataModel* model)
{
	if (!nativePercentTypes.insert(model->context).second)
		return true;
	if (!model->constructor.RegisterScalar<NativeDataPercent>([](const NativeDataPercent& percent, Rml::Variant& value) {
		value = Rml::ToString(percent.value) + "%";
	})) {
		nativePercentTypes.erase(model->context);
		return false;
	}
	return true;
}

static void NativeDataModelBindPixels(const RmlDataModelBindPixelsQuery* query, RmlDataModelBindResult* result)
{
	result->error = nullptr;
	result->variableHandle = 0;
	result->success = false;
	NativeDataModel* model = GetNativeDataModel(query->dataModelHandle);
	if (model == nullptr || query->name == nullptr || !RegisterNativePixelType(model)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	auto variable = std::make_unique<NativeDataVariable>();
	variable->type = NativeDataValueType::Pixels;
	variable->name = query->name;
	variable->pixelsValue.value = query->initialValue;
	if (!model->constructor.Bind(variable->name, &variable->pixelsValue)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	const uint64_t variableHandle = nextDataModelVariableHandle++;
	model->variables.emplace(variableHandle, std::move(variable));
	nativeDataVariableModels.emplace(variableHandle, query->dataModelHandle);
	result->variableHandle = variableHandle;
	result->success = true;
}

static void NativeDataModelBindPercent(const RmlDataModelBindPercentQuery* query, RmlDataModelBindResult* result)
{
	result->error = nullptr;
	result->variableHandle = 0;
	result->success = false;
	NativeDataModel* model = GetNativeDataModel(query->dataModelHandle);
	if (model == nullptr || query->name == nullptr || !RegisterNativePercentType(model)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	auto variable = std::make_unique<NativeDataVariable>();
	variable->type = NativeDataValueType::Percent;
	variable->name = query->name;
	variable->percentValue.value = query->initialValue;
	if (!model->constructor.Bind(variable->name, &variable->percentValue)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	const uint64_t variableHandle = nextDataModelVariableHandle++;
	model->variables.emplace(variableHandle, std::move(variable));
	nativeDataVariableModels.emplace(variableHandle, query->dataModelHandle);
	result->variableHandle = variableHandle;
	result->success = true;
}

static bool RegisterNativeTextRowTypes(NativeDataModel* model)
{
	if (!nativeTextRowTypes.insert(model->context).second)
		return true;

	auto row = model->constructor.RegisterStruct<NativeDataTextRow>();
	if (!row
		|| !row.RegisterMember("text", &NativeDataTextRow::text)
		|| !row.RegisterMember("muted", &NativeDataTextRow::muted)
		|| !row.RegisterMember("visible", &NativeDataTextRow::visible)
		|| !model->constructor.RegisterArray<std::vector<NativeDataTextRow>>()) {
		nativeTextRowTypes.erase(model->context);
		return false;
	}
	return true;
}

static void NativeDataModelBindTextRows(const RmlDataModelBindTextRowsQuery* query, RmlDataModelTextRowsResult* result)
{
	result->error = nullptr;
	result->rowsHandle = 0;
	result->success = false;
	NativeDataModel* model = GetNativeDataModel(query->dataModelHandle);
	if (model == nullptr || query->name == nullptr || !RegisterNativeTextRowTypes(model)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	auto rows = std::make_unique<NativeDataTextRows>();
	rows->name = query->name;
	if (!model->constructor.Bind(rows->name, &rows->rows)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	const uint64_t rowsHandle = nextDataModelVariableHandle++;
	model->textRows.emplace(rowsHandle, std::move(rows));
	nativeDataTextRowsModels.emplace(rowsHandle, query->dataModelHandle);
	result->rowsHandle = rowsHandle;
	result->success = true;
}

static bool RegisterNativeLogRowTypes(NativeDataModel* model)
{
	if (!nativeLogRowTypes.insert(model->context).second)
		return true;

	auto row = model->constructor.RegisterStruct<NativeDataLogRow>();
	if (!row
		|| !row.RegisterMember("text", &NativeDataLogRow::text)
		|| !row.RegisterMember("info", &NativeDataLogRow::info)
		|| !row.RegisterMember("warning", &NativeDataLogRow::warning)
		|| !row.RegisterMember("error", &NativeDataLogRow::error)
		|| !row.RegisterMember("selected", &NativeDataLogRow::selected)
		|| !row.RegisterMember("visible", &NativeDataLogRow::visible)
		|| !model->constructor.RegisterArray<std::vector<NativeDataLogRow>>()) {
		nativeLogRowTypes.erase(model->context);
		return false;
	}
	return true;
}

static void NativeDataModelBindLogRows(const RmlDataModelBindLogRowsQuery* query, RmlDataModelLogRowsResult* result)
{
	result->error = nullptr;
	result->rowsHandle = 0;
	result->success = false;
	NativeDataModel* model = GetNativeDataModel(query->dataModelHandle);
	if (model == nullptr || query->name == nullptr || !RegisterNativeLogRowTypes(model)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	auto rows = std::make_unique<NativeDataLogRows>();
	rows->name = query->name;
	if (!model->constructor.Bind(rows->name, &rows->rows)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	const uint64_t rowsHandle = nextDataModelVariableHandle++;
	model->logRows.emplace(rowsHandle, std::move(rows));
	nativeDataLogRowsModels.emplace(rowsHandle, query->dataModelHandle);
	result->rowsHandle = rowsHandle;
	result->success = true;
}

static bool RegisterNativeNotificationRowTypes(NativeDataModel* model)
{
	if (!nativeNotificationRowTypes.insert(model->context).second)
		return true;

	if (!RegisterNativePercentType(model)) {
		nativeNotificationRowTypes.erase(model->context);
		return false;
	}
	auto row = model->constructor.RegisterStruct<NativeDataNotificationRow>();
	if (!row
		|| !row.RegisterMember("title", &NativeDataNotificationRow::title)
		|| !row.RegisterMember("body", &NativeDataNotificationRow::body)
		|| !row.RegisterMember("warning", &NativeDataNotificationRow::warning)
		|| !row.RegisterMember("has_progress", &NativeDataNotificationRow::hasProgress)
		|| !row.RegisterMember("progress", &NativeDataNotificationRow::progress)
		|| !model->constructor.RegisterArray<std::vector<NativeDataNotificationRow>>()) {
		nativeNotificationRowTypes.erase(model->context);
		return false;
	}
	return true;
}

static void NativeDataModelBindNotificationRows(const RmlDataModelBindNotificationRowsQuery* query, RmlDataModelNotificationRowsResult* result)
{
	result->error = nullptr;
	result->rowsHandle = 0;
	result->success = false;
	NativeDataModel* model = GetNativeDataModel(query->dataModelHandle);
	if (model == nullptr || query->name == nullptr || !RegisterNativeNotificationRowTypes(model)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	auto rows = std::make_unique<NativeDataNotificationRows>();
	rows->name = query->name;
	if (!model->constructor.Bind(rows->name, &rows->rows)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	const uint64_t rowsHandle = nextDataModelVariableHandle++;
	model->notificationRows.emplace(rowsHandle, std::move(rows));
	nativeDataNotificationRowsModels.emplace(rowsHandle, query->dataModelHandle);
	result->rowsHandle = rowsHandle;
	result->success = true;
}

static bool RegisterNativeIconRowTypes(NativeDataModel* model)
{
	if (!nativeIconRowTypes.insert(model->context).second)
		return true;

	auto row = model->constructor.RegisterStruct<NativeDataIconRow>();
	if (!row
		|| !row.RegisterMember("label", &NativeDataIconRow::label)
		|| !row.RegisterMember("icon", &NativeDataIconRow::icon)
		|| !row.RegisterMember("tooltip", &NativeDataIconRow::tooltip)
		|| !row.RegisterMember("pressed", &NativeDataIconRow::pressed)
		|| !row.RegisterMember("disabled", &NativeDataIconRow::disabled)
		|| !row.RegisterMember("visible", &NativeDataIconRow::visible)
		|| !model->constructor.RegisterArray<std::vector<NativeDataIconRow>>()) {
		nativeIconRowTypes.erase(model->context);
		return false;
	}
	return true;
}

static void NativeDataModelBindIconRows(const RmlDataModelBindIconRowsQuery* query, RmlDataModelIconRowsResult* result)
{
	result->error = nullptr;
	result->rowsHandle = 0;
	result->success = false;
	NativeDataModel* model = GetNativeDataModel(query->dataModelHandle);
	if (model == nullptr || query->name == nullptr || !RegisterNativeIconRowTypes(model)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	auto rows = std::make_unique<NativeDataIconRows>();
	rows->name = query->name;
	if (!model->constructor.Bind(rows->name, &rows->rows)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	const uint64_t rowsHandle = nextDataModelVariableHandle++;
	model->iconRows.emplace(rowsHandle, std::move(rows));
	nativeDataIconRowsModels.emplace(rowsHandle, query->dataModelHandle);
	result->rowsHandle = rowsHandle;
	result->success = true;
}

static bool RegisterNativeOptionRowTypes(NativeDataModel* model)
{
	if (!nativeOptionRowTypes.insert(model->context).second)
		return true;

	auto row = model->constructor.RegisterStruct<NativeDataOptionRow>();
	if (!row
		|| !row.RegisterMember("value", &NativeDataOptionRow::value)
		|| !row.RegisterMember("label", &NativeDataOptionRow::label)
		|| !row.RegisterMember("visible", &NativeDataOptionRow::visible)
		|| !model->constructor.RegisterArray<std::vector<NativeDataOptionRow>>()) {
		nativeOptionRowTypes.erase(model->context);
		return false;
	}
	return true;
}

static void NativeDataModelBindOptionRows(const RmlDataModelBindOptionRowsQuery* query, RmlDataModelOptionRowsResult* result)
{
	result->error = nullptr;
	result->rowsHandle = 0;
	result->success = false;
	NativeDataModel* model = GetNativeDataModel(query->dataModelHandle);
	if (model == nullptr || query->name == nullptr || !RegisterNativeOptionRowTypes(model)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	auto rows = std::make_unique<NativeDataOptionRows>();
	rows->name = query->name;
	if (!model->constructor.Bind(rows->name, &rows->rows)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	const uint64_t rowsHandle = nextDataModelVariableHandle++;
	model->optionRows.emplace(rowsHandle, std::move(rows));
	nativeDataOptionRowsModels.emplace(rowsHandle, query->dataModelHandle);
	result->rowsHandle = rowsHandle;
	result->success = true;
}

static bool RegisterNativeChoiceRowTypes(NativeDataModel* model)
{
	if (!nativeChoiceRowTypes.insert(model->context).second)
		return true;

	auto row = model->constructor.RegisterStruct<NativeDataChoiceRow>();
	if (!row
		|| !row.RegisterMember("label", &NativeDataChoiceRow::label)
		|| !row.RegisterMember("detail", &NativeDataChoiceRow::detail)
		|| !row.RegisterMember("selected", &NativeDataChoiceRow::selected)
		|| !row.RegisterMember("highlighted", &NativeDataChoiceRow::highlighted)
		|| !row.RegisterMember("visible", &NativeDataChoiceRow::visible)
		|| !model->constructor.RegisterArray<std::vector<NativeDataChoiceRow>>()) {
		nativeChoiceRowTypes.erase(model->context);
		return false;
	}
	return true;
}

static void NativeDataModelBindChoiceRows(const RmlDataModelBindChoiceRowsQuery* query, RmlDataModelChoiceRowsResult* result)
{
	result->error = nullptr;
	result->rowsHandle = 0;
	result->success = false;
	NativeDataModel* model = GetNativeDataModel(query->dataModelHandle);
	if (model == nullptr || query->name == nullptr || !RegisterNativeChoiceRowTypes(model)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	auto rows = std::make_unique<NativeDataChoiceRows>();
	rows->name = query->name;
	if (!model->constructor.Bind(rows->name, &rows->rows)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	const uint64_t rowsHandle = nextDataModelVariableHandle++;
	model->choiceRows.emplace(rowsHandle, std::move(rows));
	nativeDataChoiceRowsModels.emplace(rowsHandle, query->dataModelHandle);
	result->rowsHandle = rowsHandle;
	result->success = true;
}

static bool RegisterNativeStatusRowTypes(NativeDataModel* model)
{
	if (!nativeStatusRowTypes.insert(model->context).second)
		return true;

	auto row = model->constructor.RegisterStruct<NativeDataStatusRow>();
	if (!row
		|| !row.RegisterMember("label", &NativeDataStatusRow::label)
		|| !row.RegisterMember("positive", &NativeDataStatusRow::positive)
		|| !row.RegisterMember("visible", &NativeDataStatusRow::visible)
		|| !model->constructor.RegisterArray<std::vector<NativeDataStatusRow>>()) {
		nativeStatusRowTypes.erase(model->context);
		return false;
	}
	return true;
}

static void NativeDataModelBindStatusRows(const RmlDataModelBindStatusRowsQuery* query, RmlDataModelStatusRowsResult* result)
{
	result->error = nullptr;
	result->rowsHandle = 0;
	result->success = false;
	NativeDataModel* model = GetNativeDataModel(query->dataModelHandle);
	if (model == nullptr || query->name == nullptr || !RegisterNativeStatusRowTypes(model)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	auto rows = std::make_unique<NativeDataStatusRows>();
	rows->name = query->name;
	if (!model->constructor.Bind(rows->name, &rows->rows)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	const uint64_t rowsHandle = nextDataModelVariableHandle++;
	model->statusRows.emplace(rowsHandle, std::move(rows));
	nativeDataStatusRowsModels.emplace(rowsHandle, query->dataModelHandle);
	result->rowsHandle = rowsHandle;
	result->success = true;
}

static bool RegisterNativeSwatchRowTypes(NativeDataModel* model)
{
	if (!nativeSwatchRowTypes.insert(model->context).second)
		return true;

	if (!RegisterNativeColourType(model)) {
		nativeSwatchRowTypes.erase(model->context);
		return false;
	}
	auto row = model->constructor.RegisterStruct<NativeDataSwatchRow>();
	if (!row
		|| !row.RegisterMember("label", &NativeDataSwatchRow::label)
		|| !row.RegisterMember("colour", &NativeDataSwatchRow::colour)
		|| !row.RegisterMember("actions_enabled", &NativeDataSwatchRow::actionsEnabled)
		|| !row.RegisterMember("visible", &NativeDataSwatchRow::visible)
		|| !model->constructor.RegisterArray<std::vector<NativeDataSwatchRow>>()) {
		nativeSwatchRowTypes.erase(model->context);
		return false;
	}
	return true;
}

static void NativeDataModelBindSwatchRows(const RmlDataModelBindSwatchRowsQuery* query, RmlDataModelSwatchRowsResult* result)
{
	result->error = nullptr;
	result->rowsHandle = 0;
	result->success = false;
	NativeDataModel* model = GetNativeDataModel(query->dataModelHandle);
	if (model == nullptr || query->name == nullptr || !RegisterNativeSwatchRowTypes(model)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	auto rows = std::make_unique<NativeDataSwatchRows>();
	rows->name = query->name;
	if (!model->constructor.Bind(rows->name, &rows->rows)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	const uint64_t rowsHandle = nextDataModelVariableHandle++;
	model->swatchRows.emplace(rowsHandle, std::move(rows));
	nativeDataSwatchRowsModels.emplace(rowsHandle, query->dataModelHandle);
	result->rowsHandle = rowsHandle;
	result->success = true;
}

static bool RegisterNativeGridRowTypes(NativeDataModel* model)
{
	if (!RegisterNativePixelType(model) || !nativeGridRowTypes.insert(model->context).second)
		return true;

	auto row = model->constructor.RegisterStruct<NativeDataGridRow>();
	if (!row
		|| !row.RegisterMember("label", &NativeDataGridRow::label)
		|| !row.RegisterMember("image", &NativeDataGridRow::image)
		|| !row.RegisterMember("cell_size", &NativeDataGridRow::cellSize)
		|| !row.RegisterMember("has_image", &NativeDataGridRow::hasImage)
		|| !row.RegisterMember("native_image", &NativeDataGridRow::nativeImage)
		|| !row.RegisterMember("selected", &NativeDataGridRow::selected)
		|| !row.RegisterMember("folder", &NativeDataGridRow::folder)
		|| !row.RegisterMember("filler", &NativeDataGridRow::filler)
		|| !row.RegisterMember("visible", &NativeDataGridRow::visible)
		|| !model->constructor.RegisterArray<std::vector<NativeDataGridRow>>()) {
		nativeGridRowTypes.erase(model->context);
		return false;
	}
	return true;
}

static void NativeDataModelBindGridRows(const RmlDataModelBindGridRowsQuery* query, RmlDataModelGridRowsResult* result)
{
	result->error = nullptr;
	result->rowsHandle = 0;
	result->success = false;
	NativeDataModel* model = GetNativeDataModel(query->dataModelHandle);
	if (model == nullptr || query->name == nullptr || !RegisterNativeGridRowTypes(model)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	auto rows = std::make_unique<NativeDataGridRows>();
	rows->name = query->name;
	if (!model->constructor.Bind(rows->name, &rows->rows)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	const uint64_t rowsHandle = nextDataModelVariableHandle++;
	model->gridRows.emplace(rowsHandle, std::move(rows));
	nativeDataGridRowsModels.emplace(rowsHandle, query->dataModelHandle);
	result->rowsHandle = rowsHandle;
	result->success = true;
}

static void NativeDataModelSetBool(const RmlDataModelVariableBoolQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	NativeDataModel* model = nullptr;
	NativeDataVariable* variable = GetNativeDataVariable(query->variableHandle, NativeDataValueType::Bool, &model);
	if (variable == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	variable->boolValue = query->value;
	model->handle.DirtyVariable(variable->name);
	result->success = true;
}

static void NativeDataModelSetInt(const RmlDataModelVariableIntQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	NativeDataModel* model = nullptr;
	NativeDataVariable* variable = GetNativeDataVariable(query->variableHandle, NativeDataValueType::Int, &model);
	if (variable == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	variable->intValue = query->value;
	model->handle.DirtyVariable(variable->name);
	result->success = true;
}

static void NativeDataModelSetFloat(const RmlDataModelVariableFloatQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	NativeDataModel* model = nullptr;
	NativeDataVariable* variable = GetNativeDataVariable(query->variableHandle, NativeDataValueType::Float, &model);
	if (variable == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	variable->floatValue = query->value;
	model->handle.DirtyVariable(variable->name);
	result->success = true;
}

static void NativeDataModelSetString(const RmlDataModelVariableStringQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	NativeDataModel* model = nullptr;
	NativeDataVariable* variable = GetNativeDataVariable(query->variableHandle, NativeDataValueType::String, &model);
	if (variable == nullptr || query->value == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	variable->stringValue = query->value;
	model->handle.DirtyVariable(variable->name);
	result->success = true;
}

static void NativeDataModelSetColor(const RmlDataModelVariableColorQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	NativeDataModel* model = nullptr;
	NativeDataVariable* variable = GetNativeDataVariable(query->variableHandle, NativeDataValueType::Color, &model);
	if (variable == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	variable->colourValue = Rml::Colourb(query->red, query->green, query->blue, query->alpha);
	model->handle.DirtyVariable(variable->name);
	result->success = true;
}

static void NativeDataModelSetPixels(const RmlDataModelVariablePixelsQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	NativeDataModel* model = nullptr;
	NativeDataVariable* variable = GetNativeDataVariable(query->variableHandle, NativeDataValueType::Pixels, &model);
	if (variable == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	variable->pixelsValue.value = query->value;
	model->handle.DirtyVariable(variable->name);
	result->success = true;
}

static void NativeDataModelSetPercent(const RmlDataModelVariablePercentQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	NativeDataModel* model = nullptr;
	NativeDataVariable* variable = GetNativeDataVariable(query->variableHandle, NativeDataValueType::Percent, &model);
	if (variable == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	variable->percentValue.value = query->value;
	model->handle.DirtyVariable(variable->name);
	result->success = true;
}

static void NativeDataModelSetTextRows(const RmlDataModelSetTextRowsQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	NativeDataModel* model = nullptr;
	NativeDataTextRows* textRows = GetNativeDataTextRows(query->rowsHandle, &model);
	if (textRows == nullptr || (query->count > 0 && query->rows == nullptr)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	std::vector<NativeDataTextRow> copiedRows;
	copiedRows.reserve(query->count);
	for (uint64_t index = 0; index < query->count; ++index) {
		const RmlDataTextRow& row = query->rows[index];
		if (row.text == nullptr) {
			result->error = &INVALID_ARGUMENT_ERROR;
			return;
		}
		copiedRows.push_back(NativeDataTextRow{ .text = row.text, .muted = row.muted, .visible = row.visible });
	}
	textRows->rows = std::move(copiedRows);
	model->handle.DirtyVariable(textRows->name);
	result->success = true;
}

static void NativeDataModelSetLogRows(const RmlDataModelSetLogRowsQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	NativeDataModel* model = nullptr;
	NativeDataLogRows* logRows = GetNativeDataLogRows(query->rowsHandle, &model);
	if (logRows == nullptr || (query->count > 0 && query->rows == nullptr)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	std::vector<NativeDataLogRow> copiedRows;
	copiedRows.reserve(query->count);
	for (uint64_t index = 0; index < query->count; ++index) {
		const RmlDataLogRow& row = query->rows[index];
		if (row.text == nullptr || row.severity > 2) {
			result->error = &INVALID_ARGUMENT_ERROR;
			return;
		}
		copiedRows.push_back(NativeDataLogRow{
			.text = row.text,
			.info = row.severity == 0,
			.warning = row.severity == 1,
			.error = row.severity == 2,
			.selected = row.selected,
			.visible = row.text[0] != '\0',
		});
	}
	logRows->rows = std::move(copiedRows);
	model->handle.DirtyVariable(logRows->name);
	result->success = true;
}

static void NativeDataModelSetNotificationRows(const RmlDataModelSetNotificationRowsQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	NativeDataModel* model = nullptr;
	NativeDataNotificationRows* notificationRows = GetNativeDataNotificationRows(query->rowsHandle, &model);
	if (notificationRows == nullptr || (query->count > 0 && query->rows == nullptr)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	std::vector<NativeDataNotificationRow> copiedRows;
	copiedRows.reserve(query->count);
	for (uint64_t index = 0; index < query->count; ++index) {
		const RmlDataNotificationRow& row = query->rows[index];
		if (row.title == nullptr || row.body == nullptr) {
			result->error = &INVALID_ARGUMENT_ERROR;
			return;
		}
		copiedRows.push_back(NativeDataNotificationRow{
			.title = row.title,
			.body = row.body,
			.warning = row.warning,
			.hasProgress = row.hasProgress,
			.progress = {.value = row.progress},
		});
	}
	notificationRows->rows = std::move(copiedRows);
	model->handle.DirtyVariable(notificationRows->name);
	result->success = true;
}

static void NativeDataModelSetIconRows(const RmlDataModelSetIconRowsQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	NativeDataModel* model = nullptr;
	NativeDataIconRows* iconRows = GetNativeDataIconRows(query->rowsHandle, &model);
	if (iconRows == nullptr || (query->count > 0 && query->rows == nullptr)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	std::vector<NativeDataIconRow> copiedRows;
	copiedRows.reserve(query->count);
	for (uint64_t index = 0; index < query->count; ++index) {
		const RmlDataIconRow& row = query->rows[index];
		if (row.label == nullptr || row.icon == nullptr || row.tooltip == nullptr) {
			result->error = &INVALID_ARGUMENT_ERROR;
			return;
		}
		copiedRows.push_back(NativeDataIconRow{
			.label = row.label,
			.icon = row.icon,
			.tooltip = row.tooltip,
			.pressed = row.pressed,
			.disabled = row.disabled,
			.visible = row.label[0] != '\0',
		});
	}
	iconRows->rows = std::move(copiedRows);
	model->handle.DirtyVariable(iconRows->name);
	result->success = true;
}

static void NativeDataModelSetOptionRows(const RmlDataModelSetOptionRowsQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	NativeDataModel* model = nullptr;
	NativeDataOptionRows* optionRows = GetNativeDataOptionRows(query->rowsHandle, &model);
	if (optionRows == nullptr || (query->count > 0 && query->rows == nullptr)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	std::vector<NativeDataOptionRow> copiedRows;
	copiedRows.reserve(query->count);
	for (uint64_t index = 0; index < query->count; ++index) {
		const RmlDataOptionRow& row = query->rows[index];
		if (row.value == nullptr || row.label == nullptr) {
			result->error = &INVALID_ARGUMENT_ERROR;
			return;
		}
		copiedRows.push_back(NativeDataOptionRow{
			.value = row.value,
			.label = row.label,
			.visible = row.label[0] != '\0',
		});
	}
	optionRows->rows = std::move(copiedRows);
	model->handle.DirtyVariable(optionRows->name);
	result->success = true;
}

static void NativeDataModelSetChoiceRows(const RmlDataModelSetChoiceRowsQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	NativeDataModel* model = nullptr;
	NativeDataChoiceRows* choiceRows = GetNativeDataChoiceRows(query->rowsHandle, &model);
	if (choiceRows == nullptr || (query->count > 0 && query->rows == nullptr)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	std::vector<NativeDataChoiceRow> copiedRows;
	copiedRows.reserve(query->count);
	for (uint64_t index = 0; index < query->count; ++index) {
		const RmlDataChoiceRow& row = query->rows[index];
		if (row.label == nullptr || row.detail == nullptr) {
			result->error = &INVALID_ARGUMENT_ERROR;
			return;
		}
		copiedRows.push_back(NativeDataChoiceRow{
			.label = row.label,
			.detail = row.detail,
			.selected = row.selected,
			.highlighted = row.highlighted,
			.visible = row.label[0] != '\0',
		});
	}
	choiceRows->rows = std::move(copiedRows);
	model->handle.DirtyVariable(choiceRows->name);
	result->success = true;
}

static void NativeDataModelSetStatusRows(const RmlDataModelSetStatusRowsQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	NativeDataModel* model = nullptr;
	NativeDataStatusRows* statusRows = GetNativeDataStatusRows(query->rowsHandle, &model);
	if (statusRows == nullptr || (query->count > 0 && query->rows == nullptr)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	std::vector<NativeDataStatusRow> copiedRows;
	copiedRows.reserve(query->count);
	for (uint64_t index = 0; index < query->count; ++index) {
		const RmlDataStatusRow& row = query->rows[index];
		if (row.label == nullptr) {
			result->error = &INVALID_ARGUMENT_ERROR;
			return;
		}
		copiedRows.push_back(NativeDataStatusRow{
			.label = row.label,
			.positive = row.positive,
			.visible = row.label[0] != '\0',
		});
	}
	statusRows->rows = std::move(copiedRows);
	model->handle.DirtyVariable(statusRows->name);
	result->success = true;
}

static void NativeDataModelSetSwatchRows(const RmlDataModelSetSwatchRowsQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	NativeDataModel* model = nullptr;
	NativeDataSwatchRows* swatchRows = GetNativeDataSwatchRows(query->rowsHandle, &model);
	if (swatchRows == nullptr || (query->count > 0 && query->rows == nullptr)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	std::vector<NativeDataSwatchRow> copiedRows;
	copiedRows.reserve(query->count);
	for (uint64_t index = 0; index < query->count; ++index) {
		const RmlDataSwatchRow& row = query->rows[index];
		if (row.label == nullptr) {
			result->error = &INVALID_ARGUMENT_ERROR;
			return;
		}
		copiedRows.push_back(NativeDataSwatchRow{
			.label = row.label,
			.colour = Rml::Colourb(row.red, row.green, row.blue, row.alpha),
			.actionsEnabled = row.actionsEnabled,
			.visible = row.label[0] != '\0',
		});
	}
	swatchRows->rows = std::move(copiedRows);
	model->handle.DirtyVariable(swatchRows->name);
	result->success = true;
}

static void NativeDataModelSetGridRows(const RmlDataModelSetGridRowsQuery* query, RmlElementBoolResult* result)
{
	result->error = nullptr;
	result->success = false;
	NativeDataModel* model = nullptr;
	NativeDataGridRows* gridRows = GetNativeDataGridRows(query->rowsHandle, &model);
	if (gridRows == nullptr || (query->count > 0 && query->rows == nullptr)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	std::vector<NativeDataGridRow> copiedRows;
	copiedRows.reserve(query->count);
	for (uint64_t index = 0; index < query->count; ++index) {
		const RmlDataGridRow& row = query->rows[index];
		if (row.label == nullptr || row.image == nullptr) {
			result->error = &INVALID_ARGUMENT_ERROR;
			return;
		}
		copiedRows.push_back(NativeDataGridRow{
			.label = row.label,
			.image = row.image,
			.cellSize = NativeDataPixels{.value = row.cellSize},
			.hasImage = row.hasImage,
			.nativeImage = row.nativeImage,
			.selected = row.selected,
			.folder = row.folder,
			.filler = row.filler,
			.visible = row.filler || row.label[0] != '\0',
		});
	}
	gridRows->rows = std::move(copiedRows);
	model->handle.DirtyVariable(gridRows->name);
	result->success = true;
}

static void NativeDataModelGetBool(const RmlDataModelVariableHandleQuery* query, RmlDataModelGetBoolResult* result)
{
	result->error = nullptr;
	result->value = false;
	result->success = false;
	NativeDataVariable* variable = GetNativeDataVariable(query->variableHandle, NativeDataValueType::Bool);
	if (variable == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->value = variable->boolValue;
	result->success = true;
}

static void NativeDataModelGetInt(const RmlDataModelVariableHandleQuery* query, RmlDataModelGetIntResult* result)
{
	result->error = nullptr;
	result->value = 0;
	result->success = false;
	NativeDataVariable* variable = GetNativeDataVariable(query->variableHandle, NativeDataValueType::Int);
	if (variable == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->value = variable->intValue;
	result->success = true;
}

static void NativeDataModelGetFloat(const RmlDataModelVariableHandleQuery* query, RmlDataModelGetFloatResult* result)
{
	result->error = nullptr;
	result->value = 0.0f;
	result->success = false;
	NativeDataVariable* variable = GetNativeDataVariable(query->variableHandle, NativeDataValueType::Float);
	if (variable == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->value = variable->floatValue;
	result->success = true;
}

static void NativeDataModelGetString(const RmlDataModelVariableHandleQuery* query, RmlDataModelGetStringResult* result)
{
	result->error = nullptr;
	result->value = nullptr;
	result->success = false;
	NativeDataVariable* variable = GetNativeDataVariable(query->variableHandle, NativeDataValueType::String);
	if (variable == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->value = variable->stringValue.c_str();
	result->success = true;
}

static void NativeDataModelGetColor(const RmlDataModelVariableHandleQuery* query, RmlDataModelGetColorResult* result)
{
	result->error = nullptr;
	result->red = 0;
	result->green = 0;
	result->blue = 0;
	result->alpha = 0;
	result->success = false;
	NativeDataVariable* variable = GetNativeDataVariable(query->variableHandle, NativeDataValueType::Color);
	if (variable == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->red = variable->colourValue.red;
	result->green = variable->colourValue.green;
	result->blue = variable->colourValue.blue;
	result->alpha = variable->colourValue.alpha;
	result->success = true;
}

static void NativeDataModelGetPixels(const RmlDataModelVariableHandleQuery* query, RmlDataModelGetPixelsResult* result)
{
	result->error = nullptr;
	result->value = 0.0f;
	result->success = false;
	NativeDataVariable* variable = GetNativeDataVariable(query->variableHandle, NativeDataValueType::Pixels);
	if (variable == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->value = variable->pixelsValue.value;
	result->success = true;
}

static void NativeDataModelGetPercent(const RmlDataModelVariableHandleQuery* query, RmlDataModelGetPercentResult* result)
{
	result->error = nullptr;
	result->value = 0.0f;
	result->success = false;
	NativeDataVariable* variable = GetNativeDataVariable(query->variableHandle, NativeDataValueType::Percent);
	if (variable == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	result->value = variable->percentValue.value;
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
	result->elementHandle = ToElementHandle(element);
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
	result->elementHandle = ToElementHandle(element);
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
	const auto modal = query->options.hasModal ? static_cast<Rml::ModalFlag>(query->options.modal) : Rml::ModalFlag::None;
	const auto focus = query->options.hasFocus ? static_cast<Rml::FocusFlag>(query->options.focus) : Rml::FocusFlag::Auto;
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
	if (styleSheet == nullptr) {
		return;
	}

	const Rml::StyleSheetContainer* existingStyleSheet = document->GetStyleSheetContainer();
	if (existingStyleSheet != nullptr) {
		styleSheet = styleSheet->CombineStyleSheetContainer(*existingStyleSheet);
		if (styleSheet == nullptr) {
			return;
		}
	}

	document->SetStyleSheetContainer(std::move(styleSheet));
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
	result->elementHandle = ToElementHandle(element);
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

	auto* listener = new NativeRmlEventListener(query->callback, query->userData, query->destroyCallback, element);
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
		elementHandleResults.push_back(ToElementHandle(element));
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

static void NativeElementGetRect(const RmlElementHandleQuery* query, RmlElementGetRectResult* result)
{
	result->error = nullptr;
	result->left = 0.0f;
	result->top = 0.0f;
	result->width = 0.0f;
	result->height = 0.0f;
	Rml::Element* element = FromElementHandle(query->elementHandle);
	if (element == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	const Rml::Vector2f offset = element->GetAbsoluteOffset(Rml::BoxArea::Border);
	const Rml::Vector2f size = element->GetBox().GetSize(Rml::BoxArea::Border);
	result->left = offset.x;
	result->top = offset.y;
	result->width = size.x;
	result->height = size.y;
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
	if (auto* control = dynamic_cast<Rml::ElementFormControl*>(element)) {
		value = control->GetValue();
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
	static thread_local Rml::String selected;
	result->error = nullptr;
	result->start = 0;
	result->end = 0;
	result->text = nullptr;
	result->success = false;
	auto* input = dynamic_cast<Rml::ElementFormControlInput*>(FromElementHandle(query->elementHandle));
	if (input == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	input->GetSelection(&result->start, &result->end, &selected);
	result->text = selected.c_str();
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
	static thread_local Rml::String selected;
	result->error = nullptr;
	result->start = 0;
	result->end = 0;
	result->text = nullptr;
	result->success = false;
	auto* textarea = dynamic_cast<Rml::ElementFormControlTextArea*>(FromElementHandle(query->elementHandle));
	if (textarea == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}
	textarea->GetSelection(&result->start, &result->end, &selected);
	result->text = selected.c_str();
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
	result->elementHandle = ToElementHandle(currentEventElement);
	result->documentHandle = ToElementHandle(currentEventDocument);
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
	result->elementHandle = ToElementHandle(event->GetCurrentElement());
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
	result->elementHandle = ToElementHandle(event->GetTargetElement());
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
	it->second.handle.DirtyVariable(query->property);
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

namespace NativeRmlUi {

void ClearAllContexts(ContextRemover removeContext)
{
	// Context removal is normally deferred until RmlGui::Update. A native
	// reload loads the replacement module before that update, however, so a new
	// CreateContext with the same name would revive the old context and leave its
	// event callbacks pointing at freed module data.
	if (RmlGui::IsInitialized()) {
		for (const Rml::String& name : nativeContextNames) {
			if (Rml::Context* context = Rml::GetContext(name)) {
				EraseNativeDataModelHandles(context);
				if (removeContext != nullptr) {
					removeContext(ToHandle(context));
				}
			}
		}
	}

	nativeContextNames.clear();
	nativeDataModels.clear();
	nativeTextRowTypes.clear();
	nativeNotificationRowTypes.clear();
	nativeIconRowTypes.clear();
	nativeOptionRowTypes.clear();
	nativeChoiceRowTypes.clear();
	nativeStatusRowTypes.clear();
	nativeSwatchRowTypes.clear();
	nativeGridRowTypes.clear();
	nativeColourTypes.clear();
	nativePixelTypes.clear();
	nativePercentTypes.clear();
	ownedElementPtrs.clear();
	liveElements.clear();
}

} // namespace NativeRmlUi

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
	.ContextCreateDataModel = NativeContextCreateDataModel,
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
	.ElementGetRect = NativeElementGetRect,
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
	.DataModelBindBool = NativeDataModelBindBool,
	.DataModelBindInt = NativeDataModelBindInt,
	.DataModelBindFloat = NativeDataModelBindFloat,
	.DataModelBindString = NativeDataModelBindString,
	.DataModelBindTextRows = NativeDataModelBindTextRows,
	.DataModelSetBool = NativeDataModelSetBool,
	.DataModelSetInt = NativeDataModelSetInt,
	.DataModelSetFloat = NativeDataModelSetFloat,
	.DataModelSetString = NativeDataModelSetString,
	.DataModelSetTextRows = NativeDataModelSetTextRows,
	.DataModelGetBool = NativeDataModelGetBool,
	.DataModelGetInt = NativeDataModelGetInt,
	.DataModelGetFloat = NativeDataModelGetFloat,
	.DataModelGetString = NativeDataModelGetString,
	.Vector2fNew = NativeVector2fNew,
	.Vector2iNew = NativeVector2iNew,
	.DataModelBindNotificationRows = NativeDataModelBindNotificationRows,
	.DataModelSetNotificationRows = NativeDataModelSetNotificationRows,
	.DataModelBindIconRows = NativeDataModelBindIconRows,
	.DataModelSetIconRows = NativeDataModelSetIconRows,
	.DataModelBindOptionRows = NativeDataModelBindOptionRows,
	.DataModelSetOptionRows = NativeDataModelSetOptionRows,
	.DataModelBindChoiceRows = NativeDataModelBindChoiceRows,
	.DataModelSetChoiceRows = NativeDataModelSetChoiceRows,
	.DataModelBindStatusRows = NativeDataModelBindStatusRows,
	.DataModelSetStatusRows = NativeDataModelSetStatusRows,
	.DataModelBindSwatchRows = NativeDataModelBindSwatchRows,
	.DataModelSetSwatchRows = NativeDataModelSetSwatchRows,
	.DataModelBindGridRows = NativeDataModelBindGridRows,
	.DataModelSetGridRows = NativeDataModelSetGridRows,
	.DataModelBindColor = NativeDataModelBindColor,
	.DataModelSetColor = NativeDataModelSetColor,
	.DataModelGetColor = NativeDataModelGetColor,
	.DataModelBindPixels = NativeDataModelBindPixels,
	.DataModelSetPixels = NativeDataModelSetPixels,
	.DataModelGetPixels = NativeDataModelGetPixels,
	.DataModelBindPercent = NativeDataModelBindPercent,
	.DataModelSetPercent = NativeDataModelSetPercent,
	.DataModelGetPercent = NativeDataModelGetPercent,
	.DataModelBindLogRows = NativeDataModelBindLogRows,
	.DataModelSetLogRows = NativeDataModelSetLogRows,
	.ContextPullToFront = NativeContextPullToFront,
	.ContextSetPointerCapture = NativeContextSetPointerCapture,
	.ContextTakePointerCaptureDelta = NativeContextTakePointerCaptureDelta,
	.GetDocumentPathRequests = NativeGetDocumentPathRequests,
	.ClearDocumentPathRequests = NativeClearDocumentPathRequests,
};
