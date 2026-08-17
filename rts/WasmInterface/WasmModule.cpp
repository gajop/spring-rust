/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmModule.h"

#include <algorithm>
#include <charconv>
#include <cctype>
#include <limits>
#include <unordered_set>
#include <utility>

#include "wasm/generated/WasmCalloutRegistry.h"

#if defined(RECOIL_WASMTIME_AVAILABLE)
#include <wasmtime.h>
#endif

namespace {

// Lua strings are byte sequences, while the Component Model string type is
// UTF-8. Preserve the engine's byte-for-byte Lua contract at the boundary by
// representing every native byte as the corresponding U+0000..U+00FF scalar.
// This keeps colour/control bytes in tooltips and console text valid for
// Wasmtime without silently replacing or dropping them.
std::string EncodeComponentString(std::string_view value)
{
	std::string result;
	result.reserve(value.size());
	for (const unsigned char byte : value) {
		if (byte < 0x80) {
			result.push_back(static_cast<char>(byte));
		} else if (byte < 0xC0) {
			result.push_back(static_cast<char>(0xC2));
			result.push_back(static_cast<char>(byte));
		} else {
			result.push_back(static_cast<char>(0xC3));
			result.push_back(static_cast<char>(byte - 0x40));
		}
	}
	return result;
}

std::string DecodeComponentString(std::string_view value)
{
	std::string result;
	result.reserve(value.size());
	for (std::size_t index = 0; index < value.size();) {
		const unsigned char first = static_cast<unsigned char>(value[index]);
		if (first < 0x80) {
			result.push_back(static_cast<char>(first));
			++index;
			continue;
		}

		std::uint32_t codePoint = 0;
		std::size_t length = 0;
		if (first >= 0xC2 && first <= 0xDF) {
			length = 2;
			codePoint = first & 0x1F;
		} else if (first >= 0xE0 && first <= 0xEF) {
			length = 3;
			codePoint = first & 0x0F;
		} else if (first >= 0xF0 && first <= 0xF4) {
			length = 4;
			codePoint = first & 0x07;
		}

		if (length == 0 || index + length > value.size()) {
			result.push_back(static_cast<char>(first));
			++index;
			continue;
		}
		bool valid = true;
		for (std::size_t offset = 1; offset < length; ++offset) {
			const unsigned char continuation = static_cast<unsigned char>(value[index + offset]);
			if ((continuation & 0xC0) != 0x80) {
				valid = false;
				break;
			}
			codePoint = (codePoint << 6) | (continuation & 0x3F);
		}
		if (!valid || (length == 2 && codePoint < 0x80) ||
			(length == 3 && codePoint < 0x800) ||
			(length == 4 && codePoint < 0x10000) ||
			(codePoint >= 0xD800 && codePoint <= 0xDFFF) ||
			codePoint > 0x10FFFF) {
			result.push_back(static_cast<char>(first));
			++index;
			continue;
		}

		if (codePoint <= 0xFF)
			result.push_back(static_cast<char>(codePoint));
		else
			result.append(value, index, length);
		index += length;
	}
	return result;
}

#if defined(RECOIL_WASMTIME_AVAILABLE)
std::string WasmtimeErrorMessage(wasmtime_error_t* error)
{
	if (error == nullptr)
		return {};
	wasm_name_t message;
	wasmtime_error_message(error, &message);
	std::string result(message.data, message.size);
	wasm_name_delete(&message);
	wasmtime_error_delete(error);
	return result;
}

std::string WasmTrapMessage(wasm_trap_t* trap)
{
	if (trap == nullptr)
		return {};
	wasm_message_t message;
	wasm_trap_message(trap, &message);
	std::string result(message.data, message.size);
	wasm_name_delete(&message);
	wasm_trap_delete(trap);
	return result;
}
#endif

bool ParseNativeApiError(std::string_view error, std::int32_t& code)
{
	constexpr std::string_view prefix = "native API error ";
	if (error.size() <= prefix.size() || error.substr(0, prefix.size()) != prefix)
		return false;

	std::string_view numeric = error.substr(prefix.size());
	if (const std::size_t separator = numeric.find(':'); separator != std::string_view::npos)
		numeric = numeric.substr(0, separator);
	if (numeric.empty())
		return false;

	std::int64_t parsed = 0;
	const auto [end, status] = std::from_chars(
		numeric.data(), numeric.data() + numeric.size(), parsed);
	if (status != std::errc{} || end != numeric.data() + numeric.size() ||
		parsed < std::numeric_limits<std::int32_t>::min() ||
		parsed > std::numeric_limits<std::int32_t>::max())
		return false;

	code = static_cast<std::int32_t>(parsed);
	return true;
}

bool IsComponent(const std::vector<std::uint8_t>& bytes)
{
	return bytes.size() >= 8 && bytes[4] == 0x0d && bytes[5] == 0x00 &&
		bytes[6] == 0x01 && bytes[7] == 0x00;
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

const WasmValue* FindSemanticRecordField(const WasmValueRecord& record,
	std::string_view witName)
{
	if (const auto iter = record.find(std::string(witName)); iter != record.end())
		return &iter->second;
	for (const auto& [fieldName, fieldValue] : record) {
		if (ToWitFieldName(fieldName) == witName)
			return &fieldValue;
	}
	return nullptr;
}

std::size_t AddValueBytes(std::size_t total, std::size_t value)
{
	if (value > std::numeric_limits<std::size_t>::max() - total)
		return std::numeric_limits<std::size_t>::max();
	return total + value;
}

std::size_t WasmValueBytes(const WasmValue& value)
{
	if (const auto* string = std::get_if<std::string>(&value.storage))
		return string->size();
	if (const auto* bytes = std::get_if<std::vector<std::uint8_t>>(&value.storage))
		return bytes->size();
	if (const auto* list = std::get_if<WasmValueList>(&value.storage)) {
		std::size_t total = 0;
		for (const auto& element : *list)
			total = AddValueBytes(total, WasmValueBytes(element));
		return total;
	}
	if (const auto* record = std::get_if<WasmValueRecord>(&value.storage)) {
		std::size_t total = 0;
		for (const auto& [name, field] : *record) {
			total = AddValueBytes(total, name.size());
			total = AddValueBytes(total, WasmValueBytes(field));
		}
		return total;
	}
	if (const auto* variant = std::get_if<WasmValueVariant>(&value.storage)) {
		std::size_t total = variant->discriminant.size();
		if (variant->value != nullptr)
			total = AddValueBytes(total, WasmValueBytes(*variant->value));
		return total;
	}
	if (const auto* resource = std::get_if<WasmValueResource>(&value.storage))
		return AddValueBytes(sizeof(resource->handle), resource->family.size());
	return 0;
}

std::size_t WasmValuesBytes(const std::vector<WasmValue>& values)
{
	std::size_t total = 0;
	for (const auto& value : values)
		total = AddValueBytes(total, WasmValueBytes(value));
	return total;
}

#if defined(RECOIL_WASMTIME_AVAILABLE)
bool ChargeComponentValueBytes(std::size_t amount, std::size_t limit,
	std::size_t& used, std::string& error)
{
	if (used > limit || amount > limit - used) {
		error = "Wasm component value exceeds the configured byte limit";
		return false;
	}
	used += amount;
	return true;
}

bool CheckComponentValueBudget(const wasmtime_component_val_t& value,
	std::size_t byteLimit, std::uint32_t nodeLimit, std::uint32_t depthLimit,
	std::uint32_t depth, std::size_t& usedBytes, std::size_t& usedNodes,
	std::string& error)
{
	if (depth > depthLimit) {
		error = "Wasm component value nesting exceeds the configured limit";
		return false;
	}
	if (usedNodes == nodeLimit) {
		error = "Wasm component value node count exceeds the configured limit";
		return false;
	}
	++usedNodes;

	switch (value.kind) {
		case WASMTIME_COMPONENT_STRING:
			return ChargeComponentValueBytes(value.of.string.size, byteLimit, usedBytes, error);
		case WASMTIME_COMPONENT_LIST: {
			if (value.of.list.size != 0 && value.of.list.data == nullptr) {
				error = "Wasm component list has no data";
				return false;
			}
			bool byteList = true;
			for (std::size_t index = 0; index < value.of.list.size; ++index) {
				if (value.of.list.data[index].kind != WASMTIME_COMPONENT_U8) {
					byteList = false;
					break;
				}
			}
			if (byteList)
				return ChargeComponentValueBytes(value.of.list.size, byteLimit, usedBytes, error);
			for (std::size_t index = 0; index < value.of.list.size; ++index) {
				if (!CheckComponentValueBudget(value.of.list.data[index], byteLimit, nodeLimit,
					depthLimit, depth + 1, usedBytes, usedNodes, error))
					return false;
			}
			return true;
		}
		case WASMTIME_COMPONENT_RECORD:
			if (value.of.record.size != 0 && value.of.record.data == nullptr) {
				error = "Wasm component record has no data";
				return false;
			}
			for (std::size_t index = 0; index < value.of.record.size; ++index) {
				const auto& entry = value.of.record.data[index];
				if (!ChargeComponentValueBytes(entry.name.size, byteLimit, usedBytes, error) ||
					!CheckComponentValueBudget(entry.val, byteLimit, nodeLimit, depthLimit,
						depth + 1, usedBytes, usedNodes, error))
					return false;
			}
			return true;
		case WASMTIME_COMPONENT_TUPLE:
			if (value.of.tuple.size != 0 && value.of.tuple.data == nullptr) {
				error = "Wasm component tuple has no data";
				return false;
			}
			for (std::size_t index = 0; index < value.of.tuple.size; ++index) {
				if (!CheckComponentValueBudget(value.of.tuple.data[index], byteLimit, nodeLimit,
					depthLimit, depth + 1, usedBytes, usedNodes, error))
					return false;
			}
			return true;
		case WASMTIME_COMPONENT_OPTION:
			return value.of.option == nullptr || CheckComponentValueBudget(*value.of.option,
				byteLimit, nodeLimit, depthLimit, depth + 1, usedBytes, usedNodes, error);
		case WASMTIME_COMPONENT_RESULT:
			return value.of.result.val == nullptr || CheckComponentValueBudget(*value.of.result.val,
				byteLimit, nodeLimit, depthLimit, depth + 1, usedBytes, usedNodes, error);
		case WASMTIME_COMPONENT_ENUM:
			return ChargeComponentValueBytes(value.of.enumeration.size, byteLimit, usedBytes, error);
		case WASMTIME_COMPONENT_FLAGS:
			if (value.of.flags.size != 0 && value.of.flags.data == nullptr) {
				error = "Wasm component flags have no data";
				return false;
			}
			for (std::size_t index = 0; index < value.of.flags.size; ++index) {
				if (!ChargeComponentValueBytes(value.of.flags.data[index].size, byteLimit,
					usedBytes, error))
					return false;
			}
			return true;
		case WASMTIME_COMPONENT_VARIANT:
			if (!ChargeComponentValueBytes(value.of.variant.discriminant.size, byteLimit,
				usedBytes, error))
				return false;
			return value.of.variant.val == nullptr || CheckComponentValueBudget(
				*value.of.variant.val, byteLimit, nodeLimit, depthLimit, depth + 1,
				usedBytes, usedNodes, error);
		case WASMTIME_COMPONENT_RESOURCE:
			if (value.of.resource == nullptr) {
				error = "component resource value has no payload";
				return false;
			}
			return true;
		default:
			return true;
	}
}

struct WasmHostFunctionData {
	WasmModule* module = nullptr;
	std::string moduleName;
	std::string functionName;
};

std::string ComponentImportModule(std::string_view importName)
{
	const std::size_t slash = importName.find_last_of('/');
	std::string module(importName.substr(slash == std::string_view::npos ? 0 : slash + 1));
	const std::size_t version = module.find('@');
	if (version != std::string::npos)
		module.resize(version);
	for (char& character : module) {
		if (character == '-')
			character = '_';
	}
	return std::string(recoil::wasm::generated::CanonicalModule(module));
}

std::string ComponentImportFunction(std::string_view moduleName,
	std::string_view importName)
{
	// WIT uses kebab-case, while the NativeInterface keeps its historical
	// spelling, including initialisms such as COB and ID.  Resolve through the
	// generated callout inventory before applying the fallback conversion so
	// dispatch receives the canonical native symbol.
	for (const auto& descriptor : recoil::wasm::generated::kCallouts) {
		if (descriptor.module != moduleName)
			continue;
		std::string witName;
		witName.reserve(std::char_traits<char>::length(descriptor.name) + 4);
		for (std::size_t index = 0; descriptor.name[index] != '\0'; ++index) {
			const unsigned char character = static_cast<unsigned char>(descriptor.name[index]);
			const bool uppercase = std::isupper(character) != 0;
			const bool previousUppercase = index > 0 &&
				std::isupper(static_cast<unsigned char>(descriptor.name[index - 1])) != 0;
			const bool nextLowercase = descriptor.name[index + 1] != '\0' &&
				std::islower(static_cast<unsigned char>(descriptor.name[index + 1])) != 0;
			if (uppercase && index != 0 && (!previousUppercase || nextLowercase))
				witName.push_back('-');
			witName.push_back(static_cast<char>(std::tolower(character)));
		}
		if (witName == importName)
			return descriptor.name;
	}

	std::string function;
	bool uppercase = true;
	for (const char character : importName) {
		if (character == '-' || character == '_') {
			uppercase = true;
			continue;
		}
		if (uppercase) {
			function.push_back(static_cast<char>(std::toupper(static_cast<unsigned char>(character))));
			uppercase = false;
		} else {
			function.push_back(character);
		}
	}
	return function;
}

wasmtime_error_t* ComponentHostError(std::string_view message)
{
	return wasmtime_error_new(std::string(message).c_str());
}

void DeleteComponentValues(std::vector<wasmtime_component_val_t>& values)
{
	for (auto& value : values)
		wasmtime_component_val_delete(&value);
}

void DeleteComponentRecordFields(
	std::vector<wasmtime_component_valrecord_entry_t>& fields)
{
	for (auto& field : fields) {
		wasm_name_delete(&field.name);
		wasmtime_component_val_delete(&field.val);
	}
}

void DeleteComponentNames(std::vector<wasm_name_t>& names)
{
	for (auto& name : names)
		wasm_name_delete(&name);
}

#if defined(RECOIL_WASMTIME_AVAILABLE)
void CollectComponentExports(const wasmtime_component_type_t* type,
	const wasm_engine_t* engine, std::string_view prefix,
	std::unordered_set<std::string>& functions);

void CollectComponentInstanceExports(const wasmtime_component_instance_type_t* type,
	const wasm_engine_t* engine, std::string_view prefix,
	std::unordered_set<std::string>& functions)
{
	const std::size_t exportCount = wasmtime_component_instance_type_export_count(type, engine);
	for (std::size_t index = 0; index < exportCount; ++index) {
		const char* name = nullptr;
		std::size_t nameLength = 0;
		wasmtime_component_item_t item{};
		if (!wasmtime_component_instance_type_export_nth(type, engine, index,
				&name, &nameLength, &item))
			continue;

		std::string path(prefix);
		if (!path.empty())
			path.push_back('/');
		path.append(name, nameLength);
		switch (item.kind) {
			case WASMTIME_COMPONENT_ITEM_COMPONENT_FUNC:
				functions.insert(std::move(path));
				break;
			case WASMTIME_COMPONENT_ITEM_COMPONENT_INSTANCE:
				CollectComponentInstanceExports(item.of.component_instance, engine, path, functions);
				break;
			case WASMTIME_COMPONENT_ITEM_COMPONENT:
				CollectComponentExports(item.of.component, engine, path, functions);
				break;
			default:
				break;
		}
		wasmtime_component_item_delete(&item);
	}
}

void CollectComponentExports(const wasmtime_component_type_t* type,
	const wasm_engine_t* engine, std::string_view prefix,
	std::unordered_set<std::string>& functions)
{
	const std::size_t exportCount = wasmtime_component_type_export_count(type, engine);
	for (std::size_t index = 0; index < exportCount; ++index) {
		const char* name = nullptr;
		std::size_t nameLength = 0;
		wasmtime_component_item_t item{};
		if (!wasmtime_component_type_export_nth(type, engine, index,
				&name, &nameLength, &item))
		continue;

		std::string path(prefix);
		if (!path.empty())
			path.push_back('/');
		path.append(name, nameLength);
		switch (item.kind) {
			case WASMTIME_COMPONENT_ITEM_COMPONENT_FUNC:
				functions.insert(std::move(path));
				break;
			case WASMTIME_COMPONENT_ITEM_COMPONENT_INSTANCE:
				CollectComponentInstanceExports(item.of.component_instance, engine, path, functions);
				break;
			case WASMTIME_COMPONENT_ITEM_COMPONENT:
				CollectComponentExports(item.of.component, engine, path, functions);
				break;
			default:
				break;
		}
		wasmtime_component_item_delete(&item);
	}
}
#endif

class ComponentExportIndexPath {
public:
	~ComponentExportIndexPath()
	{
		for (auto iter = indexes.rbegin(); iter != indexes.rend(); ++iter)
			wasmtime_component_export_index_delete(*iter);
	}

	bool Resolve(const wasmtime_component_instance_t& instance, wasmtime_context_t* context,
		std::string_view path)
	{
		const std::size_t separator = path.find_last_of('/');
		if (separator == std::string_view::npos) {
			auto* index = wasmtime_component_instance_get_export_index(
				&instance, context, nullptr, path.data(), path.size());
			if (index == nullptr)
				return false;
			indexes.push_back(index);
			return true;
		}

		// Package-qualified WIT interfaces are exported as one root item whose
		// name contains slashes (for example
		// `recoil:spring-api/callins-ui@1.0.0`). Resolve that root first, then
		// resolve the function inside it. Passing the complete
		// `interface/function` spelling to the C API is not equivalent and can
		// make a missing optional callin lookup scan indefinitely.
		const std::string_view interfaceName = path.substr(0, separator);
		const std::string_view functionName = path.substr(separator + 1);
		if (interfaceName.empty() || functionName.empty())
			return false;
		auto* interfaceIndex = wasmtime_component_instance_get_export_index(
			&instance, context, nullptr, interfaceName.data(), interfaceName.size());
		if (interfaceIndex == nullptr)
			return false;
		auto* functionIndex = wasmtime_component_instance_get_export_index(
			&instance, context, interfaceIndex, functionName.data(), functionName.size());
		if (functionIndex == nullptr) {
			wasmtime_component_export_index_delete(interfaceIndex);
			return false;
		}
		indexes.push_back(interfaceIndex);
		indexes.push_back(functionIndex);
		return true;
	}

	wasmtime_component_export_index_t* Get() const
	{
		return indexes.empty() ? nullptr : indexes.back();
	}

private:
	std::vector<wasmtime_component_export_index_t*> indexes;
};

bool LiftComponentValue(const wasmtime_component_val_t& value, WasmModule* module,
	void* context, WasmValue& output, std::string& error)
{
	switch (value.kind) {
		case WASMTIME_COMPONENT_BOOL:
			output = WasmValue::Bool(value.of.boolean);
			return true;
		case WASMTIME_COMPONENT_S8:
			output = WasmValue::I64(value.of.s8);
			return true;
		case WASMTIME_COMPONENT_U8:
			output = WasmValue::U64(value.of.u8);
			return true;
		case WASMTIME_COMPONENT_S16:
			output = WasmValue::I64(value.of.s16);
			return true;
		case WASMTIME_COMPONENT_U16:
			output = WasmValue::U64(value.of.u16);
			return true;
		case WASMTIME_COMPONENT_S32:
			output = WasmValue::I64(value.of.s32);
			return true;
		case WASMTIME_COMPONENT_U32:
			output = WasmValue::U64(value.of.u32);
			return true;
		case WASMTIME_COMPONENT_S64:
			output = WasmValue::I64(value.of.s64);
			return true;
		case WASMTIME_COMPONENT_U64:
			output = WasmValue::U64(value.of.u64);
			return true;
		case WASMTIME_COMPONENT_F32:
			output = WasmValue::F64(value.of.f32);
			return true;
		case WASMTIME_COMPONENT_F64:
			output = WasmValue::F64(value.of.f64);
			return true;
		case WASMTIME_COMPONENT_CHAR:
			output = WasmValue::U64(value.of.character);
			return true;
		case WASMTIME_COMPONENT_STRING:
			output = WasmValue::String(DecodeComponentString(
				std::string_view(value.of.string.data, value.of.string.size)));
			return true;
		case WASMTIME_COMPONENT_LIST: {
			WasmValueList list;
			list.reserve(value.of.list.size);
			for (std::size_t index = 0; index < value.of.list.size; ++index) {
				WasmValue element;
				if (!LiftComponentValue(value.of.list.data[index], module, context, element, error))
					return false;
				list.push_back(std::move(element));
			}
			output = WasmValue::List(std::move(list));
			return true;
		}
		case WASMTIME_COMPONENT_RECORD: {
			WasmValueRecord record;
			for (std::size_t index = 0; index < value.of.record.size; ++index) {
				const auto& entry = value.of.record.data[index];
				WasmValue field;
				if (!LiftComponentValue(entry.val, module, context, field, error))
					return false;
				record.emplace(std::string(entry.name.data, entry.name.size), std::move(field));
			}
			output = WasmValue::Record(std::move(record));
			return true;
		}
		case WASMTIME_COMPONENT_TUPLE: {
			WasmValueList tuple;
			tuple.reserve(value.of.tuple.size);
			for (std::size_t index = 0; index < value.of.tuple.size; ++index) {
				WasmValue element;
				if (!LiftComponentValue(value.of.tuple.data[index], module, context, element, error))
					return false;
				tuple.push_back(std::move(element));
			}
			output = tuple.empty() ? WasmValue::Unit() : WasmValue::List(std::move(tuple));
			return true;
		}
		case WASMTIME_COMPONENT_ENUM:
			output = WasmValue::String(std::string(value.of.enumeration.data,
				value.of.enumeration.size));
			return true;
		case WASMTIME_COMPONENT_OPTION:
			if (value.of.option == nullptr) {
				output = WasmValue::Unit();
				return true;
			}
			return LiftComponentValue(*value.of.option, module, context, output, error);
		case WASMTIME_COMPONENT_RESULT: {
			WasmValueRecord record;
			record.emplace("ok", WasmValue::Bool(value.of.result.is_ok));
			if (value.of.result.val != nullptr) {
				WasmValue payload;
				if (!LiftComponentValue(*value.of.result.val, module, context, payload, error))
					return false;
				record.emplace("value", std::move(payload));
			}
			output = WasmValue::Record(std::move(record));
			return true;
		}
		case WASMTIME_COMPONENT_FLAGS: {
			WasmValueList flags;
			flags.reserve(value.of.flags.size);
			for (std::size_t index = 0; index < value.of.flags.size; ++index) {
				const auto& flag = value.of.flags.data[index];
				flags.push_back(WasmValue::String(std::string(flag.data, flag.size)));
			}
			output = WasmValue::List(std::move(flags));
			return true;
		}
		case WASMTIME_COMPONENT_VARIANT: {
			if (value.of.variant.val == nullptr) {
				output = WasmValue::EmptyVariant(std::string(value.of.variant.discriminant.data,
					value.of.variant.discriminant.size));
				return true;
			}
			WasmValue payload;
			if (!LiftComponentValue(*value.of.variant.val, module, context, payload, error))
				return false;
			output = WasmValue::Variant(std::string(value.of.variant.discriminant.data,
				value.of.variant.discriminant.size), std::move(payload));
			return true;
		}
		case WASMTIME_COMPONENT_RESOURCE:
			if (module == nullptr || context == nullptr || value.of.resource == nullptr) {
				error = "component resource has no live module context";
				return false;
			}
			return module->ImportComponentResource(context, value.of.resource,
				wasmtime_component_resource_any_owned(value.of.resource), output, error);
		default:
			error = "unknown component value kind";
			return false;
	}
}

bool LiftComponentValueTyped(const wasmtime_component_val_t& value,
	const wasmtime_component_valtype_t& type, WasmModule* module, void* context,
	WasmValue& output, std::string& error)
{
	switch (type.kind) {
		case WASMTIME_COMPONENT_VALTYPE_LIST: {
			if (value.kind != WASMTIME_COMPONENT_LIST) {
				error = "component list value does not match its type";
				return false;
			}
			wasmtime_component_valtype_t elementType{};
			wasmtime_component_list_type_element(type.of.list, &elementType);
			if (elementType.kind == WASMTIME_COMPONENT_VALTYPE_U8) {
				std::vector<std::uint8_t> bytes;
				bytes.reserve(value.of.list.size);
				for (std::size_t index = 0; index < value.of.list.size; ++index) {
					if (value.of.list.data[index].kind != WASMTIME_COMPONENT_U8) {
						wasmtime_component_valtype_delete(&elementType);
						error = "component byte list contains a non-u8 value";
						return false;
					}
					bytes.push_back(value.of.list.data[index].of.u8);
				}
				wasmtime_component_valtype_delete(&elementType);
				output = WasmValue::Bytes(std::move(bytes));
				return true;
			}
			WasmValueList list;
			list.reserve(value.of.list.size);
			for (std::size_t index = 0; index < value.of.list.size; ++index) {
				WasmValue element;
				if (!LiftComponentValueTyped(value.of.list.data[index], elementType, module,
					context, element, error)) {
					wasmtime_component_valtype_delete(&elementType);
					return false;
				}
				list.push_back(std::move(element));
			}
			wasmtime_component_valtype_delete(&elementType);
			output = WasmValue::List(std::move(list));
			return true;
		}
		case WASMTIME_COMPONENT_VALTYPE_RECORD: {
			WasmValueRecord record;
			const std::size_t fieldCount = wasmtime_component_record_type_field_count(type.of.record);
			if (value.kind != WASMTIME_COMPONENT_RECORD || value.of.record.size != fieldCount) {
				error = "component record value does not match its type";
				return false;
			}
			for (std::size_t index = 0; index < fieldCount; ++index) {
				const char* name = nullptr;
				std::size_t nameLength = 0;
				wasmtime_component_valtype_t fieldType{};
				if (!wasmtime_component_record_type_field_nth(type.of.record, index, &name,
					&nameLength, &fieldType)) {
					error = "component record field type is unavailable";
					return false;
				}
				WasmValue field;
				const bool success = LiftComponentValueTyped(value.of.record.data[index].val,
					fieldType, module, context, field, error);
				wasmtime_component_valtype_delete(&fieldType);
				if (!success)
					return false;
				record.emplace(std::string(name, nameLength), std::move(field));
			}
			output = WasmValue::Record(std::move(record));
			return true;
		}
		case WASMTIME_COMPONENT_VALTYPE_TUPLE: {
			const std::size_t typeCount = wasmtime_component_tuple_type_types_count(type.of.tuple);
			if (value.kind != WASMTIME_COMPONENT_TUPLE || value.of.tuple.size != typeCount) {
				error = "component tuple value does not match its type";
				return false;
			}
			WasmValueList tuple;
			tuple.reserve(typeCount);
			for (std::size_t index = 0; index < typeCount; ++index) {
				wasmtime_component_valtype_t elementType{};
				if (!wasmtime_component_tuple_type_types_nth(type.of.tuple, index, &elementType)) {
					error = "component tuple element type is unavailable";
					return false;
				}
				WasmValue element;
				const bool success = LiftComponentValueTyped(value.of.tuple.data[index],
					 elementType, module, context, element, error);
				wasmtime_component_valtype_delete(&elementType);
				if (!success)
					return false;
				tuple.push_back(std::move(element));
			}
			output = tuple.empty() ? WasmValue::Unit() : WasmValue::List(std::move(tuple));
			return true;
		}
		case WASMTIME_COMPONENT_VALTYPE_OPTION: {
			if (value.kind != WASMTIME_COMPONENT_OPTION) {
				error = "component option value does not match its type";
				return false;
			}
			if (value.of.option == nullptr) {
				output = WasmValue::Unit();
				return true;
			}
			wasmtime_component_valtype_t innerType{};
			wasmtime_component_option_type_ty(type.of.option, &innerType);
				const bool success = LiftComponentValueTyped(*value.of.option, innerType,
					module, context, output, error);
			wasmtime_component_valtype_delete(&innerType);
			return success;
		}
		case WASMTIME_COMPONENT_VALTYPE_RESULT: {
			if (value.kind != WASMTIME_COMPONENT_RESULT) {
				error = "component result value does not match its type";
				return false;
			}
			WasmValueRecord record;
			record.emplace("ok", WasmValue::Bool(value.of.result.is_ok));
			if (value.of.result.val != nullptr) {
				wasmtime_component_valtype_t payloadType{};
				const bool hasType = value.of.result.is_ok ?
					wasmtime_component_result_type_ok(type.of.result, &payloadType) :
					wasmtime_component_result_type_err(type.of.result, &payloadType);
				if (!hasType) {
					error = "component result payload type is unavailable";
					return false;
				}
				WasmValue payload;
					const bool success = LiftComponentValueTyped(*value.of.result.val,
						payloadType, module, context, payload, error);
				wasmtime_component_valtype_delete(&payloadType);
				if (!success)
					return false;
				record.emplace("value", std::move(payload));
			}
			output = WasmValue::Record(std::move(record));
			return true;
		}
		case WASMTIME_COMPONENT_VALTYPE_VARIANT: {
			if (value.kind != WASMTIME_COMPONENT_VARIANT) {
				error = "component variant value does not match its type";
				return false;
			}
			const std::string discriminant(value.of.variant.discriminant.data,
				value.of.variant.discriminant.size);
			const std::size_t count = wasmtime_component_variant_type_case_count(
				type.of.variant);
			for (std::size_t index = 0; index < count; ++index) {
				const char* caseName = nullptr;
				std::size_t caseNameLength = 0;
				bool hasPayload = false;
				wasmtime_component_valtype_t payloadType{};
				if (!wasmtime_component_variant_type_case_nth(type.of.variant, index, &caseName,
						&caseNameLength, &hasPayload, &payloadType)) {
					error = "component variant case type is unavailable";
					return false;
				}
				if (discriminant != std::string(caseName, caseNameLength)) {
					if (hasPayload)
						wasmtime_component_valtype_delete(&payloadType);
					continue;
				}
				if (!hasPayload) {
					if (value.of.variant.val != nullptr) {
						error = "component variant case unexpectedly has a payload";
						return false;
					}
					output = WasmValue::EmptyVariant(discriminant);
					return true;
				}
				if (value.of.variant.val == nullptr) {
					wasmtime_component_valtype_delete(&payloadType);
					error = "component variant case is missing its payload";
					return false;
				}
				WasmValue payload;
				const bool success = LiftComponentValueTyped(*value.of.variant.val, payloadType,
					module, context, payload, error);
				wasmtime_component_valtype_delete(&payloadType);
				if (!success)
					return false;
				output = WasmValue::Variant(discriminant, std::move(payload));
				return true;
			}
			error = "component variant has an unknown discriminant";
			return false;
		}
		case WASMTIME_COMPONENT_VALTYPE_OWN:
		case WASMTIME_COMPONENT_VALTYPE_BORROW:
			if (value.kind != WASMTIME_COMPONENT_RESOURCE || value.of.resource == nullptr ||
				module == nullptr || context == nullptr) {
				error = "component resource value does not match its type";
				return false;
			}
			return module->ImportComponentResource(context, value.of.resource,
				type.kind == WASMTIME_COMPONENT_VALTYPE_OWN, output, error);
		default:
			return LiftComponentValue(value, module, context, output, error);
	}
}

bool ReadSignedValue(const WasmValue& value, std::int64_t& output)
{
	if (const auto* signedValue = std::get_if<std::int64_t>(&value.storage)) {
		output = *signedValue;
		return true;
	}
	if (const auto* unsignedValue = std::get_if<std::uint64_t>(&value.storage)) {
		if (*unsignedValue > static_cast<std::uint64_t>(std::numeric_limits<std::int64_t>::max()))
			return false;
		output = static_cast<std::int64_t>(*unsignedValue);
		return true;
	}
	return false;
}

bool ReadUnsignedValue(const WasmValue& value, std::uint64_t& output)
{
	if (const auto* unsignedValue = std::get_if<std::uint64_t>(&value.storage)) {
		output = *unsignedValue;
		return true;
	}
	if (const auto* signedValue = std::get_if<std::int64_t>(&value.storage)) {
		if (*signedValue < 0)
			return false;
		output = static_cast<std::uint64_t>(*signedValue);
		return true;
	}
	return false;
}

bool ReadFloatingValue(const WasmValue& value, double& output)
{
	if (const auto* floatingValue = std::get_if<double>(&value.storage)) {
		output = *floatingValue;
		return true;
	}
	std::int64_t signedValue = 0;
	if (ReadSignedValue(value, signedValue)) {
		output = static_cast<double>(signedValue);
		return true;
	}
	std::uint64_t unsignedValue = 0;
	if (ReadUnsignedValue(value, unsignedValue)) {
		output = static_cast<double>(unsignedValue);
		return true;
	}
	return false;
}

bool LowerComponentValue(const WasmValue& value, const wasmtime_component_valtype_t& type,
	WasmModule* module, void* context, wasmtime_component_val_t& output,
	std::string& error, std::vector<WasmHandle>* pendingTransfers)
{
	output = {};
	switch (type.kind) {
		case WASMTIME_COMPONENT_VALTYPE_BOOL: {
			const auto* boolean = std::get_if<bool>(&value.storage);
			if (boolean == nullptr) { error = "component argument is not a bool"; return false; }
			output.kind = WASMTIME_COMPONENT_BOOL;
			output.of.boolean = *boolean;
			return true;
		}
		case WASMTIME_COMPONENT_VALTYPE_S8:
		case WASMTIME_COMPONENT_VALTYPE_S16:
		case WASMTIME_COMPONENT_VALTYPE_S32:
		case WASMTIME_COMPONENT_VALTYPE_S64: {
			std::int64_t integer = 0;
			if (!ReadSignedValue(value, integer)) { error = "component argument is not a signed integer"; return false; }
			switch (type.kind) {
				case WASMTIME_COMPONENT_VALTYPE_S8:
					if (integer < std::numeric_limits<std::int8_t>::min() || integer > std::numeric_limits<std::int8_t>::max()) { error = "component s8 argument is outside its range"; return false; }
					output.kind = WASMTIME_COMPONENT_S8;
					output.of.s8 = static_cast<std::int8_t>(integer);
					break;
				case WASMTIME_COMPONENT_VALTYPE_S16:
					if (integer < std::numeric_limits<std::int16_t>::min() || integer > std::numeric_limits<std::int16_t>::max()) { error = "component s16 argument is outside its range"; return false; }
					output.kind = WASMTIME_COMPONENT_S16;
					output.of.s16 = static_cast<std::int16_t>(integer);
					break;
				case WASMTIME_COMPONENT_VALTYPE_S32:
					if (integer < std::numeric_limits<std::int32_t>::min() || integer > std::numeric_limits<std::int32_t>::max()) { error = "component s32 argument is outside its range"; return false; }
					output.kind = WASMTIME_COMPONENT_S32;
					output.of.s32 = static_cast<std::int32_t>(integer);
					break;
				default:
					output.kind = WASMTIME_COMPONENT_S64;
					output.of.s64 = integer;
					break;
			}
			return true;
		}
		case WASMTIME_COMPONENT_VALTYPE_U8:
		case WASMTIME_COMPONENT_VALTYPE_U16:
		case WASMTIME_COMPONENT_VALTYPE_U32:
		case WASMTIME_COMPONENT_VALTYPE_U64: {
			std::uint64_t integer = 0;
			if (!ReadUnsignedValue(value, integer)) { error = "component argument is not an unsigned integer"; return false; }
			switch (type.kind) {
				case WASMTIME_COMPONENT_VALTYPE_U8:
					if (integer > std::numeric_limits<std::uint8_t>::max()) { error = "component u8 argument is outside its range"; return false; }
					output.kind = WASMTIME_COMPONENT_U8;
					output.of.u8 = static_cast<std::uint8_t>(integer);
					break;
				case WASMTIME_COMPONENT_VALTYPE_U16:
					if (integer > std::numeric_limits<std::uint16_t>::max()) { error = "component u16 argument is outside its range"; return false; }
					output.kind = WASMTIME_COMPONENT_U16;
					output.of.u16 = static_cast<std::uint16_t>(integer);
					break;
				case WASMTIME_COMPONENT_VALTYPE_U32:
					if (integer > std::numeric_limits<std::uint32_t>::max()) { error = "component u32 argument is outside its range"; return false; }
					output.kind = WASMTIME_COMPONENT_U32;
					output.of.u32 = static_cast<std::uint32_t>(integer);
					break;
				default:
					output.kind = WASMTIME_COMPONENT_U64;
					output.of.u64 = integer;
					break;
			}
			return true;
		}
		case WASMTIME_COMPONENT_VALTYPE_F32:
		case WASMTIME_COMPONENT_VALTYPE_F64: {
			double number = 0.0;
			if (!ReadFloatingValue(value, number)) { error = "component argument is not a float"; return false; }
			output.kind = type.kind;
			if (type.kind == WASMTIME_COMPONENT_VALTYPE_F32) output.of.f32 = static_cast<float>(number);
			else output.of.f64 = number;
			return true;
		}
		case WASMTIME_COMPONENT_VALTYPE_CHAR: {
			std::uint64_t character = 0;
			if (!ReadUnsignedValue(value, character) || character > 0x10ffffu ||
				(character >= 0xd800u && character <= 0xdfffu)) {
				error = "component argument is not a valid char";
				return false;
			}
			output.kind = WASMTIME_COMPONENT_CHAR;
			output.of.character = static_cast<std::uint32_t>(character);
			return true;
		}
		case WASMTIME_COMPONENT_VALTYPE_STRING: {
			const auto* string = std::get_if<std::string>(&value.storage);
			if (string == nullptr) { error = "component argument is not a string"; return false; }
			output.kind = WASMTIME_COMPONENT_STRING;
			const std::string componentString = EncodeComponentString(*string);
			wasm_name_new(&output.of.string, componentString.size(), componentString.data());
			return true;
		}
		case WASMTIME_COMPONENT_VALTYPE_LIST: {
			WasmValueList byteList;
			const auto* list = std::get_if<WasmValueList>(&value.storage);
			if (list == nullptr) {
				if (const auto* bytes = std::get_if<std::vector<std::uint8_t>>(&value.storage)) {
					byteList.reserve(bytes->size());
					for (const auto byte : *bytes)
						byteList.push_back(WasmValue::U64(byte));
					list = &byteList;
				} else {
					error = "component argument is not a list";
					return false;
				}
			}
			wasmtime_component_valtype_t elementType{};
			wasmtime_component_list_type_element(type.of.list, &elementType);
			std::vector<wasmtime_component_val_t> elements(list->size());
			for (std::size_t index = 0; index < list->size(); ++index) {
				if (!LowerComponentValue((*list)[index], elementType, module, context,
					elements[index], error, pendingTransfers)) {
					wasmtime_component_valtype_delete(&elementType);
					for (auto& element : elements) wasmtime_component_val_delete(&element);
					return false;
				}
			}
			output.kind = WASMTIME_COMPONENT_LIST;
			wasmtime_component_vallist_new_uninit(&output.of.list, elements.size());
			for (std::size_t index = 0; index < elements.size(); ++index) {
				output.of.list.data[index] = elements[index];
				elements[index] = {};
			}
			wasmtime_component_valtype_delete(&elementType);
			return true;
		}
		case WASMTIME_COMPONENT_VALTYPE_RECORD: {
			const auto* record = std::get_if<WasmValueRecord>(&value.storage);
			if (record == nullptr) { error = "component argument is not a record"; return false; }
			const std::size_t fieldCount = wasmtime_component_record_type_field_count(type.of.record);
			if (record->size() != fieldCount) {
				error = "component record has an unexpected field count";
				return false;
			}
			std::vector<wasmtime_component_valrecord_entry_t> fields(fieldCount);
			for (std::size_t index = 0; index < fieldCount; ++index) {
				const char* name = nullptr;
				std::size_t nameLength = 0;
				wasmtime_component_valtype_t fieldType{};
				if (!wasmtime_component_record_type_field_nth(type.of.record, index, &name,
						&nameLength, &fieldType)) {
					error = "component record field type is unavailable";
					DeleteComponentRecordFields(fields);
					return false;
				}
				const std::string fieldName(name, nameLength);
				const auto* field = FindSemanticRecordField(*record, fieldName);
				if (field == nullptr) {
					wasmtime_component_valtype_delete(&fieldType);
					error = "component record is missing field: " + fieldName;
					DeleteComponentRecordFields(fields);
					return false;
				}
				wasm_name_new(&fields[index].name, nameLength, name);
				if (!LowerComponentValue(*field, fieldType, module, context,
					fields[index].val, error, pendingTransfers)) {
					wasmtime_component_valtype_delete(&fieldType);
					DeleteComponentRecordFields(fields);
					return false;
				}
				wasmtime_component_valtype_delete(&fieldType);
			}
			output.kind = WASMTIME_COMPONENT_RECORD;
			wasmtime_component_valrecord_new_uninit(&output.of.record, fields.size());
			for (std::size_t index = 0; index < fields.size(); ++index) {
				output.of.record.data[index] = fields[index];
				fields[index].name = {};
				fields[index].val = {};
			}
			return true;
		}
		case WASMTIME_COMPONENT_VALTYPE_TUPLE: {
			const std::size_t typeCount = wasmtime_component_tuple_type_types_count(type.of.tuple);
			if (typeCount == 0 && value.IsUnit()) {
				output.kind = WASMTIME_COMPONENT_TUPLE;
				wasmtime_component_valtuple_new(&output.of.tuple, 0, nullptr);
				return true;
			}
			const auto* tuple = std::get_if<WasmValueList>(&value.storage);
			if (tuple == nullptr) { error = "component argument is not a tuple"; return false; }
			if (tuple->size() != typeCount) { error = "component tuple has the wrong arity"; return false; }
			std::vector<wasmtime_component_val_t> values(typeCount);
			for (std::size_t index = 0; index < typeCount; ++index) {
				wasmtime_component_valtype_t elementType{};
				if (!wasmtime_component_tuple_type_types_nth(type.of.tuple, index, &elementType)) {
					error = "component tuple element type is unavailable";
					for (auto& element : values) wasmtime_component_val_delete(&element);
					return false;
				}
				if (!LowerComponentValue((*tuple)[index], elementType, module, context,
					values[index], error, pendingTransfers)) {
					wasmtime_component_valtype_delete(&elementType);
					for (auto& element : values) wasmtime_component_val_delete(&element);
					return false;
				}
				wasmtime_component_valtype_delete(&elementType);
			}
			output.kind = WASMTIME_COMPONENT_TUPLE;
			wasmtime_component_valtuple_new_uninit(&output.of.tuple, values.size());
			for (std::size_t index = 0; index < values.size(); ++index) {
				output.of.tuple.data[index] = values[index];
				values[index] = {};
			}
			return true;
		}
		case WASMTIME_COMPONENT_VALTYPE_OPTION: {
			output.kind = WASMTIME_COMPONENT_OPTION;
			if (value.IsUnit()) { output.of.option = nullptr; return true; }
			wasmtime_component_valtype_t innerType{};
			wasmtime_component_option_type_ty(type.of.option, &innerType);
			wasmtime_component_val_t inner{};
			if (!LowerComponentValue(value, innerType, module, context, inner, error,
				pendingTransfers)) {
				wasmtime_component_valtype_delete(&innerType);
				return false;
			}
			output.of.option = wasmtime_component_val_new(&inner);
			wasmtime_component_valtype_delete(&innerType);
			if (output.of.option == nullptr) {
				wasmtime_component_val_delete(&inner);
				error = "component option allocation failed";
				return false;
			}
			return true;
		}
		case WASMTIME_COMPONENT_VALTYPE_ENUM: {
			const auto* name = std::get_if<std::string>(&value.storage);
			if (name == nullptr) { error = "component argument is not an enum name"; return false; }
			const std::size_t count = wasmtime_component_enum_type_names_count(type.of.enum_);
			for (std::size_t index = 0; index < count; ++index) {
				const char* enumName = nullptr;
				std::size_t enumNameLength = 0;
				if (wasmtime_component_enum_type_names_nth(type.of.enum_, index, &enumName, &enumNameLength) &&
					*name == std::string(enumName, enumNameLength)) {
					output.kind = WASMTIME_COMPONENT_ENUM;
					wasm_name_new(&output.of.enumeration, enumNameLength, enumName);
					return true;
				}
			}
			error = "component argument has an unknown enum value";
			return false;
		}
		case WASMTIME_COMPONENT_VALTYPE_FLAGS: {
			const auto* flags = std::get_if<WasmValueList>(&value.storage);
			if (flags == nullptr) { error = "component argument is not flags"; return false; }
			const std::size_t count = wasmtime_component_flags_type_names_count(type.of.flags);
			std::vector<wasm_name_t> names;
				for (const auto& flag : *flags) {
					const auto* name = std::get_if<std::string>(&flag.storage);
					if (name == nullptr) {
						DeleteComponentNames(names);
						error = "component flag is not a string";
						return false;
					}
				bool found = false;
				for (std::size_t index = 0; index < count; ++index) {
					const char* flagName = nullptr;
					std::size_t flagLength = 0;
					if (wasmtime_component_flags_type_names_nth(type.of.flags, index, &flagName, &flagLength) &&
						*name == std::string(flagName, flagLength)) { found = true; break; }
				}
				if (!found) {
					DeleteComponentNames(names);
					error = "component argument has an unknown flag";
					return false;
				}
				wasm_name_t nameValue{};
				wasm_name_new(&nameValue, name->size(), name->data());
				names.push_back(nameValue);
			}
			output.kind = WASMTIME_COMPONENT_FLAGS;
			wasmtime_component_valflags_new_uninit(&output.of.flags, names.size());
			for (std::size_t index = 0; index < names.size(); ++index) {
				output.of.flags.data[index] = names[index];
				names[index] = {};
			}
			return true;
		}
		case WASMTIME_COMPONENT_VALTYPE_RESULT: {
			const auto* record = std::get_if<WasmValueRecord>(&value.storage);
			if (record == nullptr) { error = "component result is not a record"; return false; }
			const auto ok = record->find("ok");
			const auto payload = record->find("value");
			if (record->size() != (payload == record->end() ? 1u : 2u)) {
				error = "component result has unexpected fields";
				return false;
			}
			if (ok == record->end() || !std::holds_alternative<bool>(ok->second.storage)) {
				error = "component result is missing its ok flag";
				return false;
			}
			output.kind = WASMTIME_COMPONENT_RESULT;
			output.of.result.is_ok = std::get<bool>(ok->second.storage);
			output.of.result.val = nullptr;
			wasmtime_component_valtype_t payloadType{};
			const bool hasType = output.of.result.is_ok ?
				wasmtime_component_result_type_ok(type.of.result, &payloadType) :
				wasmtime_component_result_type_err(type.of.result, &payloadType);
			if (payload != record->end()) {
				if (!hasType) {
					error = "component result does not accept a payload";
					return false;
				}
				wasmtime_component_val_t nested{};
				if (!LowerComponentValue(payload->second, payloadType, module, context,
					nested, error, pendingTransfers)) {
					wasmtime_component_valtype_delete(&payloadType);
					return false;
				}
				output.of.result.val = wasmtime_component_val_new(&nested);
				if (output.of.result.val == nullptr) {
					wasmtime_component_val_delete(&nested);
					wasmtime_component_valtype_delete(&payloadType);
					error = "component result payload allocation failed";
					return false;
				}
			} else if (hasType) {
				wasmtime_component_valtype_delete(&payloadType);
				error = "component result is missing its payload";
				return false;
			}
			if (hasType)
				wasmtime_component_valtype_delete(&payloadType);
			return true;
		}
		case WASMTIME_COMPONENT_VALTYPE_VARIANT: {
			const auto* variant = std::get_if<WasmValueVariant>(&value.storage);
			if (variant == nullptr) {
				error = "component argument is not a variant";
				return false;
			}
			const std::size_t count = wasmtime_component_variant_type_case_count(
				type.of.variant);
			for (std::size_t index = 0; index < count; ++index) {
				const char* caseName = nullptr;
				std::size_t caseNameLength = 0;
				bool hasPayload = false;
				wasmtime_component_valtype_t payloadType{};
				if (!wasmtime_component_variant_type_case_nth(type.of.variant, index, &caseName,
						&caseNameLength, &hasPayload, &payloadType)) {
					error = "component variant case type is unavailable";
					return false;
				}
				if (variant->discriminant != std::string(caseName, caseNameLength)) {
					if (hasPayload)
						wasmtime_component_valtype_delete(&payloadType);
					continue;
				}
				if (hasPayload != variant->HasValue()) {
					if (hasPayload)
						wasmtime_component_valtype_delete(&payloadType);
					error = hasPayload ? "component variant is missing its payload" :
						"component variant case does not accept a payload";
					return false;
				}
				output.kind = WASMTIME_COMPONENT_VARIANT;
				wasm_name_new(&output.of.variant.discriminant, caseNameLength, caseName);
				output.of.variant.val = nullptr;
				if (hasPayload) {
					wasmtime_component_val_t payload{};
					if (!LowerComponentValue(*variant->value, payloadType, module, context,
							payload, error, pendingTransfers)) {
						wasmtime_component_valtype_delete(&payloadType);
						wasmtime_component_val_delete(&output);
						return false;
					}
					output.of.variant.val = wasmtime_component_val_new(&payload);
					if (output.of.variant.val == nullptr) {
						wasmtime_component_val_delete(&payload);
						wasmtime_component_valtype_delete(&payloadType);
						wasmtime_component_val_delete(&output);
						error = "component variant payload allocation failed";
						return false;
					}
					wasmtime_component_valtype_delete(&payloadType);
				}
				return true;
			}
			error = "component variant has an unknown discriminant";
			return false;
		}
		case WASMTIME_COMPONENT_VALTYPE_OWN:
		case WASMTIME_COMPONENT_VALTYPE_BORROW: {
			const auto* resourceValue = std::get_if<WasmValueResource>(&value.storage);
			if (resourceValue == nullptr || module == nullptr || context == nullptr) {
				error = "component argument is not an instance-owned resource";
				return false;
			}
			void* resource = nullptr;
			const bool transferOwnership = type.kind == WASMTIME_COMPONENT_VALTYPE_OWN;
			if (!module->ExportComponentResource(context, *resourceValue, transferOwnership,
				resource, error, pendingTransfers))
				return false;
			output.kind = WASMTIME_COMPONENT_RESOURCE;
			output.of.resource = static_cast<wasmtime_component_resource_any_t*>(resource);
			return true;
		}
		default:
			error = "component argument uses an unsupported value type";
			return false;
	}
}

bool RegisterComponentInstanceImports(wasmtime_component_linker_instance_t* linkerInstance,
	const wasmtime_component_instance_type_t* instanceType, std::string_view importPath,
	wasm_engine_t* engine, WasmModule* module,
	std::vector<std::unique_ptr<WasmHostFunctionData>>& hostFunctions,
	std::string& error);

bool RegisterComponentItem(wasmtime_component_linker_instance_t* linkerInstance,
	std::string_view name, const wasmtime_component_item_t& item, std::string_view importPath,
	wasm_engine_t* engine, WasmModule* module,
	std::vector<std::unique_ptr<WasmHostFunctionData>>& hostFunctions,
	std::string& error)
{
	const std::string childPath = std::string(importPath) + "/" + std::string(name);
	if (item.kind == WASMTIME_COMPONENT_ITEM_COMPONENT_INSTANCE) {
		wasmtime_component_linker_instance_t* child = nullptr;
		if (wasmtime_error_t* addError = wasmtime_component_linker_instance_add_instance(
				linkerInstance, name.data(), name.size(), &child);
			addError != nullptr) {
			error = WasmtimeErrorMessage(addError);
			return false;
		}
		const bool success = RegisterComponentInstanceImports(child, item.of.component_instance,
			childPath, engine, module, hostFunctions, error);
		wasmtime_component_linker_instance_delete(child);
		return success;
	}
	if (item.kind == WASMTIME_COMPONENT_ITEM_RESOURCE) {
		// Resource values are kept in the module's instance-owned table while
		// crossing the C++ semantic boundary. Wasmtime still owns the guest-side
		// representation and invokes this destructor when an imported `own`
		// resource is released; no native pointer is exposed to the guest.
		if (wasmtime_error_t* addError = wasmtime_component_linker_instance_add_resource(
				linkerInstance, name.data(), name.size(), item.of.resource,
				[](void*, wasmtime_context_t*, std::uint32_t) -> wasmtime_error_t* {
					return nullptr;
				}, module, nullptr);
			addError != nullptr) {
			error = "component resource linker registration failed: " +
				WasmtimeErrorMessage(addError);
			return false;
		}
		return true;
	}
	if (item.kind != WASMTIME_COMPONENT_ITEM_COMPONENT_FUNC)
		return true;
	auto function = std::make_unique<WasmHostFunctionData>();
	function->module = module;
	function->moduleName = ComponentImportModule(importPath);
	function->functionName = ComponentImportFunction(function->moduleName, name);
	if (!WasmEnvironmentMatrix::HasModule(module->Descriptor().environment,
		function->moduleName)) {
		error = "component host interface is unavailable in environment " +
			std::string(WasmEnvironmentMatrix::Name(module->Descriptor().environment)) +
			": " + function->moduleName;
		return false;
	}
	auto* functionData = function.get();
	if (wasmtime_error_t* addError = wasmtime_component_linker_instance_add_func(
			linkerInstance, name.data(), name.size(),
				[](void* data, wasmtime_context_t* context, const wasmtime_component_func_type_t* type,
				wasmtime_component_val_t* arguments, std::size_t argumentCount,
				wasmtime_component_val_t* results, std::size_t resultCount) -> wasmtime_error_t* {
				auto* function = static_cast<WasmHostFunctionData*>(data);
				if (function == nullptr || function->module == nullptr)
					return ComponentHostError("component host function has no owner");
				std::vector<WasmValue> values;
				values.reserve(argumentCount);
				std::size_t componentValueBytes = 0;
				std::size_t componentValueNodes = 0;
				std::string error;
				for (std::size_t index = 0; index < argumentCount; ++index) {
					const char* parameterName = nullptr;
					std::size_t parameterNameLength = 0;
					wasmtime_component_valtype_t parameterType{};
					if (type == nullptr || !wasmtime_component_func_type_param_nth(type, index,
						&parameterName, &parameterNameLength, &parameterType))
						return ComponentHostError("component host parameter type is unavailable");
					(void)parameterName;
					(void)parameterNameLength;
					if (!CheckComponentValueBudget(arguments[index],
						function->module->Runtime().Config().resultBytesLimit,
						function->module->Runtime().Config().maxValueNodes,
						function->module->Runtime().Config().maxComponentNesting, 0,
						componentValueBytes, componentValueNodes, error)) {
						wasmtime_component_valtype_delete(&parameterType);
						return ComponentHostError(error);
					}
					WasmValue value;
					const bool success = LiftComponentValueTyped(arguments[index], parameterType,
						function->module, context, value, error);
					wasmtime_component_valtype_delete(&parameterType);
					if (!success)
						return ComponentHostError(error);
					values.push_back(std::move(value));
				}
				const std::size_t importCheckpoint = function->module->ImportGuardCheckpoint();
				struct DeferredImportGuard {
					WasmModule* module;
					std::size_t checkpoint;

					~DeferredImportGuard()
					{
						module->ReleaseDeferredImportGuards(checkpoint);
					}
				} deferredImportGuard{function->module, importCheckpoint};
				WasmValue result;
				std::int32_t nativeErrorCode = 0;
				bool nativeError = false;
				if (!function->module->InvokeCallout(function->moduleName,
					function->functionName, values, result, error, true))
				{
					if (!ParseNativeApiError(error, nativeErrorCode))
						return ComponentHostError(error);
					nativeError = true;
					error.clear();
				}
				if (type == nullptr)
					return ComponentHostError("component host result type is unavailable");
				wasmtime_component_valtype_t resultType{};
				const bool hasResultType = wasmtime_component_func_type_result(type, &resultType);
				if (resultCount == 0) {
					if (hasResultType)
						wasmtime_component_valtype_delete(&resultType);
					if (hasResultType || !result.IsUnit())
						return ComponentHostError(
							"component host returned a value for a unit result");
					return nullptr;
				}
				if (resultCount != 1) {
					if (hasResultType)
						wasmtime_component_valtype_delete(&resultType);
					return ComponentHostError(
						"component host functions with multiple results are unsupported");
				}
				if (!hasResultType)
					return ComponentHostError("component host result type is unavailable");
				// Generated Spring callouts use `result<T, spring-error>` in WIT,
				// while the native adapter returns the successful T payload.  The
				// host boundary owns this envelope so every adapter gets identical
				// success/error semantics.
				WasmValue componentResult = std::move(result);
				if (resultType.kind == WASMTIME_COMPONENT_VALTYPE_RESULT) {
					if (nativeError) {
						wasmtime_component_valtype_t errorType{};
						const bool hasError = wasmtime_component_result_type_err(
							resultType.of.result, &errorType);
						if (hasError)
							wasmtime_component_valtype_delete(&errorType);
						if (!hasError) {
							wasmtime_component_valtype_delete(&resultType);
							return ComponentHostError(
								"native API error was returned for a result without an error type");
						}
						componentResult = WasmValue::Record({
							{"ok", WasmValue::Bool(false)},
							{"value", WasmValue::Record({
								{"code", WasmValue::I64(nativeErrorCode)},
							})},
						});
					} else {
						wasmtime_component_valtype_t payloadType{};
						const bool hasPayload = wasmtime_component_result_type_ok(
							resultType.of.result, &payloadType);
						if (hasPayload)
							wasmtime_component_valtype_delete(&payloadType);
						if (hasPayload) {
							componentResult = WasmValue::Record({
								{"ok", WasmValue::Bool(true)},
								{"value", std::move(componentResult)},
							});
						} else if (!componentResult.IsUnit()) {
							wasmtime_component_valtype_delete(&resultType);
							return ComponentHostError(
								"component host returned a payload for a unit result");
						} else {
							componentResult = WasmValue::Record({
								{"ok", WasmValue::Bool(true)},
							});
						}
					}
				} else if (nativeError) {
					wasmtime_component_valtype_delete(&resultType);
					return ComponentHostError(
						"native API error was returned for a non-result component function");
				}
				std::vector<WasmHandle> pendingTransfers;
				const bool success = LowerComponentValue(componentResult, resultType,
					function->module, context, results[0], error, &pendingTransfers);
				const bool committed = success &&
					function->module->CommitComponentResourceTransfers(pendingTransfers, error);
				wasmtime_component_valtype_delete(&resultType);
				if (!committed) {
					wasmtime_component_val_delete(&results[0]);
					return ComponentHostError(error);
				}
				return nullptr;
			},
			functionData, nullptr);
		addError != nullptr) {
			error = WasmtimeErrorMessage(addError);
			return false;
		}
	hostFunctions.push_back(std::move(function));
	return true;
}

bool RegisterComponentInstanceImports(wasmtime_component_linker_instance_t* linkerInstance,
	const wasmtime_component_instance_type_t* instanceType, std::string_view importPath,
	wasm_engine_t* engine, WasmModule* module,
	std::vector<std::unique_ptr<WasmHostFunctionData>>& hostFunctions,
	std::string& error)
{
	const std::size_t exportCount = wasmtime_component_instance_type_export_count(instanceType, engine);
	for (std::size_t index = 0; index < exportCount; ++index) {
		const char* name = nullptr;
		std::size_t nameLength = 0;
		wasmtime_component_item_t item{};
		if (!wasmtime_component_instance_type_export_nth(instanceType, engine, index, &name,
				&nameLength, &item)) {
			error = "component import instance export type is unavailable";
			return false;
		}
		const bool success = RegisterComponentItem(linkerInstance,
			std::string_view(name, nameLength), item, importPath, engine, module, hostFunctions, error);
		wasmtime_component_item_delete(&item);
		if (!success)
			return false;
	}
	return true;
}

bool RegisterComponentImports(wasmtime_component_linker_t* linker,
	wasmtime_component_t* component, wasm_engine_t* engine, WasmModule* module,
	std::vector<std::unique_ptr<WasmHostFunctionData>>& hostFunctions,
	std::string& error)
{
	auto* type = wasmtime_component_type(component);
	if (type == nullptr) {
		error = "component type reflection failed";
		return false;
	}
	auto* root = wasmtime_component_linker_root(linker);
	if (root == nullptr) {
		wasmtime_component_type_delete(type);
		error = "component linker root is unavailable";
		return false;
	}
	const std::size_t importCount = wasmtime_component_type_import_count(type, engine);
	for (std::size_t index = 0; index < importCount; ++index) {
		const char* name = nullptr;
		std::size_t nameLength = 0;
		wasmtime_component_item_t item{};
		if (!wasmtime_component_type_import_nth(type, engine, index, &name, &nameLength, &item)) {
			error = "component import type is unavailable";
			wasmtime_component_linker_instance_delete(root);
			wasmtime_component_type_delete(type);
			return false;
		}
		const std::string importName(name, nameLength);
		const bool success = RegisterComponentItem(root, importName, item, importName, engine,
			module, hostFunctions, error);
		wasmtime_component_item_delete(&item);
		if (!success) {
			wasmtime_component_linker_instance_delete(root);
			wasmtime_component_type_delete(type);
			return false;
		}
	}
	wasmtime_component_linker_instance_delete(root);
	wasmtime_component_type_delete(type);
	return true;
}
#endif

}

struct WasmModule::BackendState {
#if defined(RECOIL_WASMTIME_AVAILABLE)
	struct ComponentResourceEntry {
		wasmtime_component_resource_any_t* resource = nullptr;
		bool owned = false;
	};

