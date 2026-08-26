    pub mod selection {
        use super::{Result, Vec};

        #[derive(Debug, Clone, PartialEq)]
        pub struct DeselectUnitArrayQuery {
            pub unit_i_ds: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DeselectUnitArrayResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DeselectUnitQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DeselectUnitResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGroupListQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroupListResult {
            pub groups: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGroupUnitsCountQuery {
            pub group_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGroupUnitsCountResult {
            pub count: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGroupUnitsCountsQuery {
            pub group_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroupUnitsCountsResult {
            pub counts: SelectionCounts,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGroupUnitsQuery {
            pub group_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroupUnitsResult {
            pub units: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetGroupUnitsSortedQuery {
            pub group_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGroupUnitsSortedResult {
            pub groups: Vec<TeamUnitsByDef>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetSelectedGroupQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetSelectedGroupResult {
            pub group_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetSelectedUnitsCountQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetSelectedUnitsCountResult {
            pub count: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetSelectedUnitsCountsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSelectedUnitsCountsResult {
            pub counts: SelectionCounts,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetSelectedUnitsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSelectedUnitsResult {
            pub units: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetSelectedUnitsSortedQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSelectedUnitsSortedResult {
            pub units: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitGroupQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitGroupResult {
            pub group_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SelectUnitArrayQuery {
            pub unit_i_ds: Vec<i32>,
            pub append: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SelectUnitArrayResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SelectUnitQuery {
            pub unit_id: i32,
            pub append: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SelectUnitResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SelectionCounts {
            pub unit_def_i_ds: Vec<i32>,
            pub counts: Vec<u32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitGroupQuery {
            pub unit_id: i32,
            pub group_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetUnitGroupResult {
            pub success: bool,
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

        pub use super::types::{AtmosphereParams, BoolResult, BoxQuery, CollisionVolumeData, CommonErrorCode, CylinderQuery, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, GetAllUnitsQuery, GetAllUnitsResult, GetClosestEnemyUnitOptions, GetClosestEnemyUnitQuery, GetClosestEnemyUnitResult, GetRenderUnitsDrawFlagChangedQuery, GetRenderUnitsDrawFlagChangedResult, GetRenderUnitsQuery, GetRenderUnitsResult, GetTeamUnitCountQuery, GetTeamUnitCountResult, GetTeamUnitDefCountQuery, GetTeamUnitDefCountResult, GetTeamUnitsByDefsQuery, GetTeamUnitsByDefsResult, GetTeamUnitsCountsQuery, GetTeamUnitsCountsResult, GetTeamUnitsQuery, GetTeamUnitsResult, GetTeamUnitsSortedQuery, GetTeamUnitsSortedResult, GetUnitArrayCentroidQuery, GetUnitArrayCentroidResult, GetUnitMapCentroidQuery, GetUnitMapCentroidResult, GetUnitNearestAllyQuery, GetUnitNearestAllyResult, GetUnitNearestEnemyOptions, GetUnitNearestEnemyQuery, GetUnitNearestEnemyResult, GetUnitSeparationOptions, GetUnitSeparationQuery, GetUnitSeparationResult, GetUnitsInBoxQuery, GetUnitsInBoxResult, GetUnitsInCylinderQuery, GetUnitsInCylinderResult, GetUnitsInPlanesQuery, GetUnitsInPlanesResult, GetUnitsInRectangleQuery, GetUnitsInRectangleResult, GetUnitsInSphereQuery, GetUnitsInSphereResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, PlanesQuery, ProjectileTargetRef, RectangleQuery, ResourcePack, RgbColor, SoundEffectParams, SphereQuery, StringArray, StringResult, SunLightingParams, TeamUnitsByDef, UInt32Array, UInt32Result, UnitCostOverrides, UnitDefCount, UnitFilter, UnitFilterParams, UnitHealthValue, UnitTargetRef, ValidUnitIDQuery, ValidUnitIDResult, WaterParams};

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_group_list {
            #[link(wasm_import_module = "spring:selection")]
            unsafe extern "C" {
                #[link_name = "get-group-list"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_group_units {
            #[link(wasm_import_module = "spring:selection")]
            unsafe extern "C" {
                #[link_name = "get-group-units"]
                pub fn call(pgroup_id: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_selected_units {
            #[link(wasm_import_module = "spring:selection")]
            unsafe extern "C" {
                #[link_name = "get-selected-units"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_selected_units_sorted {
            #[link(wasm_import_module = "spring:selection")]
            unsafe extern "C" {
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
        pub fn deselect_unit_array(unit_i_ds: &[i32]) -> Result<bool> {
            crate::generated::borrowed::selection::deselect_unit_array(unit_i_ds)
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
                    let status = unsafe { __core_variable_output_get_group_units::call(group_id, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (group_id);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_group_units_count(group_id: i32) -> Result<u32> {
            let value = crate::generated::selection::get_group_units_count(group_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_group_units_counts(group_id: i32) -> Result<SelectionCounts> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::selection::get_group_units_counts(group_id, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = SelectionCounts { unit_def_i_ds: { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items }, counts: { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items } };
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
        pub fn get_group_units_sorted(group_id: i32) -> Result<Vec<TeamUnitsByDef>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::selection::get_group_units_sorted(group_id, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(TeamUnitsByDef { unit_def_id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, units: { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items } }); } __items };
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

        #[inline]
        pub fn get_selected_units_counts(unused: u8) -> Result<SelectionCounts> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::selection::get_selected_units_counts(unused as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = SelectionCounts { unit_def_i_ds: { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items }, counts: { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items } };
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
        pub fn select_unit_array(unit_i_ds: &[i32], append: bool) -> Result<bool> {
            crate::generated::borrowed::selection::select_unit_array(unit_i_ds, append)
        }

        #[inline]
        pub fn set_unit_group(unit_id: i32, group_id: i32) -> Result<bool> {
            let value = crate::generated::selection::set_unit_group(unit_id, group_id)?;
            Ok(value)
        }

    }

