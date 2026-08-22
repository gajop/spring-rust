    pub mod vfs {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum CommonErrorCode {
            ErrorAlreadyExists,
            ErrorBufferOverflow,
            ErrorInternal,
            ErrorInvalidArgument,
            ErrorInvalidId,
            ErrorInvalidState,
            ErrorNone,
            ErrorNotAvailable,
            ErrorNotFound,
            ErrorOperationFailed,
            ErrorOutOfBounds,
            ErrorPermissionDenied,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AIInfoEntry {
            pub short_name: String,
            pub version: String,
            pub is_lua_ai: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct AbortDownloadQuery {
            pub id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
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
        pub struct AtmosphereParams {
            pub fog_color: Option<Vec<f32>>,
            pub sky_color: Option<Vec<f32>>,
            pub sun_color: Option<Vec<f32>>,
            pub cloud_color: Option<Vec<f32>>,
            pub sky_axis_angle: Option<Vec<f32>>,
            pub fog_start: Option<f32>,
            pub fog_end: Option<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct BoolResult {
            pub value: bool,
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
        pub struct CollisionVolumeData {
            pub scale_x: f32,
            pub scale_y: f32,
            pub scale_z: f32,
            pub offset_x: f32,
            pub offset_y: f32,
            pub offset_z: f32,
            pub volume_type: i32,
            pub test_type: i32,
            pub primary_axis: i32,
            pub disabled: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CompressFolderQuery {
            pub folder_path: String,
            pub archive_type: String,
            pub compressed_file_path: String,
            pub include_folder: bool,
            pub mode: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CompressFolderResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CreateDirQuery {
            pub path: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CreateDirResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DefRef {
            pub name: String,
            pub id: i32,
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct DownloadArchiveResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Error {
            pub code: i32,
            pub message: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ExtractModArchiveFileQuery {
            pub path: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ExtractModArchiveFileResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FileExistsQuery {
            pub path: String,
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float2 {
            pub x: f32,
            pub y: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float2Result {
            pub value: Float2,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float3 {
            pub x: f32,
            pub y: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float3Array {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float3Result {
            pub value: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float4 {
            pub x: f32,
            pub y: f32,
            pub z: f32,
            pub w: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Float4Result {
            pub value: Float4,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FloatArray {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FloatResult {
            pub value: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFileSizeResult {
            pub size: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGamesQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGamesResult {
            pub games: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetLoadedArchivesQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetLoadedArchivesResult {
            pub archives: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMapSquareTextureInfoQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMapSquareTextureResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct HasArchiveResult {
            pub has_archive: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Int2 {
            pub x: i32,
            pub y: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Int3 {
            pub x: i32,
            pub y: i32,
            pub z: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Int32Array {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Int32Result {
            pub value: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsDirectoryQuery {
            pub path: String,
        }

        #[derive(Debug, Clone, PartialEq)]
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
        pub struct MapRenderingParams {
            pub splat_tex_scales: Option<Vec<f32>>,
            pub splat_tex_mults: Option<Vec<f32>>,
            pub void_water: Option<bool>,
            pub void_ground: Option<bool>,
            pub splat_detail_normal_diffuse_alpha: Option<bool>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct NativeExplosionParams {
            pub damages: f32,
            pub weapon_def_id: i32,
            pub owner_id: i32,
            pub hit_unit_id: i32,
            pub hit_feature_id: i32,
            pub crater_area_of_effect: f32,
            pub damage_area_of_effect: f32,
            pub edge_effectiveness: f32,
            pub explosion_speed: f32,
            pub gfx_mod: f32,
            pub impact_only: bool,
            pub ignore_owner: bool,
            pub damage_ground: bool,
            pub projectile_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct NativeProjectileParams {
            pub pos: Float3,
            pub speed: Float3,
            pub spread: Float3,
            pub end: Float3,
            pub owner: i32,
            pub team: i32,
            pub weapon_num: i32,
            pub ttl: f32,
            pub gravity: f32,
            pub tracking: f32,
            pub max_range: f32,
            pub up_time: f32,
            pub start_alpha: f32,
            pub end_alpha: f32,
            pub model: String,
            pub ceg_tag: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct NumberOrBool {
            pub number: f32,
            pub boolean: bool,
            pub use_boolean: bool,
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
        pub struct ProjectileTargetRef {
            pub target_id: i32,
            pub target_type: i32,
            pub pos: Float3,
            pub is_ground_target: bool,
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct ResourcePack {
            pub metal: f32,
            pub energy: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RgbColor {
            pub r: f32,
            pub g: f32,
            pub b: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ScanAllDirsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ScanAllDirsResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetMapSquareTextureQuery {
            pub tex_square_x: i32,
            pub tex_square_y: i32,
            pub texture_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetMapSquareTextureResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SoundEffectParams {
            pub preset: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct StringArray {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct StringResult {
            pub value: String,
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
        pub struct SunLightingParams {
            pub ground_ambient_color: Option<Vec<f32>>,
            pub ground_diffuse_color: Option<Vec<f32>>,
            pub ground_specular_color: Option<Vec<f32>>,
            pub model_ambient_color: Option<Vec<f32>>,
            pub model_diffuse_color: Option<Vec<f32>>,
            pub model_specular_color: Option<Vec<f32>>,
            pub specular_exponent: Option<f32>,
            pub ground_shadow_density: Option<f32>,
            pub model_shadow_density: Option<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UInt32Array {
            pub data: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UInt32Result {
            pub value: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitCostOverrides {
            pub build_time: f32,
            pub metal_cost: f32,
            pub energy_cost: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitHealthValue {
            pub health: f32,
            pub capture: f32,
            pub paralyze: f32,
            pub build: f32,
            pub use_amounts: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitTargetRef {
            pub target_id: i32,
            pub pos: Float3,
            pub is_ground_target: bool,
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct UseArchiveResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct WaterParams {
            pub absorb: Option<Vec<f32>>,
            pub base_color: Option<Vec<f32>>,
            pub min_color: Option<Vec<f32>>,
            pub surface_color: Option<Vec<f32>>,
            pub diffuse_color: Option<Vec<f32>>,
            pub specular_color: Option<Vec<f32>>,
            pub plane_color: Option<Vec<f32>>,
            pub repeat_x: Option<f32>,
            pub repeat_y: Option<f32>,
            pub surface_alpha: Option<f32>,
            pub ambient_factor: Option<f32>,
            pub diffuse_factor: Option<f32>,
            pub specular_factor: Option<f32>,
            pub specular_power: Option<f32>,
            pub fresnel_min: Option<f32>,
            pub fresnel_max: Option<f32>,
            pub fresnel_power: Option<f32>,
            pub reflection_distortion: Option<f32>,
            pub blur_base: Option<f32>,
            pub blur_exponent: Option<f32>,
            pub perlin_start_freq: Option<f32>,
            pub perlin_lacunarity: Option<f32>,
            pub perlin_amplitude: Option<f32>,
            pub wind_speed: Option<f32>,
            pub wave_offset_factor: Option<f32>,
            pub wave_length: Option<f32>,
            pub wave_foam_distortion: Option<f32>,
            pub wave_foam_intensity: Option<f32>,
            pub caustics_resolution: Option<f32>,
            pub caustics_strength: Option<f32>,
            pub num_tiles: Option<f32>,
            pub shore_waves: Option<bool>,
            pub force_rendering: Option<bool>,
            pub has_water_plane: Option<bool>,
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

        #[derive(Debug, Clone, PartialEq)]
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
            extern "C" {
                #[link_name = "calculate-hash"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.calculate-hash."]
        #[inline]
        pub unsafe fn calculate_hash(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_calculate_hash::call(p0, p1, p2) }
        }

        #[inline]
        pub fn compress_folder(folder_path: &str, archive_type: &str, compressed_file_path: &str, include_folder: bool, mode: &str) -> Result<bool> {
            let mut folder_path_bytes = folder_path.as_bytes().to_vec();
            if folder_path_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            folder_path_bytes.push(0);
            let folder_path_cstr = core::ffi::CStr::from_bytes_with_nul(&folder_path_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            let mut archive_type_bytes = archive_type.as_bytes().to_vec();
            if archive_type_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            archive_type_bytes.push(0);
            let archive_type_cstr = core::ffi::CStr::from_bytes_with_nul(&archive_type_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            let mut compressed_file_path_bytes = compressed_file_path.as_bytes().to_vec();
            if compressed_file_path_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            compressed_file_path_bytes.push(0);
            let compressed_file_path_cstr = core::ffi::CStr::from_bytes_with_nul(&compressed_file_path_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            let mut mode_bytes = mode.as_bytes().to_vec();
            if mode_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            mode_bytes.push(0);
            let mode_cstr = core::ffi::CStr::from_bytes_with_nul(&mode_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::vfs::compress_folder(&folder_path_cstr, &archive_type_cstr, &compressed_file_path_cstr, include_folder, &mode_cstr)
        }

        #[inline]
        pub fn create_dir(path: &str) -> Result<bool> {
            let mut path_bytes = path.as_bytes().to_vec();
            if path_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            path_bytes.push(0);
            let path_cstr = core::ffi::CStr::from_bytes_with_nul(&path_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::vfs::create_dir(&path_cstr)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_dir_list {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "dir-list"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.dir-list."]
        #[inline]
        pub unsafe fn dir_list(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_dir_list::call(p0, p1, p2) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_download_archive {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "download-archive"]
                pub fn call(p0: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.download-archive."]
        #[inline]
        pub unsafe fn download_archive(p0: i32) -> i32 {
            unsafe { __core_owned_download_archive::call(p0) }
        }

        #[inline]
        pub fn extract_mod_archive_file(path: &str) -> Result<bool> {
            let mut path_bytes = path.as_bytes().to_vec();
            if path_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            path_bytes.push(0);
            let path_cstr = core::ffi::CStr::from_bytes_with_nul(&path_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::vfs::extract_mod_archive_file(&path_cstr)
        }

        #[inline]
        pub fn file_exists(path: &str) -> Result<bool> {
            let mut path_bytes = path.as_bytes().to_vec();
            if path_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            path_bytes.push(0);
            let path_cstr = core::ffi::CStr::from_bytes_with_nul(&path_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::vfs::file_exists(&path_cstr)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_all_archives {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "get-all-archives"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.get-all-archives."]
        #[inline]
        pub unsafe fn get_all_archives(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_all_archives::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_archive_checksum {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "get-archive-checksum"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.get-archive-checksum."]
        #[inline]
        pub unsafe fn get_archive_checksum(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_archive_checksum::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_archive_containing_file {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "get-archive-containing-file"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.get-archive-containing-file."]
        #[inline]
        pub unsafe fn get_archive_containing_file(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_archive_containing_file::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_archive_dependencies {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "get-archive-dependencies"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.get-archive-dependencies."]
        #[inline]
        pub unsafe fn get_archive_dependencies(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_archive_dependencies::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_archive_info {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "get-archive-info"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.get-archive-info."]
        #[inline]
        pub unsafe fn get_archive_info(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_archive_info::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_archive_path {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "get-archive-path"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.get-archive-path."]
        #[inline]
        pub unsafe fn get_archive_path(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_archive_path::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_archive_replaces {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "get-archive-replaces"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.get-archive-replaces."]
        #[inline]
        pub unsafe fn get_archive_replaces(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_archive_replaces::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_archives {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "get-archives"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.get-archives."]
        #[inline]
        pub unsafe fn get_archives(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_archives::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_available_a_is {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "get-available-a-is"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.get-available-a-is."]
        #[inline]
        pub unsafe fn get_available_a_is(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_available_a_is::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_file_absolute_path {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "get-file-absolute-path"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.get-file-absolute-path."]
        #[inline]
        pub unsafe fn get_file_absolute_path(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_file_absolute_path::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_file_info {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "get-file-info"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.get-file-info."]
        #[inline]
        pub unsafe fn get_file_info(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_file_info::call(p0, p1) }
        }

        #[inline]
        pub fn get_file_size(path: &str) -> Result<u32> {
            let mut path_bytes = path.as_bytes().to_vec();
            if path_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            path_bytes.push(0);
            let path_cstr = core::ffi::CStr::from_bytes_with_nul(&path_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::vfs::get_file_size(&path_cstr)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_games {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "get-games"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.get-games."]
        #[inline]
        pub unsafe fn get_games(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_games::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_loaded_archives {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "get-loaded-archives"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.get-loaded-archives."]
        #[inline]
        pub unsafe fn get_loaded_archives(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_loaded_archives::call(p0, p1) }
        }

        #[inline]
        pub fn get_map_square_texture(tex_square_x: i32, tex_square_y: i32, lod_min: i32, texture_name: &str, lod_max: i32) -> Result<bool> {
            let mut texture_name_bytes = texture_name.as_bytes().to_vec();
            if texture_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            texture_name_bytes.push(0);
            let texture_name_cstr = core::ffi::CStr::from_bytes_with_nul(&texture_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::vfs::get_map_square_texture(tex_square_x, tex_square_y, lod_min, &texture_name_cstr, lod_max)
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

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_maps {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "get-maps"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.get-maps."]
        #[inline]
        pub unsafe fn get_maps(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_maps::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_name_from_rapid_tag {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "get-name-from-rapid-tag"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.get-name-from-rapid-tag."]
        #[inline]
        pub unsafe fn get_name_from_rapid_tag(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_name_from_rapid_tag::call(p0, p1) }
        }

        #[inline]
        pub fn has_archive(archive_name: &str) -> Result<bool> {
            let mut archive_name_bytes = archive_name.as_bytes().to_vec();
            if archive_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            archive_name_bytes.push(0);
            let archive_name_cstr = core::ffi::CStr::from_bytes_with_nul(&archive_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::vfs::has_archive(&archive_name_cstr)
        }

        #[inline]
        pub fn is_directory(path: &str) -> Result<bool> {
            let mut path_bytes = path.as_bytes().to_vec();
            if path_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            path_bytes.push(0);
            let path_cstr = core::ffi::CStr::from_bytes_with_nul(&path_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::vfs::is_directory(&path_cstr)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_list_dir {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "list-dir"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.list-dir."]
        #[inline]
        pub unsafe fn list_dir(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_list_dir::call(p0, p1, p2) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_load_file {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "load-file"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.load-file."]
        #[inline]
        pub unsafe fn load_file(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_load_file::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_pack_f32 {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "pack-f32"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.pack-f32."]
        #[inline]
        pub unsafe fn pack_f32(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_pack_f32::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_pack_s16 {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "pack-s16"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.pack-s16."]
        #[inline]
        pub unsafe fn pack_s16(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_pack_s16::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_pack_s32 {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "pack-s32"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.pack-s32."]
        #[inline]
        pub unsafe fn pack_s32(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_pack_s32::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_pack_s8 {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "pack-s8"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.pack-s8."]
        #[inline]
        pub unsafe fn pack_s8(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_pack_s8::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_pack_u16 {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "pack-u16"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.pack-u16."]
        #[inline]
        pub unsafe fn pack_u16(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_pack_u16::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_pack_u32 {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "pack-u32"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.pack-u32."]
        #[inline]
        pub unsafe fn pack_u32(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_pack_u32::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_pack_u8 {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "pack-u8"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.pack-u8."]
        #[inline]
        pub unsafe fn pack_u8(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_pack_u8::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_read_file {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "read-file"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.read-file."]
        #[inline]
        pub unsafe fn read_file(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_read_file::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_read_file_as_string {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "read-file-as-string"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.read-file-as-string."]
        #[inline]
        pub unsafe fn read_file_as_string(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_read_file_as_string::call(p0, p1) }
        }

        #[inline]
        pub fn scan_all_dirs(unused: u8) -> Result<()> {
            crate::generated::vfs::scan_all_dirs(unused)?;
            Ok(())
        }

        #[inline]
        pub fn set_map_square_texture(tex_square_x: i32, tex_square_y: i32, texture_name: &str) -> Result<bool> {
            let mut texture_name_bytes = texture_name.as_bytes().to_vec();
            if texture_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            texture_name_bytes.push(0);
            let texture_name_cstr = core::ffi::CStr::from_bytes_with_nul(&texture_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::vfs::set_map_square_texture(tex_square_x, tex_square_y, &texture_name_cstr)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_sub_dirs {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "sub-dirs"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.sub-dirs."]
        #[inline]
        pub unsafe fn sub_dirs(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_sub_dirs::call(p0, p1, p2) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_unpack_f32 {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "unpack-f32"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.unpack-f32."]
        #[inline]
        pub unsafe fn unpack_f32(p0: i32, p1: i32, p2: i32, p3: i32) -> i32 {
            unsafe { __core_owned_unpack_f32::call(p0, p1, p2, p3) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_unpack_s16 {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "unpack-s16"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.unpack-s16."]
        #[inline]
        pub unsafe fn unpack_s16(p0: i32, p1: i32, p2: i32, p3: i32) -> i32 {
            unsafe { __core_owned_unpack_s16::call(p0, p1, p2, p3) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_unpack_s32 {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "unpack-s32"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.unpack-s32."]
        #[inline]
        pub unsafe fn unpack_s32(p0: i32, p1: i32, p2: i32, p3: i32) -> i32 {
            unsafe { __core_owned_unpack_s32::call(p0, p1, p2, p3) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_unpack_s8 {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "unpack-s8"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.unpack-s8."]
        #[inline]
        pub unsafe fn unpack_s8(p0: i32, p1: i32, p2: i32, p3: i32) -> i32 {
            unsafe { __core_owned_unpack_s8::call(p0, p1, p2, p3) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_unpack_u16 {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "unpack-u16"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.unpack-u16."]
        #[inline]
        pub unsafe fn unpack_u16(p0: i32, p1: i32, p2: i32, p3: i32) -> i32 {
            unsafe { __core_owned_unpack_u16::call(p0, p1, p2, p3) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_unpack_u32 {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "unpack-u32"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.unpack-u32."]
        #[inline]
        pub unsafe fn unpack_u32(p0: i32, p1: i32, p2: i32, p3: i32) -> i32 {
            unsafe { __core_owned_unpack_u32::call(p0, p1, p2, p3) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_unpack_u8 {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "unpack-u8"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.unpack-u8."]
        #[inline]
        pub unsafe fn unpack_u8(p0: i32, p1: i32, p2: i32, p3: i32) -> i32 {
            unsafe { __core_owned_unpack_u8::call(p0, p1, p2, p3) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_use_archive {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "use-archive"]
                pub fn call(p0: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.use-archive."]
        #[inline]
        pub unsafe fn use_archive(p0: i32) -> i64 {
            unsafe { __core_owned_use_archive::call(p0) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_zlib_compress {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "zlib-compress"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.zlib-compress."]
        #[inline]
        pub unsafe fn zlib_compress(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_zlib_compress::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_zlib_decompress {
            #[link(wasm_import_module = "spring:vfs")]
            extern "C" {
                #[link_name = "zlib-decompress"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:vfs.zlib-decompress."]
        #[inline]
        pub unsafe fn zlib_decompress(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_zlib_decompress::call(p0, p1) }
        }

    }

