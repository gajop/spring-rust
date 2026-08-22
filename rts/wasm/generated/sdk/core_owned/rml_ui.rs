    pub mod rml_ui {
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

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum RmlDataFieldType {
            RmlFieldBool,
            RmlFieldColor,
            RmlFieldFloat,
            RmlFieldInt,
            RmlFieldPercent,
            RmlFieldPixels,
            RmlFieldString,
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
        pub struct MapRenderingParams {
            pub splat_tex_scales: Option<Vec<f32>>,
            pub splat_tex_mults: Option<Vec<f32>>,
            pub void_water: Option<bool>,
            pub void_ground: Option<bool>,
            pub splat_detail_normal_diffuse_alpha: Option<bool>,
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
        pub struct RmlAddTranslationStringQuery {
            pub key: String,
            pub translation: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlAddTranslationStringResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlClearDocumentPathRequestsQuery {
            pub document_path: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlClearDocumentPathRequestsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlClearTranslationsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlClearTranslationsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextBoolQuery {
            pub context_handle: u64,
            pub value: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextBoolResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextCreateDataModelQuery {
            pub context_handle: u64,
            pub name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextCreateDocumentQuery {
            pub context_handle: u64,
            pub tag: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextCreateDocumentResult {
            pub document_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextDocumentQuery {
            pub context_handle: u64,
            pub document_handle: u64,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextEventListenerCallbackQuery {
            pub context_handle: u64,
            pub event: String,
            pub in_capture_phase: bool,
            pub callback: u32,
            pub user_data: u32,
            pub destroy_callback: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextEventListenerRemoveQuery {
            pub context_handle: u64,
            pub event_listener_handle: u64,
            pub event: String,
            pub in_capture_phase: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextGetDimensionsResult {
            pub x: i32,
            pub y: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextGetDocumentQuery {
            pub context_handle: u64,
            pub name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextGetDocumentResult {
            pub document_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextGetElementAtPointQuery {
            pub context_handle: u64,
            pub x: f32,
            pub y: f32,
            pub ignore_element_handle: u64,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextGetElementAtPointResult {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextGetElementResult {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextGetFloatResult {
            pub value: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextGetNameResult {
            pub name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextHandleQuery {
            pub context_handle: u64,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextKeyQuery {
            pub context_handle: u64,
            pub key: i32,
            pub key_modifier_state: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextLoadDocumentQuery {
            pub context_handle: u64,
            pub document_path: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextLoadDocumentResult {
            pub document_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextMouseButtonQuery {
            pub context_handle: u64,
            pub button: i32,
            pub key_modifier_state: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextMouseMoveQuery {
            pub context_handle: u64,
            pub x: f32,
            pub y: f32,
            pub key_modifier_state: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextMouseWheelQuery {
            pub context_handle: u64,
            pub x: f32,
            pub y: f32,
            pub key_modifier_state: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextOpenDataModelQuery {
            pub context_handle: u64,
            pub name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextOpenDataModelResult {
            pub data_model_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextPointerCaptureQuery {
            pub context_handle: u64,
            pub anchor_x: i32,
            pub anchor_y: i32,
            pub active: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextPointerDeltaResult {
            pub delta_x: i32,
            pub delta_y: i32,
            pub status: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextSetDimensionsQuery {
            pub context_handle: u64,
            pub x: i32,
            pub y: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextSetFloatQuery {
            pub context_handle: u64,
            pub value: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextStringBoolQuery {
            pub context_handle: u64,
            pub name: String,
            pub value: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextStringQuery {
            pub context_handle: u64,
            pub name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextTextInputQuery {
            pub context_handle: u64,
            pub text: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlCreateContextQuery {
            pub name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlCreateContextResult {
            pub context_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataEventArgs {
            pub event_handle: u64,
            pub target_element_handle: u64,
            pub values: Vec<RmlDataValue>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataFieldDef {
            pub name: String,
            pub type_: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelBindBoolQuery {
            pub data_model_handle: u64,
            pub name: String,
            pub initial_value: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelBindColorQuery {
            pub data_model_handle: u64,
            pub name: String,
            pub red: u8,
            pub green: u8,
            pub blue: u8,
            pub alpha: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelBindEventQuery {
            pub data_model_handle: u64,
            pub name: String,
            pub callback: u32,
            pub user_data: u32,
            pub destroy_callback: u32,
            pub field_types: Vec<u8>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelBindEventResult {
            pub event_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelBindFloatQuery {
            pub data_model_handle: u64,
            pub name: String,
            pub initial_value: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelBindIntQuery {
            pub data_model_handle: u64,
            pub name: String,
            pub initial_value: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelBindPercentQuery {
            pub data_model_handle: u64,
            pub name: String,
            pub initial_value: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelBindPixelsQuery {
            pub data_model_handle: u64,
            pub name: String,
            pub initial_value: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelBindResult {
            pub variable_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelBindRowsQuery {
            pub data_model_handle: u64,
            pub name: String,
            pub fields: Vec<RmlDataFieldDef>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelBindStringQuery {
            pub data_model_handle: u64,
            pub name: String,
            pub initial_value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelEventHandleQuery {
            pub event_handle: u64,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelGetBoolResult {
            pub value: bool,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelGetColorResult {
            pub red: u8,
            pub green: u8,
            pub blue: u8,
            pub alpha: u8,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelGetFloatResult {
            pub value: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelGetIntResult {
            pub value: i32,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelGetPercentResult {
            pub value: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelGetPixelsResult {
            pub value: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelGetStringResult {
            pub value: String,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelRowsResult {
            pub rows_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelSetRowsQuery {
            pub rows_handle: u64,
            pub values: Vec<RmlDataValue>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelVariableBoolQuery {
            pub variable_handle: u64,
            pub value: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelVariableColorQuery {
            pub variable_handle: u64,
            pub red: u8,
            pub green: u8,
            pub blue: u8,
            pub alpha: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelVariableFloatQuery {
            pub variable_handle: u64,
            pub value: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelVariableHandleQuery {
            pub variable_handle: u64,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelVariableIntQuery {
            pub variable_handle: u64,
            pub value: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelVariablePercentQuery {
            pub variable_handle: u64,
            pub value: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelVariablePixelsQuery {
            pub variable_handle: u64,
            pub value: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelVariableStringQuery {
            pub variable_handle: u64,
            pub value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataValue {
            pub type_: u8,
            pub bool_value: bool,
            pub int_value: i32,
            pub float_value: f32,
            pub string_value: String,
            pub red: u8,
            pub green: u8,
            pub blue: u8,
            pub alpha: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDocumentBoolResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDocumentCreateElementQuery {
            pub document_handle: u64,
            pub tag_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDocumentCreateElementResult {
            pub element_ptr_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDocumentGetContextResult {
            pub context_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDocumentGetStringResult {
            pub value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDocumentHandleQuery {
            pub document_handle: u64,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDocumentInlineScriptQuery {
            pub document_handle: u64,
            pub content: String,
            pub source_path: String,
            pub source_line: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDocumentSetTitleQuery {
            pub document_handle: u64,
            pub title: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDocumentShowOptions {
            pub modal: Option<i32>,
            pub focus: Option<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDocumentShowQuery {
            pub document_handle: u64,
            pub options: RmlDocumentShowOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDocumentStringQuery {
            pub document_handle: u64,
            pub value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementAppendChildQuery {
            pub element_handle: u64,
            pub element_ptr_handle: u64,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementBoolResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementChildQuery {
            pub element_handle: u64,
            pub child_element_handle: u64,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementDispatchEventQuery {
            pub element_handle: u64,
            pub event: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementEventListenerRemoveQuery {
            pub element_handle: u64,
            pub event_listener_handle: u64,
            pub event: String,
            pub in_capture_phase: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementFormControlSelectAddQuery {
            pub element_handle: u64,
            pub element_ptr_handle: u64,
            pub before: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementFormControlSelectRemoveQuery {
            pub element_handle: u64,
            pub index: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementFormControlSelectionQuery {
            pub element_handle: u64,
            pub start: i32,
            pub end: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementFormControlSelectionResult {
            pub start: i32,
            pub end: i32,
            pub text: String,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementFormSubmitQuery {
            pub element_handle: u64,
            pub name: String,
            pub value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementGetAttributeQuery {
            pub element_handle: u64,
            pub name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementGetAttributeResult {
            pub value: String,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementGetByStringQuery {
            pub element_handle: u64,
            pub value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementGetChildQuery {
            pub element_handle: u64,
            pub index: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementGetElementResult {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementGetFloatResult {
            pub value: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementGetIntResult {
            pub value: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementGetRectResult {
            pub left: f32,
            pub top: f32,
            pub width: f32,
            pub height: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementGetStringResult {
            pub value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementHandleListResult {
            pub element_handles: Vec<u64>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementHandleQuery {
            pub element_handle: u64,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementInsertBeforeQuery {
            pub element_handle: u64,
            pub element_ptr_handle: u64,
            pub adjacent_element_handle: u64,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementPointQuery {
            pub element_handle: u64,
            pub x: f32,
            pub y: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementProcessDefaultActionQuery {
            pub element_handle: u64,
            pub event_handle: u64,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementReplaceChildQuery {
            pub element_handle: u64,
            pub element_ptr_handle: u64,
            pub replaced_element_handle: u64,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementScrollIntoViewQuery {
            pub element_handle: u64,
            pub align_with_top: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementSetAttributeQuery {
            pub element_handle: u64,
            pub name: String,
            pub value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementSetFloatQuery {
            pub element_handle: u64,
            pub value: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementSetIntQuery {
            pub element_handle: u64,
            pub value: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementSetStringQuery {
            pub element_handle: u64,
            pub value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementStringBoolQuery {
            pub element_handle: u64,
            pub name: String,
            pub value: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementStringListResult {
            pub values: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementStringQuery {
            pub element_handle: u64,
            pub value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementTabSetIndexQuery {
            pub element_handle: u64,
            pub index: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlElementTabSetIndexStringQuery {
            pub element_handle: u64,
            pub index: i32,
            pub rml: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlEventCurrentQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlEventCurrentResult {
            pub event_handle: u64,
            pub element_handle: u64,
            pub document_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlEventGetBoolResult {
            pub value: bool,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlEventGetFloatResult {
            pub value: f32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlEventGetIntResult {
            pub value: i32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlEventGetStringResult {
            pub value: String,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlEventHandleQuery {
            pub event_handle: u64,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlEventListenerCallbackQuery {
            pub element_handle: u64,
            pub event: String,
            pub in_capture_phase: bool,
            pub callback: u32,
            pub user_data: u32,
            pub destroy_callback: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlEventListenerCallbackResult {
            pub event_listener_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlEventListenerElementQuery {
            pub event_listener_handle: u64,
            pub element_handle: u64,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlEventListenerEventQuery {
            pub event_listener_handle: u64,
            pub event_handle: u64,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlEventListenerHandleQuery {
            pub event_listener_handle: u64,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlEventParameterQuery {
            pub event_handle: u64,
            pub name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlGetContextQuery {
            pub name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlGetContextResult {
            pub context_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlGetDocumentPathRequestsQuery {
            pub document_path: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlGetDocumentPathRequestsResult {
            pub paths: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlGetVersionQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlGetVersionResult {
            pub version: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlIsReadyQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlIsReadyResult {
            pub ready: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlLoadFontFaceQuery {
            pub file_path: String,
            pub fallback: bool,
            pub weight: Option<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlLoadFontFaceResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlRegisterEventTypeOptions {
            pub interruptible: bool,
            pub bubbles: bool,
            pub default_phase: Option<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlRegisterEventTypeQuery {
            pub event_type: String,
            pub options: RmlRegisterEventTypeOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlRegisterEventTypeResult {
            pub event_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlRemoveContextByNameQuery {
            pub name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlRemoveContextByNameResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlRemoveContextQuery {
            pub context_handle: u64,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlRemoveContextResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlSetDebugContextByNameQuery {
            pub name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlSetDebugContextByNameResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlSetDebugContextQuery {
            pub context_handle: u64,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlSetDebugContextResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlSetMouseCursorAliasQuery {
            pub rml_name: String,
            pub recoil_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlSetMouseCursorAliasResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlSolLuaDataModelSetDirtyQuery {
            pub data_model_handle: u64,
            pub property: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlVector2fNewQuery {
            pub x: f32,
            pub y: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlVector2fNewResult {
            pub x: f32,
            pub y: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlVector2iNewQuery {
            pub x: i32,
            pub y: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlVector2iNewResult {
            pub x: i32,
            pub y: i32,
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
        mod __core_variable_output_context_get_name {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "context-get-name"]
                pub fn call(pcontext_handle: i64, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_document_get_title {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "document-get-title"]
                pub fn call(pdocument_handle: i64, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_document_get_url {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "document-get-url"]
                pub fn call(pdocument_handle: i64, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_element_get_class_name {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "element-get-class-name"]
                pub fn call(pelement_handle: i64, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_element_get_id {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "element-get-id"]
                pub fn call(pelement_handle: i64, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_element_get_inner_rml {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "element-get-inner-rml"]
                pub fn call(pelement_handle: i64, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_element_get_tag_name {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "element-get-tag-name"]
                pub fn call(pelement_handle: i64, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_element_get_value {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "element-get-value"]
                pub fn call(pelement_handle: i64, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_version {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "get-version"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ContextAddEventListenerValue {
            pub event_listener_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ContextCreateDataModelValue {
            pub data_model_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ContextCreateDocumentValue {
            pub document_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ContextGetDimensionsValue {
            pub x: i32,
            pub y: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ContextGetDocumentValue {
            pub document_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ContextGetElementAtPointValue {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ContextGetFocusElementValue {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ContextGetHoverElementValue {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ContextGetRootElementValue {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ContextLoadDocumentValue {
            pub document_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ContextOpenDataModelValue {
            pub data_model_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ContextTakePointerCaptureDeltaValue {
            pub delta_x: i32,
            pub delta_y: i32,
            pub status: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CreateContextValue {
            pub context_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DataModelBindBoolValue {
            pub variable_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DataModelBindColorValue {
            pub variable_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DataModelBindEventValue {
            pub event_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DataModelBindFloatValue {
            pub variable_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DataModelBindIntValue {
            pub variable_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DataModelBindPercentValue {
            pub variable_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DataModelBindPixelsValue {
            pub variable_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DataModelBindRowsValue {
            pub rows_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DataModelBindStringValue {
            pub variable_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DataModelGetBoolValue {
            pub value: bool,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DataModelGetColorValue {
            pub red: u8,
            pub green: u8,
            pub blue: u8,
            pub alpha: u8,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DataModelGetFloatValue {
            pub value: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DataModelGetIntValue {
            pub value: i32,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DataModelGetPercentValue {
            pub value: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DataModelGetPixelsValue {
            pub value: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DataModelGetStringValue {
            pub value: String,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DocumentCreateElementValue {
            pub element_ptr_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DocumentCreateTextNodeValue {
            pub element_ptr_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DocumentGetContextValue {
            pub context_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ElementAddEventListenerValue {
            pub event_listener_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ElementAppendChildValue {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ElementCloneValue {
            pub element_ptr_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ElementClosestValue {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ElementFormControlInputGetSelectionValue {
            pub start: i32,
            pub end: i32,
            pub text: String,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ElementFormControlTextAreaGetSelectionValue {
            pub start: i32,
            pub end: i32,
            pub text: String,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ElementGetAttributeValue {
            pub value: String,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ElementGetChildValue {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ElementGetElementByIdValue {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ElementGetRectValue {
            pub left: f32,
            pub top: f32,
            pub width: f32,
            pub height: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ElementInsertBeforeValue {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ElementQuerySelectorValue {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ElementRemoveChildValue {
            pub element_ptr_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ElementReplaceChildValue {
            pub element_ptr_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EventGetCurrentValue {
            pub event_handle: u64,
            pub element_handle: u64,
            pub document_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EventGetCurrentElementValue {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EventGetParameterBoolValue {
            pub value: bool,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EventGetParameterFloatValue {
            pub value: f32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EventGetParameterIntValue {
            pub value: i32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EventGetParameterStringValue {
            pub value: String,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EventGetParameterTypeValue {
            pub value: i32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EventGetPhaseValue {
            pub value: i32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EventGetTargetElementValue {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EventGetTypeValue {
            pub value: String,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EventIsImmediatePropagatingValue {
            pub value: bool,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EventIsInterruptibleValue {
            pub value: bool,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EventIsPropagatingValue {
            pub value: bool,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetContextValue {
            pub context_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Vector2fNewValue {
            pub x: f32,
            pub y: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Vector2iNewValue {
            pub x: i32,
            pub y: i32,
        }

        #[inline]
        pub fn add_translation_string(key: &str, translation: &str) -> Result<bool> {
            let mut key_bytes = key.as_bytes().to_vec();
            if key_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            key_bytes.push(0);
            let key_cstr = core::ffi::CStr::from_bytes_with_nul(&key_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            let mut translation_bytes = translation.as_bytes().to_vec();
            if translation_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            translation_bytes.push(0);
            let translation_cstr = core::ffi::CStr::from_bytes_with_nul(&translation_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::add_translation_string(&key_cstr, &translation_cstr)
        }

        #[inline]
        pub fn clear_document_path_requests(document_path: &str) -> Result<bool> {
            let mut document_path_bytes = document_path.as_bytes().to_vec();
            if document_path_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            document_path_bytes.push(0);
            let document_path_cstr = core::ffi::CStr::from_bytes_with_nul(&document_path_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::clear_document_path_requests(&document_path_cstr)
        }

        #[inline]
        pub fn clear_translations(unused: u8) -> Result<bool> {
            let value = crate::generated::rml_ui::clear_translations(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn context_activate_theme(context_handle: u64, name: &str, value: bool) -> Result<bool> {
            let mut name_bytes = name.as_bytes().to_vec();
            if name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            name_bytes.push(0);
            let name_cstr = core::ffi::CStr::from_bytes_with_nul(&name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::context_activate_theme(context_handle, &name_cstr, value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_context_add_event_listener {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "context-add-event-listener"]
                pub fn call(p0: i64, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.context-add-event-listener."]
        #[inline]
        pub unsafe fn context_add_event_listener(p0: i64, p1: i32, p2: i32, p3: i32) -> i32 {
            unsafe { __core_owned_context_add_event_listener::call(p0, p1, p2, p3) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_context_create_data_model {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "context-create-data-model"]
                pub fn call(p0: i64, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.context-create-data-model."]
        #[inline]
        pub unsafe fn context_create_data_model(p0: i64, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_context_create_data_model::call(p0, p1, p2) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_context_create_document {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "context-create-document"]
                pub fn call(p0: i64, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.context-create-document."]
        #[inline]
        pub unsafe fn context_create_document(p0: i64, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_context_create_document::call(p0, p1, p2) }
        }

        #[inline]
        pub fn context_enable_mouse_cursor(context_handle: u64, value: bool) -> Result<bool> {
            let value = crate::generated::rml_ui::context_enable_mouse_cursor(context_handle, value)?;
            Ok(value)
        }

        #[inline]
        pub fn context_get_density_independent_pixel_ratio(context_handle: u64) -> Result<f32> {
            let value = crate::generated::rml_ui::context_get_density_independent_pixel_ratio(context_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn context_get_dimensions(context_handle: u64) -> Result<ContextGetDimensionsValue> {
            let value = crate::generated::rml_ui::context_get_dimensions(context_handle)?;
            Ok(ContextGetDimensionsValue {
                x: value.0,
                y: value.1
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_context_get_document {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "context-get-document"]
                pub fn call(p0: i64, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.context-get-document."]
        #[inline]
        pub unsafe fn context_get_document(p0: i64, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_context_get_document::call(p0, p1, p2) }
        }

        #[inline]
        pub fn context_get_element_at_point(context_handle: u64, x: f32, y: f32, ignore_element_handle: u64) -> Result<ContextGetElementAtPointValue> {
            let value = crate::generated::rml_ui::context_get_element_at_point(context_handle, x, y, ignore_element_handle)?;
            Ok(ContextGetElementAtPointValue {
                element_handle: value.0,
                exists: value.1
            })
        }

        #[inline]
        pub fn context_get_focus_element(context_handle: u64) -> Result<ContextGetFocusElementValue> {
            let value = crate::generated::rml_ui::context_get_focus_element(context_handle)?;
            Ok(ContextGetFocusElementValue {
                element_handle: value.0,
                exists: value.1
            })
        }

        #[inline]
        pub fn context_get_hover_element(context_handle: u64) -> Result<ContextGetHoverElementValue> {
            let value = crate::generated::rml_ui::context_get_hover_element(context_handle)?;
            Ok(ContextGetHoverElementValue {
                element_handle: value.0,
                exists: value.1
            })
        }

        #[inline]
        pub fn context_get_name(context_handle: u64) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_context_get_name::call(context_handle as i64, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (context_handle as i64);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn context_get_root_element(context_handle: u64) -> Result<ContextGetRootElementValue> {
            let value = crate::generated::rml_ui::context_get_root_element(context_handle)?;
            Ok(ContextGetRootElementValue {
                element_handle: value.0,
                exists: value.1
            })
        }

        #[inline]
        pub fn context_is_mouse_interacting(context_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::context_is_mouse_interacting(context_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn context_is_theme_active(context_handle: u64, name: &str) -> Result<bool> {
            let mut name_bytes = name.as_bytes().to_vec();
            if name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            name_bytes.push(0);
            let name_cstr = core::ffi::CStr::from_bytes_with_nul(&name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::context_is_theme_active(context_handle, &name_cstr)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_context_load_document {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "context-load-document"]
                pub fn call(p0: i64, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.context-load-document."]
        #[inline]
        pub unsafe fn context_load_document(p0: i64, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_context_load_document::call(p0, p1, p2) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_context_open_data_model {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "context-open-data-model"]
                pub fn call(p0: i64, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.context-open-data-model."]
        #[inline]
        pub unsafe fn context_open_data_model(p0: i64, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_context_open_data_model::call(p0, p1, p2) }
        }

        #[inline]
        pub fn context_process_key_down(context_handle: u64, key: i32, key_modifier_state: i32) -> Result<bool> {
            let value = crate::generated::rml_ui::context_process_key_down(context_handle, key, key_modifier_state)?;
            Ok(value)
        }

        #[inline]
        pub fn context_process_key_up(context_handle: u64, key: i32, key_modifier_state: i32) -> Result<bool> {
            let value = crate::generated::rml_ui::context_process_key_up(context_handle, key, key_modifier_state)?;
            Ok(value)
        }

        #[inline]
        pub fn context_process_mouse_button_down(context_handle: u64, button: i32, key_modifier_state: i32) -> Result<bool> {
            let value = crate::generated::rml_ui::context_process_mouse_button_down(context_handle, button, key_modifier_state)?;
            Ok(value)
        }

        #[inline]
        pub fn context_process_mouse_button_up(context_handle: u64, button: i32, key_modifier_state: i32) -> Result<bool> {
            let value = crate::generated::rml_ui::context_process_mouse_button_up(context_handle, button, key_modifier_state)?;
            Ok(value)
        }

        #[inline]
        pub fn context_process_mouse_leave(context_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::context_process_mouse_leave(context_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn context_process_mouse_move(context_handle: u64, x: f32, y: f32, key_modifier_state: i32) -> Result<bool> {
            let value = crate::generated::rml_ui::context_process_mouse_move(context_handle, x, y, key_modifier_state)?;
            Ok(value)
        }

        #[inline]
        pub fn context_process_mouse_wheel(context_handle: u64, x: f32, y: f32, key_modifier_state: i32) -> Result<bool> {
            let value = crate::generated::rml_ui::context_process_mouse_wheel(context_handle, x, y, key_modifier_state)?;
            Ok(value)
        }

        #[inline]
        pub fn context_process_text_input(context_handle: u64, text: &str) -> Result<bool> {
            let mut text_bytes = text.as_bytes().to_vec();
            if text_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            text_bytes.push(0);
            let text_cstr = core::ffi::CStr::from_bytes_with_nul(&text_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::context_process_text_input(context_handle, &text_cstr)
        }

        #[inline]
        pub fn context_pull_document_to_front(context_handle: u64, document_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::context_pull_document_to_front(context_handle, document_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn context_pull_to_front(context_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::context_pull_to_front(context_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn context_push_document_to_back(context_handle: u64, document_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::context_push_document_to_back(context_handle, document_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn context_remove_data_model(context_handle: u64, name: &str) -> Result<bool> {
            let mut name_bytes = name.as_bytes().to_vec();
            if name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            name_bytes.push(0);
            let name_cstr = core::ffi::CStr::from_bytes_with_nul(&name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::context_remove_data_model(context_handle, &name_cstr)
        }

        #[inline]
        pub fn context_remove_event_listener(context_handle: u64, event_listener_handle: u64, event: &str, in_capture_phase: bool) -> Result<bool> {
            let mut event_bytes = event.as_bytes().to_vec();
            if event_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            event_bytes.push(0);
            let event_cstr = core::ffi::CStr::from_bytes_with_nul(&event_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::context_remove_event_listener(context_handle, event_listener_handle, &event_cstr, in_capture_phase)
        }

        #[inline]
        pub fn context_render(context_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::context_render(context_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn context_set_density_independent_pixel_ratio(context_handle: u64, value: f32) -> Result<bool> {
            let value = crate::generated::rml_ui::context_set_density_independent_pixel_ratio(context_handle, value)?;
            Ok(value)
        }

        #[inline]
        pub fn context_set_dimensions(context_handle: u64, x: i32, y: i32) -> Result<bool> {
            let value = crate::generated::rml_ui::context_set_dimensions(context_handle, x, y)?;
            Ok(value)
        }

        #[inline]
        pub fn context_set_pointer_capture(context_handle: u64, anchor_x: i32, anchor_y: i32, active: bool) -> Result<bool> {
            let value = crate::generated::rml_ui::context_set_pointer_capture(context_handle, anchor_x, anchor_y, active)?;
            Ok(value)
        }

        #[inline]
        pub fn context_take_pointer_capture_delta(context_handle: u64) -> Result<ContextTakePointerCaptureDeltaValue> {
            let value = crate::generated::rml_ui::context_take_pointer_capture_delta(context_handle)?;
            Ok(ContextTakePointerCaptureDeltaValue {
                delta_x: value.0,
                delta_y: value.1,
                status: value.2
            })
        }

        #[inline]
        pub fn context_unload_all_documents(context_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::context_unload_all_documents(context_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn context_unload_document(context_handle: u64, document_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::context_unload_document(context_handle, document_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn context_update(context_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::context_update(context_handle)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_create_context {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "create-context"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.create-context."]
        #[inline]
        pub unsafe fn create_context(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_create_context::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_data_model_bind_bool {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "data-model-bind-bool"]
                pub fn call(p0: i64, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.data-model-bind-bool."]
        #[inline]
        pub unsafe fn data_model_bind_bool(p0: i64, p1: i32, p2: i32, p3: i32) -> i32 {
            unsafe { __core_owned_data_model_bind_bool::call(p0, p1, p2, p3) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_data_model_bind_color {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "data-model-bind-color"]
                pub fn call(p0: i64, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.data-model-bind-color."]
        #[inline]
        pub unsafe fn data_model_bind_color(p0: i64, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32) -> i32 {
            unsafe { __core_owned_data_model_bind_color::call(p0, p1, p2, p3, p4, p5, p6) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_data_model_bind_event {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "data-model-bind-event"]
                pub fn call(p0: i64, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.data-model-bind-event."]
        #[inline]
        pub unsafe fn data_model_bind_event(p0: i64, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_data_model_bind_event::call(p0, p1, p2) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_data_model_bind_float {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "data-model-bind-float"]
                pub fn call(p0: i64, p1: f32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.data-model-bind-float."]
        #[inline]
        pub unsafe fn data_model_bind_float(p0: i64, p1: f32, p2: i32, p3: i32) -> i32 {
            unsafe { __core_owned_data_model_bind_float::call(p0, p1, p2, p3) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_data_model_bind_int {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "data-model-bind-int"]
                pub fn call(p0: i64, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.data-model-bind-int."]
        #[inline]
        pub unsafe fn data_model_bind_int(p0: i64, p1: i32, p2: i32, p3: i32) -> i32 {
            unsafe { __core_owned_data_model_bind_int::call(p0, p1, p2, p3) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_data_model_bind_percent {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "data-model-bind-percent"]
                pub fn call(p0: i64, p1: f32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.data-model-bind-percent."]
        #[inline]
        pub unsafe fn data_model_bind_percent(p0: i64, p1: f32, p2: i32, p3: i32) -> i32 {
            unsafe { __core_owned_data_model_bind_percent::call(p0, p1, p2, p3) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_data_model_bind_pixels {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "data-model-bind-pixels"]
                pub fn call(p0: i64, p1: f32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.data-model-bind-pixels."]
        #[inline]
        pub unsafe fn data_model_bind_pixels(p0: i64, p1: f32, p2: i32, p3: i32) -> i32 {
            unsafe { __core_owned_data_model_bind_pixels::call(p0, p1, p2, p3) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_data_model_bind_rows {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "data-model-bind-rows"]
                pub fn call(p0: i64, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.data-model-bind-rows."]
        #[inline]
        pub unsafe fn data_model_bind_rows(p0: i64, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_data_model_bind_rows::call(p0, p1, p2) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_data_model_bind_string {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "data-model-bind-string"]
                pub fn call(p0: i64, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.data-model-bind-string."]
        #[inline]
        pub unsafe fn data_model_bind_string(p0: i64, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_data_model_bind_string::call(p0, p1, p2) }
        }

        #[inline]
        pub fn data_model_get_bool(variable_handle: u64) -> Result<DataModelGetBoolValue> {
            let value = crate::generated::rml_ui::data_model_get_bool(variable_handle)?;
            Ok(DataModelGetBoolValue {
                value: value.0,
                success: value.1
            })
        }

        #[inline]
        pub fn data_model_get_color(variable_handle: u64) -> Result<DataModelGetColorValue> {
            let value = crate::generated::rml_ui::data_model_get_color(variable_handle)?;
            Ok(DataModelGetColorValue {
                red: value.0,
                green: value.1,
                blue: value.2,
                alpha: value.3,
                success: value.4
            })
        }

        #[inline]
        pub fn data_model_get_float(variable_handle: u64) -> Result<DataModelGetFloatValue> {
            let value = crate::generated::rml_ui::data_model_get_float(variable_handle)?;
            Ok(DataModelGetFloatValue {
                value: value.0,
                success: value.1
            })
        }

        #[inline]
        pub fn data_model_get_int(variable_handle: u64) -> Result<DataModelGetIntValue> {
            let value = crate::generated::rml_ui::data_model_get_int(variable_handle)?;
            Ok(DataModelGetIntValue {
                value: value.0,
                success: value.1
            })
        }

        #[inline]
        pub fn data_model_get_percent(variable_handle: u64) -> Result<DataModelGetPercentValue> {
            let value = crate::generated::rml_ui::data_model_get_percent(variable_handle)?;
            Ok(DataModelGetPercentValue {
                value: value.0,
                success: value.1
            })
        }

        #[inline]
        pub fn data_model_get_pixels(variable_handle: u64) -> Result<DataModelGetPixelsValue> {
            let value = crate::generated::rml_ui::data_model_get_pixels(variable_handle)?;
            Ok(DataModelGetPixelsValue {
                value: value.0,
                success: value.1
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_data_model_get_string {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "data-model-get-string"]
                pub fn call(p0: i64, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.data-model-get-string."]
        #[inline]
        pub unsafe fn data_model_get_string(p0: i64, p1: i32) -> i32 {
            unsafe { __core_owned_data_model_get_string::call(p0, p1) }
        }

        #[inline]
        pub fn data_model_set_bool(variable_handle: u64, value: bool) -> Result<bool> {
            let value = crate::generated::rml_ui::data_model_set_bool(variable_handle, value)?;
            Ok(value)
        }

        #[inline]
        pub fn data_model_set_color(variable_handle: u64, red: u8, green: u8, blue: u8, alpha: u8) -> Result<bool> {
            let value = crate::generated::rml_ui::data_model_set_color(variable_handle, red, green, blue, alpha)?;
            Ok(value)
        }

        #[inline]
        pub fn data_model_set_float(variable_handle: u64, value: f32) -> Result<bool> {
            let value = crate::generated::rml_ui::data_model_set_float(variable_handle, value)?;
            Ok(value)
        }

        #[inline]
        pub fn data_model_set_int(variable_handle: u64, value: i32) -> Result<bool> {
            let value = crate::generated::rml_ui::data_model_set_int(variable_handle, value)?;
            Ok(value)
        }

        #[inline]
        pub fn data_model_set_percent(variable_handle: u64, value: f32) -> Result<bool> {
            let value = crate::generated::rml_ui::data_model_set_percent(variable_handle, value)?;
            Ok(value)
        }

        #[inline]
        pub fn data_model_set_pixels(variable_handle: u64, value: f32) -> Result<bool> {
            let value = crate::generated::rml_ui::data_model_set_pixels(variable_handle, value)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_data_model_set_rows {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "data-model-set-rows"]
                pub fn call(p0: i64, p1: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.data-model-set-rows."]
        #[inline]
        pub unsafe fn data_model_set_rows(p0: i64, p1: i32) -> i64 {
            unsafe { __core_owned_data_model_set_rows::call(p0, p1) }
        }

        #[inline]
        pub fn data_model_set_string(variable_handle: u64, value: &str) -> Result<bool> {
            let mut value_bytes = value.as_bytes().to_vec();
            if value_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            value_bytes.push(0);
            let value_cstr = core::ffi::CStr::from_bytes_with_nul(&value_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::data_model_set_string(variable_handle, &value_cstr)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_data_model_unbind_event {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "data-model-unbind-event"]
                pub fn call(p0: i64) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.data-model-unbind-event."]
        #[inline]
        pub unsafe fn data_model_unbind_event(p0: i64) -> i64 {
            unsafe { __core_owned_data_model_unbind_event::call(p0) }
        }

        #[inline]
        pub fn document_append_to_style_sheet(document_handle: u64, value: &str) -> Result<bool> {
            let mut value_bytes = value.as_bytes().to_vec();
            if value_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            value_bytes.push(0);
            let value_cstr = core::ffi::CStr::from_bytes_with_nul(&value_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::document_append_to_style_sheet(document_handle, &value_cstr)
        }

        #[inline]
        pub fn document_close(document_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::document_close(document_handle)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_document_create_element {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "document-create-element"]
                pub fn call(p0: i64, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.document-create-element."]
        #[inline]
        pub unsafe fn document_create_element(p0: i64, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_document_create_element::call(p0, p1, p2) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_document_create_text_node {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "document-create-text-node"]
                pub fn call(p0: i64, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.document-create-text-node."]
        #[inline]
        pub unsafe fn document_create_text_node(p0: i64, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_document_create_text_node::call(p0, p1, p2) }
        }

        #[inline]
        pub fn document_get_context(document_handle: u64) -> Result<DocumentGetContextValue> {
            let value = crate::generated::rml_ui::document_get_context(document_handle)?;
            Ok(DocumentGetContextValue {
                context_handle: value.0,
                exists: value.1
            })
        }

        #[inline]
        pub fn document_get_title(document_handle: u64) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_document_get_title::call(document_handle as i64, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (document_handle as i64);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn document_get_url(document_handle: u64) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_document_get_url::call(document_handle as i64, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (document_handle as i64);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn document_hide(document_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::document_hide(document_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn document_is_modal(document_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::document_is_modal(document_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn document_load_external_script(document_handle: u64, value: &str) -> Result<bool> {
            let mut value_bytes = value.as_bytes().to_vec();
            if value_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            value_bytes.push(0);
            let value_cstr = core::ffi::CStr::from_bytes_with_nul(&value_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::document_load_external_script(document_handle, &value_cstr)
        }

        #[inline]
        pub fn document_load_inline_script(document_handle: u64, content: &str, source_path: &str, source_line: i32) -> Result<bool> {
            let mut content_bytes = content.as_bytes().to_vec();
            if content_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            content_bytes.push(0);
            let content_cstr = core::ffi::CStr::from_bytes_with_nul(&content_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            let mut source_path_bytes = source_path.as_bytes().to_vec();
            if source_path_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            source_path_bytes.push(0);
            let source_path_cstr = core::ffi::CStr::from_bytes_with_nul(&source_path_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::document_load_inline_script(document_handle, &content_cstr, &source_path_cstr, source_line)
        }

        #[inline]
        pub fn document_pull_to_front(document_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::document_pull_to_front(document_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn document_push_to_back(document_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::document_push_to_back(document_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn document_reload_style_sheet(document_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::document_reload_style_sheet(document_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn document_set_title(document_handle: u64, title: &str) -> Result<bool> {
            let mut title_bytes = title.as_bytes().to_vec();
            if title_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            title_bytes.push(0);
            let title_cstr = core::ffi::CStr::from_bytes_with_nul(&title_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::document_set_title(document_handle, &title_cstr)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_document_show {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "document-show"]
                pub fn call(p0: i64, p1: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.document-show."]
        #[inline]
        pub unsafe fn document_show(p0: i64, p1: i32) -> i64 {
            unsafe { __core_owned_document_show::call(p0, p1) }
        }

        #[inline]
        pub fn document_update_document(document_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::document_update_document(document_handle)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_element_add_event_listener {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "element-add-event-listener"]
                pub fn call(p0: i64, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.element-add-event-listener."]
        #[inline]
        pub unsafe fn element_add_event_listener(p0: i64, p1: i32, p2: i32, p3: i32) -> i32 {
            unsafe { __core_owned_element_add_event_listener::call(p0, p1, p2, p3) }
        }

        #[inline]
        pub fn element_append_child(element_handle: u64, element_ptr_handle: u64) -> Result<ElementAppendChildValue> {
            let value = crate::generated::rml_ui::element_append_child(element_handle, element_ptr_handle)?;
            Ok(ElementAppendChildValue {
                element_handle: value.0,
                exists: value.1
            })
        }

        #[inline]
        pub fn element_are_pseudo_classes_set(element_handle: u64, value: &str) -> Result<bool> {
            let mut value_bytes = value.as_bytes().to_vec();
            if value_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            value_bytes.push(0);
            let value_cstr = core::ffi::CStr::from_bytes_with_nul(&value_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::element_are_pseudo_classes_set(element_handle, &value_cstr)
        }

        #[inline]
        pub fn element_blur(element_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::element_blur(element_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn element_click(element_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::element_click(element_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn element_clone(element_handle: u64) -> Result<ElementCloneValue> {
            let value = crate::generated::rml_ui::element_clone(element_handle)?;
            Ok(ElementCloneValue {
                element_ptr_handle: value.0,
                success: value.1
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_element_closest {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "element-closest"]
                pub fn call(p0: i64, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.element-closest."]
        #[inline]
        pub unsafe fn element_closest(p0: i64, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_element_closest::call(p0, p1, p2) }
        }

        #[inline]
        pub fn element_dispatch_event(element_handle: u64, event: &str) -> Result<bool> {
            let mut event_bytes = event.as_bytes().to_vec();
            if event_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            event_bytes.push(0);
            let event_cstr = core::ffi::CStr::from_bytes_with_nul(&event_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::element_dispatch_event(element_handle, &event_cstr)
        }

        #[inline]
        pub fn element_focus(element_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::element_focus(element_handle)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_element_form_control_input_get_selection {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "element-form-control-input-get-selection"]
                pub fn call(p0: i64, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.element-form-control-input-get-selection."]
        #[inline]
        pub unsafe fn element_form_control_input_get_selection(p0: i64, p1: i32) -> i32 {
            unsafe { __core_owned_element_form_control_input_get_selection::call(p0, p1) }
        }

        #[inline]
        pub fn element_form_control_input_select(element_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::element_form_control_input_select(element_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn element_form_control_input_set_selection(element_handle: u64, start: i32, end: i32) -> Result<bool> {
            let value = crate::generated::rml_ui::element_form_control_input_set_selection(element_handle, start, end)?;
            Ok(value)
        }

        #[inline]
        pub fn element_form_control_select_add(element_handle: u64, element_ptr_handle: u64, before: i32) -> Result<bool> {
            let value = crate::generated::rml_ui::element_form_control_select_add(element_handle, element_ptr_handle, before)?;
            Ok(value)
        }

        #[inline]
        pub fn element_form_control_select_remove(element_handle: u64, index: i32) -> Result<bool> {
            let value = crate::generated::rml_ui::element_form_control_select_remove(element_handle, index)?;
            Ok(value)
        }

        #[inline]
        pub fn element_form_control_select_remove_all(element_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::element_form_control_select_remove_all(element_handle)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_element_form_control_text_area_get_selection {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "element-form-control-text-area-get-selection"]
                pub fn call(p0: i64, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.element-form-control-text-area-get-selection."]
        #[inline]
        pub unsafe fn element_form_control_text_area_get_selection(p0: i64, p1: i32) -> i32 {
            unsafe { __core_owned_element_form_control_text_area_get_selection::call(p0, p1) }
        }

        #[inline]
        pub fn element_form_control_text_area_select(element_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::element_form_control_text_area_select(element_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn element_form_control_text_area_set_selection(element_handle: u64, start: i32, end: i32) -> Result<bool> {
            let value = crate::generated::rml_ui::element_form_control_text_area_set_selection(element_handle, start, end)?;
            Ok(value)
        }

        #[inline]
        pub fn element_form_submit(element_handle: u64, name: &str, value: &str) -> Result<bool> {
            let mut name_bytes = name.as_bytes().to_vec();
            if name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            name_bytes.push(0);
            let name_cstr = core::ffi::CStr::from_bytes_with_nul(&name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            let mut value_bytes = value.as_bytes().to_vec();
            if value_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            value_bytes.push(0);
            let value_cstr = core::ffi::CStr::from_bytes_with_nul(&value_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::element_form_submit(element_handle, &name_cstr, &value_cstr)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_element_get_active_pseudo_classes {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "element-get-active-pseudo-classes"]
                pub fn call(p0: i64, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.element-get-active-pseudo-classes."]
        #[inline]
        pub unsafe fn element_get_active_pseudo_classes(p0: i64, p1: i32) -> i32 {
            unsafe { __core_owned_element_get_active_pseudo_classes::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_element_get_attribute {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "element-get-attribute"]
                pub fn call(p0: i64, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.element-get-attribute."]
        #[inline]
        pub unsafe fn element_get_attribute(p0: i64, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_element_get_attribute::call(p0, p1, p2) }
        }

        #[inline]
        pub fn element_get_child(element_handle: u64, index: i32) -> Result<ElementGetChildValue> {
            let value = crate::generated::rml_ui::element_get_child(element_handle, index)?;
            Ok(ElementGetChildValue {
                element_handle: value.0,
                exists: value.1
            })
        }

        #[inline]
        pub fn element_get_class_name(element_handle: u64) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_element_get_class_name::call(element_handle as i64, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (element_handle as i64);
                Err(unreachable!())
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_element_get_element_by_id {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "element-get-element-by-id"]
                pub fn call(p0: i64, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.element-get-element-by-id."]
        #[inline]
        pub unsafe fn element_get_element_by_id(p0: i64, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_element_get_element_by_id::call(p0, p1, p2) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_element_get_elements_by_class_name {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "element-get-elements-by-class-name"]
                pub fn call(p0: i64, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.element-get-elements-by-class-name."]
        #[inline]
        pub unsafe fn element_get_elements_by_class_name(p0: i64, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_element_get_elements_by_class_name::call(p0, p1, p2) }
        }

        #[inline]
        pub fn element_get_elements_by_class_name_count(element_handle: u64, value: &str) -> Result<i32> {
            let mut value_bytes = value.as_bytes().to_vec();
            if value_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            value_bytes.push(0);
            let value_cstr = core::ffi::CStr::from_bytes_with_nul(&value_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::element_get_elements_by_class_name_count(element_handle, &value_cstr)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_element_get_elements_by_tag_name {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "element-get-elements-by-tag-name"]
                pub fn call(p0: i64, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.element-get-elements-by-tag-name."]
        #[inline]
        pub unsafe fn element_get_elements_by_tag_name(p0: i64, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_element_get_elements_by_tag_name::call(p0, p1, p2) }
        }

        #[inline]
        pub fn element_get_elements_by_tag_name_count(element_handle: u64, value: &str) -> Result<i32> {
            let mut value_bytes = value.as_bytes().to_vec();
            if value_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            value_bytes.push(0);
            let value_cstr = core::ffi::CStr::from_bytes_with_nul(&value_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::element_get_elements_by_tag_name_count(element_handle, &value_cstr)
        }

        #[inline]
        pub fn element_get_id(element_handle: u64) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_element_get_id::call(element_handle as i64, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (element_handle as i64);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn element_get_inner_rml(element_handle: u64) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_element_get_inner_rml::call(element_handle as i64, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (element_handle as i64);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn element_get_rect(element_handle: u64) -> Result<ElementGetRectValue> {
            let value = crate::generated::rml_ui::element_get_rect(element_handle)?;
            Ok(ElementGetRectValue {
                left: value.0,
                top: value.1,
                width: value.2,
                height: value.3
            })
        }

        #[inline]
        pub fn element_get_scroll_left(element_handle: u64) -> Result<i32> {
            let value = crate::generated::rml_ui::element_get_scroll_left(element_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn element_get_scroll_top(element_handle: u64) -> Result<i32> {
            let value = crate::generated::rml_ui::element_get_scroll_top(element_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn element_get_tag_name(element_handle: u64) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_element_get_tag_name::call(element_handle as i64, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (element_handle as i64);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn element_get_value(element_handle: u64) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_element_get_value::call(element_handle as i64, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (element_handle as i64);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn element_has_attribute(element_handle: u64, value: &str) -> Result<bool> {
            let mut value_bytes = value.as_bytes().to_vec();
            if value_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            value_bytes.push(0);
            let value_cstr = core::ffi::CStr::from_bytes_with_nul(&value_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::element_has_attribute(element_handle, &value_cstr)
        }

        #[inline]
        pub fn element_has_child_nodes(element_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::element_has_child_nodes(element_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn element_insert_before(element_handle: u64, element_ptr_handle: u64, adjacent_element_handle: u64) -> Result<ElementInsertBeforeValue> {
            let value = crate::generated::rml_ui::element_insert_before(element_handle, element_ptr_handle, adjacent_element_handle)?;
            Ok(ElementInsertBeforeValue {
                element_handle: value.0,
                exists: value.1
            })
        }

        #[inline]
        pub fn element_is_class_set(element_handle: u64, value: &str) -> Result<bool> {
            let mut value_bytes = value.as_bytes().to_vec();
            if value_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            value_bytes.push(0);
            let value_cstr = core::ffi::CStr::from_bytes_with_nul(&value_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::element_is_class_set(element_handle, &value_cstr)
        }

        #[inline]
        pub fn element_is_point_within_element(element_handle: u64, x: f32, y: f32) -> Result<bool> {
            let value = crate::generated::rml_ui::element_is_point_within_element(element_handle, x, y)?;
            Ok(value)
        }

        #[inline]
        pub fn element_is_pseudo_class_set(element_handle: u64, value: &str) -> Result<bool> {
            let mut value_bytes = value.as_bytes().to_vec();
            if value_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            value_bytes.push(0);
            let value_cstr = core::ffi::CStr::from_bytes_with_nul(&value_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::element_is_pseudo_class_set(element_handle, &value_cstr)
        }

        #[inline]
        pub fn element_is_visible(element_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::element_is_visible(element_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn element_matches(element_handle: u64, value: &str) -> Result<bool> {
            let mut value_bytes = value.as_bytes().to_vec();
            if value_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            value_bytes.push(0);
            let value_cstr = core::ffi::CStr::from_bytes_with_nul(&value_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::element_matches(element_handle, &value_cstr)
        }

        #[inline]
        pub fn element_process_default_action(element_handle: u64, event_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::element_process_default_action(element_handle, event_handle)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_element_query_selector {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "element-query-selector"]
                pub fn call(p0: i64, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.element-query-selector."]
        #[inline]
        pub unsafe fn element_query_selector(p0: i64, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_element_query_selector::call(p0, p1, p2) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_element_query_selector_all {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "element-query-selector-all"]
                pub fn call(p0: i64, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.element-query-selector-all."]
        #[inline]
        pub unsafe fn element_query_selector_all(p0: i64, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_element_query_selector_all::call(p0, p1, p2) }
        }

        #[inline]
        pub fn element_query_selector_all_count(element_handle: u64, value: &str) -> Result<i32> {
            let mut value_bytes = value.as_bytes().to_vec();
            if value_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            value_bytes.push(0);
            let value_cstr = core::ffi::CStr::from_bytes_with_nul(&value_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::element_query_selector_all_count(element_handle, &value_cstr)
        }

        #[inline]
        pub fn element_remove_attribute(element_handle: u64, value: &str) -> Result<bool> {
            let mut value_bytes = value.as_bytes().to_vec();
            if value_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            value_bytes.push(0);
            let value_cstr = core::ffi::CStr::from_bytes_with_nul(&value_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::element_remove_attribute(element_handle, &value_cstr)
        }

        #[inline]
        pub fn element_remove_child(element_handle: u64, child_element_handle: u64) -> Result<ElementRemoveChildValue> {
            let value = crate::generated::rml_ui::element_remove_child(element_handle, child_element_handle)?;
            Ok(ElementRemoveChildValue {
                element_ptr_handle: value.0,
                success: value.1
            })
        }

        #[inline]
        pub fn element_remove_event_listener(element_handle: u64, event_listener_handle: u64, event: &str, in_capture_phase: bool) -> Result<bool> {
            let mut event_bytes = event.as_bytes().to_vec();
            if event_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            event_bytes.push(0);
            let event_cstr = core::ffi::CStr::from_bytes_with_nul(&event_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::element_remove_event_listener(element_handle, event_listener_handle, &event_cstr, in_capture_phase)
        }

        #[inline]
        pub fn element_replace_child(element_handle: u64, element_ptr_handle: u64, replaced_element_handle: u64) -> Result<ElementReplaceChildValue> {
            let value = crate::generated::rml_ui::element_replace_child(element_handle, element_ptr_handle, replaced_element_handle)?;
            Ok(ElementReplaceChildValue {
                element_ptr_handle: value.0,
                success: value.1
            })
        }

        #[inline]
        pub fn element_scroll_into_view(element_handle: u64, align_with_top: bool) -> Result<bool> {
            let value = crate::generated::rml_ui::element_scroll_into_view(element_handle, align_with_top)?;
            Ok(value)
        }

        #[inline]
        pub fn element_set_attribute(element_handle: u64, name: &str, value: &str) -> Result<bool> {
            let mut name_bytes = name.as_bytes().to_vec();
            if name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            name_bytes.push(0);
            let name_cstr = core::ffi::CStr::from_bytes_with_nul(&name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            let mut value_bytes = value.as_bytes().to_vec();
            if value_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            value_bytes.push(0);
            let value_cstr = core::ffi::CStr::from_bytes_with_nul(&value_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::element_set_attribute(element_handle, &name_cstr, &value_cstr)
        }

        #[inline]
        pub fn element_set_class(element_handle: u64, name: &str, value: bool) -> Result<bool> {
            let mut name_bytes = name.as_bytes().to_vec();
            if name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            name_bytes.push(0);
            let name_cstr = core::ffi::CStr::from_bytes_with_nul(&name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::element_set_class(element_handle, &name_cstr, value)
        }

        #[inline]
        pub fn element_set_class_name(element_handle: u64, value: &str) -> Result<bool> {
            let mut value_bytes = value.as_bytes().to_vec();
            if value_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            value_bytes.push(0);
            let value_cstr = core::ffi::CStr::from_bytes_with_nul(&value_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::element_set_class_name(element_handle, &value_cstr)
        }

        #[inline]
        pub fn element_set_id(element_handle: u64, value: &str) -> Result<bool> {
            let mut value_bytes = value.as_bytes().to_vec();
            if value_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            value_bytes.push(0);
            let value_cstr = core::ffi::CStr::from_bytes_with_nul(&value_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::element_set_id(element_handle, &value_cstr)
        }

        #[inline]
        pub fn element_set_inner_rml(element_handle: u64, value: &str) -> Result<bool> {
            let mut value_bytes = value.as_bytes().to_vec();
            if value_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            value_bytes.push(0);
            let value_cstr = core::ffi::CStr::from_bytes_with_nul(&value_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::element_set_inner_rml(element_handle, &value_cstr)
        }

        #[inline]
        pub fn element_set_pseudo_class(element_handle: u64, name: &str, value: bool) -> Result<bool> {
            let mut name_bytes = name.as_bytes().to_vec();
            if name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            name_bytes.push(0);
            let name_cstr = core::ffi::CStr::from_bytes_with_nul(&name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::element_set_pseudo_class(element_handle, &name_cstr, value)
        }

        #[inline]
        pub fn element_set_scroll_left(element_handle: u64, value: i32) -> Result<bool> {
            let value = crate::generated::rml_ui::element_set_scroll_left(element_handle, value)?;
            Ok(value)
        }

        #[inline]
        pub fn element_set_scroll_top(element_handle: u64, value: i32) -> Result<bool> {
            let value = crate::generated::rml_ui::element_set_scroll_top(element_handle, value)?;
            Ok(value)
        }

        #[inline]
        pub fn element_tab_set_remove_tab(element_handle: u64, index: i32) -> Result<bool> {
            let value = crate::generated::rml_ui::element_tab_set_remove_tab(element_handle, index)?;
            Ok(value)
        }

        #[inline]
        pub fn element_tab_set_set_panel(element_handle: u64, index: i32, rml: &str) -> Result<bool> {
            let mut rml_bytes = rml.as_bytes().to_vec();
            if rml_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            rml_bytes.push(0);
            let rml_cstr = core::ffi::CStr::from_bytes_with_nul(&rml_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::element_tab_set_set_panel(element_handle, index, &rml_cstr)
        }

        #[inline]
        pub fn element_tab_set_set_tab(element_handle: u64, index: i32, rml: &str) -> Result<bool> {
            let mut rml_bytes = rml.as_bytes().to_vec();
            if rml_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            rml_bytes.push(0);
            let rml_cstr = core::ffi::CStr::from_bytes_with_nul(&rml_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::element_tab_set_set_tab(element_handle, index, &rml_cstr)
        }

        #[inline]
        pub fn event_get_current(unused: u8) -> Result<EventGetCurrentValue> {
            let value = crate::generated::rml_ui::event_get_current(unused)?;
            Ok(EventGetCurrentValue {
                event_handle: value.0,
                element_handle: value.1,
                document_handle: value.2,
                exists: value.3
            })
        }

        #[inline]
        pub fn event_get_current_element(event_handle: u64) -> Result<EventGetCurrentElementValue> {
            let value = crate::generated::rml_ui::event_get_current_element(event_handle)?;
            Ok(EventGetCurrentElementValue {
                element_handle: value.0,
                exists: value.1
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_event_get_parameter_bool {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "event-get-parameter-bool"]
                pub fn call(p0: i64, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.event-get-parameter-bool."]
        #[inline]
        pub unsafe fn event_get_parameter_bool(p0: i64, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_event_get_parameter_bool::call(p0, p1, p2) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_event_get_parameter_float {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "event-get-parameter-float"]
                pub fn call(p0: i64, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.event-get-parameter-float."]
        #[inline]
        pub unsafe fn event_get_parameter_float(p0: i64, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_event_get_parameter_float::call(p0, p1, p2) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_event_get_parameter_int {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "event-get-parameter-int"]
                pub fn call(p0: i64, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.event-get-parameter-int."]
        #[inline]
        pub unsafe fn event_get_parameter_int(p0: i64, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_event_get_parameter_int::call(p0, p1, p2) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_event_get_parameter_string {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "event-get-parameter-string"]
                pub fn call(p0: i64, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.event-get-parameter-string."]
        #[inline]
        pub unsafe fn event_get_parameter_string(p0: i64, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_event_get_parameter_string::call(p0, p1, p2) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_event_get_parameter_type {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "event-get-parameter-type"]
                pub fn call(p0: i64, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.event-get-parameter-type."]
        #[inline]
        pub unsafe fn event_get_parameter_type(p0: i64, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_event_get_parameter_type::call(p0, p1, p2) }
        }

        #[inline]
        pub fn event_get_phase(event_handle: u64) -> Result<EventGetPhaseValue> {
            let value = crate::generated::rml_ui::event_get_phase(event_handle)?;
            Ok(EventGetPhaseValue {
                value: value.0,
                exists: value.1
            })
        }

        #[inline]
        pub fn event_get_target_element(event_handle: u64) -> Result<EventGetTargetElementValue> {
            let value = crate::generated::rml_ui::event_get_target_element(event_handle)?;
            Ok(EventGetTargetElementValue {
                element_handle: value.0,
                exists: value.1
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_event_get_type {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "event-get-type"]
                pub fn call(p0: i64, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.event-get-type."]
        #[inline]
        pub unsafe fn event_get_type(p0: i64, p1: i32) -> i32 {
            unsafe { __core_owned_event_get_type::call(p0, p1) }
        }

        #[inline]
        pub fn event_is_immediate_propagating(event_handle: u64) -> Result<EventIsImmediatePropagatingValue> {
            let value = crate::generated::rml_ui::event_is_immediate_propagating(event_handle)?;
            Ok(EventIsImmediatePropagatingValue {
                value: value.0,
                exists: value.1
            })
        }

        #[inline]
        pub fn event_is_interruptible(event_handle: u64) -> Result<EventIsInterruptibleValue> {
            let value = crate::generated::rml_ui::event_is_interruptible(event_handle)?;
            Ok(EventIsInterruptibleValue {
                value: value.0,
                exists: value.1
            })
        }

        #[inline]
        pub fn event_is_propagating(event_handle: u64) -> Result<EventIsPropagatingValue> {
            let value = crate::generated::rml_ui::event_is_propagating(event_handle)?;
            Ok(EventIsPropagatingValue {
                value: value.0,
                exists: value.1
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_event_listener_on_attach {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "event-listener-on-attach"]
                pub fn call(p0: i64, p1: i64) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.event-listener-on-attach."]
        #[inline]
        pub unsafe fn event_listener_on_attach(p0: i64, p1: i64) -> i64 {
            unsafe { __core_owned_event_listener_on_attach::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_event_listener_on_detach {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "event-listener-on-detach"]
                pub fn call(p0: i64, p1: i64) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.event-listener-on-detach."]
        #[inline]
        pub unsafe fn event_listener_on_detach(p0: i64, p1: i64) -> i64 {
            unsafe { __core_owned_event_listener_on_detach::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_event_listener_process_event {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "event-listener-process-event"]
                pub fn call(p0: i64, p1: i64) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.event-listener-process-event."]
        #[inline]
        pub unsafe fn event_listener_process_event(p0: i64, p1: i64) -> i64 {
            unsafe { __core_owned_event_listener_process_event::call(p0, p1) }
        }

        #[inline]
        pub fn event_stop_immediate_propagation(event_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::event_stop_immediate_propagation(event_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn event_stop_propagation(event_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::event_stop_propagation(event_handle)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_context {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "get-context"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.get-context."]
        #[inline]
        pub unsafe fn get_context(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_context::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_document_path_requests {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "get-document-path-requests"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.get-document-path-requests."]
        #[inline]
        pub unsafe fn get_document_path_requests(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_document_path_requests::call(p0, p1) }
        }

        #[inline]
        pub fn get_version(unused: u8) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_version::call(unused as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
        pub fn is_ready(unused: u8) -> Result<bool> {
            let value = crate::generated::rml_ui::is_ready(unused)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_load_font_face {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "load-font-face"]
                pub fn call(p0: i32, p1: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.load-font-face."]
        #[inline]
        pub unsafe fn load_font_face(p0: i32, p1: i32) -> i64 {
            unsafe { __core_owned_load_font_face::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_regiser_event_type {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "regiser-event-type"]
                pub fn call(p0: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.regiser-event-type."]
        #[inline]
        pub unsafe fn regiser_event_type(p0: i32) -> i64 {
            unsafe { __core_owned_regiser_event_type::call(p0) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_register_event_type {
            #[link(wasm_import_module = "spring:rml-ui")]
            extern "C" {
                #[link_name = "register-event-type"]
                pub fn call(p0: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.register-event-type."]
        #[inline]
        pub unsafe fn register_event_type(p0: i32) -> i64 {
            unsafe { __core_owned_register_event_type::call(p0) }
        }

        #[inline]
        pub fn remove_context(context_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::remove_context(context_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn remove_context_by_name(name: &str) -> Result<bool> {
            let mut name_bytes = name.as_bytes().to_vec();
            if name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            name_bytes.push(0);
            let name_cstr = core::ffi::CStr::from_bytes_with_nul(&name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::remove_context_by_name(&name_cstr)
        }

        #[inline]
        pub fn set_debug_context(context_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::set_debug_context(context_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn set_debug_context_by_name(name: &str) -> Result<bool> {
            let mut name_bytes = name.as_bytes().to_vec();
            if name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            name_bytes.push(0);
            let name_cstr = core::ffi::CStr::from_bytes_with_nul(&name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::set_debug_context_by_name(&name_cstr)
        }

        #[inline]
        pub fn set_mouse_cursor_alias(rml_name: &str, recoil_name: &str) -> Result<bool> {
            let mut rml_name_bytes = rml_name.as_bytes().to_vec();
            if rml_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            rml_name_bytes.push(0);
            let rml_name_cstr = core::ffi::CStr::from_bytes_with_nul(&rml_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            let mut recoil_name_bytes = recoil_name.as_bytes().to_vec();
            if recoil_name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            recoil_name_bytes.push(0);
            let recoil_name_cstr = core::ffi::CStr::from_bytes_with_nul(&recoil_name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::set_mouse_cursor_alias(&rml_name_cstr, &recoil_name_cstr)
        }

        #[inline]
        pub fn sol_lua_data_model_set_dirty(data_model_handle: u64, property: &str) -> Result<bool> {
            let mut property_bytes = property.as_bytes().to_vec();
            if property_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            property_bytes.push(0);
            let property_cstr = core::ffi::CStr::from_bytes_with_nul(&property_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::rml_ui::sol_lua_data_model_set_dirty(data_model_handle, &property_cstr)
        }

        #[inline]
        pub fn vector2f_new(x: f32, y: f32) -> Result<Vector2fNewValue> {
            let value = crate::generated::rml_ui::vector2f_new(x, y)?;
            Ok(Vector2fNewValue {
                x: value.0,
                y: value.1
            })
        }

        #[inline]
        pub fn vector2i_new(x: i32, y: i32) -> Result<Vector2iNewValue> {
            let value = crate::generated::rml_ui::vector2i_new(x, y)?;
            Ok(Vector2iNewValue {
                x: value.0,
                y: value.1
            })
        }

    }

