/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <cstddef>
#include <cstdint>
#include <map>
#include <string>
#include <string_view>
#include <vector>

/**
 * A per-game data directory a module may read and write, and nothing else: `<write dir>/game-data/
 * <id>/`. The module is treated as hostile: it may come from a downloaded archive. The id comes from
 * the archive's `gameDataId` modinfo field, never from the module. See
 * rts/wasm/docs/game_data_sandbox.md.
 *
 * Paths are checked lexically before the disk is touched and a bad one is refused, never repaired.
 * Every directory is opened one component at a time without following links (openat with O_NOFOLLOW
 * on POSIX; on Windows a handle with no delete sharing held on each component, reparse points refused),
 * so a link planted anywhere under the write directory cannot redirect anything.
 */
namespace GameData {
	/// The directory under the write directory. Not `gamedata`: that is a path games' archives use,
	/// and raw-first file lookups search the data directories for it.
	inline constexpr const char* ROOT = "game-data";

	inline constexpr std::size_t MAX_COMPONENT_LENGTH = 64;
	inline constexpr std::size_t MAX_PATH_LENGTH = 200;
	/// Components of a path, the file included.
	inline constexpr std::size_t MAX_DEPTH = 8;
	inline constexpr std::size_t MAX_FILE_BYTES = 8 * 1024 * 1024;
	/// The whole of one game's tree. Replacing a file counts only what it adds.
	inline constexpr std::size_t MAX_FILES = 256;
	inline constexpr std::size_t MAX_FOLDERS = 64;
	inline constexpr std::uint64_t MAX_TOTAL_BYTES = 64 * 1024 * 1024;
	/// Names a listing returns at most.
	inline constexpr std::size_t MAX_LIST = 256;
	/// Changes (writes and deletes): a burst, then this many a second.
	inline constexpr std::uint32_t CHANGE_BURST = 32;
	inline constexpr std::uint32_t CHANGES_PER_SECOND = 2;

	enum class Status {
		Ok,
		InvalidPath,   ///< the path's text breaks a rule; nothing on disk was looked at
		NoGameId,      ///< the archive declares no valid id
		ReadOnly,      ///< a replay is playing
		NotFound,
		WrongType,     ///< a file where a folder is needed, or the reverse
		NotEmpty,      ///< a folder with something in it; there is no recursive delete
		Link,          ///< a symlink or reparse point on the way
		FileTooLarge,
		TooManyFiles,
		TooManyFolders,
		TooManyBytes,
		RateLimited,
		IoError,
	};
	/// Words for a status, with no paths in them.
	const char* Describe(Status status);

	/// An id: lowercase letters, digits, `_` and `-`, a letter or digit first and last, not a
	/// Windows device name.
	bool IsValidId(std::string_view id);
	/// The same rules as an id, for one folder of a path.
	bool IsValidFolderName(std::string_view name);
	/// A folder name's rules for the stem, one `.`, and an extension on the data-only list.
	bool IsValidFileName(std::string_view name);

	/// Splits `a/b/c.json` into components if the whole path is valid. A folder path may be empty
	/// (the game's own directory). Never repairs: any bad part refuses the whole path.
	bool ParseFilePath(std::string_view path, std::vector<std::string>& components);
	bool ParseFolderPath(std::string_view path, std::vector<std::string>& components);

	/// Limits how often a game may change files on disk (each change flushes it).
	class ChangeBudget {
	public:
		bool Allow(std::uint64_t nowMilliseconds);
	private:
		std::uint64_t lastRefill = 0;
		std::uint32_t tokens = CHANGE_BURST;
		bool started = false;
	};

	struct Entry {
		std::string name;
		bool isDirectory;
	};

	// ---- Output files: append-only streams a module writes and never reads back, such as recordings
	// and exports, in `<write dir>/game-output/<id>/`. The same path rules as game data, apart from
	// the extension list; no read, list or delete call exists. See rts/wasm/docs/game_output_files.md.

	inline constexpr const char* OUTPUT_ROOT = "game-output";
	/// Streams open at once, for all modules together.
	inline constexpr std::size_t MAX_OPEN_OUTPUTS = 16;
	/// Opens: a burst, then this many a second.
	inline constexpr std::uint32_t OPEN_BURST = 16;
	inline constexpr std::uint32_t OPENS_PER_SECOND = 2;

