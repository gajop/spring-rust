#include "VFS.h"

#include "System/FileSystem/VFSHandler.h"
#include "System/FileSystem/FileHandler.h"
#include "System/FileSystem/ArchiveScanner.h"
#include "System/FileSystem/FileSystem.h"
#include <vector>
#include <string>

namespace {

// Error constants
static const Error NOT_READY_ERROR = {
	.code = ERROR_NOT_AVAILABLE,
	.message = "VFS system not ready"
};

static const Error FILE_NOT_FOUND_ERROR = {
	.code = ERROR_INVALID_ARGUMENT,
	.message = "File not found"
};

static const Error READ_ERROR = {
	.code = ERROR_INTERNAL,
	.message = "Failed to read file"
};

// Helper: check if VFS is ready
static bool IsReady()
{
	return (vfsHandler != nullptr);
}

// Helper: check if archiveScanner is ready
static bool IsArchiveScannerReady()
{
	return (archiveScanner != nullptr);
}

// File queries
static BoolResult NativeFileExists(const char* path)
{
	BoolResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	result.value = CFileHandler::FileExists(path, SPRING_VFS_ALL);
	return result;
}

static FileInfoResult NativeGetFileInfo(const char* path)
{
	FileInfoResult result = {};
	result.exists = false;

	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	CFileHandler fh(path, SPRING_VFS_ALL);
	if (!fh.FileExists()) {
		return result; // Not an error, just doesn't exist
	}

	result.exists = true;
	result.info.name = path;
	result.info.size = static_cast<uint32_t>(fh.FileSize());
	result.info.mode = 0644; // Default read permissions
	result.info.isDirectory = false; // Files only, VFS doesn't track directory metadata
	return result;
}

static UInt32Result NativeGetFileSize(const char* path)
{
	UInt32Result result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	CFileHandler fh(path, SPRING_VFS_ALL);
	if (!fh.FileExists()) {
		result.error = &FILE_NOT_FOUND_ERROR;
		return result;
	}

	result.value = static_cast<uint32_t>(fh.FileSize());
	return result;
}

// Directory operations
static DirListingResult NativeListDir(const char* path, const char* pattern)
{
	DirListingResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Use static storage - valid for call duration only
	static thread_local std::vector<DirEntry> entries;
	static thread_local std::vector<std::string> fileStrings;
	static thread_local std::vector<std::string> dirStrings;

	entries.clear();
	fileStrings.clear();
	dirStrings.clear();

	const std::string dirPath = path ? path : "";
	const std::string patternStr = pattern ? pattern : "*";

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

	result.entries = entries.data();
	result.count = static_cast<uint32_t>(entries.size());
	return result;
}

static BoolResult NativeIsDirectory(const char* path)
{
	BoolResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// VFS doesn't have explicit directory metadata
	// Check if directory listing returns results
	const std::string dirPath = path ? path : "";
	const auto files = vfsHandler->GetFilesInDir(dirPath, false, CVFSHandler::Section::Mod);
	const auto dirs = vfsHandler->GetDirsInDir(dirPath, false, CVFSHandler::Section::Mod);

	result.value = (!files.empty() || !dirs.empty());
	return result;
}

// File reading
static FileContentResult NativeReadFile(const char* path)
{
	FileContentResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	CFileHandler fh(path, SPRING_VFS_ALL);
	if (!fh.FileExists()) {
		result.error = &FILE_NOT_FOUND_ERROR;
		return result;
	}

	// Use static storage - valid for call duration only
	static thread_local std::vector<uint8_t> buffer;
	buffer.clear();

	const int fileSize = fh.FileSize();
	if (fileSize <= 0) {
		result.data = nullptr;
		result.size = 0;
		return result;
	}

	buffer.resize(fileSize);
	const int bytesRead = fh.Read(buffer.data(), fileSize);

	if (bytesRead != fileSize) {
		result.error = &READ_ERROR;
		return result;
	}

	result.data = buffer.data();
	result.size = static_cast<uint32_t>(buffer.size());
	return result;
}

static StringResult NativeReadFileAsString(const char* path)
{
	StringResult result = {};
	if (!IsReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	CFileHandler fh(path, SPRING_VFS_ALL);
	if (!fh.FileExists()) {
		result.error = &FILE_NOT_FOUND_ERROR;
		return result;
	}

	// Use static storage - valid for call duration only
	static thread_local std::string content;
	content.clear();

	const int fileSize = fh.FileSize();
	if (fileSize <= 0) {
		result.value = "";
		return result;
	}

	content.resize(fileSize);
	const int bytesRead = fh.Read(&content[0], fileSize);

	if (bytesRead != fileSize) {
		result.error = &READ_ERROR;
		return result;
	}

	result.value = content.c_str();
	return result;
}

// Archives
static StringArray NativeGetArchives()
{
	StringArray result = {};
	if (!IsArchiveScannerReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

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

	result.data = archiveNames.data();
	result.length = static_cast<uint32_t>(archiveNames.size());
	return result;
}

static StringArray NativeGetMaps()
{
	StringArray result = {};
	if (!IsArchiveScannerReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

	// Use static storage - valid for call duration only
	static thread_local std::vector<const char*> mapNames;
	static thread_local std::vector<std::string> mapStrings;

	mapNames.clear();
	mapStrings.clear();

	mapStrings = archiveScanner->GetMaps();

	for (const auto& str : mapStrings) {
		mapNames.push_back(str.c_str());
	}

	result.data = mapNames.data();
	result.length = static_cast<uint32_t>(mapNames.size());
	return result;
}

static StringArray NativeGetGames()
{
	StringArray result = {};
	if (!IsArchiveScannerReady()) {
		result.error = &NOT_READY_ERROR;
		return result;
	}

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

	result.data = gameNames.data();
	result.length = static_cast<uint32_t>(gameNames.size());
	return result;
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
};
