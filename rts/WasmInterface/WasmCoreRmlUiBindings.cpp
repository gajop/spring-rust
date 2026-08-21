/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreRmlUiBindings.h"

#include <cstdint>
#include <limits>
#include <span>
#include <string>

#include "WasmCoreGeneratedSupport.h"
#include "WasmCoreGuestInput.h"
#include "WasmCoreWire.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)
namespace {

using generated::ImportGuard;
using generated::Trap;

struct RetainedCallbackContext {
	HostState* state = nullptr;
	std::uint32_t callbackID = 0;
	std::uint32_t userData = 0;
	std::uint32_t destroyCallbackID = 0;
};

thread_local const RmlDataEventArgs* currentDataEvent = nullptr;

void InvokeRetainedCallback(void* data)
{
	auto* callback = static_cast<RetainedCallbackContext*>(data);
	if (callback == nullptr || callback->state == nullptr)
		return;
	std::string error;
	generated::DispatchRetainedCallback(*callback->state,
		callback->callbackID, callback->userData, error);
}

void InvokeRetainedDataCallback(void* data, const RmlDataEventArgs* arguments)
{
	auto* callback = static_cast<RetainedCallbackContext*>(data);
	if (callback == nullptr || callback->state == nullptr)
		return;
	const RmlDataEventArgs* previous = currentDataEvent;
	currentDataEvent = arguments;
	std::string error;
	generated::DispatchRetainedCallback(*callback->state,
		callback->callbackID, callback->userData, error);
	currentDataEvent = previous;
}

void DestroyRetainedCallback(void* data)
{
	auto* callback = static_cast<RetainedCallbackContext*>(data);
	if (callback == nullptr)
		return;
	if (callback->state != nullptr && callback->destroyCallbackID != 0) {
		std::string error;
		generated::DispatchRetainedCallback(*callback->state,
			callback->destroyCallbackID, callback->userData, error);
	}
	delete callback;
}

bool WriteHandleBool(std::span<std::uint8_t> output,
	std::uint64_t handle, bool success)
{
	WireWriter writer(output);
	return writer.U64(handle) && writer.Bool(success) && writer.Finish(8);
}

template<typename Query>
wasm_trap_t* AddEventListener(HostState* state, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount,
	void (*function)(const Query*, RmlEventListenerCallbackResult*),
	const char* name)
{
	if (slots == nullptr || slotCount != 8)
		return Trap(std::string(name) + " Core ABI signature mismatch");
	ImportGuard guard(state, 9);
	if (!guard.Ok())
		return Trap(guard.Error());
	const std::uint32_t eventLength = static_cast<std::uint32_t>(slots[2].i32);
	if (!guard.Charge(eventLength))
		return Trap(guard.Error());
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);
	std::span<std::uint8_t> output;
	if (!state->memory.MutableView(static_cast<std::uint32_t>(slots[7].i32), 16u, output)) {
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
		return nullptr;
	}
	GuestCString<> event;
	if (!event.Read(state->memory, static_cast<std::uint32_t>(slots[1].i32), eventLength)) {
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
		return nullptr;
	}
	std::string callbackError;
	if (!generated::ResolveCallback(*state, caller, callbackError))
		return Trap(callbackError);

	auto* callback = new RetainedCallbackContext{
		state,
		static_cast<std::uint32_t>(slots[4].i32),
		static_cast<std::uint32_t>(slots[5].i32),
		static_cast<std::uint32_t>(slots[6].i32),
	};
	Query query{};
	query.event = event.c_str();
	query.inCapturePhase = slots[3].i32 != 0;
	query.callback = InvokeRetainedCallback;
	query.userData = callback;
	query.destroyCallback = DestroyRetainedCallback;
	if constexpr (requires { query.contextHandle; })
		query.contextHandle = static_cast<std::uint64_t>(slots[0].i64);
	else
		query.elementHandle = static_cast<std::uint64_t>(slots[0].i64);

	RmlEventListenerCallbackResult result{};
	function(&query, &result);
	const std::int32_t status = result.error == nullptr ? 0 : result.error->code;
	if (status != 0 || !result.success)
		DestroyRetainedCallback(callback);
	if (!WriteHandleBool(output, result.eventListenerHandle, result.success))
		return Trap("RmlUi Core listener result layout mismatch");
	slots[0].i32 = status;
	return nullptr;
}

wasm_trap_t* ContextAddEventListener(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->rmlUi == nullptr ||
		state->native->rmlUi->ContextAddEventListener == nullptr)
		return Trap("RmlUi ContextAddEventListener Core binding is unavailable");
	return AddEventListener<RmlContextEventListenerCallbackQuery>(state, caller, slots,
		slotCount, state->native->rmlUi->ContextAddEventListener,
		"ContextAddEventListener");
}

