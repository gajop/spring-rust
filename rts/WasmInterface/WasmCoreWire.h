/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <bit>
#include <cstddef>
#include <cstdint>
#include <cstring>
#include <span>

namespace recoil::wasm::core {

// Canonical fixed-layout wire codec used by generated Core ABI bindings.
// WebAssembly linear memory is little-endian; never memcpy native C++ structs
// across the boundary because native padding/bool representation/endianness are
// not ABI contracts.
class WireWriter {
public:
	explicit WireWriter(std::span<std::uint8_t> bytes)
		: bytes(bytes)
	{
	}

	bool U8(std::uint8_t value) { return PutByte(value); }
	bool Bool(bool value) { return U32(value ? 1u : 0u); }
	bool I32(std::int32_t value) { return U32(static_cast<std::uint32_t>(value)); }
	bool U32(std::uint32_t value)
	{
		if (!Align(4) || !Reserve(4)) return false;
		bytes[cursor + 0] = static_cast<std::uint8_t>(value);
		bytes[cursor + 1] = static_cast<std::uint8_t>(value >> 8);
		bytes[cursor + 2] = static_cast<std::uint8_t>(value >> 16);
		bytes[cursor + 3] = static_cast<std::uint8_t>(value >> 24);
		cursor += 4;
		return true;
	}
	bool PatchU32(std::size_t offset, std::uint32_t value)
	{
		// Callers may save the writer cursor before the placeholder U32 is
		// emitted. Mirror U32's alignment so variable-length preceding payloads
		// still backpatch the exact four bytes that U32 reserved.
		if (offset > bytes.size())
			return false;
		const std::size_t aligned = (offset + 3u) & ~std::size_t{3u};
		if (aligned < offset || aligned > bytes.size() || bytes.size() - aligned < 4u)
			return false;
		offset = aligned;
		bytes[offset + 0] = static_cast<std::uint8_t>(value);
		bytes[offset + 1] = static_cast<std::uint8_t>(value >> 8);
		bytes[offset + 2] = static_cast<std::uint8_t>(value >> 16);
		bytes[offset + 3] = static_cast<std::uint8_t>(value >> 24);
		return true;
	}
	bool I64(std::int64_t value) { return U64(static_cast<std::uint64_t>(value)); }
	bool U64(std::uint64_t value)
	{
		if (!Align(8) || !Reserve(8)) return false;
		for (unsigned shift = 0; shift < 64; shift += 8)
			bytes[cursor++] = static_cast<std::uint8_t>(value >> shift);
		return true;
	}
	bool F32(float value) { return U32(std::bit_cast<std::uint32_t>(value)); }
	bool F64(double value) { return U64(std::bit_cast<std::uint64_t>(value)); }

	// Dynamic payloads are length-prefixed by their generated caller. This bulk
	// primitive writes the payload itself without an O(n) sequence of U8 calls.
	// Alignment is explicit because numeric-list payloads need naturally aligned
	// guest views while strings/bytes use alignment 1.
	bool Bytes(std::span<const std::uint8_t> value, std::size_t alignment = 1)
	{
		if (!Align(alignment) || !Reserve(value.size()))
			return false;
		if (!value.empty())
			std::memcpy(bytes.data() + cursor, value.data(), value.size());
		cursor += value.size();
		return true;
	}

	bool Align(std::size_t alignment)
	{
		if (alignment == 0 || (alignment & (alignment - 1)) != 0)
			return false;
		const std::size_t aligned = (cursor + alignment - 1) & ~(alignment - 1);
		if (aligned > bytes.size())
			return false;
		while (cursor < aligned)
			bytes[cursor++] = 0;
		return true;
	}

	bool Finish(std::size_t alignment = 1)
	{
		return Align(alignment) && cursor == bytes.size();
	}

	std::size_t Offset() const { return cursor; }

private:
	bool Reserve(std::size_t count) const { return count <= bytes.size() - cursor; }
	bool PutByte(std::uint8_t value)
	{
		if (!Reserve(1)) return false;
		bytes[cursor++] = value;
		return true;
	}

	std::span<std::uint8_t> bytes;
	std::size_t cursor = 0;
};

class WireReader {
public:
	explicit WireReader(std::span<const std::uint8_t> bytes)
		: bytes(bytes)
	{
	}

	bool U8(std::uint8_t& value)
	{
		if (!Reserve(1)) return false;
		value = bytes[cursor++];
		return true;
	}
	bool Bool(bool& value)
	{
		std::uint32_t raw = 0;
		if (!U32(raw) || raw > 1) return false;
		value = raw != 0;
		return true;
	}
	bool I32(std::int32_t& value)
	{
		std::uint32_t raw = 0;
		if (!U32(raw)) return false;
		value = static_cast<std::int32_t>(raw);
		return true;
	}
	bool U32(std::uint32_t& value)
	{
		if (!Align(4) || !Reserve(4)) return false;
		value = static_cast<std::uint32_t>(bytes[cursor + 0]) |
			(static_cast<std::uint32_t>(bytes[cursor + 1]) << 8) |
			(static_cast<std::uint32_t>(bytes[cursor + 2]) << 16) |
			(static_cast<std::uint32_t>(bytes[cursor + 3]) << 24);
		cursor += 4;
		return true;
	}
	bool I64(std::int64_t& value)
	{
		std::uint64_t raw = 0;
		if (!U64(raw)) return false;
		value = static_cast<std::int64_t>(raw);
		return true;
	}
	bool U64(std::uint64_t& value)
	{
		if (!Align(8) || !Reserve(8)) return false;
		value = 0;
		for (unsigned shift = 0; shift < 64; shift += 8)
			value |= static_cast<std::uint64_t>(bytes[cursor++]) << shift;
		return true;
	}
	bool F32(float& value)
	{
		std::uint32_t raw = 0;
		if (!U32(raw)) return false;
		value = std::bit_cast<float>(raw);
		return true;
	}
	bool F64(double& value)
	{
		std::uint64_t raw = 0;
		if (!U64(raw)) return false;
		value = std::bit_cast<double>(raw);
		return true;
	}

	// Borrow a payload directly from the source span. The returned view remains
	// valid for the lifetime of the WireReader's source bytes.
	bool Bytes(std::size_t count, std::span<const std::uint8_t>& value,
		std::size_t alignment = 1)
	{
		if (!Align(alignment) || !Reserve(count))
			return false;
		value = bytes.subspan(cursor, count);
		cursor += count;
		return true;
	}

	bool Align(std::size_t alignment)
	{
		if (alignment == 0 || (alignment & (alignment - 1)) != 0)
			return false;
		const std::size_t aligned = (cursor + alignment - 1) & ~(alignment - 1);
		if (aligned > bytes.size())
			return false;
		cursor = aligned;
		return true;
	}

	bool Finish(std::size_t alignment = 1)
	{
		return Align(alignment) && cursor == bytes.size();
	}

	std::size_t Offset() const { return cursor; }

private:
	bool Reserve(std::size_t count) const { return count <= bytes.size() - cursor; }

	std::span<const std::uint8_t> bytes;
	std::size_t cursor = 0;
};

} // namespace recoil::wasm::core