/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include <cstdint>
#include <filesystem>
#include <fstream>
#include <random>
#include <sstream>
#include <string>
#include <vector>

#include <catch_amalgamated.hpp>

#ifndef _WIN32
	#include <sys/stat.h>
#endif

#include "System/FileSystem/GameDataFiles.h"

namespace fs = std::filesystem;
using namespace GameData;

namespace {

/// A write directory of its own, with a canary file beside it that nothing may reach.
struct Disk {
	fs::path top;
	fs::path write;
	fs::path game;
	fs::path outside;
	Disk() {
		std::random_device random;
		top = fs::temp_directory_path() / ("recoil-game-data-" + std::to_string(random()) + std::to_string(random()));
		write = top / "write";
		game = write / ROOT / "testgame";
		outside = top / "outside";
		fs::create_directories(write);
		fs::create_directories(outside);
		std::ofstream(outside / "canary.txt") << "canary";
	}
	~Disk() { std::error_code ignored; fs::remove_all(top, ignored); }
	Sandbox sandbox(bool readOnly = false) const { return Sandbox(write.string(), "testgame", readOnly); }
	Status put(std::string_view path, std::string_view text = "x") const { return sandbox().Write(path, text.data(), text.size()); }
	std::size_t writeDirEntries() const {
		std::size_t n = 0;
		for (const auto& entry: fs::recursive_directory_iterator(write)) { (void)entry; ++n; }
		return n;
	}
	bool canaryIntact() const {
		std::ifstream file(outside / "canary.txt");
		std::stringstream text;
		text << file.rdbuf();
		std::size_t n = 0;
		for (const auto& entry: fs::directory_iterator(outside)) { (void)entry; ++n; }
		return text.str() == "canary" && n == 1;
	}
};

std::string ReadText(const fs::path& path)
{
	std::ifstream file(path, std::ios::binary);
	std::stringstream text;
	text << file.rdbuf();
	return text.str();
}

bool MakeLink(const fs::path& target, const fs::path& link, bool directory)
{
	std::error_code error;
	if (directory)
		fs::create_directory_symlink(target, link, error);
	else
		fs::create_symlink(target, link, error);
	return !error;
}

/// Every refused text path: lexical, before anything on disk is looked at.
const std::vector<std::string_view> HOSTILE_FILE_PATHS = {
	"", ".", "..", "/", "//", "./", "../",
	// Traversal and absolute.
	"../x.txt", "../../x.txt", "a/../b.txt", "a/../../b.txt", "a/../../../../outside/canary.txt", "/etc/passwd", "/x.txt",
	"C:/x.txt", "C:x.txt", "c:", "\\x.txt", "\\\\server\\share\\x.txt", "~/x.txt", "~x.txt",
	// Encoded and alternative separators.
	"a%2fb.txt", "a%2Fb.txt", "%2e%2e/x.txt", "..%2fx.txt", "%2e%2e%2fx.txt", "a\\b.txt", "..\\x.txt", "a\xef\xbc\x8f" "b.txt", "a\xe2\x88\x95" "b.txt",
	// Empty, dot and dotted components.
	"a//b.txt", "./a.txt", "a/./b.txt", "a/.", "a/..", "a/", "a/b/", "/a.txt", ".hidden.txt", "a/.hidden.txt", ".a/b.txt", ".tmp-x.txt",
	// Case: lowercase only, so a case-insensitive disk sees the same rules.
	"A.txt", "a.TXT", "Saves/x.txt", "CON.txt", "Con.txt",
	// Characters, trailing dots and spaces, streams, wildcards, short names.
	"a b.txt", " a.txt", "a.txt ", "a.txt.", "a. ", "a.", "a ", "a/b /c.txt", "a./b.txt", "a:b.txt", "a.txt:stream", "a.txt::$data", "*.txt", "a?.txt",
	"progra~1/x.txt", "a\tb.txt", "a\nb.txt", "a\rb.txt", "\xc5\xa1.txt", "a|b.txt", "a<b.txt", "a>b.txt", "a\"b.txt", "-a.txt", "_a.txt", "a-.txt",
	std::string_view("a\0.txt", 6), std::string_view("a.txt\0", 6), std::string_view("a.txt\0/../../x", 14),
	// Reserved device names, in any component, with or without an extension.
	"con.txt", "prn.txt", "aux.txt", "nul.txt", "com1.txt", "com9.txt", "lpt1.txt", "lpt0.txt", "con/x.txt", "a/con/b.txt", "a/nul/b.txt", "a/com1/b.txt", "con", "nul",
	// Disallowed and missing extensions.
	"x", "x.lua", "x.so", "x.dll", "x.exe", "x.sdd", "x.sdz", "x.sd7", "x.cfg", "x.bat", "x.sh", "x.ps1", "x.dylib", "x.py", "x.wasm", "x.conf",
	"x.txt.lua", "x.lua.txt", "x.tar.bin", "a.b/c.txt", "a/x.txt/y.txt", "x.", ".txt",
};

}

