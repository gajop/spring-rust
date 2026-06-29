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

struct ArchiveInfoEntry {
	const char* key;
	const char* type;
	const char* stringValue;
	int32_t intValue;
	float floatValue;
	bool boolValue;
};

struct AIInfoEntry {
	const char* shortName;
	const char* version;
};

typedef void (*NativeVFSArchiveCallback)(void* userData);

// Queries
struct FileExistsQuery { const char* path; };
struct FileExistsResult { const Error* error; bool exists; };

struct GetFileInfoQuery { const char* path; };
struct GetFileInfoResult { const Error* error; FileInfo info; bool exists; };

struct GetFileSizeQuery { const char* path; };
struct GetFileSizeResult { const Error* error; uint32_t size; };

struct LoadFileQuery { const char* path; const char* mode; };
struct LoadFileResult { const Error* error; const uint8_t* data; uint32_t size; };

struct ListDirQuery { const char* path; const char* pattern; const char* mode; bool recursive; };
struct ListDirResult { const Error* error; DirEntry* entries; uint32_t count; };

struct SubDirsQuery { const char* path; const char* pattern; const char* mode; bool recursive; };
struct SubDirsResult { const Error* error; const char** dirs; uint32_t count; };

struct GetFileAbsolutePathQuery { const char* path; const char* mode; };
struct GetFileAbsolutePathResult { const Error* error; const char* path; };

struct GetArchiveContainingFileQuery { const char* path; const char* mode; };
struct GetArchiveContainingFileResult { const Error* error; const char* archiveName; };

struct IsDirectoryQuery { const char* path; };
struct IsDirectoryResult { const Error* error; bool isDirectory; };

struct ReadFileQuery { const char* path; };
struct ReadFileResult { const Error* error; const uint8_t* data; uint32_t size; };

struct ReadFileAsStringQuery { const char* path; };
struct ReadFileAsStringResult { const Error* error; const char* content; uint32_t contentLength; };

struct GetArchivesQuery { uint8_t _unused; };
struct GetArchivesResult { const Error* error; const char** archives; uint32_t count; };

struct GetMapsQuery { uint8_t _unused; };
struct GetMapsResult { const Error* error; const char** maps; uint32_t count; };

struct GetGamesQuery { uint8_t _unused; };
struct GetGamesResult { const Error* error; const char** games; uint32_t count; };

struct GetAllArchivesQuery { uint8_t _unused; };
struct GetAllArchivesResult { const Error* error; const char** archives; uint32_t count; };

struct HasArchiveQuery { const char* archiveName; };
struct HasArchiveResult { const Error* error; bool hasArchive; };

struct GetLoadedArchivesQuery { uint8_t _unused; };
struct GetLoadedArchivesResult { const Error* error; const char** archives; uint32_t count; };

struct GetArchivePathQuery { const char* archiveName; };
struct GetArchivePathResult { const Error* error; const char* path; };

struct GetArchiveInfoQuery { const char* archiveName; };
struct GetArchiveInfoResult { const Error* error; ArchiveInfoEntry* entries; uint32_t count; };

struct GetArchiveDependenciesQuery { const char* archiveName; };
struct GetArchiveDependenciesResult { const Error* error; const char** archives; uint32_t count; };

struct GetArchiveReplacesQuery { const char* archiveName; };
struct GetArchiveReplacesResult { const Error* error; const char** archives; uint32_t count; };

struct GetArchiveChecksumQuery { const char* archiveName; };
struct GetArchiveChecksumResult { const Error* error; const char* singleChecksum; const char* completeChecksum; };

struct GetNameFromRapidTagQuery { const char* rapidTag; };
struct GetNameFromRapidTagResult { const Error* error; const char* archiveName; };

struct GetAvailableAIsQuery { const char* gameArchiveName; const char* mapArchiveName; };
struct GetAvailableAIsResult { const Error* error; AIInfoEntry* ais; uint32_t count; };

struct UseArchiveQuery { const char* archiveName; NativeVFSArchiveCallback callback; void* userData; };
struct UseArchiveResult { const Error* error; bool success; };

struct CreateDirQuery { const char* path; };
struct CreateDirResult { const Error* error; bool success; };

struct ExtractModArchiveFileQuery { const char* path; };
struct ExtractModArchiveFileResult { const Error* error; bool success; };

struct CompressFolderQuery { const char* folderPath; const char* archiveType; const char* compressedFilePath; bool includeFolder; const char* mode; };
struct CompressFolderResult { const Error* error; bool success; };

