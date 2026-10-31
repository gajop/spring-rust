/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#ifndef DEMO_RECORDER
#define DEMO_RECORDER

#include <vector>
#include <sstream>
#include <zlib.h>

#include "Demo.h"
#include "Game/Players/PlayerStatistics.h"
#include "Sim/Misc/TeamStatistics.h"


/**
 * @brief Used to record demos
 */
class CDemoRecorder : public CDemo
{
public:
	CDemoRecorder() { memset(&fileHeader, 0, sizeof(fileHeader)); }
	CDemoRecorder(const std::string& mapName, const std::string& modName, bool serverDemo);

	CDemoRecorder(const CDemoRecorder&) = delete;
	CDemoRecorder(CDemoRecorder&& r) { *this = std::move(r); }

	~CDemoRecorder();


	CDemoRecorder& operator = (const CDemoRecorder&) = delete;
	CDemoRecorder& operator = (CDemoRecorder&& r) {
		memcpy(&fileHeader, &r.fileHeader, sizeof(fileHeader));
		memset(&r.fileHeader, 0, sizeof(fileHeader));

		std::swap(file, r.file);

		std::swap(demoName, r.demoName);
		std::swap(playerStats, r.playerStats);
		std::swap(teamStats, r.teamStats);
		std::swap(winningAllyTeams, r.winningAllyTeams);

		std::swap(isServerDemo, r.isServerDemo);
		std::swap(savedSoFar, r.savedSoFar);
		return *this;
	}


	bool IsValid() const { return (file != nullptr); }

	void WriteSetupText(const std::string& text);
	void SaveToDemo(const unsigned char* buf, const unsigned length, const float modGameTime);

	void SetStream();
	void SetName(const std::string& mapName, const std::string& modName);

	void SetGameID(const unsigned char* buf);
	void SetTime(int gameTime, int wallclockTime);

	void AddNewPlayer(const std::string& name, int playerNum);
	void InitializeStats(int numPlayers, int numTeams);
	void SetPlayerStats(int playerNum, const PlayerStatistics& stats);
	void SetTeamStats(int teamNum, const std::vector<TeamStatistics>& stats);
	void SetWinningAllyTeams(const std::vector<unsigned char>& winningAllyTeams);

	/**
	 * @brief Write what is recorded so far to the demo's file as a complete, playable demo
	 * (without statistics); recording goes on, and the whole demo replaces it at the end.
	 * @param path the file written, on success
	 */
	bool SaveSoFar(int gameTime, int wallclockTime, std::string& path);

private:
	unsigned int WriteFileHeader(bool updateStreamLength);
	void SetFileHeader();
	void WritePlayerStats();
	void WriteTeamStats();
	void WriteWinnerList();
	void WriteDemoFile();

private:
	gzFile file = nullptr;

	std::vector<PlayerStatistics> playerStats;
	std::vector< std::vector<TeamStatistics> > teamStats;
	std::vector<unsigned char> winningAllyTeams;

	bool isServerDemo = false;
	/// SaveSoFar wrote the file through its own handle; the end reopens it to write in full.
	bool savedSoFar = false;
};


#endif
