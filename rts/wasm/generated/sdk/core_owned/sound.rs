    pub mod sound {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetSoundDevicesQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSoundDevicesResult {
            pub devices: Vec<String>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetSoundEffectParamsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetSoundEffectParamsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetSoundStreamTimeQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetSoundStreamTimeResult {
            pub time: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct LoadSoundDefQuery {
            pub sound_name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct LoadSoundDefResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct PauseSoundStreamQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct PauseSoundStreamResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PlaySoundFileQuery {
            pub sound_file: String,
            pub volume: f32,
            pub pos: Float3,
            pub velocity: Float3,
            pub channel: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct PlaySoundFileResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PlaySoundStreamQuery {
            pub ogg_file: String,
            pub volume: f32,
            pub enqueue: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct PlaySoundStreamResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct PreloadSoundItemQuery {
            pub sound_name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct PreloadSoundItemResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SetSoundEffectParamsQuery {
            pub params: SoundEffectParams,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetSoundEffectParamsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetSoundStreamVolumeQuery {
            pub volume: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SetSoundStreamVolumeResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct StopSoundStreamQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct StopSoundStreamResult {
            pub success: bool,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[inline]
        pub fn get_sound_devices(unused: u8) -> Result<Vec<String>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::sound::get_sound_devices(unused as i32, &mut __output) {
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
        pub fn get_sound_effect_params(unused: u8) -> Result<bool> {
            let value = crate::generated::sound::get_sound_effect_params(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn get_sound_stream_time(unused: u8) -> Result<f32> {
            let value = crate::generated::sound::get_sound_stream_time(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn load_sound_def(sound_name: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(sound_name, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(sound_name)?),
            };
            crate::generated::borrowed::sound::load_sound_def(__core_string_0_buf.as_cstr())
        }

        #[inline]
        pub fn pause_sound_stream(unused: u8) -> Result<bool> {
            let value = crate::generated::sound::pause_sound_stream(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn play_sound_file(sound_file: &str, volume: f32, pos: Float3, velocity: Float3, channel: i32) -> Result<bool> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + sound_file.len()); __b.extend_from_slice(&(sound_file.len() as u32).to_le_bytes()); __b.extend_from_slice(sound_file.as_bytes()); __b };
            let __blob1 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&pos.x.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&pos.y.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&pos.z.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            let __blob2 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&velocity.x.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&velocity.y.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&velocity.z.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            crate::generated::dynamic_input::sound::play_sound_file(volume, channel, &__blob0, &__blob1, &__blob2)
        }

        #[inline]
        pub fn play_sound_stream(ogg_file: &str, volume: f32, enqueue: bool) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(ogg_file, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(ogg_file)?),
            };
            crate::generated::borrowed::sound::play_sound_stream(__core_string_0_buf.as_cstr(), volume, enqueue)
        }

        #[inline]
        pub fn preload_sound_item(sound_name: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(sound_name, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(sound_name)?),
            };
            crate::generated::borrowed::sound::preload_sound_item(__core_string_0_buf.as_cstr())
        }

        #[inline]
        pub fn set_sound_effect_params(params: &SoundEffectParams) -> Result<bool> {
            let __blob0 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(params.preset.len() as u32).to_le_bytes()); __b.extend_from_slice(params.preset.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            crate::generated::dynamic_input::sound::set_sound_effect_params(&__blob0)
        }

        #[inline]
        pub fn set_sound_stream_volume(volume: f32) -> Result<bool> {
            let value = crate::generated::sound::set_sound_stream_volume(volume)?;
            Ok(value)
        }

        #[inline]
        pub fn stop_sound_stream(unused: u8) -> Result<bool> {
            let value = crate::generated::sound::stop_sound_stream(unused)?;
            Ok(value)
        }

    }

