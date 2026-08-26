//! Reviewed scoped system-control imports.
//!
//! Only `CallAsTeam` is exposed here. Process lifecycle/watchdog/restart
//! functions remain outside the production Core capability set.



#[cfg(feature = "alloc")]
pub use crate::owned::system_control::{clear_watch_dog_timer, garbage_collect_ctrl, get_game_name, get_game_state, get_gather_mode, get_menu_name, get_replay_file_path, get_replay_length, get_replay_recording_file_path, get_video_capturing_mode, get_window_display_mode, is_replay, ping, quit, reload, request_start_position, restart, set_share_level, share_resources, start, yield_};

use super::{Result, SyncCallback};

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:system-control")]
    unsafe extern "C" {
        #[link_name = "call-as-team"]
        pub fn call_as_team(team_id: i32, callback_id: i32, user_data: i32) -> i64;
    }
}

#[inline]
pub fn call_as_team(team_id: i32, callback: SyncCallback) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        super::unpack_bool(unsafe {
            raw::call_as_team(team_id, callback.id as i32, callback.user_data as i32)
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (team_id, callback);
        Err(unreachable!())
    }
}
