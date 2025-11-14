#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

struct Error {
	int32_t code;
	const char* message;
};

#ifdef __cplusplus
}
#endif
