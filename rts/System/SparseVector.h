/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#ifndef SPARSE_VECTOR_H
#define SPARSE_VECTOR_H

#include <cstddef>
#include <vector>

#include "System/ZeroPageAllocator.h"

namespace recoil {

/**
 * One optional element per slot, created on first write: for grids of non-trivial elements
 * (vectors of objects per area) that stay mostly empty on large maps. Slots cost one pointer,
 * zeroed by the allocator and left unwritten until used.
 */
template<typename T> class SparseVector {
public:
	SparseVector() = default;
	SparseVector(const SparseVector&) = delete;
	SparseVector& operator = (const SparseVector&) = delete;
	~SparseVector() { clear(); }

	/// n empty slots, in fresh memory
	void resize(size_t n) {
		clear();
		decltype(slots)().swap(slots);
		slots.resize(n);
	}
	/// deletes every element: all slots empty again
	void clear() {
		for (T*& p: slots) {
			if (p == nullptr)
				continue;

			delete p;
			p = nullptr;
		}
	}

	size_t size() const { return slots.size(); }

	/// nullptr for an empty or out-of-range slot
	const T* get(size_t i) const { return (i < slots.size())? slots[i]: nullptr; }
	      T* get(size_t i)       { return (i < slots.size())? slots[i]: nullptr; }
	/// the slot's element, created if empty
	template<typename Init> T& write(size_t i, Init&& init) {
		T*& p = slots[i];

		if (p == nullptr)
			init(*(p = new T()));

		return *p;
	}
	T& write(size_t i) { return write(i, [](T&) {}); }

private:
	std::vector<T*, ZeroPageAllocator<T*>> slots;
};

}

#endif