struct ZlibCompressQuery { const uint8_t* data; uint32_t dataSize; };
struct ZlibCompressResult { const Error* error; const uint8_t* data; uint32_t size; };

struct ZlibDecompressQuery { const uint8_t* data; uint32_t dataSize; };
struct ZlibDecompressResult { const Error* error; const uint8_t* data; uint32_t size; };

struct CalculateHashQuery { const uint8_t* data; uint32_t dataSize; int32_t hashType; };
struct CalculateHashResult { const Error* error; const char* hash; };

struct PackU8Query { const uint8_t* values; uint32_t count; };
struct PackU8Result { const Error* error; const uint8_t* data; uint32_t size; };
struct PackU16Query { const uint16_t* values; uint32_t count; };
struct PackU16Result { const Error* error; const uint8_t* data; uint32_t size; };
struct PackU32Query { const uint32_t* values; uint32_t count; };
struct PackU32Result { const Error* error; const uint8_t* data; uint32_t size; };
struct PackS8Query { const int8_t* values; uint32_t count; };
struct PackS8Result { const Error* error; const uint8_t* data; uint32_t size; };
struct PackS16Query { const int16_t* values; uint32_t count; };
struct PackS16Result { const Error* error; const uint8_t* data; uint32_t size; };
struct PackS32Query { const int32_t* values; uint32_t count; };
struct PackS32Result { const Error* error; const uint8_t* data; uint32_t size; };
struct PackF32Query { const float* values; uint32_t count; };
struct PackF32Result { const Error* error; const uint8_t* data; uint32_t size; };

struct UnpackU8Query { const uint8_t* data; uint32_t dataSize; uint32_t byteOffset; uint32_t count; };
struct UnpackU8Result { const Error* error; const uint8_t* values; uint32_t count; };
struct UnpackU16Query { const uint8_t* data; uint32_t dataSize; uint32_t byteOffset; uint32_t count; };
struct UnpackU16Result { const Error* error; const uint16_t* values; uint32_t count; };
struct UnpackU32Query { const uint8_t* data; uint32_t dataSize; uint32_t byteOffset; uint32_t count; };
struct UnpackU32Result { const Error* error; const uint32_t* values; uint32_t count; };
struct UnpackS8Query { const uint8_t* data; uint32_t dataSize; uint32_t byteOffset; uint32_t count; };
struct UnpackS8Result { const Error* error; const int8_t* values; uint32_t count; };
struct UnpackS16Query { const uint8_t* data; uint32_t dataSize; uint32_t byteOffset; uint32_t count; };
struct UnpackS16Result { const Error* error; const int16_t* values; uint32_t count; };
struct UnpackS32Query { const uint8_t* data; uint32_t dataSize; uint32_t byteOffset; uint32_t count; };
struct UnpackS32Result { const Error* error; const int32_t* values; uint32_t count; };
struct UnpackF32Query { const uint8_t* data; uint32_t dataSize; uint32_t byteOffset; uint32_t count; };
struct UnpackF32Result { const Error* error; const float* values; uint32_t count; };

struct GetMapSquareTextureQuery { int32_t texSquareX; int32_t texSquareY; int32_t lodMin; const char* textureName; int32_t lodMax; };
struct GetMapSquareTextureResult { const Error* error; bool success; };

struct SetMapSquareTextureQuery { int32_t texSquareX; int32_t texSquareY; const char* textureName; };
struct SetMapSquareTextureResult { const Error* error; bool success; };

// NEW (no Lua equivalent): map-square texture grid metadata, so native callers
// can size square-texture FBOs and iterate valid (texSquareX, texSquareY) indices
// from engine state instead of hardcoding the SMF square size / recomputing counts.
// squareSize is the diffuse square edge in texels (SMF bigTexSize, e.g. 1024);
// numSquaresX/Z are the square counts (SMF numBigTexX/numBigTexY).
struct GetMapSquareTextureInfoQuery { uint8_t _unused; };
struct GetMapSquareTextureInfoResult { const Error* error; int32_t squareSize; int32_t numSquaresX; int32_t numSquaresZ; };