TEST_CASE("GameDataIds")
{
	for (const char* good: {"game", "some-game_2", "a"})
		CHECK(IsValidId(good));
	for (const char* bad: {"", ".", "..", "a/b", "a\\b", "../x", "Ship", "a b", "a.b", "-a", "a-", "con", "nul", "com1", "lpt9", "a%2fb"})
		CHECK_FALSE(IsValidId(bad));
	CHECK(IsValidId(std::string(MAX_COMPONENT_LENGTH, 'a')));
	CHECK_FALSE(IsValidId(std::string(MAX_COMPONENT_LENGTH + 1, 'a')));
}

TEST_CASE("GameDataGoodPaths")
{
	const Disk disk;
	for (const char* good: {"campaign.toml", "saves/campaign.json", "profiles/a/settings.toml", "a/b/c/d/e/f/g/h.txt", "x.bin", "x.png", "x.jpg", "x.csv", "x.dat", "console.txt", "com10.txt", "nullify.txt", "con1/x.txt"}) {
		INFO(good);
		CHECK(disk.put(good) == Status::Ok);
	}
}

TEST_CASE("GameDataRefusesEveryHostilePathAndTouchesNothing")
{
	const Disk disk;
	const Sandbox sandbox = disk.sandbox();
	std::vector<std::uint8_t> bytes;
	std::vector<Entry> entries;
	for (const std::string_view path: HOSTILE_FILE_PATHS) {
		INFO(std::string(path));
		CHECK(sandbox.Write(path, "x", 1) == Status::InvalidPath);
		CHECK(sandbox.Read(path, bytes) == Status::InvalidPath);
	}
	for (const std::string_view path: {std::string_view("../x"), std::string_view("a/../b"), std::string_view("a//b"), std::string_view("./a"), std::string_view("a/."), std::string_view("a/con/b"), std::string_view("/"), std::string_view("a/"), std::string_view("A"), std::string_view("a.b")}) {
		INFO(std::string(path));
		CHECK(sandbox.List(path, entries) == Status::InvalidPath);
		CHECK(sandbox.Delete(path) == Status::InvalidPath);
	}
	// Nothing was created, not even the root.
	CHECK(disk.writeDirEntries() == 0);
	CHECK(disk.canaryIntact());
}

TEST_CASE("GameDataBoundsThePath")
{
	const Disk disk;
	std::string deep;
	for (std::size_t i = 1; i < MAX_DEPTH; ++i)
		deep += "a/";
	CHECK(disk.put(deep + "x.txt") == Status::Ok);
	CHECK(disk.put(deep + "a/x.txt") == Status::InvalidPath);

	const std::string longest(MAX_COMPONENT_LENGTH - 4, 'a');
	CHECK(disk.put(longest + ".txt") == Status::Ok);
	CHECK(disk.put(longest + "a.txt") == Status::InvalidPath);
	CHECK(disk.put(std::string(MAX_COMPONENT_LENGTH + 1, 'a') + "/x.txt") == Status::InvalidPath);

	std::string longPath;
	while (longPath.size() < MAX_PATH_LENGTH)
		longPath += std::string(60, 'b') + "/";
	CHECK(disk.put(longPath + "x.txt") == Status::InvalidPath);
}

TEST_CASE("GameDataNeedsAValidId")
{
	const Disk disk;
	for (const char* id: {"", "..", "a/b", "../outside", "Upper", "con"}) {
		INFO(id);
		const Sandbox sandbox(disk.write.string(), id, false);
		std::vector<std::uint8_t> bytes;
		std::vector<Entry> entries;
		CHECK(sandbox.Write("x.txt", "x", 1) == Status::NoGameId);
		CHECK(sandbox.Read("x.txt", bytes) == Status::NoGameId);
		CHECK(sandbox.Delete("x.txt") == Status::NoGameId);
		CHECK(sandbox.List("", entries) == Status::NoGameId);
	}
	CHECK(disk.writeDirEntries() == 0);
}

