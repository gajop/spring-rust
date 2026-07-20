use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, raw::copy_c_string, sys};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameModInfo {
    pub game_name: String,
    pub game_short_name: String,
    pub game_version: String,
    pub game_mutator: String,
    pub game_description: String,
    pub mod_name: String,
    pub mod_short_name: String,
    pub mod_version: String,
    pub mod_mutator: String,
    pub mod_description: String,
    pub mod_checksum: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameMapInfo {
    pub map_name: String,
    pub map_description: String,
    pub map_x: i32,
    pub map_y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SideData {
    pub name: String,
    pub case_name: String,
    pub start_unit: String,
    pub index: u32,
}

pub struct Game<'a> {
    api: &'a sys::GameApi,
}

impl<'a> Game<'a> {
    pub(crate) fn new(api: &'a sys::GameApi) -> Self {
        Self { api }
    }

    pub fn get_game_mod_info_owned(&self) -> Result<GameModInfo, Error> {
        self.get_game_mod_info().map(GameModInfo::from_raw)
    }

    pub fn get_game_map_info_owned(&self) -> Result<GameMapInfo, Error> {
        self.get_game_map_info().map(GameMapInfo::from_raw)
    }

    pub fn get_side_data_owned(&self, side_name: &str) -> Result<SideData, Error> {
        self.get_side_data(side_name).map(SideData::from_raw)
    }

    pub fn get_side_data_by_index_owned(&self, side_index: u32) -> Result<SideData, Error> {
        self.get_side_data_by_index(side_index)
            .map(SideData::from_raw)
    }
}

impl GameModInfo {
    fn from_raw(info: sys::GameModInfo) -> Self {
        // SAFETY: every pointer belongs to the engine and remains valid while
        // this result is copied into owned Rust strings.
        unsafe {
            Self {
                game_name: copy_c_string(info.gameName).unwrap_or_default(),
                game_short_name: copy_c_string(info.gameShortName).unwrap_or_default(),
                game_version: copy_c_string(info.gameVersion).unwrap_or_default(),
                game_mutator: copy_c_string(info.gameMutator).unwrap_or_default(),
                game_description: copy_c_string(info.gameDesc).unwrap_or_default(),
                mod_name: copy_c_string(info.modName).unwrap_or_default(),
                mod_short_name: copy_c_string(info.modShortName).unwrap_or_default(),
                mod_version: copy_c_string(info.modVersion).unwrap_or_default(),
                mod_mutator: copy_c_string(info.modMutator).unwrap_or_default(),
                mod_description: copy_c_string(info.modDesc).unwrap_or_default(),
                mod_checksum: copy_c_string(info.modChecksum).unwrap_or_default(),
            }
        }
    }
}

impl GameMapInfo {
    fn from_raw(info: sys::GameMapInfo) -> Self {
        // SAFETY: every pointer belongs to the engine and remains valid while
        // this result is copied into owned Rust strings.
        unsafe {
            Self {
                map_name: copy_c_string(info.mapName).unwrap_or_default(),
                map_description: copy_c_string(info.mapDescription).unwrap_or_default(),
                map_x: info.mapX,
                map_y: info.mapY,
            }
        }
    }
}

impl SideData {
    fn from_raw(data: sys::SideData) -> Self {
        // SAFETY: every pointer belongs to the engine and remains valid while
        // this result is copied into owned Rust strings.
        unsafe {
            Self {
                name: copy_c_string(data.sideName).unwrap_or_default(),
                case_name: copy_c_string(data.caseName).unwrap_or_default(),
                start_unit: copy_c_string(data.startUnit).unwrap_or_default(),
                index: data.sideIndex,
            }
        }
    }
}

include!(concat!(env!("OUT_DIR"), "/game_generated.rs"));
