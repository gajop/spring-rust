/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreGfxResourceBindings.h"

#include <cstdint>
#include <span>

#include "WasmCoreGeneratedSupport.h"
#include "WasmCoreWire.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)
namespace {

using generated::ImportGuard;
using generated::Trap;

// Native Gfx.cpp names are "!native" + a uint32 counter for textures and
// "*native" + a uint32 counter for atlases. Ten decimal digits is therefore
// the largest possible suffix. Requiring this capacity before the mutating
// native call guarantees that result delivery never needs a probe/retry call.
constexpr std::uint32_t kNativeResourceNameMaxBytes =
	static_cast<std::uint32_t>((sizeof("!native") - 1u) + 10u);
static_assert(kNativeResourceNameMaxBytes == 17u);
static_assert((sizeof("*native") - 1u) + 10u == kNativeResourceNameMaxBytes);

Status ReadTextureParams(HostState* state, std::uint32_t pointer,
	GfxTextureParams& params)
{
	std::span<const std::uint8_t> wire;
	if (!state->memory.View(pointer, 56u, wire))
		return Status::OutOfBounds;

	WireReader reader(wire);
	if (!reader.U32(params.target) ||
		!reader.U32(params.format) ||
		!reader.I32(params.border) ||
		!reader.U32(params.minFilter) ||
		!reader.U32(params.magFilter) ||
		!reader.U32(params.wrapS) ||
		!reader.U32(params.wrapT) ||
		!reader.U32(params.wrapR) ||
		!reader.U32(params.compareFunc) ||
		!reader.F32(params.lodBias) ||
		!reader.F32(params.aniso) ||
		!reader.U32(params.samples) ||
		!reader.Bool(params.fbo) ||
		!reader.Bool(params.fboDepth) ||
		!reader.Finish(4u))
		return Status::InvalidArgument;
	return Status::Ok;
}

Status PrepareNameOutput(HostState* state, std::uint32_t pointer,
	std::uint32_t capacity)
{
	if (capacity < kNativeResourceNameMaxBytes)
		return Status::BufferOverflow;
	if (!generated::CheckResultBytes(state, kNativeResourceNameMaxBytes))
		return Status::BufferOverflow;
	if (!state->memory.Contains(pointer, kNativeResourceNameMaxBytes))
		return Status::OutOfBounds;
	return Status::Ok;
}

wasm_trap_t* FinishNameResult(HostState* state, std::uint32_t output,
	const GfxStringResult& result, wasmtime_val_raw_t* slots)
{
	const std::int32_t errorCode = generated::NativeErrorCode(result.error);
	if (errorCode != 0) {
		slots[0].i64 = static_cast<std::int64_t>(PackU32(0u, errorCode));
		return nullptr;
	}
	if (result.value == nullptr) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0u, static_cast<std::int32_t>(Status::Internal)));
		return nullptr;
	}

	const std::size_t length = std::char_traits<char>::length(result.value);
	if (length > kNativeResourceNameMaxBytes) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0u, static_cast<std::int32_t>(Status::Internal)));
		return nullptr;
	}
	if (length != 0 && !state->memory.Write(output, result.value, length))
		return Trap("Gfx resource-name output range changed unexpectedly");

	slots[0].i64 = static_cast<std::int64_t>(PackU32(
		static_cast<std::uint32_t>(length), 0));
	return nullptr;
}

