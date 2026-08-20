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

bool RangesOverlap(std::uint32_t firstPtr, std::size_t firstBytes,
	std::uint32_t secondPtr, std::size_t secondBytes)
{
	if (firstBytes == 0 || secondBytes == 0)
		return false;
	const std::uint64_t firstBegin = firstPtr;
	const std::uint64_t secondBegin = secondPtr;
	const std::uint64_t firstEnd = firstBegin + firstBytes;
	const std::uint64_t secondEnd = secondBegin + secondBytes;
	return firstBegin < secondEnd && secondBegin < firstEnd;
}

wasm_trap_t* GetLogSections(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->config == nullptr ||
		state->native->config->GetLogSections == nullptr)
		return Trap("GetLogSections Core binding is unavailable");
	if (slots == nullptr || slotCount != 5)
		return Trap("GetLogSections Core ABI signature mismatch");

	ImportGuard guard(state, 8);
	if (!guard.Ok())
		return Trap(guard.Error());

	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);

	const std::uint32_t descriptorPtr = static_cast<std::uint32_t>(slots[0].i32);
	const std::uint32_t descriptorCapacity = static_cast<std::uint32_t>(slots[1].i32);
	const std::uint32_t bytesPtr = static_cast<std::uint32_t>(slots[2].i32);
	const std::uint32_t bytesCapacity = static_cast<std::uint32_t>(slots[3].i32);
	const std::uint32_t metaPtr = static_cast<std::uint32_t>(slots[4].i32);

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
	if (RangesOverlap(descriptorPtr, descriptorBytes, bytesPtr, bytesCapacity) ||
		RangesOverlap(descriptorPtr, descriptorBytes, metaPtr, STRING_LIST_META_BYTES) ||
		RangesOverlap(bytesPtr, bytesCapacity, metaPtr, STRING_LIST_META_BYTES)) {
		slots[0].i32 = static_cast<std::int32_t>(Status::InvalidArgument);
		return nullptr;
	}

	GetLogSectionsQuery query{};
	GetLogSectionsResult result{};
	state->native->config->GetLogSections(&query, &result);
	if (result.error != nullptr) {
		slots[0].i32 = result.error->code;
		return nullptr;
	}
	if (result.count != 0 && result.sections == nullptr) {
		slots[0].i32 = static_cast<std::int32_t>(Status::Internal);
		return nullptr;
	}

	std::uint64_t requiredBytes64 = 0;
	bool fits = result.count <= descriptorCapacity;
	std::uint32_t writtenBytes = 0;
	for (std::uint32_t index = 0; index < result.count; ++index) {
		const char* section = result.sections[index];
		if (section == nullptr) {
			slots[0].i32 = static_cast<std::int32_t>(Status::Internal);
			return nullptr;
		}
		const std::size_t length = std::strlen(section);
		if (length > std::numeric_limits<std::uint32_t>::max() ||
			requiredBytes64 + length > std::numeric_limits<std::uint32_t>::max()) {
			requiredBytes64 = std::numeric_limits<std::uint32_t>::max();
			fits = false;
			continue;
		}

		const std::uint32_t offset = static_cast<std::uint32_t>(requiredBytes64);
		requiredBytes64 += length;
		const std::uint32_t requiredBytes = static_cast<std::uint32_t>(requiredBytes64);
		const bool itemFits = fits && requiredBytes <= bytesCapacity;
		if (!itemFits) {
			fits = false;
			continue;
		}

		std::uint8_t* descriptor = descriptors.data() +
			static_cast<std::size_t>(index) * STRING_DESCRIPTOR_BYTES;
		WriteU32LE(descriptor + 0, offset);
		WriteU32LE(descriptor + 4, static_cast<std::uint32_t>(length));
		if (length != 0)
			std::memcpy(bytes.data() + offset, section, length);
		writtenBytes = requiredBytes;
	}

	const std::uint32_t requiredBytes = static_cast<std::uint32_t>(requiredBytes64);
	WriteU32LE(meta.data() + 0, result.count);
	WriteU32LE(meta.data() + 4, requiredBytes);

	// Output contents are unspecified when BufferOverflow is returned. This lets
	// the successful reused-buffer path remain one strlen+memcpy pass with no
	// temporary length vector and no second traversal of all strings.
	if (!fits || writtenBytes != requiredBytes) {
		slots[0].i32 = static_cast<std::int32_t>(Status::BufferOverflow);
		return nullptr;
	}

	slots[0].i32 = 0;
	return nullptr;
}

bool Define(wasmtime_linker_t* linker, HostState* state, std::string& error)
{
	const wasm_valkind_t params[] = {WASM_I32, WASM_I32, WASM_I32, WASM_I32, WASM_I32};
	const wasm_valkind_t results[] = {WASM_I32};
	wasm_functype_t* type = MakeFuncType(params, 5, results, 1);
	constexpr char module[] = "spring:config";
	constexpr char name[] = "get-log-sections-flat";
	wasmtime_error_t* defineError = wasmtime_linker_define_func_unchecked(
		linker, module, sizeof(module) - 1, name, sizeof(name) - 1,
		type, GetLogSections, state, nullptr);
	wasm_functype_delete(type);
	if (defineError == nullptr)
		return true;
	error = ErrorMessage(defineError);
	return false;
}

} // namespace

bool RegisterConfigImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error)
{
	if (linker == nullptr || state == nullptr || state->native == nullptr ||
		state->native->config == nullptr) {
		error = "cannot register Config Core imports without linker/host/API";
		return false;
	}
	return Define(linker, state, error);
}

#endif

} // namespace recoil::wasm::core
