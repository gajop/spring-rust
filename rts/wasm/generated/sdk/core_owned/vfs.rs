    pub mod vfs {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, PartialEq)]
        pub struct AIInfoEntry {
            pub short_name: String,
            pub version: String,
            pub is_lua_ai: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AbortDownloadQuery {
            pub id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct AbortDownloadResult {
            pub removed: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ArchiveInfoEntry {
            pub key: String,
            pub type_: String,
            pub string_value: String,
            pub int_value: i32,
            pub float_value: f32,
            pub bool_value: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CalculateHashQuery {
            pub data: Vec<u8>,
            pub hash_type: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CalculateHashResult {
            pub hash: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CompressFolderQuery {
            pub folder_path: String,
            pub archive_type: String,
            pub compressed_file_path: String,
            pub include_folder: bool,
            pub mode: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct CompressFolderResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CreateDirQuery {
            pub path: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct CreateDirResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DirEntry {
            pub name: String,
            pub is_directory: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DownloadArchiveQuery {
            pub filename: String,
            pub category: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DownloadArchiveResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ExtractModArchiveFileQuery {
            pub path: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ExtractModArchiveFileResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FileExistsQuery {
            pub path: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct FileExistsResult {
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FileInfo {
            pub name: String,
            pub size: u32,
            pub mode: u32,
            pub is_directory: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetAllArchivesQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAllArchivesResult {
            pub archives: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetArchiveChecksumQuery {
            pub archive_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetArchiveChecksumResult {
            pub single_checksum: String,
            pub complete_checksum: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetArchiveContainingFileQuery {
            pub path: String,
            pub mode: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetArchiveContainingFileResult {
            pub archive_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetArchiveDependenciesQuery {
            pub archive_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetArchiveDependenciesResult {
            pub archives: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetArchiveInfoQuery {
            pub archive_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetArchiveInfoResult {
            pub entries: Vec<ArchiveInfoEntry>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetArchivePathQuery {
            pub archive_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetArchivePathResult {
            pub path: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetArchiveReplacesQuery {
            pub archive_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetArchiveReplacesResult {
            pub archives: Vec<String>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetArchivesQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetArchivesResult {
            pub archives: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAvailableAIsQuery {
            pub game_archive_name: String,
            pub map_archive_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAvailableAIsResult {
            pub ais: Vec<AIInfoEntry>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFileAbsolutePathQuery {
            pub path: String,
            pub mode: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFileAbsolutePathResult {
            pub path: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFileInfoQuery {
            pub path: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFileInfoResult {
            pub info: FileInfo,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFileSizeQuery {
            pub path: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFileSizeResult {
            pub size: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGamesQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGamesResult {
            pub games: Vec<String>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetLoadedArchivesQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetLoadedArchivesResult {
            pub archives: Vec<String>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetMapSquareTextureInfoQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetMapSquareTextureInfoResult {
            pub square_size: i32,
            pub num_squares_x: i32,
            pub num_squares_z: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMapSquareTextureQuery {
            pub tex_square_x: i32,
            pub tex_square_y: i32,
            pub lod_min: i32,
            pub texture_name: String,
            pub lod_max: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetMapSquareTextureResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetMapsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMapsResult {
            pub maps: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetNameFromRapidTagQuery {
            pub rapid_tag: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetNameFromRapidTagResult {
            pub archive_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct HasArchiveQuery {
            pub archive_name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct HasArchiveResult {
            pub has_archive: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsDirectoryQuery {
            pub path: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsDirectoryResult {
            pub is_directory: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ListDirQuery {
            pub path: String,
            pub pattern: String,
            pub mode: String,
            pub recursive: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ListDirResult {
            pub entries: Vec<DirEntry>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct LoadFileQuery {
            pub path: String,
            pub mode: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct LoadFileResult {
            pub data: Vec<u8>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PackF32Query {
            pub values: Vec<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PackF32Result {
            pub data: Vec<u8>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PackS16Query {
            pub values: Vec<i16>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PackS16Result {
            pub data: Vec<u8>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PackS32Query {
            pub values: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PackS32Result {
            pub data: Vec<u8>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PackS8Query {
            pub values: Vec<i8>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PackS8Result {
            pub data: Vec<u8>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PackU16Query {
            pub values: Vec<u16>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PackU16Result {
            pub data: Vec<u8>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PackU32Query {
            pub values: Vec<u32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PackU32Result {
            pub data: Vec<u8>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PackU8Query {
            pub values: Vec<u8>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PackU8Result {
            pub data: Vec<u8>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ReadFileAsStringQuery {
            pub path: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ReadFileAsStringResult {
            pub content: String,
            pub content_length: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ReadFileQuery {
            pub path: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ReadFileResult {
            pub data: Vec<u8>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ScanAllDirsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ScanAllDirsResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetMapSquareTextureQuery {
            pub tex_square_x: i32,
            pub tex_square_y: i32,
            pub texture_name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetMapSquareTextureResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SubDirsQuery {
            pub path: String,
            pub pattern: String,
            pub mode: String,
            pub recursive: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SubDirsResult {
            pub dirs: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnpackF32Query {
            pub data: Vec<u8>,
            pub byte_offset: u32,
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnpackF32Result {
            pub values: Vec<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnpackS16Query {
            pub data: Vec<u8>,
            pub byte_offset: u32,
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnpackS16Result {
            pub values: Vec<i16>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnpackS32Query {
            pub data: Vec<u8>,
            pub byte_offset: u32,
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnpackS32Result {
            pub values: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnpackS8Query {
            pub data: Vec<u8>,
            pub byte_offset: u32,
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnpackS8Result {
            pub values: Vec<i8>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnpackU16Query {
            pub data: Vec<u8>,
            pub byte_offset: u32,
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnpackU16Result {
            pub values: Vec<u16>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnpackU32Query {
            pub data: Vec<u8>,
            pub byte_offset: u32,
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnpackU32Result {
            pub values: Vec<u32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnpackU8Query {
            pub data: Vec<u8>,
            pub byte_offset: u32,
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnpackU8Result {
            pub values: Vec<u8>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UseArchiveQuery {
            pub archive_name: String,
            pub callback: u32,
            pub user_data: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct UseArchiveResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ZlibCompressQuery {
            pub data: Vec<u8>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ZlibCompressResult {
            pub data: Vec<u8>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ZlibDecompressQuery {
            pub data: Vec<u8>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ZlibDecompressResult {
            pub data: Vec<u8>,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetArchiveChecksumValue {
            pub single_checksum: String,
            pub complete_checksum: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFileInfoValue {
            pub info: FileInfo,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetMapSquareTextureInfoValue {
            pub square_size: i32,
            pub num_squares_x: i32,
            pub num_squares_z: i32,
        }

        #[inline]
        pub fn abort_download(id: i32) -> Result<bool> {
            let value = crate::generated::vfs::abort_download(id)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_calculate_hash {
            #[link(wasm_import_module = "spring:vfs")]
            unsafe extern "C" {
                #[link_name = "calculate-hash"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.calculate-hash."]
        #[doc(hidden)]
        #[inline]
        pub fn calculate_hash(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_calculate_hash::call(p0, p1, p2)
        }

        #[inline]
        pub fn compress_folder(folder_path: &str, archive_type: &str, compressed_file_path: &str, include_folder: bool, mode: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(folder_path, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(folder_path)?),
            };
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(archive_type, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(archive_type)?),
            };
            let mut __core_string_2_scratch = [0u8; 256];
            let __core_string_2_buf = match super::write_cstr(compressed_file_path, &mut __core_string_2_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(compressed_file_path)?),
            };
            let mut __core_string_4_scratch = [0u8; 256];
            let __core_string_4_buf = match super::write_cstr(mode, &mut __core_string_4_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(mode)?),
            };
            crate::generated::borrowed::vfs::compress_folder(__core_string_0_buf.as_cstr(), __core_string_1_buf.as_cstr(), __core_string_2_buf.as_cstr(), include_folder, __core_string_4_buf.as_cstr())
        }

        #[inline]
        pub fn create_dir(path: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(path, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(path)?),
            };
            crate::generated::borrowed::vfs::create_dir(__core_string_0_buf.as_cstr())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_dir_list {
            #[link(wasm_import_module = "spring:vfs")]
            unsafe extern "C" {
                #[link_name = "dir-list"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.dir-list."]
        #[doc(hidden)]
        #[inline]
        pub fn dir_list(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_dir_list::call(p0, p1, p2)
        }

        #[inline]
        pub fn download_archive(filename: &str, category: &str) -> Result<()> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(filename, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(filename)?),
            };
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(category, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(category)?),
            };
            crate::generated::borrowed::vfs::download_archive(__core_string_0_buf.as_cstr(), __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn extract_mod_archive_file(path: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(path, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(path)?),
            };
            crate::generated::borrowed::vfs::extract_mod_archive_file(__core_string_0_buf.as_cstr())
        }

        #[inline]
        pub fn file_exists(path: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(path, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(path)?),
            };
            crate::generated::borrowed::vfs::file_exists(__core_string_0_buf.as_cstr())
        }

        #[inline]
        pub fn get_all_archives(unused: u8) -> Result<Vec<String>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::vfs::get_all_archives(unused as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items };
                        if !crate::generated::__core_wire::finish(&__output, &mut __cursor, 8) {
                            return Err(crate::ApiError::new(crate::ErrorCode::Internal as i32));
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_archive_checksum {
            #[link(wasm_import_module = "spring:vfs")]
            unsafe extern "C" {
                #[link_name = "get-archive-checksum"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.get-archive-checksum."]
        #[doc(hidden)]
        #[inline]
        pub fn get_archive_checksum(p0: i32, p1: i32) -> i32 {
            __core_owned_get_archive_checksum::call(p0, p1)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_archive_containing_file {
            #[link(wasm_import_module = "spring:vfs")]
            unsafe extern "C" {
                #[link_name = "get-archive-containing-file"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.get-archive-containing-file."]
        #[doc(hidden)]
        #[inline]
        pub fn get_archive_containing_file(p0: i32, p1: i32) -> i32 {
            __core_owned_get_archive_containing_file::call(p0, p1)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_archive_dependencies {
            #[link(wasm_import_module = "spring:vfs")]
            unsafe extern "C" {
                #[link_name = "get-archive-dependencies"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.get-archive-dependencies."]
        #[doc(hidden)]
        #[inline]
        pub fn get_archive_dependencies(p0: i32, p1: i32) -> i32 {
            __core_owned_get_archive_dependencies::call(p0, p1)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_archive_info {
            #[link(wasm_import_module = "spring:vfs")]
            unsafe extern "C" {
                #[link_name = "get-archive-info"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.get-archive-info."]
        #[doc(hidden)]
        #[inline]
        pub fn get_archive_info(p0: i32, p1: i32) -> i32 {
            __core_owned_get_archive_info::call(p0, p1)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_archive_path {
            #[link(wasm_import_module = "spring:vfs")]
            unsafe extern "C" {
                #[link_name = "get-archive-path"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.get-archive-path."]
        #[doc(hidden)]
        #[inline]
        pub fn get_archive_path(p0: i32, p1: i32) -> i32 {
            __core_owned_get_archive_path::call(p0, p1)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_archive_replaces {
            #[link(wasm_import_module = "spring:vfs")]
            unsafe extern "C" {
                #[link_name = "get-archive-replaces"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.get-archive-replaces."]
        #[doc(hidden)]
        #[inline]
        pub fn get_archive_replaces(p0: i32, p1: i32) -> i32 {
            __core_owned_get_archive_replaces::call(p0, p1)
        }

        #[inline]
        pub fn get_archives(unused: u8) -> Result<Vec<String>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::vfs::get_archives(unused as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items };
                        if !crate::generated::__core_wire::finish(&__output, &mut __cursor, 8) {
                            return Err(crate::ApiError::new(crate::ErrorCode::Internal as i32));
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_available_a_is {
            #[link(wasm_import_module = "spring:vfs")]
            unsafe extern "C" {
                #[link_name = "get-available-a-is"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.get-available-a-is."]
        #[doc(hidden)]
        #[inline]
        pub fn get_available_a_is(p0: i32, p1: i32) -> i32 {
            __core_owned_get_available_a_is::call(p0, p1)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_file_absolute_path {
            #[link(wasm_import_module = "spring:vfs")]
            unsafe extern "C" {
                #[link_name = "get-file-absolute-path"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.get-file-absolute-path."]
        #[doc(hidden)]
        #[inline]
        pub fn get_file_absolute_path(p0: i32, p1: i32) -> i32 {
            __core_owned_get_file_absolute_path::call(p0, p1)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_file_info {
            #[link(wasm_import_module = "spring:vfs")]
            unsafe extern "C" {
                #[link_name = "get-file-info"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.get-file-info."]
        #[doc(hidden)]
        #[inline]
        pub fn get_file_info(p0: i32, p1: i32) -> i32 {
            __core_owned_get_file_info::call(p0, p1)
        }

        #[inline]
        pub fn get_file_size(path: &str) -> Result<u32> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(path, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(path)?),
            };
            crate::generated::borrowed::vfs::get_file_size(__core_string_0_buf.as_cstr())
        }

        #[inline]
        pub fn get_games(unused: u8) -> Result<Vec<String>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::vfs::get_games(unused as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items };
                        if !crate::generated::__core_wire::finish(&__output, &mut __cursor, 8) {
                            return Err(crate::ApiError::new(crate::ErrorCode::Internal as i32));
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn get_loaded_archives(unused: u8) -> Result<Vec<String>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::vfs::get_loaded_archives(unused as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items };
                        if !crate::generated::__core_wire::finish(&__output, &mut __cursor, 8) {
                            return Err(crate::ApiError::new(crate::ErrorCode::Internal as i32));
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn get_map_square_texture(tex_square_x: i32, tex_square_y: i32, lod_min: i32, texture_name: &str, lod_max: i32) -> Result<bool> {
            let mut __core_string_3_scratch = [0u8; 256];
            let __core_string_3_buf = match super::write_cstr(texture_name, &mut __core_string_3_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(texture_name)?),
            };
            crate::generated::borrowed::vfs::get_map_square_texture(tex_square_x, tex_square_y, lod_min, __core_string_3_buf.as_cstr(), lod_max)
        }

        #[inline]
        pub fn get_map_square_texture_info(unused: u8) -> Result<GetMapSquareTextureInfoValue> {
            let value = crate::generated::vfs::get_map_square_texture_info(unused)?;
            Ok(GetMapSquareTextureInfoValue {
                square_size: value.0,
                num_squares_x: value.1,
                num_squares_z: value.2
            })
        }

        #[inline]
        pub fn get_maps(unused: u8) -> Result<Vec<String>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::vfs::get_maps(unused as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items };
                        if !crate::generated::__core_wire::finish(&__output, &mut __cursor, 8) {
                            return Err(crate::ApiError::new(crate::ErrorCode::Internal as i32));
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_name_from_rapid_tag {
            #[link(wasm_import_module = "spring:vfs")]
            unsafe extern "C" {
                #[link_name = "get-name-from-rapid-tag"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.get-name-from-rapid-tag."]
        #[doc(hidden)]
        #[inline]
        pub fn get_name_from_rapid_tag(p0: i32, p1: i32) -> i32 {
            __core_owned_get_name_from_rapid_tag::call(p0, p1)
        }

        #[inline]
        pub fn has_archive(archive_name: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(archive_name, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(archive_name)?),
            };
            crate::generated::borrowed::vfs::has_archive(__core_string_0_buf.as_cstr())
        }

        #[inline]
        pub fn is_directory(path: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(path, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(path)?),
            };
            crate::generated::borrowed::vfs::is_directory(__core_string_0_buf.as_cstr())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_list_dir {
            #[link(wasm_import_module = "spring:vfs")]
            unsafe extern "C" {
                #[link_name = "list-dir"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.list-dir."]
        #[doc(hidden)]
        #[inline]
        pub fn list_dir(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_list_dir::call(p0, p1, p2)
        }

        #[inline]
        pub fn load_file(path: &str, mode: &str) -> Result<Vec<u8>> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + path.len()); __b.extend_from_slice(&(path.len() as u32).to_le_bytes()); __b.extend_from_slice(path.as_bytes()); __b };
            let __blob1 = { let mut __b = Vec::with_capacity(4 + mode.len()); __b.extend_from_slice(&(mode.len() as u32).to_le_bytes()); __b.extend_from_slice(mode.as_bytes()); __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::vfs::load_file(&__blob0, &__blob1, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 4);
                        let mut __result = Vec::<u8>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as u8);
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 4, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn pack_f32(values: &[f32]) -> Result<Vec<u8>> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(values.len() as u32).to_le_bytes()); for __item in values.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.to_bits().to_le_bytes());} __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::vfs::pack_f32(&__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 4);
                        let mut __result = Vec::<u8>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as u8);
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 4, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn pack_s16(values: &[i16]) -> Result<Vec<u8>> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(values.len() as u32).to_le_bytes()); for __item in values.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item as u32).to_le_bytes());} __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::vfs::pack_s16(&__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 4);
                        let mut __result = Vec::<u8>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as u8);
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 4, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn pack_s32(values: &[i32]) -> Result<Vec<u8>> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(values.len() as u32).to_le_bytes()); for __item in values.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.to_le_bytes());} __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::vfs::pack_s32(&__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 4);
                        let mut __result = Vec::<u8>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as u8);
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 4, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn pack_s8(values: &[i8]) -> Result<Vec<u8>> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(values.len() as u32).to_le_bytes()); for __item in values.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item as u32).to_le_bytes());} __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::vfs::pack_s8(&__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 4);
                        let mut __result = Vec::<u8>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as u8);
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 4, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn pack_u16(values: &[u16]) -> Result<Vec<u8>> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(values.len() as u32).to_le_bytes()); for __item in values.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item as u32).to_le_bytes());} __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::vfs::pack_u16(&__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 4);
                        let mut __result = Vec::<u8>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as u8);
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 4, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn pack_u32(values: &[u32]) -> Result<Vec<u8>> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(values.len() as u32).to_le_bytes()); for __item in values.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.to_le_bytes());} __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::vfs::pack_u32(&__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 4);
                        let mut __result = Vec::<u8>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as u8);
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 4, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn pack_u8(values: &[u8]) -> Result<Vec<u8>> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(values.len() as u32).to_le_bytes()); for __item in values.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item as u32).to_le_bytes());} __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::vfs::pack_u8(&__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 4);
                        let mut __result = Vec::<u8>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as u8);
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 4, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn read_file(path: &str) -> Result<Vec<u8>> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + path.len()); __b.extend_from_slice(&(path.len() as u32).to_le_bytes()); __b.extend_from_slice(path.as_bytes()); __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::vfs::read_file(&__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 4);
                        let mut __result = Vec::<u8>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as u8);
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 4, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_read_file_as_string {
            #[link(wasm_import_module = "spring:vfs")]
            unsafe extern "C" {
                #[link_name = "read-file-as-string"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.read-file-as-string."]
        #[doc(hidden)]
        #[inline]
        pub fn read_file_as_string(p0: i32, p1: i32) -> i32 {
            __core_owned_read_file_as_string::call(p0, p1)
        }

        #[inline]
        pub fn scan_all_dirs(unused: u8) -> Result<()> {
            crate::generated::vfs::scan_all_dirs(unused)?;
            Ok(())
        }

        #[inline]
        pub fn set_map_square_texture(tex_square_x: i32, tex_square_y: i32, texture_name: &str) -> Result<bool> {
            let mut __core_string_2_scratch = [0u8; 256];
            let __core_string_2_buf = match super::write_cstr(texture_name, &mut __core_string_2_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(texture_name)?),
            };
            crate::generated::borrowed::vfs::set_map_square_texture(tex_square_x, tex_square_y, __core_string_2_buf.as_cstr())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_sub_dirs {
            #[link(wasm_import_module = "spring:vfs")]
            unsafe extern "C" {
                #[link_name = "sub-dirs"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.sub-dirs."]
        #[doc(hidden)]
        #[inline]
        pub fn sub_dirs(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_sub_dirs::call(p0, p1, p2)
        }

        #[inline]
        pub fn unpack_f32(data: &[u8], byte_offset: u32, count: u32) -> Result<Vec<f32>> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(data.len() as u32).to_le_bytes()); for __item in data.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item as u32).to_le_bytes());} __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::vfs::unpack_f32(byte_offset as i32, count as i32, &__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 4);
                        let mut __result = Vec::<f32>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?);
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 4, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn unpack_s16(data: &[u8], byte_offset: u32, count: u32) -> Result<Vec<i16>> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(data.len() as u32).to_le_bytes()); for __item in data.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item as u32).to_le_bytes());} __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::vfs::unpack_s16(byte_offset as i32, count as i32, &__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 4);
                        let mut __result = Vec::<i16>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as i16);
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 4, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn unpack_s32(data: &[u8], byte_offset: u32, count: u32) -> Result<Vec<i32>> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(data.len() as u32).to_le_bytes()); for __item in data.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item as u32).to_le_bytes());} __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::vfs::unpack_s32(byte_offset as i32, count as i32, &__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 4);
                        let mut __result = Vec::<i32>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?);
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 4, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn unpack_s8(data: &[u8], byte_offset: u32, count: u32) -> Result<Vec<i8>> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(data.len() as u32).to_le_bytes()); for __item in data.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item as u32).to_le_bytes());} __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::vfs::unpack_s8(byte_offset as i32, count as i32, &__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 4);
                        let mut __result = Vec::<i8>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as i8);
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 4, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn unpack_u16(data: &[u8], byte_offset: u32, count: u32) -> Result<Vec<u16>> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(data.len() as u32).to_le_bytes()); for __item in data.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item as u32).to_le_bytes());} __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::vfs::unpack_u16(byte_offset as i32, count as i32, &__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 4);
                        let mut __result = Vec::<u16>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as u16);
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 4, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn unpack_u32(data: &[u8], byte_offset: u32, count: u32) -> Result<Vec<u32>> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(data.len() as u32).to_le_bytes()); for __item in data.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item as u32).to_le_bytes());} __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::vfs::unpack_u32(byte_offset as i32, count as i32, &__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 4);
                        let mut __result = Vec::<u32>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?);
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 4, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn unpack_u8(data: &[u8], byte_offset: u32, count: u32) -> Result<Vec<u8>> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(data.len() as u32).to_le_bytes()); for __item in data.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item as u32).to_le_bytes());} __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::vfs::unpack_u8(byte_offset as i32, count as i32, &__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 4);
                        let mut __result = Vec::<u8>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as u8);
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 4, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_use_archive {
            #[link(wasm_import_module = "spring:vfs")]
            unsafe extern "C" {
                #[link_name = "use-archive"]
                pub safe fn call(p0: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.use-archive."]
        #[doc(hidden)]
        #[inline]
        pub fn use_archive(p0: i32) -> i64 {
            __core_owned_use_archive::call(p0)
        }

        #[inline]
        pub fn zlib_compress(data: &[u8]) -> Result<Vec<u8>> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(data.len() as u32).to_le_bytes()); for __item in data.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item as u32).to_le_bytes());} __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::vfs::zlib_compress(&__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 4);
                        let mut __result = Vec::<u8>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as u8);
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 4, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn zlib_decompress(data: &[u8]) -> Result<Vec<u8>> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(data.len() as u32).to_le_bytes()); for __item in data.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item as u32).to_le_bytes());} __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::vfs::zlib_decompress(&__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 4);
                        let mut __result = Vec::<u8>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as u8);
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 4, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

    }

