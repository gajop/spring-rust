/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "Platform.h"

#include "Game/GameVersion.h"
#include "System/Platform/Misc.h"

#include <ctime>

namespace {

static const Error LOCAL_TIME_ERROR = { .code = native_error::OperationFailed, .message = "the local time is not available" };

static void NativeGetArchitecture(const GetArchitectureQuery*, GetArchitectureResult* result)
{
	static const std::string architecture = Platform::GetArchitectureStr();

	result->error = nullptr;
	result->architecture = architecture.c_str();
}

static void NativeIsHeadless(const IsHeadlessQuery*, IsHeadlessResult* result)
{
	result->error = nullptr;
	result->isHeadless = SpringVersion::IsHeadless();
}

static void NativeGetLocalTime(const GetLocalTimeQuery*, GetLocalTimeResult* result)
{
	const std::time_t now = std::time(nullptr);
	std::tm local = {};
#ifdef _WIN32
	const bool ok = localtime_s(&local, &now) == 0;
#else
	const bool ok = localtime_r(&now, &local) != nullptr;
#endif
	result->error = ok ? nullptr : &LOCAL_TIME_ERROR;
	result->year = local.tm_year + 1900;
	result->month = local.tm_mon + 1;
	result->day = local.tm_mday;
	result->hour = local.tm_hour;
	result->minute = local.tm_min;
	result->second = local.tm_sec;
}

} // namespace

const PlatformApi PLATFORM_API = {
	.GetArchitecture = NativeGetArchitecture,
	.IsHeadless = NativeIsHeadless,
	.GetLocalTime = NativeGetLocalTime,
};
