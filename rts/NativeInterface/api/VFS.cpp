#include "VFS.h"

#include "System/FileSystem/VFSHandler.h"
#include "System/FileSystem/FileHandler.h"
#include "System/FileSystem/ArchiveScanner.h"
#include "System/FileSystem/RapidHandler.h"
#include "System/FileSystem/FileSystem.h"
#include "System/Info.h"
#include "Map/ReadMap.h"
#include "Map/BaseGroundDrawer.h"
#include "Map/BaseGroundTextures.h"
#include "Map/SMF/SMFReadMap.h"
#include "Rendering/Textures/NamedTextures.h"
#include "Rendering/GL/myGL.h"
#include "Lua/LuaZip.h"
#include "System/Log/ILog.h"
#include "System/StringUtil.h"
#include "System/Sync/SHA512.hpp"
#include "ExternalAI/LuaAIImplHandler.h"
#include "ExternalAI/AILibraryManager.h"
#include "ExternalAI/Interface/SSkirmishAILibrary.h"
#include "../tools/pr-downloader/src/pr-downloader.h"
#include <vector>
#include <string>
#include <cstring>
#include <fstream>
#include <algorithm>

extern "C" bool GetNativeGfxTextureInfo(const char* name, uint32_t* id, int32_t* xsize, int32_t* ysize, uint32_t* target);

namespace {

// Scratch buffer
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;
static thread_local std::string dynamicErrorMessage;

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "VFS system not ready" };
static const Error FILE_NOT_FOUND_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "File not found" };
static const Error READ_ERROR = { .code = ERROR_INTERNAL, .message = "Failed to read file" };
static const Error INVALID_ARGUMENT_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid argument" };
static const Error MAP_TEX_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Map texture system not available" };
static const Error HASH_TYPE_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Unsupported hash type" };
static const Error COMPRESS_ERROR = { .code = ERROR_INTERNAL, .message = "Compression failed" };
static const Error DECOMPRESS_ERROR = { .code = ERROR_INTERNAL, .message = "Decompression failed" };
static const Error ARCHIVE_NOT_FOUND_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Archive not found" };
static const Error ARCHIVE_ALREADY_LOADED_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Archive already loaded" };
static const Error UNSUPPORTED_ARCHIVE_TYPE_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Unsupported archive type" };

static const Error* MakeDynamicError(int32_t code, std::string message)
{
	dynamicErrorMessage = std::move(message);
	dynamicError.code = code;
	dynamicError.message = dynamicErrorMessage.c_str();
	return &dynamicError;
}

static bool IsReady() { return (vfsHandler != nullptr); }
static bool IsArchiveScannerReady() { return (archiveScanner != nullptr); }

static std::string GetModes(const char* mode)
{
	return ((mode == nullptr || mode[0] == '\0') ? std::string(SPRING_VFS_RAW_FIRST) : std::string(mode));
}

static void StoreStringVector(const std::vector<std::string>& strings, std::vector<std::string>& storage, std::vector<const char*>& ptrs)
{
	storage = strings;
	ptrs.clear();
	ptrs.reserve(storage.size());

	for (const auto& str : storage) {
		ptrs.push_back(str.c_str());
	}
}

template <typename T>
static void PackValues(const T* values, uint32_t count, const Error** error, const uint8_t** data, uint32_t* size)
{
	static thread_local std::vector<uint8_t> buffer;
	buffer.clear();

	if (values == nullptr && count > 0) {
		*error = &INVALID_ARGUMENT_ERROR;
		*data = nullptr;
		*size = 0;
		return;
	}

	buffer.resize(sizeof(T) * count);
	if (!buffer.empty()) {
		std::memcpy(buffer.data(), values, buffer.size());
	}

	*error = nullptr;
	*data = buffer.data();
	*size = static_cast<uint32_t>(buffer.size());
}

template <typename T>
static void UnpackValues(const uint8_t* data, uint32_t dataSize, uint32_t byteOffset, uint32_t count, const Error** error, const T** values, uint32_t* outCount)
{
	static thread_local std::vector<T> buffer;
	buffer.clear();

	if (data == nullptr || byteOffset > dataSize || ((dataSize - byteOffset) < sizeof(T))) {
		*error = nullptr;
		*values = nullptr;
		*outCount = 0;
		return;
	}

	const uint32_t maxCount = (dataSize - byteOffset) / sizeof(T);
	const uint32_t readCount = (count == 0) ? 1 : std::min(count, maxCount);
	buffer.resize(readCount);
	std::memcpy(buffer.data(), data + byteOffset, sizeof(T) * readCount);

	*error = nullptr;
	*values = buffer.data();
	*outCount = static_cast<uint32_t>(buffer.size());
}

static void NativeFileExists(const FileExistsQuery* query, FileExistsResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	result->error = nullptr;
	result->exists = CFileHandler::FileExists(query->path, SPRING_VFS_ALL);
}

