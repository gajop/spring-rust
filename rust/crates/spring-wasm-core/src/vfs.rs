use super::{ApiError, ErrorCode, Result, SyncCallback};

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:vfs")]
    extern "C" {
        #[link_name = "use-archive"]
        pub fn use_archive(
            archive_ptr: i32,
            archive_len: i32,
            callback_id: i32,
            user_data: i32,
        ) -> i64;
    }
}

#[inline]
pub fn use_archive(archive: &str, callback: SyncCallback) -> Result<bool> {
    #[cfg(target_arch = "wasm32")]
    {
        let pointer = archive.as_ptr() as usize;
        if pointer > u32::MAX as usize || archive.len() > u32::MAX as usize {
            return Err(ApiError::new(ErrorCode::InvalidArgument as i32));
        }
        return super::unpack_bool(unsafe {
            raw::use_archive(
                pointer as u32 as i32,
                archive.len() as u32 as i32,
                callback.id as i32,
                callback.user_data as i32,
            )
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (archive, callback);
        Err(ApiError::new(ErrorCode::UnsupportedHostTarget as i32))
    }
}
