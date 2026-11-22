/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

/**
 * @brief error messages
 * Error handling based on platform.
 */


#include "errorhandler.h"

#include <cstdio>
#include <cstdlib>
#include <ctime>
#include <string>
#include <functional>

#ifdef _WIN32
	#include <process.h>
	#define GETPID _getpid
#else
	#include <unistd.h>
	#define GETPID getpid
#endif

#include "System/SpringExitCode.h"
#include "System/Log/ILog.h"
#include "System/Log/LogSinkHandler.h"
#include "System/Threading/SpringThreading.h"
#include "System/FileSystem/DataDirLocater.h"

#if !defined(DEDICATED)
	#include "System/SpringApp.h"
	#include "System/Platform/Threading.h"
#endif
#if !defined(DEDICATED) && !defined(HEADLESS)
	#include "System/Platform/MessageBox.h"
#endif
#ifdef DEDICATED
	#include "Net/GameServer.h"
	#include "System/SafeUtil.h"
#endif


/**
 * With SPRING_FATAL_LOG=<file> set, appends one line per fatal error to it:
 * time (UTC), pid, caption, message (newlines as \\n) and the write-dir.
 * The file is moved to <file>.1 once it passes 256 KiB, so it stays small.
 * Meant for unattended runs whose own directories are deleted afterwards.
 */
static void AppendFatalLog(const char* msg, const char* caption)
{
	const char* path = std::getenv("SPRING_FATAL_LOG");

	if (path == nullptr || path[0] == '\0')
		return;

	if (FILE* f = std::fopen(path, "rb")) {
		std::fseek(f, 0, SEEK_END);
		const long size = std::ftell(f);
		std::fclose(f);

		if (size > 256 * 1024) {
			const std::string old = std::string(path) + ".1";
			std::remove(old.c_str());
			std::rename(path, old.c_str());
		}
	}

	FILE* f = std::fopen(path, "ab");

	if (f == nullptr)
		return;

	char stamp[32] = "";
	const std::time_t now = std::time(nullptr);
	std::strftime(stamp, sizeof(stamp), "%Y-%m-%dT%H:%M:%SZ", std::gmtime(&now));

	std::string text = std::string(caption) + " | " + msg;

	for (char& c: text) {
		if (c == '\n' || c == '\r')
			c = ' ';
	}

	std::fprintf(f, "%s pid=%d caption+message=\"%s\" writedir=\"%s\"\n", stamp, int(GETPID()), text.c_str(), dataDirLocater.GetWriteDirPath().c_str());
	std::fclose(f);
}


static void ExitSpringProcessAux(bool waitForExit, bool exitSuccess)
{
	// wait a bit before forcing the kill
	if (waitForExit)
		spring::this_thread::sleep_for(std::chrono::seconds(5));

	logSinkHandler.SetSinking(false);

#ifdef _MSC_VER
	if (!exitSuccess)
		TerminateProcess(GetCurrentProcess(), spring::EXIT_CODE_CRASHED);
#endif

	exit(spring::EXIT_CODE_CRASHED);
}


#ifdef DEDICATED
static void ExitSpringProcess(const char* msg, const char* caption, unsigned int flags)
{
	LOG_L(L_ERROR, "[%s] errorMsg=\"%s\" msgCaption=\"%s\"", __func__, msg, caption);
	AppendFatalLog(msg, caption);

	spring::SafeDelete(gameServer);
	ExitSpringProcessAux(false, true);
}

#else

static void ExitSpringProcess(const char* msg, const char* caption, unsigned int flags)
{
	LOG_L(L_FATAL, "[%s] errorMsg=\"%s\" msgCaption=\"%s\" mainThread=%d", __func__, msg, caption, Threading::IsMainThread());
	AppendFatalLog(msg, caption);

	switch (SpringApp::PostKill(Threading::Error(caption, msg, flags))) {
		case -1: {
			// main thread; either gets to ESPA first and cleans up our process or exit is forced by this
			std::function<void()> forcedExitFunc = [&]() { ExitSpringProcessAux(true, false); };
			spring::thread forcedExitThread = spring::thread(forcedExitFunc);

			// .join can (very rarely) throw a no-such-process exception if it runs in parallel with exit
			assert(forcedExitThread.joinable());
			forcedExitThread.detach();

			SpringApp::Kill(false);
		} break;
		case 0: { assert(false); } break; // [unreachable] thread failed to post, ESPA
		case 1: {        return; } break; // thread posted successfully
	}

	ExitSpringProcessAux(false, false);
}
#endif


void ErrorMessageBox(const char* msg, const char* caption, unsigned int flags)
{
	#if (!defined(DEDICATED) && !defined(HEADLESS))
	if (Threading::IsMainThread()) {
		// the thread that throws up this message-box will be blocked
		// until it is clicked away which can cause spurious detected
		// hangs, so deregister it here (by forwarding an empty error)
		// note: main also does the talking for other threads via Run
		SpringApp::PostKill({});
		Platform::MsgBox(msg, caption, flags);
	}
	#endif

	ExitSpringProcess(msg, caption, flags);
}

