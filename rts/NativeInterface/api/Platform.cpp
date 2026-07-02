/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "Platform.h"

#include "Game/GameVersion.h"
#include "System/Platform/Misc.h"

namespace {

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

} // namespace

const PlatformApi PLATFORM_API = {
	.GetArchitecture = NativeGetArchitecture,
	.IsHeadless = NativeIsHeadless,
};
