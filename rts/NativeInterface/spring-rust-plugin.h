#pragma once

extern "C" {

void DownloadFailed(int ID, int errorID);
void DownloadFinished(int ID);
void DownloadProgress(int ID, long downloaded, long total);
void DownloadQueued(int ID, const char *archiveName, const char *archiveType);
void DownloadStarted(int ID);
void FeatureCreated(int featureID);
void FeatureDestroyed(int featureID);
void GameID(const unsigned char *gameID, unsigned int numBytes);
void GamePaused(int playerID, bool paused);
void GamePreload();
void GameStart();
void PlayerAdded(int playerID);
void PlayerChanged(int playerID);
void PlayerRemoved(int playerID, int reason);
void RenderUnitDestroyed(int unitID);
void RunDelayedFunctions(int frameNum);
void Shutdown();
void TeamChanged(int teamID);
void TeamDied(int teamID);
void UnitCreated(int unitID, int builderID);
void UnitDestroyed(int unitID, int attackerID);
void UnitExperience(int unitID, float oldExperience);
void UnitFinished(int unitID);
void UnitFromFactory(int unitID, int factoryID, bool userOrders);
void UnitGiven(int unitID, int oldTeam, int newTeam);
void UnitLoaded(int unitID, int transportID);
void UnitStunned(int unitID, bool stunned);
void UnitTaken(int unitID, int oldTeam, int newTeam);
void UnitUnloaded(int unitID, int transportID);

}
