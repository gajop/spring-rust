/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#ifndef _MESSAGEBOX_H_
#define _MESSAGEBOX_H_

#include "System/Platform/errorhandler.h" // MBF_OK, etc.

namespace Platform {
	/**
	 * Whether message boxes are disabled: SPRING_NO_MESSAGEBOX is set to anything but "" or "0".
	 * Meant for unattended runs (tools, CI, servers) where a dialog nobody dismisses outlives
	 * the process. Read on every call, so it also holds in the crash handler before any config.
	 */
	bool MsgBoxDisabled();

	/**
	 * Will pop up an message window, or write it to stderr when MsgBoxDisabled()
	 * (the engine still logs fatal errors itself and exits as usual)
	 * @param  message the main text, describing the error
	 * @param  caption will appear in the title bar of the error window
	 * @param  flags   @see errorhandler.h
	 */
	void MsgBox(const char* message, const char* caption, unsigned int flags);

	/// the platform's own window, behind MsgBox
	void MsgBoxNative(const char* message, const char* caption, unsigned int flags);
}

#endif // _MESSAGEBOX_H_
