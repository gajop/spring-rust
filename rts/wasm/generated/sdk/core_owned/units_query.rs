    pub mod units_query {
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

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum UnitFilter {
            UnitFilterAll,
            UnitFilterAllyteam,
            UnitFilterAllyUnits,
            UnitFilterEnemyUnits,
            UnitFilterMyUnits,
            UnitFilterTeam,
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
        pub struct BoxQuery {
            pub min: Float3,
            pub max: Float3,
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
        pub struct CylinderQuery {
            pub center: Float3,
            pub radius: f32,
            pub height: f32,
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
        pub struct GetAllUnitsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAllUnitsResult {
            pub units: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetClosestEnemyUnitOptions {
            pub use_los: bool,
            pub sphere_dist_test: bool,
            pub check_sight_dist: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetClosestEnemyUnitQuery {
            pub pos: Float3,
            pub range: f32,
            pub ally_team_id: i32,
            pub options: GetClosestEnemyUnitOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetClosestEnemyUnitResult {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetRenderUnitsDrawFlagChangedQuery {
            pub send_mask: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetRenderUnitsDrawFlagChangedResult {
            pub units: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetRenderUnitsQuery {
            pub draw_mask: i32,
            pub send_mask: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetRenderUnitsResult {
            pub units: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamUnitCountQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamUnitCountResult {
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamUnitDefCountQuery {
            pub team_id: i32,
            pub unit_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamUnitDefCountResult {
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamUnitsByDefsQuery {
            pub team_id: i32,
            pub unit_def_i_ds: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamUnitsByDefsResult {
            pub units: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamUnitsCountsQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamUnitsCountsResult {
            pub counts: Vec<UnitDefCount>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamUnitsQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamUnitsResult {
            pub units: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamUnitsSortedQuery {
            pub team_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTeamUnitsSortedResult {
            pub groups: Vec<TeamUnitsByDef>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitArrayCentroidQuery {
            pub unit_i_ds: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitArrayCentroidResult {
            pub centroid: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitMapCentroidQuery {
            pub unit_i_ds: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitMapCentroidResult {
            pub centroid: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitNearestAllyQuery {
            pub unit_id: i32,
            pub range: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitNearestAllyResult {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitNearestEnemyOptions {
            pub use_los: bool,
            pub sphere_dist_test: bool,
            pub check_sight_dist: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitNearestEnemyQuery {
            pub unit_id: i32,
            pub range: f32,
            pub options: GetUnitNearestEnemyOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitNearestEnemyResult {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitSeparationOptions {
            pub positional: bool,
            pub check_map: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitSeparationQuery {
            pub unit_id1: i32,
            pub unit_id2: i32,
            pub options: GetUnitSeparationOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitSeparationResult {
            pub separation: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitsInBoxQuery {
            pub xmin: f32,
            pub ymin: f32,
            pub zmin: f32,
            pub xmax: f32,
            pub ymax: f32,
            pub zmax: f32,
            pub allegiance: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitsInBoxResult {
            pub units: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitsInCylinderQuery {
            pub x: f32,
            pub z: f32,
            pub radius: f32,
            pub allegiance: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitsInCylinderResult {
            pub units: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitsInPlanesQuery {
            pub planes: PlanesQuery,
            pub allegiance: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitsInPlanesResult {
            pub units: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitsInRectangleQuery {
            pub xmin: f32,
            pub zmin: f32,
            pub xmax: f32,
            pub zmax: f32,
            pub allegiance: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitsInRectangleResult {
            pub units: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitsInSphereQuery {
            pub x: f32,
            pub y: f32,
            pub z: f32,
            pub radius: f32,
            pub allegiance: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitsInSphereResult {
            pub units: Vec<i32>,
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
        pub struct PlanesQuery {
            pub planes: Vec<Float4>,
            pub plane_count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ProjectileTargetRef {
            pub target_id: i32,
            pub target_type: i32,
            pub pos: Float3,
            pub is_ground_target: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RectangleQuery {
            pub min_x: f32,
            pub min_z: f32,
            pub max_x: f32,
            pub max_z: f32,
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
        pub struct SoundEffectParams {
            pub preset: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SphereQuery {
            pub center: Float3,
            pub radius: f32,
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
        pub struct TeamUnitsByDef {
            pub unit_def_id: i32,
            pub units: Vec<i32>,
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
        pub struct UnitDefCount {
            pub unit_def_id: i32,
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitFilterParams {
            pub filter: UnitFilter,
            pub team_id: i32,
            pub ally_team_id: i32,
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
        pub struct ValidUnitIDQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ValidUnitIDResult {
            pub valid: bool,
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
        mod __core_variable_output_get_render_units {
            #[link(wasm_import_module = "spring:units-query")]
            extern "C" {
                #[link_name = "get-render-units"]
                pub fn call(pdraw_mask: i32, psend_mask: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_render_units_draw_flag_changed {
            #[link(wasm_import_module = "spring:units-query")]
            extern "C" {
                #[link_name = "get-render-units-draw-flag-changed"]
                pub fn call(psend_mask: i32, output: i32) -> i32;
            }
        }

        #[inline]
        pub fn get_all_units(unused: u8) -> Result<Vec<i32>> {
            let _ = unused; crate::get_all_units()
        }

        #[inline]
        pub fn get_closest_enemy_unit(pos: Float3, range: f32, ally_team_id: i32, options: GetClosestEnemyUnitOptions) -> Result<i32> {
            let value = crate::generated::units_query::get_closest_enemy_unit(crate::generated::units_query::Float3 { x: pos.x, y: pos.y, z: pos.z }, range, ally_team_id, crate::generated::units_query::GetClosestEnemyUnitOptions { use_los: options.use_los, sphere_dist_test: options.sphere_dist_test, check_sight_dist: options.check_sight_dist })?;
            Ok(value)
        }

        #[inline]
        pub fn get_render_units(draw_mask: i32, send_mask: bool) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_render_units::call(draw_mask as i32, u32::from(send_mask) as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(output);
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, Default::default());
                    descriptor[0] = output.as_mut_ptr() as usize as u32;
                    descriptor[1] = output.len() as u32;
                    descriptor[2] = 0;
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = (draw_mask as i32, u32::from(send_mask) as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_render_units_draw_flag_changed(send_mask: bool) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_render_units_draw_flag_changed::call(u32::from(send_mask) as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(output);
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, Default::default());
                    descriptor[0] = output.as_mut_ptr() as usize as u32;
                    descriptor[1] = output.len() as u32;
                    descriptor[2] = 0;
                }
            }
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = (u32::from(send_mask) as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_team_unit_count(team_id: i32) -> Result<u32> {
            let value = crate::generated::units_query::get_team_unit_count(team_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_team_unit_def_count(team_id: i32, unit_def_id: i32) -> Result<u32> {
            let value = crate::generated::units_query::get_team_unit_def_count(team_id, unit_def_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_team_units(team_id: i32) -> Result<Vec<i32>> {
            crate::get_team_units(team_id)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_team_units_by_defs {
            #[link(wasm_import_module = "spring:units-query")]
            extern "C" {
                #[link_name = "get-team-units-by-defs"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-query.get-team-units-by-defs."]
        #[inline]
        pub unsafe fn get_team_units_by_defs(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32) -> i64 {
            unsafe { __core_owned_get_team_units_by_defs::call(p0, p1, p2, p3, p4) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_team_units_counts {
            #[link(wasm_import_module = "spring:units-query")]
            extern "C" {
                #[link_name = "get-team-units-counts"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-query.get-team-units-counts."]
        #[inline]
        pub unsafe fn get_team_units_counts(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_team_units_counts::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_team_units_sorted {
            #[link(wasm_import_module = "spring:units-query")]
            extern "C" {
                #[link_name = "get-team-units-sorted"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-query.get-team-units-sorted."]
        #[inline]
        pub unsafe fn get_team_units_sorted(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_team_units_sorted::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_array_centroid {
            #[link(wasm_import_module = "spring:units-query")]
            extern "C" {
                #[link_name = "get-unit-array-centroid"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-query.get-unit-array-centroid."]
        #[inline]
        pub unsafe fn get_unit_array_centroid(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_get_unit_array_centroid::call(p0, p1, p2) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_map_centroid {
            #[link(wasm_import_module = "spring:units-query")]
            extern "C" {
                #[link_name = "get-unit-map-centroid"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-query.get-unit-map-centroid."]
        #[inline]
        pub unsafe fn get_unit_map_centroid(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_get_unit_map_centroid::call(p0, p1, p2) }
        }

        #[inline]
        pub fn get_unit_nearest_ally(unit_id: i32, range: f32) -> Result<i32> {
            let value = crate::generated::units_query::get_unit_nearest_ally(unit_id, range)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_nearest_enemy(unit_id: i32, range: f32, options: GetUnitNearestEnemyOptions) -> Result<i32> {
            crate::get_unit_nearest_enemy(unit_id, range, options.use_los, options.sphere_dist_test, options.check_sight_dist)
        }

        #[inline]
        pub fn get_unit_separation(unit_id1: i32, unit_id2: i32, options: GetUnitSeparationOptions) -> Result<f32> {
            crate::get_unit_separation(unit_id1, unit_id2, options.positional, options.check_map)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_units_in_box {
            #[link(wasm_import_module = "spring:units-query")]
            extern "C" {
                #[link_name = "get-units-in-box"]
                pub fn call(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: i32, p7: i32, p8: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-query.get-units-in-box."]
        #[inline]
        pub unsafe fn get_units_in_box(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: i32, p7: i32, p8: i32) -> i64 {
            unsafe { __core_owned_get_units_in_box::call(p0, p1, p2, p3, p4, p5, p6, p7, p8) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_units_in_cylinder {
            #[link(wasm_import_module = "spring:units-query")]
            extern "C" {
                #[link_name = "get-units-in-cylinder"]
                pub fn call(p0: f32, p1: f32, p2: f32, p3: i32, p4: i32, p5: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-query.get-units-in-cylinder."]
        #[inline]
        pub unsafe fn get_units_in_cylinder(p0: f32, p1: f32, p2: f32, p3: i32, p4: i32, p5: i32) -> i64 {
            unsafe { __core_owned_get_units_in_cylinder::call(p0, p1, p2, p3, p4, p5) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_units_in_planes {
            #[link(wasm_import_module = "spring:units-query")]
            extern "C" {
                #[link_name = "get-units-in-planes"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-query.get-units-in-planes."]
        #[inline]
        pub unsafe fn get_units_in_planes(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_get_units_in_planes::call(p0, p1, p2) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_units_in_rectangle {
            #[link(wasm_import_module = "spring:units-query")]
            extern "C" {
                #[link_name = "get-units-in-rectangle"]
                pub fn call(p0: f32, p1: f32, p2: f32, p3: f32, p4: i32, p5: i32, p6: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-query.get-units-in-rectangle."]
        #[inline]
        pub unsafe fn get_units_in_rectangle(p0: f32, p1: f32, p2: f32, p3: f32, p4: i32, p5: i32, p6: i32) -> i64 {
            unsafe { __core_owned_get_units_in_rectangle::call(p0, p1, p2, p3, p4, p5, p6) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_units_in_sphere {
            #[link(wasm_import_module = "spring:units-query")]
            extern "C" {
                #[link_name = "get-units-in-sphere"]
                pub fn call(p0: f32, p1: f32, p2: f32, p3: f32, p4: i32, p5: i32, p6: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-query.get-units-in-sphere."]
        #[inline]
        pub unsafe fn get_units_in_sphere(p0: f32, p1: f32, p2: f32, p3: f32, p4: i32, p5: i32, p6: i32) -> i64 {
            unsafe { __core_owned_get_units_in_sphere::call(p0, p1, p2, p3, p4, p5, p6) }
        }

        #[inline]
        pub fn valid_unit_id(unit_id: i32) -> Result<bool> {
            let value = crate::generated::units_query::valid_unit_id(unit_id)?;
            Ok(value)
        }

    }

