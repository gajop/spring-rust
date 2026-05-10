#include "VFS.h"

#include "System/FileSystem/VFSHandler.h"
#include "System/FileSystem/FileHandler.h"
#include "System/FileSystem/ArchiveScanner.h"
#include "System/FileSystem/FileSystem.h"
#include "Map/ReadMap.h"
#include "Map/BaseGroundDrawer.h"
#include "Map/BaseGroundTextures.h"
#include "Rendering/Textures/NamedTextures.h"
#include "System/Log/ILog.h"
#include <vector>
#include <string>

namespace {

// Scratch buffer
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

// Static errors
static const Error NOT_READY_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "VFS system not ready" };
static const Error FILE_NOT_FOUND_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "File not found" };
static const Error READ_ERROR = { .code = ERROR_INTERNAL, .message = "Failed to read file" };
static const Error INVALID_ARGUMENT_ERROR = { .code = ERROR_INVALID_ARGUMENT, .message = "Invalid argument" };
static const Error MAP_TEX_ERROR = { .code = ERROR_NOT_AVAILABLE, .message = "Map texture system not available" };

static bool IsReady() { return (vfsHandler != nullptr); }
static bool IsArchiveScannerReady() { return (archiveScanner != nullptr); }

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

	// Get files
	const auto files = vfsHandler->GetFilesInDir(dirPath, false, CVFSHandler::Section::Mod);
	for (const auto& file : files) {
		// Simple pattern matching (only supports "*" for now)
		if (patternStr == "*" || file.find(patternStr) != std::string::npos) {
			fileStrings.push_back(file);
		}
	}

	// Get directories
	const auto dirs = vfsHandler->GetDirsInDir(dirPath, false, CVFSHandler::Section::Mod);
	for (const auto& dir : dirs) {
		dirStrings.push_back(dir);
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

	const int lodMax = (query->lodMax > 0) ? query->lodMax : query->lodMin;

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

	const CNamedTextures::TexInfo* namedTexture = CNamedTextures::GetInfo(texName);
	if (namedTexture == nullptr || namedTexture->xsize != namedTexture->ysize) {
		result->error = &INVALID_ARGUMENT_ERROR;
		return;
	}

	result->success = groundTextures->SetSquareLuaTexture(query->texSquareX, query->texSquareY, namedTexture->id);
}

} // namespace

const VFSApi VFS_API = {
	.FileExists = NativeFileExists,
	.GetFileInfo = NativeGetFileInfo,
	.GetFileSize = NativeGetFileSize,
	.ListDir = NativeListDir,
	.IsDirectory = NativeIsDirectory,
	.ReadFile = NativeReadFile,
	.ReadFileAsString = NativeReadFileAsString,
	.GetArchives = NativeGetArchives,
	.GetMaps = NativeGetMaps,
	.GetGames = NativeGetGames,
	.CreateDir = NativeCreateDir,
	.ExtractModArchiveFile = NativeExtractModArchiveFile,
	.GetMapSquareTexture = NativeGetMapSquareTexture,
	.SetMapSquareTexture = NativeSetMapSquareTexture,
};
