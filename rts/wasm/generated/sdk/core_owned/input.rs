    pub mod input {
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
        pub struct DefRef {
            pub name: String,
            pub id: i32,
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
        pub struct GetActionHotKeysQuery {
            pub action: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetActionHotKeysResult {
            pub hotkeys: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetActivePageQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetActivePageResult {
            pub active_page: i32,
            pub max_page: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetInvertQueueKeyQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetKeyStateQuery {
            pub key_code: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetKeyStateResult {
            pub pressed: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetKeySymbolQuery {
            pub key_code: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetKeySymbolResult {
            pub key_code_name: String,
            pub key_code_default_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetModKeyStateQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMouseCursorQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMouseCursorResult {
            pub cursor: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMouseStartPositionQuery {
            pub button: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMouseStartPositionResult {
            pub position: Float2,
            pub cam_pos: Float3,
            pub dir: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMouseStateQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMouseStateResult {
            pub state: MouseState,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPressedKeysQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPressedKeysResult {
            pub keys: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPressedScansQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetPressedScansResult {
            pub scans: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetScanSymbolQuery {
            pub scan_code: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetScanSymbolResult {
            pub scan_code_name: String,
            pub scan_code_default_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSelectionBoxQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSelectionBoxResult {
            pub box_: SelectionBox,
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
        pub struct IsAboveMiniMapQuery {
            pub screen_x: f32,
            pub screen_y: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsAboveMiniMapResult {
            pub above: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct KeyBindingEntry {
            pub command: String,
            pub extra: String,
            pub bound_with: String,
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
        pub struct ProjectileTargetRef {
            pub target_id: i32,
            pub target_type: i32,
            pub pos: Float3,
            pub is_ground_target: bool,
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
        pub struct SelectionBox {
            pub left: f32,
            pub top: f32,
            pub right: f32,
            pub bottom: f32,
            pub active: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct SoundEffectParams {
            pub preset: String,
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
        mod __core_variable_output_get_mouse_cursor {
            #[link(wasm_import_module = "spring:input")]
            extern "C" {
                #[link_name = "get-mouse-cursor"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_pressed_keys {
            #[link(wasm_import_module = "spring:input")]
            extern "C" {
                #[link_name = "get-pressed-keys"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_pressed_scans {
            #[link(wasm_import_module = "spring:input")]
            extern "C" {
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

        #[derive(Debug, Clone, PartialEq)]
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

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetModKeyStateValue {
            pub alt: bool,
            pub ctrl: bool,
            pub meta: bool,
            pub shift: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
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
            extern "C" {
                #[link_name = "get-action-hot-keys"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:input.get-action-hot-keys."]
        #[inline]
        pub unsafe fn get_action_hot_keys(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_action_hot_keys::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_active_command {
            #[link(wasm_import_module = "spring:input")]
            extern "C" {
                #[link_name = "get-active-command"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:input.get-active-command."]
        #[inline]
        pub unsafe fn get_active_command(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_active_command::call(p0, p1) }
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
            extern "C" {
                #[link_name = "get-default-command"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:input.get-default-command."]
        #[inline]
        pub unsafe fn get_default_command(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_default_command::call(p0, p1) }
        }

        #[inline]
        pub fn get_invert_queue_key(unused: u8) -> Result<bool> {
            let value = crate::generated::input::get_invert_queue_key(unused)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_key_bindings {
            #[link(wasm_import_module = "spring:input")]
            extern "C" {
                #[link_name = "get-key-bindings"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:input.get-key-bindings."]
        #[inline]
        pub unsafe fn get_key_bindings(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_key_bindings::call(p0, p1) }
        }

        #[inline]
        pub fn get_key_code(key_sym: &str) -> Result<i32> {
            let mut key_sym_bytes = key_sym.as_bytes().to_vec();
            if key_sym_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            key_sym_bytes.push(0);
            let key_sym_cstr = core::ffi::CStr::from_bytes_with_nul(&key_sym_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::input::get_key_code(&key_sym_cstr)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_key_from_scan_symbol {
            #[link(wasm_import_module = "spring:input")]
            extern "C" {
                #[link_name = "get-key-from-scan-symbol"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:input.get-key-from-scan-symbol."]
        #[inline]
        pub unsafe fn get_key_from_scan_symbol(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_key_from_scan_symbol::call(p0, p1) }
        }

        #[inline]
        pub fn get_key_state(key_code: i32) -> Result<bool> {
            let value = crate::generated::input::get_key_state(key_code)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_key_symbol {
            #[link(wasm_import_module = "spring:input")]
            extern "C" {
                #[link_name = "get-key-symbol"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:input.get-key-symbol."]
        #[inline]
        pub unsafe fn get_key_symbol(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_key_symbol::call(p0, p1) }
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

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_mouse_buttons_pressed {
            #[link(wasm_import_module = "spring:input")]
            extern "C" {
                #[link_name = "get-mouse-buttons-pressed"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:input.get-mouse-buttons-pressed."]
        #[inline]
        pub unsafe fn get_mouse_buttons_pressed(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_mouse_buttons_pressed::call(p0, p1) }
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
            extern "C" {
                #[link_name = "get-scan-symbol"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:input.get-scan-symbol."]
        #[inline]
        pub unsafe fn get_scan_symbol(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_scan_symbol::call(p0, p1) }
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

