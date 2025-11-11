#include "Memory.h"
#include <cstdlib>

namespace {

static void NativeFreeStringArray(const char** data, uint32_t length)
{
	if (data == nullptr) {
		return;
	}

	// Free each individual string
	for (uint32_t i = 0; i < length; i++) {
		if (data[i] != nullptr) {
			std::free(const_cast<char*>(data[i]));
		}
	}

	// Free the array of pointers
	std::free(const_cast<char**>(data));
}

static void NativeFreeInt32Array(int32_t* data, uint32_t length)
{
	(void)length;  // Unused, but kept for API consistency
	std::free(data);
}

static void NativeFreeUInt32Array(uint32_t* data, uint32_t length)
{
	(void)length;
	std::free(data);
}

static void NativeFreeFloatArray(float* data, uint32_t length)
{
	(void)length;
	std::free(data);
}

static void NativeFreeFloat2Array(Float2* data, uint32_t length)
{
	(void)length;
	std::free(data);
}

static void NativeFreeFloat3Array(Float3* data, uint32_t length)
{
	(void)length;
	std::free(data);
}

static void NativeFreeFloat4Array(Float4* data, uint32_t length)
{
	(void)length;
	std::free(data);
}

static void NativeFreeInt3Array(Int3* data, uint32_t length)
{
	(void)length;
	std::free(data);
}

static void NativeFree(void* ptr)
{
	std::free(ptr);
}

} // namespace

const MemoryApi MEMORY_API = {
	.FreeStringArray = NativeFreeStringArray,
	.FreeInt32Array = NativeFreeInt32Array,
	.FreeUInt32Array = NativeFreeUInt32Array,
	.FreeFloatArray = NativeFreeFloatArray,
	.FreeFloat2Array = NativeFreeFloat2Array,
	.FreeFloat3Array = NativeFreeFloat3Array,
	.FreeFloat4Array = NativeFreeFloat4Array,
	.FreeInt3Array = NativeFreeInt3Array,
	.Free = NativeFree,
};