static void NativeGetFileInfo(const GetFileInfoQuery* query, GetFileInfoResult* result) {
	bufferPos = 0;
	result->exists = false;

	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	CFileHandler fh(query->path, SPRING_VFS_ALL);
	if (!fh.FileExists()) {
		result->error = nullptr;
		return; // Not an error, just doesn't exist
	}

	result->error = nullptr;
	result->exists = true;
	result->info.name = query->path;
	result->info.size = static_cast<uint32_t>(fh.FileSize());
	result->info.mode = 0644; // Default read permissions
	result->info.isDirectory = false; // Files only, VFS doesn't track directory metadata
}

static void NativeGetFileSize(const GetFileSizeQuery* query, GetFileSizeResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	CFileHandler fh(query->path, SPRING_VFS_ALL);
	if (!fh.FileExists()) {
		result->error = &FILE_NOT_FOUND_ERROR;
		return;
	}

	result->error = nullptr;
	result->size = static_cast<uint32_t>(fh.FileSize());
}

static void NativeLoadFile(const LoadFileQuery* query, LoadFileResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	static thread_local std::vector<uint8_t> content;
	content.clear();

	std::string data;
	CFileHandler fh(query->path, GetModes(query->mode));
	if (!fh.FileExists()) {
		const int loadCode = fh.LoadCode();
		if (loadCode != 1) {
			result->error = nullptr;
			result->data = nullptr;
			result->size = 0;
			return;
		}
	}

	if (fh.LoadStringData(data) != 1) {
		result->error = nullptr;
		result->data = nullptr;
		result->size = 0;
		return;
	}

	content.assign(data.begin(), data.end());
	result->error = nullptr;
	result->data = content.data();
	result->size = static_cast<uint32_t>(content.size());
}

static void NativeListDir(const ListDirQuery* query, ListDirResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	// Use static storage - valid for call duration only
	static thread_local std::vector<DirEntry> entries;
	static thread_local std::vector<std::string> fileStrings;
	static thread_local std::vector<std::string> dirStrings;

	entries.clear();
	fileStrings.clear();
	dirStrings.clear();

	const std::string dirPath = query->path ? query->path : "";
	const std::string patternStr = query->pattern ? query->pattern : "*";
	const std::string modes = GetModes(query->mode);

	const auto files = CFileHandler::DirList(dirPath, patternStr, modes, query->recursive);
	for (const auto& file : files) {
		fileStrings.push_back(file);
	}

	// Build entry array
	for (const auto& file : fileStrings) {
		DirEntry entry;
		entry.name = file.c_str();
		entry.isDirectory = false;
		entries.push_back(entry);
	}

	for (const auto& dir : dirStrings) {
		DirEntry entry;
		entry.name = dir.c_str();
		entry.isDirectory = true;
		entries.push_back(entry);
	}

	result->error = nullptr;
	result->entries = entries.data();
	result->count = static_cast<uint32_t>(entries.size());
}

static void NativeSubDirs(const SubDirsQuery* query, SubDirsResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	static thread_local std::vector<std::string> dirStrings;
	static thread_local std::vector<const char*> dirs;

	const std::string dirPath = query->path ? query->path : "";
	const std::string patternStr = query->pattern ? query->pattern : "*";
	StoreStringVector(CFileHandler::SubDirs(dirPath, patternStr, GetModes(query->mode), query->recursive), dirStrings, dirs);

	result->error = nullptr;
	result->dirs = dirs.data();
	result->count = static_cast<uint32_t>(dirs.size());
}

static void NativeGetFileAbsolutePath(const GetFileAbsolutePathQuery* query, GetFileAbsolutePathResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	static thread_local std::string absolutePath;
	absolutePath.clear();

	if (query->path == nullptr || !CFileHandler::FileExists(query->path, GetModes(query->mode))) {
		result->error = nullptr;
		result->path = nullptr;
		return;
	}

	absolutePath = CFileHandler::GetFileAbsolutePath(query->path, GetModes(query->mode));
	result->error = nullptr;
	result->path = absolutePath.empty() ? nullptr : absolutePath.c_str();
}

static void NativeGetArchiveContainingFile(const GetArchiveContainingFileQuery* query, GetArchiveContainingFileResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	static thread_local std::string archiveName;
	archiveName.clear();

	if (query->path == nullptr || !CFileHandler::FileExists(query->path, GetModes(query->mode))) {
		result->error = nullptr;
		result->archiveName = nullptr;
		return;
	}

	archiveName = CFileHandler::GetArchiveContainingFile(query->path, GetModes(query->mode));
	result->error = nullptr;
	result->archiveName = archiveName.empty() ? nullptr : archiveName.c_str();
}

static void NativeIsDirectory(const IsDirectoryQuery* query, IsDirectoryResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	// VFS doesn't have explicit directory metadata
	// Check if directory listing returns results
	const std::string dirPath = query->path ? query->path : "";
	const auto files = vfsHandler->GetFilesInDir(dirPath, false, CVFSHandler::Section::Mod);
	const auto dirs = vfsHandler->GetDirsInDir(dirPath, false, CVFSHandler::Section::Mod);

	result->error = nullptr;
	result->isDirectory = (!files.empty() || !dirs.empty());
}