TEST_CASE("GameDataRoundTrip")
{
	const Disk disk;
	const Sandbox sandbox = disk.sandbox();
	std::vector<std::uint8_t> bytes;

	SECTION("files land in the game's own directory, private, replaced whole") {
		CHECK(disk.put("saves/one.json", "first") == Status::Ok);
		CHECK(disk.put("saves/one.json", "second!") == Status::Ok);
		CHECK(ReadText(disk.game / "saves" / "one.json") == "second!");
		CHECK(sandbox.Read("saves/one.json", bytes) == Status::Ok);
		CHECK(std::string(bytes.begin(), bytes.end()) == "second!");
		// No temporary file stays.
		std::size_t n = 0;
		for (const auto& entry: fs::directory_iterator(disk.game / "saves")) { (void)entry; ++n; }
		CHECK(n == 1);
#ifndef _WIN32
		const auto others = fs::perms::group_all | fs::perms::others_all;
		CHECK((fs::status(disk.game / "saves" / "one.json").permissions() & others) == fs::perms::none);
		CHECK((fs::status(disk.game / "saves").permissions() & others) == fs::perms::none);
		CHECK((fs::status(disk.game).permissions() & others) == fs::perms::none);
#endif
	}
	SECTION("a base directory that does not exist yet") {
		const Sandbox fresh((disk.top / "not" / "yet").string(), "testgame", false);
		std::vector<Entry> entries;
		CHECK(fresh.Read("x.txt", bytes) == Status::NotFound);
		CHECK(fresh.List("", entries) == Status::NotFound);
		CHECK(fresh.Delete("x.txt") == Status::NotFound);
		CHECK_FALSE(fs::exists(disk.top / "not"));
		CHECK(fresh.Write("x.txt", "x", 1) == Status::Ok);
		CHECK(fresh.Read("x.txt", bytes) == Status::Ok);
	}
	SECTION("what is missing") {
		CHECK(sandbox.Read("none.txt", bytes) == Status::NotFound);
		CHECK(disk.put("a/x.txt") == Status::Ok);
		CHECK(sandbox.Read("a/none.txt", bytes) == Status::NotFound);
		CHECK(sandbox.Read("b/none.txt", bytes) == Status::NotFound);
		CHECK(sandbox.Delete("none.txt") == Status::NotFound);
	}
	SECTION("an empty file is a file") {
		CHECK(sandbox.Write("empty.txt", nullptr, 0) == Status::Ok);
		CHECK(sandbox.Read("empty.txt", bytes) == Status::Ok);
		CHECK(bytes.empty());
	}
	SECTION("a temporary file left by a crash neither counts nor stays") {
		fs::create_directories(disk.game);
		std::ofstream(disk.game / ".tmp-one.txt") << "half a sa";
		CHECK(disk.put("one.txt", "whole") == Status::Ok);
		CHECK(ReadText(disk.game / "one.txt") == "whole");
		CHECK_FALSE(fs::exists(disk.game / ".tmp-one.txt"));
	}
	SECTION("games do not see each other's files") {
		CHECK(disk.put("mine.txt") == Status::Ok);
		const Sandbox other(disk.write.string(), "othergame", false);
		CHECK(other.Read("mine.txt", bytes) == Status::NotFound);
	}
}

TEST_CASE("GameDataWrongTypes")
{
	const Disk disk;
	const Sandbox sandbox = disk.sandbox();
	std::vector<std::uint8_t> bytes;
	std::vector<Entry> entries;
	CHECK(disk.put("folder/file.txt") == Status::Ok);
	CHECK(disk.put("plain.txt") == Status::Ok);
	// A file where a folder is expected: `plain.txt` cannot be named as a folder at all, so plant one.
	std::ofstream(disk.game / "afile") << "x";
	CHECK(disk.put("afile/x.txt") == Status::WrongType);
	CHECK(sandbox.Read("afile/x.txt", bytes) == Status::WrongType);
	CHECK(sandbox.List("afile", entries) == Status::WrongType);
	// A folder where a file is expected.
	fs::create_directories(disk.game / "dir.txt");
	CHECK(disk.put("dir.txt") == Status::WrongType);
	CHECK(sandbox.Read("dir.txt", bytes) == Status::WrongType);
	CHECK(fs::is_directory(disk.game / "dir.txt"));
}

TEST_CASE("GameDataListAndDelete")
{
	const Disk disk;
	const Sandbox sandbox = disk.sandbox();
	std::vector<Entry> entries;
	CHECK(sandbox.List("", entries) == Status::NotFound);
	CHECK(disk.put("b.txt") == Status::Ok);
	CHECK(disk.put("a/x.txt") == Status::Ok);
	CHECK(disk.put("a/y/z.txt") == Status::Ok);
	std::ofstream(disk.game / "Outside-Rules.lua") << "x";

	CHECK(sandbox.List("", entries) == Status::Ok);
	REQUIRE(entries.size() == 2);
	CHECK((entries[0].name == "a" && entries[0].isDirectory));
	CHECK((entries[1].name == "b.txt" && !entries[1].isDirectory));
	CHECK(sandbox.List("a", entries) == Status::Ok);
	REQUIRE(entries.size() == 2);
	CHECK(sandbox.List("none", entries) == Status::NotFound);

	// No recursive delete.
	CHECK(sandbox.Delete("a") == Status::NotEmpty);
	CHECK(sandbox.Delete("a/y") == Status::NotEmpty);
	CHECK(sandbox.Delete("a/y/z.txt") == Status::Ok);
	CHECK(sandbox.Delete("a/y") == Status::Ok);
	CHECK_FALSE(fs::exists(disk.game / "a" / "y"));
	CHECK(fs::exists(disk.game / "a" / "x.txt"));
	CHECK(sandbox.Delete("b.txt") == Status::Ok);

	SECTION("a listing is bounded") {
		for (std::size_t i = 0; i < MAX_LIST + 8; ++i)
			std::ofstream(disk.game / ("f" + std::to_string(i) + ".txt")) << "";
		CHECK(sandbox.List("", entries) == Status::Ok);
		CHECK(entries.size() == MAX_LIST);
	}
}