	/// The bounds the host sets from the player's settings (VFS.cpp); a module cannot change them.
	struct OutputLimits {
		std::uint64_t maxFileBytes = 1024ull * 1024 * 1024;
		/// Everything in the game's output directory, from earlier runs too.
		std::uint64_t maxTotalBytes = 8ull * 1024 * 1024 * 1024;
		std::size_t maxFiles = 4096;
		std::size_t maxFolders = 64;
		/// Bytes written: a burst of four seconds' worth, then this many a second.
		std::uint64_t bytesPerSecond = 32ull * 1024 * 1024;
	};

	enum class OutputStatus {
		Ok,
		InvalidPath,   ///< the path's text breaks a rule; nothing on disk was looked at
		NoGameId,
		Exists,        ///< a new file was asked for and the name is taken
		NoStream,      ///< no open stream has this handle
		TooManyOpen,
		Link,
		WrongType,
		FileTooLarge,
		TooManyFiles,
		TooManyFolders,
		TooManyBytes,
		RateLimited,
		IoError,
	};
	const char* Describe(OutputStatus status);

	/// A file name with an output extension: `jsonl json txt csv tsv log dat bin`.
	bool IsValidOutputFileName(std::string_view name);
	bool ParseOutputFilePath(std::string_view path, std::vector<std::string>& components);

	/// A token bucket: `burst` at most, `perSecond` back each second.
	class RateBudget {
	public:
		RateBudget(std::uint64_t burst, std::uint64_t perSecond) : burst(burst), perSecond(perSecond), tokens(burst) {}
		bool Allow(std::uint64_t amount, std::uint64_t nowMilliseconds);
	private:
		std::uint64_t burst;
		std::uint64_t perSecond;
		std::uint64_t tokens;
		std::uint64_t lastRefill = 0;
		bool started = false;
	};

	/// The open output streams of one engine process. Writes are allowed during replay playback:
	/// they change nothing a game reads. Streams still open when the game ends are closed by
	/// `CloseAll`.
	class Outputs {
	public:
		explicit Outputs(OutputLimits limits = {});
		~Outputs();
		Outputs(const Outputs&) = delete;
		Outputs& operator=(const Outputs&) = delete;

		/// Opens `path` under `<base>/game-output/<id>/` for appending, making its folders. `append`
		/// false asks for a new file and refuses a name already taken (`Exists`); true continues an
		/// existing file or makes it. A handle is never 0 and never reused.
		OutputStatus Open(const std::string& base, const std::string& id, std::string_view path, bool append, std::uint64_t nowMilliseconds, std::uint32_t& handle);
		/// Appends all of `data`, or nothing when a bound refuses it.
		OutputStatus Write(std::uint32_t handle, const void* data, std::size_t size, std::uint64_t nowMilliseconds);
		OutputStatus Close(std::uint32_t handle);
		/// Closes every stream; returns how many were open.
		std::size_t CloseAll();
		std::size_t OpenCount() const { return streams.size(); }
		/// New bounds apply to the next call; what is already written stays.
		void SetLimits(const OutputLimits& next);

	private:
		struct Stream {
			std::string id;
			std::uint64_t size = 0;
#ifdef _WIN32
			void* file = nullptr;
#else
			int file = -1;
#endif
		};
		struct Usage {
			std::uint64_t bytes = 0;
			std::size_t files = 0;
			std::size_t folders = 0;
		};
		void CloseStream(Stream& stream);

		OutputLimits limits;
		RateBudget opens;
		RateBudget bytes;
		std::map<std::uint32_t, Stream> streams;
		/// Per id while any of its streams is open: what is on disk, kept current by writes, so an
		/// open file's size is never read back from the directory.
		std::map<std::string, Usage> usage;
		std::uint32_t nextHandle = 1;
	};

	/// The engine's one set of output streams.
	Outputs& ProcessOutputs();

	class Sandbox {
	public:
		/// `id` empty or invalid refuses everything. `readOnly` refuses writes and deletes.
		Sandbox(std::string writeDir, std::string id, bool readOnly)
			: writeDir(std::move(writeDir)), id(std::move(id)), readOnly(readOnly) {}

		/// Creates the folders it needs; written to a temporary file, flushed and renamed over the
		/// old file, so a reader or a crash sees the old file or the new one.
		Status Write(std::string_view path, const void* data, std::size_t size) const;
		Status Read(std::string_view path, std::vector<std::uint8_t>& data) const;
		/// A file, or an empty folder.
		Status Delete(std::string_view path) const;
		/// Files and folders of one folder, sorted, without links or names outside the rules; at
		/// most `MAX_LIST`.
		Status List(std::string_view folder, std::vector<Entry>& entries) const;

	private:
		std::string writeDir;
		std::string id;
		bool readOnly;
	};
}
