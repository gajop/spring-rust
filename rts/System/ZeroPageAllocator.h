/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#ifndef ZERO_PAGE_ALLOCATOR_H
#define ZERO_PAGE_ALLOCATOR_H

#include <algorithm>
#include <cstddef>
#include <new>
#include <type_traits>
#include <utility>
#include <vector>

#include "System/MemoryOverride.hpp"

namespace recoil {

// For large vectors whose elements mostly stay in their empty state, such as per-map-square
// grids on a mostly empty map. Memory comes zeroed from calloc without being written, and
// resize() leaves new elements as they are, so a page only becomes resident once an element
// on it is written. T must be trivially default-constructible with all-zero bytes as its
// empty state, and elements must be back in that state before the vector shrinks past them.
template<typename T> struct ZeroPageAllocator {
	static_assert(std::is_trivially_default_constructible_v<T>);

	using value_type = T;

	ZeroPageAllocator() = default;
	template<typename U> ZeroPageAllocator(const ZeroPageAllocator<U>&) {}

	T* allocate(size_t n) {
		void* p = recoil::aligned_rezalloc(nullptr, 0, n * sizeof(T), std::max(alignof(T), alignof(std::max_align_t)));

		if (p == nullptr)
			throw std::bad_alloc();

		return static_cast<T*>(p);
	}
	void deallocate(T* p, size_t) { recoil::aligned_free(p); }

	// default-initialization, which writes nothing for T
	template<typename U> void construct(U* p) { ::new (static_cast<void*>(p)) U; }
	template<typename U, typename... A> void construct(U* p, A&&... a) { ::new (static_cast<void*>(p)) U(std::forward<A>(a)...); }

	template<typename U> bool operator == (const ZeroPageAllocator<U>&) const { return true; }
};

/// Gives v n elements of all-zero bytes. Lazy: in fresh memory, left unwritten, so that only the
/// pages later written become resident. Otherwise written at once, as std::vector::resize(n, T{}).
template<typename T> void ResizeZeroed(std::vector<T, ZeroPageAllocator<T>>& v, size_t n, bool lazy)
{
	v.clear();

	if (!lazy) {
		v.resize(n, T{});
		return;
	}

	std::vector<T, ZeroPageAllocator<T>>().swap(v);
	v.resize(n);
}

}

#endif
