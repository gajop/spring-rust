/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "WasmModuleManifest.h"

#include <algorithm>
#include <charconv>
#include <cctype>
#include <string>

namespace {
	std::string Trim(std::string_view value)
	{
		std::size_t first = 0;
		while (first < value.size() && std::isspace(static_cast<unsigned char>(value[first])))
			++first;
		std::size_t last = value.size();
		while (last > first && std::isspace(static_cast<unsigned char>(value[last - 1])))
			--last;
		return std::string(value.substr(first, last - first));
	}

	bool IsSafeContentPath(std::string_view path)
	{
		if (path.empty() || path.size() > 4096 || path.front() == '/' ||
			path.find('\\') != std::string_view::npos ||
			path.find(':') != std::string_view::npos)
			return false;
		std::size_t start = 0;
		while (start <= path.size()) {
			const std::size_t end = path.find('/', start);
			const auto component = path.substr(start,
				end == std::string_view::npos ? path.size() - start : end - start);
			if (component.empty() || component == "." || component == "..")
				return false;
			if (end == std::string_view::npos)
				break;
			start = end + 1;
		}
		return true;
	}

	bool IsSafeModuleName(std::string_view name)
	{
		if (name.empty() || name.size() > 128)
			return false;
		for (const unsigned char character : name) {
			if (!std::isalnum(character) && character != '-' && character != '_' &&
				character != '.')
				return false;
		}
		return name.front() != '.' && name.back() != '.';
	}

	bool IsInterfaceVersion(std::string_view version)
	{
		// Keep the package format deliberately strict.  The runtime currently
		// exposes one semver-like Core interface, so accepting a range or a
		// partially specified version would make synced compatibility ambiguous.
		if (version.size() > 32 || version.empty())
			return false;
		unsigned components = 0;
		unsigned digits = 0;
		for (const unsigned char character : version) {
			if (std::isdigit(character)) {
				if (++digits > 9)
					return false;
				continue;
			}
			if (character != '.' || digits == 0 || ++components > 2)
				return false;
			digits = 0;
		}
		return components == 2 && digits != 0;
	}
}

bool WasmModuleManifest::Parse(std::string_view text,
	std::vector<WasmModuleDeclaration>& declarations, std::string& error)
{
	constexpr std::size_t maxDeclarations = 256;
	declarations.clear();
	if (text.size() > 1024u * 1024u) {
		error = "Wasm manifest exceeds the configured byte limit";
		return false;
	}
	std::size_t lineStart = 0;
	std::size_t lineNumber = 0;
	while (lineStart <= text.size()) {
		++lineNumber;
		const std::size_t lineEnd = text.find('\n', lineStart);
		const std::size_t length = lineEnd == std::string_view::npos ?
			text.size() - lineStart : lineEnd - lineStart;
		std::string line = Trim(text.substr(lineStart, length));
		if (!line.empty() && line.front() != '#') {
			if (line.rfind("module(", 0) != 0 || line.back() != ')') {
				error = "manifest line " + std::to_string(lineNumber) +
					" must use module(name, path, environment, order[, interface-version])";
				return false;
			}
			const std::string body = line.substr(7, line.size() - 8);
			std::vector<std::string> fields;
			std::size_t fieldStart = 0;
			while (fieldStart <= body.size()) {
				const std::size_t fieldEnd = body.find(',', fieldStart);
				const std::size_t fieldLength = fieldEnd == std::string::npos ?
					body.size() - fieldStart : fieldEnd - fieldStart;
				fields.push_back(Trim(std::string_view(body).substr(fieldStart, fieldLength)));
				if (fieldEnd == std::string::npos)
					break;
				fieldStart = fieldEnd + 1;
			}
			if ((fields.size() != 4 && fields.size() != 5) || !IsSafeModuleName(fields[0]) || fields[1].empty() ||
				fields[2].empty() || fields[3].empty()) {
				error = "manifest line " + std::to_string(lineNumber) +
					" has the wrong number of non-empty fields";
				return false;
			}
			if (!IsSafeContentPath(fields[1])) {
				error = "manifest line " + std::to_string(lineNumber) +
					" has a path outside the content archive";
				return false;
			}
			WasmEnvironment environment = WasmEnvironment::RulesSynced;
			if (!WasmEnvironmentMatrix::Parse(fields[2], environment)) {
				error = "manifest line " + std::to_string(lineNumber) +
					" has an unknown execution environment";
				return false;
			}
			std::uint32_t order = 0;
			const auto [end, conversionError] = std::from_chars(
				fields[3].data(), fields[3].data() + fields[3].size(), order);
			if (conversionError != std::errc{} || end != fields[3].data() + fields[3].size()) {
				error = "manifest line " + std::to_string(lineNumber) +
					" has an invalid module order";
				return false;
			}
			const std::string interfaceVersion = fields.size() == 5 ? fields[4] : "1.0.0";
			if (!IsInterfaceVersion(interfaceVersion)) {
				error = "manifest line " + std::to_string(lineNumber) +
					" has an invalid interface version";
				return false;
			}
			if (std::any_of(declarations.begin(), declarations.end(), [&fields](const auto& declaration) {
				return declaration.name == fields[0];
			})) {
				error = "manifest contains duplicate module " + fields[0];
				return false;
			}
			if (declarations.size() >= maxDeclarations) {
				error = "manifest contains more than " + std::to_string(maxDeclarations) +
					" Wasm modules";
				return false;
			}
			declarations.push_back({fields[0], fields[1], environment, order,
				interfaceVersion, {}});
		}
		if (lineEnd == std::string_view::npos)
			break;
		lineStart = lineEnd + 1;
	}
	return true;
}