wasm_trap_t* ElementAddEventListener(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->rmlUi == nullptr ||
		state->native->rmlUi->ElementAddEventListener == nullptr)
		return Trap("RmlUi ElementAddEventListener Core binding is unavailable");
	return AddEventListener<RmlEventListenerCallbackQuery>(state, caller, slots,
		slotCount, state->native->rmlUi->ElementAddEventListener,
		"ElementAddEventListener");
}

wasm_trap_t* DataModelBindEvent(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->rmlUi == nullptr ||
		state->native->rmlUi->DataModelBindEvent == nullptr)
		return Trap("RmlUi DataModelBindEvent Core binding is unavailable");
	if (slots == nullptr || slotCount != 9)
		return Trap("RmlUi DataModelBindEvent Core ABI signature mismatch");
	ImportGuard guard(state, 10);
	if (!guard.Ok())
		return Trap(guard.Error());
	const std::uint32_t nameLength = static_cast<std::uint32_t>(slots[2].i32);
	const std::uint64_t fieldCount = static_cast<std::uint64_t>(slots[7].i64);
	if (fieldCount > std::numeric_limits<std::size_t>::max() ||
		fieldCount > state->maxValueNodes || !guard.Charge(nameLength + fieldCount)) {
		slots[0].i32 = static_cast<std::int32_t>(Status::BufferOverflow);
		return nullptr;
	}
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);
	std::span<std::uint8_t> output;
	if (!state->memory.MutableView(static_cast<std::uint32_t>(slots[8].i32), 16u, output)) {
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
		return nullptr;
	}
	GuestCString<> name;
	if (!name.Read(state->memory, static_cast<std::uint32_t>(slots[1].i32), nameLength)) {
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
		return nullptr;
	}
	std::span<const std::uint8_t> fieldTypes;
	if (fieldCount != 0 && !state->memory.View(
			static_cast<std::uint32_t>(slots[6].i32),
			static_cast<std::size_t>(fieldCount), fieldTypes)) {
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
		return nullptr;
	}
	std::string callbackError;
	if (!generated::ResolveCallback(*state, caller, callbackError))
		return Trap(callbackError);

	auto* callback = new RetainedCallbackContext{
		state,
		static_cast<std::uint32_t>(slots[3].i32),
		static_cast<std::uint32_t>(slots[4].i32),
		static_cast<std::uint32_t>(slots[5].i32),
	};
	RmlDataModelBindEventQuery query{
		.dataModelHandle = static_cast<std::uint64_t>(slots[0].i64),
		.name = name.c_str(),
		.callback = InvokeRetainedDataCallback,
		.userData = callback,
		.destroyCallback = DestroyRetainedCallback,
		.fieldTypes = fieldTypes.empty() ? nullptr : fieldTypes.data(),
		.fieldCount = fieldCount,
	};
	RmlDataModelBindEventResult result{};
	// Native DataModelBindEvent either retains the callback or invokes the
	// destroy callback on every failure path.
	state->native->rmlUi->DataModelBindEvent(&query, &result);
	const std::int32_t status = result.error == nullptr ? 0 : result.error->code;
	if (!WriteHandleBool(output, result.eventHandle, result.success))
		return Trap("RmlUi Core data-event result layout mismatch");
	slots[0].i32 = status;
	return nullptr;
}

wasm_trap_t* DataModelUnbindEvent(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->rmlUi == nullptr ||
		state->native->rmlUi->DataModelUnbindEvent == nullptr)
		return Trap("RmlUi DataModelUnbindEvent Core binding is unavailable");
	if (slots == nullptr || slotCount != 1)
		return Trap("RmlUi DataModelUnbindEvent Core ABI signature mismatch");
	ImportGuard guard(state, 2);
	if (!guard.Ok())
		return Trap(guard.Error());
	RmlDataModelEventHandleQuery query{static_cast<std::uint64_t>(slots[0].i64)};
	RmlElementBoolResult result{};
	state->native->rmlUi->DataModelUnbindEvent(&query, &result);
	const std::int32_t status = result.error == nullptr ? 0 : result.error->code;
	slots[0].i64 = static_cast<std::int64_t>(PackU32(result.success ? 1u : 0u, status));
	return nullptr;
}

