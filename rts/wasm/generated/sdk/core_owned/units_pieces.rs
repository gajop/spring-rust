    pub mod units_pieces {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeaturePieceDirectionQuery {
            pub feature_id: i32,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeaturePieceDirectionResult {
            pub direction: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeaturePieceInfoQuery {
            pub feature_id: i32,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePieceInfoResult {
            pub info: PieceInfo,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeaturePieceListQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePieceListResult {
            pub names: Vec<String>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeaturePieceMapQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePieceMapResult {
            pub entries: Vec<PieceMapEntry>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeaturePieceMatrixQuery {
            pub feature_id: i32,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePieceMatrixResult {
            pub matrix: PieceMatrix,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeaturePiecePosDirQuery {
            pub feature_id: i32,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePiecePosDirResult {
            pub pos_dir: PiecePosDir,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeaturePiecePositionQuery {
            pub feature_id: i32,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeaturePiecePositionResult {
            pub position: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureRootPieceQuery {
            pub feature_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetFeatureRootPieceResult {
            pub root_piece: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetModelPieceListQuery {
            pub model_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetModelPieceListResult {
            pub names: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetModelPieceMapQuery {
            pub model_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetModelPieceMapResult {
            pub entries: Vec<PieceMapEntry>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetModelRootPieceQuery {
            pub model_name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetModelRootPieceResult {
            pub root_piece: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitPieceDirectionQuery {
            pub unit_id: i32,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitPieceDirectionResult {
            pub direction: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitPieceInfoQuery {
            pub unit_id: i32,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitPieceInfoResult {
            pub info: PieceInfo,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitPieceListQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitPieceListResult {
            pub names: Vec<String>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitPieceMapQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitPieceMapResult {
            pub entries: Vec<PieceMapEntry>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitPieceMatrixQuery {
            pub unit_id: i32,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitPieceMatrixResult {
            pub matrix: PieceMatrix,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitPiecePosDirQuery {
            pub unit_id: i32,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitPiecePosDirResult {
            pub pos_dir: PiecePosDir,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitPiecePositionQuery {
            pub unit_id: i32,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitPiecePositionResult {
            pub position: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitRootPieceQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitRootPieceResult {
            pub root_piece: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitScriptNamesQuery {
            pub unit_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitScriptNamesResult {
            pub names: Vec<String>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitScriptPieceQuery {
            pub unit_id: i32,
            pub script_num: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetUnitScriptPieceResult {
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PieceInfo {
            pub name: String,
            pub parent: String,
            pub children: Vec<String>,
            pub is_empty: bool,
            pub min: Float3,
            pub max: Float3,
            pub piece_num: i32,
            pub offset: Float3,
            pub emit_dir: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PieceMapEntry {
            pub name: String,
            pub piece_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PieceMatrix {
            pub m: Vec<f32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct PiecePosDir {
            pub position: Float3,
            pub direction: Float3,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFeaturePieceInfoValue {
            pub info: PieceInfo,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetUnitPieceInfoValue {
            pub info: PieceInfo,
            pub exists: bool,
        }

        #[inline]
        pub fn get_feature_piece_direction(feature_id: i32, piece_num: i32) -> Result<Float3> {
            let value = crate::generated::units_pieces::get_feature_piece_direction(feature_id, piece_num)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn get_feature_piece_info(feature_id: i32, piece_num: i32) -> Result<GetFeaturePieceInfoValue> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::units_pieces::get_feature_piece_info(feature_id, piece_num, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = GetFeaturePieceInfoValue {
                            info: PieceInfo { name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, parent: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, children: { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items }, is_empty: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, min: Float3 { x: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, y: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, z: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? }, max: Float3 { x: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, y: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, z: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? }, piece_num: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, offset: Float3 { x: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, y: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, z: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? }, emit_dir: Float3 { x: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, y: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, z: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? } },
                            exists: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
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
        pub fn get_feature_piece_list(feature_id: i32) -> Result<Vec<String>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::units_pieces::get_feature_piece_list(feature_id, &mut __output) {
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
        pub fn get_feature_piece_map(feature_id: i32) -> Result<Vec<PieceMapEntry>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::units_pieces::get_feature_piece_map(feature_id, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(PieceMapEntry { name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, piece_num: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? }); } __items };
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
        pub fn get_feature_piece_matrix(feature_id: i32, piece_num: i32) -> Result<PieceMatrix> {
            let value = crate::generated::units_pieces::get_feature_piece_matrix(feature_id, piece_num)?;
            Ok(PieceMatrix { m: value.m.into_iter().collect::<Vec<_>>() })
        }

        #[inline]
        pub fn get_feature_piece_pos_dir(feature_id: i32, piece_num: i32) -> Result<PiecePosDir> {
            let value = crate::generated::units_pieces::get_feature_piece_pos_dir(feature_id, piece_num)?;
            Ok(PiecePosDir { position: Float3 { x: value.position.x, y: value.position.y, z: value.position.z }, direction: Float3 { x: value.direction.x, y: value.direction.y, z: value.direction.z } })
        }

        #[inline]
        pub fn get_feature_piece_position(feature_id: i32, piece_num: i32) -> Result<Float3> {
            let value = crate::generated::units_pieces::get_feature_piece_position(feature_id, piece_num)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn get_feature_root_piece(feature_id: i32) -> Result<i32> {
            let value = crate::generated::units_pieces::get_feature_root_piece(feature_id)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_model_piece_list {
            #[link(wasm_import_module = "spring:units-pieces")]
            unsafe extern "C" {
                #[link_name = "get-model-piece-list"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-pieces.get-model-piece-list."]
        #[doc(hidden)]
        #[inline]
        pub fn get_model_piece_list(p0: i32, p1: i32) -> i32 {
            __core_owned_get_model_piece_list::call(p0, p1)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_model_piece_map {
            #[link(wasm_import_module = "spring:units-pieces")]
            unsafe extern "C" {
                #[link_name = "get-model-piece-map"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:units-pieces.get-model-piece-map."]
        #[doc(hidden)]
        #[inline]
        pub fn get_model_piece_map(p0: i32, p1: i32) -> i32 {
            __core_owned_get_model_piece_map::call(p0, p1)
        }

        #[inline]
        pub fn get_model_root_piece(model_name: &str) -> Result<i32> {
            let mut model_name_bytes = model_name.as_bytes().to_vec();
            if model_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            model_name_bytes.push(0);
            let model_name_cstr = core::ffi::CStr::from_bytes_with_nul(&model_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::units_pieces::get_model_root_piece(model_name_cstr)
        }

        #[inline]
        pub fn get_unit_piece_direction(unit_id: i32, piece_num: i32) -> Result<Float3> {
            let value = crate::generated::units_pieces::get_unit_piece_direction(unit_id, piece_num)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn get_unit_piece_info(unit_id: i32, piece_num: i32) -> Result<GetUnitPieceInfoValue> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::units_pieces::get_unit_piece_info(unit_id, piece_num, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = GetUnitPieceInfoValue {
                            info: PieceInfo { name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, parent: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, children: { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __items }, is_empty: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, min: Float3 { x: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, y: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, z: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? }, max: Float3 { x: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, y: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, z: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? }, piece_num: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, offset: Float3 { x: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, y: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, z: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? }, emit_dir: Float3 { x: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, y: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, z: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? } },
                            exists: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
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
        pub fn get_unit_piece_list(unit_id: i32) -> Result<Vec<String>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::units_pieces::get_unit_piece_list(unit_id, &mut __output) {
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
        pub fn get_unit_piece_map(unit_id: i32) -> Result<Vec<PieceMapEntry>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::units_pieces::get_unit_piece_map(unit_id, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(PieceMapEntry { name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, piece_num: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? }); } __items };
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
        pub fn get_unit_piece_matrix(unit_id: i32, piece_num: i32) -> Result<PieceMatrix> {
            let value = crate::generated::units_pieces::get_unit_piece_matrix(unit_id, piece_num)?;
            Ok(PieceMatrix { m: value.m.into_iter().collect::<Vec<_>>() })
        }

        #[inline]
        pub fn get_unit_piece_pos_dir(unit_id: i32, piece_num: i32) -> Result<PiecePosDir> {
            let value = crate::generated::units_pieces::get_unit_piece_pos_dir(unit_id, piece_num)?;
            Ok(PiecePosDir { position: Float3 { x: value.position.x, y: value.position.y, z: value.position.z }, direction: Float3 { x: value.direction.x, y: value.direction.y, z: value.direction.z } })
        }

        #[inline]
        pub fn get_unit_piece_position(unit_id: i32, piece_num: i32) -> Result<Float3> {
            let value = crate::generated::units_pieces::get_unit_piece_position(unit_id, piece_num)?;
            Ok(Float3 { x: value.x, y: value.y, z: value.z })
        }

        #[inline]
        pub fn get_unit_root_piece(unit_id: i32) -> Result<i32> {
            let value = crate::generated::units_pieces::get_unit_root_piece(unit_id)?;
            Ok(value)
        }

        #[inline]
        pub fn get_unit_script_names(unit_id: i32) -> Result<Vec<String>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::units_pieces::get_unit_script_names(unit_id, &mut __output) {
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
        pub fn get_unit_script_piece(unit_id: i32, script_num: i32) -> Result<i32> {
            let value = crate::generated::units_pieces::get_unit_script_piece(unit_id, script_num)?;
            Ok(value)
        }

    }

