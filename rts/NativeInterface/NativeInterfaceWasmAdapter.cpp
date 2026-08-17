/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "NativeInterfaceWasmAdapter.h"

#include <cctype>
#include <cmath>
#include <limits>
#include <memory>
#include <string>
#include <vector>

#include "generated/WasmHostAdapter.h"
#include "WasmUiVisibility.h"
#include "WasmInterface/WasmModule.h"

namespace {
	const WasmValueRecord* RecordArgument(const std::vector<WasmValue>& arguments,
		std::string& error)
	{
		if (arguments.size() != 1) {
			error = "Wasm native adapter expected one query record";
			return nullptr;
		}
		const auto* record = std::get_if<WasmValueRecord>(&arguments.front().storage);
		if (record == nullptr)
			error = "Wasm native adapter query is not a record";
		return record;
	}

	std::string ToWitFieldName(std::string_view value)
	{
		std::string result;
		result.reserve(value.size() + value.size() / 3);
		for (std::size_t index = 0; index < value.size(); ++index) {
			const unsigned char character = static_cast<unsigned char>(value[index]);
			const bool uppercase = std::isupper(character) != 0;
			const bool previousUppercase = index > 0 &&
				std::isupper(static_cast<unsigned char>(value[index - 1])) != 0;
			const bool nextLowercase = index + 1 < value.size() &&
				std::islower(static_cast<unsigned char>(value[index + 1])) != 0;
			if (uppercase && index != 0 && (!previousUppercase || nextLowercase))
				result.push_back('-');
			result.push_back(static_cast<char>(std::tolower(character)));
		}
		return result;
	}

	const WasmValue* FindField(const WasmValueRecord& record, std::string_view name)
	{
		const auto iter = record.find(std::string(name));
		if (iter != record.end())
			return &iter->second;
		const std::string witName = ToWitFieldName(name);
		for (const auto& [fieldName, fieldValue] : record) {
			if (fieldName == witName)
				return &fieldValue;
		}
		return nullptr;
	}

	const WasmValue* ArgumentValue(const std::vector<WasmValue>& arguments,
		std::size_t index, std::string_view name, std::string& error)
	{
		if (arguments.size() == 1) {
			if (const auto* record = std::get_if<WasmValueRecord>(&arguments.front().storage)) {
				const WasmValue* field = FindField(*record, name);
				if (field != nullptr)
					return field;
			}
		}
		if (index >= arguments.size()) {
			error = "missing positional query argument: " + std::string(name);
			return nullptr;
		}
		return &arguments[index];
	}

	bool ReadI32Value(const WasmValue& field, std::int32_t& value, std::string& error,
		std::string_view name)
	{
		if (const auto* signedValue = std::get_if<std::int64_t>(&field.storage)) {
			if (*signedValue < std::numeric_limits<std::int32_t>::min() ||
				*signedValue > std::numeric_limits<std::int32_t>::max()) {
				error = "integer query field is outside i32: " + std::string(name);
				return false;
			}
			value = static_cast<std::int32_t>(*signedValue);
			return true;
		}
		if (const auto* unsignedValue = std::get_if<std::uint64_t>(&field.storage)) {
			if (*unsignedValue > std::numeric_limits<std::int32_t>::max()) {
				error = "unsigned query field is outside i32: " + std::string(name);
				return false;
			}
			value = static_cast<std::int32_t>(*unsignedValue);
			return true;
		}
		error = "query field is not an integer: " + std::string(name);
		return false;
	}

	bool ReadU32Value(const WasmValue& field, std::uint32_t& value, std::string& error,
		std::string_view name)
	{
		if (const auto* unsignedValue = std::get_if<std::uint64_t>(&field.storage)) {
			if (*unsignedValue > std::numeric_limits<std::uint32_t>::max()) {
				error = "unsigned query field is outside u32: " + std::string(name);
				return false;
			}
			value = static_cast<std::uint32_t>(*unsignedValue);
			return true;
		}
		if (const auto* signedValue = std::get_if<std::int64_t>(&field.storage)) {
			if (*signedValue < 0 ||
				static_cast<std::uint64_t>(*signedValue) > std::numeric_limits<std::uint32_t>::max()) {
				error = "signed query field is outside u32: " + std::string(name);
				return false;
			}
			value = static_cast<std::uint32_t>(*signedValue);
			return true;
		}
		error = "query field is not an unsigned integer: " + std::string(name);
		return false;
	}

	bool ReadU64Value(const WasmValue& field, std::uint64_t& value, std::string& error,
		std::string_view name)
	{
		if (const auto* unsignedValue = std::get_if<std::uint64_t>(&field.storage)) {
			value = *unsignedValue;
			return true;
		}
		if (const auto* signedValue = std::get_if<std::int64_t>(&field.storage)) {
			if (*signedValue < 0) {
				error = "signed query field is negative: " + std::string(name);
				return false;
			}
			value = static_cast<std::uint64_t>(*signedValue);
			return true;
		}
		error = "query field is not an unsigned integer: " + std::string(name);
		return false;
	}

	bool ReadBoolValue(const WasmValue& field, bool& value, std::string& error,
		std::string_view name)
	{
		if (const auto* boolean = std::get_if<bool>(&field.storage)) {
			value = *boolean;
			return true;
		}
		error = "query field is not a boolean: " + std::string(name);
		return false;
	}

	bool ReadFloatValue(const WasmValue& field, float& value, std::string& error,
		std::string_view name)
	{
		if (const auto* number = std::get_if<double>(&field.storage)) {
			if (!std::isfinite(*number) || *number < -std::numeric_limits<float>::max() ||
				*number > std::numeric_limits<float>::max()) {
				error = "query field is outside finite f32: " + std::string(name);
				return false;
			}
			value = static_cast<float>(*number);
			return true;
		}
		error = "query field is not a float: " + std::string(name);
		return false;
	}

	bool ReadStringValue(const WasmValue& field, std::string& value, std::string& error,
		std::string_view name)
	{
		if (const auto* string = std::get_if<std::string>(&field.storage)) {
			value = *string;
			return true;
		}
		error = "query field is not a string: " + std::string(name);
		return false;
	}

	bool ReadRecordValue(const WasmValue& field, const WasmValueRecord*& value,
		std::string& error, std::string_view name)
	{
		value = std::get_if<WasmValueRecord>(&field.storage);
		if (value == nullptr)
			error = "query field is not a record: " + std::string(name);
		return value != nullptr;
	}

	bool ReadI32(const WasmValueRecord& record, std::string_view name, std::int32_t& value,
		std::string& error)
	{
		const WasmValue* field = FindField(record, name);
		if (field == nullptr) {
			error = "missing integer query field: " + std::string(name);
			return false;
		}
		return ReadI32Value(*field, value, error, name);
	}

	bool ReadU32(const WasmValueRecord& record, std::string_view name, std::uint32_t& value,
		std::string& error)
	{
		const WasmValue* field = FindField(record, name);
		if (field == nullptr) {
			error = "missing unsigned integer query field: " + std::string(name);
			return false;
		}
		return ReadU32Value(*field, value, error, name);
	}

	bool ReadU64(const WasmValueRecord& record, std::string_view name, std::uint64_t& value,
		std::string& error)
	{
		const WasmValue* field = FindField(record, name);
		if (field == nullptr) {
			error = "missing unsigned integer query field: " + std::string(name);
			return false;
		}
		return ReadU64Value(*field, value, error, name);
	}

	bool ReadBool(const WasmValueRecord& record, std::string_view name, bool& value,
		std::string& error)
	{
		const WasmValue* field = FindField(record, name);
		if (field == nullptr) {
			error = "missing boolean query field: " + std::string(name);
			return false;
		}
		return ReadBoolValue(*field, value, error, name);
	}

	bool ReadRecord(const WasmValueRecord& record, std::string_view name,
		const WasmValueRecord*& value, std::string& error)
	{
		const WasmValue* field = FindField(record, name);
		if (field == nullptr) {
			error = "missing record query field: " + std::string(name);
			return false;
		}
		return ReadRecordValue(*field, value, error, name);
	}

	bool ReadFloat(const WasmValueRecord& record, std::string_view name, float& value,
		std::string& error)
	{
		const WasmValue* field = FindField(record, name);
		if (field == nullptr) {
			error = "missing float query field: " + std::string(name);
			return false;
		}
		return ReadFloatValue(*field, value, error, name);
	}

	bool ReadString(const WasmValueRecord& record, std::string_view name, std::string& value,
		std::string& error)
	{
		const WasmValue* field = FindField(record, name);
		if (field == nullptr) {
			error = "missing string query field: " + std::string(name);
			return false;
		}
		return ReadStringValue(*field, value, error, name);
	}

	bool ReadFloatList(const WasmValueRecord& record, std::string_view name,
		std::vector<float>& values, std::string& error)
	{
		const WasmValue* field = FindField(record, name);
		if (field == nullptr) {
			error = "missing list query field: " + std::string(name);
			return false;
		}
		const auto* list = std::get_if<WasmValueList>(&field->storage);
		if (list == nullptr) {
			error = "query field is not a list: " + std::string(name);
			return false;
		}
		values.clear();
		values.reserve(list->size());
		for (const WasmValue& item : *list) {
			const auto* number = std::get_if<double>(&item.storage);
			if (number == nullptr || !std::isfinite(*number) ||
				*number < -std::numeric_limits<float>::max() ||
				*number > std::numeric_limits<float>::max()) {
				error = "query list contains a non-finite or out-of-range float";
				return false;
			}
			values.push_back(static_cast<float>(*number));
		}
		return true;
	}

	bool ReadFloatListValue(const WasmValue& field, std::vector<float>& values,
		std::string& error, std::string_view name)
	{
		const auto* list = std::get_if<WasmValueList>(&field.storage);
		if (list == nullptr) {
			error = "query field is not a list: " + std::string(name);
			return false;
		}
		values.clear();
		values.reserve(list->size());
		for (const WasmValue& item : *list) {
			float value = 0.0f;
			if (!ReadFloatValue(item, value, error, name))
				return false;
			values.push_back(value);
		}
		return true;
	}