TEST_CASE("GameDataQuotas")
{
	const Disk disk;
	const Sandbox sandbox = disk.sandbox();

	SECTION("a file's size, keeping the old file") {
		CHECK(disk.put("big.bin", "old") == Status::Ok);
		const std::string limit(MAX_FILE_BYTES, 'x');
		CHECK(disk.put("limit.bin", limit) == Status::Ok);
		const std::string over(MAX_FILE_BYTES + 1, 'x');
		CHECK(disk.put("big.bin", over) == Status::FileTooLarge);
		CHECK(ReadText(disk.game / "big.bin") == "old");
	}
	SECTION("the number of files, still replacing the ones there") {
		for (std::size_t i = 0; i < MAX_FILES; ++i)
			REQUIRE(disk.put("f" + std::to_string(i) + ".txt") == Status::Ok);
		CHECK(disk.put("onemore.txt") == Status::TooManyFiles);
		CHECK_FALSE(fs::exists(disk.game / "onemore.txt"));
		CHECK(disk.put("f0.txt", "again") == Status::Ok);
		CHECK(sandbox.Delete("f1.txt") == Status::Ok);
		CHECK(disk.put("onemore.txt") == Status::Ok);
	}
	SECTION("the number of folders, none created when refused") {
		for (std::size_t i = 0; i < MAX_FOLDERS - 1; ++i)
			REQUIRE(disk.put("d" + std::to_string(i) + "/x.txt") == Status::Ok);
		CHECK(disk.put("n1/n2/x.txt") == Status::TooManyFolders);
		CHECK_FALSE(fs::exists(disk.game / "n1"));
		CHECK(disk.put("n1/x.txt") == Status::Ok);
		CHECK(disk.put("n2/x.txt") == Status::TooManyFolders);
	}
	SECTION("the total bytes, counting what a replacement adds") {
		const std::string block(MAX_FILE_BYTES, 'x');
		const std::size_t blocks = static_cast<std::size_t>(MAX_TOTAL_BYTES / MAX_FILE_BYTES);
		for (std::size_t i = 0; i < blocks; ++i)
			REQUIRE(disk.put("b" + std::to_string(i) + ".bin", block) == Status::Ok);
		CHECK(disk.put("over.bin", "x") == Status::TooManyBytes);
		CHECK(disk.put("b0.bin", block) == Status::Ok);
		CHECK(disk.put("b0.bin", "small") == Status::Ok);
		CHECK(disk.put("over.bin", "x") == Status::Ok);
	}
}

TEST_CASE("GameDataChangeBudget")
{
	ChangeBudget budget;
	for (std::uint32_t i = 0; i < CHANGE_BURST; ++i)
		CHECK(budget.Allow(1000));
	CHECK_FALSE(budget.Allow(1000));
	CHECK_FALSE(budget.Allow(1100));
	CHECK(budget.Allow(1000 + 1000 / CHANGES_PER_SECOND));
	CHECK_FALSE(budget.Allow(1000 + 1000 / CHANGES_PER_SECOND));
	// A long rest fills it no further than the burst.
	for (std::uint32_t i = 0; i < CHANGE_BURST; ++i)
		CHECK(budget.Allow(100000000));
	CHECK_FALSE(budget.Allow(100000000));
}

TEST_CASE("GameDataReplayIsReadOnly")
{
	const Disk disk;
	CHECK(disk.put("kept.txt", "kept") == Status::Ok);
	const Sandbox replay = disk.sandbox(true);
	std::vector<std::uint8_t> bytes;
	std::vector<Entry> entries;
	CHECK(replay.Write("new.txt", "x", 1) == Status::ReadOnly);
	CHECK(replay.Write("kept.txt", "x", 1) == Status::ReadOnly);
	CHECK(replay.Delete("kept.txt") == Status::ReadOnly);
	CHECK(replay.Read("kept.txt", bytes) == Status::Ok);
	CHECK(std::string(bytes.begin(), bytes.end()) == "kept");
	CHECK(replay.List("", entries) == Status::Ok);
	CHECK_FALSE(fs::exists(disk.game / "new.txt"));
}

