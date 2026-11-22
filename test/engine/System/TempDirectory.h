/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <filesystem>
#include <random>
#include <string>
#include <system_error>

namespace spring::test {

class TempDirectory {
public:
	explicit TempDirectory(const char* prefix)
	{
		std::random_device random;
		const auto root = std::filesystem::temp_directory_path();
		for (;;) {
			directory = root / (std::string(prefix) + std::to_string(random()) + "-" + std::to_string(random()));
			std::error_code error;
			if (std::filesystem::create_directory(directory, error))
				break;
			if (error && error != std::errc::file_exists)
				throw std::filesystem::filesystem_error("create temporary directory", directory, error);
		}
	}

	~TempDirectory()
	{
		std::error_code error;
		std::filesystem::remove_all(directory, error);
	}

	TempDirectory(const TempDirectory&) = delete;
	TempDirectory& operator=(const TempDirectory&) = delete;
	const std::filesystem::path& Path() const { return directory; }

private:
	std::filesystem::path directory;
};

}
