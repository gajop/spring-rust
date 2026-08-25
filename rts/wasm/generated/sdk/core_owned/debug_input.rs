    pub mod debug_input {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ClearEmulatedInputQuery {
            pub fire_releases: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ClearEmulatedInputResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct EmulateKeyQuery {
            pub key_code: i32,
            pub pressed: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct EmulateKeyResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct EmulateMouseButtonQuery {
            pub button: i32,
            pub pressed: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct EmulateMouseButtonResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct EmulateMouseMoveQuery {
            pub x: i32,
            pub y: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct EmulateMouseMoveResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct EmulateMouseWheelQuery {
            pub delta: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct EmulateMouseWheelResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EmulateTextEditingQuery {
            pub utf8_text: String,
            pub start: u32,
            pub length: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct EmulateTextEditingResult {
            pub consumed: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EmulateTextInputQuery {
            pub utf8_text: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct EmulateTextInputResult {
            pub consumed: bool,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[inline]
        pub fn clear_emulated_input(fire_releases: bool) -> Result<()> {
            crate::generated::debug_input::clear_emulated_input(fire_releases)?;
            Ok(())
        }

        #[inline]
        pub fn emulate_key(key_code: i32, pressed: bool) -> Result<()> {
            crate::generated::debug_input::emulate_key(key_code, pressed)?;
            Ok(())
        }

        #[inline]
        pub fn emulate_mouse_button(button: i32, pressed: bool) -> Result<()> {
            crate::generated::debug_input::emulate_mouse_button(button, pressed)?;
            Ok(())
        }

        #[inline]
        pub fn emulate_mouse_move(x: i32, y: i32) -> Result<()> {
            crate::generated::debug_input::emulate_mouse_move(x, y)?;
            Ok(())
        }

        #[inline]
        pub fn emulate_mouse_wheel(delta: f32) -> Result<()> {
            crate::generated::debug_input::emulate_mouse_wheel(delta)?;
            Ok(())
        }

        #[inline]
        pub fn emulate_text_editing(utf8_text: &str, start: u32, length: u32) -> Result<bool> {
            let mut utf8_text_bytes = utf8_text.as_bytes().to_vec();
            if utf8_text_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            utf8_text_bytes.push(0);
            let utf8_text_cstr = core::ffi::CStr::from_bytes_with_nul(&utf8_text_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::debug_input::emulate_text_editing(utf8_text_cstr, start, length)
        }

        #[inline]
        pub fn emulate_text_input(utf8_text: &str) -> Result<bool> {
            let mut utf8_text_bytes = utf8_text.as_bytes().to_vec();
            if utf8_text_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            utf8_text_bytes.push(0);
            let utf8_text_cstr = core::ffi::CStr::from_bytes_with_nul(&utf8_text_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::debug_input::emulate_text_input(utf8_text_cstr)
        }

    }