TEST_CASE("GameDataRefusesLinks")
{
	const Disk disk;
	std::vector<std::uint8_t> bytes;
	std::vector<Entry> entries;
	const Sandbox sandbox = disk.sandbox();
	fs::create_directories(disk.game);
	if (!MakeLink(disk.outside, disk.game / "probe", true))
		SKIP("this system does not make symlinks");
	fs::remove(disk.game / "probe");

	SECTION("the root") {
		fs::remove_all(disk.write / ROOT);
		REQUIRE(MakeLink(disk.outside, disk.write / ROOT, true));
		CHECK(disk.put("x.txt") == Status::Link);
		CHECK(sandbox.Read("canary.txt", bytes) == Status::Link);
		CHECK(sandbox.List("", entries) == Status::Link);
		CHECK(sandbox.Delete("x.txt") == Status::Link);
	}
	SECTION("the id directory") {
		fs::remove_all(disk.game);
		REQUIRE(MakeLink(disk.outside, disk.game, true));
		CHECK(disk.put("x.txt") == Status::Link);
		CHECK(sandbox.Read("canary.txt", bytes) == Status::Link);
		CHECK(sandbox.Delete("canary.txt") == Status::Link);
		CHECK(sandbox.List("", entries) == Status::Link);
	}
	SECTION("a folder on the way, first and in the middle") {
		REQUIRE(MakeLink(disk.outside, disk.game / "link", true));
		CHECK(disk.put("link/x.txt") == Status::Link);
		CHECK(sandbox.Read("link/canary.txt", bytes) == Status::Link);
		CHECK(sandbox.List("link", entries) == Status::Link);
		CHECK(sandbox.Delete("link/canary.txt") == Status::Link);
		CHECK(sandbox.Delete("link") == Status::Link);
		CHECK(fs::is_symlink(disk.game / "link"));

		fs::create_directories(disk.game / "a" / "b");
		REQUIRE(MakeLink(disk.outside, disk.game / "a" / "b" / "link", true));
		CHECK(disk.put("a/b/link/c/x.txt") == Status::Link);
		CHECK(sandbox.Read("a/b/link/canary.txt", bytes) == Status::Link);
	}
	SECTION("the file") {
		REQUIRE(MakeLink(disk.outside / "canary.txt", disk.game / "file.txt", false));
		CHECK(disk.put("file.txt", "overwritten") == Status::Link);
		CHECK(sandbox.Read("file.txt", bytes) != Status::Ok);
		CHECK(sandbox.Delete("file.txt") == Status::Link);
		CHECK(fs::is_symlink(disk.game / "file.txt"));
	}
	SECTION("a dangling link at the file, or at a temporary file's name") {
		REQUIRE(MakeLink(disk.outside / "new.txt", disk.game / "file.txt", false));
		CHECK(disk.put("file.txt") == Status::Link);
		REQUIRE(MakeLink(disk.outside / "planted.txt", disk.game / ".tmp-other.txt", false));
		CHECK(disk.put("other.txt", "safe") == Status::Ok);
		CHECK_FALSE(fs::exists(disk.outside / "planted.txt"));
		CHECK(ReadText(disk.game / "other.txt") == "safe");
	}
	SECTION("a listing shows no link, and quotas do not walk through one") {
		for (int i = 0; i < 4; ++i)
			std::ofstream(disk.outside / ("big" + std::to_string(i) + ".txt")) << std::string(1024, 'x');
		REQUIRE(MakeLink(disk.outside, disk.game / "link", true));
		CHECK(disk.put("real.txt") == Status::Ok);
		CHECK(sandbox.List("", entries) == Status::Ok);
		REQUIRE(entries.size() == 1);
		CHECK(entries[0].name == "real.txt");
	}
	CHECK(fs::exists(disk.outside / "canary.txt"));
	CHECK(ReadText(disk.outside / "canary.txt") == "canary");
	CHECK_FALSE(fs::exists(disk.outside / "x.txt"));
	CHECK_FALSE(fs::exists(disk.outside / "c"));
	CHECK_FALSE(fs::exists(disk.outside / "new.txt"));
}

// ---- Output files: append-only streams in <write dir>/game-output/<id>/.

namespace {

struct OutputDisk : Disk {
	fs::path output;
	OutputDisk() { output = write / OUTPUT_ROOT / "testgame"; }
	OutputStatus open(Outputs& outputs, std::string_view path, bool append, std::uint32_t& handle, std::uint64_t now = 0) const {
		return outputs.Open(write.string(), "testgame", path, append, now, handle);
	}
};

OutputStatus Put(Outputs& outputs, std::uint32_t handle, std::string_view text, std::uint64_t now = 0)
{
	return outputs.Write(handle, text.data(), text.size(), now);
}

}

