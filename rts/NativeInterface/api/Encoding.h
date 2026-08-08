/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <stdint.h>

#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Encoding API
// @see rts/Lua/LuaEncoding.cpp
//
// Base64 helpers use byte slices for encoding and decoding results so the
// NativeInterface preserves Lua's string-as-bytes behavior, including decoded
// NUL bytes.  The encoded input is ASCII and therefore remains a C string.
// ============================================================================

struct DecodeBase64Query { const char* text; };
struct DecodeBase64Result {
	const Error* error;
	const uint8_t* decoded;
	uint32_t decodedLength;
};

struct EncodeBase64Query {
	const uint8_t* text;
	uint32_t textLength;
	bool stripPadding;
};
struct EncodeBase64Result { const Error* error; const char* encoded; };

struct IsValidBase64Query { const char* text; };
struct IsValidBase64Result { const Error* error; bool valid; };

struct DecodeBase64UrlQuery { const char* text; };
struct DecodeBase64UrlResult {
	const Error* error;
	const uint8_t* decoded;
	uint32_t decodedLength;
};

struct EncodeBase64UrlQuery {
	const uint8_t* text;
	uint32_t textLength;
};
struct EncodeBase64UrlResult { const Error* error; const char* encoded; };

struct IsValidBase64UrlQuery { const char* text; };
struct IsValidBase64UrlResult { const Error* error; bool valid; };

struct EncodingApi {
	void (*DecodeBase64)(const DecodeBase64Query* query, DecodeBase64Result* result);
	void (*EncodeBase64)(const EncodeBase64Query* query, EncodeBase64Result* result);
	void (*IsValidBase64)(const IsValidBase64Query* query, IsValidBase64Result* result);
	void (*DecodeBase64Url)(const DecodeBase64UrlQuery* query, DecodeBase64UrlResult* result);
	void (*EncodeBase64Url)(const EncodeBase64UrlQuery* query, EncodeBase64UrlResult* result);
	void (*IsValidBase64Url)(const IsValidBase64UrlQuery* query, IsValidBase64UrlResult* result);
};

extern const EncodingApi ENCODING_API;

#ifdef __cplusplus
}
#endif
