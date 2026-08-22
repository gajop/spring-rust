/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreVfsBindings.h"

#include <cstdint>

#include "WasmCoreGeneratedSupport.h"
#include "WasmCoreGuestInput.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)
namespace {

using generated::CallbackContext;
using generated::ImportGuard;
using generated::InvokeCallback;
using generated::Trap;

wasm_trap_t* UseArchive(void* environment, wasmtime_caller_t* caller,
	wasmtime_val_raw_t* slots, std::size_t slotCount)
{
	auto* state = static_cast<HostState*>(environment);
	if (state == nullptr || state->native == nullptr || state->native->vfs == nullptr ||
		state->native->vfs->UseArchive == nullptr)
		return Trap("VFS UseArchive Core binding is unavailable");
	if (slots == nullptr || slotCount != 4)
		return Trap("VFS UseArchive Core ABI signature mismatch");

	ImportGuard guard(state, 5);
	if (!guard.Ok())
		return Trap(guard.Error());
	const std::uint32_t length = static_cast<std::uint32_t>(slots[1].i32);
	if (!guard.Charge(length))
		return Trap(guard.Error());
	std::string memoryError;
	if (!generated::EnsureMemory(state, caller, memoryError))
		return Trap(memoryError);
	GuestCString<> archive;
	if (!archive.Read(state->memory, static_cast<std::uint32_t>(slots[0].i32), length)) {
		slots[0].i64 = static_cast<std::int64_t>(PackU32(
			0u, static_cast<std::int32_t>(Status::OutOfBounds)));
		return nullptr;
	}

	CallbackContext callback{};
	callback.state = state;
	callback.caller = caller;
	callback.callbackID = static_cast<std::uint32_t>(slots[2].i32);
	callback.userData = static_cast<std::uint32_t>(slots[3].i32);
	UseArchiveQuery query{archive.c_str(), InvokeCallback, &callback};
	UseArchiveResult result{};
	state->native->vfs->UseArchive(&query, &result);
	if (!callback.success)
		return Trap(callback.error.empty() ? "Core VFS callback failed" : callback.error);
	const std::int32_t status = result.error == nullptr ? 0 : result.error->code;
	slots[0].i64 = static_cast<std::int64_t>(PackU32(result.success ? 1u : 0u, status));
	return nullptr;
}

} // namespace

bool RegisterVfsImports(wasmtime_linker_t* linker, HostState* state,
	std::string& error)
{
	if (linker == nullptr || state == nullptr || state->native == nullptr ||
		state->native->vfs == nullptr) {
		error = "cannot register VFS Core imports without linker/host/API";
		return false;
	}
	const wasm_valkind_t params[] = {WASM_I32, WASM_I32, WASM_I32, WASM_I32};
	const wasm_valkind_t results[] = {WASM_I64};
	wasm_functype_t* type = MakeFuncType(params, 4, results, 1);
	wasmtime_error_t* defineError = wasmtime_linker_define_func_unchecked(
		linker, "spring:vfs", 10, "use-archive", 11, type, UseArchive, state, nullptr);
	wasm_functype_delete(type);
	if (defineError == nullptr)
		return true;
	error = ErrorMessage(defineError);
	return false;
}

#endif

} // namespace recoil::wasm::core
