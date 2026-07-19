use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, raw::copy_c_string, sys};

pub struct Vfs<'a> {
    api: &'a sys::VFSApi,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryEntry {
    pub name: String,
    pub is_directory: bool,
}

impl<'a> Vfs<'a> {
    pub(crate) fn new(api: &'a sys::VFSApi) -> Self {
        Self { api }
    }

    /// File names under `path`, as owned strings.
    ///
    /// The generated `dir_list` hands back `DirEntry`, whose `name` is a raw
    /// `*const c_char` -- reading it means `unsafe` at every call site. The
    /// binding owns that risk, not its callers, so it is unwrapped once here.
    pub fn dir_list_names(
        &self,
        path: &str,
        pattern: &str,
        mode: &str,
        recursive: bool,
    ) -> Result<Vec<String>, Error> {
        Ok(entry_names(
            self.dir_list(path, pattern, mode, recursive)?,
            false,
        ))
    }

    /// Directory names under `path`, as owned strings.
    pub fn list_dir_names(
        &self,
        path: &str,
        pattern: &str,
        mode: &str,
        recursive: bool,
    ) -> Result<Vec<String>, Error> {
        Ok(entry_names(
            self.list_dir(path, pattern, mode, recursive)?,
            true,
        ))
    }

    /// Files and directories under `path`, with all engine-owned strings copied.
    pub fn list_entries(
        &self,
        path: &str,
        pattern: &str,
        mode: &str,
        recursive: bool,
    ) -> Result<Vec<DirectoryEntry>, Error> {
        Ok(self
            .list_dir(path, pattern, mode, recursive)?
            .iter()
            .filter_map(directory_entry)
            .collect())
    }
}

fn directory_entry(entry: &sys::DirEntry) -> Option<DirectoryEntry> {
    // SAFETY: `DirEntry::name` is engine-owned and valid for this call.
    unsafe { copy_c_string(entry.name) }.map(|name| DirectoryEntry {
        name,
        is_directory: entry.isDirectory,
    })
}

fn entry_names(entries: Vec<sys::DirEntry>, directories: bool) -> Vec<String> {
    entries
        .iter()
        .filter(|entry| entry.isDirectory == directories)
        .filter_map(|entry| {
            // SAFETY: `DirEntry::name` is engine-owned and valid for this call.
            unsafe { copy_c_string(entry.name) }
        })
        .collect()
}

include!(concat!(env!("OUT_DIR"), "/vfs_generated.rs"));
