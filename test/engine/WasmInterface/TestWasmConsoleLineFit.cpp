/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

// A console line of any length has to reach a Wasm module's callin scratch without faulting it;
// test/wasm_api/data/console_line_fixture/run.py checks the same against a real guest.
// The chat input's Tab listing stays a few rows of whole names, however many match.

#include <catch_amalgamated.hpp>

#include <string>
#include <vector>

#include "Game/WordCompletion.h"
#include "WasmInterface/core/bindings/WasmConsoleLineFit.h"

using recoil::wasm::core::FitConsoleLine;
using recoil::wasm::core::Utf8Prefix;

TEST_CASE("A console line that fits is untouched")
{
	const auto fit = FitConsoleLine("hello", "section", 12);
	CHECK(fit.message == "hello");
	CHECK(fit.section == "section");
	CHECK(fit.marker.empty());
}

TEST_CASE("A longer console line is cut to the room, with a marker")
{
	const std::string line(1 << 20, 'x');
	for (const std::size_t room: {std::size_t(0), std::size_t(10), std::size_t(60), std::size_t(236), std::size_t(16364)}) {
		const auto fit = FitConsoleLine(line, "section", room);
		CHECK(fit.message.size() + fit.marker.size() + fit.section.size() <= room);
		CHECK(fit.message.size() < line.size());
	}
	const auto fit = FitConsoleLine(line, "section", 236);
	CHECK_FALSE(fit.message.empty());
	CHECK(fit.marker.find("1048576 bytes") != std::string::npos);
	CHECK(fit.section == "section");
}

TEST_CASE("A long section does not push out the message, and a whole message keeps no marker")
{
	const std::string section(1000, 's');
	const auto fit = FitConsoleLine("hello", section, 100);
	CHECK(fit.message == "hello");
	CHECK(fit.marker.empty());
	CHECK(fit.section.size() == 25);
}

TEST_CASE("A cut falls between UTF-8 characters")
{
	const std::string two = "\xC3\xA9\xC3\xA9";    // two 2-byte characters
	const std::string four = "\xF0\x9F\x9A\x80";   // one 4-byte character
	CHECK(Utf8Prefix(two, 1).empty());
	CHECK(Utf8Prefix(two, 2).size() == 2);
	CHECK(Utf8Prefix(two, 3).size() == 2);
	CHECK(Utf8Prefix(two, 4).size() == 4);
	CHECK(Utf8Prefix(four, 3).empty());
	CHECK(Utf8Prefix(four, 9).size() == 4);
}

TEST_CASE("The Tab listing is a few rows of whole names")
{
	std::vector<std::string> many;
	for (int index = 0; index < 813; ++index)
		many.push_back("gen_ship_with_a_name_" + std::to_string(index));

	const auto rows = CWordCompletion::ListMatches(many);
	CHECK(rows.size() <= 5);
	std::size_t listed = 0;
	for (std::size_t index = 0; index + 1 < rows.size(); ++index) {
		CHECK(rows[index].size() <= 100);
		for (std::size_t at = rows[index].find("gen_"); at != std::string::npos; at = rows[index].find("gen_", at + 1))
			listed += 1;
		CHECK(rows[index].rfind("  ", 0) == 0);
	}
	CHECK(listed < many.size());
	CHECK(rows.back() == "  ... and " + std::to_string(many.size() - listed) + " more, 813 in all");

	CHECK(CWordCompletion::ListMatches({}).empty());
	CHECK(CWordCompletion::ListMatches({"alpha", "beta"}) == std::vector<std::string>{"  alpha  beta"});

	const std::string longName(300, 'n');
	CHECK(CWordCompletion::ListMatches({longName}) == std::vector<std::string>{"  " + longName});
}
