    pub mod units_query {
        use super::{Result, Vec};

        pub use super::types::{AtmosphereParams, BoolResult, BoxQuery, CollisionVolumeData, CommonErrorCode, CylinderQuery, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, GetAllUnitsQuery, GetAllUnitsResult, GetClosestEnemyUnitOptions, GetClosestEnemyUnitQuery, GetClosestEnemyUnitResult, GetRenderUnitsDrawFlagChangedQuery, GetRenderUnitsDrawFlagChangedResult, GetRenderUnitsQuery, GetRenderUnitsResult, GetTeamUnitCountQuery, GetTeamUnitCountResult, GetTeamUnitDefCountQuery, GetTeamUnitDefCountResult, GetTeamUnitsByDefsQuery, GetTeamUnitsByDefsResult, GetTeamUnitsCountsQuery, GetTeamUnitsCountsResult, GetTeamUnitsQuery, GetTeamUnitsResult, GetTeamUnitsSortedQuery, GetTeamUnitsSortedResult, GetUnitArrayCentroidQuery, GetUnitArrayCentroidResult, GetUnitMapCentroidQuery, GetUnitMapCentroidResult, GetUnitNearestAllyQuery, GetUnitNearestAllyResult, GetUnitNearestEnemyOptions, GetUnitNearestEnemyQuery, GetUnitNearestEnemyResult, GetUnitSeparationOptions, GetUnitSeparationQuery, GetUnitSeparationResult, GetUnitsInBoxQuery, GetUnitsInBoxResult, GetUnitsInCylinderQuery, GetUnitsInCylinderResult, GetUnitsInPlanesQuery, GetUnitsInPlanesResult, GetUnitsInRectangleQuery, GetUnitsInRectangleResult, GetUnitsInSphereQuery, GetUnitsInSphereResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, PlanesQuery, ProjectileTargetRef, RectangleQuery, ResourcePack, RgbColor, SoundEffectParams, SphereQuery, StringArray, StringResult, SunLightingParams, TeamUnitsByDef, UInt32Array, UInt32Result, UnitCostOverrides, UnitDefCount, UnitFilter, UnitFilterParams, UnitHealthValue, UnitTargetRef, ValidUnitIDQuery, ValidUnitIDResult, WaterParams};

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_render_units {
            #[link(wasm_import_module = "spring:units-query")]
            unsafe extern "C" {
                #[link_name = "get-render-units"]
                pub fn call(pdraw_mask: i32, psend_mask: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_render_units_draw_flag_changed {
            #[link(wasm_import_module = "spring:units-query")]
            unsafe extern "C" {
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
                    let status = unsafe { __core_variable_output_get_render_units::call(draw_mask, u32::from(send_mask) as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (draw_mask, u32::from(send_mask) as i32);
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
            unsafe extern "C" {
                #[link_name = "get-team-units-by-defs"]
                pub safe fn call(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-query.get-team-units-by-defs."]
        #[doc(hidden)]
        #[inline]
        pub fn get_team_units_by_defs(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32) -> i64 {
            __core_owned_get_team_units_by_defs::call(p0, p1, p2, p3, p4)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_team_units_counts {
            #[link(wasm_import_module = "spring:units-query")]
            unsafe extern "C" {
                #[link_name = "get-team-units-counts"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-query.get-team-units-counts."]
        #[doc(hidden)]
        #[inline]
        pub fn get_team_units_counts(p0: i32, p1: i32) -> i32 {
            __core_owned_get_team_units_counts::call(p0, p1)
        }

        #[inline]
        pub fn get_team_units_sorted(team_id: i32) -> Result<Vec<TeamUnitsByDef>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::units_query::get_team_units_sorted(team_id, &mut __output) {
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

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_array_centroid {
            #[link(wasm_import_module = "spring:units-query")]
            unsafe extern "C" {
                #[link_name = "get-unit-array-centroid"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-query.get-unit-array-centroid."]
        #[doc(hidden)]
        #[inline]
        pub fn get_unit_array_centroid(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_get_unit_array_centroid::call(p0, p1, p2)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_map_centroid {
            #[link(wasm_import_module = "spring:units-query")]
            unsafe extern "C" {
                #[link_name = "get-unit-map-centroid"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-query.get-unit-map-centroid."]
        #[doc(hidden)]
        #[inline]
        pub fn get_unit_map_centroid(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_get_unit_map_centroid::call(p0, p1, p2)
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
            unsafe extern "C" {
                #[link_name = "get-units-in-box"]
                pub safe fn call(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: i32, p7: i32, p8: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-query.get-units-in-box."]
        #[doc(hidden)]
        #[inline]
        #[expect(clippy::too_many_arguments, reason = "Core function preserves the corresponding Lua API arity")]
            pub fn get_units_in_box(p0: f32, p1: f32, p2: f32, p3: f32, p4: f32, p5: f32, p6: i32, p7: i32, p8: i32) -> i64 {
            __core_owned_get_units_in_box::call(p0, p1, p2, p3, p4, p5, p6, p7, p8)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_units_in_cylinder {
            #[link(wasm_import_module = "spring:units-query")]
            unsafe extern "C" {
                #[link_name = "get-units-in-cylinder"]
                pub safe fn call(p0: f32, p1: f32, p2: f32, p3: i32, p4: i32, p5: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-query.get-units-in-cylinder."]
        #[doc(hidden)]
        #[inline]
        pub fn get_units_in_cylinder(p0: f32, p1: f32, p2: f32, p3: i32, p4: i32, p5: i32) -> i64 {
            __core_owned_get_units_in_cylinder::call(p0, p1, p2, p3, p4, p5)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_units_in_planes {
            #[link(wasm_import_module = "spring:units-query")]
            unsafe extern "C" {
                #[link_name = "get-units-in-planes"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-query.get-units-in-planes."]
        #[doc(hidden)]
        #[inline]
        pub fn get_units_in_planes(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_get_units_in_planes::call(p0, p1, p2)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_units_in_rectangle {
            #[link(wasm_import_module = "spring:units-query")]
            unsafe extern "C" {
                #[link_name = "get-units-in-rectangle"]
                pub safe fn call(p0: f32, p1: f32, p2: f32, p3: f32, p4: i32, p5: i32, p6: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-query.get-units-in-rectangle."]
        #[doc(hidden)]
        #[inline]
        pub fn get_units_in_rectangle(p0: f32, p1: f32, p2: f32, p3: f32, p4: i32, p5: i32, p6: i32) -> i64 {
            __core_owned_get_units_in_rectangle::call(p0, p1, p2, p3, p4, p5, p6)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_units_in_sphere {
            #[link(wasm_import_module = "spring:units-query")]
            unsafe extern "C" {
                #[link_name = "get-units-in-sphere"]
                pub safe fn call(p0: f32, p1: f32, p2: f32, p3: f32, p4: i32, p5: i32, p6: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-query.get-units-in-sphere."]
        #[doc(hidden)]
        #[inline]
        pub fn get_units_in_sphere(p0: f32, p1: f32, p2: f32, p3: f32, p4: i32, p5: i32, p6: i32) -> i64 {
            __core_owned_get_units_in_sphere::call(p0, p1, p2, p3, p4, p5, p6)
        }

        #[inline]
        pub fn valid_unit_id(unit_id: i32) -> Result<bool> {
            let value = crate::generated::units_query::valid_unit_id(unit_id)?;
            Ok(value)
        }

    }

