use std::{mem::MaybeUninit, slice};

use crate::{error::Error, raw::copy_c_string, sys};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RosterEntry {
    pub name: String,
    pub player_id: i32,
    pub team_id: i32,
    pub ally_team_id: i32,
    pub is_ai: bool,
    pub is_spectator: bool,
    pub is_active: bool,
    pub ping_time: u32,
    pub cpu_usage: u32,
    pub country: String,
    pub rank: i32,
}

pub struct Player<'a> {
    api: &'a sys::PlayerApi,
}

impl<'a> Player<'a> {
    pub(crate) fn new(api: &'a sys::PlayerApi) -> Self {
        Self { api }
    }

    pub fn get_player_roster_owned(
        &self,
        sort_mode: i32,
        show_pathing_players: bool,
    ) -> Result<Vec<RosterEntry>, Error> {
        self.get_player_roster(sort_mode, show_pathing_players)
            .map(|entries| entries.into_iter().map(RosterEntry::from_raw).collect())
    }
}

impl RosterEntry {
    fn from_raw(entry: sys::RosterEntry) -> Self {
        // SAFETY: roster strings are engine-owned and valid for this call.
        unsafe {
            Self {
                name: copy_c_string(entry.name).unwrap_or_default(),
                player_id: entry.playerID,
                team_id: entry.teamID,
                ally_team_id: entry.allyTeamID,
                is_ai: entry.isAI,
                is_spectator: entry.isSpec,
                is_active: entry.isActive,
                ping_time: entry.pingTime,
                cpu_usage: entry.cpuUsage,
                country: copy_c_string(entry.country).unwrap_or_default(),
                rank: entry.rank,
            }
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/player_generated.rs"));
