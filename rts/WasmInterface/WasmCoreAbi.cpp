/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreAbi.h"

#include <algorithm>
#include <limits>

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)

std::string ErrorMessage(wasmtime_error_t* error)
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

std::string TrapMessage(wasm_trap_t* trap)
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

bool Memory::BindFromCaller(wasmtime_caller_t* caller, std::string& error)
{
	if (bound)
		return true;
	if (caller == nullptr) {
		error = "core Wasm import has no caller while resolving memory";
		return false;
	}
	wasmtime_extern_t item{};
	if (!wasmtime_caller_export_get(caller, "memory", 6, &item) ||
		item.kind != WASMTIME_EXTERN_MEMORY) {
		error = "core Wasm module must export linear memory as `memory`";
		return false;
	}
	Bind(wasmtime_caller_context(caller), item.of.memory);
	wasmtime_extern_delete(&item);
	return true;
}

bool Memory::BindFromInstance(wasmtime_context_t* context, const wasmtime_instance_t& instance,
	std::string& error)
{
	wasmtime_extern_t item{};
	if (!wasmtime_instance_export_get(context, &instance, "memory", 6, &item) ||
		item.kind != WASMTIME_EXTERN_MEMORY) {
		error = "core Wasm module must export linear memory as `memory`";
		return false;
	}
	Bind(context, item.of.memory);
	wasmtime_extern_delete(&item);
	return true;
}

std::uint8_t* Memory::CurrentBase() const
{
	if (!bound || storeContext == nullptr)
		return nullptr;
	return stable ? cachedBase : wasmtime_memory_data(storeContext, &linearMemory);
}

std::size_t Memory::CurrentSize() const
{
	if (!bound || storeContext == nullptr)
		return 0;
	return stable ? cachedSize : wasmtime_memory_data_size(storeContext, &linearMemory);
}

std::size_t Memory::Size() const
{
	return CurrentSize();
}

bool Memory::Contains(std::uint32_t offset, std::size_t bytes) const
{
	if (!bound || storeContext == nullptr)
		return false;
	const std::size_t size = CurrentSize();
	const std::size_t begin = static_cast<std::size_t>(offset);
	return begin <= size && bytes <= size - begin;
}

bool Memory::Range(std::uint32_t offset, std::size_t bytes, std::uint8_t*& base) const
{
	if (!Contains(offset, bytes))
		return false;
	std::uint8_t* memory = CurrentBase();
	if (memory == nullptr && CurrentSize() != 0)
		return false;
	base = memory + static_cast<std::size_t>(offset);
	return true;
}

bool Memory::Read(std::uint32_t offset, void* destination, std::size_t bytes) const
{
	if (bytes != 0 && destination == nullptr)
		return false;
	std::uint8_t* source = nullptr;
	if (!Range(offset, bytes, source))
		return false;
	if (bytes != 0)
		std::memcpy(destination, source, bytes);
	return true;
}

bool Memory::Write(std::uint32_t offset, const void* source, std::size_t bytes) const
{
	if (bytes != 0 && source == nullptr)
		return false;
	std::uint8_t* destination = nullptr;
	if (!Range(offset, bytes, destination))
		return false;
	if (bytes != 0)
		std::memcpy(destination, source, bytes);
	return true;
}

bool Memory::ReadI32SliceLE(std::uint32_t offset, std::span<std::int32_t> values) const
{
	if (values.size() > std::numeric_limits<std::size_t>::max() / sizeof(std::int32_t))
		return false;
	const std::size_t bytes = values.size() * sizeof(std::int32_t);
	std::uint8_t* source = nullptr;
	if (!Range(offset, bytes, source))
		return false;
	if constexpr (std::endian::native == std::endian::little) {
		if (bytes != 0)
			std::memcpy(values.data(), source, bytes);
		return true;
	}
	for (std::size_t index = 0; index < values.size(); ++index) {
		const std::uint8_t* item = source + index * 4;
		const std::uint32_t raw = static_cast<std::uint32_t>(item[0]) |
			(static_cast<std::uint32_t>(item[1]) << 8) |
			(static_cast<std::uint32_t>(item[2]) << 16) |
			(static_cast<std::uint32_t>(item[3]) << 24);
		values[index] = static_cast<std::int32_t>(raw);
	}
	return true;
}

bool Memory::WriteI32SliceLE(std::uint32_t offset, std::span<const std::int32_t> values) const
{
	if (values.size() > std::numeric_limits<std::size_t>::max() / sizeof(std::int32_t))
		return false;
	const std::size_t bytes = values.size() * sizeof(std::int32_t);
	std::uint8_t* destination = nullptr;
	if (!Range(offset, bytes, destination))
		return false;
	if constexpr (std::endian::native == std::endian::little) {
		if (bytes != 0)
			std::memcpy(destination, values.data(), bytes);
		return true;
	}
	for (std::size_t index = 0; index < values.size(); ++index) {
		const std::uint32_t raw = static_cast<std::uint32_t>(values[index]);
		std::uint8_t* item = destination + index * 4;
		item[0] = static_cast<std::uint8_t>(raw);
		item[1] = static_cast<std::uint8_t>(raw >> 8);
		item[2] = static_cast<std::uint8_t>(raw >> 16);
		item[3] = static_cast<std::uint8_t>(raw >> 24);
	}
	return true;
}