	bool ReadByteList(const WasmValueRecord& record, std::string_view name,
		std::vector<std::uint8_t>& values, std::string& error)
	{
		const WasmValue* field = FindField(record, name);
		if (field == nullptr) {
			error = "missing byte list query field: " + std::string(name);
			return false;
		}
		if (const auto* bytes = std::get_if<std::vector<std::uint8_t>>(&field->storage)) {
			values = *bytes;
			return true;
		}
		const auto* list = std::get_if<WasmValueList>(&field->storage);
		if (list == nullptr) {
			error = "query field is not a byte list: " + std::string(name);
			return false;
		}
		values.clear();
		values.reserve(list->size());
		for (const WasmValue& item : *list) {
			std::uint64_t byte = 0;
			if (const auto* unsignedValue = std::get_if<std::uint64_t>(&item.storage))
				byte = *unsignedValue;
			else if (const auto* signedValue = std::get_if<std::int64_t>(&item.storage))
				byte = (*signedValue < 0) ? std::numeric_limits<std::uint64_t>::max() :
					static_cast<std::uint64_t>(*signedValue);
			else {
				error = "query byte list contains a non-integer";
				return false;
			}
			if (byte > std::numeric_limits<std::uint8_t>::max()) {
				error = "query byte list contains a value outside u8";
				return false;
			}
			values.push_back(static_cast<std::uint8_t>(byte));
		}
		return true;
	}

	bool ReadByteListValue(const WasmValue& field, std::vector<std::uint8_t>& values,
		std::string& error, std::string_view name)
	{
		if (const auto* bytes = std::get_if<std::vector<std::uint8_t>>(&field.storage)) {
			values = *bytes;
			return true;
		}
		const auto* list = std::get_if<WasmValueList>(&field.storage);
		if (list == nullptr) {
			error = "query field is not a byte list: " + std::string(name);
			return false;
		}
		values.clear();
		values.reserve(list->size());
		for (const WasmValue& item : *list) {
			std::uint64_t byte = 0;
			if (const auto* unsignedValue = std::get_if<std::uint64_t>(&item.storage))
				byte = *unsignedValue;
			else if (const auto* signedValue = std::get_if<std::int64_t>(&item.storage)) {
				if (*signedValue < 0)
					byte = std::numeric_limits<std::uint64_t>::max();
				else
					byte = static_cast<std::uint64_t>(*signedValue);
			} else {
				error = "query byte list contains a non-integer: " + std::string(name);
				return false;
			}
			if (byte > std::numeric_limits<std::uint8_t>::max()) {
				error = "query byte list contains a value outside u8: " + std::string(name);
				return false;
			}
			values.push_back(static_cast<std::uint8_t>(byte));
		}
		return true;
	}

	bool ReadFloat3Value(const WasmValue& field, Float3& value, std::string& error,
		std::string_view name)
	{
		const WasmValueRecord* record = nullptr;
		if (!ReadRecordValue(field, record, error, name))
			return false;
		return ReadFloat(*record, "x", value.x, error) &&
			ReadFloat(*record, "y", value.y, error) &&
			ReadFloat(*record, "z", value.z, error);
	}

	bool ReadFloat3(const WasmValueRecord& record, std::string_view name, Float3& value,
		std::string& error)
	{
		const WasmValueRecord* position = nullptr;
		if (!ReadRecord(record, name, position, error))
			return false;
		return ReadFloat(*position, "x", value.x, error) &&
			ReadFloat(*position, "y", value.y, error) &&
			ReadFloat(*position, "z", value.z, error);
	}

	WasmValue Float3Value(const Float3& value)
	{
		return WasmValue::Record({
			{"x", WasmValue::F64(value.x)},
			{"y", WasmValue::F64(value.y)},
			{"z", WasmValue::F64(value.z)},
		});
	}

	template<typename Result>
	bool CheckNativeError(const Result& result, std::string& error)
	{
		if (result.error == nullptr)
			return true;
		error = "native API error " + std::to_string(result.error->code);
		if (result.error->message != nullptr)
			error += ": " + std::string(result.error->message);
		return false;
	}

	bool IsFunction(std::string_view value, std::string_view nativeName,
		std::string_view witName)
	{
		return value == nativeName || value == witName;
	}

	WasmValue WasmBoolValue(bool value)
	{
		return WasmValue::Bool(value);
	}

	struct GuestCallbackState {
		std::shared_ptr<WasmCallbackLifetime> lifetime;
		WasmCallbackID callbackID = 0;
		WasmCallbackID destroyCallbackID = 0;
		std::uint64_t userData = 0;
		bool destroyed = false;
	};

	struct GuestCallbackContext {
		std::shared_ptr<GuestCallbackState> state;
	};

	bool InvokeGuestCallback(const std::shared_ptr<GuestCallbackState>& state,
		WasmCallbackID callbackID, const std::vector<std::uint64_t>& arguments)
	{
		if (state == nullptr || state->destroyed || state->lifetime == nullptr ||
			!state->lifetime->active || state->lifetime->module == nullptr)
			return true;
		std::string error;
		return state->lifetime->module->InvokeGuestCallback(callbackID, arguments, error);
	}

	void GuestCallbackThunk(void* userData)
	{
		auto* context = static_cast<GuestCallbackContext*>(userData);
		if (context == nullptr)
			return;
		InvokeGuestCallback(context->state, context->state->callbackID,
			{context->state->userData});
	}

	void GuestDataCallbackThunk(void* userData, const RmlDataEventArgs* arguments)
	{
		auto* context = static_cast<GuestCallbackContext*>(userData);
		if (context == nullptr || arguments == nullptr)
			return;
		InvokeGuestCallback(context->state, context->state->callbackID, {
			context->state->userData,
			arguments->eventHandle,
			arguments->targetElementHandle,
			arguments->count,
		});
	}

	void DestroyGuestCallbackContext(void* userData)
	{
		auto* context = static_cast<GuestCallbackContext*>(userData);
		if (context == nullptr)
			return;
		const auto state = context->state;
		if (state != nullptr && !state->destroyed) {
			state->destroyed = true;
			if (state->lifetime != nullptr && state->lifetime->active &&
				state->lifetime->module != nullptr) {
				if (state->destroyCallbackID != 0)
					InvokeGuestCallback(state, state->destroyCallbackID, {state->userData});
				state->lifetime->module->DropGuestCallback(state->callbackID);
				state->lifetime->module->DropGuestCallback(state->destroyCallbackID);
			}
		}
		delete context;
	}

	bool MakeGuestCallbackContext(WasmModule& owner, std::uint32_t callbackID,
		std::uint32_t destroyCallbackID, std::uint32_t userData,
		GuestCallbackContext*& context, std::shared_ptr<GuestCallbackState>& state,
		std::string& error)
	{
		state = std::make_shared<GuestCallbackState>();
		state->lifetime = owner.CallbackLifetime();
		state->userData = userData;
		state->callbackID = owner.RegisterGuestCallback(callbackID, {.reentrant = true}, error);
		if (state->callbackID == 0)
			return false;
		if (destroyCallbackID != 0) {
			state->destroyCallbackID = owner.RegisterGuestCallback(destroyCallbackID,
				{.reentrant = true}, error);
			if (state->destroyCallbackID == 0) {
				owner.DropGuestCallback(state->callbackID);
				return false;
			}
		}
		context = new GuestCallbackContext{state};
		return true;
	}

	bool ReadCallbackArguments(const std::vector<WasmValue>& arguments,
		std::size_t callbackIndex, std::size_t userDataIndex,
		std::uint32_t& callbackID, std::uint32_t& userData,
		std::string& error)
	{
		const WasmValue* callbackValue = ArgumentValue(arguments, callbackIndex, "callback", error);
		const WasmValue* userDataValue = ArgumentValue(arguments, userDataIndex, "userData", error);
		if (callbackValue == nullptr || userDataValue == nullptr)
			return false;
		return ReadU32Value(*callbackValue, callbackID, error, "callback") &&
			ReadU32Value(*userDataValue, userData, error, "userData");
	}

	bool RequireCallbackOwner(WasmModule* owner, std::string_view function, std::string& error)
	{
		if (owner != nullptr)
			return true;
		error = "callback-capable Wasm callout requires a module owner: " + std::string(function);
		return false;
	}

	void DestroyFailedGuestCallbackContext(GuestCallbackContext* context)
	{
		if (context != nullptr)
			DestroyGuestCallbackContext(context);
	}

	bool MakeGuestCallbackFromArguments(WasmModule* owner,
		const std::vector<WasmValue>& arguments, std::size_t callbackIndex,
		std::size_t userDataIndex, std::size_t destroyCallbackIndex,
		GuestCallbackContext*& context, std::shared_ptr<GuestCallbackState>& state,
		std::string_view function, std::string& error)
	{
		if (!RequireCallbackOwner(owner, function, error))
			return false;
		std::uint32_t callbackID = 0;
		std::uint32_t userData = 0;
		if (!ReadCallbackArguments(arguments, callbackIndex, userDataIndex,
			callbackID, userData, error))
			return false;
		std::uint32_t destroyCallbackID = 0;
		if (destroyCallbackIndex != std::numeric_limits<std::size_t>::max()) {
			const WasmValue* destroyValue = ArgumentValue(arguments, destroyCallbackIndex,
				"destroyCallback", error);
			if (destroyValue == nullptr || !ReadU32Value(*destroyValue, destroyCallbackID,
				error, "destroyCallback"))
				return false;
		}
		return MakeGuestCallbackContext(*owner, callbackID, destroyCallbackID, userData,
			context, state, error);
	}

	bool ReadGfxTextureParams(const WasmValueRecord& record, GfxTextureParams& params,
		std::string& error)
	{
		return ReadU32(record, "target", params.target, error) &&
			ReadU32(record, "format", params.format, error) &&
			ReadI32(record, "border", params.border, error) &&
			ReadU32(record, "minFilter", params.minFilter, error) &&
			ReadU32(record, "magFilter", params.magFilter, error) &&
			ReadU32(record, "wrapS", params.wrapS, error) &&
			ReadU32(record, "wrapT", params.wrapT, error) &&
			ReadU32(record, "wrapR", params.wrapR, error) &&
			ReadU32(record, "compareFunc", params.compareFunc, error) &&
			ReadFloat(record, "lodBias", params.lodBias, error) &&
			ReadFloat(record, "aniso", params.aniso, error) &&
			ReadU32(record, "samples", params.samples, error) &&
			ReadBool(record, "fbo", params.fbo, error) &&
			ReadBool(record, "fboDepth", params.fboDepth, error);
	}

