    pub mod display {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetDrawFrameQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetDrawFrameResult {
            pub low16: u32,
            pub high16: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetDualViewGeometryQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetDualViewGeometryResult {
            pub geom: ViewGeometry,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFPSQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFPSResult {
            pub fps: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFrameTimeOffsetQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFrameTimeOffsetResult {
            pub offset: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGameSpeedQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGameSpeedResult {
            pub wanted_speed: f32,
            pub speed: f32,
            pub paused: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetLastUpdateSecondsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetLastUpdateSecondsResult {
            pub seconds: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetLosViewColorsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetLosViewColorsResult {
            pub always_color: Float3,
            pub los_color: Float3,
            pub radar_color: Float3,
            pub jam_color: Float3,
            pub radar_color2: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetMapDrawModeQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMapDrawModeResult {
            pub mode: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetMiniMapDualScreenQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMiniMapDualScreenResult {
            pub position: String,
            pub dual_screen: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetMiniMapGeometryQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetMiniMapGeometryResult {
            pub geom: MinimapGeometry,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetMiniMapRotationQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetMiniMapRotationResult {
            pub rotation: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetNumDisplaysQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetNumDisplaysResult {
            pub count: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetScreenGeometryQuery {
            pub screen_num: i32,
            pub query_usable: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetScreenGeometryResult {
            pub geom: ViewGeometry,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamColorQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamColorResult {
            pub color: TeamColor,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamOrigColorQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamOrigColorResult {
            pub color: TeamColor,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetViewGeometryQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetViewGeometryResult {
            pub geom: ViewGeometry,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetWaterModeQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWaterModeResult {
            pub mode: i32,
            pub name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetWindowGeometryQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetWindowGeometryResult {
            pub geom: ViewGeometry,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct HaveAdvShadingQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct HaveAdvShadingResult {
            pub enabled: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct HaveShadowsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct HaveShadowsResult {
            pub enabled: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsAABBInViewQuery {
            pub mins: Float3,
            pub maxs: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsAABBInViewResult {
            pub in_view: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsGUIHiddenQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsGUIHiddenResult {
            pub hidden: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsSphereInViewQuery {
            pub center: Float3,
            pub radius: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsSphereInViewResult {
            pub in_view: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct MinimapGeometry {
            pub size_x: i32,
            pub size_y: i32,
            pub pos_x: i32,
            pub pos_y: i32,
            pub minimized: bool,
            pub maximized: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetTeamColorQuery {
            pub team_id: i32,
            pub color: TeamColor,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetTeamColorResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct TeamColor {
            pub r: f32,
            pub g: f32,
            pub b: f32,
            pub a: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ViewGeometry {
            pub view_size_x: i32,
            pub view_size_y: i32,
            pub view_pos_x: i32,
            pub view_pos_y: i32,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_map_draw_mode {
            #[link(wasm_import_module = "spring:display")]
            unsafe extern "C" {
                #[link_name = "get-map-draw-mode"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetDrawFrameValue {
            pub low16: u32,
            pub high16: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGameSpeedValue {
            pub wanted_speed: f32,
            pub speed: f32,
            pub paused: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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
            unsafe extern "C" {
                #[link_name = "get-mini-map-dual-screen"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:display.get-mini-map-dual-screen."]
        #[doc(hidden)]
        #[inline]
        pub fn get_mini_map_dual_screen(p0: i32, p1: i32) -> i32 {
            __core_owned_get_mini_map_dual_screen::call(p0, p1)
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
            unsafe extern "C" {
                #[link_name = "get-water-mode"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:display.get-water-mode."]
        #[doc(hidden)]
        #[inline]
        pub fn get_water_mode(p0: i32, p1: i32) -> i32 {
            __core_owned_get_water_mode::call(p0, p1)
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

