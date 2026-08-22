    pub mod selection {
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
        pub struct DeselectUnitArrayQuery {
            pub unit_i_ds: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DeselectUnitArrayResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DeselectUnitQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DeselectUnitResult {
            pub success: bool,
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
        pub struct GetGroupListQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroupListResult {
            pub groups: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroupUnitsCountQuery {
            pub group_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroupUnitsCountResult {
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroupUnitsCountsQuery {
            pub group_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroupUnitsCountsResult {
            pub counts: SelectionCounts,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroupUnitsQuery {
            pub group_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroupUnitsResult {
            pub units: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroupUnitsSortedQuery {
            pub group_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroupUnitsSortedResult {
            pub groups: Vec<TeamUnitsByDef>,
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
        pub struct GetSelectedGroupQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSelectedGroupResult {
            pub group_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSelectedUnitsCountQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSelectedUnitsCountResult {
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSelectedUnitsCountsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSelectedUnitsCountsResult {
            pub counts: SelectionCounts,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSelectedUnitsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSelectedUnitsResult {
            pub units: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSelectedUnitsSortedQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSelectedUnitsSortedResult {
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
        pub struct GetUnitGroupQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitGroupResult {
            pub group_id: i32,
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
        pub struct SelectUnitArrayQuery {
            pub unit_i_ds: Vec<i32>,
            pub append: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SelectUnitArrayResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SelectUnitQuery {
            pub unit_id: i32,
            pub append: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SelectUnitResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SelectionCounts {
            pub unit_def_i_ds: Vec<i32>,
            pub counts: Vec<u32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitGroupQuery {
            pub unit_id: i32,
            pub group_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetUnitGroupResult {
            pub success: bool,
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
        pub struct UnitsQueryApi {
            pub valid_unit_id: u32,
            pub get_all_units: u32,
            pub get_team_units: u32,
            pub get_team_units_sorted: u32,
            pub get_team_units_counts: u32,
            pub get_team_units_by_defs: u32,
            pub get_team_unit_def_count: u32,
            pub get_team_unit_count: u32,
            pub get_units_in_rectangle: u32,
            pub get_units_in_box: u32,
            pub get_units_in_planes: u32,
            pub get_units_in_sphere: u32,
            pub get_units_in_cylinder: u32,
            pub get_unit_array_centroid: u32,
            pub get_unit_map_centroid: u32,
            pub get_unit_nearest_ally: u32,
            pub get_unit_nearest_enemy: u32,
            pub get_closest_enemy_unit: u32,
            pub get_unit_separation: u32,
            pub get_render_units: u32,
            pub get_render_units_draw_flag_changed: u32,
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
        mod __core_variable_output_get_group_list {
            #[link(wasm_import_module = "spring:selection")]
            extern "C" {
                #[link_name = "get-group-list"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_group_units {
            #[link(wasm_import_module = "spring:selection")]
            extern "C" {
                #[link_name = "get-group-units"]
                pub fn call(pgroup_id: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_selected_units {
            #[link(wasm_import_module = "spring:selection")]
            extern "C" {
                #[link_name = "get-selected-units"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_selected_units_sorted {
            #[link(wasm_import_module = "spring:selection")]
            extern "C" {
                #[link_name = "get-selected-units-sorted"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[inline]
        pub fn deselect_unit(unit_id: i32) -> Result<bool> {
            let value = crate::generated::selection::deselect_unit(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn deselect_unit_array(unit_i_ds: &Vec<i32>) -> Result<bool> {
            crate::generated::borrowed::selection::deselect_unit_array(unit_i_ds.as_slice())
        }

        #[inline]
        pub fn get_group_list(unused: u8) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_group_list::call(unused as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (unused as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_group_units(group_id: i32) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_group_units::call(group_id as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (group_id as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_group_units_count(group_id: i32) -> Result<u32> {
            let value = crate::generated::selection::get_group_units_count(group_id)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_group_units_counts {
            #[link(wasm_import_module = "spring:selection")]
            extern "C" {
                #[link_name = "get-group-units-counts"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:selection.get-group-units-counts."]
        #[inline]
        pub unsafe fn get_group_units_counts(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_group_units_counts::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_group_units_sorted {
            #[link(wasm_import_module = "spring:selection")]
            extern "C" {
                #[link_name = "get-group-units-sorted"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:selection.get-group-units-sorted."]
        #[inline]
        pub unsafe fn get_group_units_sorted(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_group_units_sorted::call(p0, p1) }
        }

        #[inline]
        pub fn get_selected_group(unused: u8) -> Result<i32> {
            let value = crate::generated::selection::get_selected_group(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_selected_units(unused: u8) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_selected_units::call(unused as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (unused as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_selected_units_count(unused: u8) -> Result<u32> {
            let value = crate::generated::selection::get_selected_units_count(unused)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_selected_units_counts {
            #[link(wasm_import_module = "spring:selection")]
            extern "C" {
                #[link_name = "get-selected-units-counts"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:selection.get-selected-units-counts."]
        #[inline]
        pub unsafe fn get_selected_units_counts(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_selected_units_counts::call(p0, p1) }
        }

        #[inline]
        pub fn get_selected_units_sorted(unused: u8) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_selected_units_sorted::call(unused as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (unused as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_unit_group(unit_id: i32) -> Result<i32> {
            let value = crate::generated::selection::get_unit_group(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn select_unit(unit_id: i32, append: bool) -> Result<bool> {
            let value = crate::generated::selection::select_unit(unit_id, append)?;
            Ok(value)
        }

        #[inline]
        pub fn select_unit_array(unit_i_ds: &Vec<i32>, append: bool) -> Result<bool> {
            crate::generated::borrowed::selection::select_unit_array(unit_i_ds.as_slice(), append)
        }

        #[inline]
        pub fn set_unit_group(unit_id: i32, group_id: i32) -> Result<bool> {
            let value = crate::generated::selection::set_unit_group(unit_id, group_id)?;
            Ok(value)
        }

    }

