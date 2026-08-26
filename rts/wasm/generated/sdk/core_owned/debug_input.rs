    pub mod debug_input {
        use super::{Result, String};

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
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(utf8_text, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(utf8_text)?),
            };
            crate::generated::borrowed::debug_input::emulate_text_editing(__core_string_0_buf.as_cstr(), start, length)
        }

        #[inline]
        pub fn emulate_text_input(utf8_text: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(utf8_text, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(utf8_text)?),
            };
            crate::generated::borrowed::debug_input::emulate_text_input(__core_string_0_buf.as_cstr())
        }

    }

