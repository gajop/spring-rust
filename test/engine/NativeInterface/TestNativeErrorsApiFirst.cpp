/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "NativeInterface/api/CommonTypes.h"
#include <catch_amalgamated.hpp>

// A later platform include used to change the meaning at C++ call sites.
#undef ERROR_INVALID_ARGUMENT
#define ERROR_INVALID_ARGUMENT 87
#undef ERROR_NOT_FOUND
#define ERROR_NOT_FOUND 1168
#undef ERROR_INVALID_STATE
#define ERROR_INVALID_STATE 5023
#undef ERROR_ALREADY_EXISTS
#define ERROR_ALREADY_EXISTS 183
#undef ERROR_BUFFER_OVERFLOW
#define ERROR_BUFFER_OVERFLOW 111

TEST_CASE("Native errors survive platform headers included later")
{
	CHECK(ERROR_INVALID_ARGUMENT == 87);
	CHECK(ERROR_NOT_FOUND == 1168);
	CHECK(ERROR_INVALID_STATE == 5023);
	CHECK(ERROR_ALREADY_EXISTS == 183);
	CHECK(ERROR_BUFFER_OVERFLOW == 111);
	CHECK(native_error::InvalidArgument == 1);
	CHECK(native_error::NotFound == 3);
	CHECK(native_error::InvalidState == 5);
	CHECK(native_error::AlreadyExists == 7);
	CHECK(native_error::BufferOverflow == 9);
	const Error missing{.code = native_error::NotFound, .message = "missing"};
	CHECK(missing.code == 3);
}