static void NativeReadFile(const ReadFileQuery* query, ReadFileResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	CFileHandler fh(query->path, SPRING_VFS_ALL);
	if (!fh.FileExists()) {
		result->error = &FILE_NOT_FOUND_ERROR;
		return;
	}

	// Use static storage - valid for call duration only
	static thread_local std::vector<uint8_t> buffer;
	buffer.clear();

	const int fileSize = fh.FileSize();
	if (fileSize <= 0) {
		result->error = nullptr;
		result->data = nullptr;
		result->size = 0;
		return;
	}

	buffer.resize(fileSize);
	const int bytesRead = fh.Read(buffer.data(), fileSize);

	if (bytesRead != fileSize) {
		result->error = &READ_ERROR;
		return;
	}

	result->error = nullptr;
	result->data = buffer.data();
	result->size = static_cast<uint32_t>(buffer.size());
}

static void NativeReadFileAsString(const ReadFileAsStringQuery* query, ReadFileAsStringResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	CFileHandler fh(query->path, SPRING_VFS_ALL);
	if (!fh.FileExists()) {
		result->error = &FILE_NOT_FOUND_ERROR;
		return;
	}

	// Use static storage - valid for call duration only
	static thread_local std::string content;
	content.clear();

	const int fileSize = fh.FileSize();
	if (fileSize <= 0) {
		result->error = nullptr;
		result->content = "";
		result->contentLength = 0;
		return;
	}

	content.resize(fileSize);
	const int bytesRead = fh.Read(&content[0], fileSize);

	if (bytesRead != fileSize) {
		result->error = &READ_ERROR;
		return;
	}

	result->error = nullptr;
	result->content = content.c_str();
	result->contentLength = static_cast<uint32_t>(content.size());
}

static void NativeGetArchives(const GetArchivesQuery* query, GetArchivesResult* result) {
	bufferPos = 0;
	if (!IsArchiveScannerReady()) { result->error = &NOT_READY_ERROR; return; }

	// Use static storage - valid for call duration only
	static thread_local std::vector<const char*> archiveNames;
	static thread_local std::vector<std::string> archiveStrings;

	archiveNames.clear();
	archiveStrings.clear();

	const auto& archives = archiveScanner->GetAllArchives();
	for (const auto& archive : archives) {
		archiveStrings.push_back(archive.GetNameVersioned());
	}

	for (const auto& str : archiveStrings) {
		archiveNames.push_back(str.c_str());
	}

	result->error = nullptr;
	result->archives = archiveNames.data();
	result->count = static_cast<uint32_t>(archiveNames.size());
}

static void NativeGetMaps(const GetMapsQuery* query, GetMapsResult* result) {
	bufferPos = 0;
	if (!IsArchiveScannerReady()) { result->error = &NOT_READY_ERROR; return; }

	// Use static storage - valid for call duration only
	static thread_local std::vector<const char*> mapNames;
	static thread_local std::vector<std::string> mapStrings;

	mapNames.clear();
	mapStrings.clear();

	mapStrings = archiveScanner->GetMaps();

	for (const auto& str : mapStrings) {
		mapNames.push_back(str.c_str());
	}

	result->error = nullptr;
	result->maps = mapNames.data();
	result->count = static_cast<uint32_t>(mapNames.size());
}

static void NativeGetGames(const GetGamesQuery* query, GetGamesResult* result) {
	bufferPos = 0;
	if (!IsArchiveScannerReady()) { result->error = &NOT_READY_ERROR; return; }

	// Use static storage - valid for call duration only
	static thread_local std::vector<const char*> gameNames;
	static thread_local std::vector<std::string> gameStrings;

	gameNames.clear();
	gameStrings.clear();

	const auto& games = archiveScanner->GetPrimaryMods();
	for (const auto& game : games) {
		gameStrings.push_back(game.GetNameVersioned());
	}

	for (const auto& str : gameStrings) {
		gameNames.push_back(str.c_str());
	}

	result->error = nullptr;
	result->games = gameNames.data();
	result->count = static_cast<uint32_t>(gameNames.size());
}

static void NativeGetAllArchives(const GetAllArchivesQuery* query, GetAllArchivesResult* result) {
	bufferPos = 0;
	if (!IsArchiveScannerReady()) { result->error = &NOT_READY_ERROR; return; }

	static thread_local std::vector<const char*> archiveNames;
	static thread_local std::vector<std::string> archiveStrings;

	archiveNames.clear();
	archiveStrings.clear();

	const auto& archives = archiveScanner->GetAllArchives();
	for (const auto& archive : archives) {
		archiveStrings.push_back(archive.GetNameVersioned());
	}

	for (const auto& str : archiveStrings) {
		archiveNames.push_back(str.c_str());
	}

	result->error = nullptr;
	result->archives = archiveNames.data();
	result->count = static_cast<uint32_t>(archiveNames.size());
}