// Dispatching a retained listener re-enters the guest: the native call runs
// Rml::EventListener::OnAttach/OnDetach/ProcessEvent, which lands back in
// InvokeRetainedCallback for a listener the guest registered. That re-entry is
// bounded by the CallbackGuard nesting limit inside DispatchRetainedCallback,
// so these stay plain scalar transports rather than needing their own guard.
// Handles are opaque and validated natively; an unknown handle is rejected as
// an invalid argument rather than trapping.
template<typename Query, typename Api>
wasm_trap_t* DispatchEventListener(HostState* state, wasmtime_val_raw_t* slots,
	std::size_t slotCount, Api function, const char* name)
{
	if (state == nullptr || state->native == nullptr || state->native->rmlUi == nullptr ||
		function == nullptr)
		return Trap(std::string("RmlUi ") + name + " Core binding is unavailable");
	if (slots == nullptr || slotCount != 2)
		return Trap(std::string("RmlUi ") + name + " Core ABI signature mismatch");
	ImportGuard guard(state, 3);
	if (!guard.Ok())
		return Trap(guard.Error());
	Query query{
		static_cast<std::uint64_t>(slots[0].i64),
		static_cast<std::uint64_t>(slots[1].i64),
	};
	RmlElementBoolResult result{};
	function(&query, &result);
	const std::int32_t status = result.error == nullptr ? 0 : result.error->code;
	slots[0].i64 = static_cast<std::int64_t>(PackU32(result.success ? 1u : 0u, status));
	return nullptr;
}

wasm_trap_t* EventListenerOnAttach(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	return DispatchEventListener<RmlEventListenerElementQuery>(state, slots, slotCount,
		state == nullptr || state->native == nullptr || state->native->rmlUi == nullptr
			? nullptr
			: state->native->rmlUi->EventListenerOnAttach,
		"EventListenerOnAttach");
}

wasm_trap_t* EventListenerOnDetach(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	return DispatchEventListener<RmlEventListenerElementQuery>(state, slots, slotCount,
		state == nullptr || state->native == nullptr || state->native->rmlUi == nullptr
			? nullptr
			: state->native->rmlUi->EventListenerOnDetach,
		"EventListenerOnDetach");
}

wasm_trap_t* EventListenerProcessEvent(void* environment, wasmtime_caller_t*,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	return DispatchEventListener<RmlEventListenerEventQuery>(state, slots, slotCount,
		state == nullptr || state->native == nullptr || state->native->rmlUi == nullptr
			? nullptr
			: state->native->rmlUi->EventListenerProcessEvent,
		"EventListenerProcessEvent");
}

wasm_trap_t* DataModelCurrentEvent(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || slots == nullptr || slotCount != 1)
		return Trap("RmlUi DataModelCurrentEvent Core ABI signature mismatch");
	ImportGuard guard(state, 2);
	if (!guard.Ok())
		return Trap(guard.Error());
	if (currentDataEvent == nullptr) {
		slots[0].i32 = static_cast<std::int32_t>(Status::InvalidState);
		return nullptr;
	}
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);
	std::span<std::uint8_t> output;
	if (!state->memory.MutableView(static_cast<std::uint32_t>(slots[0].i32), 24u, output)) {
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
		return nullptr;
	}
	WireWriter writer(output);
	if (!writer.U64(currentDataEvent->eventHandle) ||
		!writer.U64(currentDataEvent->targetElementHandle) ||
		!writer.U64(currentDataEvent->count) || !writer.Finish(8))
		return Trap("RmlUi current data-event layout mismatch");
	slots[0].i32 = 0;
	return nullptr;
}

