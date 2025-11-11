#pragma once

#include <stdint.h>
#include "CommonTypes.h"

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Virtual File System API
// @see rts/Lua/LuaVFS.cpp
//
// File system access (archives, maps, mods, etc.)
// ============================================================================

// File info
struct FileInfo {
	const char* name;
	uint32_t size;
	uint32_t mode;  // File mode (permissions)
	bool isDirectory;
};

struct FileInfoResult {
	const Error* error;
	FileInfo info;
	bool exists;
};

// Directory listing entry
struct DirEntry {
	const char* name;
	bool isDirectory;
};

struct DirListingResult {
	const Error* error;
	DirEntry* entries;
	uint32_t count;
};

// File content
struct FileContentResult {
	const Error* error;
	const uint8_t* data;
	uint32_t size;
};

// API structure
struct VFSApi {
	// File queries
	BoolResult (*FileExists)(const char* path);
	FileInfoResult (*GetFileInfo)(const char* path);
	UInt32Result (*GetFileSize)(const char* path);

	// Directory operations
	DirListingResult (*ListDir)(const char* path, const char* pattern);
	BoolResult (*IsDirectory)(const char* path);

	// File reading
	FileContentResult (*ReadFile)(const char* path);
	StringResult (*ReadFileAsString)(const char* path);

	// Archives
	StringArray (*GetArchives)();
	StringArray (*GetMaps)();
	StringArray (*GetGames)();
};

extern const VFSApi VFS_API;

#ifdef __cplusplus
}
#endif
