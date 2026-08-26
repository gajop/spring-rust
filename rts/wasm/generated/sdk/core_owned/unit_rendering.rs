    pub mod unit_rendering {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnsyncedReadApi {
            pub unit_rendering: u32,
            pub get_clipboard: u32,
            pub get_prev_frame_sync_checksum: u32,
            pub get_active_cmd_desc: u32,
            pub get_active_cmd_descs: u32,
            pub get_cmd_desc_index: u32,
            pub get_box_selection_by_engine: u32,
            pub get_build_facing: u32,
            pub get_build_spacing: u32,
            pub get_draw_selection_info: u32,
            pub get_nano_projectile_params: u32,
            pub get_piece_projectile_name: u32,
            pub get_team_damage_stats: u32,
            pub get_last_message_positions: u32,
            pub solve_nurbs_curve: u32,
            pub is_unit_selected: u32,
            pub is_unit_allied: u32,
            pub get_custom_palette_color: u32,
            pub get_unit_palette_index: u32,
            pub get_feature_palette_index: u32,
            pub get_game_seconds_interpolated: u32,
        }

        pub use super::types::{ActiveCommandDescription, AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, GetActiveCmdDescQuery, GetActiveCmdDescResult, GetActiveCmdDescsQuery, GetActiveCmdDescsResult, GetBoxSelectionByEngineQuery, GetBoxSelectionByEngineResult, GetBuildFacingQuery, GetBuildFacingResult, GetBuildSpacingQuery, GetBuildSpacingResult, GetCameraRotationQuery, GetCameraRotationResult, GetCameraVectorsQuery, GetCameraVectorsResult, GetClipboardQuery, GetClipboardResult, GetCmdDescIndexQuery, GetCmdDescIndexResult, GetCustomPaletteColorQuery, GetCustomPaletteColorResult, GetDrawSelectionInfoQuery, GetDrawSelectionInfoResult, GetFeaturePaletteIndexQuery, GetFeaturePaletteIndexResult, GetFeaturesInScreenRectangleQuery, GetFeaturesInScreenRectangleResult, GetFrustumPlanesQuery, GetFrustumPlanesResult, GetGameSecondsInterpolatedQuery, GetGameSecondsInterpolatedResult, GetLastMessagePositionsQuery, GetLastMessagePositionsResult, GetNanoProjectileParamsQuery, GetNanoProjectileParamsResult, GetPieceProjectileNameQuery, GetPieceProjectileNameResult, GetPrevFrameSyncChecksumQuery, GetPrevFrameSyncChecksumResult, GetTeamDamageStatsQuery, GetTeamDamageStatsResult, GetUnitAlwaysUpdateMatrixQuery, GetUnitAlwaysUpdateMatrixResult, GetUnitDrawFlagQuery, GetUnitDrawFlagResult, GetUnitEngineDrawMaskQuery, GetUnitEngineDrawMaskResult, GetUnitIconDataQuery, GetUnitIconDataResult, GetUnitIconQuery, GetUnitIconResult, GetUnitLuaDrawQuery, GetUnitLuaDrawResult, GetUnitNoDrawQuery, GetUnitNoDrawResult, GetUnitNoGroupQuery, GetUnitNoGroupResult, GetUnitNoMinimapQuery, GetUnitNoMinimapResult, GetUnitNoSelectQuery, GetUnitNoSelectResult, GetUnitPaletteIndexQuery, GetUnitPaletteIndexResult, GetUnitSelectionVolumeDataQuery, GetUnitSelectionVolumeDataResult, GetUnitTransformMatrixQuery, GetUnitTransformMatrixResult, GetUnitViewPositionQuery, GetUnitViewPositionResult, GetUnitsInScreenRectangleQuery, GetUnitsInScreenRectangleResult, GetVisibleFeaturesOptions, GetVisibleFeaturesQuery, GetVisibleFeaturesResult, GetVisibleProjectilesOptions, GetVisibleProjectilesQuery, GetVisibleProjectilesResult, GetVisibleUnitsQuery, GetVisibleUnitsResult, Int2, Int3, Int32Array, Int32Result, IsUnitAlliedQuery, IsUnitAlliedResult, IsUnitIconQuery, IsUnitIconResult, IsUnitInViewQuery, IsUnitInViewResult, IsUnitSelectedQuery, IsUnitSelectedResult, IsUnitVisibleQuery, IsUnitVisibleResult, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SolveNURBSCurveQuery, SolveNURBSCurveResult, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_features_in_screen_rectangle {
            #[link(wasm_import_module = "spring:unit-rendering")]
            unsafe extern "C" {
                #[link_name = "get-features-in-screen-rectangle"]
                pub fn call(pleft: f32, ptop: f32, pright: f32, pbottom: f32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_units_in_screen_rectangle {
            #[link(wasm_import_module = "spring:unit-rendering")]
            unsafe extern "C" {
                #[link_name = "get-units-in-screen-rectangle"]
                pub fn call(pleft: f32, ptop: f32, pright: f32, pbottom: f32, pallegiance: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_visible_units {
            #[link(wasm_import_module = "spring:unit-rendering")]
            unsafe extern "C" {
                #[link_name = "get-visible-units"]
                pub fn call(pteam_id: i32, pradius: f32, pinclude_icons: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetCameraRotationValue {
            pub rot_x: f32,
            pub rot_y: f32,
            pub rot_z: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetCameraVectorsValue {
            pub forward: Float3,
            pub up: Float3,
            pub right: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitIconValue {
            pub icon_name: String,
            pub atlas_tex_coords: Vec<f32>,
            pub size: f32,
            pub distance: f32,
            pub radius_adjust: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitIconDataValue {
            pub icon_name: String,
            pub atlas_tex_coords: Vec<f32>,
            pub size: f32,
            pub distance: f32,
            pub radius_adjust: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitSelectionVolumeDataValue {
            pub scales: Float3,
            pub offsets: Float3,
            pub volume_type: i32,
            pub use_cont_hit_test: bool,
            pub primary_axis: i32,
            pub ignore_hits: bool,
        }

        #[inline]
        pub fn get_camera_rotation(unused: u8) -> Result<GetCameraRotationValue> {
            let value = crate::generated::unit_rendering::get_camera_rotation(unused)?;
            Ok(GetCameraRotationValue {
                rot_x: value.0,
                rot_y: value.1,
                rot_z: value.2
            })
        }

        #[inline]
        pub fn get_camera_vectors(unused: u8) -> Result<GetCameraVectorsValue> {
            let value = crate::generated::unit_rendering::get_camera_vectors(unused)?;
            Ok(GetCameraVectorsValue {
                forward: Float3 { x: value.0.x, y: value.0.y, z: value.0.z },
                up: Float3 { x: value.1.x, y: value.1.y, z: value.1.z },
                right: Float3 { x: value.2.x, y: value.2.y, z: value.2.z }
            })
        }

        #[inline]
        pub fn get_features_in_screen_rectangle(left: f32, top: f32, right: f32, bottom: f32) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_features_in_screen_rectangle::call(left, top, right, bottom, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (left, top, right, bottom);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_frustum_planes(unused: u8) -> Result<Vec<f32>> {
            let value = crate::generated::unit_rendering::get_frustum_planes(unused)?;
            Ok(value.into_iter().collect::<Vec<_>>())
        }

        #[inline]
        pub fn get_unit_always_update_matrix(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unit_rendering::get_unit_always_update_matrix(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_draw_flag(unit_id: i32) -> Result<u8> {
            let value = crate::generated::unit_rendering::get_unit_draw_flag(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_engine_draw_mask(unit_id: i32) -> Result<u32> {
            let value = crate::generated::unit_rendering::get_unit_engine_draw_mask(unit_id)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_icon {
            #[link(wasm_import_module = "spring:unit-rendering")]
            unsafe extern "C" {
                #[link_name = "get-unit-icon"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unit-rendering.get-unit-icon."]
        #[doc(hidden)]
        #[inline]
        pub fn get_unit_icon(p0: i32, p1: i32) -> i32 {
            __core_owned_get_unit_icon::call(p0, p1)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_unit_icon_data {
            #[link(wasm_import_module = "spring:unit-rendering")]
            unsafe extern "C" {
                #[link_name = "get-unit-icon-data"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unit-rendering.get-unit-icon-data."]
        #[doc(hidden)]
        #[inline]
        pub fn get_unit_icon_data(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_get_unit_icon_data::call(p0, p1, p2)
        }

        #[inline]
        pub fn get_unit_lua_draw(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unit_rendering::get_unit_lua_draw(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_no_draw(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unit_rendering::get_unit_no_draw(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_no_group(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unit_rendering::get_unit_no_group(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_no_minimap(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unit_rendering::get_unit_no_minimap(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_no_select(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unit_rendering::get_unit_no_select(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_selection_volume_data(unit_id: i32) -> Result<GetUnitSelectionVolumeDataValue> {
            let value = crate::generated::unit_rendering::get_unit_selection_volume_data(unit_id)?;
            Ok(GetUnitSelectionVolumeDataValue {
                scales: Float3 { x: value.0.x, y: value.0.y, z: value.0.z },
                offsets: Float3 { x: value.1.x, y: value.1.y, z: value.1.z },
                volume_type: value.2,
                use_cont_hit_test: value.3,
                primary_axis: value.4,
                ignore_hits: value.5
            })
        }

        #[inline]
        pub fn get_unit_transform_matrix(unit_id: i32) -> Result<Vec<f32>> {
            let value = crate::generated::unit_rendering::get_unit_transform_matrix(unit_id)?;
            Ok(value.into_iter().collect::<Vec<_>>())
        }

        #[inline]
        pub fn get_unit_view_position(unit_id: i32, use_mid_pos: bool) -> Result<Float3> {
            let value = crate::generated::unit_rendering::get_unit_view_position(unit_id, use_mid_pos)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn get_units_in_screen_rectangle(left: f32, top: f32, right: f32, bottom: f32, allegiance: i32) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_units_in_screen_rectangle::call(left, top, right, bottom, allegiance, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (left, top, right, bottom, allegiance);
                Err(unreachable!())
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_visible_features {
            #[link(wasm_import_module = "spring:unit-rendering")]
            unsafe extern "C" {
                #[link_name = "get-visible-features"]
                pub safe fn call(p0: i32, p1: f32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unit-rendering.get-visible-features."]
        #[doc(hidden)]
        #[inline]
        pub fn get_visible_features(p0: i32, p1: f32, p2: i32, p3: i32) -> i32 {
            __core_owned_get_visible_features::call(p0, p1, p2, p3)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_visible_projectiles {
            #[link(wasm_import_module = "spring:unit-rendering")]
            unsafe extern "C" {
                #[link_name = "get-visible-projectiles"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unit-rendering.get-visible-projectiles."]
        #[doc(hidden)]
        #[inline]
        pub fn get_visible_projectiles(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_get_visible_projectiles::call(p0, p1, p2)
        }

        #[inline]
        pub fn get_visible_units(team_id: i32, radius: f32, include_icons: bool) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_visible_units::call(team_id, radius, u32::from(include_icons) as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (team_id, radius, u32::from(include_icons) as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn is_unit_icon(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unit_rendering::is_unit_icon(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn is_unit_in_view(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unit_rendering::is_unit_in_view(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn is_unit_visible(unit_id: i32, radius: f32, check_icon: bool) -> Result<bool> {
            let value = crate::generated::unit_rendering::is_unit_visible(unit_id, radius, check_icon)?;
            Ok(value)
        }

    }

