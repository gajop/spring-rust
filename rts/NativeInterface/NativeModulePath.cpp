/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "NativeModulePath.h"

#include <cstdlib>
#include <filesystem>

#include "Sim/Misc/ModInfo.h"
#include "System/Log/ILog.h"

namespace fs = std::filesystem;

namespace {
	std::string PlatformLibraryPath(const std::string& pathStem)
	{
		fs::path path(pathStem);
		if (path.has_extension())
			return path.generic_string();

		std::string filename = path.filename().string();

	#if defined(_WIN32)
		filename += ".dll";
	#elif defined(__APPLE__)
		filename = "lib" + filename + ".dylib";
	#else
		filename = "lib" + filename + ".so";
	#endif

		return (path.parent_path() / filename).generic_string();
	}
}

std::optional<NativeModuleSource> ResolveNativeModuleSource()
{
	const char* overridePath = std::getenv("SPRING_NATIVE_MODULE");
	if (overridePath != nullptr && overridePath[0] != '\0')
		return NativeModuleSource{overridePath, false};

	if (modInfo.nativeModule.empty())
		return std::nullopt;

	const fs::path moduleStem(modInfo.nativeModule);
	if (moduleStem.is_absolute()) {
		LOG_L(L_ERROR, "Game nativeModule must be a relative VFS path: %s", modInfo.nativeModule.c_str());
		return std::nullopt;
	}

	const fs::path normalized = moduleStem.lexically_normal();
	if (normalized.empty() || *normalized.begin() == "..") {
		LOG_L(L_ERROR, "Game nativeModule must remain inside its archive: %s", modInfo.nativeModule.c_str());
		return std::nullopt;
	}

	return NativeModuleSource{PlatformLibraryPath(normalized.generic_string()), true};
}