static void NativeHasArchive(const HasArchiveQuery* query, HasArchiveResult* result) {
	bufferPos = 0;
	if (!IsArchiveScannerReady()) { result->error = &NOT_READY_ERROR; return; }

	const auto archiveData = archiveScanner->GetArchiveData(query->archiveName ? query->archiveName : "");
	result->error = nullptr;
	result->hasArchive = !archiveData.IsEmpty();
}

static void NativeGetLoadedArchives(const GetLoadedArchivesQuery* query, GetLoadedArchivesResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	static thread_local std::vector<std::string> archiveStrings;
	static thread_local std::vector<const char*> archiveNames;
	StoreStringVector(vfsHandler->GetAllArchiveNames(), archiveStrings, archiveNames);

	result->error = nullptr;
	result->archives = archiveNames.data();
	result->count = static_cast<uint32_t>(archiveNames.size());
}

static void NativeGetArchivePath(const GetArchivePathQuery* query, GetArchivePathResult* result) {
	bufferPos = 0;
	if (!IsArchiveScannerReady()) { result->error = &NOT_READY_ERROR; return; }

	static thread_local std::string archivePath;
	const std::string archive = archiveScanner->ArchiveFromName(query->archiveName ? query->archiveName : "");
	archivePath = archiveScanner->GetArchivePath(archive) + archive;

	result->error = nullptr;
	result->path = archivePath.empty() ? nullptr : archivePath.c_str();
}

static void NativeGetArchiveInfo(const GetArchiveInfoQuery* query, GetArchiveInfoResult* result) {
	bufferPos = 0;
	if (!IsArchiveScannerReady()) { result->error = &NOT_READY_ERROR; return; }

	static thread_local std::vector<ArchiveInfoEntry> entries;
	static thread_local std::vector<std::string> keys;
	static thread_local std::vector<std::string> types;
	static thread_local std::vector<std::string> strings;

	entries.clear();
	keys.clear();
	types.clear();
	strings.clear();

	const auto archiveData = archiveScanner->GetArchiveData(query->archiveName ? query->archiveName : "");
	if (archiveData.IsEmpty()) {
		result->error = nullptr;
		result->entries = nullptr;
		result->count = 0;
		return;
	}

	const auto& info = archiveData.GetInfo();
	keys.reserve(info.size());
	types.reserve(info.size());
	strings.reserve(info.size());
	entries.reserve(info.size());

	for (const auto& pair : info) {
		const InfoItem& item = pair.second;
		keys.push_back(pair.first);
		types.push_back(info_convertTypeToString(item.valueType));
		strings.push_back(item.valueType == INFO_VALUE_TYPE_STRING ? item.valueTypeString : "");

		ArchiveInfoEntry entry = {};
		entry.key = keys.back().c_str();
		entry.type = types.back().c_str();
		entry.stringValue = strings.back().c_str();
		entry.intValue = (item.valueType == INFO_VALUE_TYPE_INTEGER) ? item.value.typeInteger : 0;
		entry.floatValue = (item.valueType == INFO_VALUE_TYPE_FLOAT) ? item.value.typeFloat : 0.0f;
		entry.boolValue = (item.valueType == INFO_VALUE_TYPE_BOOL) ? item.value.typeBool : false;
		entries.push_back(entry);
	}

	result->error = nullptr;
	result->entries = entries.data();
	result->count = static_cast<uint32_t>(entries.size());
}

static void NativeGetArchiveDependencies(const GetArchiveDependenciesQuery* query, GetArchiveDependenciesResult* result) {
	bufferPos = 0;
	if (!IsArchiveScannerReady()) { result->error = &NOT_READY_ERROR; return; }

	static thread_local std::vector<std::string> archiveStrings;
	static thread_local std::vector<const char*> archiveNames;
	const auto archiveData = archiveScanner->GetArchiveData(query->archiveName ? query->archiveName : "");
	if (archiveData.IsEmpty()) {
		result->error = nullptr;
		result->archives = nullptr;
		result->count = 0;
		return;
	}

	StoreStringVector(archiveData.GetDependencies(), archiveStrings, archiveNames);
	result->error = nullptr;
	result->archives = archiveNames.data();
	result->count = static_cast<uint32_t>(archiveNames.size());
}

static void NativeGetArchiveReplaces(const GetArchiveReplacesQuery* query, GetArchiveReplacesResult* result) {
	bufferPos = 0;
	if (!IsArchiveScannerReady()) { result->error = &NOT_READY_ERROR; return; }

	static thread_local std::vector<std::string> archiveStrings;
	static thread_local std::vector<const char*> archiveNames;
	const auto archiveData = archiveScanner->GetArchiveData(query->archiveName ? query->archiveName : "");
	if (archiveData.IsEmpty()) {
		result->error = nullptr;
		result->archives = nullptr;
		result->count = 0;
		return;
	}

	StoreStringVector(archiveData.GetReplaces(), archiveStrings, archiveNames);
	result->error = nullptr;
	result->archives = archiveNames.data();
	result->count = static_cast<uint32_t>(archiveNames.size());
}

