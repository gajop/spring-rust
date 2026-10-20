/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstdint>
#include <utility>
#include <span>
#include <vector>

#include "WasmJobProgram.h"

struct WasmJobBatchInput {
	std::vector<std::uint8_t> bytes;
	std::uint32_t maxOutputBytes;
};

// Explicit little-endian wire format, shared by the import and its tests.
// Input: version, count, then (input length, output limit, input bytes).
// Output: version, count, then (job status, output length, output bytes).
// No pointers or native struct layout cross the boundary.
namespace WasmJobBatch {
inline constexpr std::size_t MAX_JOBS = 256;
inline constexpr std::size_t MAX_BYTES = 1024 * 1024;
inline constexpr std::size_t MAX_TOTAL_BYTES = 16 * MAX_BYTES;
inline constexpr std::size_t MAX_WIRE_BYTES = MAX_TOTAL_BYTES + 8 + 8 * MAX_JOBS;

inline bool ReadWord(std::span<const std::uint8_t>& wire, std::uint32_t& value)
{
	if (wire.size() < 4) return false;
	value = std::uint32_t(wire[0]) | (std::uint32_t(wire[1]) << 8) |
		(std::uint32_t(wire[2]) << 16) | (std::uint32_t(wire[3]) << 24);
	wire = wire.subspan(4);
	return true;
}

inline void WriteWord(std::vector<std::uint8_t>& wire, std::uint32_t value)
{
	for (int shift = 0; shift < 32; shift += 8) wire.push_back(value >> shift);
}

// Validate the ENTIRE descriptor before copying or changing admission state.
// A second pass owns the inputs, so overlapping guest input/output is safe.
inline bool Decode(std::span<const std::uint8_t> wire,
	std::vector<WasmJobBatchInput>& inputs, std::size_t& outputCapacity)
{
	if (wire.size() > MAX_WIRE_BYTES) return false;
	auto cursor = wire;
	std::uint32_t version, count;
	if (!ReadWord(cursor, version) || version != 1 || !ReadWord(cursor, count) || count == 0 || count > MAX_JOBS)
		return false;
	std::size_t inputBytes = 0, outputBytes = 0;
	for (std::uint32_t i = 0; i < count; ++i) {
		std::uint32_t size, limit;
		if (!ReadWord(cursor, size) || !ReadWord(cursor, limit) || size > MAX_BYTES || limit > MAX_BYTES ||
			size > cursor.size() || size > MAX_TOTAL_BYTES - inputBytes || limit > MAX_TOTAL_BYTES - outputBytes)
			return false;
		inputBytes += size;
		outputBytes += limit;
		cursor = cursor.subspan(size);
	}
	if (!cursor.empty()) return false;
	std::vector<WasmJobBatchInput> owned;
	owned.reserve(count);
	cursor = wire.subspan(8);
	for (std::uint32_t i = 0; i < count; ++i) {
		std::uint32_t size, limit;
		ReadWord(cursor, size);
		ReadWord(cursor, limit);
		owned.push_back({{cursor.begin(), cursor.begin() + size}, limit});
		cursor = cursor.subspan(size);
	}
	outputCapacity = 8 + 8 * count + outputBytes;
	inputs = std::move(owned);
	return true;
}

inline std::vector<std::uint8_t> Encode(std::span<const WasmJobResult> results)
{
	std::size_t size = 8 + 8 * results.size();
	for (const auto& result: results) size += result.bytes.size();
	std::vector<std::uint8_t> wire;
	wire.reserve(size);
	WriteWord(wire, 1);
	WriteWord(wire, results.size());
	for (const auto& result: results) {
		WriteWord(wire, static_cast<std::uint32_t>(result.status));
		WriteWord(wire, result.bytes.size());
		wire.insert(wire.end(), result.bytes.begin(), result.bytes.end());
	}
	return wire;
}
} // namespace WasmJobBatch