wasm_functype_t* MakeFuncType(const wasm_valkind_t* params, std::size_t paramCount,
	const wasm_valkind_t* results, std::size_t resultCount)
{
	wasm_valtype_vec_t paramTypes;
	wasm_valtype_vec_t resultTypes;
	wasm_valtype_vec_new_uninitialized(&paramTypes, paramCount);
	wasm_valtype_vec_new_uninitialized(&resultTypes, resultCount);
	for (std::size_t index = 0; index < paramCount; ++index)
		paramTypes.data[index] = wasm_valtype_new(params[index]);
	for (std::size_t index = 0; index < resultCount; ++index)
		resultTypes.data[index] = wasm_valtype_new(results[index]);
	return wasm_functype_new(&paramTypes, &resultTypes);
}

bool FunctionHasSignature(wasmtime_context_t* context, const wasmtime_func_t& function,
	const wasm_valkind_t* params, std::size_t paramCount,
	const wasm_valkind_t* results, std::size_t resultCount)
{
	wasm_functype_t* type = wasmtime_func_type(context, &function);
	if (type == nullptr)
		return false;
	const wasm_valtype_vec_t* actualParams = wasm_functype_params(type);
	const wasm_valtype_vec_t* actualResults = wasm_functype_results(type);
	bool matches = actualParams->size == paramCount && actualResults->size == resultCount;
	for (std::size_t index = 0; matches && index < paramCount; ++index)
		matches = wasm_valtype_kind(actualParams->data[index]) == params[index];
	for (std::size_t index = 0; matches && index < resultCount; ++index)
		matches = wasm_valtype_kind(actualResults->data[index]) == results[index];
	wasm_functype_delete(type);
	return matches;
}

bool RawExport::Resolve(wasmtime_context_t* context, const wasmtime_instance_t& instance,
	const char* name, std::size_t nameLength,
	std::span<const wasm_valkind_t> params,
	std::span<const wasm_valkind_t> results,
	bool optional, std::string& error)
{
	present = false;
	slotCount = 0;
	wasmtime_extern_t item{};
	if (!wasmtime_instance_export_get(context, &instance, name, nameLength, &item)) {
		if (optional)
			return true;
		error = "required core Wasm export is missing: " + std::string(name, nameLength);
		return false;
	}
	if (item.kind != WASMTIME_EXTERN_FUNC) {
		wasmtime_extern_delete(&item);
		error = "core Wasm export is not a function: " + std::string(name, nameLength);
		return false;
	}
	if (!FunctionHasSignature(context, item.of.func, params.data(), params.size(),
		results.data(), results.size())) {
		wasmtime_extern_delete(&item);
		error = "core Wasm export has the wrong signature: " + std::string(name, nameLength);
		return false;
	}
	function = item.of.func;
	slotCount = std::max(params.size(), results.size());
	present = true;
	wasmtime_extern_delete(&item);
	return true;
}

bool RawExport::Call(wasmtime_context_t* context, wasmtime_val_raw_t* slots,
	std::size_t providedSlotCount, std::string& error) const
{
	if (!present)
		return true;
	if (providedSlotCount != slotCount || (slotCount != 0 && slots == nullptr)) {
		error = "core Wasm export raw slot count does not match its bound signature";
		return false;
	}
	wasm_trap_t* trap = nullptr;
	if (wasmtime_error_t* callError = wasmtime_func_call_unchecked(
			context, &function, slots, providedSlotCount, &trap);
		callError != nullptr) {
		error = "core Wasm export call failed: " + ErrorMessage(callError);
		if (trap != nullptr)
			error += ": " + TrapMessage(trap);
		return false;
	}
	if (trap != nullptr) {
		error = "core Wasm export trapped: " + TrapMessage(trap);
		return false;
	}
	return true;
}

bool I32ToVoidExport::Resolve(wasmtime_context_t* context, const wasmtime_instance_t& instance,
	const char* name, std::size_t nameLength, bool optional, std::string& error)
{
	const wasm_valkind_t params[] = {WASM_I32};
	return raw.Resolve(context, instance, name, nameLength,
		std::span<const wasm_valkind_t>(params, 1), {}, optional, error);
}

bool I32ToVoidExport::Call(wasmtime_context_t* context, std::int32_t value,
	std::string& error) const
{
	if (!raw.Present())
		return true;
	wasmtime_val_raw_t slot{};
	slot.i32 = value;
	return raw.Call(context, &slot, 1, error);
}

#endif

} // namespace recoil::wasm::core
