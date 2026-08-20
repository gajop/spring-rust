/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <array>
#include <cstdint>
#include <limits>
#include <vector>

#include "WasmCoreAbi.h"

namespace recoil::wasm::core {

// Call-scoped C-string storage for Core-Wasm inputs. NativeInterface string
// fields are NUL-terminated pointers while the Core ABI transports ptr+len.
// Keep the common short-string path on the stack; long strings retain the
// existing owned-copy lifetime semantics through one heap allocation.
template<std::size_t InlineBytes = 256>
class GuestCString {
public:
	static_assert(InlineBytes >= 2);

	bool Read(const Memory& memory, std::uint32_t pointer, std::uint32_t length)
	{
		if (length > std::numeric_limits<std::size_t>::max() - 1)
			return false;

		if (static_cast<std::size_t>(length) + 1 <= inlineBytes.size()) {
			if (length != 0 && !memory.Read(pointer, inlineBytes.data(), length))
				return false;
			inlineBytes[length] = '\0';
			value = inlineBytes.data();
			return true;
		}

		heapBytes.resize(static_cast<std::size_t>(length) + 1);
		if (length != 0 && !memory.Read(pointer, heapBytes.data(), length))
			return false;
		heapBytes[length] = '\0';
		value = heapBytes.data();
		return true;
	}

	const char* c_str() const { return value == nullptr ? "" : value; }
	bool UsesHeap() const { return value != nullptr && value == heapBytes.data(); }

private:
	std::array<char, InlineBytes> inlineBytes{};
	std::vector<char> heapBytes;
	const char* value = nullptr;
};

} // namespace recoil::wasm::core
