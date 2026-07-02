use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

#[derive(Debug, Clone)]
pub struct CommandDescription {
    pub id: i32,
    pub action: String,
    pub command_type: CommandType,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandType {
    Icon,
    IconMode,
    IconMap,
    IconArea,
    IconUnit,
    IconUnitOrMap,
    IconFront,
    IconUnitOrArea,
    Next,
    Prev,
    IconUnitFeatureOrArea,
    IconBuilding,
    Custom,
    IconUnitOrRectangle,
    Number,
    Unknown(i32),
}

pub struct UnitsCommands<'a> {
    api: &'a sys::UnitsCommandsApi,
}

impl<'a> UnitsCommands<'a> {
    pub(crate) fn new(api: &'a sys::UnitsCommandsApi) -> Self {
        Self { api }
    }

    pub fn get_unit_command_descriptions(
        &self,
        unit_id: i32,
    ) -> Result<Vec<CommandDescription>, Error> {
        self.get_unit_cmd_descs(unit_id).map(|descs| {
            descs
                .into_iter()
                .map(|desc| CommandDescription {
                    id: desc.cmdID,
                    action: c_string(desc.action),
                    command_type: CommandType::from(desc.type_),
                    name: c_string(desc.name),
                })
                .collect()
        })
    }
}

include!(concat!(env!("OUT_DIR"), "/units_commands_generated.rs"));

fn c_string(ptr: *const std::ffi::c_char) -> String {
    if ptr.is_null() {
        String::new()
    } else {
        unsafe { CStr::from_ptr(ptr).to_string_lossy().into_owned() }
    }
}

impl From<i32> for CommandType {
    fn from(value: i32) -> Self {
        match value {
            0 => CommandType::Icon,
            5 => CommandType::IconMode,
            10 => CommandType::IconMap,
            11 => CommandType::IconArea,
            12 => CommandType::IconUnit,
            13 => CommandType::IconUnitOrMap,
            14 => CommandType::IconFront,
            16 => CommandType::IconUnitOrArea,
            17 => CommandType::Next,
            18 => CommandType::Prev,
            19 => CommandType::IconUnitFeatureOrArea,
            20 => CommandType::IconBuilding,
            21 => CommandType::Custom,
            22 => CommandType::IconUnitOrRectangle,
            23 => CommandType::Number,
            other => CommandType::Unknown(other),
        }
    }
}
