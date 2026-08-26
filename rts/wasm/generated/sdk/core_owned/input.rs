    pub mod input {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetActionHotKeysQuery {
            pub action: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetActionHotKeysResult {
            pub hotkeys: Vec<String>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetActiveCommandQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetActiveCommandResult {
            pub command_index: i32,
            pub command_id: i32,
            pub command_type: i32,
            pub command_name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetActivePageQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetActivePageResult {
            pub active_page: i32,
            pub max_page: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetDefaultCommandQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetDefaultCommandResult {
            pub command_index: i32,
            pub command_id: i32,
            pub command_type: i32,
            pub command_name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetInvertQueueKeyQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetInvertQueueKeyResult {
            pub invert: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetKeyBindingsQuery {
            pub key_set1: String,
            pub key_set2: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetKeyBindingsResult {
            pub bindings: Vec<KeyBindingEntry>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetKeyCodeQuery {
            pub key_sym: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetKeyCodeResult {
            pub key_code: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetKeyFromScanSymbolQuery {
            pub scan_symbol: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetKeyFromScanSymbolResult {
            pub key_name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetKeyStateQuery {
            pub key_code: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetKeyStateResult {
            pub pressed: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetKeySymbolQuery {
            pub key_code: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetKeySymbolResult {
            pub key_code_name: String,
            pub key_code_default_name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetModKeyStateQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetModKeyStateResult {
            pub alt: bool,
            pub ctrl: bool,
            pub meta: bool,
            pub shift: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMouseButtonsPressedQuery {
            pub buttons: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMouseButtonsPressedResult {
            pub pressed: Vec<bool>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetMouseCursorQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMouseCursorResult {
            pub cursor: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetMouseStartPositionQuery {
            pub button: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetMouseStartPositionResult {
            pub position: Float2,
            pub cam_pos: Float3,
            pub dir: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetMouseStateQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetMouseStateResult {
            pub state: MouseState,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetPressedKeysQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPressedKeysResult {
            pub keys: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetPressedScansQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPressedScansResult {
            pub scans: Vec<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetScanSymbolQuery {
            pub scan_code: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetScanSymbolResult {
            pub scan_code_name: String,
            pub scan_code_default_name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetSelectionBoxQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetSelectionBoxResult {
            pub box_: SelectionBox,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsAboveMiniMapQuery {
            pub screen_x: f32,
            pub screen_y: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsAboveMiniMapResult {
            pub above: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct KeyBindingEntry {
            pub command: String,
            pub extra: String,
            pub bound_with: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct MouseState {
            pub x: f32,
            pub y: f32,
            pub dx: f32,
            pub dy: f32,
            pub left: bool,
            pub middle: bool,
            pub right: bool,
            pub offscreen: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct SelectionBox {
            pub left: f32,
            pub top: f32,
            pub right: f32,
            pub bottom: f32,
            pub active: bool,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_mouse_cursor {
            #[link(wasm_import_module = "spring:input")]
            unsafe extern "C" {
                #[link_name = "get-mouse-cursor"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_pressed_keys {
            #[link(wasm_import_module = "spring:input")]
            unsafe extern "C" {
                #[link_name = "get-pressed-keys"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_pressed_scans {
            #[link(wasm_import_module = "spring:input")]
            unsafe extern "C" {
                #[link_name = "get-pressed-scans"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetActiveCommandValue {
            pub command_index: i32,
            pub command_id: i32,
            pub command_type: i32,
            pub command_name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetActivePageValue {
            pub active_page: i32,
            pub max_page: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetDefaultCommandValue {
            pub command_index: i32,
            pub command_id: i32,
            pub command_type: i32,
            pub command_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetKeySymbolValue {
            pub key_code_name: String,
            pub key_code_default_name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetModKeyStateValue {
            pub alt: bool,
            pub ctrl: bool,
            pub meta: bool,
            pub shift: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetMouseStartPositionValue {
            pub position: Float2,
            pub cam_pos: Float3,
            pub dir: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetScanSymbolValue {
            pub scan_code_name: String,
            pub scan_code_default_name: String,
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_action_hot_keys {
            #[link(wasm_import_module = "spring:input")]
            unsafe extern "C" {
                #[link_name = "get-action-hot-keys"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:input.get-action-hot-keys."]
        #[doc(hidden)]
        #[inline]
        pub fn get_action_hot_keys(p0: i32, p1: i32) -> i32 {
            __core_owned_get_action_hot_keys::call(p0, p1)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_active_command {
            #[link(wasm_import_module = "spring:input")]
            unsafe extern "C" {
                #[link_name = "get-active-command"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:input.get-active-command."]
        #[doc(hidden)]
        #[inline]
        pub fn get_active_command(p0: i32, p1: i32) -> i32 {
            __core_owned_get_active_command::call(p0, p1)
        }

        #[inline]
        pub fn get_active_page(unused: u8) -> Result<GetActivePageValue> {
            let value = crate::generated::input::get_active_page(unused)?;
            Ok(GetActivePageValue {
                active_page: value.0,
                max_page: value.1
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_default_command {
            #[link(wasm_import_module = "spring:input")]
            unsafe extern "C" {
                #[link_name = "get-default-command"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:input.get-default-command."]
        #[doc(hidden)]
        #[inline]
        pub fn get_default_command(p0: i32, p1: i32) -> i32 {
            __core_owned_get_default_command::call(p0, p1)
        }

        #[inline]
        pub fn get_invert_queue_key(unused: u8) -> Result<bool> {
            let value = crate::generated::input::get_invert_queue_key(unused)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_key_bindings {
            #[link(wasm_import_module = "spring:input")]
            unsafe extern "C" {
                #[link_name = "get-key-bindings"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:input.get-key-bindings."]
        #[doc(hidden)]
        #[inline]
        pub fn get_key_bindings(p0: i32, p1: i32) -> i32 {
            __core_owned_get_key_bindings::call(p0, p1)
        }

        #[inline]
        pub fn get_key_code(key_sym: &str) -> Result<i32> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(key_sym, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(key_sym)?),
            };
            crate::generated::borrowed::input::get_key_code(__core_string_0_buf.as_cstr())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_key_from_scan_symbol {
            #[link(wasm_import_module = "spring:input")]
            unsafe extern "C" {
                #[link_name = "get-key-from-scan-symbol"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:input.get-key-from-scan-symbol."]
        #[doc(hidden)]
        #[inline]
        pub fn get_key_from_scan_symbol(p0: i32, p1: i32) -> i32 {
            __core_owned_get_key_from_scan_symbol::call(p0, p1)
        }

        #[inline]
        pub fn get_key_state(key_code: i32) -> Result<bool> {
            let value = crate::generated::input::get_key_state(key_code)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_key_symbol {
            #[link(wasm_import_module = "spring:input")]
            unsafe extern "C" {
                #[link_name = "get-key-symbol"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:input.get-key-symbol."]
        #[doc(hidden)]
        #[inline]
        pub fn get_key_symbol(p0: i32, p1: i32) -> i32 {
            __core_owned_get_key_symbol::call(p0, p1)
        }

        #[inline]
        pub fn get_mod_key_state(unused: u8) -> Result<GetModKeyStateValue> {
            let value = crate::generated::input::get_mod_key_state(unused)?;
            Ok(GetModKeyStateValue {
                alt: value.0,
                ctrl: value.1,
                meta: value.2,
                shift: value.3
            })
        }

        #[inline]
        pub fn get_mouse_buttons_pressed(buttons: &[i32]) -> Result<Vec<bool>> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(buttons.len() as u32).to_le_bytes()); for __item in buttons.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.to_le_bytes());} __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::input::get_mouse_buttons_pressed(&__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 4);
                        let mut __result = Vec::<bool>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?);
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 4, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn get_mouse_cursor(unused: u8) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_mouse_cursor::call(unused as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
        pub fn get_mouse_start_position(button: i32) -> Result<GetMouseStartPositionValue> {
            let value = crate::generated::input::get_mouse_start_position(button)?;
            Ok(GetMouseStartPositionValue {
                position: Float2 { x: value.0.x, y: value.0.y },
                cam_pos: Float3 { x: value.1.x, y: value.1.y, z: value.1.z },
                dir: Float3 { x: value.2.x, y: value.2.y, z: value.2.z }
            })
        }

        #[inline]
        pub fn get_mouse_state(unused: u8) -> Result<MouseState> {
            let value = crate::generated::input::get_mouse_state(unused)?;
            Ok(MouseState { x: value.x, y: value.y, dx: value.dx, dy: value.dy, left: value.left, middle: value.middle, right: value.right, offscreen: value.offscreen })
        }

        #[inline]
        pub fn get_pressed_keys(unused: u8) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_pressed_keys::call(unused as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
        pub fn get_pressed_scans(unused: u8) -> Result<Vec<i32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<i32>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_pressed_scans::call(unused as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_scan_symbol {
            #[link(wasm_import_module = "spring:input")]
            unsafe extern "C" {
                #[link_name = "get-scan-symbol"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:input.get-scan-symbol."]
        #[doc(hidden)]
        #[inline]
        pub fn get_scan_symbol(p0: i32, p1: i32) -> i32 {
            __core_owned_get_scan_symbol::call(p0, p1)
        }

        #[inline]
        pub fn get_selection_box(unused: u8) -> Result<SelectionBox> {
            let value = crate::generated::input::get_selection_box(unused)?;
            Ok(SelectionBox { left: value.left, top: value.top, right: value.right, bottom: value.bottom, active: value.active })
        }

        #[inline]
        pub fn is_above_mini_map(screen_x: f32, screen_y: f32) -> Result<bool> {
            let value = crate::generated::input::is_above_mini_map(screen_x, screen_y)?;
            Ok(value)
        }

    }