// API structure
struct VFSApi {
	void (*FileExists)(const FileExistsQuery* query, FileExistsResult* result);
	void (*GetFileInfo)(const GetFileInfoQuery* query, GetFileInfoResult* result);
	void (*GetFileSize)(const GetFileSizeQuery* query, GetFileSizeResult* result);
	void (*LoadFile)(const LoadFileQuery* query, LoadFileResult* result);
	void (*ListDir)(const ListDirQuery* query, ListDirResult* result);
	void (*DirList)(const ListDirQuery* query, ListDirResult* result);
	void (*SubDirs)(const SubDirsQuery* query, SubDirsResult* result);
	void (*GetFileAbsolutePath)(const GetFileAbsolutePathQuery* query, GetFileAbsolutePathResult* result);
	void (*GetArchiveContainingFile)(const GetArchiveContainingFileQuery* query, GetArchiveContainingFileResult* result);
	void (*IsDirectory)(const IsDirectoryQuery* query, IsDirectoryResult* result);
	void (*ReadFile)(const ReadFileQuery* query, ReadFileResult* result);
	void (*ReadFileAsString)(const ReadFileAsStringQuery* query, ReadFileAsStringResult* result);
	void (*GetArchives)(const GetArchivesQuery* query, GetArchivesResult* result);
	void (*GetMaps)(const GetMapsQuery* query, GetMapsResult* result);
	void (*GetGames)(const GetGamesQuery* query, GetGamesResult* result);
	void (*GetAllArchives)(const GetAllArchivesQuery* query, GetAllArchivesResult* result);
	void (*HasArchive)(const HasArchiveQuery* query, HasArchiveResult* result);
	void (*GetLoadedArchives)(const GetLoadedArchivesQuery* query, GetLoadedArchivesResult* result);
	void (*GetArchivePath)(const GetArchivePathQuery* query, GetArchivePathResult* result);
	void (*GetArchiveInfo)(const GetArchiveInfoQuery* query, GetArchiveInfoResult* result);
	void (*GetArchiveDependencies)(const GetArchiveDependenciesQuery* query, GetArchiveDependenciesResult* result);
	void (*GetArchiveReplaces)(const GetArchiveReplacesQuery* query, GetArchiveReplacesResult* result);
	void (*GetArchiveChecksum)(const GetArchiveChecksumQuery* query, GetArchiveChecksumResult* result);
	void (*GetNameFromRapidTag)(const GetNameFromRapidTagQuery* query, GetNameFromRapidTagResult* result);
	void (*GetAvailableAIs)(const GetAvailableAIsQuery* query, GetAvailableAIsResult* result);
	void (*UseArchive)(const UseArchiveQuery* query, UseArchiveResult* result);
	void (*CreateDir)(const CreateDirQuery* query, CreateDirResult* result);
	void (*ExtractModArchiveFile)(const ExtractModArchiveFileQuery* query, ExtractModArchiveFileResult* result);
	void (*CompressFolder)(const CompressFolderQuery* query, CompressFolderResult* result);
	void (*ZlibCompress)(const ZlibCompressQuery* query, ZlibCompressResult* result);
	void (*ZlibDecompress)(const ZlibDecompressQuery* query, ZlibDecompressResult* result);
	void (*CalculateHash)(const CalculateHashQuery* query, CalculateHashResult* result);
	void (*PackU8)(const PackU8Query* query, PackU8Result* result);
	void (*PackU16)(const PackU16Query* query, PackU16Result* result);
	void (*PackU32)(const PackU32Query* query, PackU32Result* result);
	void (*PackS8)(const PackS8Query* query, PackS8Result* result);
	void (*PackS16)(const PackS16Query* query, PackS16Result* result);
	void (*PackS32)(const PackS32Query* query, PackS32Result* result);
	void (*PackF32)(const PackF32Query* query, PackF32Result* result);
	void (*UnpackU8)(const UnpackU8Query* query, UnpackU8Result* result);
	void (*UnpackU16)(const UnpackU16Query* query, UnpackU16Result* result);
	void (*UnpackU32)(const UnpackU32Query* query, UnpackU32Result* result);
	void (*UnpackS8)(const UnpackS8Query* query, UnpackS8Result* result);
	void (*UnpackS16)(const UnpackS16Query* query, UnpackS16Result* result);
	void (*UnpackS32)(const UnpackS32Query* query, UnpackS32Result* result);
	void (*UnpackF32)(const UnpackF32Query* query, UnpackF32Result* result);
	void (*GetMapSquareTexture)(const GetMapSquareTextureQuery* query, GetMapSquareTextureResult* result);
	void (*SetMapSquareTexture)(const SetMapSquareTextureQuery* query, SetMapSquareTextureResult* result);
	// NEW (no Lua equivalent) — see GetMapSquareTextureInfoResult above.
	void (*GetMapSquareTextureInfo)(const GetMapSquareTextureInfoQuery* query, GetMapSquareTextureInfoResult* result);
};

extern const VFSApi VFS_API;

#ifdef __cplusplus
}
#endif
