use std::{ffi::CStr, mem::MaybeUninit, slice};

use crate::{error::Error, sys};

pub struct Vfs<'a> {
    api: &'a sys::VFSApi,
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
        Ok(entry_names(self.dir_list(path, pattern, mode, recursive)?, false))
    }

    /// Directory names under `path`, as owned strings.
    pub fn list_dir_names(
        &self,
        path: &str,
        pattern: &str,
        mode: &str,
        recursive: bool,
    ) -> Result<Vec<String>, Error> {
        Ok(entry_names(self.list_dir(path, pattern, mode, recursive)?, true))
    }
}

fn entry_names(entries: Vec<sys::DirEntry>, directories: bool) -> Vec<String> {
    entries
        .iter()
        .filter(|entry| entry.isDirectory == directories)
        .filter_map(|entry| {
            if entry.name.is_null() {
                return None;
            }
            // Safety: the engine hands back a NUL-terminated name that outlives
            // this call; it is copied out immediately.
            let name = unsafe { CStr::from_ptr(entry.name) };
            Some(name.to_string_lossy().into_owned())
        })
        .collect()
}

include!(concat!(env!("OUT_DIR"), "/vfs_generated.rs"));
