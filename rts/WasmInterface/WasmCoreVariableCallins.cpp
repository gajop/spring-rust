/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmCoreVariableCallins.h"

#include <bit>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <limits>
#include <span>
#include <string_view>
#include <utility>

#include "NativeInterface/api/Callins.h"
#include "System/BenchmarkCallins.h"
#include "WasmResources.h"

namespace recoil::wasm::core {

#if defined(RECOIL_WASMTIME_AVAILABLE)
namespace {

constexpr std::size_t ADD_CONSOLE_HEADER_BYTES = 20;
constexpr std::size_t COMMAND_NOTIFY_HEADER_BYTES = 24;

struct BenchmarkScope {
	explicit BenchmarkScope(std::string_view event)
		: token(spring::benchmark_callins::Begin("wasm", event))
	{}
	~BenchmarkScope() { spring::benchmark_callins::End(std::move(token)); }

	spring::benchmark_callins::Token token;
};

struct ScratchScope {
	explicit ScratchScope(bool& inUse)
		: inUse(inUse)
	{
		inUse = true;
	}
	~ScratchScope() { inUse = false; }

	bool& inUse;
};

bool AddSize(std::size_t& value, std::size_t amount)
{
	if (amount > std::numeric_limits<std::size_t>::max() - value)
		return false;
	value += amount;
	return true;
}

void WriteU32(std::uint8_t* output, std::uint32_t value)
{
	output[0] = static_cast<std::uint8_t>(value);
	output[1] = static_cast<std::uint8_t>(value >> 8);
	output[2] = static_cast<std::uint8_t>(value >> 16);
	output[3] = static_cast<std::uint8_t>(value >> 24);
}

void WriteI32(std::uint8_t* output, std::int32_t value)
{
	WriteU32(output, static_cast<std::uint32_t>(value));
}

void WriteF32(std::uint8_t* output, float value)
{
	WriteU32(output, std::bit_cast<std::uint32_t>(value));
}

bool ResolveOptional(RawExport& target, wasmtime_context_t* context,
	const wasmtime_instance_t& instance, const char* name,
	std::span<const wasm_valkind_t> params, std::span<const wasm_valkind_t> results,
	std::string& error)
{
	return target.Resolve(context, instance, name, std::char_traits<char>::length(name),
		params, results, true, error);
}

} // namespace

bool& VariableCallinScratchInUse()
{
	// Callins execute synchronously on an engine thread. A thread-local guard is
	// intentionally broader than one module instance so hand-written and
	// generated serializers cannot overwrite each other's scratch during reentry.
	static thread_local bool inUse = false;
	return inUse;
}

bool VariableCallinBindings::Bind(wasmtime_context_t* context,
	const wasmtime_instance_t& instance, Memory& memory, std::string& error)
{
	const wasm_valkind_t payloadParams[] = {WASM_I32};
	const wasm_valkind_t i64Result[] = {WASM_I64};
	if (!ResolveOptional(addConsoleLine, context, instance,
			"spring:callin/add-console-line", payloadParams, i64Result, error) ||
		!ResolveOptional(commandNotify, context, instance,
			"spring:callin/command-notify", payloadParams, i64Result, error))
		return false;

	if (!AnyPresent())
		return true;

	if (!scratchInfo.Resolve(context, instance, "spring:callin/scratch-info",
			std::char_traits<char>::length("spring:callin/scratch-info"),
			std::span<const wasm_valkind_t>{}, i64Result, false, error))
		return false;

	wasmtime_val_raw_t slot{};
	if (!scratchInfo.Call(context, &slot, 1, error))
		return false;
	const std::uint64_t packed = static_cast<std::uint64_t>(slot.i64);
	scratchOffset = static_cast<std::uint32_t>(packed);
	scratchCapacity = static_cast<std::uint32_t>(packed >> 32);
	if (scratchCapacity == 0) {
		error = "Core variable callin scratch capacity is zero";
		return false;
	}
	if (!memory.Contains(scratchOffset, scratchCapacity)) {
		error = "Core variable callin scratch range is outside guest memory";
		return false;
	}
	return true;
}

bool VariableCallinBindings::CallBool(wasmtime_context_t* context,
	const RawExport& function, std::uint32_t usedBytes, BoolCallinResult& result,
	std::string& error) const
{
	wasmtime_val_raw_t slot{};
	slot.i32 = static_cast<std::int32_t>(usedBytes);
	if (!function.Call(context, &slot, 1, error))
		return false;

	const std::uint64_t packed = static_cast<std::uint64_t>(slot.i64);
	const std::int32_t status = UnpackStatus(packed);
	if (status != 0) {
		error = "Core variable callin returned status " + std::to_string(status);
		return false;
	}
	const std::int32_t value = UnpackI32Value(packed);
	if (value != 0 && value != 1) {
		error = "Core variable callin returned a non-boolean value";
		return false;
	}
	result.error = nullptr;
	result.value = value != 0;
	return true;
}

bool VariableCallinBindings::AddConsoleLine(wasmtime_context_t* context,
	WasmExecutionBudget& budget, Memory& memory, const AddConsoleLineQuery& query,
	BoolCallinResult& result, std::string& error) const
{
	if (!addConsoleLine.Present()) {
		error = "Core AddConsoleLine export is unavailable";
		return false;
	}
	bool& scratchInUse = VariableCallinScratchInUse();
	if (scratchInUse) {
		error = "nested Core variable callin would overwrite guest scratch";
		return false;
	}
	ScratchScope scratchScope(scratchInUse);
	BenchmarkScope benchmark("AddConsoleLine");
	const std::string_view message = query.message == nullptr
		? std::string_view{}
		: std::string_view(query.message);
	const std::string_view section = query.section == nullptr
		? std::string_view{}
		: std::string_view(query.section);

	std::size_t required = ADD_CONSOLE_HEADER_BYTES;
	if (!AddSize(required, message.size()) || !AddSize(required, section.size()) ||
		required > scratchCapacity || required > std::numeric_limits<std::uint32_t>::max()) {
		error = "Core AddConsoleLine payload exceeds guest scratch capacity";
		return false;
	}
	if (!budget.ChargeHost(static_cast<std::uint64_t>(required))) {
		error = "Core AddConsoleLine scratch host-work budget exhausted";
		return false;
	}

	std::span<std::uint8_t> scratch;
	if (!memory.MutableView(scratchOffset, required, scratch)) {
		error = "Core AddConsoleLine scratch range became invalid";
		return false;
	}

	const std::uint32_t messageOffset = static_cast<std::uint32_t>(ADD_CONSOLE_HEADER_BYTES);
	const std::uint32_t sectionOffset = messageOffset + static_cast<std::uint32_t>(message.size());
	WriteU32(scratch.data() + 0, messageOffset);
	WriteU32(scratch.data() + 4, static_cast<std::uint32_t>(message.size()));
	WriteU32(scratch.data() + 8, sectionOffset);
	WriteU32(scratch.data() + 12, static_cast<std::uint32_t>(section.size()));
	WriteI32(scratch.data() + 16, query.level);
	if (!message.empty())
		std::memcpy(scratch.data() + messageOffset, message.data(), message.size());
	if (!section.empty())
		std::memcpy(scratch.data() + sectionOffset, section.data(), section.size());

	return CallBool(context, addConsoleLine, static_cast<std::uint32_t>(required),
		result, error);
}

bool VariableCallinBindings::CommandNotify(wasmtime_context_t* context,
	WasmExecutionBudget& budget, Memory& memory, const CommandNotifyQuery& query,
	BoolCallinResult& result, std::string& error) const
{
	if (!commandNotify.Present()) {
		error = "Core CommandNotify export is unavailable";
		return false;
	}
	bool& scratchInUse = VariableCallinScratchInUse();
	if (scratchInUse) {
		error = "nested Core variable callin would overwrite guest scratch";
		return false;
	}
	ScratchScope scratchScope(scratchInUse);
	BenchmarkScope benchmark("CommandNotify");
	const NativeCallinCommand& command = query.command;
	if (command.numParams != 0 && command.params == nullptr) {
		error = "Core CommandNotify received a null parameter list";
		return false;
	}
	if (command.numParams > (std::numeric_limits<std::uint32_t>::max() -
		COMMAND_NOTIFY_HEADER_BYTES) / sizeof(float)) {
		error = "Core CommandNotify parameter count overflows its wire payload";
		return false;
	}
	const std::size_t required = COMMAND_NOTIFY_HEADER_BYTES +
		static_cast<std::size_t>(command.numParams) * sizeof(float);
	if (required > scratchCapacity) {
		error = "Core CommandNotify payload exceeds guest scratch capacity";
		return false;
	}
	if (!budget.ChargeHost(static_cast<std::uint64_t>(required))) {
		error = "Core CommandNotify scratch host-work budget exhausted";
		return false;
	}

	std::span<std::uint8_t> scratch;
	if (!memory.MutableView(scratchOffset, required, scratch)) {
		error = "Core CommandNotify scratch range became invalid";
		return false;
	}
	WriteI32(scratch.data() + 0, command.id);
	WriteI32(scratch.data() + 4, command.timeOut);
	WriteU32(scratch.data() + 8, command.pageIndex);
	WriteU32(scratch.data() + 12, command.numParams);
	WriteU32(scratch.data() + 16, command.tag);
	WriteU32(scratch.data() + 20, command.options);

	std::uint8_t* params = scratch.data() + COMMAND_NOTIFY_HEADER_BYTES;
	if constexpr (std::endian::native == std::endian::little) {
		if (command.numParams != 0)
			std::memcpy(params, command.params,
				static_cast<std::size_t>(command.numParams) * sizeof(float));
	} else {
		for (std::uint32_t index = 0; index < command.numParams; ++index)
			WriteF32(params + static_cast<std::size_t>(index) * sizeof(float),
				command.params[index]);
	}

	return CallBool(context, commandNotify, static_cast<std::uint32_t>(required),
		result, error);
}

#endif

} // namespace recoil::wasm::core
