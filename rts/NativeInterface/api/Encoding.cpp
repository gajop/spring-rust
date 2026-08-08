/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "Encoding.h"

#include "../../../tools/pr-downloader/src/lib/base64/base64.h"

#include <algorithm>
#include <string>

namespace {

static const std::string BASE64_CHARS =
	"ABCDEFGHIJKLMNOPQRSTUVWXYZ"
	"abcdefghijklmnopqrstuvwxyz"
	"0123456789+/=";

static const std::string BASE64_URL_CHARS =
	"ABCDEFGHIJKLMNOPQRSTUVWXYZ"
	"abcdefghijklmnopqrstuvwxyz"
	"0123456789-_";

static const Error NULL_TEXT_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "text is null",
};

static thread_local std::string decodedStorage;
static thread_local std::string encodedStorage;

static bool IsValidBase64Text(const std::string& text)
{
	bool valid = text.find_first_not_of(BASE64_CHARS) == std::string::npos;

	if (valid) {
		const size_t firstPadding = text.find_first_of("=");
		if (firstPadding != std::string::npos)
			valid = firstPadding == (text.find_last_not_of("=") + 1);
	}

	return valid;
}

static void NativeDecodeBase64(const DecodeBase64Query* query, DecodeBase64Result* result)
{
	result->error = nullptr;
	result->decoded = nullptr;
	result->decodedLength = 0;

	if (query->text == nullptr) {
		result->error = &NULL_TEXT_ERROR;
		return;
	}

	decodedStorage = base64_decode(query->text);
	result->decoded = reinterpret_cast<const uint8_t*>(decodedStorage.data());
	result->decodedLength = static_cast<uint32_t>(decodedStorage.size());
}

static void NativeEncodeBase64(const EncodeBase64Query* query, EncodeBase64Result* result)
{
	result->error = nullptr;
	result->encoded = nullptr;

	if (query->text == nullptr && query->textLength != 0) {
		result->error = &NULL_TEXT_ERROR;
		return;
	}

	encodedStorage = base64_encode(
		reinterpret_cast<const unsigned char*>(query->text),
		query->textLength
	);
	if (query->stripPadding)
		encodedStorage.erase(encodedStorage.find_last_not_of("=") + 1);

	result->encoded = encodedStorage.c_str();
}

static void NativeIsValidBase64(const IsValidBase64Query* query, IsValidBase64Result* result)
{
	result->error = nullptr;
	result->valid = false;

	if (query->text == nullptr) {
		result->error = &NULL_TEXT_ERROR;
		return;
	}

	result->valid = IsValidBase64Text(query->text);
}

static void NativeDecodeBase64Url(const DecodeBase64UrlQuery* query, DecodeBase64UrlResult* result)
{
	result->error = nullptr;
	result->decoded = nullptr;
	result->decodedLength = 0;

	if (query->text == nullptr) {
		result->error = &NULL_TEXT_ERROR;
		return;
	}

	std::string text(query->text);
	std::ranges::replace(text, '-', '+');
	std::ranges::replace(text, '_', '/');
	decodedStorage = base64_decode(text);
	result->decoded = reinterpret_cast<const uint8_t*>(decodedStorage.data());
	result->decodedLength = static_cast<uint32_t>(decodedStorage.size());
}

static void NativeEncodeBase64Url(const EncodeBase64UrlQuery* query, EncodeBase64UrlResult* result)
{
	result->error = nullptr;
	result->encoded = nullptr;

	if (query->text == nullptr && query->textLength != 0) {
		result->error = &NULL_TEXT_ERROR;
		return;
	}

	encodedStorage = base64_encode(
		reinterpret_cast<const unsigned char*>(query->text),
		query->textLength
	);
	encodedStorage.erase(encodedStorage.find_last_not_of("=") + 1);
	std::ranges::replace(encodedStorage, '+', '-');
	std::ranges::replace(encodedStorage, '/', '_');

	result->encoded = encodedStorage.c_str();
}

static void NativeIsValidBase64Url(const IsValidBase64UrlQuery* query, IsValidBase64UrlResult* result)
{
	result->error = nullptr;
	result->valid = false;

	if (query->text == nullptr) {
		result->error = &NULL_TEXT_ERROR;
		return;
	}

	result->valid = query->text[0] == '\0' ||
		std::string(query->text).find_first_not_of(BASE64_URL_CHARS) == std::string::npos;
}

} // namespace

const EncodingApi ENCODING_API = {
	.DecodeBase64 = NativeDecodeBase64,
	.EncodeBase64 = NativeEncodeBase64,
	.IsValidBase64 = NativeIsValidBase64,
	.DecodeBase64Url = NativeDecodeBase64Url,
	.EncodeBase64Url = NativeEncodeBase64Url,
	.IsValidBase64Url = NativeIsValidBase64Url,
};
