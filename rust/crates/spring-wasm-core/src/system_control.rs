//! Reviewed scoped system-control imports.
//!
//! Only `CallAsTeam` is exposed here. Process lifecycle/watchdog/restart
//! functions remain outside the production Core capability set.

use super::{ApiError, ErrorCode, Result, SyncCallback};

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:system-control")]
    extern "C" {
        #[link_name = "call-as-team"]
        pub fn call_as_team(team_id: i32, callback_id: i32, user_data: i32) -> i64;
    }
}

#[inline]
pub fn call_as_team(team_id: i32, callback: SyncCallback) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        return super::unpack_bool(unsafe {
            raw::call_as_team(team_id, callback.id as i32, callback.user_data as i32)
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (team_id, callback);
        Err(unreachable!())
    }
}
