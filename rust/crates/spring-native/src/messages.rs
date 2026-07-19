use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, raw::copy_c_string, sys};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsoleEntry {
    pub text: String,
    pub priority: u32,
}

pub struct Messages<'a> {
    api: &'a sys::MessagesApi,
}

impl<'a> Messages<'a> {
    pub(crate) fn new(api: &'a sys::MessagesApi) -> Self {
        Self { api }
    }

    pub fn get_console_entries(&self, max_lines: u32) -> Result<Vec<ConsoleEntry>, Error> {
        self.get_console_buffer(max_lines).map(|entries| {
            entries
                .into_iter()
                .filter_map(ConsoleEntry::from_raw)
                .collect()
        })
    }
}

impl ConsoleEntry {
    fn from_raw(entry: sys::ConsoleEntry) -> Option<Self> {
        // SAFETY: `ConsoleEntry::text` is engine-owned and valid for this call.
        unsafe { copy_c_string(entry.text) }.map(|text| Self {
            text,
            priority: entry.priority,
        })
    }
}

include!(concat!(env!("OUT_DIR"), "/messages_generated.rs"));
