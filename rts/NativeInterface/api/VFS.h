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

// Directory listing entry
struct DirEntry {
	const char* name;
	bool isDirectory;
};

// Queries
struct FileExistsQuery { const char* path; };
struct FileExistsResult { const Error* error; bool exists; };

struct GetFileInfoQuery { const char* path; };
struct GetFileInfoResult { const Error* error; FileInfo info; bool exists; };

struct GetFileSizeQuery { const char* path; };
struct GetFileSizeResult { const Error* error; uint32_t size; };

struct ListDirQuery { const char* path; const char* pattern; };
struct ListDirResult { const Error* error; DirEntry* entries; uint32_t count; };

struct IsDirectoryQuery { const char* path; };
struct IsDirectoryResult { const Error* error; bool isDirectory; };

struct ReadFileQuery { const char* path; };
struct ReadFileResult { const Error* error; const uint8_t* data; uint32_t size; };

struct ReadFileAsStringQuery { const char* path; };
struct ReadFileAsStringResult { const Error* error; const char* content; };

struct GetArchivesQuery { uint8_t _unused; };
struct GetArchivesResult { const Error* error; const char** archives; uint32_t count; };

struct GetMapsQuery { uint8_t _unused; };
struct GetMapsResult { const Error* error; const char** maps; uint32_t count; };

struct GetGamesQuery { uint8_t _unused; };
struct GetGamesResult { const Error* error; const char** games; uint32_t count; };

struct CreateDirQuery { const char* path; };
struct CreateDirResult { const Error* error; bool success; };

struct ExtractModArchiveFileQuery { const char* path; };
struct ExtractModArchiveFileResult { const Error* error; bool success; };

struct GetMapSquareTextureQuery { int32_t texSquareX; int32_t texSquareY; int32_t lodMin; int32_t lodMax; const char* textureName; };
struct GetMapSquareTextureResult { const Error* error; bool success; };

struct SetMapSquareTextureQuery { int32_t texSquareX; int32_t texSquareY; const char* textureName; };
struct SetMapSquareTextureResult { const Error* error; bool success; };

// API structure
struct VFSApi {
	void (*FileExists)(const FileExistsQuery* query, FileExistsResult* result);
	void (*GetFileInfo)(const GetFileInfoQuery* query, GetFileInfoResult* result);
	void (*GetFileSize)(const GetFileSizeQuery* query, GetFileSizeResult* result);
	void (*ListDir)(const ListDirQuery* query, ListDirResult* result);
	void (*IsDirectory)(const IsDirectoryQuery* query, IsDirectoryResult* result);
	void (*ReadFile)(const ReadFileQuery* query, ReadFileResult* result);
	void (*ReadFileAsString)(const ReadFileAsStringQuery* query, ReadFileAsStringResult* result);
	void (*GetArchives)(const GetArchivesQuery* query, GetArchivesResult* result);
	void (*GetMaps)(const GetMapsQuery* query, GetMapsResult* result);
	void (*GetGames)(const GetGamesQuery* query, GetGamesResult* result);
	void (*CreateDir)(const CreateDirQuery* query, CreateDirResult* result);
	void (*ExtractModArchiveFile)(const ExtractModArchiveFileQuery* query, ExtractModArchiveFileResult* result);
	void (*GetMapSquareTexture)(const GetMapSquareTextureQuery* query, GetMapSquareTextureResult* result);
	void (*SetMapSquareTexture)(const SetMapSquareTextureQuery* query, SetMapSquareTextureResult* result);
};

extern const VFSApi VFS_API;

#ifdef __cplusplus
}
#endif