wasm_trap_t* DataModelCurrentValue(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || slots == nullptr || slotCount != 4)
		return Trap("RmlUi DataModelCurrentValue Core ABI signature mismatch");
	ImportGuard guard(state, 5);
	if (!guard.Ok())
		return Trap(guard.Error());
	if (currentDataEvent == nullptr) {
		slots[0].i32 = static_cast<std::int32_t>(Status::InvalidState);
		return nullptr;
	}
	const std::uint64_t index = static_cast<std::uint64_t>(slots[0].i64);
	if (index >= currentDataEvent->count) {
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
		return nullptr;
	}
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);
	std::span<std::uint8_t> output;
	if (!state->memory.MutableView(static_cast<std::uint32_t>(slots[1].i32), 24u, output)) {
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
		return nullptr;
	}
	const RmlDataValue& value = currentDataEvent->values[index];
	const std::size_t requiredSize =
		(value.type == RML_FIELD_STRING && value.stringValue != nullptr)
			? std::char_traits<char>::length(value.stringValue) : 0u;
	if (requiredSize > std::numeric_limits<std::uint32_t>::max()) {
		slots[0].i32 = static_cast<std::int32_t>(Status::BufferOverflow);
		return nullptr;
	}
	const std::uint32_t required = static_cast<std::uint32_t>(requiredSize);
	const std::uint32_t capacity = static_cast<std::uint32_t>(slots[3].i32);
	const std::uint32_t packedColor =
		static_cast<std::uint32_t>(value.red) |
		(static_cast<std::uint32_t>(value.green) << 8) |
		(static_cast<std::uint32_t>(value.blue) << 16) |
		(static_cast<std::uint32_t>(value.alpha) << 24);
	WireWriter writer(output);
	if (!writer.U32(value.type) || !writer.Bool(value.boolValue) ||
		!writer.I32(value.intValue) || !writer.F32(value.floatValue) ||
		!writer.U32(packedColor) || !writer.U32(required) || !writer.Finish(4))
		return Trap("RmlUi current data-value layout mismatch");
	if (required > capacity) {
		slots[0].i32 = static_cast<std::int32_t>(Status::BufferOverflow);
		return nullptr;
	}
	if (required != 0) {
		if (!guard.Charge(required))
			return Trap(guard.Error());
		if (!state->memory.Write(static_cast<std::uint32_t>(slots[2].i32),
				value.stringValue, required)) {
			slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
			return nullptr;
		}
	}
	slots[0].i32 = 0;
	return nullptr;
}

bool Define(wasmtime_linker_t* linker, const char* name, wasm_functype_t* type,
	wasmtime_func_unchecked_callback_t callback, HostState* state, std::string& error)
{
	wasmtime_error_t* defineError = wasmtime_linker_define_func_unchecked(
		linker, "spring:rml-ui", 13,
		name, std::char_traits<char>::length(name), type, callback, state, nullptr);
	wasm_functype_delete(type);
	if (defineError == nullptr)
		return true;
	error = ErrorMessage(defineError);
	return false;
}

} // namespace

bool RegisterRmlUiImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error)
{
	if (linker == nullptr || state == nullptr || state->native == nullptr ||
		state->native->rmlUi == nullptr) {
		error = "cannot register RmlUi Core imports without linker/host/API";
		return false;
	}
	const wasm_valkind_t addParams[] = {
		WASM_I64, WASM_I32, WASM_I32, WASM_I32,
		WASM_I32, WASM_I32, WASM_I32, WASM_I32,
	};
	const wasm_valkind_t status[] = {WASM_I32};
	if (!Define(linker, "context-add-event-listener",
			MakeFuncType(addParams, 8, status, 1), ContextAddEventListener, state, error) ||
		!Define(linker, "element-add-event-listener",
			MakeFuncType(addParams, 8, status, 1), ElementAddEventListener, state, error))
		return false;
	const wasm_valkind_t bindParams[] = {
		WASM_I64, WASM_I32, WASM_I32, WASM_I32, WASM_I32,
		WASM_I32, WASM_I32, WASM_I64, WASM_I32,
	};
	if (!Define(linker, "data-model-bind-event",
			MakeFuncType(bindParams, 9, status, 1), DataModelBindEvent, state, error))
		return false;
	const wasm_valkind_t unbindParams[] = {WASM_I64};
	const wasm_valkind_t packed[] = {WASM_I64};
	if (!Define(linker, "data-model-unbind-event",
			MakeFuncType(unbindParams, 1, packed, 1), DataModelUnbindEvent, state, error))
		return false;
	const wasm_valkind_t currentEventParams[] = {WASM_I32};
	if (!Define(linker, "data-model-current-event",
			MakeFuncType(currentEventParams, 1, status, 1), DataModelCurrentEvent, state, error))
		return false;
	const wasm_valkind_t currentValueParams[] = {WASM_I64, WASM_I32, WASM_I32, WASM_I32};
	if (!Define(linker, "data-model-current-value",
			MakeFuncType(currentValueParams, 4, status, 1), DataModelCurrentValue, state, error))
		return false;
	const wasm_valkind_t listenerParams[] = {WASM_I64, WASM_I64};
	return Define(linker, "event-listener-on-attach",
			MakeFuncType(listenerParams, 2, packed, 1), EventListenerOnAttach, state, error) &&
		Define(linker, "event-listener-on-detach",
			MakeFuncType(listenerParams, 2, packed, 1), EventListenerOnDetach, state, error) &&
		Define(linker, "event-listener-process-event",
			MakeFuncType(listenerParams, 2, packed, 1), EventListenerProcessEvent, state, error);
}

#endif

} // namespace recoil::wasm::core