static void NativeGetArchiveChecksum(const GetArchiveChecksumQuery* query, GetArchiveChecksumResult* result) {
	bufferPos = 0;
	if (!IsArchiveScannerReady()) { result->error = &NOT_READY_ERROR; return; }

	static thread_local sha512::hex_digest singleChecksum;
	static thread_local sha512::hex_digest completeChecksum;
	sha512::dump_digest(archiveScanner->GetArchiveSingleChecksumBytes(query->archiveName ? query->archiveName : ""), singleChecksum);
	sha512::dump_digest(archiveScanner->GetArchiveCompleteChecksumBytes(query->archiveName ? query->archiveName : ""), completeChecksum);

	result->error = nullptr;
	result->singleChecksum = singleChecksum.data();
	result->completeChecksum = completeChecksum.data();
}

static void NativeGetNameFromRapidTag(const GetNameFromRapidTagQuery* query, GetNameFromRapidTagResult* result) {
	bufferPos = 0;
	if (!IsArchiveScannerReady()) { result->error = &NOT_READY_ERROR; return; }

	static thread_local std::string name;
	const std::string rapidName = query->rapidTag ? query->rapidTag : "";
	const std::string archiveName = GetRapidPackageFromTag(rapidName);
	name = (archiveName == rapidName) ? "" : archiveScanner->NameFromArchive(archiveName);

	result->error = nullptr;
	result->archiveName = name.empty() ? nullptr : name.c_str();
}

static void NativeGetAvailableAIs(const GetAvailableAIsQuery* query, GetAvailableAIsResult* result) {
	bufferPos = 0;
	if (!IsReady()) { result->error = &NOT_READY_ERROR; return; }

	static thread_local std::vector<AIInfoEntry> ais;
	static thread_local std::vector<std::string> shortNames;
	static thread_local std::vector<std::string> versions;
	ais.clear();
	shortNames.clear();
	versions.clear();

	const std::string gameArchiveName = query->gameArchiveName ? query->gameArchiveName : "";
	const std::string mapArchiveName = query->mapArchiveName ? query->mapArchiveName : "";

	vfsHandler->GrabLock();
	vfsHandler->SetName("NativeArchiveVFS");
	vfsHandler->UnMapArchives(false);

	if (!gameArchiveName.empty())
		vfsHandler->AddArchive(gameArchiveName, false);
	if (!mapArchiveName.empty())
		vfsHandler->AddArchive(mapArchiveName, false);

	const auto luaAIInfoItems = luaAIImplHandler.LoadInfoItems();
	const auto skirmishAIKeys = aiLibManager->GetSkirmishAIKeys();
	shortNames.reserve(luaAIInfoItems.size() + skirmishAIKeys.size());
	versions.reserve(luaAIInfoItems.size() + skirmishAIKeys.size());

	for (const auto& luaAIInfo : luaAIInfoItems) {
		shortNames.emplace_back();
		versions.emplace_back();
		for (const auto& item : luaAIInfo) {
			if (item.key == SKIRMISH_AI_PROPERTY_SHORT_NAME) shortNames.back() = item.GetValueAsString();
			if (item.key == SKIRMISH_AI_PROPERTY_VERSION) versions.back() = item.GetValueAsString();
		}
	}

	for (const auto& aiKey : skirmishAIKeys) {
		shortNames.push_back(aiKey.GetShortName());
		versions.push_back(aiKey.GetVersion());
	}

	vfsHandler->ReMapArchives(false);
	vfsHandler->SetName("SpringVFS");
	vfsHandler->FreeLock();

	for (size_t i = 0; i < shortNames.size(); ++i) {
		ais.push_back({ .shortName = shortNames[i].c_str(), .version = versions[i].c_str() });
	}

	result->error = nullptr;
	result->ais = ais.data();
	result->count = static_cast<uint32_t>(ais.size());
}

