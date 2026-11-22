/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include <cstdio>
#include <cstdlib>
#include <cstring>

#include "System/Platform/MessageBox.h"

namespace Platform {

bool MsgBoxDisabled()
{
	const char* value = std::getenv("SPRING_NO_MESSAGEBOX");
	return (value != nullptr && value[0] != '\0' && std::strcmp(value, "0") != 0);
}

void MsgBox(const char* message, const char* caption, unsigned int flags)
{
	if (!MsgBoxDisabled()) {
		MsgBoxNative(message, caption, flags);
		return;
	}

	const char* kind = "Error";

	if ((flags & MBF_INFO) != 0) {
		kind = "Info";
	} else if ((flags & MBF_EXCL) != 0) {
		kind = "Warning";
	}

	std::fprintf(stderr, "%s: %s\n  %s\n", kind, caption, message);
	std::fflush(stderr);
}

} //namespace Platform