	bool ReadNativeCommandDescription(const WasmValueRecord& record,
		NativeCommandDescription& description, std::vector<std::string>& strings,
		std::vector<const char*>& params, std::string& error)
	{
		const WasmValue* paramsValue = FindField(record, "params");
		if (paramsValue == nullptr) {
			error = "missing list query field: params";
			return false;
		}
		const auto* list = std::get_if<WasmValueList>(&paramsValue->storage);
		if (list == nullptr) {
			error = "query field is not a list: params";
			return false;
		}
		strings.reserve(5 + list->size());
		if (!ReadI32(record, "id", description.id, error) ||
			!ReadI32(record, "type", description.type, error) ||
			!ReadBool(record, "queueing", description.queueing, error) ||
			!ReadBool(record, "hidden", description.hidden, error) ||
			!ReadBool(record, "disabled", description.disabled, error) ||
			!ReadBool(record, "showUnique", description.showUnique, error) ||
			!ReadBool(record, "onlyTexture", description.onlyTexture, error) ||
			!ReadString(record, "name", strings.emplace_back(), error) ||
			!ReadString(record, "action", strings.emplace_back(), error) ||
			!ReadString(record, "iconname", strings.emplace_back(), error) ||
			!ReadString(record, "mouseicon", strings.emplace_back(), error) ||
			!ReadString(record, "tooltip", strings.emplace_back(), error))
			return false;

		for (const WasmValue& item : *list) {
			std::string& parameter = strings.emplace_back();
			if (const auto* value = std::get_if<std::string>(&item.storage))
				parameter = *value;
			else {
				error = "command description params contains a non-string";
				return false;
			}
		}
		params.reserve(list->size());
		for (std::size_t index = 5; index < strings.size(); ++index)
			params.push_back(strings[index].c_str());
		description.name = strings[0].c_str();
		description.action = strings[1].c_str();
		description.iconname = strings[2].c_str();
		description.mouseicon = strings[3].c_str();
		description.tooltip = strings[4].c_str();
		description.params = params.empty() ? nullptr : params.data();
		description.paramCount = static_cast<std::uint32_t>(params.size());
		return true;
	}
}

bool NativeInterfaceWasmAdapter::Callout(std::string_view module, std::string_view function,
	const std::vector<WasmValue>& arguments, WasmValue& result, std::string& error)
{
	return CalloutImpl(nullptr, module, function, arguments, result, error);
}

bool NativeInterfaceWasmAdapter::Callout(WasmModule& owner, std::string_view module,
	std::string_view function, const std::vector<WasmValue>& arguments, WasmValue& result,
	std::string& error)
{
	return CalloutImpl(&owner, module, function, arguments, result, error);
}

