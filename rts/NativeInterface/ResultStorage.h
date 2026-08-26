#pragma once

#include <array>
#include <cstddef>
#include <cstring>
#include <limits>
#include <memory>
#include <new>
#include <type_traits>
#include <utility>

// Reusable storage for synchronous NativeInterface result pointers.
//
// Most queries fit in the inline buffer and therefore do not allocate.  A
// query which genuinely needs more space grows a retained spill buffer; the
// allocation is reused by later queries on the same engine thread.  Reset is
// intentionally only a cursor reset, so the common path is O(1).
class NativeResultStorage final {
public:
	static constexpr size_t INLINE_CAPACITY = 1024;

	void Reset() { bufferPos = 0; }

	bool ReserveAdditional(size_t bytes)
	{
		if (bytes > std::numeric_limits<size_t>::max() - bufferPos)
			return false;

		return Reserve(bufferPos + bytes);
	}

	template <typename T>
	T* Allocate(size_t count)
	{
		static_assert(std::is_trivially_copyable_v<T>);
		if (count == 0)
			return nullptr;
		if (count > std::numeric_limits<size_t>::max() / sizeof(T))
			return nullptr;

		constexpr size_t alignment = alignof(T);
		const size_t alignedPos = (bufferPos + alignment - 1) & ~(alignment - 1);
		const size_t bytes = count * sizeof(T);
		if (alignedPos < bufferPos || bytes > std::numeric_limits<size_t>::max() - alignedPos)
			return nullptr;
		if (!Reserve(alignedPos + bytes))
			return nullptr;

		auto* result = reinterpret_cast<T*>(Data() + alignedPos);
		bufferPos = alignedPos + bytes;
		return result;
	}

private:
	bool Reserve(size_t required)
	{
		if (required <= INLINE_CAPACITY || required <= heapCapacity)
			return true;

		size_t newCapacity = (heapCapacity == 0) ? INLINE_CAPACITY : heapCapacity;
		while (newCapacity < required) {
			if (newCapacity > std::numeric_limits<size_t>::max() / 2) {
				newCapacity = required;
				break;
			}
			newCapacity *= 2;
		}

		std::unique_ptr<std::byte[]> replacement(new (std::nothrow) std::byte[newCapacity]);
		if (replacement == nullptr)
			return false;

		if (bufferPos != 0)
			std::memcpy(replacement.get(), Data(), bufferPos);

		heapStorage = std::move(replacement);
		heapCapacity = newCapacity;
		return true;
	}

	std::byte* Data()
	{
		return (heapStorage != nullptr) ? heapStorage.get() : inlineStorage.data();
	}

	alignas(std::max_align_t) std::array<std::byte, INLINE_CAPACITY> inlineStorage{};
	std::unique_ptr<std::byte[]> heapStorage;
	size_t heapCapacity = 0;
	size_t bufferPos = 0;
};