static void NativeUseArchive(const UseArchiveQuery* query, UseArchiveResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (!IsReady() || !IsArchiveScannerReady()) {
		result->error = &NOT_READY_ERROR;
		return;
	}

	if (query->archiveName == nullptr || query->callback == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const std::string archiveName = query->archiveName;
	const auto archiveData = archiveScanner->GetArchiveData(archiveName);
	if (archiveData.IsEmpty()) {
		result->error = &ARCHIVE_NOT_FOUND_ERROR;
		return;
	}

	if (vfsHandler->HasArchive(archiveName)) {
		result->error = &ARCHIVE_ALREADY_LOADED_ERROR;
		return;
	}

	vfsHandler->GrabLock();
	vfsHandler->SetName("NativeVFS");
	vfsHandler->UnMapArchives(false);
	vfsHandler->AddArchive(archiveName, false);

	query->callback(query->userData);

	vfsHandler->RemoveArchive(archiveName);
	vfsHandler->ReMapArchives(false);
	vfsHandler->SetName("SpringVFS");
	vfsHandler->FreeLock();

	result->success = true;
}

static void NativeCreateDir(const CreateDirQuery* query, CreateDirResult* result) {
	bufferPos = 0;
	if (query->path == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		result->success = false;
		return;
	}

	const std::string dir = query->path;
	if (dir.empty() || dir[0] == '/' || dir[0] == '\\' || dir[0] == '~' || dir[0] == ' ' || dir[0] == '\t' || (dir.size() > 1 && dir[1] == ':') || dir.find("..") != std::string::npos) {
		result->error = &INVALID_ARGUMENT_ERROR;
		result->success = false;
		return;
	}

	result->success = FileSystem::CreateDirectory(dir);
	result->error = nullptr;
}

static void NativeExtractModArchiveFile(const ExtractModArchiveFileQuery* query, ExtractModArchiveFileResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (query->path == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const std::string path = query->path;

	CFileHandler vfsFile(path, SPRING_VFS_ZIP);
	CFileHandler rawFile(path, SPRING_VFS_RAW);

	if (!vfsFile.FileExists()) {
		result->error = &FILE_NOT_FOUND_ERROR;
		return;
	}

	if (rawFile.FileExists()) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	std::string dname = FileSystem::GetDirectory(path);
	std::string fname = FileSystem::GetFilename(path);

#ifdef _WIN32
	const size_t s = dname.size();
	if ((s > 0) && ((dname[s - 1] == '/') || (dname[s - 1] == '\\')))
		dname = dname.substr(0, s - 1);
#endif

	if (!dname.empty() && !FileSystem::CreateDirectory(dname)) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	std::vector<uint8_t> buffer;
	std::fstream fstr(path.c_str(), std::ios::out | std::ios::binary);

	if (!vfsFile.IsBuffered()) {
		buffer.resize(vfsFile.FileSize(), 0);
		vfsFile.Read(buffer.data(), buffer.size());
	} else {
		buffer = vfsFile.GetBuffer();
	}

	fstr.write(reinterpret_cast<const char*>(buffer.data()), buffer.size());
	fstr.close();

	result->success = true;
	if (!dname.empty()) {
		LOG("[%s] extracted file \"%s\" to directory \"%s\"", __func__, fname.c_str(), dname.c_str());
	} else {
		LOG("[%s] extracted file \"%s\"", __func__, fname.c_str());
	}
}

static void NativeCompressFolder(const CompressFolderQuery* query, CompressFolderResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (query->folderPath == nullptr || query->folderPath[0] == '\0') {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const std::string folderPath = query->folderPath;
	const std::string archiveType = (query->archiveType == nullptr || query->archiveType[0] == '\0') ? "zip" : query->archiveType;
	if (archiveType != "zip") {
		result->error = &UNSUPPORTED_ARCHIVE_TYPE_ERROR;
		return;
	}

	const std::string compressedFilePath = (query->compressedFilePath == nullptr || query->compressedFilePath[0] == '\0')
		? (folderPath + ".sdz")
		: query->compressedFilePath;
	const std::string modes = GetModes(query->mode);

	if (CFileHandler::FileExists(compressedFilePath, modes)) {
		result->error = MakeDynamicError(ERROR_INVALID_ARGUMENT, "File already exists: " + compressedFilePath);
		return;
	}

	std::string error;
	if (!LuaZipFolder::ZipFolder(folderPath, compressedFilePath, query->includeFolder, modes, &error)) {
		result->error = MakeDynamicError(ERROR_OPERATION_FAILED, error);
		return;
	}

	result->success = true;
}

static void NativeZlibCompress(const ZlibCompressQuery* query, ZlibCompressResult* result) {
	bufferPos = 0;
	static thread_local std::vector<uint8_t> compressed;
	compressed = zlib::deflate(query->data, query->dataSize);

	result->error = compressed.empty() ? &COMPRESS_ERROR : nullptr;
	result->data = compressed.data();
	result->size = static_cast<uint32_t>(compressed.size());
}

static void NativeZlibDecompress(const ZlibDecompressQuery* query, ZlibDecompressResult* result) {
	bufferPos = 0;
	static thread_local std::vector<uint8_t> decompressed;
	decompressed = zlib::inflate(query->data, query->dataSize);

	result->error = decompressed.empty() ? &DECOMPRESS_ERROR : nullptr;
	result->data = decompressed.data();
	result->size = static_cast<uint32_t>(decompressed.size());
}

static void NativeCalculateHash(const CalculateHashQuery* query, CalculateHashResult* result) {
	bufferPos = 0;
	static thread_local std::string hashStorage;
	hashStorage.clear();

	if (query->data == nullptr && query->dataSize > 0) {
		result->error = &INVALID_ARGUMENT_ERROR;
		result->hash = nullptr;
		return;
	}

	switch (query->hashType) {
		case 0: {
			char* hash = CalcHash(reinterpret_cast<const char*>(query->data), query->dataSize, 0);
			hashStorage = hash;
			free(hash);
		} break;
		case 1: {
			sha512::hex_digest hexHash;
			sha512::raw_digest rawHash;
			hexHash.fill(0);
			rawHash.fill(0);
			sha512::calc_digest({reinterpret_cast<const char*>(query->data), reinterpret_cast<const char*>(query->data + query->dataSize)}, rawHash);
			sha512::dump_digest(rawHash, hexHash);
			hashStorage = hexHash.data();
		} break;
		default: {
			result->error = &HASH_TYPE_ERROR;
			result->hash = nullptr;
			return;
		}
	}

	result->error = nullptr;
	result->hash = hashStorage.c_str();
}

static void NativePackU8(const PackU8Query* query, PackU8Result* result) { PackValues(query->values, query->count, &result->error, &result->data, &result->size); }
static void NativePackU16(const PackU16Query* query, PackU16Result* result) { PackValues(query->values, query->count, &result->error, &result->data, &result->size); }
static void NativePackU32(const PackU32Query* query, PackU32Result* result) { PackValues(query->values, query->count, &result->error, &result->data, &result->size); }
static void NativePackS8(const PackS8Query* query, PackS8Result* result) { PackValues(query->values, query->count, &result->error, &result->data, &result->size); }
static void NativePackS16(const PackS16Query* query, PackS16Result* result) { PackValues(query->values, query->count, &result->error, &result->data, &result->size); }
static void NativePackS32(const PackS32Query* query, PackS32Result* result) { PackValues(query->values, query->count, &result->error, &result->data, &result->size); }
static void NativePackF32(const PackF32Query* query, PackF32Result* result) { PackValues(query->values, query->count, &result->error, &result->data, &result->size); }

static void NativeUnpackU8(const UnpackU8Query* query, UnpackU8Result* result) { UnpackValues(query->data, query->dataSize, query->byteOffset, query->count, &result->error, &result->values, &result->count); }
static void NativeUnpackU16(const UnpackU16Query* query, UnpackU16Result* result) { UnpackValues(query->data, query->dataSize, query->byteOffset, query->count, &result->error, &result->values, &result->count); }
static void NativeUnpackU32(const UnpackU32Query* query, UnpackU32Result* result) { UnpackValues(query->data, query->dataSize, query->byteOffset, query->count, &result->error, &result->values, &result->count); }
static void NativeUnpackS8(const UnpackS8Query* query, UnpackS8Result* result) { UnpackValues(query->data, query->dataSize, query->byteOffset, query->count, &result->error, &result->values, &result->count); }
static void NativeUnpackS16(const UnpackS16Query* query, UnpackS16Result* result) { UnpackValues(query->data, query->dataSize, query->byteOffset, query->count, &result->error, &result->values, &result->count); }
static void NativeUnpackS32(const UnpackS32Query* query, UnpackS32Result* result) { UnpackValues(query->data, query->dataSize, query->byteOffset, query->count, &result->error, &result->values, &result->count); }
static void NativeUnpackF32(const UnpackF32Query* query, UnpackF32Result* result) { UnpackValues(query->data, query->dataSize, query->byteOffset, query->count, &result->error, &result->values, &result->count); }

static void NativeGetMapSquareTexture(const GetMapSquareTextureQuery* query, GetMapSquareTextureResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (readMap == nullptr || readMap->GetGroundDrawer() == nullptr) {
		result->error = &MAP_TEX_ERROR;
		return;
	}

	if (query->textureName == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	CBaseGroundTextures* groundTextures = readMap->GetGroundDrawer()->GetGroundTextures();
	if (groundTextures == nullptr) {
		result->error = &MAP_TEX_ERROR;
		return;
	}

	const int lodMax = (query->lodMax > 0) ? query->lodMax : query->lodMin;

	// Mirror Lua Spring.GetMapSquareTexture, which resolves the destination
	// through the per-handle texture set. The native equivalent of that set is
	// the Gfx texture registry, so it must be tried first; SBC's scratch FBO is
	// always a native Gfx texture and is invisible to CNamedTextures.
	uint32_t nativeID = 0;
	int32_t nativeXSize = 0;
	int32_t nativeYSize = 0;
	uint32_t nativeTarget = 0;
	if (GetNativeGfxTextureInfo(query->textureName, &nativeID, &nativeXSize, &nativeYSize, &nativeTarget)) {
		if (nativeTarget != GL_TEXTURE_2D || nativeXSize != nativeYSize) {
			result->error = &INVALID_ARGUMENT_ERROR;
			return;
		}

		result->success = groundTextures->GetSquareLuaTexture(
			query->texSquareX,
			query->texSquareY,
			nativeID,
			nativeXSize,
			nativeYSize,
			query->lodMin,
			lodMax
		);
		return;
	}

	const CNamedTextures::TexInfo* namedTexture = CNamedTextures::GetInfo(query->textureName);
	if (namedTexture == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const int tid = namedTexture->id;
	const int txs = namedTexture->xsize;
	const int tys = namedTexture->ysize;

	if (txs != tys) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	result->success = groundTextures->GetSquareLuaTexture(
		query->texSquareX,
		query->texSquareY,
		tid,
		txs,
		tys,
		query->lodMin,
		lodMax
	);
}

static void NativeSetMapSquareTexture(const SetMapSquareTextureQuery* query, SetMapSquareTextureResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->success = false;

	if (readMap == nullptr || readMap->GetGroundDrawer() == nullptr) {
		result->error = &MAP_TEX_ERROR;
		return;
	}

	CBaseGroundTextures* groundTextures = readMap->GetGroundDrawer()->GetGroundTextures();
	if (groundTextures == nullptr) {
		result->error = &MAP_TEX_ERROR;
		return;
	}

	if (query->textureName == nullptr) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	const std::string texName = query->textureName;
	if (texName.empty()) {
		result->success = groundTextures->SetSquareLuaTexture(query->texSquareX, query->texSquareY, 0);
		return;
	}

	uint32_t nativeID = 0;
	int32_t nativeXSize = 0;
	int32_t nativeYSize = 0;
	uint32_t nativeTarget = 0;
	if (GetNativeGfxTextureInfo(texName.c_str(), &nativeID, &nativeXSize, &nativeYSize, &nativeTarget)) {
		if (nativeTarget != GL_TEXTURE_2D || nativeXSize != nativeYSize) {
			result->error = &INVALID_ARGUMENT_ERROR;
			return;
		}

		result->success = groundTextures->SetSquareLuaTexture(query->texSquareX, query->texSquareY, nativeID);
		return;
	}

	const CNamedTextures::TexInfo* namedTexture = CNamedTextures::GetInfo(texName);
	if (namedTexture == nullptr || namedTexture->xsize != namedTexture->ysize) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	result->success = groundTextures->SetSquareLuaTexture(query->texSquareX, query->texSquareY, namedTexture->id);
}

// NEW (no Lua equivalent): expose the SMF map-square texture grid so callers can
// size square FBOs and iterate valid (texSquareX, texSquareY) pairs from engine
// state. readMap is always a CSMFReadMap (SM3 was removed; CReadMap::LoadMap only
// ever constructs CSMFReadMap), so the static_cast is safe.
static void NativeGetMapSquareTextureInfo(const GetMapSquareTextureInfoQuery* /*query*/, GetMapSquareTextureInfoResult* result) {
	bufferPos = 0;
	result->error = nullptr;
	result->squareSize = 0;
	result->numSquaresX = 0;
	result->numSquaresZ = 0;

	if (readMap == nullptr) {
		result->error = &MAP_TEX_ERROR;
		return;
	}

	const CSMFReadMap* smfMap = static_cast<const CSMFReadMap*>(readMap);
	result->squareSize = smfMap->bigTexSize;
	result->numSquaresX = smfMap->numBigTexX;
	result->numSquaresZ = smfMap->numBigTexY;
}

} // namespace

const VFSApi VFS_API = {
	.FileExists = NativeFileExists,
	.GetFileInfo = NativeGetFileInfo,
	.GetFileSize = NativeGetFileSize,
	.LoadFile = NativeLoadFile,
	.ListDir = NativeListDir,
	.DirList = NativeListDir,
	.SubDirs = NativeSubDirs,
	.GetFileAbsolutePath = NativeGetFileAbsolutePath,
	.GetArchiveContainingFile = NativeGetArchiveContainingFile,
	.IsDirectory = NativeIsDirectory,
	.ReadFile = NativeReadFile,
	.ReadFileAsString = NativeReadFileAsString,
	.GetArchives = NativeGetArchives,
	.GetMaps = NativeGetMaps,
	.GetGames = NativeGetGames,
	.GetAllArchives = NativeGetAllArchives,
	.HasArchive = NativeHasArchive,
	.GetLoadedArchives = NativeGetLoadedArchives,
	.GetArchivePath = NativeGetArchivePath,
	.GetArchiveInfo = NativeGetArchiveInfo,
	.GetArchiveDependencies = NativeGetArchiveDependencies,
	.GetArchiveReplaces = NativeGetArchiveReplaces,
	.GetArchiveChecksum = NativeGetArchiveChecksum,
	.GetNameFromRapidTag = NativeGetNameFromRapidTag,
	.GetAvailableAIs = NativeGetAvailableAIs,
	.UseArchive = NativeUseArchive,
	.CreateDir = NativeCreateDir,
	.ExtractModArchiveFile = NativeExtractModArchiveFile,
	.CompressFolder = NativeCompressFolder,
	.ZlibCompress = NativeZlibCompress,
	.ZlibDecompress = NativeZlibDecompress,
	.CalculateHash = NativeCalculateHash,
	.PackU8 = NativePackU8,
	.PackU16 = NativePackU16,
	.PackU32 = NativePackU32,
	.PackS8 = NativePackS8,
	.PackS16 = NativePackS16,
	.PackS32 = NativePackS32,
	.PackF32 = NativePackF32,
	.UnpackU8 = NativeUnpackU8,
	.UnpackU16 = NativeUnpackU16,
	.UnpackU32 = NativeUnpackU32,
	.UnpackS8 = NativeUnpackS8,
	.UnpackS16 = NativeUnpackS16,
	.UnpackS32 = NativeUnpackS32,
	.UnpackF32 = NativeUnpackF32,
	.GetMapSquareTexture = NativeGetMapSquareTexture,
	.SetMapSquareTexture = NativeSetMapSquareTexture,
	.GetMapSquareTextureInfo = NativeGetMapSquareTextureInfo,
};