	wasmtime_store_t* store = nullptr;
	wasmtime_linker_t* coreLinker = nullptr;
	wasmtime_component_linker_t* componentLinker = nullptr;
	wasmtime_module_t* coreModule = nullptr;
	wasmtime_component_t* component = nullptr;
	wasmtime_instance_t coreInstance{};
	wasmtime_component_instance_t componentInstance{};
	bool isComponent = false;
	std::unordered_set<std::string> componentFunctionExports;
	std::vector<std::unique_ptr<WasmHostFunctionData>> hostFunctions;
	std::map<WasmHandle, ComponentResourceEntry> componentResources;

	~BackendState()
	{
		if (coreLinker != nullptr)
			wasmtime_linker_delete(coreLinker);
		if (componentLinker != nullptr)
			wasmtime_component_linker_delete(componentLinker);
		if (coreModule != nullptr)
			wasmtime_module_delete(coreModule);
		if (component != nullptr)
			wasmtime_component_delete(component);
		if (store != nullptr)
			wasmtime_store_delete(store);
	}
#endif
};

WasmModule::WasmModule(WasmInstanceID instanceID, WasmModuleDescriptor descriptor,
	const WasmRuntime& runtime, WasmHostAdapter* hostAdapter)
	: instanceID(instanceID)
	, descriptor(std::move(descriptor))
	, runtime(runtime)
	, hostAdapter(hostAdapter)
	, budget(runtime.Config().instructionFuel, runtime.Config().hostWorkLimit,
		runtime.Config().resultBytesLimit)
	, callbackLifetime(std::make_shared<WasmCallbackLifetime>())
{
	callbackLifetime->module = this;
	callbackLifetime->active = true;
}

WasmModule::~WasmModule()
{
	if (callbackLifetime != nullptr) {
		callbackLifetime->active = false;
		callbackLifetime->module = nullptr;
	}
	Shutdown();
}

bool WasmModule::Initialize(std::string& error)
{
	if (state != WasmModuleState::Created && state != WasmModuleState::Stopped) {
		error = "Wasm module has already been initialized";
		return false;
	}
	budget.Reset(runtime.Config().instructionFuel, runtime.Config().hostWorkLimit,
		runtime.Config().resultBytesLimit);
	deferredImportLeaves = 0;
	resources.Clear();
	if (!resources.SetLimit(runtime.Config().maxResources)) {
		error = "Wasm resource table limit is smaller than live resources";
		Fault(error);
		return false;
	}
	callbacks.Clear();
	cleanupCallbacks.clear();
	faultReason.clear();
	const WasmValidationResult validation = runtime.ValidateModule(
		descriptor.bytes, descriptor.environment, WasmEnvironmentMatrix::Name(descriptor.environment),
		descriptor.interfaceVersion);
	if (!validation.valid) {
		error = validation.error;
		Fault(error);
		return false;
	}
	identity = validation.identity;
	if (callbackLifetime != nullptr) {
		callbackLifetime->module = this;
		callbackLifetime->active = true;
	}
	state = WasmModuleState::Validated;
	if (!runtime.IsAvailable()) {
		error = "the pinned Wasmtime Component Model backend is not available in this build";
		Fault(error);
		return false;
	}

#if defined(RECOIL_WASMTIME_AVAILABLE)
	const auto hostFunction = [](void* data, wasmtime_caller_t*,
		const wasmtime_val_t* arguments, std::size_t argumentCount,
		wasmtime_val_t* results, std::size_t resultCount) -> wasm_trap_t* {
		auto* function = static_cast<WasmHostFunctionData*>(data);
		if (function == nullptr || function->module == nullptr)
			return wasmtime_trap_new("Wasm host function has no owner", 31);
		if (argumentCount != 1 || resultCount != 1 || arguments == nullptr ||
			results == nullptr || arguments[0].kind != WASMTIME_I32) {
			return wasmtime_trap_new("Wasm host scalar signature mismatch", 36);
		}

		std::vector<WasmValue> values;
		values.push_back(WasmValue::I64(arguments[0].of.i32));
		WasmValue result;
		std::string error;
		if (!function->module->InvokeCallout(function->moduleName,
			function->functionName, values, result, error))
			return wasmtime_trap_new(error.c_str(), error.size());

		if (const auto* value = std::get_if<std::int64_t>(&result.storage)) {
			if (*value < std::numeric_limits<std::int32_t>::min() ||
				*value > std::numeric_limits<std::int32_t>::max())
				return wasmtime_trap_new("Wasm host result is outside i32", 33);
			results[0].kind = WASMTIME_I32;
			results[0].of.i32 = static_cast<std::int32_t>(*value);
			return nullptr;
		}
		if (const auto* value = std::get_if<std::uint64_t>(&result.storage)) {
			if (*value > std::numeric_limits<std::uint32_t>::max())
				return wasmtime_trap_new("Wasm host result is outside i32", 33);
			results[0].kind = WASMTIME_I32;
			results[0].of.i32 = static_cast<std::int32_t>(*value);
			return nullptr;
		}
		return wasmtime_trap_new("Wasm host result is not an integer", 37);
	};

	backendState = std::make_unique<BackendState>();
	backendState->isComponent = IsComponent(descriptor.bytes);
	backendState->store = wasmtime_store_new(
		static_cast<wasm_engine_t*>(runtime.BackendEngine()), nullptr, nullptr);
	if (backendState->store == nullptr) {
		error = "Wasmtime could not create a module store";
		Fault(error);
		return false;
	}
	wasmtime_store_limiter(backendState->store,
		static_cast<std::int64_t>(runtime.Config().maxMemoryPages) * 65536,
		static_cast<std::int64_t>(runtime.Config().maxTableElements),
		std::max<std::int64_t>(2, runtime.Config().maxComponentNesting + 1),
		std::max<std::int64_t>(2, runtime.Config().maxComponentNesting + 1),
		std::max<std::int64_t>(2, runtime.Config().maxComponentNesting + 1));
	if (runtime.Config().instructionFuel != 0) {
		if (wasmtime_error_t* fuelError = wasmtime_context_set_fuel(
			wasmtime_store_context(backendState->store), runtime.Config().instructionFuel);
			fuelError != nullptr) {
			error = "Wasmtime could not configure instruction fuel: " +
				WasmtimeErrorMessage(fuelError);
			Fault(error);
			return false;
		}
	}

	if (backendState->isComponent) {
		wasmtime_error_t* compileError = wasmtime_component_new(
			static_cast<wasm_engine_t*>(runtime.BackendEngine()), descriptor.bytes.data(),
			descriptor.bytes.size(), &backendState->component);
		if (compileError != nullptr) {
			error = "Wasmtime component compilation failed: " +
				WasmtimeErrorMessage(compileError);
			Fault(error);
			return false;
		}
		if (auto* componentType = wasmtime_component_type(backendState->component);
			componentType != nullptr) {
			CollectComponentExports(componentType,
				static_cast<wasm_engine_t*>(runtime.BackendEngine()), {},
				backendState->componentFunctionExports);
			wasmtime_component_type_delete(componentType);
		}
		backendState->componentLinker = wasmtime_component_linker_new(
			static_cast<wasm_engine_t*>(runtime.BackendEngine()));
		if (backendState->componentLinker == nullptr) {
			error = "Wasmtime could not create a component linker";
			Fault(error);
			return false;
		}
		if (wasmtime_error_t* linkError =
				wasmtime_component_linker_define_unknown_imports_as_traps(
					backendState->componentLinker, backendState->component);
			linkError != nullptr) {
			error = "Wasmtime component import policy setup failed: " +
				WasmtimeErrorMessage(linkError);
			Fault(error);
			return false;
		}
		if (hostAdapter != nullptr) {
			wasmtime_component_linker_allow_shadowing(backendState->componentLinker, true);
			std::string registrationError;
			if (!RegisterComponentImports(backendState->componentLinker, backendState->component,
				static_cast<wasm_engine_t*>(runtime.BackendEngine()), this,
				backendState->hostFunctions, registrationError)) {
				error = "Wasmtime component host import registration failed: " + registrationError;
				Fault(error);
				return false;
			}
		}
		const std::size_t importCheckpoint = ImportGuardCheckpoint();
		wasmtime_error_t* instantiateError = wasmtime_component_linker_instantiate(
				backendState->componentLinker,
				wasmtime_store_context(backendState->store), backendState->component,
				&backendState->componentInstance);
		ReleaseDeferredImportGuards(importCheckpoint);
		if (instantiateError != nullptr) {
			error = "Wasmtime component instantiation failed: " +
				WasmtimeErrorMessage(instantiateError);
			Fault(error);
			return false;
		}
	} else {
		wasmtime_error_t* compileError = wasmtime_module_new(
			static_cast<wasm_engine_t*>(runtime.BackendEngine()), descriptor.bytes.data(),
			descriptor.bytes.size(), &backendState->coreModule);
		if (compileError != nullptr) {
			error = "Wasmtime module compilation failed: " +
				WasmtimeErrorMessage(compileError);
			Fault(error);
			return false;
		}
		backendState->coreLinker = wasmtime_linker_new(
			static_cast<wasm_engine_t*>(runtime.BackendEngine()));
		if (backendState->coreLinker == nullptr) {
			error = "Wasmtime could not create a module linker";
			Fault(error);
			return false;
		}
		if (hostAdapter != nullptr) {
			auto hostFunctionData = std::make_unique<WasmHostFunctionData>();
			hostFunctionData->module = this;
			hostFunctionData->moduleName = "spring";
			hostFunctionData->functionName = "add-i32";
			auto* hostFunctionDataPtr = hostFunctionData.get();
			wasm_functype_t* type = wasm_functype_new_1_1(
				wasm_valtype_new_i32(), wasm_valtype_new_i32());
			wasmtime_error_t* defineError = wasmtime_linker_define_func(
				backendState->coreLinker, "spring", 6, "add-i32", 7, type,
				hostFunction, hostFunctionDataPtr, nullptr);
			wasm_functype_delete(type);
			if (defineError != nullptr) {
				error = "Wasmtime host import registration failed: " +
					WasmtimeErrorMessage(defineError);
				Fault(error);
				return false;
			}
			backendState->hostFunctions.push_back(std::move(hostFunctionData));
		}
		if (wasmtime_error_t* linkError =
				wasmtime_linker_define_unknown_imports_as_traps(
					backendState->coreLinker, backendState->coreModule);
			linkError != nullptr) {
			error = "Wasmtime module import policy setup failed: " +
				WasmtimeErrorMessage(linkError);
			Fault(error);
			return false;
		}
		wasm_trap_t* trap = nullptr;
		if (wasmtime_error_t* instantiateError = wasmtime_linker_instantiate(
				backendState->coreLinker, wasmtime_store_context(backendState->store),
				backendState->coreModule, &backendState->coreInstance, &trap);
			instantiateError != nullptr) {
			error = "Wasmtime module instantiation failed: " +
				WasmtimeErrorMessage(instantiateError);
			if (trap != nullptr)
				error += ": " + WasmTrapMessage(trap);
			Fault(error);
			return false;
		} else if (trap != nullptr) {
			error = "Wasmtime module start trapped: " + WasmTrapMessage(trap);
			Fault(error);
			return false;
		}
	}
#endif

	state = WasmModuleState::Running;
	return true;
}

void WasmModule::Shutdown()
{
	if (state == WasmModuleState::Stopped)
		return;
	if (callbackLifetime != nullptr)
		callbackLifetime->active = false;
	// Cleanup runs in reverse registration order so nested host resources are
	// released in the same order in which they were acquired.  The lifetime
	// token is inactive already, so destroy callbacks cannot re-enter the guest
	// while its backend is being torn down.
	for (auto iter = cleanupCallbacks.rbegin(); iter != cleanupCallbacks.rend(); ++iter) {
		if (*iter)
			(*iter)();
	}
	cleanupCallbacks.clear();
	callbacks.Clear();
	ClearComponentResources();
	resources.Clear();
	backendState.reset();
	state = WasmModuleState::Stopped;
}

bool WasmModule::Callout(std::string_view interfaceName, std::string_view functionName,
	std::string& error)
{
	WasmValue result;
	return InvokeCallout(interfaceName, functionName, {}, result, error);
}

bool WasmModule::InvokeCallout(std::string_view interfaceName, std::string_view functionName,
	const std::vector<WasmValue>& arguments, WasmValue& result, std::string& error,
	bool keepImportEntered)
{
	if (state != WasmModuleState::Running) {
		error = "Wasm module is not running";
		return false;
	}
	if (interfaceName.empty() || functionName.empty()) {
		error = "Wasm callout interface/function name is empty";
		return false;
	}
	// A host import may not recursively enter another Spring import.  The only
	// exception is a callback explicitly marked re-entrant; a clean call stack
	// must not be treated as permission to nest merely because no callback is
	// currently active.
	const bool allowImportReentry = budget.CallbackDepth() != 0 &&
		budget.CallbackReentryAllowed();
	if (!budget.EnterImport(allowImportReentry)) {
		error = "Wasm import re-entry denied";
		return false;
	}
	if (!runtime.IsAvailable()) {
		error = "the Wasmtime Component Model backend is unavailable";
		budget.LeaveImport();
		return false;
	}
	if (!budget.ChargeHost(1 + static_cast<std::uint64_t>(arguments.size()))) {
		error = "Wasm callout host-work budget exhausted";
		budget.LeaveImport();
		return false;
	}
	if (!budget.CheckResultSize(WasmValuesBytes(arguments))) {
		error = "Wasm callout arguments exceed the configured byte limit";
		budget.LeaveImport();
		return false;
	}
	if (hostAdapter == nullptr) {
		error = "Wasm host adapter is unavailable";
		budget.LeaveImport();
		return false;
	}
	bool success = hostAdapter->Callout(*this, interfaceName, functionName, arguments, result, error);
	if (success && !budget.CheckResultSize(WasmValueBytes(result))) {
		error = "Wasm callout result exceeds the configured byte limit";
		success = false;
	}
	if (!success || !keepImportEntered)
		budget.LeaveImport();
	else
		++deferredImportLeaves;
	return success;
}

void WasmModule::ReleaseDeferredImportGuards(std::size_t checkpoint)
{
	while (deferredImportLeaves > checkpoint) {
		budget.LeaveImport();
		--deferredImportLeaves;
	}
}

WasmCallbackID WasmModule::RegisterGuestCallback(std::uint32_t guestCallbackID,
	WasmCallbackPolicy policy, std::string& error)
{
	if (guestCallbackID == 0) {
		error = "Wasm callback ID 0 is reserved for no callback";
		return 0;
	}
	if (state != WasmModuleState::Running) {
		error = "Wasm module is not running";
		return 0;
	}
	const std::string legacyPath = "callback-" + std::to_string(guestCallbackID);
	std::string callbackPath = legacyPath;
#if defined(RECOIL_WASMTIME_AVAILABLE)
	if (backendState == nullptr || !backendState->isComponent) {
		error = "Wasm callbacks require a Component Model module";
		return 0;
	}
	const std::string canonicalPath = "callbacks/" + std::to_string(guestCallbackID);
	if (backendState->componentFunctionExports.contains(canonicalPath)) {
		callbackPath = canonicalPath;
	} else if (!backendState->componentFunctionExports.contains(legacyPath)) {
		error = "Wasm callback export not found: " + legacyPath;
		return 0;
	}
#endif
	const WasmCallbackID callbackID = callbacks.Register(policy, [this, guestCallbackID, callbackPath](
		const std::vector<std::uint64_t>& arguments) {
		std::vector<WasmValue> values;
		values.reserve(arguments.size());
		for (const std::uint64_t argument : arguments)
			values.push_back(WasmValue::U64(argument));
		WasmValue result;
		std::string callbackError;
		if (Callin(callbackPath, values, result, callbackError))
			return true;
		if (state == WasmModuleState::Running)
			Fault("Wasm callback " + std::to_string(guestCallbackID) +
				" failed: " + callbackError);
		return false;
	});
	if (callbackID == 0)
		error = "Wasm callback registry is exhausted";
	return callbackID;
}

bool WasmModule::InvokeGuestCallback(WasmCallbackID callbackID,
	const std::vector<std::uint64_t>& arguments, std::string& error)
{
	if (state != WasmModuleState::Running) {
		error = "Wasm module is not running";
		return false;
	}
	const bool reentrant = callbacks.IsReentrant(callbackID);
	if (!budget.EnterCallback(reentrant)) {
		error = reentrant ? "Wasm callback nesting limit exceeded" :
		"Wasm callback re-entry denied";
		return false;
	}
	bool reentryAllowed = false;
	const bool success = callbacks.Invoke(callbackID, arguments, true, reentryAllowed);
	budget.LeaveCallback();
	if (!success) {
		error = faultReason.empty() ? "Wasm callback invocation failed" : faultReason;
		return false;
	}
	return true;
}

void WasmModule::DropGuestCallback(WasmCallbackID callbackID)
{
	if (callbackID != 0)
		callbacks.Drop(callbackID);
}

bool WasmModule::ImportComponentResource(void* context, void* resource, bool owned,
	WasmValue& output, std::string& error)
{
#if defined(RECOIL_WASMTIME_AVAILABLE)
	if (state != WasmModuleState::Running || backendState == nullptr ||
		backendState->store == nullptr || context == nullptr || resource == nullptr) {
		error = "component resource import has no live Wasmtime store";
		return false;
	}
	if (backendState->componentResources.size() >= runtime.Config().maxResources) {
		error = "Wasm component resource table limit exceeded";
		return false;
	}
	auto* raw = static_cast<wasmtime_component_resource_any_t*>(resource);
	auto* clone = wasmtime_component_resource_any_clone(raw);
	if (clone == nullptr) {
		error = "component resource clone failed";
		return false;
	}
	const WasmHandle handle = resources.Insert(instanceID, "component");
	if (handle == 0) {
		wasmtime_component_resource_any_delete(clone);
		error = "Wasm component resource table is full";
		return false;
	}
	backendState->componentResources.emplace(handle,
		BackendState::ComponentResourceEntry{clone, owned});
	output = WasmValue::Resource(handle, "component", owned);
	return true;
#else
	(void)context;
	(void)resource;
	(void)owned;
	(void)output;
	error = "the Wasmtime Component Model backend is unavailable";
	return false;
#endif
}

bool WasmModule::ExportComponentResource(void* context, const WasmValueResource& value,
	bool transferOwnership, void*& resource, std::string& error,
	std::vector<WasmHandle>* pendingTransfers)
{
#if defined(RECOIL_WASMTIME_AVAILABLE)
	resource = nullptr;
	if (state != WasmModuleState::Running || backendState == nullptr ||
		backendState->store == nullptr || context == nullptr) {
		error = "component resource export has no live Wasmtime store";
		return false;
	}
	if (value.handle == 0 || !resources.Validate(value.handle, instanceID, "component")) {
		error = "component resource handle is stale, foreign, or has the wrong family";
		return false;
	}
	const auto iter = backendState->componentResources.find(value.handle);
	if (iter == backendState->componentResources.end() || iter->second.resource == nullptr) {
		error = "component resource handle has no live Wasmtime resource";
		return false;
	}
	if (transferOwnership) {
		if (!value.owned || !iter->second.owned) {
			error = "a borrowed component resource cannot be transferred as own";
			return false;
		}
		if (pendingTransfers != nullptr) {
			if (std::find(pendingTransfers->begin(), pendingTransfers->end(), value.handle) !=
				pendingTransfers->end()) {
				error = "a component resource cannot be transferred more than once";
				return false;
			}
			auto* clone = wasmtime_component_resource_any_clone(iter->second.resource);
			if (clone == nullptr) {
				error = "component resource clone failed";
				return false;
			}
			resource = clone;
			pendingTransfers->push_back(value.handle);
			return true;
		}
		resource = iter->second.resource;
		iter->second.resource = nullptr;
		backendState->componentResources.erase(iter);
		if (!resources.Drop(value.handle, instanceID, "component")) {
			wasmtime_component_resource_any_delete(
				static_cast<wasmtime_component_resource_any_t*>(resource));
			resource = nullptr;
			error = "component resource handle could not be released after transfer";
			return false;
		}
		return true;
	}
	resource = wasmtime_component_resource_any_clone(iter->second.resource);
	if (resource == nullptr) {
		error = "component resource clone failed";
		return false;
	}
	return true;
#else
	(void)context;
	(void)value;
	(void)transferOwnership;
	(void)resource;
	error = "the Wasmtime Component Model backend is unavailable";
	return false;
#endif
}

bool WasmModule::CommitComponentResourceTransfers(const std::vector<WasmHandle>& transfers,
	std::string& error)
{
#if defined(RECOIL_WASMTIME_AVAILABLE)
	if (transfers.empty())
		return true;
	if (state != WasmModuleState::Running || backendState == nullptr ||
		backendState->store == nullptr) {
		error = "component resource transfer commit has no live Wasmtime store";
		return false;
	}

	// Validate the whole batch before changing either table. This keeps a
	// failed commit from consuming only a prefix of a record/list argument.
	for (std::size_t index = 0; index < transfers.size(); ++index) {
		const WasmHandle handle = transfers[index];
		if (std::find(transfers.begin(), transfers.begin() + index, handle) !=
			transfers.begin() + index) {
			error = "component resource transfer batch contains a duplicate handle";
			return false;
		}
		if (!resources.Validate(handle, instanceID, "component")) {
			error = "component resource transfer handle became stale";
			return false;
		}
		const auto iter = backendState->componentResources.find(handle);
		if (iter == backendState->componentResources.end() || iter->second.resource == nullptr ||
			!iter->second.owned) {
			error = "component resource transfer entry is unavailable or borrowed";
			return false;
		}
	}

	for (const WasmHandle handle : transfers) {
		if (!resources.Drop(handle, instanceID, "component")) {
			error = "component resource handle could not be released after transfer";
			return false;
		}
	}
	for (const WasmHandle handle : transfers) {
		const auto iter = backendState->componentResources.find(handle);
		wasmtime_component_resource_any_delete(iter->second.resource);
		backendState->componentResources.erase(iter);
	}
	return true;
#else
	(void)transfers;
	error = "the Wasmtime Component Model backend is unavailable";
	return false;
#endif
}

bool WasmModule::DropComponentResource(void* context, const WasmValueResource& value,
	std::string& error)
{
#if defined(RECOIL_WASMTIME_AVAILABLE)
	if (state != WasmModuleState::Running || backendState == nullptr ||
		backendState->store == nullptr || context == nullptr) {
		error = "component resource drop has no live Wasmtime store";
		return false;
	}
	if (value.handle == 0 || !resources.Validate(value.handle, instanceID, "component")) {
		error = "component resource handle is stale, foreign, or has the wrong family";
		return false;
	}
	const auto iter = backendState->componentResources.find(value.handle);
	if (iter == backendState->componentResources.end() || iter->second.resource == nullptr) {
		error = "component resource handle has no live Wasmtime resource";
		return false;
	}
	if (wasmtime_error_t* dropError = wasmtime_component_resource_any_drop(
			static_cast<wasmtime_context_t*>(context), iter->second.resource);
		dropError != nullptr) {
		error = "component resource drop failed: " + WasmtimeErrorMessage(dropError);
		return false;
	}
	wasmtime_component_resource_any_delete(iter->second.resource);
	backendState->componentResources.erase(iter);
	if (!resources.Drop(value.handle, instanceID, "component")) {
		error = "component resource table release failed";
		return false;
	}
	return true;
#else
	(void)context;
	(void)value;
	error = "the Wasmtime Component Model backend is unavailable";
	return false;
#endif
}

void WasmModule::ClearComponentResources()
{
#if defined(RECOIL_WASMTIME_AVAILABLE)
	if (backendState == nullptr)
		return;
	wasmtime_context_t* context = backendState->store == nullptr ? nullptr :
		wasmtime_store_context(backendState->store);
	for (auto& [handle, entry] : backendState->componentResources) {
		if (entry.resource == nullptr)
			continue;
		if (context != nullptr)
			wasmtime_component_resource_any_drop(context, entry.resource);
		wasmtime_component_resource_any_delete(entry.resource);
		(void)handle;
	}
	backendState->componentResources.clear();
#endif
}

bool WasmModule::RegisterCleanup(std::function<void()> cleanup)
{
	if (!cleanup || state != WasmModuleState::Running)
		return false;
	cleanupCallbacks.push_back(std::move(cleanup));
	return true;
}

bool WasmModule::Callin(std::string_view name, const std::vector<std::uint64_t>& arguments,
	std::string& error)

{
	std::vector<std::uint64_t> results;
	return Callin(name, arguments, results, error);
}

bool WasmModule::Callin(std::string_view name, const std::vector<std::uint64_t>& arguments,
	std::vector<std::uint64_t>& results, std::string& error)
{
	if (state != WasmModuleState::Running) {
		error = "Wasm module is not running";
		return false;
	}
	results.clear();
	if (!budget.ChargeHost(1 + static_cast<std::uint64_t>(arguments.size()))) {
		error = "Wasm callin host-work budget exhausted";
		return false;
	}
#if defined(RECOIL_WASMTIME_AVAILABLE)
	if (backendState == nullptr) {
		error = "Wasm module backend state is missing";
		return false;
	}
	if (backendState->isComponent) {
		wasmtime_component_export_index_t* exportIndex =
			wasmtime_component_instance_get_export_index(
				&backendState->componentInstance, wasmtime_store_context(backendState->store),
				nullptr, name.data(), name.size());
		wasmtime_component_func_t function{};
		if (exportIndex == nullptr || !wasmtime_component_instance_get_func(
				&backendState->componentInstance, wasmtime_store_context(backendState->store),
				exportIndex, &function)) {
			if (exportIndex != nullptr)
				wasmtime_component_export_index_delete(exportIndex);
			error = "Wasm component callin export not found: " + std::string(name);
			return false;
		}
		wasmtime_component_func_type_t* functionType = wasmtime_component_func_type(
			&function, wasmtime_store_context(backendState->store));
		if (functionType == nullptr) {
			wasmtime_component_export_index_delete(exportIndex);
			error = "Wasm component callin function type is unavailable";
			return false;
		}
		const std::size_t parameterCount = wasmtime_component_func_type_param_count(functionType);
		if (parameterCount != arguments.size()) {
			wasmtime_component_func_type_delete(functionType);
			wasmtime_component_export_index_delete(exportIndex);
			error = "Wasm component callin argument arity mismatch";
			return false;
		}
		std::vector<wasmtime_component_val_t> values(arguments.size());
		std::vector<WasmHandle> pendingTransfers;
		auto cleanupValues = [&values]() { DeleteComponentValues(values); };
		for (std::size_t index = 0; index < arguments.size(); ++index) {
			const char* parameterName = nullptr;
			std::size_t parameterNameLength = 0;
			wasmtime_component_valtype_t parameterType{};
			if (!wasmtime_component_func_type_param_nth(functionType, index, &parameterName,
					&parameterNameLength, &parameterType)) {
				cleanupValues();
				wasmtime_component_func_type_delete(functionType);
				wasmtime_component_export_index_delete(exportIndex);
				error = "Wasm component callin parameter type is unavailable";
				return false;
			}
			(void)parameterName;
			(void)parameterNameLength;
			const auto parameterKind = parameterType.kind;
			bool parameterSupported = true;
			if (parameterKind == WASMTIME_COMPONENT_VALTYPE_BOOL) {
				if (arguments[index] > 1) {
					error = "Wasm component callin bool parameter is outside its range";
					parameterSupported = false;
				} else {
					values[index].kind = WASMTIME_COMPONENT_BOOL;
					values[index].of.boolean = arguments[index] != 0;
				}
			} else if (parameterKind == WASMTIME_COMPONENT_VALTYPE_S8 ||
				parameterKind == WASMTIME_COMPONENT_VALTYPE_S16 ||
				parameterKind == WASMTIME_COMPONENT_VALTYPE_S32 ||
				parameterKind == WASMTIME_COMPONENT_VALTYPE_S64 ||
				parameterKind == WASMTIME_COMPONENT_VALTYPE_U8 ||
				parameterKind == WASMTIME_COMPONENT_VALTYPE_U16 ||
				parameterKind == WASMTIME_COMPONENT_VALTYPE_U32 ||
				parameterKind == WASMTIME_COMPONENT_VALTYPE_U64) {
				std::string lowerError;
				parameterSupported = LowerComponentValue(WasmValue::U64(arguments[index]),
					parameterType, this, wasmtime_store_context(backendState->store),
					values[index], lowerError, &pendingTransfers);
				if (!parameterSupported)
					error = "Wasm component callin parameter is invalid: " + lowerError;
			} else {
				error = "Wasm component callin has an unsupported parameter type";
				parameterSupported = false;
			}
			wasmtime_component_valtype_delete(&parameterType);
			if (!parameterSupported) {
				cleanupValues();
				wasmtime_component_func_type_delete(functionType);
				wasmtime_component_export_index_delete(exportIndex);
				return false;
			}
		}
		wasmtime_component_valtype_t resultType{};
		const bool hasResult = wasmtime_component_func_type_result(functionType, &resultType);
		const bool resultSupported = !hasResult ||
			resultType.kind == WASMTIME_COMPONENT_VALTYPE_BOOL ||
			(resultType.kind >= WASMTIME_COMPONENT_VALTYPE_S8 &&
				resultType.kind <= WASMTIME_COMPONENT_VALTYPE_U64);
		if (hasResult && !resultSupported) {
			wasmtime_component_valtype_delete(&resultType);
			wasmtime_component_func_type_delete(functionType);
			wasmtime_component_export_index_delete(exportIndex);
				error = "Wasm component callin has an unsupported result type";
				return false;
		}
		if (!CommitComponentResourceTransfers(pendingTransfers, error)) {
			cleanupValues();
			if (hasResult)
				wasmtime_component_valtype_delete(&resultType);
			wasmtime_component_func_type_delete(functionType);
			wasmtime_component_export_index_delete(exportIndex);
			return false;
		}
		const std::size_t resultCount = hasResult ? 1 : 0;
		std::vector<wasmtime_component_val_t> componentResults(resultCount);
		wasmtime_component_func_type_delete(functionType);
		const std::size_t importCheckpoint = ImportGuardCheckpoint();
		wasmtime_error_t* callError = wasmtime_component_func_call(
			&function, wasmtime_store_context(backendState->store), values.data(),
			values.size(), componentResults.data(), componentResults.size());
		ReleaseDeferredImportGuards(importCheckpoint);
		cleanupValues();
		if (hasResult)
			wasmtime_component_valtype_delete(&resultType);
		wasmtime_component_export_index_delete(exportIndex);
		if (callError != nullptr) {
			error = "Wasm component callin failed: " + WasmtimeErrorMessage(callError);
			DeleteComponentValues(componentResults);
			Fault(error);
			return false;
		}
		for (const auto& value : componentResults) {
			switch (value.kind) {
				case WASMTIME_COMPONENT_BOOL:
					results.push_back(value.of.boolean ? 1 : 0);
					break;
				case WASMTIME_COMPONENT_S8:
					results.push_back(static_cast<std::uint64_t>(value.of.s8));
					break;
				case WASMTIME_COMPONENT_U8:
					results.push_back(value.of.u8);
					break;
				case WASMTIME_COMPONENT_S16:
					results.push_back(static_cast<std::uint64_t>(value.of.s16));
					break;
				case WASMTIME_COMPONENT_U16:
					results.push_back(value.of.u16);
					break;
				case WASMTIME_COMPONENT_S32:
					results.push_back(static_cast<std::uint64_t>(value.of.s32));
					break;
				case WASMTIME_COMPONENT_U32:
					results.push_back(value.of.u32);
					break;
				case WASMTIME_COMPONENT_S64:
					results.push_back(static_cast<std::uint64_t>(value.of.s64));
					break;
				case WASMTIME_COMPONENT_U64:
					results.push_back(value.of.u64);
					break;
				default:
					error = "Wasm component callin returned a non-integer value";
					DeleteComponentValues(componentResults);
					return false;
			}
		}
		DeleteComponentValues(componentResults);
		return true;
	}
	wasmtime_extern_t functionExtern{};
	if (!wasmtime_instance_export_get(wasmtime_store_context(backendState->store),
			&backendState->coreInstance, name.data(), name.size(), &functionExtern) ||
		functionExtern.kind != WASMTIME_EXTERN_FUNC) {
		wasmtime_extern_delete(&functionExtern);
		error = "Wasm module callin export not found: " + std::string(name);
		return false;
	}
	wasm_functype_t* functionType = wasmtime_func_type(
		wasmtime_store_context(backendState->store), &functionExtern.of.func);
	if (functionType == nullptr) {
		wasmtime_extern_delete(&functionExtern);
		error = "Wasm callin function type is unavailable";
		return false;
	}
	const wasm_valtype_vec_t* parameters = wasm_functype_params(functionType);
	const wasm_valtype_vec_t* functionResults = wasm_functype_results(functionType);
	if (parameters->size != arguments.size()) {
		wasm_functype_delete(functionType);
		wasmtime_extern_delete(&functionExtern);
		error = "Wasm callin argument/result arity mismatch";
		return false;
	}
	if (functionResults->size > runtime.Config().resultBytesLimit / sizeof(std::uint64_t)) {
		wasm_functype_delete(functionType);
		wasmtime_extern_delete(&functionExtern);
		error = "Wasm callin result count exceeds the configured byte limit";
		return false;
	}
	std::vector<wasmtime_val_t> values(arguments.size());
	for (std::size_t index = 0; index < arguments.size(); ++index) {
		switch (wasm_valtype_kind(parameters->data[index])) {
			case WASM_I32:
				values[index].kind = WASMTIME_I32;
				values[index].of.i32 = static_cast<std::int32_t>(arguments[index]);
				break;
			case WASM_I64:
				values[index].kind = WASMTIME_I64;
				values[index].of.i64 = static_cast<std::int64_t>(arguments[index]);
				break;
			default:
				wasm_functype_delete(functionType);
				wasmtime_extern_delete(&functionExtern);
				error = "Wasm callin has an unsupported numeric parameter type";
				return false;
		}
	}
	std::vector<wasmtime_val_t> callResults(functionResults->size);
	wasm_trap_t* trap = nullptr;
	wasmtime_error_t* callError = wasmtime_func_call(
		wasmtime_store_context(backendState->store), &functionExtern.of.func,
		values.data(), values.size(), callResults.data(), callResults.size(), &trap);
	wasm_functype_delete(functionType);
	wasmtime_extern_delete(&functionExtern);
	if (callError != nullptr) {
		error = "Wasm callin failed: " + WasmtimeErrorMessage(callError);
		if (trap != nullptr)
			error += ": " + WasmTrapMessage(trap);
		Fault(error);
		return false;
	}
	if (trap != nullptr) {
		error = "Wasm callin trapped: " + WasmTrapMessage(trap);
		Fault(error);
		return false;
	}
	for (const auto& value : callResults) {
		switch (value.kind) {
			case WASMTIME_I32:
				results.push_back(static_cast<std::uint64_t>(
					static_cast<std::int64_t>(value.of.i32)));
				break;
			case WASMTIME_I64:
				results.push_back(static_cast<std::uint64_t>(value.of.i64));
				break;
			default:
				error = "Wasm callin returned an unsupported numeric type";
				results.clear();
				return false;
		}
	}
#else
	error = "the Wasmtime Component Model backend is unavailable";
	return false;
#endif
	return true;
}

bool WasmModule::Callin(std::string_view name, const std::vector<WasmValue>& arguments,
	WasmValue& result, std::string& error)
{
	if (state != WasmModuleState::Running) {
		error = "Wasm module is not running";
		return false;
	}
	if (!budget.ChargeHost(1 + static_cast<std::uint64_t>(arguments.size()))) {
		error = "Wasm callin host-work budget exhausted";
		return false;
	}
	if (!budget.CheckResultSize(WasmValuesBytes(arguments))) {
		error = "Wasm callin arguments exceed the configured byte limit";
		return false;
	}
#if defined(RECOIL_WASMTIME_AVAILABLE)
	if (backendState == nullptr || !backendState->isComponent) {
		error = "semantic Wasm callins require a Component Model module";
		return false;
	}
	// Callins are optional component exports. Check the component type before
	// asking the instance API for a nested function index; the latter is a
	// relatively expensive operation and, for a missing function in an
	// exported interface, can scan the complete component export tree.
		if (name.find('/') != std::string_view::npos &&
			backendState->componentFunctionExports.find(std::string(name)) ==
				backendState->componentFunctionExports.end()) {
			result = WasmValue::Unit();
			return true;
		}
		ComponentExportIndexPath exportPath;
	if (!exportPath.Resolve(backendState->componentInstance,
		wasmtime_store_context(backendState->store), name)) {
		// Component worlds are intentionally modular: a guest may export only
		// the callins it implements.  Missing exports are therefore a no-op;
		// malformed exports and invocation failures below remain errors.
		result = WasmValue::Unit();
		return true;
	}
	wasmtime_component_func_t function{};
	if (!wasmtime_component_instance_get_func(
			&backendState->componentInstance, wasmtime_store_context(backendState->store),
			exportPath.Get(), &function)) {
		error = "Wasm component callin export is not a function: " + std::string(name);
		return false;
	}
	wasmtime_component_func_type_t* functionType = wasmtime_component_func_type(
		&function, wasmtime_store_context(backendState->store));
	if (functionType == nullptr) {
		error = "Wasm component callin function type is unavailable";
		return false;
	}
	const std::size_t parameterCount = wasmtime_component_func_type_param_count(functionType);
	if (parameterCount != arguments.size()) {
		wasmtime_component_func_type_delete(functionType);
		error = "Wasm component callin argument arity mismatch";
		return false;
	}

	std::vector<wasmtime_component_val_t> values(arguments.size());
	std::vector<WasmHandle> pendingTransfers;
	auto cleanupValues = [&values]() { DeleteComponentValues(values); };
	for (std::size_t index = 0; index < arguments.size(); ++index) {
		const char* parameterName = nullptr;
		std::size_t parameterNameLength = 0;
		wasmtime_component_valtype_t parameterType{};
		if (!wasmtime_component_func_type_param_nth(functionType, index, &parameterName,
				&parameterNameLength, &parameterType)) {
			cleanupValues();
			wasmtime_component_func_type_delete(functionType);
			error = "Wasm component callin parameter type is unavailable";
			return false;
		}
		(void)parameterName;
		(void)parameterNameLength;
		std::string lowerError;
		const bool lowered = LowerComponentValue(arguments[index], parameterType,
				this, wasmtime_store_context(backendState->store), values[index], lowerError,
				&pendingTransfers);
		wasmtime_component_valtype_delete(&parameterType);
		if (!lowered) {
			cleanupValues();
			wasmtime_component_func_type_delete(functionType);
			error = "Wasm component callin parameter is invalid: " + lowerError;
			return false;
		}
	}
	if (!CommitComponentResourceTransfers(pendingTransfers, error)) {
		cleanupValues();
		wasmtime_component_func_type_delete(functionType);
		return false;
	}

	wasmtime_component_valtype_t resultType{};
	const bool hasResult = wasmtime_component_func_type_result(functionType, &resultType);
	std::vector<wasmtime_component_val_t> callResults(hasResult ? 1 : 0);
	wasmtime_component_func_type_delete(functionType);
	const std::size_t importCheckpoint = ImportGuardCheckpoint();
	wasmtime_error_t* callError = wasmtime_component_func_call(
		&function, wasmtime_store_context(backendState->store), values.data(), values.size(),
		callResults.data(), callResults.size());
	ReleaseDeferredImportGuards(importCheckpoint);
	cleanupValues();
	if (callError != nullptr) {
		error = "Wasm component callin failed: " + WasmtimeErrorMessage(callError);
		if (hasResult)
			wasmtime_component_valtype_delete(&resultType);
		DeleteComponentValues(callResults);
		Fault(error);
		return false;
	}
	if (!hasResult) {
		result = WasmValue::Unit();
		return true;
	}

	std::size_t componentValueBytes = 0;
	std::size_t componentValueNodes = 0;
	if (!CheckComponentValueBudget(callResults.front(), runtime.Config().resultBytesLimit,
		runtime.Config().maxValueNodes, runtime.Config().maxComponentNesting, 0,
		componentValueBytes, componentValueNodes, error)) {
		wasmtime_component_valtype_delete(&resultType);
		DeleteComponentValues(callResults);
		return false;
	}
	const bool lifted = LiftComponentValueTyped(callResults.front(), resultType, this,
		wasmtime_store_context(backendState->store), result, error);
	wasmtime_component_valtype_delete(&resultType);
	DeleteComponentValues(callResults);
	if (!lifted)
		return false;
	if (!budget.CheckResultSize(WasmValueBytes(result))) {
		error = "Wasm callin result exceeds the configured byte limit";
		return false;
	}
	return true;
#else
	(void)name;
	(void)arguments;
	(void)result;
	error = "the Wasmtime Component Model backend is unavailable";
	return false;
#endif
}

void WasmModule::Fault(std::string reason)
{
	faultReason = std::move(reason);
	if (callbackLifetime != nullptr)
		callbackLifetime->active = false;
	callbacks.Clear();
	ClearComponentResources();
	resources.Clear();
	state = WasmModuleState::Faulted;
}
