use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

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
            side: c_string(info.side).unwrap_or_default(),
            color: info.color,
            custom_keys: c_string(info.customKeys),
        })
    }
}

include!(concat!(env!("OUT_DIR"), "/teams_generated.rs"));

fn c_string(ptr: *const std::ffi::c_char) -> Option<String> {
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(ptr) }.to_string_lossy().into_owned())
    }
}
