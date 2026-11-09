/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <algorithm>
#include <cstddef>
#include <string>
#include <string_view>

namespace recoil::wasm::core {

// What of a console line goes into a guest's callin scratch.
struct ConsoleLineFit {
	std::string_view message;
	std::string_view section;
	std::string marker;  // follows the message when it was cut
};

// Longest prefix of `text` of at most `bytes` that ends between UTF-8 characters.
inline std::string_view Utf8Prefix(std::string_view text, std::size_t bytes)
{
	if (bytes >= text.size())
		return text;
	while (bytes > 0 && (static_cast<unsigned char>(text[bytes]) & 0xC0) == 0x80)
		--bytes;
	return text.substr(0, bytes);
}

// A console line belongs to whatever logged it, not to the module listening, so its length can
// never be that module's fault: what does not fit `room` bytes is cut, and the marker says so.
// The infolog has its own copy of the line, whole.
inline ConsoleLineFit FitConsoleLine(std::string_view message, std::string_view section,
	std::size_t room)
{
	if (message.size() <= room && section.size() <= room - message.size())
		return {message, section, {}};

	ConsoleLineFit fit;
	fit.section = Utf8Prefix(section, room / 4);
	room -= fit.section.size();
	fit.marker = " ...[cut to fit this module: " + std::to_string(message.size()) +
		" bytes in the infolog]";
	if (fit.marker.size() > room)
		fit.marker.clear();
	if (message.size() <= room) {
		fit.marker.clear();
		fit.message = message;
		return fit;
	}
	fit.message = Utf8Prefix(message, room - fit.marker.size());
	return fit;
}

} // namespace recoil::wasm::core