TEST_CASE("GameOutputPaths")
{
	const OutputDisk disk;
	Outputs outputs;
	std::uint32_t handle = 0;
	for (const std::string_view path: HOSTILE_FILE_PATHS) {
		if (path == "x.dat" || path == "x.bin")
			continue;
		INFO(std::string(path));
		CHECK(disk.open(outputs, path, false, handle) == OutputStatus::InvalidPath);
		CHECK(disk.open(outputs, path, true, handle) == OutputStatus::InvalidPath);
		CHECK(handle == 0);
	}
	// Game data's own extensions that a loader or viewer picks up are not output.
	for (const char* path: {"x.png", "x.jpg", "x.toml", "x.jsonl.lua", "x.lua", "x.wasm"}) {
		INFO(path);
		CHECK(disk.open(outputs, path, false, handle) == OutputStatus::InvalidPath);
	}
	CHECK(disk.writeDirEntries() == 0);
	CHECK(disk.canaryIntact());
	for (const char* good: {"world.jsonl", "recordings/world-20261113-120000-30.jsonl", "a/b.json", "x.txt", "x.csv", "x.tsv", "x.log", "x.dat", "x.bin"}) {
		INFO(good);
		CHECK(disk.open(outputs, good, false, handle) == OutputStatus::Ok);
		CHECK(handle != 0);
		CHECK(outputs.Close(handle) == OutputStatus::Ok);
	}
}

TEST_CASE("GameOutputNeedsAValidId")
{
	const OutputDisk disk;
	Outputs outputs;
	std::uint32_t handle = 0;
	for (const char* id: {"", "..", "a/b", "../outside", "Upper", "con"}) {
		INFO(id);
		CHECK(outputs.Open(disk.write.string(), id, "x.jsonl", false, 0, handle) == OutputStatus::NoGameId);
	}
	CHECK(disk.writeDirEntries() == 0);
}

TEST_CASE("GameOutputAppends")
{
	const OutputDisk disk;
	Outputs outputs;
	std::uint32_t first = 0;
	REQUIRE(disk.open(outputs, "recordings/world.jsonl", false, first) == OutputStatus::Ok);
	CHECK(Put(outputs, first, "{\"a\":1}\n") == OutputStatus::Ok);
	CHECK(Put(outputs, first, "") == OutputStatus::Ok);
	CHECK(Put(outputs, first, "{\"b\":2}\n") == OutputStatus::Ok);
	// What a write put down is on disk before the stream closes.
	CHECK(ReadText(disk.output / "recordings" / "world.jsonl") == "{\"a\":1}\n{\"b\":2}\n");
	CHECK(outputs.Close(first) == OutputStatus::Ok);
	CHECK(outputs.Close(first) == OutputStatus::NoStream);
	CHECK(Put(outputs, first, "late") == OutputStatus::NoStream);
	CHECK(Put(outputs, 0, "none") == OutputStatus::NoStream);

	std::uint32_t again = 0;
	CHECK(disk.open(outputs, "recordings/world.jsonl", false, again) == OutputStatus::Exists);
	CHECK(again == 0);
	REQUIRE(disk.open(outputs, "recordings/world.jsonl", true, again) == OutputStatus::Ok);
	CHECK(again != first);
	CHECK(Put(outputs, again, "{\"c\":3}\n") == OutputStatus::Ok);
	CHECK(outputs.Close(again) == OutputStatus::Ok);
	CHECK(ReadText(disk.output / "recordings" / "world.jsonl") == "{\"a\":1}\n{\"b\":2}\n{\"c\":3}\n");

	// Separate from game data: neither directory reaches the other.
	std::vector<std::uint8_t> bytes;
	CHECK(disk.sandbox().Read("recordings/world.json", bytes) == Status::NotFound);
	CHECK_FALSE(fs::exists(disk.game));
#ifndef _WIN32
	const auto others = fs::perms::group_all | fs::perms::others_all;
	CHECK((fs::status(disk.output / "recordings" / "world.jsonl").permissions() & others) == fs::perms::none);
	CHECK((fs::status(disk.output / "recordings").permissions() & others) == fs::perms::none);
#endif
	SECTION("CloseAll ends every stream") {
		std::uint32_t a = 0, b = 0;
		REQUIRE(disk.open(outputs, "a.txt", false, a) == OutputStatus::Ok);
		REQUIRE(disk.open(outputs, "b.txt", false, b) == OutputStatus::Ok);
		CHECK(outputs.CloseAll() == 2);
		CHECK(outputs.OpenCount() == 0);
		CHECK(Put(outputs, a, "x") == OutputStatus::NoStream);
	}
}

