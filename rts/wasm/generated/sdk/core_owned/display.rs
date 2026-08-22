    pub mod display {
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
        pub struct DefRef {
            pub name: String,
            pub id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Error {
            pub code: i32,
            pub message: String,
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
        pub struct GetDrawFrameQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetDrawFrameResult {
            pub low16: u32,
            pub high16: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetDualViewGeometryQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetDualViewGeometryResult {
            pub geom: ViewGeometry,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFPSQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFPSResult {
            pub fps: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFrameTimeOffsetQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFrameTimeOffsetResult {
            pub offset: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameSpeedQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameSpeedResult {
            pub wanted_speed: f32,
            pub speed: f32,
            pub paused: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetLastUpdateSecondsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetLastUpdateSecondsResult {
            pub seconds: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetLosViewColorsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetLosViewColorsResult {
            pub always_color: Float3,
            pub los_color: Float3,
            pub radar_color: Float3,
            pub jam_color: Float3,
            pub radar_color2: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMapDrawModeQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMapDrawModeResult {
            pub mode: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMiniMapDualScreenQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMiniMapDualScreenResult {
            pub position: String,
            pub dual_screen: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMiniMapGeometryQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMiniMapGeometryResult {
            pub geom: MinimapGeometry,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMiniMapRotationQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMiniMapRotationResult {
            pub rotation: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetNumDisplaysQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetNumDisplaysResult {
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetScreenGeometryQuery {
            pub screen_num: i32,
            pub query_usable: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetScreenGeometryResult {
            pub geom: ViewGeometry,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamColorQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamColorResult {
            pub color: TeamColor,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamOrigColorQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamOrigColorResult {
            pub color: TeamColor,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetViewGeometryQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetViewGeometryResult {
            pub geom: ViewGeometry,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWaterModeQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWaterModeResult {
            pub mode: i32,
            pub name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWindowGeometryQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWindowGeometryResult {
            pub geom: ViewGeometry,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct HaveAdvShadingQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct HaveAdvShadingResult {
            pub enabled: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct HaveShadowsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct HaveShadowsResult {
            pub enabled: bool,
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
        pub struct IsAABBInViewQuery {
            pub mins: Float3,
            pub maxs: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsAABBInViewResult {
            pub in_view: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsGUIHiddenQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsGUIHiddenResult {
            pub hidden: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsSphereInViewQuery {
            pub center: Float3,
            pub radius: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsSphereInViewResult {
            pub in_view: bool,
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
        pub struct MinimapGeometry {
            pub size_x: i32,
            pub size_y: i32,
            pub pos_x: i32,
            pub pos_y: i32,
            pub minimized: bool,
            pub maximized: bool,
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
        pub struct ProjectileTargetRef {
            pub target_id: i32,
            pub target_type: i32,
            pub pos: Float3,
            pub is_ground_target: bool,
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
        pub struct SetTeamColorQuery {
            pub team_id: i32,
            pub color: TeamColor,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetTeamColorResult {
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
        pub struct TeamColor {
            pub r: f32,
            pub g: f32,
            pub b: f32,
            pub a: f32,
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
        pub struct ViewGeometry {
            pub view_size_x: i32,
            pub view_size_y: i32,
            pub view_pos_x: i32,
            pub view_pos_y: i32,
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

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_map_draw_mode {
            #[link(wasm_import_module = "spring:display")]
            extern "C" {
                #[link_name = "get-map-draw-mode"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetDrawFrameValue {
            pub low16: u32,
            pub high16: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGameSpeedValue {
            pub wanted_speed: f32,
            pub speed: f32,
            pub paused: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetLosViewColorsValue {
            pub always_color: Float3,
            pub los_color: Float3,
            pub radar_color: Float3,
            pub jam_color: Float3,
            pub radar_color2: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMiniMapDualScreenValue {
            pub position: String,
            pub dual_screen: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWaterModeValue {
            pub mode: i32,
            pub name: String,
        }

        #[inline]
        pub fn get_draw_frame(unused: u8) -> Result<GetDrawFrameValue> {
            let value = crate::generated::display::get_draw_frame(unused)?;
            Ok(GetDrawFrameValue {
                low16: value.0,
                high16: value.1
            })
        }

        #[inline]
        pub fn get_dual_view_geometry(unused: u8) -> Result<ViewGeometry> {
            let value = crate::generated::display::get_dual_view_geometry(unused)?;
            Ok(ViewGeometry { view_size_x: value.view_size_x, view_size_y: value.view_size_y, view_pos_x: value.view_pos_x, view_pos_y: value.view_pos_y })
        }

        #[inline]
        pub fn get_fps(unused: u8) -> Result<u32> {
            let value = crate::generated::display::get_fps(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_frame_time_offset(unused: u8) -> Result<f32> {
            let value = crate::generated::display::get_frame_time_offset(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_game_speed(unused: u8) -> Result<GetGameSpeedValue> {
            let value = crate::generated::display::get_game_speed(unused)?;
            Ok(GetGameSpeedValue {
                wanted_speed: value.0,
                speed: value.1,
                paused: value.2
            })
        }

        #[inline]
        pub fn get_last_update_seconds(unused: u8) -> Result<f32> {
            let value = crate::generated::display::get_last_update_seconds(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_los_view_colors(unused: u8) -> Result<GetLosViewColorsValue> {
            let value = crate::generated::display::get_los_view_colors(unused)?;
            Ok(GetLosViewColorsValue {
                always_color: Float3 { x: value.0.x, y: value.0.y, z: value.0.z },
                los_color: Float3 { x: value.1.x, y: value.1.y, z: value.1.z },
                radar_color: Float3 { x: value.2.x, y: value.2.y, z: value.2.z },
                jam_color: Float3 { x: value.3.x, y: value.3.y, z: value.3.z },
                radar_color2: Float3 { x: value.4.x, y: value.4.y, z: value.4.z }
            })
        }

        #[inline]
        pub fn get_map_draw_mode(unused: u8) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_map_draw_mode::call(unused as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(super::decode_core_string(output));
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, 0);
                    descriptor[0] = output.as_mut_ptr() as usize as u32;
                    descriptor[1] = output.len() as u32;
                    descriptor[2] = 0;
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = (unused as i32);
                Err(unreachable!())
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_mini_map_dual_screen {
            #[link(wasm_import_module = "spring:display")]
            extern "C" {
                #[link_name = "get-mini-map-dual-screen"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:display.get-mini-map-dual-screen."]
        #[inline]
        pub unsafe fn get_mini_map_dual_screen(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_mini_map_dual_screen::call(p0, p1) }
        }

        #[inline]
        pub fn get_mini_map_geometry(unused: u8) -> Result<MinimapGeometry> {
            let value = crate::generated::display::get_mini_map_geometry(unused)?;
            Ok(MinimapGeometry { size_x: value.size_x, size_y: value.size_y, pos_x: value.pos_x, pos_y: value.pos_y, minimized: value.minimized, maximized: value.maximized })
        }

        #[inline]
        pub fn get_mini_map_rotation(unused: u8) -> Result<f32> {
            let value = crate::generated::display::get_mini_map_rotation(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_num_displays(unused: u8) -> Result<u32> {
            let value = crate::generated::display::get_num_displays(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_screen_geometry(screen_num: i32, query_usable: bool) -> Result<ViewGeometry> {
            let value = crate::generated::display::get_screen_geometry(screen_num, query_usable)?;
            Ok(ViewGeometry { view_size_x: value.view_size_x, view_size_y: value.view_size_y, view_pos_x: value.view_pos_x, view_pos_y: value.view_pos_y })
        }

        #[inline]
        pub fn get_team_color(team_id: i32) -> Result<TeamColor> {
            let value = crate::generated::display::get_team_color(team_id)?;
            Ok(TeamColor { r: value.r, g: value.g, b: value.b, a: value.a })
        }

        #[inline]
        pub fn get_team_orig_color(team_id: i32) -> Result<TeamColor> {
            let value = crate::generated::display::get_team_orig_color(team_id)?;
            Ok(TeamColor { r: value.r, g: value.g, b: value.b, a: value.a })
        }

        #[inline]
        pub fn get_view_geometry(unused: u8) -> Result<ViewGeometry> {
            let value = crate::generated::display::get_view_geometry(unused)?;
            Ok(ViewGeometry { view_size_x: value.view_size_x, view_size_y: value.view_size_y, view_pos_x: value.view_pos_x, view_pos_y: value.view_pos_y })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_water_mode {
            #[link(wasm_import_module = "spring:display")]
            extern "C" {
                #[link_name = "get-water-mode"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:display.get-water-mode."]
        #[inline]
        pub unsafe fn get_water_mode(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_water_mode::call(p0, p1) }
        }

        #[inline]
        pub fn get_window_geometry(unused: u8) -> Result<ViewGeometry> {
            let value = crate::generated::display::get_window_geometry(unused)?;
            Ok(ViewGeometry { view_size_x: value.view_size_x, view_size_y: value.view_size_y, view_pos_x: value.view_pos_x, view_pos_y: value.view_pos_y })
        }

        #[inline]
        pub fn have_adv_shading(unused: u8) -> Result<bool> {
            let value = crate::generated::display::have_adv_shading(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn have_shadows(unused: u8) -> Result<bool> {
            let value = crate::generated::display::have_shadows(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn is_aabb_in_view(mins: Float3, maxs: Float3) -> Result<bool> {
            let value = crate::generated::display::is_aabb_in_view(crate::generated::display::Float3 { x: mins.x, y: mins.y, z: mins.z }, crate::generated::display::Float3 { x: maxs.x, y: maxs.y, z: maxs.z })?;
            Ok(value)
        }

        #[inline]
        pub fn is_gui_hidden(unused: u8) -> Result<bool> {
            let value = crate::generated::display::is_gui_hidden(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn is_sphere_in_view(center: Float3, radius: f32) -> Result<bool> {
            let value = crate::generated::display::is_sphere_in_view(crate::generated::display::Float3 { x: center.x, y: center.y, z: center.z }, radius)?;
            Ok(value)
        }

        #[inline]
        pub fn set_team_color(team_id: i32, color: TeamColor) -> Result<bool> {
            let value = crate::generated::display::set_team_color(team_id, crate::generated::display::TeamColor { r: color.r, g: color.g, b: color.b, a: color.a })?;
            Ok(value)
        }

    }

