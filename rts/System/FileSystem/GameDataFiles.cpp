/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "GameDataFiles.h"
#include "GameOutputStreams.h"

#include <algorithm>
#include <array>
#include <cstdio>
#include <filesystem>
#include <memory>
#include <system_error>

#ifdef _WIN32
	#include <windows.h>
	#include <nowide/convert.hpp>
#else
	#include <cerrno>
	#include <dirent.h>
	#include <fcntl.h>
	#include <sys/stat.h>
	#include <unistd.h>
#endif

namespace GameData {

namespace {

constexpr std::array<std::string_view, 8> EXTENSIONS = {"json", "toml", "txt", "csv", "dat", "bin", "png", "jpg"};
/// Output files are data a person or a tool reads, never a loader: no image, no config, no code.
constexpr std::array<std::string_view, 8> OUTPUT_EXTENSIONS = {"jsonl", "json", "txt", "csv", "tsv", "log", "dat", "bin"};
/// Temporary files start with a dot, so no name a caller may give can reach one.
constexpr std::string_view TEMPORARY_PREFIX = ".tmp-";

bool IsNameCharacter(char c) { return (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9') || c == '_' || c == '-'; }
bool IsLetterOrDigit(char c) { return (c >= 'a' && c <= 'z') || (c >= '0' && c <= '9'); }

/// Windows opens these as devices whatever the folder, and whatever follows a dot. Names are
/// lowercase only, so one spelling covers every case on a case-insensitive disk.
bool IsDeviceName(std::string_view stem)
{
	constexpr std::array<std::string_view, 4> plain = {"con", "prn", "aux", "nul"};
	if (std::find(plain.begin(), plain.end(), stem) != plain.end())
		return true;
	return stem.size() == 4 && (stem.substr(0, 3) == "com" || stem.substr(0, 3) == "lpt") && stem[3] >= '0' && stem[3] <= '9';
}

bool IsValidStem(std::string_view name)
{
	if (name.empty() || name.size() > MAX_COMPONENT_LENGTH)
		return false;
	if (!IsLetterOrDigit(name.front()) || !IsLetterOrDigit(name.back()))
		return false;
	return std::all_of(name.begin(), name.end(), IsNameCharacter) && !IsDeviceName(name);
}

bool Split(std::string_view path, std::vector<std::string>& components)
{
	if (path.size() > MAX_PATH_LENGTH)
		return false;
	std::size_t start = 0;
	while (true) {
		const std::size_t end = path.find('/', start);
		if (components.size() == MAX_DEPTH)
			return false;
		components.emplace_back(path.substr(start, end == std::string_view::npos ? end : end - start));
		if (end == std::string_view::npos)
			return true;
		start = end + 1;
	}
}

struct Usage {
	std::size_t files = 0;
	std::size_t folders = 0;
	std::uint64_t bytes = 0;
};

struct TreeLimits {
	std::size_t files;
	std::size_t folders;
	std::uint64_t bytes;
};
constexpr TreeLimits GAME_DATA_LIMITS = {MAX_FILES, MAX_FOLDERS, MAX_TOTAL_BYTES};

template <std::size_t N>
bool IsValidFileNameIn(std::string_view name, const std::array<std::string_view, N>& extensions)
{
	if (name.size() > MAX_COMPONENT_LENGTH)
		return false;
	const std::size_t dot = name.find('.');
	if (dot == std::string_view::npos || name.find('.', dot + 1) != std::string_view::npos)
		return false;
	const std::string_view extension = name.substr(dot + 1);
	if (std::find(extensions.begin(), extensions.end(), extension) == extensions.end())
		return false;
	return IsValidStem(name.substr(0, dot));
}

template <typename IsValidLeaf>
bool ParseFilePathWith(std::string_view path, std::vector<std::string>& components, IsValidLeaf isValidLeaf)
{
	components.clear();
	if (path.empty() || !Split(path, components))
		return false;
	for (std::size_t i = 0; i + 1 < components.size(); ++i) {
		if (!IsValidStem(components[i]))
			return false;
	}
	return isValidLeaf(components.back());
}

#ifdef _WIN32

// ---- Windows: a handle on every component, held without FILE_SHARE_DELETE, so none can be renamed
// or replaced while the operation runs; each is refused if it is a reparse point.

struct Handle {
	HANDLE value = INVALID_HANDLE_VALUE;
	Handle() = default;
	explicit Handle(HANDLE h) : value(h) {}
	Handle(const Handle&) = delete;
	Handle& operator=(const Handle&) = delete;
	Handle(Handle&& o) noexcept : value(o.value) { o.value = INVALID_HANDLE_VALUE; }
	~Handle() { if (value != INVALID_HANDLE_VALUE) CloseHandle(value); }
};

/// The directory walked to: its handles, and its path in `\\?\` form (no name rewriting, no limit).
struct Dir {
	std::vector<Handle> held;
	std::wstring path;
	std::wstring Join(std::string_view name) const { return path + L"\\" + nowide::widen(std::string(name)); }
};

std::wstring RootPath(const std::string& writeDir)
{
	std::wstring wide = nowide::widen(writeDir);
	std::wstring full(32768, L'\0');
	const DWORD length = GetFullPathNameW(wide.c_str(), static_cast<DWORD>(full.size()), full.data(), nullptr);
	if (length == 0 || length >= full.size())
		return L"";
	full.resize(length);
	while (!full.empty() && (full.back() == L'\\' || full.back() == L'/'))
		full.pop_back();
	return full.rfind(L"\\\\", 0) == 0 ? L"\\\\?\\UNC\\" + full.substr(2) : L"\\\\?\\" + full;
}

Status OpenComponent(Dir& dir, const std::string& name, bool create, bool* created = nullptr)
{
	const std::wstring path = dir.Join(name);
	const auto open = [&path]() {
		return CreateFileW(path.c_str(), FILE_READ_ATTRIBUTES, FILE_SHARE_READ | FILE_SHARE_WRITE, nullptr, OPEN_EXISTING,
			FILE_FLAG_BACKUP_SEMANTICS | FILE_FLAG_OPEN_REPARSE_POINT, nullptr);
	};
	HANDLE handle = open();
	if (handle == INVALID_HANDLE_VALUE) {
		const DWORD error = GetLastError();
		if (error != ERROR_FILE_NOT_FOUND && error != ERROR_PATH_NOT_FOUND)
			return Status::IoError;
		if (!create)
			return Status::NotFound;
		if (!CreateDirectoryW(path.c_str(), nullptr) && GetLastError() != ERROR_ALREADY_EXISTS)
			return Status::IoError;
		if (created != nullptr)
			*created = true;
		if ((handle = open()) == INVALID_HANDLE_VALUE)
			return Status::IoError;
	}
	Handle held(handle);
	BY_HANDLE_FILE_INFORMATION info;
	if (!GetFileInformationByHandle(handle, &info))
		return Status::IoError;
	if (info.dwFileAttributes & FILE_ATTRIBUTE_REPARSE_POINT)
		return Status::Link;
	if (!(info.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY))
		return Status::WrongType;
	dir.held.push_back(std::move(held));
	dir.path = path;
	return Status::Ok;
}

Status OpenWriteDir(const std::string& writeDir, Dir& dir)
{
	dir.path = RootPath(writeDir);
	return dir.path.empty() ? Status::IoError : Status::Ok;
}

enum class LeafKind { Missing, File, Folder, Link, Error };

LeafKind Leaf(const Dir& dir, std::string_view name, std::uint64_t* size = nullptr)
{
	WIN32_FILE_ATTRIBUTE_DATA data;
	if (!GetFileAttributesExW(dir.Join(name).c_str(), GetFileExInfoStandard, &data)) {
		const DWORD error = GetLastError();
		return (error == ERROR_FILE_NOT_FOUND || error == ERROR_PATH_NOT_FOUND) ? LeafKind::Missing : LeafKind::Error;
	}
	if (data.dwFileAttributes & FILE_ATTRIBUTE_REPARSE_POINT)
		return LeafKind::Link;
	if (data.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY)
		return LeafKind::Folder;
	if (size != nullptr)
		*size = (static_cast<std::uint64_t>(data.nFileSizeHigh) << 32) | data.nFileSizeLow;
	return LeafKind::File;
}

template <typename Visit>
Status ForEach(const Dir& dir, Visit visit)
{
	WIN32_FIND_DATAW data;
	const HANDLE find = FindFirstFileExW((dir.path + L"\\*").c_str(), FindExInfoBasic, &data, FindExSearchNameMatch, nullptr, 0);
	if (find == INVALID_HANDLE_VALUE)
		return GetLastError() == ERROR_FILE_NOT_FOUND ? Status::Ok : Status::IoError;
	do {
		const std::string name = nowide::narrow(data.cFileName);
		if (name == "." || name == "..")
			continue;
		const LeafKind kind = (data.dwFileAttributes & FILE_ATTRIBUTE_REPARSE_POINT) ? LeafKind::Link
			: (data.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) ? LeafKind::Folder : LeafKind::File;
		if (!visit(name, kind, (static_cast<std::uint64_t>(data.nFileSizeHigh) << 32) | data.nFileSizeLow))
			break;
	} while (FindNextFileW(find, &data));
	FindClose(find);
	return Status::Ok;
}

void RemoveFile(const Dir& dir, std::string_view name) { DeleteFileW(dir.Join(name).c_str()); }

Status RemoveFolder(const Dir& dir, std::string_view name)
{
	if (RemoveDirectoryW(dir.Join(name).c_str()))
		return Status::Ok;
	return GetLastError() == ERROR_DIR_NOT_EMPTY ? Status::NotEmpty : Status::IoError;
}

Status RemoveLeafFile(const Dir& dir, std::string_view name)
{
	return DeleteFileW(dir.Join(name).c_str()) ? Status::Ok : Status::IoError;
}

Status WriteTemporary(const Dir& dir, const std::string& temporary, const void* data, std::size_t size)
{
	const HANDLE file = CreateFileW(dir.Join(temporary).c_str(), GENERIC_WRITE, 0, nullptr, CREATE_NEW,
		FILE_ATTRIBUTE_NORMAL | FILE_FLAG_OPEN_REPARSE_POINT, nullptr);
	if (file == INVALID_HANDLE_VALUE)
		return Status::IoError;
	Handle held(file);
	DWORD written = 0;
	if (size > 0 && (!WriteFile(file, data, static_cast<DWORD>(size), &written, nullptr) || written != size))
		return Status::IoError;
	return FlushFileBuffers(file) ? Status::Ok : Status::IoError;
}

Status ReplaceWith(const Dir& dir, const std::string& temporary, std::string_view name)
{
	return MoveFileExW(dir.Join(temporary).c_str(), dir.Join(name).c_str(), MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH)
		? Status::Ok : Status::IoError;
}

Status ReadWhole(const Dir& dir, std::string_view name, std::vector<std::uint8_t>& out)
{
	// CreateFileW without FILE_FLAG_BACKUP_SEMANTICS fails on a folder with a plain access error.
	if (Leaf(dir, name) == LeafKind::Folder)
		return Status::WrongType;
	const HANDLE file = CreateFileW(dir.Join(name).c_str(), GENERIC_READ, FILE_SHARE_READ, nullptr, OPEN_EXISTING,
		FILE_FLAG_OPEN_REPARSE_POINT, nullptr);
	if (file == INVALID_HANDLE_VALUE) {
		const DWORD error = GetLastError();
		return (error == ERROR_FILE_NOT_FOUND || error == ERROR_PATH_NOT_FOUND) ? Status::NotFound : Status::IoError;
	}
	Handle held(file);
	BY_HANDLE_FILE_INFORMATION info;
	if (!GetFileInformationByHandle(file, &info))
		return Status::IoError;
	if (info.dwFileAttributes & FILE_ATTRIBUTE_REPARSE_POINT)
		return Status::Link;
	if (info.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY)
		return Status::WrongType;
	if (GetFileType(file) != FILE_TYPE_DISK)
		return Status::WrongType;
	if (info.nFileSizeHigh != 0 || info.nFileSizeLow > MAX_FILE_BYTES)
		return Status::FileTooLarge;
	out.resize(info.nFileSizeLow);
	DWORD read = 0;
	if (!out.empty() && (!ReadFile(file, out.data(), info.nFileSizeLow, &read, nullptr) || read != info.nFileSizeLow))
		return Status::IoError;
	return Status::Ok;
}

void FlushDir(const Dir&) {}

OutputStatus OpenAppend(const Dir& dir, const std::string& name, bool append, void*& file, std::uint64_t& size)
{
	// FILE_APPEND_DATA alone: every write goes to the end. The reparse flag opens a link itself,
	// which is then refused, never what it points to.
	const HANDLE handle = CreateFileW(dir.Join(name).c_str(), FILE_APPEND_DATA, FILE_SHARE_READ, nullptr,
		append ? OPEN_ALWAYS : CREATE_NEW, FILE_ATTRIBUTE_NORMAL | FILE_FLAG_OPEN_REPARSE_POINT, nullptr);
	if (handle == INVALID_HANDLE_VALUE) {
		if (GetLastError() == ERROR_FILE_EXISTS)
			return OutputStatus::Exists;
		return Leaf(dir, name) == LeafKind::Folder ? OutputStatus::WrongType : OutputStatus::IoError;
	}
	BY_HANDLE_FILE_INFORMATION info;
	const auto refuse = [handle](OutputStatus status) { CloseHandle(handle); return status; };
	if (!GetFileInformationByHandle(handle, &info))
		return refuse(OutputStatus::IoError);
	if (info.dwFileAttributes & FILE_ATTRIBUTE_REPARSE_POINT)
		return refuse(OutputStatus::Link);
	if ((info.dwFileAttributes & FILE_ATTRIBUTE_DIRECTORY) || GetFileType(handle) != FILE_TYPE_DISK)
		return refuse(OutputStatus::WrongType);
	file = handle;
	size = (static_cast<std::uint64_t>(info.nFileSizeHigh) << 32) | info.nFileSizeLow;
	return OutputStatus::Ok;
}

bool AppendAll(void* file, const std::uint8_t* data, std::size_t size)
{
	while (size > 0) {
		const DWORD chunk = static_cast<DWORD>(std::min<std::size_t>(size, 1u << 30));
		DWORD written = 0;
		if (!WriteFile(static_cast<HANDLE>(file), data, chunk, &written, nullptr) || written == 0)
			return false;
		data += written;
		size -= written;
	}
	return true;
}

void CloseOutputFile(void* file) { CloseHandle(static_cast<HANDLE>(file)); }

#else

// ---- POSIX: every step is relative to the directory before it, none follows a link.

struct Dir {
	int fd = -1;
	Dir() = default;
	Dir(const Dir&) = delete;
	Dir& operator=(const Dir&) = delete;
	~Dir() { if (fd >= 0) close(fd); }
};

Status OpenComponent(Dir& dir, const std::string& name, bool create, bool* created = nullptr)
{
	const auto open = [&]() {
		return openat(dir.fd, name.c_str(), O_RDONLY | O_DIRECTORY | O_NOFOLLOW | O_CLOEXEC);
	};
	int next = open();
	if (next < 0 && errno == ENOENT) {
		if (!create)
			return Status::NotFound;
		if (mkdirat(dir.fd, name.c_str(), 0700) != 0 && errno != EEXIST)
			return Status::IoError;
		if (created != nullptr)
			*created = true;
		next = open();
	}
	if (next < 0) {
		if (errno != ENOTDIR && errno != ELOOP)
			return Status::IoError;
		struct stat info;
		if (fstatat(dir.fd, name.c_str(), &info, AT_SYMLINK_NOFOLLOW) != 0)
			return Status::IoError;
		return S_ISLNK(info.st_mode) ? Status::Link : Status::WrongType;
	}
	close(dir.fd);
	dir.fd = next;
	return Status::Ok;
}

Status OpenWriteDir(const std::string& writeDir, Dir& dir)
{
	// The write directory itself is the user's choice, and may well be reached through a link.
	dir.fd = open(writeDir.c_str(), O_RDONLY | O_DIRECTORY | O_CLOEXEC);
	if (dir.fd >= 0)
		return Status::Ok;
	return errno == ENOENT ? Status::NotFound : Status::IoError;
}

enum class LeafKind { Missing, File, Folder, Link, Error };

LeafKind Leaf(const Dir& dir, const std::string& name, std::uint64_t* size = nullptr)
{
	struct stat info;
	if (fstatat(dir.fd, name.c_str(), &info, AT_SYMLINK_NOFOLLOW) != 0)
		return errno == ENOENT ? LeafKind::Missing : LeafKind::Error;
	if (S_ISLNK(info.st_mode))
		return LeafKind::Link;
	if (S_ISDIR(info.st_mode))
		return LeafKind::Folder;
	if (size != nullptr)
		*size = static_cast<std::uint64_t>(info.st_size);
	return LeafKind::File;
}

template <typename Visit>
Status ForEach(const Dir& dir, Visit visit)
{
	const int copy = openat(dir.fd, ".", O_RDONLY | O_DIRECTORY | O_CLOEXEC);
	if (copy < 0)
		return Status::IoError;
	DIR* stream = fdopendir(copy);
	if (stream == nullptr) {
		close(copy);
		return Status::IoError;
	}
	while (const dirent* entry = readdir(stream)) {
		const std::string name = entry->d_name;
		if (name == "." || name == "..")
			continue;
		std::uint64_t size = 0;
		const LeafKind kind = Leaf(dir, name, &size);
		if (kind == LeafKind::Missing || kind == LeafKind::Error)
			continue;
		if (!visit(name, kind, size))
			break;
	}
	closedir(stream);
	return Status::Ok;
}

void RemoveFile(const Dir& dir, const std::string& name) { unlinkat(dir.fd, name.c_str(), 0); }

Status RemoveFolder(const Dir& dir, const std::string& name)
{
	if (unlinkat(dir.fd, name.c_str(), AT_REMOVEDIR) == 0)
		return Status::Ok;
	return (errno == ENOTEMPTY || errno == EEXIST) ? Status::NotEmpty : Status::IoError;
}

Status RemoveLeafFile(const Dir& dir, const std::string& name)
{
	return unlinkat(dir.fd, name.c_str(), 0) == 0 ? Status::Ok : Status::IoError;
}

Status WriteTemporary(const Dir& dir, const std::string& temporary, const void* data, std::size_t size)
{
	const int file = openat(dir.fd, temporary.c_str(), O_WRONLY | O_CREAT | O_EXCL | O_NOFOLLOW | O_CLOEXEC, 0600);
	if (file < 0)
		return Status::IoError;
	const auto* bytes = static_cast<const std::uint8_t*>(data);
	std::size_t done = 0;
	while (done < size) {
		const ssize_t written = write(file, bytes + done, size - done);
		if (written < 0 && errno == EINTR)
			continue;
		if (written <= 0) {
			close(file);
			return Status::IoError;
		}
		done += static_cast<std::size_t>(written);
	}
	const bool flushed = fsync(file) == 0;
	return (close(file) == 0 && flushed) ? Status::Ok : Status::IoError;
}

Status ReplaceWith(const Dir& dir, const std::string& temporary, const std::string& name)
{
	return renameat(dir.fd, temporary.c_str(), dir.fd, name.c_str()) == 0 ? Status::Ok : Status::IoError;
}

Status ReadWhole(const Dir& dir, const std::string& name, std::vector<std::uint8_t>& out)
{
	// O_NONBLOCK: a planted FIFO must not stall the game; it is refused below.
	const int file = openat(dir.fd, name.c_str(), O_RDONLY | O_NOFOLLOW | O_NONBLOCK | O_CLOEXEC);
	if (file < 0) {
		if (errno == ENOENT)
			return Status::NotFound;
		return errno == ELOOP ? Status::Link : Status::IoError;
	}
	const auto finish = [file](Status status) { close(file); return status; };
	struct stat info;
	if (fstat(file, &info) != 0)
		return finish(Status::IoError);
	if (!S_ISREG(info.st_mode))
		return finish(Status::WrongType);
	if (static_cast<std::uint64_t>(info.st_size) > MAX_FILE_BYTES)
		return finish(Status::FileTooLarge);
	out.resize(static_cast<std::size_t>(info.st_size));
	std::size_t done = 0;
	while (done < out.size()) {
		const ssize_t got = read(file, out.data() + done, out.size() - done);
		if (got < 0 && errno == EINTR)
			continue;
		if (got <= 0)
			return finish(Status::IoError);
		done += static_cast<std::size_t>(got);
	}
	return finish(Status::Ok);
}

void FlushDir(const Dir& dir) { fsync(dir.fd); }

OutputStatus OpenAppend(const Dir& dir, const std::string& name, bool append, int& file, std::uint64_t& size)
{
	// O_NOFOLLOW refuses a link at the name; with O_EXCL a link is a taken name. O_NONBLOCK: a
	// planted FIFO must not stall the game; it is refused below.
	const int flags = O_WRONLY | O_APPEND | O_CREAT | O_NOFOLLOW | O_NONBLOCK | O_CLOEXEC | (append ? 0 : O_EXCL);
	const int fd = openat(dir.fd, name.c_str(), flags, 0600);
	if (fd < 0) {
		switch (errno) {
			case EEXIST: return OutputStatus::Exists;
			case ELOOP:  return OutputStatus::Link;
			case EISDIR: return OutputStatus::WrongType;
			default:     return OutputStatus::IoError;
		}
	}
	struct stat info;
	if (fstat(fd, &info) != 0 || !S_ISREG(info.st_mode)) {
		close(fd);
		return S_ISREG(info.st_mode) ? OutputStatus::IoError : OutputStatus::WrongType;
	}
	file = fd;
	size = static_cast<std::uint64_t>(info.st_size);
	return OutputStatus::Ok;
}

bool AppendAll(int file, const std::uint8_t* data, std::size_t size)
{
	while (size > 0) {
		const ssize_t written = write(file, data, size);
		if (written < 0 && errno == EINTR)
			continue;
		if (written <= 0)
			return false;
		data += written;
		size -= static_cast<std::size_t>(written);
	}
	return true;
}

void CloseOutputFile(int file) { close(file); }

#endif

/// Everything in one game's tree, as it is on disk: links and names outside the rules count too and
/// are never followed. Stops descending when a limit is passed, so a planted tree costs no more
/// than the limits. Temporary files a crash left are removed on the way.
Status Scan(const Dir& dir, std::size_t depth, Usage& usage, const TreeLimits& limits = GAME_DATA_LIMITS)
{
	std::vector<std::string> folders;
	std::vector<std::string> stale;
	const Status listed = ForEach(dir, [&](const std::string& name, LeafKind kind, std::uint64_t size) {
		if (name.rfind(TEMPORARY_PREFIX, 0) == 0 && kind == LeafKind::File) {
			stale.push_back(name);
			return true;
		}
		if (kind == LeafKind::Folder) {
			usage.folders++;
			if (folders.size() <= limits.folders)
				folders.push_back(name);
		} else {
			usage.files++;
			usage.bytes += size;
		}
		return usage.files <= limits.files && usage.folders <= limits.folders && usage.bytes <= limits.bytes;
	});
	if (listed != Status::Ok)
		return listed;
	for (const std::string& name: stale)
		RemoveFile(dir, name);
	if (depth >= MAX_DEPTH + 1 || usage.files > limits.files || usage.folders > limits.folders)
		return Status::Ok;
	for (const std::string& name: folders) {
		Dir inside;
#ifdef _WIN32
		inside.path = dir.path;
#else
		inside.fd = openat(dir.fd, ".", O_RDONLY | O_DIRECTORY | O_CLOEXEC);
		if (inside.fd < 0)
			return Status::IoError;
#endif
		if (OpenComponent(inside, name, false) != Status::Ok)
			continue;
		const Status status = Scan(inside, depth + 1, usage, limits);
		if (status != Status::Ok)
			return status;
	}
	return Status::Ok;
}

Status OpenGameDir(const std::string& writeDir, const std::string& id, bool create, Dir& dir, const char* root = ROOT)
{
	if (!IsValidId(id))
		return Status::NoGameId;
	if (create) {
		// The base itself is the user's or the launcher's choice, not the game's.
		std::error_code ignored;
#ifdef _WIN32
		std::filesystem::create_directories(std::filesystem::path(nowide::widen(writeDir)), ignored);
#else
		std::filesystem::create_directories(std::filesystem::path(writeDir), ignored);
#endif
	}
	Status status = OpenWriteDir(writeDir, dir);
	if (status == Status::Ok)
		status = OpenComponent(dir, root, create);
	if (status == Status::Ok)
		status = OpenComponent(dir, id, create);
	return status;
}

/// Down the folders of `components`, all but the last `leaves` of them.
Status Walk(Dir& dir, const std::vector<std::string>& components, std::size_t leaves, bool create)
{
	for (std::size_t i = 0; i + leaves < components.size(); ++i) {
		const Status status = OpenComponent(dir, components[i], create);
		if (status != Status::Ok)
			return status;
	}
	return Status::Ok;
}

} // namespace

const char* Describe(Status status)
{
	switch (status) {
		case Status::Ok:             return "ok";
		case Status::InvalidPath:    return "the path breaks the game data rules";
		case Status::NoGameId:       return "the archive declares no valid gameDataId";
		case Status::ReadOnly:       return "a replay cannot change game data";
		case Status::NotFound:       return "not found";
		case Status::WrongType:      return "a file where a folder is needed, or the reverse";
		case Status::NotEmpty:       return "the folder is not empty";
		case Status::Link:           return "a symlink or reparse point is on the way";
		case Status::FileTooLarge:   return "the file is too large";
		case Status::TooManyFiles:   return "too many files";
		case Status::TooManyFolders: return "too many folders";
		case Status::TooManyBytes:   return "too many bytes";
		case Status::RateLimited:    return "too many changes in a short time";
		case Status::IoError:        return "the disk refused";
	}
	return "unknown";
}

bool IsValidId(std::string_view id) { return IsValidStem(id); }
bool IsValidFolderName(std::string_view name) { return IsValidStem(name); }

bool IsValidFileName(std::string_view name) { return IsValidFileNameIn(name, EXTENSIONS); }
bool IsValidOutputFileName(std::string_view name) { return IsValidFileNameIn(name, OUTPUT_EXTENSIONS); }

bool ParseFilePath(std::string_view path, std::vector<std::string>& components)
{
	return ParseFilePathWith(path, components, [](std::string_view name) { return IsValidFileName(name); });
}

bool ParseOutputFilePath(std::string_view path, std::vector<std::string>& components)
{
	return ParseFilePathWith(path, components, [](std::string_view name) { return IsValidOutputFileName(name); });
}

bool ParseFolderPath(std::string_view path, std::vector<std::string>& components)
{
	components.clear();
	if (path.empty())
		return true;
	if (!Split(path, components))
		return false;
	return std::all_of(components.begin(), components.end(), [](const std::string& c) { return IsValidFolderName(c); });
}

bool ChangeBudget::Allow(std::uint64_t now)
{
	if (!started) {
		started = true;
		lastRefill = now;
	}
	const std::uint64_t gained = (now - lastRefill) * CHANGES_PER_SECOND / 1000;
	if (gained > 0) {
		tokens = static_cast<std::uint32_t>(std::min<std::uint64_t>(CHANGE_BURST, tokens + gained));
		lastRefill = now;
	}
	if (tokens == 0)
		return false;
	tokens--;
	return true;
}

Status Sandbox::Write(std::string_view path, const void* data, std::size_t size) const
{
	std::vector<std::string> components;
	if (!ParseFilePath(path, components))
		return Status::InvalidPath;
	if (!IsValidId(id))
		return Status::NoGameId;
	if (readOnly)
		return Status::ReadOnly;
	if (size > MAX_FILE_BYTES)
		return Status::FileTooLarge;
	if (data == nullptr && size > 0)
		return Status::IoError;

	Dir dir;
	Status status = OpenGameDir(writeDir, id, true, dir);
	if (status != Status::Ok)
		return status;

	Usage usage;
	if ((status = Scan(dir, 0, usage)) != Status::Ok)
		return status;

	// What the path needs that is not there yet, without creating anything.
	std::size_t newFolders = 0;
	std::uint64_t replaced = 0;
	bool fileExists = false;
	{
		Dir probe;
		if ((status = OpenGameDir(writeDir, id, false, probe)) != Status::Ok)
			return status;
		std::size_t i = 0;
		for (; i + 1 < components.size(); ++i) {
			status = OpenComponent(probe, components[i], false);
			if (status == Status::NotFound)
				break;
			if (status != Status::Ok)
				return status;
		}
		newFolders = components.size() - 1 - i;
		if (newFolders == 0) {
			switch (Leaf(probe, components.back(), &replaced)) {
				case LeafKind::Missing: break;
				case LeafKind::File: fileExists = true; break;
				case LeafKind::Folder: return Status::WrongType;
				case LeafKind::Link: return Status::Link;
				case LeafKind::Error: return Status::IoError;
			}
		}
	}
	if (usage.folders + newFolders > MAX_FOLDERS)
		return Status::TooManyFolders;
	if (!fileExists && usage.files + 1 > MAX_FILES)
		return Status::TooManyFiles;
	if (usage.bytes - std::min(usage.bytes, replaced) + size > MAX_TOTAL_BYTES)
		return Status::TooManyBytes;

	if ((status = Walk(dir, components, 1, true)) != Status::Ok)
		return status;

	const std::string temporary = std::string(TEMPORARY_PREFIX) + components.back();
	RemoveFile(dir, temporary);
	if ((status = WriteTemporary(dir, temporary, data, size)) != Status::Ok) {
		RemoveFile(dir, temporary);
		return status;
	}
	// The rename replaces a link itself rather than writing through it, but none should be here:
	// the check above and this one leave a planted link in place and report it.
	const LeafKind target = Leaf(dir, components.back());
	if (target == LeafKind::Link || target == LeafKind::Folder) {
		RemoveFile(dir, temporary);
		return target == LeafKind::Link ? Status::Link : Status::WrongType;
	}
	if ((status = ReplaceWith(dir, temporary, components.back())) != Status::Ok) {
		RemoveFile(dir, temporary);
		return status;
	}
	FlushDir(dir);
	return Status::Ok;
}

Status Sandbox::Read(std::string_view path, std::vector<std::uint8_t>& data) const
{
	data.clear();
	std::vector<std::string> components;
	if (!ParseFilePath(path, components))
		return Status::InvalidPath;
	Dir dir;
	Status status = OpenGameDir(writeDir, id, false, dir);
	if (status != Status::Ok)
		return status;
	if ((status = Walk(dir, components, 1, false)) != Status::Ok)
		return status;
	return ReadWhole(dir, components.back(), data);
}

Status Sandbox::Delete(std::string_view path) const
{
	std::vector<std::string> components;
	if (!ParseFilePath(path, components) && (!ParseFolderPath(path, components) || components.empty()))
		return Status::InvalidPath;
	if (!IsValidId(id))
		return Status::NoGameId;
	if (readOnly)
		return Status::ReadOnly;
	Dir dir;
	Status status = OpenGameDir(writeDir, id, false, dir);
	if (status != Status::Ok)
		return status;
	if ((status = Walk(dir, components, 1, false)) != Status::Ok)
		return status;
	switch (Leaf(dir, components.back())) {
		case LeafKind::Missing: return Status::NotFound;
		case LeafKind::Link: return Status::Link;
		case LeafKind::Error: return Status::IoError;
		case LeafKind::File: break;
		case LeafKind::Folder: return RemoveFolder(dir, components.back());
	}
	if ((status = RemoveLeafFile(dir, components.back())) != Status::Ok)
		return status;
	FlushDir(dir);
	return Status::Ok;
}

Status Sandbox::List(std::string_view folder, std::vector<Entry>& entries) const
{
	entries.clear();
	std::vector<std::string> components;
	if (!ParseFolderPath(folder, components))
		return Status::InvalidPath;
	Dir dir;
	Status status = OpenGameDir(writeDir, id, false, dir);
	if (status != Status::Ok)
		return status;
	if ((status = Walk(dir, components, 0, false)) != Status::Ok)
		return status;
	status = ForEach(dir, [&](const std::string& name, LeafKind kind, std::uint64_t) {
		if (kind == LeafKind::File && IsValidFileName(name))
			entries.push_back({name, false});
		else if (kind == LeafKind::Folder && IsValidFolderName(name))
			entries.push_back({name, true});
		return entries.size() < MAX_LIST;
	});
	std::sort(entries.begin(), entries.end(), [](const Entry& a, const Entry& b) { return a.name < b.name; });
	return status;
}

} // namespace GameData


namespace GameData {

namespace {

OutputStatus FromStatus(Status status)
{
	switch (status) {
		case Status::Ok:          return OutputStatus::Ok;
		case Status::InvalidPath: return OutputStatus::InvalidPath;
		case Status::NoGameId:    return OutputStatus::NoGameId;
		case Status::Link:        return OutputStatus::Link;
		case Status::WrongType:   return OutputStatus::WrongType;
		default:                  return OutputStatus::IoError;
	}
}

} // namespace

const char* Describe(OutputStatus status)
{
	switch (status) {
		case OutputStatus::Ok:             return "ok";
		case OutputStatus::InvalidPath:    return "the path breaks the game output rules";
		case OutputStatus::NoGameId:       return "the archive declares no valid gameDataId";
		case OutputStatus::Exists:         return "a new file was asked for and the name is taken";
		case OutputStatus::NoStream:       return "no open output stream has this handle";
		case OutputStatus::TooManyOpen:    return "too many output streams are open";
		case OutputStatus::Link:           return "a symlink or reparse point is on the way";
		case OutputStatus::WrongType:      return "not a regular file, or a file where a folder is needed";
		case OutputStatus::FileTooLarge:   return "the file would pass its size limit";
		case OutputStatus::TooManyFiles:   return "too many output files";
		case OutputStatus::TooManyFolders: return "too many output folders";
		case OutputStatus::TooManyBytes:   return "the game's output would pass its total limit";
		case OutputStatus::RateLimited:    return "too much output in a short time";
		case OutputStatus::IoError:        return "the disk refused";
	}
	return "unknown";
}

bool RateBudget::Allow(std::uint64_t amount, std::uint64_t now)
{
	if (!started) {
		started = true;
		lastRefill = now;
	}
	if (now > lastRefill && perSecond > 0) {
		const std::uint64_t elapsed = now - lastRefill;
		// A long rest fills the bucket; the product below cannot overflow under it.
		if (elapsed >= 1000000 || (elapsed * perSecond) / 1000 >= burst) {
			tokens = burst;
			lastRefill = now;
		} else {
			const std::uint64_t gained = elapsed * perSecond / 1000;
			if (gained > 0) {
				tokens = std::min(burst, tokens + gained);
				// Keep the remainder of a token, so many small calls still earn the full rate.
				lastRefill += gained * 1000 / perSecond;
			}
		}
	}
	if (amount > tokens)
		return false;
	tokens -= amount;
	return true;
}

Outputs::Outputs(OutputLimits limits)
	: limits(limits)
	, opens(OPEN_BURST, OPENS_PER_SECOND)
	, bytes(limits.bytesPerSecond * 4, limits.bytesPerSecond)
{}

Outputs::~Outputs() { CloseAll(); }

void Outputs::SetLimits(const OutputLimits& next)
{
	if (next.bytesPerSecond != limits.bytesPerSecond)
		bytes = RateBudget(next.bytesPerSecond * 4, next.bytesPerSecond);
	limits = next;
}

void Outputs::CloseStream(Stream& stream)
{
	CloseOutputFile(stream.file);
}

OutputStatus Outputs::Open(const std::string& base, const std::string& id, std::string_view path, bool append, std::uint64_t now, std::uint32_t& handle)
{
	handle = 0;
	std::vector<std::string> components;
	if (!ParseOutputFilePath(path, components))
		return OutputStatus::InvalidPath;
	if (!IsValidId(id))
		return OutputStatus::NoGameId;
	if (streams.size() >= MAX_OPEN_OUTPUTS)
		return OutputStatus::TooManyOpen;
	if (!opens.Allow(1, now))
		return OutputStatus::RateLimited;

	Dir dir;
	Status status = OpenGameDir(base, id, true, dir, OUTPUT_ROOT);
	if (status != Status::Ok)
		return FromStatus(status);

	// What is on disk, read from the directory only while none of this id's streams is open.
	auto known = usage.find(id);
	if (known == usage.end()) {
		GameData::Usage scanned;
		const TreeLimits bounds = {limits.maxFiles, limits.maxFolders, limits.maxTotalBytes};
		if ((status = Scan(dir, 0, scanned, bounds)) != Status::Ok)
			return FromStatus(status);
		known = usage.emplace(id, Usage{scanned.bytes, scanned.files, scanned.folders}).first;
	}
	Usage& current = known->second;
	const auto forget = [&]() {
		if (std::none_of(streams.begin(), streams.end(), [&](const auto& open) { return open.second.id == id; }))
			usage.erase(id);
	};

	// What the path needs that is not there yet, without creating anything.
	std::size_t newFolders = 0;
	bool fileExists = false;
	{
		Dir probe;
		if ((status = OpenGameDir(base, id, false, probe, OUTPUT_ROOT)) != Status::Ok) {
			forget();
			return FromStatus(status);
		}
		std::size_t i = 0;
		for (; i + 1 < components.size(); ++i) {
			status = OpenComponent(probe, components[i], false);
			if (status == Status::NotFound)
				break;
			if (status != Status::Ok) {
				forget();
				return FromStatus(status);
			}
		}
		newFolders = components.size() - 1 - i;
		if (newFolders == 0) {
			switch (Leaf(probe, components.back())) {
				case LeafKind::Missing: break;
				case LeafKind::File: fileExists = true; break;
				case LeafKind::Folder: forget(); return OutputStatus::WrongType;
				case LeafKind::Link: forget(); return append ? OutputStatus::Link : OutputStatus::Exists;
				case LeafKind::Error: forget(); return OutputStatus::IoError;
			}
		}
	}
	OutputStatus refusal = OutputStatus::Ok;
	if (fileExists && !append)
		refusal = OutputStatus::Exists;
	else if (current.folders + newFolders > limits.maxFolders)
		refusal = OutputStatus::TooManyFolders;
	else if (!fileExists && current.files + 1 > limits.maxFiles)
		refusal = OutputStatus::TooManyFiles;
	else if (current.bytes >= limits.maxTotalBytes)
		refusal = OutputStatus::TooManyBytes;
	if (refusal != OutputStatus::Ok) {
		forget();
		return refusal;
	}

	if ((status = Walk(dir, components, 1, true)) != Status::Ok) {
		forget();
		return FromStatus(status);
	}
	Stream stream;
	stream.id = id;
	const OutputStatus opened = OpenAppend(dir, components.back(), append, stream.file, stream.size);
	if (opened != OutputStatus::Ok) {
		forget();
		return opened;
	}
	FlushDir(dir);
	current.folders += newFolders;
	if (!fileExists)
		current.files++;
	handle = nextHandle++;
	streams.emplace(handle, std::move(stream));
	return OutputStatus::Ok;
}

OutputStatus Outputs::Write(std::uint32_t handle, const void* data, std::size_t size, std::uint64_t now)
{
	const auto found = streams.find(handle);
	if (found == streams.end())
		return OutputStatus::NoStream;
	if (size == 0)
		return OutputStatus::Ok;
	if (data == nullptr)
		return OutputStatus::IoError;
	Stream& stream = found->second;
	Usage& current = usage[stream.id];
	if (size > limits.maxFileBytes || stream.size + size > limits.maxFileBytes)
		return OutputStatus::FileTooLarge;
	if (size > limits.maxTotalBytes || current.bytes + size > limits.maxTotalBytes)
		return OutputStatus::TooManyBytes;
	if (!bytes.Allow(size, now))
		return OutputStatus::RateLimited;
	const bool written = AppendAll(stream.file, static_cast<const std::uint8_t*>(data), size);
	// A failed write may have put part of the data down: count it all, never less than the disk has.
	stream.size += size;
	current.bytes += size;
	return written ? OutputStatus::Ok : OutputStatus::IoError;
}

OutputStatus Outputs::Close(std::uint32_t handle)
{
	const auto found = streams.find(handle);
	if (found == streams.end())
		return OutputStatus::NoStream;
	const std::string id = found->second.id;
	CloseStream(found->second);
	streams.erase(found);
	if (std::none_of(streams.begin(), streams.end(), [&](const auto& open) { return open.second.id == id; }))
		usage.erase(id);
	return OutputStatus::Ok;
}

std::size_t Outputs::CloseAll()
{
	const std::size_t closed = streams.size();
	for (auto& [handle, stream]: streams)
		CloseStream(stream);
	streams.clear();
	usage.clear();
	return closed;
}

Outputs& ProcessOutputs()
{
	static Outputs outputs;
	return outputs;
}

} // namespace GameData

std::size_t CloseGameOutputStreams() { return GameData::ProcessOutputs().CloseAll(); }