wasm_trap_t* CreateTexture(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->gfx == nullptr ||
		state->native->gfx->CreateTexture == nullptr)
		return Trap("Gfx CreateTexture Core binding is unavailable");
	if (slots == nullptr || slotCount != 6u)
		return Trap("Gfx CreateTexture Core ABI signature mismatch");

	ImportGuard guard(state, 8u);
	if (!guard.Ok())
		return Trap(guard.Error());
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);

	const std::uint32_t output = static_cast<std::uint32_t>(slots[4].i32);
	const std::uint32_t capacity = static_cast<std::uint32_t>(slots[5].i32);
	const Status outputStatus = PrepareNameOutput(state, output, capacity);
	if (outputStatus != Status::Ok) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(kNativeResourceNameMaxBytes, static_cast<std::int32_t>(outputStatus)));
		return nullptr;
	}

	GfxCreateTextureQuery query{};
	query.xsize = slots[0].i32;
	query.ysize = slots[1].i32;
	query.zsize = slots[2].i32;
	const Status paramsStatus = ReadTextureParams(
		state, static_cast<std::uint32_t>(slots[3].i32), query.params);
	if (paramsStatus != Status::Ok) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(0u, static_cast<std::int32_t>(paramsStatus)));
		return nullptr;
	}
	if (!guard.Charge(kNativeResourceNameMaxBytes))
		return Trap(guard.Error());

	GfxStringResult result{};
	state->native->gfx->CreateTexture(&query, &result);
	return FinishNameResult(state, output, result, slots);
}

wasm_trap_t* CreateTextureAtlas(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->gfx == nullptr ||
		state->native->gfx->CreateTextureAtlas == nullptr)
		return Trap("Gfx CreateTextureAtlas Core binding is unavailable");
	if (slots == nullptr || slotCount != 5u)
		return Trap("Gfx CreateTextureAtlas Core ABI signature mismatch");

	ImportGuard guard(state, 7u);
	if (!guard.Ok())
		return Trap(guard.Error());
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);

	const std::uint32_t output = static_cast<std::uint32_t>(slots[3].i32);
	const std::uint32_t capacity = static_cast<std::uint32_t>(slots[4].i32);
	const Status outputStatus = PrepareNameOutput(state, output, capacity);
	if (outputStatus != Status::Ok) {
		slots[0].i64 = static_cast<std::int64_t>(
			PackU32(kNativeResourceNameMaxBytes, static_cast<std::int32_t>(outputStatus)));
		return nullptr;
	}
	if (!guard.Charge(kNativeResourceNameMaxBytes))
		return Trap(guard.Error());

	GfxCreateTextureAtlasQuery query{
		slots[0].i32,
		slots[1].i32,
		slots[2].i32,
	};
	GfxStringResult result{};
	state->native->gfx->CreateTextureAtlas(&query, &result);
	return FinishNameResult(state, output, result, slots);
}

bool Define(wasmtime_linker_t* linker, const char* name, wasm_functype_t* type,
	wasmtime_func_unchecked_callback_t callback, HostState* state, std::string& error)
{
	constexpr char moduleName[] = "spring:gfx";
	wasmtime_error_t* defineError = wasmtime_linker_define_func_unchecked(
		linker, moduleName, sizeof(moduleName) - 1u,
		name, std::char_traits<char>::length(name), type, callback, state, nullptr);
	wasm_functype_delete(type);
	if (defineError == nullptr)
		return true;
	error = ErrorMessage(defineError);
	return false;
}

} // namespace

bool RegisterGfxResourceImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error)
{
	if (linker == nullptr || state == nullptr || state->native == nullptr ||
		state->native->gfx == nullptr) {
		error = "cannot register Gfx resource Core imports without linker/host/API";
		return false;
	}

	const wasm_valkind_t result[] = {WASM_I64};
	const wasm_valkind_t createTextureParams[] = {
		WASM_I32, WASM_I32, WASM_I32, WASM_I32, WASM_I32, WASM_I32,
	};
	if (!Define(linker, "create-texture",
			MakeFuncType(createTextureParams, 6u, result, 1u),
			CreateTexture, state, error))
		return false;

	const wasm_valkind_t createAtlasParams[] = {
		WASM_I32, WASM_I32, WASM_I32, WASM_I32, WASM_I32,
	};
	return Define(linker, "create-texture-atlas",
		MakeFuncType(createAtlasParams, 5u, result, 1u),
		CreateTextureAtlas, state, error);
}

#endif

} // namespace recoil::wasm::core
