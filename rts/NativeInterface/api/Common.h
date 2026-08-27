#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// API Versioning
// Version format: major*10000 + minor*100 + patch
// Example: 1.2.3 = 10203
#define NATIVE_API_VERSION(major, minor, patch) ((major)*10000 + (minor)*100 + (patch))
#define NATIVE_API_CURRENT_VERSION NATIVE_API_VERSION(1, 7, 0)

// Helper macros to extract version components
#define NATIVE_API_MAJOR(v) ((v) / 10000)
#define NATIVE_API_MINOR(v) (((v) / 100) % 100)
#define NATIVE_API_PATCH(v) ((v) % 100)

// Semantic relationships that cannot safely be recovered from C layout may
// be marked next to their declaration.  The annotation is visible to the
// Clang-based API generator and disappears from normal engine/module builds.
#if defined(__clang__) && defined(RECOIL_WASM_CODEGEN)
#define RECOIL_WASM_ANNOTATE(value) __attribute__((annotate(value)))
#else
#define RECOIL_WASM_ANNOTATE(value)
#endif

#define RECOIL_WASM_STRING RECOIL_WASM_ANNOTATE("spring.wasm.string")
#define RECOIL_WASM_BYTES RECOIL_WASM_ANNOTATE("spring.wasm.bytes")
#define RECOIL_WASM_HANDLE(family) RECOIL_WASM_ANNOTATE("spring.wasm.handle:" family)
#define RECOIL_WASM_RECORD(type) RECOIL_WASM_ANNOTATE("spring.wasm.record:" type)
#define RECOIL_WASM_LIST(element, count) RECOIL_WASM_ANNOTATE("spring.wasm.list:" element ":" count)
#define RECOIL_WASM_CALLBACK RECOIL_WASM_ANNOTATE("spring.wasm.callback")
#define RECOIL_WASM_MANUAL(reason) RECOIL_WASM_ANNOTATE("spring.wasm.manual:" reason)

struct Error {
	int32_t code;
	RECOIL_WASM_STRING const char* message;
};

#ifdef __cplusplus
}
#endif
