/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#ifndef LUA_DOWNLOADER_H
#define LUA_DOWNLOADER_H

#include "System/EventClient.h"

#include <string>

struct lua_State;

class LuaVFSDownload: public CEventClient {
public:
	static LuaVFSDownload* GetInstance() {
		static LuaVFSDownload instance;
		return &instance;
	}

	static void Init();
	static void Free(bool stopDownloads = false);

	static bool PushEntries(lua_State* L);

	// Shared by the Lua and NativeInterface entry points.  Keeping validation,
	// queue-id allocation, and event dispatch here prevents the two public
	// surfaces from drifting apart.
	static bool QueueArchiveDownload(const std::string& filename, const std::string& category, std::string* errorMessage = nullptr);
	static bool AbortQueuedDownload(int id);


	bool WantsEvent(const std::string& eventName) override {
		return (eventName == "Update");
	}

	// checks if events have arrived from download-threads and processes them
	void Update() override;

private:
	LuaVFSDownload();
	~LuaVFSDownload();

	static int DownloadArchive(lua_State* L);
	static int AbortDownload(lua_State* L);
	static int ScanAllDirs(lua_State* L);
};

#define luaVFSDownload (LuaVFSDownload::GetInstance())

#endif /* LUA_DOWNLOADER_H */