TEST_CASE("GameOutputWrongTypes")
{
	const OutputDisk disk;
	Outputs outputs;
	std::uint32_t handle = 0;
	fs::create_directories(disk.output / "folder.jsonl");
	CHECK(disk.open(outputs, "folder.jsonl", true, handle) == OutputStatus::WrongType);
	CHECK(disk.open(outputs, "folder.jsonl", false, handle) == OutputStatus::WrongType);
	std::ofstream(disk.output / "file") << "x";
	CHECK(disk.open(outputs, "file/x.jsonl", false, handle) == OutputStatus::WrongType);
#ifndef _WIN32
	REQUIRE(mkfifo((disk.output / "fifo.jsonl").c_str(), 0600) == 0);
	CHECK(disk.open(outputs, "fifo.jsonl", true, handle) != OutputStatus::Ok);
#endif
	CHECK(outputs.OpenCount() == 0);
}

TEST_CASE("GameOutputLimits")
{
	const OutputDisk disk;
	OutputLimits limits;
	limits.maxFileBytes = 100;
	limits.maxTotalBytes = 250;
	limits.maxFiles = 4;
	limits.maxFolders = 2;
	limits.bytesPerSecond = 1000;
	Outputs outputs(limits);
	std::uint32_t handle = 0;

	SECTION("a file's size: a write that passes it puts nothing down") {
		REQUIRE(disk.open(outputs, "a.txt", false, handle) == OutputStatus::Ok);
		CHECK(Put(outputs, handle, std::string(60, 'x')) == OutputStatus::Ok);
		CHECK(Put(outputs, handle, std::string(41, 'x')) == OutputStatus::FileTooLarge);
		CHECK(Put(outputs, handle, std::string(40, 'x')) == OutputStatus::Ok);
		CHECK(fs::file_size(disk.output / "a.txt") == 100);
		CHECK(outputs.Close(handle) == OutputStatus::Ok);
		// Continuing a file counts what it already holds.
		REQUIRE(disk.open(outputs, "a.txt", true, handle, 10000) == OutputStatus::Ok);
		CHECK(Put(outputs, handle, "x", 10000) == OutputStatus::FileTooLarge);
	}
	SECTION("the directory's total, earlier runs included") {
		fs::create_directories(disk.output);
		std::ofstream(disk.output / "old.txt") << std::string(200, 'x');
		REQUIRE(disk.open(outputs, "new.txt", false, handle) == OutputStatus::Ok);
		CHECK(Put(outputs, handle, std::string(51, 'x')) == OutputStatus::TooManyBytes);
		CHECK(Put(outputs, handle, std::string(50, 'x')) == OutputStatus::Ok);
		std::uint32_t other = 0;
		CHECK(disk.open(outputs, "other.txt", false, other) == OutputStatus::TooManyBytes);
		CHECK_FALSE(fs::exists(disk.output / "other.txt"));
		CHECK(outputs.Close(handle) == OutputStatus::Ok);
		fs::remove(disk.output / "old.txt");
		CHECK(disk.open(outputs, "other.txt", false, other, 10000) == OutputStatus::Ok);
	}
	SECTION("files and folders, none created when refused") {
		for (int i = 0; i < 4; ++i) {
			REQUIRE(disk.open(outputs, "f" + std::to_string(i) + ".txt", false, handle, 10000 * i) == OutputStatus::Ok);
			CHECK(outputs.Close(handle) == OutputStatus::Ok);
		}
		CHECK(disk.open(outputs, "f4.txt", false, handle) == OutputStatus::TooManyFiles);
		CHECK(disk.open(outputs, "f0.txt", true, handle) == OutputStatus::Ok);
		CHECK(outputs.Close(handle) == OutputStatus::Ok);
		fs::remove(disk.output / "f0.txt");
		fs::remove(disk.output / "f1.txt");
		CHECK(disk.open(outputs, "d1/d2/x.txt", false, handle, 100000) == OutputStatus::Ok);
		CHECK(outputs.Close(handle) == OutputStatus::Ok);
		CHECK(disk.open(outputs, "d3/x.txt", false, handle, 200000) == OutputStatus::TooManyFolders);
		CHECK_FALSE(fs::exists(disk.output / "d3"));
	}
	SECTION("streams open at once") {
		std::vector<std::uint32_t> open;
		for (std::size_t i = 0; i < MAX_OPEN_OUTPUTS; ++i) {
			std::uint32_t h = 0;
			OutputLimits wide;
			outputs.SetLimits(wide);
			REQUIRE(disk.open(outputs, "s" + std::to_string(i) + ".txt", false, h, 10000 * i) == OutputStatus::Ok);
			open.push_back(h);
		}
		CHECK(disk.open(outputs, "more.txt", false, handle, 1000000) == OutputStatus::TooManyOpen);
		CHECK(outputs.Close(open.front()) == OutputStatus::Ok);
		CHECK(disk.open(outputs, "more.txt", false, handle, 1000000) == OutputStatus::Ok);
	}
	SECTION("opens a second") {
		OutputLimits wide;
		outputs.SetLimits(wide);
		for (std::uint32_t i = 0; i < OPEN_BURST; ++i) {
			REQUIRE(disk.open(outputs, "o" + std::to_string(i) + ".txt", false, handle, 1000) == OutputStatus::Ok);
			CHECK(outputs.Close(handle) == OutputStatus::Ok);
		}
		CHECK(disk.open(outputs, "late.txt", false, handle, 1000) == OutputStatus::RateLimited);
		CHECK_FALSE(fs::exists(disk.output / "late.txt"));
		CHECK(disk.open(outputs, "late.txt", false, handle, 1000 + 1000 / OPENS_PER_SECOND) == OutputStatus::Ok);
	}
	SECTION("bytes a second") {
		limits.maxFileBytes = 1 << 20;
		limits.maxTotalBytes = 1 << 20;
		outputs.SetLimits(limits);
		REQUIRE(disk.open(outputs, "fast.bin", false, handle, 0) == OutputStatus::Ok);
		// A burst of four seconds' worth, then the rate.
		CHECK(Put(outputs, handle, std::string(4000, 'x'), 0) == OutputStatus::Ok);
		CHECK(Put(outputs, handle, "x", 0) == OutputStatus::RateLimited);
		CHECK(Put(outputs, handle, std::string(500, 'x'), 500) == OutputStatus::Ok);
		CHECK(Put(outputs, handle, "x", 500) == OutputStatus::RateLimited);
		CHECK(fs::file_size(disk.output / "fast.bin") == 4500);
	}
}

