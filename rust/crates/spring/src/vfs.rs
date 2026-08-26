

#[cfg(feature = "alloc")]
pub use crate::owned::vfs::{abort_download, calculate_hash, compress_folder, create_dir, dir_list, download_archive, extract_mod_archive_file, file_exists, get_all_archives, get_archive_checksum, get_archive_containing_file, get_archive_dependencies, get_archive_info, get_archive_path, get_archive_replaces, get_archives, get_available_a_is, get_file_absolute_path, get_file_info, get_file_size, get_games, get_loaded_archives, get_maps, get_map_square_texture, get_map_square_texture_info, get_name_from_rapid_tag, has_archive, is_directory, list_dir, load_file, pack_f32, pack_s16, pack_s32, pack_s8, pack_u16, pack_u32, pack_u8, read_file, read_file_as_string, scan_all_dirs, set_map_square_texture, sub_dirs, unpack_f32, unpack_s16, unpack_s32, unpack_s8, unpack_u16, unpack_u32, unpack_u8, zlib_compress, zlib_decompress};

use super::{ApiError, ErrorCode, Result, SyncCallback};

#[cfg(target_arch = "wasm32")]
mod raw {
    #[link(wasm_import_module = "spring:vfs")]
    unsafe extern "C" {
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
        super::unpack_bool(unsafe {
            raw::use_archive(
                pointer as u32 as i32,
                archive.len() as u32 as i32,
                callback.id as i32,
                callback.user_data as i32,
            )
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (archive, callback);
        Err(unreachable!())
    }
}
