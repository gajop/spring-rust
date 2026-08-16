use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, raw::copy_c_string, sys};

pub struct Teams<'a> {
    api: &'a sys::TeamsApi,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeamInfo {
    pub team_id: i32,
    pub ally_team_id: i32,
    pub leader_id: i32,
    pub is_dead: bool,
    pub side: String,
    pub color: u32,
    pub custom_keys: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlayerInfo {
    pub player_id: i32,
    pub name: String,
    pub is_active: bool,
    pub is_ai: bool,
    pub is_spectator: bool,
    pub team_id: i32,
    pub ally_team_id: i32,
    pub ping_time: f32,
    pub cpu_usage: f32,
    pub country: String,
    pub rank: i32,
    pub has_skirmish_ais_in_team: bool,
    pub custom_keys: Option<String>,
    pub desynced: bool,
}

impl<'a> Teams<'a> {
    pub(crate) fn new(api: &'a sys::TeamsApi) -> Self {
        Self { api }
    }

    pub fn get_team_info_owned(
        &self,
        team_id: i32,
        get_team_keys: bool,
    ) -> Result<TeamInfo, Error> {
        let info = self.get_team_info(team_id, get_team_keys)?;
        Ok(TeamInfo {
            team_id: info.teamID,
            ally_team_id: info.allyTeamID,
            leader_id: info.leaderID,
            is_dead: info.isDead,
            // SAFETY: team strings are engine-owned and valid for this call.
            side: unsafe { copy_c_string(info.side) }.unwrap_or_default(),
            color: info.color,
            // SAFETY: team strings are engine-owned and valid for this call.
            custom_keys: unsafe { copy_c_string(info.customKeys) },
        })
    }

    pub fn get_player_info_owned(
        &self,
        player_id: i32,
        get_player_keys: bool,
    ) -> Result<PlayerInfo, Error> {
        self.get_player_info(player_id, get_player_keys)
            .map(PlayerInfo::from_raw)
    }
}

include!(concat!(env!("OUT_DIR"), "/teams_generated.rs"));

impl PlayerInfo {
    fn from_raw(info: sys::PlayerInfo) -> Self {
        // SAFETY: player strings are engine-owned and valid for this call.
        unsafe {
            Self {
                player_id: info.playerID,
                name: copy_c_string(info.name).unwrap_or_default(),
                is_active: info.isActive,
                is_ai: info.isAI,
                is_spectator: info.isSpec,
                team_id: info.teamID,
                ally_team_id: info.allyTeamID,
                ping_time: info.pingTime,
                cpu_usage: info.cpuUsage,
                country: copy_c_string(info.country).unwrap_or_default(),
                rank: info.rank,
                has_skirmish_ais_in_team: info.hasSkirmishAIsInTeam,
                custom_keys: copy_c_string(info.customKeys),
                desynced: info.desynced,
            }
        }
    }
}