TEST_CASE("GameOutputRateBudget")
{
	RateBudget budget(10, 4);
	CHECK(budget.Allow(10, 1000));
	CHECK_FALSE(budget.Allow(1, 1000));
	// Small steps still earn the full rate: 4 a second.
	CHECK_FALSE(budget.Allow(1, 1100));
	CHECK_FALSE(budget.Allow(1, 1200));
	CHECK(budget.Allow(1, 1250));
	CHECK(budget.Allow(3, 2000));
	CHECK_FALSE(budget.Allow(1, 2000));
	// A long rest fills it no further than the burst, and an amount over the burst never passes.
	CHECK_FALSE(budget.Allow(11, 100000000));
	CHECK(budget.Allow(10, 100000000));
}

TEST_CASE("GameOutputRefusesLinks")
{
	const OutputDisk disk;
	Outputs outputs;
	std::uint32_t handle = 0;
	fs::create_directories(disk.output);
	if (!MakeLink(disk.outside, disk.output / "probe", true))
		SKIP("this system does not make symlinks");
	fs::remove(disk.output / "probe");

	SECTION("the root") {
		fs::remove_all(disk.write / OUTPUT_ROOT);
		REQUIRE(MakeLink(disk.outside, disk.write / OUTPUT_ROOT, true));
		CHECK(disk.open(outputs, "x.txt", false, handle) == OutputStatus::Link);
	}
	SECTION("the id directory") {
		fs::remove_all(disk.output);
		REQUIRE(MakeLink(disk.outside, disk.output, true));
		CHECK(disk.open(outputs, "x.txt", false, handle) == OutputStatus::Link);
		CHECK(disk.open(outputs, "canary.txt", true, handle) == OutputStatus::Link);
	}
	SECTION("a folder on the way") {
		REQUIRE(MakeLink(disk.outside, disk.output / "link", true));
		CHECK(disk.open(outputs, "link/x.txt", false, handle) == OutputStatus::Link);
		CHECK(disk.open(outputs, "link/canary.txt", true, handle) == OutputStatus::Link);
	}
	SECTION("the file, existing or dangling") {
		REQUIRE(MakeLink(disk.outside / "canary.txt", disk.output / "file.txt", false));
		CHECK(disk.open(outputs, "file.txt", true, handle) == OutputStatus::Link);
		CHECK(disk.open(outputs, "file.txt", false, handle) == OutputStatus::Exists);
		REQUIRE(MakeLink(disk.outside / "new.txt", disk.output / "dangling.txt", false));
		CHECK(disk.open(outputs, "dangling.txt", true, handle) == OutputStatus::Link);
		CHECK(disk.open(outputs, "dangling.txt", false, handle) == OutputStatus::Exists);
	}
	CHECK(outputs.OpenCount() == 0);
	CHECK(ReadText(disk.outside / "canary.txt") == "canary");
	CHECK_FALSE(fs::exists(disk.outside / "x.txt"));
	CHECK_FALSE(fs::exists(disk.outside / "new.txt"));
}
