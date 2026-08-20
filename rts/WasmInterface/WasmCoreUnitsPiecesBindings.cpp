/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreBindings.h"

#include <cstdint>
#include <cstring>
#include <limits>
#include <span>

#include "WasmCoreGeneratedSupport.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)
namespace {

using generated::ImportGuard;
using generated::Trap;

constexpr std::size_t STRING_DESCRIPTOR_BYTES = 8;
constexpr std::size_t STRING_LIST_META_BYTES = 8;

void WriteU32LE(std::uint8_t* output, std::uint32_t value)
{
	output[0] = static_cast<std::uint8_t>(value);
	output[1] = static_cast<std::uint8_t>(value >> 8);
	output[2] = static_cast<std::uint8_t>(value >> 16);
	output[3] = static_cast<std::uint8_t>(value >> 24);
}

wasm_trap_t* GetUnitScriptNames(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->unitsPieces == nullptr ||
		state->native->unitsPieces->GetUnitScriptNames == nullptr)
		return Trap("GetUnitScriptNames Core binding is unavailable");
	if (slots == nullptr || slotCount != 6)
		return Trap("GetUnitScriptNames Core ABI signature mismatch");

	ImportGuard guard(state, 8);
	if (!guard.Ok())
		return Trap(guard.Error());

	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);

	const std::int32_t unitID = slots[0].i32;
	const std::uint32_t descriptorPtr = static_cast<std::uint32_t>(slots[1].i32);
	const std::uint32_t descriptorCapacity = static_cast<std::uint32_t>(slots[2].i32);
	const std::uint32_t bytesPtr = static_cast<std::uint32_t>(slots[3].i32);
	const std::uint32_t bytesCapacity = static_cast<std::uint32_t>(slots[4].i32);
	const std::uint32_t metaPtr = static_cast<std::uint32_t>(slots[5].i32);

	if (descriptorCapacity > std::numeric_limits<std::size_t>::max() /
			STRING_DESCRIPTOR_BYTES) {
		slots[0].i32 = static_cast<std::int32_t>(Status::InvalidArgument);
		return nullptr;
	}
	const std::size_t descriptorBytes =
		static_cast<std::size_t>(descriptorCapacity) * STRING_DESCRIPTOR_BYTES;

	std::span<std::uint8_t> descriptors;
	std::span<std::uint8_t> bytes;
	std::span<std::uint8_t> meta;
	if (!state->memory.MutableView(descriptorPtr, descriptorBytes, descriptors) ||
		!state->memory.MutableView(bytesPtr, bytesCapacity, bytes) ||
		!state->memory.MutableView(metaPtr, STRING_LIST_META_BYTES, meta)) {
		slots[0].i32 = static_cast<std::int32_t>(Status::OutOfBounds);
		return nullptr;
	}

	GetUnitScriptNamesQuery query{.unitID = unitID};
	GetUnitScriptNamesResult result{};
	state->native->unitsPieces->GetUnitScriptNames(&query, &result);
	if (result.error != nullptr) {
		slots[0].i32 = result.error->code;
		return nullptr;
	}
	if (result.count != 0 && result.names == nullptr) {
		slots[0].i32 = static_cast<std::int32_t>(Status::Internal);
		return nullptr;
	}

	std::uint64_t requiredBytes64 = 0;
	bool fits = result.count <= descriptorCapacity;
	std::uint32_t writtenBytes = 0;
	for (std::uint32_t index = 0; index < result.count; ++index) {
		const char* name = result.names[index];
		if (name == nullptr) {
			slots[0].i32 = static_cast<std::int32_t>(Status::Internal);
			return nullptr;
		}
		const std::size_t length = std::strlen(name);
		if (length > std::numeric_limits<std::uint32_t>::max() ||
			requiredBytes64 + length > std::numeric_limits<std::uint32_t>::max()) {
			requiredBytes64 = std::numeric_limits<std::uint32_t>::max();
			fits = false;
			continue;
		}

		const std::uint32_t offset = static_cast<std::uint32_t>(requiredBytes64);
		requiredBytes64 += length;
		const std::uint32_t requiredBytes = static_cast<std::uint32_t>(requiredBytes64);
		if (!fits || requiredBytes > bytesCapacity) {
			fits = false;
			continue;
		}

		std::uint8_t* descriptor = descriptors.data() +
			static_cast<std::size_t>(index) * STRING_DESCRIPTOR_BYTES;
		WriteU32LE(descriptor + 0, offset);
		WriteU32LE(descriptor + 4, static_cast<std::uint32_t>(length));
		if (length != 0)
			std::memcpy(bytes.data() + offset, name, length);
		writtenBytes = requiredBytes;
	}

	const std::uint32_t requiredBytes = static_cast<std::uint32_t>(requiredBytes64);
	WriteU32LE(meta.data() + 0, result.count);
	WriteU32LE(meta.data() + 4, requiredBytes);
	if (!fits || writtenBytes != requiredBytes) {
		slots[0].i32 = static_cast<std::int32_t>(Status::BufferOverflow);
		return nullptr;
	}

	slots[0].i32 = 0;
	return nullptr;
}

} // namespace

bool RegisterUnitsPiecesImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error)
{
	if (linker == nullptr || state == nullptr || state->native == nullptr ||
		state->native->unitsPieces == nullptr) {
		error = "cannot register UnitsPieces Core imports without linker/host/API";
		return false;
	}

	const wasm_valkind_t params[] = {
		WASM_I32, WASM_I32, WASM_I32, WASM_I32, WASM_I32, WASM_I32,
	};
	const wasm_valkind_t results[] = {WASM_I32};
	wasm_functype_t* type = MakeFuncType(params, 6, results, 1);
	constexpr char module[] = "spring:units-pieces";
	constexpr char name[] = "get-unit-script-names-flat";
	wasmtime_error_t* defineError = wasmtime_linker_define_func_unchecked(
		linker, module, sizeof(module) - 1, name, sizeof(name) - 1,
		type, GetUnitScriptNames, state, nullptr);
	wasm_functype_delete(type);
	if (defineError == nullptr)
		return true;
	error = ErrorMessage(defineError);
	return false;
}

#endif

} // namespace recoil::wasm::core