bool NativeInterfaceWasmAdapter::CalloutImpl(WasmModule* owner, std::string_view module,
	std::string_view function, const std::vector<WasmValue>& arguments, WasmValue& result,
	std::string& error)
{
	if (nativeInterface == nullptr) {
		error = "NativeInterface Wasm adapter has no host interface";
		return false;
	}

	const bool uiEnvironment = owner != nullptr &&
		owner->Descriptor().environment == WasmEnvironment::UI;
	WasmUiVisibility::ScopedContext uiContext(uiEnvironment);

	// CreateContext may return an already-existing context.  Only claim a
	// context that this instance is about to create; a Wasm module must not
	// unload another module's shared UI context during teardown.
	bool trackCreatedContext = false;
	if (owner != nullptr && module == "rml_ui" &&
		IsFunction(function, "CreateContext", "create-context") &&
		nativeInterface->rmlUi != nullptr && nativeInterface->rmlUi->GetContext != nullptr) {
		std::string preflightError;
		const WasmValue* nameValue = ArgumentValue(arguments, 0, "name", preflightError);
		std::string contextName;
		if (nameValue != nullptr &&
			ReadStringValue(*nameValue, contextName, preflightError, "name")) {
			RmlGetContextQuery query{.name = contextName.c_str()};
			RmlGetContextResult nativeResult{};
			nativeInterface->rmlUi->GetContext(&query, &nativeResult);
			trackCreatedContext = nativeResult.error == nullptr && !nativeResult.exists;
		}
	}

	const auto generatedResult = recoil::wasm::generated::DispatchNativeCallout(
		nativeInterface, module, function, arguments, result, error);
	if (generatedResult == recoil::wasm::generated::NativeCalloutDispatch::handled) {
		if (trackCreatedContext && error.empty()) {
			const auto* record = std::get_if<WasmValueRecord>(&result.storage);
			const WasmValue* handleValue = record == nullptr ? nullptr :
				FindField(*record, "contextHandle");
			const WasmValue* successValue = record == nullptr ? nullptr :
				FindField(*record, "success");
			std::uint64_t contextHandle = 0;
			bool success = false;
			if (handleValue == nullptr || successValue == nullptr ||
				!ReadU64Value(*handleValue, contextHandle, error, "contextHandle") ||
				!ReadBoolValue(*successValue, success, error, "success"))
				return false;
			if (success && contextHandle != 0) {
				auto* rmlUi = nativeInterface->rmlUi;
				if (!owner->RegisterCleanup([contextHandle, rmlUi]() {
					if (rmlUi == nullptr || rmlUi->RemoveContext == nullptr)
						return;
					RmlRemoveContextQuery query{.contextHandle = contextHandle};
					RmlRemoveContextResult cleanupResult{};
					rmlUi->RemoveContext(&query, &cleanupResult);
				})) {
					if (rmlUi != nullptr && rmlUi->RemoveContext != nullptr) {
						RmlRemoveContextQuery query{.contextHandle = contextHandle};
						RmlRemoveContextResult cleanupResult{};
						rmlUi->RemoveContext(&query, &cleanupResult);
					}
					error = "could not register RmlUi context cleanup for Wasm module";
					return false;
				}
			}
		}
		return error.empty();
	}

	if (module == "cob_script" && IsFunction(function, "CallCOBScript", "call-cob-script")) {
		// MAX_COB_ARGS is an engine-side fixed bound.  Keep the Wasm adapter's
		// reviewed boundary no larger than the native COB scratch storage.
		constexpr std::uint32_t maxCobArgs = 16;
		const WasmValue* unitValue = ArgumentValue(arguments, 0, "unitID", error);
		const WasmValue* functionValue = ArgumentValue(arguments, 1, "func", error);
		const WasmValue* retArgsValue = ArgumentValue(arguments, 2, "retArgs", error);
		const WasmValue* argsValue = ArgumentValue(arguments, 3, "args", error);
		if (unitValue == nullptr || functionValue == nullptr || retArgsValue == nullptr ||
			argsValue == nullptr)
			return false;

		CallCOBScriptQuery nativeQuery{};
		if (!ReadI32Value(*unitValue, nativeQuery.unitID, error, "unitID"))
			return false;
		const WasmValueRecord* functionRecord = nullptr;
		if (!ReadRecordValue(*functionValue, functionRecord, error, "func"))
			return false;
		std::string functionName;
		if (!ReadString(*functionRecord, "name", functionName, error) ||
			!ReadI32(*functionRecord, "id", nativeQuery.func.id, error))
			return false;
		if (!ReadU32Value(*retArgsValue, nativeQuery.retArgs, error, "retArgs"))
			return false;

		const auto* argumentList = std::get_if<WasmValueList>(&argsValue->storage);
		if (argumentList == nullptr) {
			error = "query field is not a list: args";
			return false;
		}
		if (argumentList->size() > maxCobArgs) {
			error = "CallCOBScript argument list exceeds the native COB limit";
			return false;
		}
		if (nativeQuery.retArgs > maxCobArgs) {
			error = "CallCOBScript return count exceeds the native COB limit";
			return false;
		}
		if (nativeQuery.retArgs > std::numeric_limits<std::size_t>::max() / sizeof(std::int32_t)) {
			error = "CallCOBScript return count overflows the result-size calculation";
			return false;
		}
		const std::size_t resultBytes =
			static_cast<std::size_t>(nativeQuery.retArgs) * sizeof(std::int32_t);
		if (owner != nullptr && !owner->Budget().CheckResultSize(resultBytes)) {
			error = "CallCOBScript return values exceed the configured byte limit";
			return false;
		}

		std::vector<std::int32_t> nativeArguments;
		nativeArguments.reserve(argumentList->size());
		for (const WasmValue& argument : *argumentList) {
			std::int32_t value = 0;
			if (!ReadI32Value(argument, value, error, "args"))
				return false;
			nativeArguments.push_back(value);
		}
		nativeQuery.func.name = functionName.c_str();
		nativeQuery.args = nativeArguments.empty() ? nullptr : nativeArguments.data();
		nativeQuery.argCount = static_cast<std::uint32_t>(nativeArguments.size());

		if (nativeInterface->syncedCtrl == nullptr ||
			nativeInterface->syncedCtrl->cobScript == nullptr ||
			nativeInterface->syncedCtrl->cobScript->CallCOBScript == nullptr) {
			error = "NativeInterface API is unavailable: cob_script.CallCOBScript";
			return false;
		}
		CallCOBScriptResult nativeResult{};
		nativeInterface->syncedCtrl->cobScript->CallCOBScript(&nativeQuery, &nativeResult);
		if (!CheckNativeError(nativeResult, error))
			return false;
		if (nativeResult.retCount > nativeQuery.retArgs || nativeResult.retCount > maxCobArgs) {
			error = "Native CallCOBScript returned more values than requested";
			return false;
		}
		if (nativeResult.retCount != 0 && nativeResult.retValues == nullptr) {
			error = "Native CallCOBScript returned a null result buffer";
			return false;
		}
		WasmValueList returnValues;
		returnValues.reserve(nativeResult.retCount);
		for (std::uint32_t index = 0; index < nativeResult.retCount; ++index)
			returnValues.push_back(WasmValue::I64(nativeResult.retValues[index]));
		result = WasmValue::Record({
			{"retCode", WasmValue::I64(nativeResult.retCode)},
			{"retValues", WasmValue::List(std::move(returnValues))},
		});
		return true;
	}

	if (module == "los") {
		const WasmValueRecord* query = RecordArgument(arguments, error);
		if (query == nullptr)
			return false;
		if (nativeInterface->los == nullptr) {
			error = "NativeInterface API is unavailable: los";
			return false;
		}
		if (IsFunction(function, "IsUnitInLos", "is-unit-in-los") ||
			IsFunction(function, "IsUnitInRadar", "is-unit-in-radar") ||
			IsFunction(function, "IsUnitInJammer", "is-unit-in-jammer")) {
			IsUnitInLosQuery nativeQuery{};
			if (!ReadI32(*query, "unitID", nativeQuery.unitID, error) ||
				!ReadI32(*query, "allyTeamID", nativeQuery.allyTeamID, error))
				return false;
			if (IsFunction(function, "IsUnitInLos", "is-unit-in-los")) {
				if (nativeInterface->los->IsUnitInLos == nullptr) {
					error = "NativeInterface API is unavailable: los.IsUnitInLos";
					return false;
				}
				IsUnitInLosResult nativeResult{};
				nativeInterface->los->IsUnitInLos(&nativeQuery, &nativeResult);
				if (!CheckNativeError(nativeResult, error)) return false;
				result = WasmValue::Bool(nativeResult.inLos);
				return true;
			}
			if (IsFunction(function, "IsUnitInRadar", "is-unit-in-radar")) {
				if (nativeInterface->los->IsUnitInRadar == nullptr) {
					error = "NativeInterface API is unavailable: los.IsUnitInRadar";
					return false;
				}
				IsUnitInRadarQuery radarQuery{nativeQuery.unitID, nativeQuery.allyTeamID};
				IsUnitInRadarResult nativeResult{};
				nativeInterface->los->IsUnitInRadar(&radarQuery, &nativeResult);
				if (!CheckNativeError(nativeResult, error)) return false;
				result = WasmValue::Bool(nativeResult.inRadar);
				return true;
			}
			if (nativeInterface->los->IsUnitInJammer == nullptr) {
				error = "NativeInterface API is unavailable: los.IsUnitInJammer";
				return false;
			}
			IsUnitInJammerQuery jammerQuery{nativeQuery.unitID, nativeQuery.allyTeamID};
			IsUnitInJammerResult nativeResult{};
			nativeInterface->los->IsUnitInJammer(&jammerQuery, &nativeResult);
			if (!CheckNativeError(nativeResult, error)) return false;
			result = WasmValue::Bool(nativeResult.inJammer);
			return true;
		}
		if (IsFunction(function, "IsPosInLos", "is-pos-in-los") ||
			IsFunction(function, "IsPosInRadar", "is-pos-in-radar") ||
			IsFunction(function, "IsPosInAirLos", "is-pos-in-air-los")) {
			IsPosInLosQuery nativeQuery{};
			if (!ReadFloat3(*query, "pos", nativeQuery.pos, error) ||
				!ReadI32(*query, "allyTeamID", nativeQuery.allyTeamID, error))
				return false;
			if (IsFunction(function, "IsPosInLos", "is-pos-in-los")) {
				if (nativeInterface->los->IsPosInLos == nullptr) {
					error = "NativeInterface API is unavailable: los.IsPosInLos";
					return false;
				}
				IsPosInLosResult nativeResult{};
				nativeInterface->los->IsPosInLos(&nativeQuery, &nativeResult);
				if (!CheckNativeError(nativeResult, error)) return false;
				result = WasmValue::Bool(nativeResult.inLos);
				return true;
			}
			if (IsFunction(function, "IsPosInRadar", "is-pos-in-radar")) {
				if (nativeInterface->los->IsPosInRadar == nullptr) {
					error = "NativeInterface API is unavailable: los.IsPosInRadar";
					return false;
				}
				IsPosInRadarQuery radarQuery{nativeQuery.pos, nativeQuery.allyTeamID};
				IsPosInRadarResult nativeResult{};
				nativeInterface->los->IsPosInRadar(&radarQuery, &nativeResult);
				if (!CheckNativeError(nativeResult, error)) return false;
				result = WasmValue::Bool(nativeResult.inRadar);
				return true;
			}
			if (nativeInterface->los->IsPosInAirLos == nullptr) {
				error = "NativeInterface API is unavailable: los.IsPosInAirLos";
				return false;
			}
			IsPosInAirLosQuery airQuery{nativeQuery.pos, nativeQuery.allyTeamID};
			IsPosInAirLosResult nativeResult{};
			nativeInterface->los->IsPosInAirLos(&airQuery, &nativeResult);
			if (!CheckNativeError(nativeResult, error)) return false;
			result = WasmValue::Bool(nativeResult.inAirLos);
			return true;
		}
		if (IsFunction(function, "GetPositionLosState", "get-position-los-state")) {
			GetPositionLosStateQuery nativeQuery{};
			if (!ReadFloat3(*query, "pos", nativeQuery.pos, error) ||
				!ReadI32(*query, "allyTeamID", nativeQuery.allyTeamID, error))
				return false;
			if (nativeInterface->los->GetPositionLosState == nullptr) {
				error = "NativeInterface API is unavailable: los.GetPositionLosState";
				return false;
			}
			GetPositionLosStateResult nativeResult{};
			nativeInterface->los->GetPositionLosState(&nativeQuery, &nativeResult);
			if (!CheckNativeError(nativeResult, error)) return false;
			result = WasmValue::Record({
				{"inLosOrRadar", WasmValue::Bool(nativeResult.state.inLosOrRadar)},
				{"inLos", WasmValue::Bool(nativeResult.state.inLos)},
				{"inRadar", WasmValue::Bool(nativeResult.state.inRadar)},
				{"inJammer", WasmValue::Bool(nativeResult.state.inJammer)},
			});
			return true;
		}
		if (IsFunction(function, "GetRadarErrorParams", "get-radar-error-params")) {
			GetRadarErrorParamsQuery nativeQuery{};
			if (!ReadI32(*query, "allyTeamID", nativeQuery.allyTeamID, error)) return false;
			if (nativeInterface->los->GetRadarErrorParams == nullptr) {
				error = "NativeInterface API is unavailable: los.GetRadarErrorParams";
				return false;
			}
			GetRadarErrorParamsResult nativeResult{};
			nativeInterface->los->GetRadarErrorParams(&nativeQuery, &nativeResult);
			if (!CheckNativeError(nativeResult, error)) return false;
			result = WasmValue::Record({
				{"radarErrorSize", WasmValue::F64(nativeResult.params.radarErrorSize)},
				{"baseRadarErrorSize", WasmValue::F64(nativeResult.params.baseRadarErrorSize)},
				{"baseRadarErrorMult", WasmValue::F64(nativeResult.params.baseRadarErrorMult)},
			});
			return true;
		}
	}

	if (module == "units_commands" && IsFunction(function, "GetCommandParams", "get-command-params")) {
		const WasmValue* commandValue = ArgumentValue(arguments, 0, "command", error);
		if (commandValue == nullptr)
			return false;
		const WasmValueRecord* command = nullptr;
		if (!ReadRecordValue(*commandValue, command, error, "command"))
			return false;
		CommandFFI nativeCommand{};
		std::uint32_t options = 0;
		std::vector<float> commandParams;
		if (!ReadI32(*command, "cmdID", nativeCommand.cmdID, error) ||
			!ReadU32(*command, "options", options, error) ||
			!ReadI32(*command, "tag", nativeCommand.tag, error) ||
			!ReadI32(*command, "aiCommandID", nativeCommand.aiCommandID, error) ||
			!ReadFloat(*command, "timeOut", nativeCommand.timeOut, error) ||
			!ReadFloatList(*command, "params", commandParams, error))
			return false;
		if (options > std::numeric_limits<std::uint8_t>::max()) {
			error = "command options are outside u8";
			return false;
		}
		nativeCommand.options = static_cast<std::uint8_t>(options);
		if (commandParams.size() > std::numeric_limits<std::uint32_t>::max()) {
			error = "command parameter list is too large";
			return false;
		}
		nativeCommand.params = commandParams.empty() ? nullptr : commandParams.data();
		nativeCommand.paramCount = static_cast<std::uint32_t>(commandParams.size());
		if (nativeInterface->unitsCommands == nullptr || nativeInterface->unitsCommands->GetCommandParams == nullptr) {
			error = "NativeInterface API is unavailable: units_commands.GetCommandParams";
			return false;
		}
		GetCommandParamsQuery nativeQuery{.command = &nativeCommand};
		GetCommandParamsResult nativeResult{};
		nativeInterface->unitsCommands->GetCommandParams(&nativeQuery, &nativeResult);
		if (!CheckNativeError(nativeResult, error))
			return false;
		WasmValueList values;
		values.reserve(nativeResult.count);
		for (std::uint32_t index = 0; index < nativeResult.count; ++index)
			values.push_back(WasmValue::F64(nativeResult.params[index]));
		result = WasmValue::List(std::move(values));
		return true;
	}

	if (module == "math_extra" && IsFunction(function, "Normalize", "normalize")) {
		const WasmValue* vectorValue = ArgumentValue(arguments, 0, "vec", error);
		if (vectorValue == nullptr)
			return false;
		Float3 value{};
		if (!ReadFloat3Value(*vectorValue, value, error, "vec"))
			return false;
		if (nativeInterface->mathExtra == nullptr || nativeInterface->mathExtra->Normalize == nullptr) {
			error = "NativeInterface API is unavailable: math_extra.Normalize";
			return false;
		}
		NormalizeQuery nativeQuery{.vec = &value};
		NormalizeResult nativeResult{};
		nativeInterface->mathExtra->Normalize(&nativeQuery, &nativeResult);
		if (!CheckNativeError(nativeResult, error))
			return false;
		result = WasmValue::Record({
			{"length", WasmValue::F64(nativeResult.length)},
			{"vec", Float3Value(value)},
		});
		return true;
	}

	if (module == "unit_control" &&
		(IsFunction(function, "EditUnitCmdDesc", "edit-unit-cmd-desc") ||
		 IsFunction(function, "InsertUnitCmdDesc", "insert-unit-cmd-desc"))) {
		std::int32_t unitID = 0;
		std::int32_t index = 0;
		const WasmValue* unitValue = ArgumentValue(arguments, 0, "unitID", error);
		const WasmValue* indexValue = ArgumentValue(arguments, 1, "cmdDescIndex", error);
		const WasmValue* descriptionValue = ArgumentValue(arguments, 2, "cmdDesc", error);
		if (unitValue == nullptr || indexValue == nullptr || descriptionValue == nullptr)
			return false;
		if (!ReadI32Value(*unitValue, unitID, error, "unitID"))
			return false;
		if (IsFunction(function, "EditUnitCmdDesc", "edit-unit-cmd-desc")) {
			std::uint32_t unsignedIndex = 0;
			if (!ReadU32Value(*indexValue, unsignedIndex, error, "cmdDescIndex"))
				return false;
			index = static_cast<std::int32_t>(unsignedIndex);
		} else if (!ReadI32Value(*indexValue, index, error, "cmdDescIndex")) {
			return false;
		}
		const WasmValueRecord* commandDescriptionRecord = nullptr;
		if (!ReadRecordValue(*descriptionValue, commandDescriptionRecord, error, "cmdDesc"))
			return false;
		NativeCommandDescription description{};
		std::vector<std::string> strings;
		std::vector<const char*> params;
		strings.reserve(5);
		if (!ReadNativeCommandDescription(*commandDescriptionRecord, description, strings, params, error))
			return false;
		if (nativeInterface->syncedCtrl == nullptr || nativeInterface->syncedCtrl->unit == nullptr) {
			error = "NativeInterface API is unavailable: unit_control command description";
			return false;
		}
		if (IsFunction(function, "EditUnitCmdDesc", "edit-unit-cmd-desc")) {
			if (nativeInterface->syncedCtrl->unit->EditUnitCmdDesc == nullptr) {
				error = "NativeInterface API is unavailable: unit_control.EditUnitCmdDesc";
				return false;
			}
			EditUnitCmdDescQuery nativeQuery{
				.unitID = unitID,
				.cmdDescIndex = static_cast<std::uint32_t>(index),
				.cmdDesc = &description,
			};
			EditUnitCmdDescResult nativeResult{};
			nativeInterface->syncedCtrl->unit->EditUnitCmdDesc(&nativeQuery, &nativeResult);
			if (!CheckNativeError(nativeResult, error))
				return false;
			result = WasmBoolValue(nativeResult.success);
			return true;
		}
		if (nativeInterface->syncedCtrl->unit->InsertUnitCmdDesc == nullptr) {
			error = "NativeInterface API is unavailable: unit_control.InsertUnitCmdDesc";
			return false;
		}
		InsertUnitCmdDescQuery nativeQuery{
			.unitID = unitID,
			.cmdDescIndex = index,
			.cmdDesc = &description,
		};
		InsertUnitCmdDescResult nativeResult{};
		nativeInterface->syncedCtrl->unit->InsertUnitCmdDesc(&nativeQuery, &nativeResult);
		if (!CheckNativeError(nativeResult, error))
			return false;
		result = WasmBoolValue(nativeResult.success);
		return true;
	}

	if (module == "gfx" && IsFunction(function, "CreateTexture", "create-texture")) {
		const WasmValue* xsize = ArgumentValue(arguments, 0, "xsize", error);
		const WasmValue* ysize = ArgumentValue(arguments, 1, "ysize", error);
		const WasmValue* zsize = ArgumentValue(arguments, 2, "zsize", error);
		const WasmValue* paramsValue = ArgumentValue(arguments, 3, "params", error);
		if (xsize == nullptr || ysize == nullptr || zsize == nullptr || paramsValue == nullptr)
			return false;
		GfxCreateTextureQuery nativeQuery{};
		if (!ReadI32Value(*xsize, nativeQuery.xsize, error, "xsize") ||
			!ReadI32Value(*ysize, nativeQuery.ysize, error, "ysize") ||
			!ReadI32Value(*zsize, nativeQuery.zsize, error, "zsize"))
			return false;
		const WasmValueRecord* params = nullptr;
		if (!ReadRecordValue(*paramsValue, params, error, "params") ||
			!ReadGfxTextureParams(*params, nativeQuery.params, error))
			return false;
		if (nativeInterface->gfx == nullptr || nativeInterface->gfx->CreateTexture == nullptr) {
			error = "NativeInterface API is unavailable: gfx.CreateTexture";
			return false;
		}
		GfxStringResult nativeResult{};
		nativeInterface->gfx->CreateTexture(&nativeQuery, &nativeResult);
		if (!CheckNativeError(nativeResult, error))
			return false;
		result = WasmValue::String(nativeResult.value == nullptr ? std::string{} : std::string(nativeResult.value));
		return true;
	}

	if (module == "gfx" && IsFunction(function, "CreateTextureAtlas", "create-texture-atlas")) {
		const WasmValue* xsize = ArgumentValue(arguments, 0, "xsize", error);
		const WasmValue* ysize = ArgumentValue(arguments, 1, "ysize", error);
		const WasmValue* allocType = ArgumentValue(arguments, 2, "allocType", error);
		if (xsize == nullptr || ysize == nullptr || allocType == nullptr)
			return false;
		GfxCreateTextureAtlasQuery nativeQuery{};
		if (!ReadI32Value(*xsize, nativeQuery.xsize, error, "xsize") ||
			!ReadI32Value(*ysize, nativeQuery.ysize, error, "ysize") ||
			!ReadI32Value(*allocType, nativeQuery.allocType, error, "allocType"))
			return false;
		if (nativeInterface->gfx == nullptr || nativeInterface->gfx->CreateTextureAtlas == nullptr) {
			error = "NativeInterface API is unavailable: gfx.CreateTextureAtlas";
			return false;
		}
		GfxStringResult nativeResult{};
		nativeInterface->gfx->CreateTextureAtlas(&nativeQuery, &nativeResult);
		if (!CheckNativeError(nativeResult, error))
			return false;
		result = WasmValue::String(nativeResult.value == nullptr ? std::string{} : std::string(nativeResult.value));
		return true;
	}

	if (module == "gfx" && IsFunction(function, "ActiveShader", "active-shader")) {
		const WasmValue* shaderValue = ArgumentValue(arguments, 0, "shaderID", error);
		if (shaderValue == nullptr)
			return false;
		std::uint32_t shaderID = 0;
		if (!ReadU32Value(*shaderValue, shaderID, error, "shaderID"))
			return false;
		GuestCallbackContext* context = nullptr;
		std::shared_ptr<GuestCallbackState> state;
		if (!MakeGuestCallbackFromArguments(owner, arguments, 1, 2,
			std::numeric_limits<std::size_t>::max(), context, state, function, error))
			return false;
		if (nativeInterface->gfx == nullptr || nativeInterface->gfx->ActiveShader == nullptr) {
			error = "NativeInterface API is unavailable: gfx.ActiveShader";
			DestroyFailedGuestCallbackContext(context);
			return false;
		}
		GfxActiveShaderQuery nativeQuery{shaderID, GuestCallbackThunk, context};
		GfxEmptyResult nativeResult{};
		nativeInterface->gfx->ActiveShader(&nativeQuery, &nativeResult);
		const bool success = CheckNativeError(nativeResult, error);
		DestroyGuestCallbackContext(context);
		if (!success)
			return false;
		result = WasmValue::Unit();
		return true;
	}

	if (module == "gfx" && IsFunction(function, "ActiveFBO", "active-fbo")) {
		const WasmValue* fboValue = ArgumentValue(arguments, 0, "fboID", error);
		const WasmValue* targetValue = ArgumentValue(arguments, 1, "target", error);
		const WasmValue* identitiesValue = ArgumentValue(arguments, 2, "identities", error);
		if (fboValue == nullptr || targetValue == nullptr || identitiesValue == nullptr)
			return false;
		std::uint32_t fboID = 0;
		std::uint32_t target = 0;
		bool identities = false;
		if (!ReadU32Value(*fboValue, fboID, error, "fboID") ||
			!ReadU32Value(*targetValue, target, error, "target") ||
			!ReadBoolValue(*identitiesValue, identities, error, "identities"))
			return false;
		GuestCallbackContext* context = nullptr;
		std::shared_ptr<GuestCallbackState> state;
		if (!MakeGuestCallbackFromArguments(owner, arguments, 3, 4,
			std::numeric_limits<std::size_t>::max(), context, state, function, error))
			return false;
		if (nativeInterface->gfx == nullptr || nativeInterface->gfx->ActiveFBO == nullptr) {
			error = "NativeInterface API is unavailable: gfx.ActiveFBO";
			DestroyFailedGuestCallbackContext(context);
			return false;
		}
		GfxActiveFBOQuery nativeQuery{fboID, target, identities, GuestCallbackThunk, context};
		GfxEmptyResult nativeResult{};
		nativeInterface->gfx->ActiveFBO(&nativeQuery, &nativeResult);
		const bool success = CheckNativeError(nativeResult, error);
		DestroyGuestCallbackContext(context);
		if (!success)
			return false;
		result = WasmValue::Unit();
		return true;
	}

	if (module == "gfx" && IsFunction(function, "BeginEnd", "begin-end")) {
		const WasmValue* primitiveValue = ArgumentValue(arguments, 0, "primitive", error);
		if (primitiveValue == nullptr)
			return false;
		std::uint32_t primitive = 0;
		if (!ReadU32Value(*primitiveValue, primitive, error, "primitive"))
			return false;
		GuestCallbackContext* context = nullptr;
		std::shared_ptr<GuestCallbackState> state;
		if (!MakeGuestCallbackFromArguments(owner, arguments, 1, 2,
			std::numeric_limits<std::size_t>::max(), context, state, function, error))
			return false;
		if (nativeInterface->gfx == nullptr || nativeInterface->gfx->BeginEnd == nullptr) {
			error = "NativeInterface API is unavailable: gfx.BeginEnd";
			DestroyFailedGuestCallbackContext(context);
			return false;
		}
		GfxBeginEndQuery nativeQuery{primitive, GuestCallbackThunk, context};
		GfxEmptyResult nativeResult{};
		nativeInterface->gfx->BeginEnd(&nativeQuery, &nativeResult);
		const bool success = CheckNativeError(nativeResult, error);
		DestroyGuestCallbackContext(context);
		if (!success)
			return false;
		result = WasmValue::Unit();
		return true;
	}

	if (module == "gfx" && IsFunction(function, "CreateList", "create-list")) {
		GuestCallbackContext* context = nullptr;
		std::shared_ptr<GuestCallbackState> state;
		if (!MakeGuestCallbackFromArguments(owner, arguments, 0, 1,
			std::numeric_limits<std::size_t>::max(), context, state, function, error))
			return false;
		if (nativeInterface->gfx == nullptr || nativeInterface->gfx->CreateList == nullptr) {
			error = "NativeInterface API is unavailable: gfx.CreateList";
			DestroyFailedGuestCallbackContext(context);
			return false;
		}
		GfxCallbackQuery nativeQuery{GuestCallbackThunk, context};
		GfxUIntResult nativeResult{};
		nativeInterface->gfx->CreateList(&nativeQuery, &nativeResult);
		const bool success = CheckNativeError(nativeResult, error);
		DestroyGuestCallbackContext(context);
		if (!success)
			return false;
		result = WasmValue::U64(nativeResult.value);
		return true;
	}

	if (module == "gfx" && IsFunction(function, "DrawFuncAtUnit", "draw-func-at-unit")) {
		const WasmValue* unitValue = ArgumentValue(arguments, 0, "unitID", error);
		const WasmValue* midValue = ArgumentValue(arguments, 1, "useMidPos", error);
		if (unitValue == nullptr || midValue == nullptr)
			return false;
		std::int32_t unitID = 0;
		bool useMidPos = false;
		if (!ReadI32Value(*unitValue, unitID, error, "unitID") ||
			!ReadBoolValue(*midValue, useMidPos, error, "useMidPos"))
			return false;
		GuestCallbackContext* context = nullptr;
		std::shared_ptr<GuestCallbackState> state;
		if (!MakeGuestCallbackFromArguments(owner, arguments, 2, 3,
			std::numeric_limits<std::size_t>::max(), context, state, function, error))
			return false;
		if (nativeInterface->gfx == nullptr || nativeInterface->gfx->DrawFuncAtUnit == nullptr) {
			error = "NativeInterface API is unavailable: gfx.DrawFuncAtUnit";
			DestroyFailedGuestCallbackContext(context);
			return false;
		}
		GfxDrawFuncAtUnitQuery nativeQuery{unitID, useMidPos, GuestCallbackThunk, context};
		GfxEmptyResult nativeResult{};
		nativeInterface->gfx->DrawFuncAtUnit(&nativeQuery, &nativeResult);
		const bool success = CheckNativeError(nativeResult, error);
		DestroyGuestCallbackContext(context);
		if (!success)
			return false;
		result = WasmValue::Unit();
		return true;
	}

	if (module == "gfx" && IsFunction(function, "PushPopMatrix", "push-pop-matrix")) {
		GuestCallbackContext* context = nullptr;
		std::shared_ptr<GuestCallbackState> state;
		if (!MakeGuestCallbackFromArguments(owner, arguments, 0, 1,
			std::numeric_limits<std::size_t>::max(), context, state, function, error))
			return false;
		if (nativeInterface->gfx == nullptr || nativeInterface->gfx->PushPopMatrix == nullptr) {
			error = "NativeInterface API is unavailable: gfx.PushPopMatrix";
			DestroyFailedGuestCallbackContext(context);
			return false;
		}
		GfxCallbackQuery nativeQuery{GuestCallbackThunk, context};
		GfxEmptyResult nativeResult{};
		nativeInterface->gfx->PushPopMatrix(&nativeQuery, &nativeResult);
		const bool success = CheckNativeError(nativeResult, error);
		DestroyGuestCallbackContext(context);
		if (!success)
			return false;
		result = WasmValue::Unit();
		return true;
	}

	if (module == "gfx" && IsFunction(function, "RenderToTexture", "render-to-texture")) {
		const WasmValue* nameValue = ArgumentValue(arguments, 0, "name", error);
		if (nameValue == nullptr)
			return false;
		std::string name;
		if (!ReadStringValue(*nameValue, name, error, "name"))
			return false;
		GuestCallbackContext* context = nullptr;
		std::shared_ptr<GuestCallbackState> state;
		if (!MakeGuestCallbackFromArguments(owner, arguments, 1, 2,
			std::numeric_limits<std::size_t>::max(), context, state, function, error))
			return false;
		if (nativeInterface->gfx == nullptr || nativeInterface->gfx->RenderToTexture == nullptr) {
			error = "NativeInterface API is unavailable: gfx.RenderToTexture";
			DestroyFailedGuestCallbackContext(context);
			return false;
		}
		GfxRenderToTextureQuery nativeQuery{name.c_str(), GuestCallbackThunk, context};
		GfxEmptyResult nativeResult{};
		nativeInterface->gfx->RenderToTexture(&nativeQuery, &nativeResult);
		const bool success = CheckNativeError(nativeResult, error);
		DestroyGuestCallbackContext(context);
		if (!success)
			return false;
		result = WasmValue::Unit();
		return true;
	}

	if (module == "gfx" && IsFunction(function, "RunQuery", "run-query")) {
		const WasmValue* idValue = ArgumentValue(arguments, 0, "id", error);
		if (idValue == nullptr)
			return false;
		std::uint32_t id = 0;
		if (!ReadU32Value(*idValue, id, error, "id"))
			return false;
		GuestCallbackContext* context = nullptr;
		std::shared_ptr<GuestCallbackState> state;
		if (!MakeGuestCallbackFromArguments(owner, arguments, 1, 2,
			std::numeric_limits<std::size_t>::max(), context, state, function, error))
			return false;
		if (nativeInterface->gfx == nullptr || nativeInterface->gfx->RunQuery == nullptr) {
			error = "NativeInterface API is unavailable: gfx.RunQuery";
			DestroyFailedGuestCallbackContext(context);
			return false;
		}
		GfxRunQueryQuery nativeQuery{id, GuestCallbackThunk, context};
		GfxEmptyResult nativeResult{};
		nativeInterface->gfx->RunQuery(&nativeQuery, &nativeResult);
		const bool success = CheckNativeError(nativeResult, error);
		DestroyGuestCallbackContext(context);
		if (!success)
			return false;
		result = WasmValue::Unit();
		return true;
	}

	if (module == "gfx" && IsFunction(function, "UnsafeState", "unsafe-state")) {
		const WasmValue* stateValue = ArgumentValue(arguments, 0, "state", error);
		const WasmValue* reverseValue = ArgumentValue(arguments, 1, "reverse", error);
		if (stateValue == nullptr || reverseValue == nullptr)
			return false;
		std::uint32_t stateID = 0;
		bool reverse = false;
		if (!ReadU32Value(*stateValue, stateID, error, "state") ||
			!ReadBoolValue(*reverseValue, reverse, error, "reverse"))
			return false;
		GuestCallbackContext* context = nullptr;
		std::shared_ptr<GuestCallbackState> state;
		if (!MakeGuestCallbackFromArguments(owner, arguments, 2, 3,
			std::numeric_limits<std::size_t>::max(), context, state, function, error))
			return false;
		if (nativeInterface->gfx == nullptr || nativeInterface->gfx->UnsafeState == nullptr) {
			error = "NativeInterface API is unavailable: gfx.UnsafeState";
			DestroyFailedGuestCallbackContext(context);
			return false;
		}
		GfxUnsafeStateQuery nativeQuery{stateID, reverse, GuestCallbackThunk, context};
		GfxEmptyResult nativeResult{};
		nativeInterface->gfx->UnsafeState(&nativeQuery, &nativeResult);
		const bool success = CheckNativeError(nativeResult, error);
		DestroyGuestCallbackContext(context);
		if (!success)
			return false;
		result = WasmValue::Unit();
		return true;
	}

	if (module == "system_control" && IsFunction(function, "CallAsTeam", "call-as-team")) {
		const WasmValue* teamValue = ArgumentValue(arguments, 0, "teamID", error);
		if (teamValue == nullptr)
			return false;
		std::int32_t teamID = 0;
		if (!ReadI32Value(*teamValue, teamID, error, "teamID"))
			return false;
		GuestCallbackContext* context = nullptr;
		std::shared_ptr<GuestCallbackState> state;
		if (!MakeGuestCallbackFromArguments(owner, arguments, 1, 2,
			std::numeric_limits<std::size_t>::max(), context, state, function, error))
			return false;
		if (nativeInterface->systemControl == nullptr || nativeInterface->systemControl->CallAsTeam == nullptr) {
			error = "NativeInterface API is unavailable: system_control.CallAsTeam";
			DestroyFailedGuestCallbackContext(context);
			return false;
		}
		CallAsTeamQuery nativeQuery{teamID, GuestCallbackThunk, context};
		CallAsTeamResult nativeResult{};
		nativeInterface->systemControl->CallAsTeam(&nativeQuery, &nativeResult);
		const bool success = CheckNativeError(nativeResult, error);
		DestroyGuestCallbackContext(context);
		if (!success)
			return false;
		result = WasmBoolValue(nativeResult.success);
		return true;
	}

	if (module == "terrain_control" &&
		(IsFunction(function, "SetHeightMapFunc", "set-height-map-func") ||
		 IsFunction(function, "SetOriginalHeightMapFunc", "set-original-height-map-func") ||
		 IsFunction(function, "SetSmoothMeshFunc", "set-smooth-mesh-func"))) {
		GuestCallbackContext* context = nullptr;
		std::shared_ptr<GuestCallbackState> state;
		if (!MakeGuestCallbackFromArguments(owner, arguments, 0, 1,
			std::numeric_limits<std::size_t>::max(), context, state, function, error))
			return false;
		if (nativeInterface->syncedCtrl == nullptr || nativeInterface->syncedCtrl->terrain == nullptr) {
			error = "NativeInterface API is unavailable: terrain control callbacks";
			DestroyFailedGuestCallbackContext(context);
			return false;
		}
		bool success = false;
		if (IsFunction(function, "SetHeightMapFunc", "set-height-map-func")) {
			if (nativeInterface->syncedCtrl->terrain->SetHeightMapFunc == nullptr) {
				error = "NativeInterface API is unavailable: terrain_control.SetHeightMapFunc";
				DestroyFailedGuestCallbackContext(context);
				return false;
			}
			SetHeightMapFuncQuery nativeQuery{GuestCallbackThunk, context};
			SetHeightMapFuncResult nativeResult{};
			nativeInterface->syncedCtrl->terrain->SetHeightMapFunc(&nativeQuery, &nativeResult);
			success = CheckNativeError(nativeResult, error);
			if (success)
				result = WasmBoolValue(nativeResult.success);
		} else if (IsFunction(function, "SetOriginalHeightMapFunc", "set-original-height-map-func")) {
			if (nativeInterface->syncedCtrl->terrain->SetOriginalHeightMapFunc == nullptr) {
				error = "NativeInterface API is unavailable: terrain_control.SetOriginalHeightMapFunc";
				DestroyFailedGuestCallbackContext(context);
				return false;
			}
			SetOriginalHeightMapFuncQuery nativeQuery{GuestCallbackThunk, context};
			SetOriginalHeightMapFuncResult nativeResult{};
			nativeInterface->syncedCtrl->terrain->SetOriginalHeightMapFunc(&nativeQuery, &nativeResult);
			success = CheckNativeError(nativeResult, error);
			if (success)
				result = WasmBoolValue(nativeResult.success);
		} else {
			if (nativeInterface->syncedCtrl->terrain->SetSmoothMeshFunc == nullptr) {
				error = "NativeInterface API is unavailable: terrain_control.SetSmoothMeshFunc";
				DestroyFailedGuestCallbackContext(context);
				return false;
			}
			SetSmoothMeshFuncQuery nativeQuery{GuestCallbackThunk, context};
			SetSmoothMeshFuncResult nativeResult{};
			nativeInterface->syncedCtrl->terrain->SetSmoothMeshFunc(&nativeQuery, &nativeResult);
			success = CheckNativeError(nativeResult, error);
			if (success)
				result = WasmBoolValue(nativeResult.success);
		}
		DestroyGuestCallbackContext(context);
		return success;
	}

	if (module == "vfs" && IsFunction(function, "UseArchive", "use-archive")) {
		const WasmValue* archiveValue = ArgumentValue(arguments, 0, "archiveName", error);
		if (archiveValue == nullptr)
			return false;
		std::string archiveName;
		if (!ReadStringValue(*archiveValue, archiveName, error, "archiveName"))
			return false;
		GuestCallbackContext* context = nullptr;
		std::shared_ptr<GuestCallbackState> state;
		if (!MakeGuestCallbackFromArguments(owner, arguments, 1, 2,
			std::numeric_limits<std::size_t>::max(), context, state, function, error))
			return false;
		if (nativeInterface->vfs == nullptr || nativeInterface->vfs->UseArchive == nullptr) {
			error = "NativeInterface API is unavailable: vfs.UseArchive";
			DestroyFailedGuestCallbackContext(context);
			return false;
		}
		UseArchiveQuery nativeQuery{archiveName.c_str(), GuestCallbackThunk, context};
		UseArchiveResult nativeResult{};
		nativeInterface->vfs->UseArchive(&nativeQuery, &nativeResult);
		const bool success = CheckNativeError(nativeResult, error);
		DestroyGuestCallbackContext(context);
		if (!success)
			return false;
		result = WasmBoolValue(nativeResult.success);
		return true;
	}

	if (module == "rml_ui" && IsFunction(function, "ContextAddEventListener", "context-add-event-listener")) {
		const WasmValue* contextValue = ArgumentValue(arguments, 0, "contextHandle", error);
		const WasmValue* eventValue = ArgumentValue(arguments, 1, "event", error);
		const WasmValue* captureValue = ArgumentValue(arguments, 2, "inCapturePhase", error);
		if (contextValue == nullptr || eventValue == nullptr || captureValue == nullptr)
			return false;
		std::uint64_t contextHandle = 0;
		std::string event;
		bool inCapturePhase = false;
		if (!ReadU64Value(*contextValue, contextHandle, error, "contextHandle") ||
			!ReadStringValue(*eventValue, event, error, "event") ||
			!ReadBoolValue(*captureValue, inCapturePhase, error, "inCapturePhase"))
			return false;
		GuestCallbackContext* context = nullptr;
		std::shared_ptr<GuestCallbackState> state;
		if (!MakeGuestCallbackFromArguments(owner, arguments, 3, 4, 5,
			context, state, function, error))
			return false;
		if (nativeInterface->rmlUi == nullptr || nativeInterface->rmlUi->ContextAddEventListener == nullptr) {
			error = "NativeInterface API is unavailable: rml_ui.ContextAddEventListener";
			DestroyFailedGuestCallbackContext(context);
			return false;
		}
		RmlContextEventListenerCallbackQuery nativeQuery{
			.contextHandle = contextHandle,
			.event = event.c_str(),
			.inCapturePhase = inCapturePhase,
			.callback = GuestCallbackThunk,
			.userData = context,
			.destroyCallback = DestroyGuestCallbackContext,
		};
		RmlEventListenerCallbackResult nativeResult{};
		nativeInterface->rmlUi->ContextAddEventListener(&nativeQuery, &nativeResult);
		if (!CheckNativeError(nativeResult, error)) {
			if (!state->destroyed)
				DestroyGuestCallbackContext(context);
			return false;
		}
		result = WasmValue::Record({
			{"eventListenerHandle", WasmValue::U64(nativeResult.eventListenerHandle)},
			{"success", WasmValue::Bool(nativeResult.success)},
		});
		if (owner != nullptr && nativeResult.success && nativeResult.eventListenerHandle != 0) {
			const auto* rmlUi = nativeInterface->rmlUi;
			const std::uint64_t eventListenerHandle = nativeResult.eventListenerHandle;
			const std::string cleanupEvent = event;
			const auto cleanup = [rmlUi, contextHandle, eventListenerHandle, cleanupEvent,
				inCapturePhase]() {
				if (rmlUi == nullptr || rmlUi->ContextRemoveEventListener == nullptr)
					return;
				RmlContextEventListenerRemoveQuery query{
					.contextHandle = contextHandle,
					.eventListenerHandle = eventListenerHandle,
					.event = cleanupEvent.c_str(),
					.inCapturePhase = inCapturePhase,
				};
				RmlElementBoolResult cleanupResult{};
				rmlUi->ContextRemoveEventListener(&query, &cleanupResult);
			};
			if (!owner->RegisterCleanup(cleanup)) {
				cleanup();
				error = "could not register RmlUi context event-listener cleanup for Wasm module";
				return false;
			}
		}
		return true;
	}

	if (module == "rml_ui" && IsFunction(function, "ElementAddEventListener", "element-add-event-listener")) {
		const WasmValue* elementValue = ArgumentValue(arguments, 0, "elementHandle", error);
		const WasmValue* eventValue = ArgumentValue(arguments, 1, "event", error);
		const WasmValue* captureValue = ArgumentValue(arguments, 2, "inCapturePhase", error);
		if (elementValue == nullptr || eventValue == nullptr || captureValue == nullptr)
			return false;
		std::uint64_t elementHandle = 0;
		std::string event;
		bool inCapturePhase = false;
		if (!ReadU64Value(*elementValue, elementHandle, error, "elementHandle") ||
			!ReadStringValue(*eventValue, event, error, "event") ||
			!ReadBoolValue(*captureValue, inCapturePhase, error, "inCapturePhase"))
			return false;
		GuestCallbackContext* context = nullptr;
		std::shared_ptr<GuestCallbackState> state;
		if (!MakeGuestCallbackFromArguments(owner, arguments, 3, 4, 5,
			context, state, function, error))
			return false;
		if (nativeInterface->rmlUi == nullptr || nativeInterface->rmlUi->ElementAddEventListener == nullptr) {
			error = "NativeInterface API is unavailable: rml_ui.ElementAddEventListener";
			DestroyFailedGuestCallbackContext(context);
			return false;
		}
		RmlEventListenerCallbackQuery nativeQuery{
			.elementHandle = elementHandle,
			.event = event.c_str(),
			.inCapturePhase = inCapturePhase,
			.callback = GuestCallbackThunk,
			.userData = context,
			.destroyCallback = DestroyGuestCallbackContext,
		};
		RmlEventListenerCallbackResult nativeResult{};
		nativeInterface->rmlUi->ElementAddEventListener(&nativeQuery, &nativeResult);
		if (!CheckNativeError(nativeResult, error)) {
			if (!state->destroyed)
				DestroyGuestCallbackContext(context);
			return false;
		}
		result = WasmValue::Record({
			{"eventListenerHandle", WasmValue::U64(nativeResult.eventListenerHandle)},
			{"success", WasmValue::Bool(nativeResult.success)},
		});
		if (owner != nullptr && nativeResult.success && nativeResult.eventListenerHandle != 0) {
			const auto* rmlUi = nativeInterface->rmlUi;
			const std::uint64_t eventListenerHandle = nativeResult.eventListenerHandle;
			const std::string cleanupEvent = event;
			const auto cleanup = [rmlUi, elementHandle, eventListenerHandle, cleanupEvent,
				inCapturePhase]() {
				if (rmlUi == nullptr || rmlUi->ElementRemoveEventListener == nullptr)
					return;
				RmlElementEventListenerRemoveQuery query{
					.elementHandle = elementHandle,
					.eventListenerHandle = eventListenerHandle,
					.event = cleanupEvent.c_str(),
					.inCapturePhase = inCapturePhase,
				};
				RmlElementBoolResult cleanupResult{};
				rmlUi->ElementRemoveEventListener(&query, &cleanupResult);
			};
			if (!owner->RegisterCleanup(cleanup)) {
				cleanup();
				error = "could not register RmlUi element event-listener cleanup for Wasm module";
				return false;
			}
		}
		return true;
	}

	if (module == "rml_ui" && IsFunction(function, "DataModelBindEvent", "data-model-bind-event")) {
		const WasmValue* modelValue = ArgumentValue(arguments, 0, "dataModelHandle", error);
		const WasmValue* nameValue = ArgumentValue(arguments, 1, "name", error);
		const WasmValue* typesValue = ArgumentValue(arguments, 5, "fieldTypes", error);
		if (modelValue == nullptr || nameValue == nullptr || typesValue == nullptr)
			return false;
		std::uint64_t dataModelHandle = 0;
		std::string name;
		std::vector<std::uint8_t> fieldTypes;
		if (!ReadU64Value(*modelValue, dataModelHandle, error, "dataModelHandle") ||
			!ReadStringValue(*nameValue, name, error, "name") ||
			!ReadByteListValue(*typesValue, fieldTypes, error, "fieldTypes"))
			return false;
		GuestCallbackContext* context = nullptr;
		std::shared_ptr<GuestCallbackState> state;
		if (!MakeGuestCallbackFromArguments(owner, arguments, 2, 3, 4,
			context, state, function, error))
			return false;
		if (nativeInterface->rmlUi == nullptr || nativeInterface->rmlUi->DataModelBindEvent == nullptr) {
			error = "NativeInterface API is unavailable: rml_ui.DataModelBindEvent";
			DestroyFailedGuestCallbackContext(context);
			return false;
		}
		RmlDataModelBindEventQuery nativeQuery{
			.dataModelHandle = dataModelHandle,
			.name = name.c_str(),
			.callback = GuestDataCallbackThunk,
			.userData = context,
			.destroyCallback = DestroyGuestCallbackContext,
			.fieldTypes = fieldTypes.empty() ? nullptr : fieldTypes.data(),
			.fieldCount = fieldTypes.size(),
		};
		RmlDataModelBindEventResult nativeResult{};
		nativeInterface->rmlUi->DataModelBindEvent(&nativeQuery, &nativeResult);
		if (!CheckNativeError(nativeResult, error)) {
			if (!state->destroyed)
				DestroyGuestCallbackContext(context);
			return false;
		}
		result = WasmValue::Record({
			{"eventHandle", WasmValue::U64(nativeResult.eventHandle)},
			{"success", WasmValue::Bool(nativeResult.success)},
		});
		if (owner != nullptr && nativeResult.success && nativeResult.eventHandle != 0) {
			const auto* rmlUi = nativeInterface->rmlUi;
			const std::uint64_t eventHandle = nativeResult.eventHandle;
			const auto cleanup = [rmlUi, eventHandle]() {
				if (rmlUi == nullptr || rmlUi->DataModelUnbindEvent == nullptr)
					return;
				RmlDataModelEventHandleQuery query{.eventHandle = eventHandle};
				RmlElementBoolResult cleanupResult{};
				rmlUi->DataModelUnbindEvent(&query, &cleanupResult);
			};
			if (!owner->RegisterCleanup(cleanup)) {
				cleanup();
				error = "could not register RmlUi data-model event cleanup for Wasm module";
				return false;
			}
		}
		return true;
	}

	if (module == "rml_ui" && IsFunction(function, "DataModelUnbindEvent", "data-model-unbind-event")) {
		const WasmValue* eventValue = ArgumentValue(arguments, 0, "eventHandle", error);
		if (eventValue == nullptr)
			return false;
		std::uint64_t eventHandle = 0;
		if (!ReadU64Value(*eventValue, eventHandle, error, "eventHandle"))
			return false;
		if (nativeInterface->rmlUi == nullptr || nativeInterface->rmlUi->DataModelUnbindEvent == nullptr) {
			error = "NativeInterface API is unavailable: rml_ui.DataModelUnbindEvent";
			return false;
		}
		RmlDataModelEventHandleQuery nativeQuery{.eventHandle = eventHandle};
		RmlElementBoolResult nativeResult{};
		nativeInterface->rmlUi->DataModelUnbindEvent(&nativeQuery, &nativeResult);
		if (!CheckNativeError(nativeResult, error))
			return false;
		result = WasmBoolValue(nativeResult.success);
		return true;
	}

	if (module == "rml_ui" &&
		(IsFunction(function, "EventListenerOnAttach", "event-listener-on-attach") ||
		 IsFunction(function, "EventListenerOnDetach", "event-listener-on-detach"))) {
		const WasmValue* listenerValue = ArgumentValue(arguments, 0, "eventListenerHandle", error);
		const WasmValue* elementValue = ArgumentValue(arguments, 1, "elementHandle", error);
		if (listenerValue == nullptr || elementValue == nullptr)
			return false;
		std::uint64_t listenerHandle = 0;
		std::uint64_t elementHandle = 0;
		if (!ReadU64Value(*listenerValue, listenerHandle, error, "eventListenerHandle") ||
			!ReadU64Value(*elementValue, elementHandle, error, "elementHandle"))
			return false;
		if (nativeInterface->rmlUi == nullptr)
			error = "NativeInterface API is unavailable: rml_ui event listener";
		else if ((IsFunction(function, "EventListenerOnAttach", "event-listener-on-attach") &&
				nativeInterface->rmlUi->EventListenerOnAttach == nullptr) ||
			(IsFunction(function, "EventListenerOnDetach", "event-listener-on-detach") &&
				nativeInterface->rmlUi->EventListenerOnDetach == nullptr))
			error = "NativeInterface API is unavailable: rml_ui event listener callback";
		else {
			RmlEventListenerElementQuery nativeQuery{
				.eventListenerHandle = listenerHandle,
				.elementHandle = elementHandle,
			};
			RmlElementBoolResult nativeResult{};
			if (IsFunction(function, "EventListenerOnAttach", "event-listener-on-attach"))
				nativeInterface->rmlUi->EventListenerOnAttach(&nativeQuery, &nativeResult);
			else
				nativeInterface->rmlUi->EventListenerOnDetach(&nativeQuery, &nativeResult);
			if (!CheckNativeError(nativeResult, error))
				return false;
			result = WasmBoolValue(nativeResult.success);
			return true;
		}
		return false;
	}

	if (module == "rml_ui" && IsFunction(function, "EventListenerProcessEvent", "event-listener-process-event")) {
		const WasmValue* listenerValue = ArgumentValue(arguments, 0, "eventListenerHandle", error);
		const WasmValue* eventValue = ArgumentValue(arguments, 1, "eventHandle", error);
		if (listenerValue == nullptr || eventValue == nullptr)
			return false;
		std::uint64_t listenerHandle = 0;
		std::uint64_t eventHandle = 0;
		if (!ReadU64Value(*listenerValue, listenerHandle, error, "eventListenerHandle") ||
			!ReadU64Value(*eventValue, eventHandle, error, "eventHandle"))
			return false;
		if (nativeInterface->rmlUi == nullptr || nativeInterface->rmlUi->EventListenerProcessEvent == nullptr) {
			error = "NativeInterface API is unavailable: rml_ui.EventListenerProcessEvent";
			return false;
		}
		RmlEventListenerEventQuery nativeQuery{
			.eventListenerHandle = listenerHandle,
			.eventHandle = eventHandle,
		};
		RmlElementBoolResult nativeResult{};
		nativeInterface->rmlUi->EventListenerProcessEvent(&nativeQuery, &nativeResult);
		if (!CheckNativeError(nativeResult, error))
			return false;
		result = WasmBoolValue(nativeResult.success);
		return true;
	}

	if (module == "units_query" && IsFunction(function, "ValidUnitID", "valid-unit-id")) {
		const WasmValueRecord* query = RecordArgument(arguments, error);
		if (query == nullptr) return false;
		if (nativeInterface->unitsQuery == nullptr || nativeInterface->unitsQuery->ValidUnitID == nullptr) {
			error = "NativeInterface API is unavailable: units_query.ValidUnitID";
			return false;
		}
		ValidUnitIDQuery nativeQuery{};
		if (!ReadI32(*query, "unitID", nativeQuery.unitID, error)) return false;
		ValidUnitIDResult nativeResult{};
		nativeInterface->unitsQuery->ValidUnitID(&nativeQuery, &nativeResult);
		if (!CheckNativeError(nativeResult, error)) return false;
		result = WasmValue::Bool(nativeResult.valid);
		return true;
	}

	if (module == "player") {
		if (IsFunction(function, "GetLocalPlayerID", "get-local-player-id") ||
			IsFunction(function, "GetLocalTeamID", "get-local-team-id") ||
			IsFunction(function, "GetLocalAllyTeamID", "get-local-ally-team-id")) {
			if (!arguments.empty()) {
				error = "local player query takes no arguments";
				return false;
			}
			if (IsFunction(function, "GetLocalPlayerID", "get-local-player-id")) {
				if (nativeInterface->player == nullptr || nativeInterface->player->GetLocalPlayerID == nullptr) {
					error = "NativeInterface API is unavailable: player.GetLocalPlayerID";
					return false;
				}
				GetLocalPlayerIDQuery nativeQuery{};
				GetLocalPlayerIDResult nativeResult{};
				nativeInterface->player->GetLocalPlayerID(&nativeQuery, &nativeResult);
				if (!CheckNativeError(nativeResult, error)) return false;
				result = WasmValue::I64(nativeResult.playerID);
				return true;
			}
			if (IsFunction(function, "GetLocalTeamID", "get-local-team-id")) {
				if (nativeInterface->player == nullptr || nativeInterface->player->GetLocalTeamID == nullptr) {
					error = "NativeInterface API is unavailable: player.GetLocalTeamID";
					return false;
				}
				GetLocalTeamIDQuery nativeQuery{};
				GetLocalTeamIDResult nativeResult{};
				nativeInterface->player->GetLocalTeamID(&nativeQuery, &nativeResult);
				if (!CheckNativeError(nativeResult, error)) return false;
				result = WasmValue::I64(nativeResult.teamID);
				return true;
			}
			if (nativeInterface->player == nullptr || nativeInterface->player->GetLocalAllyTeamID == nullptr) {
				error = "NativeInterface API is unavailable: player.GetLocalAllyTeamID";
				return false;
			}
			GetLocalAllyTeamIDQuery nativeQuery{};
			GetLocalAllyTeamIDResult nativeResult{};
			nativeInterface->player->GetLocalAllyTeamID(&nativeQuery, &nativeResult);
			if (!CheckNativeError(nativeResult, error)) return false;
			result = WasmValue::I64(nativeResult.allyTeamID);
			return true;
		}
	}

	error = "no generated NativeInterface Wasm adapter for " + std::string(module) + "." +
		std::string(function);
	return false;
}
