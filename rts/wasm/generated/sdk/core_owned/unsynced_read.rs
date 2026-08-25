    pub mod unsynced_read {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnitRenderingApi {
            pub get_unit_no_draw: u32,
            pub get_unit_lua_draw: u32,
            pub get_unit_engine_draw_mask: u32,
            pub get_unit_always_update_matrix: u32,
            pub get_unit_draw_flag: u32,
            pub get_unit_no_select: u32,
            pub get_unit_no_minimap: u32,
            pub get_unit_no_group: u32,
            pub get_unit_view_position: u32,
            pub get_unit_transform_matrix: u32,
            pub get_unit_selection_volume_data: u32,
            pub get_unit_icon_data: u32,
            pub get_unit_icon: u32,
            pub get_camera_rotation: u32,
            pub get_camera_vectors: u32,
            pub get_frustum_planes: u32,
            pub get_visible_units: u32,
            pub get_visible_features: u32,
            pub get_visible_projectiles: u32,
            pub get_units_in_screen_rectangle: u32,
            pub get_features_in_screen_rectangle: u32,
            pub is_unit_visible: u32,
            pub is_unit_in_view: u32,
            pub is_unit_icon: u32,
        }

        pub use super::types::{ActiveCommandDescription, AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, GetActiveCmdDescQuery, GetActiveCmdDescResult, GetActiveCmdDescsQuery, GetActiveCmdDescsResult, GetBoxSelectionByEngineQuery, GetBoxSelectionByEngineResult, GetBuildFacingQuery, GetBuildFacingResult, GetBuildSpacingQuery, GetBuildSpacingResult, GetCameraRotationQuery, GetCameraRotationResult, GetCameraVectorsQuery, GetCameraVectorsResult, GetClipboardQuery, GetClipboardResult, GetCmdDescIndexQuery, GetCmdDescIndexResult, GetCustomPaletteColorQuery, GetCustomPaletteColorResult, GetDrawSelectionInfoQuery, GetDrawSelectionInfoResult, GetFeaturePaletteIndexQuery, GetFeaturePaletteIndexResult, GetFeaturesInScreenRectangleQuery, GetFeaturesInScreenRectangleResult, GetFrustumPlanesQuery, GetFrustumPlanesResult, GetGameSecondsInterpolatedQuery, GetGameSecondsInterpolatedResult, GetLastMessagePositionsQuery, GetLastMessagePositionsResult, GetNanoProjectileParamsQuery, GetNanoProjectileParamsResult, GetPieceProjectileNameQuery, GetPieceProjectileNameResult, GetPrevFrameSyncChecksumQuery, GetPrevFrameSyncChecksumResult, GetTeamDamageStatsQuery, GetTeamDamageStatsResult, GetUnitAlwaysUpdateMatrixQuery, GetUnitAlwaysUpdateMatrixResult, GetUnitDrawFlagQuery, GetUnitDrawFlagResult, GetUnitEngineDrawMaskQuery, GetUnitEngineDrawMaskResult, GetUnitIconDataQuery, GetUnitIconDataResult, GetUnitIconQuery, GetUnitIconResult, GetUnitLuaDrawQuery, GetUnitLuaDrawResult, GetUnitNoDrawQuery, GetUnitNoDrawResult, GetUnitNoGroupQuery, GetUnitNoGroupResult, GetUnitNoMinimapQuery, GetUnitNoMinimapResult, GetUnitNoSelectQuery, GetUnitNoSelectResult, GetUnitPaletteIndexQuery, GetUnitPaletteIndexResult, GetUnitSelectionVolumeDataQuery, GetUnitSelectionVolumeDataResult, GetUnitTransformMatrixQuery, GetUnitTransformMatrixResult, GetUnitViewPositionQuery, GetUnitViewPositionResult, GetUnitsInScreenRectangleQuery, GetUnitsInScreenRectangleResult, GetVisibleFeaturesOptions, GetVisibleFeaturesQuery, GetVisibleFeaturesResult, GetVisibleProjectilesOptions, GetVisibleProjectilesQuery, GetVisibleProjectilesResult, GetVisibleUnitsQuery, GetVisibleUnitsResult, Int2, Int3, Int32Array, Int32Result, IsUnitAlliedQuery, IsUnitAlliedResult, IsUnitIconQuery, IsUnitIconResult, IsUnitInViewQuery, IsUnitInViewResult, IsUnitSelectedQuery, IsUnitSelectedResult, IsUnitVisibleQuery, IsUnitVisibleResult, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SolveNURBSCurveQuery, SolveNURBSCurveResult, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_clipboard {
            #[link(wasm_import_module = "spring:unsynced-read")]
            extern "C" {
                #[link_name = "get-clipboard"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_piece_projectile_name {
            #[link(wasm_import_module = "spring:unsynced-read")]
            extern "C" {
                #[link_name = "get-piece-projectile-name"]
                pub fn call(pprojectile_id: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_prev_frame_sync_checksum {
            #[link(wasm_import_module = "spring:unsynced-read")]
            extern "C" {
                #[link_name = "get-prev-frame-sync-checksum"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetActiveCmdDescValue {
            pub cmd_desc: ActiveCommandDescription,
            pub has_command: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetCustomPaletteColorValue {
            pub r: f32,
            pub g: f32,
            pub b: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeaturePaletteIndexValue {
            pub custom_index: i32,
            pub using_custom_color: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetNanoProjectileParamsValue {
            pub r: f32,
            pub v: f32,
            pub a: f32,
            pub rand_r: f32,
            pub rand_v: f32,
            pub rand_a: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTeamDamageStatsValue {
            pub damage_dealt: f32,
            pub damage_received: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitPaletteIndexValue {
            pub custom_index: i32,
            pub using_custom_color: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SolveNURBSCurveValue {
            pub points: Vec<Float3>,
            pub success: bool,
        }

        #[inline]
        pub fn get_active_cmd_desc(cmd_index: i32) -> Result<GetActiveCmdDescValue> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::unsynced_read::get_active_cmd_desc(cmd_index, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = GetActiveCmdDescValue {
                            cmd_desc: ActiveCommandDescription { id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, type_: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, action: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, tooltip: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, texture: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, cursor: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, queueing: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, hidden: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, disabled: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, show_unique: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, only_texture: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, params: { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items } },
                            has_command: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                        };
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
        pub fn get_active_cmd_descs(unused: u8) -> Result<Vec<ActiveCommandDescription>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::unsynced_read::get_active_cmd_descs(unused as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(ActiveCommandDescription { id: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, type_: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, action: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, tooltip: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, texture: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, cursor: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, queueing: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, hidden: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, disabled: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, show_unique: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, only_texture: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, params: { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items } }); } __items };
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
        pub fn get_box_selection_by_engine(unused: u8) -> Result<bool> {
            let value = crate::generated::unsynced_read::get_box_selection_by_engine(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_build_facing(unused: u8) -> Result<i32> {
            let value = crate::generated::unsynced_read::get_build_facing(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_build_spacing(unused: u8) -> Result<i32> {
            let value = crate::generated::unsynced_read::get_build_spacing(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_clipboard(unused: u8) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_clipboard::call(unused as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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

        #[inline]
        pub fn get_cmd_desc_index(cmd_id: i32) -> Result<i32> {
            let value = crate::generated::unsynced_read::get_cmd_desc_index(cmd_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_custom_palette_color(index: i32) -> Result<GetCustomPaletteColorValue> {
            let value = crate::generated::unsynced_read::get_custom_palette_color(index)?;
            Ok(GetCustomPaletteColorValue {
                r: value.0,
                g: value.1,
                b: value.2,
                success: value.3
            })
        }

        #[inline]
        pub fn get_draw_selection_info(unused: u8) -> Result<bool> {
            let value = crate::generated::unsynced_read::get_draw_selection_info(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_feature_palette_index(feature_id: i32) -> Result<GetFeaturePaletteIndexValue> {
            let value = crate::generated::unsynced_read::get_feature_palette_index(feature_id)?;
            Ok(GetFeaturePaletteIndexValue {
                custom_index: value.0,
                using_custom_color: value.1
            })
        }

        #[inline]
        pub fn get_game_seconds_interpolated(unused: u8) -> Result<f32> {
            let value = crate::generated::unsynced_read::get_game_seconds_interpolated(unused)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_last_message_positions {
            #[link(wasm_import_module = "spring:unsynced-read")]
            unsafe extern "C" {
                #[link_name = "get-last-message-positions"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unsynced-read.get-last-message-positions."]
        #[doc(hidden)]
        #[inline]
        pub fn get_last_message_positions(p0: i32, p1: i32) -> i32 {
            __core_owned_get_last_message_positions::call(p0, p1)
        }

        #[inline]
        pub fn get_nano_projectile_params(unused: u8) -> Result<GetNanoProjectileParamsValue> {
            let value = crate::generated::unsynced_read::get_nano_projectile_params(unused)?;
            Ok(GetNanoProjectileParamsValue {
                r: value.0,
                v: value.1,
                a: value.2,
                rand_r: value.3,
                rand_v: value.4,
                rand_a: value.5
            })
        }

        #[inline]
        pub fn get_piece_projectile_name(projectile_id: i32) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_piece_projectile_name::call(projectile_id, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (projectile_id);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_prev_frame_sync_checksum(unused: u8) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_prev_frame_sync_checksum::call(unused as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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

        #[inline]
        pub fn get_team_damage_stats(team_id: i32) -> Result<GetTeamDamageStatsValue> {
            let value = crate::generated::unsynced_read::get_team_damage_stats(team_id)?;
            Ok(GetTeamDamageStatsValue {
                damage_dealt: value.0,
                damage_received: value.1,
                success: value.2
            })
        }

        #[inline]
        pub fn get_unit_palette_index(unit_id: i32) -> Result<GetUnitPaletteIndexValue> {
            let value = crate::generated::unsynced_read::get_unit_palette_index(unit_id)?;
            Ok(GetUnitPaletteIndexValue {
                custom_index: value.0,
                using_custom_color: value.1
            })
        }

        #[inline]
        pub fn is_unit_allied(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unsynced_read::is_unit_allied(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn is_unit_selected(unit_id: i32) -> Result<bool> {
            let value = crate::generated::unsynced_read::is_unit_selected(unit_id)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_solve_nurbs_curve {
            #[link(wasm_import_module = "spring:unsynced-read")]
            unsafe extern "C" {
                #[link_name = "solve-nurbs-curve"]
                pub safe fn call(p0: i32, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:unsynced-read.solve-nurbs-curve."]
        #[doc(hidden)]
        #[inline]
        pub fn solve_nurbs_curve(p0: i32, p1: i32, p2: i32, p3: i32) -> i32 {
            __core_owned_solve_nurbs_curve::call(p0, p1, p2, p3)
        }

    }

