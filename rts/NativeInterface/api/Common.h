#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// API Versioning
// Version format: major*10000 + minor*100 + patch
// Example: 1.2.3 = 10203
#define NATIVE_API_VERSION(major, minor, patch) ((major)*10000 + (minor)*100 + (patch))
#define NATIVE_API_CURRENT_VERSION NATIVE_API_VERSION(1, 0, 0)

// Helper macros to extract version components
#define NATIVE_API_MAJOR(v) ((v) / 10000)
#define NATIVE_API_MINOR(v) (((v) / 100) % 100)
#define NATIVE_API_PATCH(v) ((v) % 100)

struct Error {
	int32_t code;
	const char* message;
};

#ifdef __cplusplus
}
#endif
