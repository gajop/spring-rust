/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreBindings.h"

#include <cstdint>
#include <cstring>
#include <limits>
#include <span>
#include <string_view>

#include "NativeInterface/WasmUiVisibility.h"
#include "Rendering/Models/3DModelPiece.hpp"
#include "Sim/Misc/GlobalSynced.h"
#include "Sim/Units/Scripts/UnitScript.h"
#include "Sim/Units/Unit.h"
#include "Sim/Units/UnitHandler.h"
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
	// Piece/script names reveal model/type details. Match the existing
	// UnitsInfo detail policy: UI may query them only for a typed unit. Reuse
	// the visibility lookup below instead of looking the unit up twice.
	const CUnit* unit = nullptr;
	if (state->environment == WasmEnvironment::UI) {
		unit = WasmUiVisibility::FindUnit(unitID, WasmUiVisibility::UnitAccess::Typed);
		if (unit == nullptr) {
			slots[0].i32 = static_cast<std::int32_t>(Status::InvalidArgument);
			return nullptr;
		}
	}

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
	if (RangesOverlap(descriptorPtr, descriptorBytes, bytesPtr, bytesCapacity) ||
		RangesOverlap(descriptorPtr, descriptorBytes, metaPtr, STRING_LIST_META_BYTES) ||
		RangesOverlap(bytesPtr, bytesCapacity, metaPtr, STRING_LIST_META_BYTES)) {
		slots[0].i32 = static_cast<std::int32_t>(Status::InvalidArgument);
		return nullptr;
	}

	// This binding used to call NativeInterface::GetUnitScriptNames, which
	// copied every model-owned std::string into a 1 KiB thread-local scratch
	// buffer before this function copied the bytes again into guest memory.
	// Script pieces reference model-owned S3DModelPiece objects whose names
	// outlive this call, so Core can read those stable strings directly.
	if (gs == nullptr) {
		slots[0].i32 = static_cast<std::int32_t>(Status::NotAvailable);
		return nullptr;
	}
	if (unit == nullptr)
		unit = unitHandler.GetUnit(unitID);
	if (unit == nullptr) {
		slots[0].i32 = static_cast<std::int32_t>(Status::InvalidArgument);
		return nullptr;
	}

	const std::size_t count = unit->script == nullptr ? 0 : unit->script->pieces.size();
	if (count > std::numeric_limits<std::uint32_t>::max()) {
		slots[0].i32 = static_cast<std::int32_t>(Status::Internal);
		return nullptr;
	}
	const std::uint32_t requiredCount = static_cast<std::uint32_t>(count);

	std::uint64_t requiredBytes64 = 0;
	bool fits = requiredCount <= descriptorCapacity;
	std::uint32_t writtenBytes = 0;
	for (std::uint32_t index = 0; index < requiredCount; ++index) {
		const LocalModelPiece* piece = unit->script->pieces[index];
		const S3DModelPiece* original = piece == nullptr ? nullptr : piece->original;
		const std::string_view name = original == nullptr
			? std::string_view{}
			: std::string_view(original->name);
		const std::size_t length = name.size();
		if (length > std::numeric_limits<std::uint32_t>::max() ||
			requiredBytes64 + length > std::numeric_limits<std::uint32_t>::max()) {
			requiredBytes64 = std::numeric_limits<std::uint32_t>::max();
			fits = false;
			continue;
		}

		const std::uint32_t offset = static_cast<std::uint32_t>(requiredBytes64);
		requiredBytes64 += length;
		const std::uint32_t requiredBytes = static_cast<std::uint32_t>(requiredBytes64);
		if (!fits || !generated::CheckResultBytes(state, requiredBytes) ||
			requiredBytes > bytesCapacity) {
			fits = false;
			continue;
		}

		std::uint8_t* descriptor = descriptors.data() +
			static_cast<std::size_t>(index) * STRING_DESCRIPTOR_BYTES;
		WriteU32LE(descriptor + 0, offset);
		WriteU32LE(descriptor + 4, static_cast<std::uint32_t>(length));
		if (length != 0)
			std::memcpy(bytes.data() + offset, name.data(), length);
		writtenBytes = requiredBytes;
	}

	const std::uint32_t requiredBytes = static_cast<std::uint32_t>(requiredBytes64);
	WriteU32LE(meta.data() + 0, requiredCount);
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
