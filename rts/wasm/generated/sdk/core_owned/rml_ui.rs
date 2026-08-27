    pub mod rml_ui {
        use super::{Result, String, Vec};

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
        pub struct RmlAddTranslationStringQuery {
            pub key: String,
            pub translation: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlAddTranslationStringResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlClearDocumentPathRequestsQuery {
            pub document_path: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlClearDocumentPathRequestsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlClearTranslationsQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlClearTranslationsResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlContextBoolQuery {
            pub context_handle: u64,
            pub value: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlContextCreateDocumentResult {
            pub document_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlContextGetDimensionsResult {
            pub x: i32,
            pub y: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextGetDocumentQuery {
            pub context_handle: u64,
            pub name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlContextGetDocumentResult {
            pub document_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlContextGetElementAtPointQuery {
            pub context_handle: u64,
            pub x: f32,
            pub y: f32,
            pub ignore_element_handle: u64,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlContextGetElementAtPointResult {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlContextGetElementResult {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlContextGetFloatResult {
            pub value: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlContextGetNameResult {
            pub name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlContextHandleQuery {
            pub context_handle: u64,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlContextLoadDocumentResult {
            pub document_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlContextMouseButtonQuery {
            pub context_handle: u64,
            pub button: i32,
            pub key_modifier_state: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlContextMouseMoveQuery {
            pub context_handle: u64,
            pub x: f32,
            pub y: f32,
            pub key_modifier_state: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlContextOpenDataModelResult {
            pub data_model_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlContextPointerCaptureQuery {
            pub context_handle: u64,
            pub anchor_x: i32,
            pub anchor_y: i32,
            pub active: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlContextPointerDeltaResult {
            pub delta_x: i32,
            pub delta_y: i32,
            pub status: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlContextSetDimensionsQuery {
            pub context_handle: u64,
            pub x: i32,
            pub y: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlDataModelEventHandleQuery {
            pub event_handle: u64,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlDataModelGetBoolResult {
            pub value: bool,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlDataModelGetColorResult {
            pub red: u8,
            pub green: u8,
            pub blue: u8,
            pub alpha: u8,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlDataModelGetFloatResult {
            pub value: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlDataModelGetIntResult {
            pub value: i32,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlDataModelGetPercentResult {
            pub value: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlDataModelGetPixelsResult {
            pub value: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelGetStringResult {
            pub value: String,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlDataModelRowsResult {
            pub rows_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDataModelSetRowsQuery {
            pub rows_handle: u64,
            pub values: Vec<RmlDataValue>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlDataModelVariableBoolQuery {
            pub variable_handle: u64,
            pub value: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlDataModelVariableColorQuery {
            pub variable_handle: u64,
            pub red: u8,
            pub green: u8,
            pub blue: u8,
            pub alpha: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlDataModelVariableFloatQuery {
            pub variable_handle: u64,
            pub value: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlDataModelVariableHandleQuery {
            pub variable_handle: u64,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlDataModelVariableIntQuery {
            pub variable_handle: u64,
            pub value: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlDataModelVariablePercentQuery {
            pub variable_handle: u64,
            pub value: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlDocumentBoolResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDocumentCreateElementQuery {
            pub document_handle: u64,
            pub tag_name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlDocumentCreateElementResult {
            pub element_ptr_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlDocumentGetContextResult {
            pub context_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlDocumentGetStringResult {
            pub value: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlElementAppendChildQuery {
            pub element_handle: u64,
            pub element_ptr_handle: u64,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlElementBoolResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlElementFormControlSelectAddQuery {
            pub element_handle: u64,
            pub element_ptr_handle: u64,
            pub before: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlElementFormControlSelectRemoveQuery {
            pub element_handle: u64,
            pub index: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlElementGetChildQuery {
            pub element_handle: u64,
            pub index: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlElementGetElementResult {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlElementGetFloatResult {
            pub value: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlElementGetIntResult {
            pub value: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlElementHandleQuery {
            pub element_handle: u64,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlElementInsertBeforeQuery {
            pub element_handle: u64,
            pub element_ptr_handle: u64,
            pub adjacent_element_handle: u64,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlElementPointQuery {
            pub element_handle: u64,
            pub x: f32,
            pub y: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlElementProcessDefaultActionQuery {
            pub element_handle: u64,
            pub event_handle: u64,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlElementReplaceChildQuery {
            pub element_handle: u64,
            pub element_ptr_handle: u64,
            pub replaced_element_handle: u64,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlElementSetFloatQuery {
            pub element_handle: u64,
            pub value: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlEventCurrentQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlEventCurrentResult {
            pub event_handle: u64,
            pub element_handle: u64,
            pub document_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlEventGetBoolResult {
            pub value: bool,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlEventGetFloatResult {
            pub value: f32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlEventGetIntResult {
            pub value: i32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlEventGetStringResult {
            pub value: String,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlEventListenerCallbackResult {
            pub event_listener_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlEventListenerElementQuery {
            pub event_listener_handle: u64,
            pub element_handle: u64,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlEventListenerEventQuery {
            pub event_listener_handle: u64,
            pub event_handle: u64,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlGetVersionQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlGetVersionResult {
            pub version: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlIsReadyQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlIsReadyResult {
            pub ready: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlLoadFontFaceQuery {
            pub file_path: String,
            pub fallback: bool,
            pub weight: Option<i32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlRegisterEventTypeResult {
            pub event_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlRemoveContextByNameQuery {
            pub name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlRemoveContextByNameResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlRemoveContextQuery {
            pub context_handle: u64,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlRemoveContextResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlSetDebugContextByNameQuery {
            pub name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlSetDebugContextByNameResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlSetDebugContextQuery {
            pub context_handle: u64,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlSetDebugContextResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlSetMouseCursorAliasQuery {
            pub rml_name: String,
            pub recoil_name: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlSetMouseCursorAliasResult {
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RmlSolLuaDataModelSetDirtyQuery {
            pub data_model_handle: u64,
            pub property: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlVector2fNewQuery {
            pub x: f32,
            pub y: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlVector2fNewResult {
            pub x: f32,
            pub y: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlVector2iNewQuery {
            pub x: i32,
            pub y: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RmlVector2iNewResult {
            pub x: i32,
            pub y: i32,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_context_get_name {
            #[link(wasm_import_module = "spring:rml-ui")]
            unsafe extern "C" {
                #[link_name = "context-get-name"]
                pub safe fn call(pcontext_handle: i64, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_document_get_title {
            #[link(wasm_import_module = "spring:rml-ui")]
            unsafe extern "C" {
                #[link_name = "document-get-title"]
                pub safe fn call(pdocument_handle: i64, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_document_get_url {
            #[link(wasm_import_module = "spring:rml-ui")]
            unsafe extern "C" {
                #[link_name = "document-get-url"]
                pub safe fn call(pdocument_handle: i64, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_element_get_class_name {
            #[link(wasm_import_module = "spring:rml-ui")]
            unsafe extern "C" {
                #[link_name = "element-get-class-name"]
                pub safe fn call(pelement_handle: i64, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_element_get_id {
            #[link(wasm_import_module = "spring:rml-ui")]
            unsafe extern "C" {
                #[link_name = "element-get-id"]
                pub safe fn call(pelement_handle: i64, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_element_get_inner_rml {
            #[link(wasm_import_module = "spring:rml-ui")]
            unsafe extern "C" {
                #[link_name = "element-get-inner-rml"]
                pub safe fn call(pelement_handle: i64, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_element_get_tag_name {
            #[link(wasm_import_module = "spring:rml-ui")]
            unsafe extern "C" {
                #[link_name = "element-get-tag-name"]
                pub safe fn call(pelement_handle: i64, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_element_get_value {
            #[link(wasm_import_module = "spring:rml-ui")]
            unsafe extern "C" {
                #[link_name = "element-get-value"]
                pub safe fn call(pelement_handle: i64, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_version {
            #[link(wasm_import_module = "spring:rml-ui")]
            unsafe extern "C" {
                #[link_name = "get-version"]
                pub safe fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ContextAddEventListenerValue {
            pub event_listener_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ContextCreateDataModelValue {
            pub data_model_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ContextCreateDocumentValue {
            pub document_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ContextGetDimensionsValue {
            pub x: i32,
            pub y: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ContextGetDocumentValue {
            pub document_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ContextGetElementAtPointValue {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ContextGetFocusElementValue {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ContextGetHoverElementValue {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ContextGetRootElementValue {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ContextLoadDocumentValue {
            pub document_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ContextOpenDataModelValue {
            pub data_model_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ContextTakePointerCaptureDeltaValue {
            pub delta_x: i32,
            pub delta_y: i32,
            pub status: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct CreateContextValue {
            pub context_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DataModelBindBoolValue {
            pub variable_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DataModelBindColorValue {
            pub variable_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DataModelBindEventValue {
            pub event_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DataModelBindFloatValue {
            pub variable_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DataModelBindIntValue {
            pub variable_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DataModelBindPercentValue {
            pub variable_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DataModelBindPixelsValue {
            pub variable_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DataModelBindRowsValue {
            pub rows_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DataModelBindStringValue {
            pub variable_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DataModelGetBoolValue {
            pub value: bool,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DataModelGetColorValue {
            pub red: u8,
            pub green: u8,
            pub blue: u8,
            pub alpha: u8,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DataModelGetFloatValue {
            pub value: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DataModelGetIntValue {
            pub value: i32,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DataModelGetPercentValue {
            pub value: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DataModelGetPixelsValue {
            pub value: f32,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct DataModelGetStringValue {
            pub value: String,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DocumentCreateElementValue {
            pub element_ptr_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DocumentCreateTextNodeValue {
            pub element_ptr_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct DocumentGetContextValue {
            pub context_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ElementAddEventListenerValue {
            pub event_listener_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ElementAppendChildValue {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ElementCloneValue {
            pub element_ptr_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ElementGetChildValue {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ElementGetElementByIdValue {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ElementGetRectValue {
            pub left: f32,
            pub top: f32,
            pub width: f32,
            pub height: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ElementInsertBeforeValue {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ElementQuerySelectorValue {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ElementRemoveChildValue {
            pub element_ptr_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct ElementReplaceChildValue {
            pub element_ptr_handle: u64,
            pub success: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct EventGetCurrentValue {
            pub event_handle: u64,
            pub element_handle: u64,
            pub document_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct EventGetCurrentElementValue {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct EventGetParameterBoolValue {
            pub value: bool,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct EventGetParameterFloatValue {
            pub value: f32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct EventGetParameterIntValue {
            pub value: i32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EventGetParameterStringValue {
            pub value: String,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct EventGetParameterTypeValue {
            pub value: i32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct EventGetPhaseValue {
            pub value: i32,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct EventGetTargetElementValue {
            pub element_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct EventGetTypeValue {
            pub value: String,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct EventIsImmediatePropagatingValue {
            pub value: bool,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct EventIsInterruptibleValue {
            pub value: bool,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct EventIsPropagatingValue {
            pub value: bool,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetContextValue {
            pub context_handle: u64,
            pub exists: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct Vector2fNewValue {
            pub x: f32,
            pub y: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct Vector2iNewValue {
            pub x: i32,
            pub y: i32,
        }

        #[inline]
        pub fn add_translation_string(key: &str, translation: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(key, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(key)?),
            };
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(translation, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(translation)?),
            };
            crate::generated::borrowed::rml_ui::add_translation_string(__core_string_0_buf.as_cstr(), __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn clear_document_path_requests(document_path: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(document_path, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(document_path)?),
            };
            crate::generated::borrowed::rml_ui::clear_document_path_requests(__core_string_0_buf.as_cstr())
        }

        #[inline]
        pub fn clear_translations(unused: u8) -> Result<bool> {
            let value = crate::generated::rml_ui::clear_translations(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn context_activate_theme(context_handle: u64, name: &str, value: bool) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(name, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(name)?),
            };
            crate::generated::borrowed::rml_ui::context_activate_theme(context_handle, __core_string_1_buf.as_cstr(), value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_context_add_event_listener {
            #[link(wasm_import_module = "spring:rml-ui")]
            unsafe extern "C" {
                #[link_name = "context-add-event-listener"]
                pub safe fn call(p0: i64, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.context-add-event-listener."]
        #[doc(hidden)]
        #[inline]
        pub fn context_add_event_listener(p0: i64, p1: i32, p2: i32, p3: i32) -> i32 {
            __core_owned_context_add_event_listener::call(p0, p1, p2, p3)
        }

        #[inline]
        pub fn context_create_data_model(context_handle: u64, name: &str) -> Result<ContextCreateDataModelValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + name.len()); __b.extend_from_slice(&(name.len() as u32).to_le_bytes()); __b.extend_from_slice(name.as_bytes()); __b };
            let mut __output = [0u8; 16];
            crate::generated::dynamic_input::rml_ui::context_create_data_model(context_handle as i64, &__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(ContextCreateDataModelValue {
                data_model_handle: crate::generated::__core_wire::u64(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                success: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[inline]
        pub fn context_create_document(context_handle: u64, tag: &str) -> Result<ContextCreateDocumentValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + tag.len()); __b.extend_from_slice(&(tag.len() as u32).to_le_bytes()); __b.extend_from_slice(tag.as_bytes()); __b };
            let mut __output = [0u8; 16];
            crate::generated::dynamic_input::rml_ui::context_create_document(context_handle as i64, &__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(ContextCreateDocumentValue {
                document_handle: crate::generated::__core_wire::u64(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                success: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
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

        #[inline]
        pub fn context_get_document(context_handle: u64, name: &str) -> Result<ContextGetDocumentValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + name.len()); __b.extend_from_slice(&(name.len() as u32).to_le_bytes()); __b.extend_from_slice(name.as_bytes()); __b };
            let mut __output = [0u8; 16];
            crate::generated::dynamic_input::rml_ui::context_get_document(context_handle as i64, &__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(ContextGetDocumentValue {
                document_handle: crate::generated::__core_wire::u64(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                exists: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
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
                    let descriptor_ptr = crate::wasm_output_ptr(&mut descriptor)?;
                    let (output_ptr, output_capacity) = crate::wasm_mut_slice_parts(&mut output)?;
                    descriptor[0] = output_ptr as u32;
                    descriptor[1] = output_capacity as u32;
                    let status = __core_variable_output_context_get_name::call(context_handle as i64, descriptor_ptr);
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(super::decode_core_string(output));
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, 0);
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
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(name, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(name)?),
            };
            crate::generated::borrowed::rml_ui::context_is_theme_active(context_handle, __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn context_load_document(context_handle: u64, document_path: &str) -> Result<ContextLoadDocumentValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + document_path.len()); __b.extend_from_slice(&(document_path.len() as u32).to_le_bytes()); __b.extend_from_slice(document_path.as_bytes()); __b };
            let mut __output = [0u8; 16];
            crate::generated::dynamic_input::rml_ui::context_load_document(context_handle as i64, &__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(ContextLoadDocumentValue {
                document_handle: crate::generated::__core_wire::u64(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                success: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[inline]
        pub fn context_open_data_model(context_handle: u64, name: &str) -> Result<ContextOpenDataModelValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + name.len()); __b.extend_from_slice(&(name.len() as u32).to_le_bytes()); __b.extend_from_slice(name.as_bytes()); __b };
            let mut __output = [0u8; 16];
            crate::generated::dynamic_input::rml_ui::context_open_data_model(context_handle as i64, &__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(ContextOpenDataModelValue {
                data_model_handle: crate::generated::__core_wire::u64(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                success: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
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
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(text, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(text)?),
            };
            crate::generated::borrowed::rml_ui::context_process_text_input(context_handle, __core_string_1_buf.as_cstr())
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
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(name, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(name)?),
            };
            crate::generated::borrowed::rml_ui::context_remove_data_model(context_handle, __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn context_remove_event_listener(context_handle: u64, event_listener_handle: u64, event: &str, in_capture_phase: bool) -> Result<bool> {
            let mut __core_string_2_scratch = [0u8; 256];
            let __core_string_2_buf = match super::write_cstr(event, &mut __core_string_2_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(event)?),
            };
            crate::generated::borrowed::rml_ui::context_remove_event_listener(context_handle, event_listener_handle, __core_string_2_buf.as_cstr(), in_capture_phase)
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

        #[inline]
        pub fn create_context(name: &str) -> Result<CreateContextValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + name.len()); __b.extend_from_slice(&(name.len() as u32).to_le_bytes()); __b.extend_from_slice(name.as_bytes()); __b };
            let mut __output = [0u8; 16];
            crate::generated::dynamic_input::rml_ui::create_context(&__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(CreateContextValue {
                context_handle: crate::generated::__core_wire::u64(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                success: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[inline]
        pub fn data_model_bind_bool(data_model_handle: u64, name: &str, initial_value: bool) -> Result<DataModelBindBoolValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + name.len()); __b.extend_from_slice(&(name.len() as u32).to_le_bytes()); __b.extend_from_slice(name.as_bytes()); __b };
            let mut __output = [0u8; 16];
            crate::generated::dynamic_input::rml_ui::data_model_bind_bool(data_model_handle as i64, initial_value as i32, &__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(DataModelBindBoolValue {
                variable_handle: crate::generated::__core_wire::u64(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                success: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[inline]
        pub fn data_model_bind_color(data_model_handle: u64, name: &str, red: u8, green: u8, blue: u8, alpha: u8) -> Result<DataModelBindColorValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + name.len()); __b.extend_from_slice(&(name.len() as u32).to_le_bytes()); __b.extend_from_slice(name.as_bytes()); __b };
            let mut __output = [0u8; 16];
            crate::generated::dynamic_input::rml_ui::data_model_bind_color(data_model_handle as i64, red as i32, green as i32, blue as i32, alpha as i32, &__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(DataModelBindColorValue {
                variable_handle: crate::generated::__core_wire::u64(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                success: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_data_model_bind_event {
            #[link(wasm_import_module = "spring:rml-ui")]
            unsafe extern "C" {
                #[link_name = "data-model-bind-event"]
                pub safe fn call(p0: i64, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.data-model-bind-event."]
        #[doc(hidden)]
        #[inline]
        pub fn data_model_bind_event(p0: i64, p1: i32, p2: i32) -> i32 {
            __core_owned_data_model_bind_event::call(p0, p1, p2)
        }

        #[inline]
        pub fn data_model_bind_float(data_model_handle: u64, name: &str, initial_value: f32) -> Result<DataModelBindFloatValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + name.len()); __b.extend_from_slice(&(name.len() as u32).to_le_bytes()); __b.extend_from_slice(name.as_bytes()); __b };
            let mut __output = [0u8; 16];
            crate::generated::dynamic_input::rml_ui::data_model_bind_float(data_model_handle as i64, initial_value, &__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(DataModelBindFloatValue {
                variable_handle: crate::generated::__core_wire::u64(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                success: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[inline]
        pub fn data_model_bind_int(data_model_handle: u64, name: &str, initial_value: i32) -> Result<DataModelBindIntValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + name.len()); __b.extend_from_slice(&(name.len() as u32).to_le_bytes()); __b.extend_from_slice(name.as_bytes()); __b };
            let mut __output = [0u8; 16];
            crate::generated::dynamic_input::rml_ui::data_model_bind_int(data_model_handle as i64, initial_value, &__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(DataModelBindIntValue {
                variable_handle: crate::generated::__core_wire::u64(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                success: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[inline]
        pub fn data_model_bind_percent(data_model_handle: u64, name: &str, initial_value: f32) -> Result<DataModelBindPercentValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + name.len()); __b.extend_from_slice(&(name.len() as u32).to_le_bytes()); __b.extend_from_slice(name.as_bytes()); __b };
            let mut __output = [0u8; 16];
            crate::generated::dynamic_input::rml_ui::data_model_bind_percent(data_model_handle as i64, initial_value, &__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(DataModelBindPercentValue {
                variable_handle: crate::generated::__core_wire::u64(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                success: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[inline]
        pub fn data_model_bind_pixels(data_model_handle: u64, name: &str, initial_value: f32) -> Result<DataModelBindPixelsValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + name.len()); __b.extend_from_slice(&(name.len() as u32).to_le_bytes()); __b.extend_from_slice(name.as_bytes()); __b };
            let mut __output = [0u8; 16];
            crate::generated::dynamic_input::rml_ui::data_model_bind_pixels(data_model_handle as i64, initial_value, &__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(DataModelBindPixelsValue {
                variable_handle: crate::generated::__core_wire::u64(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                success: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[inline]
        pub fn data_model_bind_rows(data_model_handle: u64, name: &str, fields: &[RmlDataFieldDef]) -> Result<DataModelBindRowsValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + name.len()); __b.extend_from_slice(&(name.len() as u32).to_le_bytes()); __b.extend_from_slice(name.as_bytes()); __b };
            let __blob1 = { let mut __b = Vec::new(); __b.extend_from_slice(&(fields.len() as u32).to_le_bytes()); for __item in fields.iter() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item.name.len() as u32).to_le_bytes()); __b.extend_from_slice(__item.name.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item.type_ as u32).to_le_bytes());} __b };
            let mut __output = [0u8; 16];
            crate::generated::dynamic_input::rml_ui::data_model_bind_rows(data_model_handle as i64, &__blob0, &__blob1, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(DataModelBindRowsValue {
                rows_handle: crate::generated::__core_wire::u64(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                success: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[inline]
        pub fn data_model_bind_string(data_model_handle: u64, name: &str, initial_value: &str) -> Result<DataModelBindStringValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + name.len()); __b.extend_from_slice(&(name.len() as u32).to_le_bytes()); __b.extend_from_slice(name.as_bytes()); __b };
            let __blob1 = { let mut __b = Vec::with_capacity(4 + initial_value.len()); __b.extend_from_slice(&(initial_value.len() as u32).to_le_bytes()); __b.extend_from_slice(initial_value.as_bytes()); __b };
            let mut __output = [0u8; 16];
            crate::generated::dynamic_input::rml_ui::data_model_bind_string(data_model_handle as i64, &__blob0, &__blob1, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(DataModelBindStringValue {
                variable_handle: crate::generated::__core_wire::u64(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                success: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
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
            unsafe extern "C" {
                #[link_name = "data-model-get-string"]
                pub safe fn call(p0: i64, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.data-model-get-string."]
        #[doc(hidden)]
        #[inline]
        pub fn data_model_get_string(p0: i64, p1: i32) -> i32 {
            __core_owned_data_model_get_string::call(p0, p1)
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

        #[inline]
        pub fn data_model_set_rows(rows_handle: u64, values: &[RmlDataValue]) -> Result<bool> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(values.len() as u32).to_le_bytes()); for __item in values.iter() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item.type_ as u32).to_le_bytes()); __b.extend_from_slice(&(if __item.bool_value { 1u32 } else { 0u32 }).to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.int_value.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.float_value.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item.string_value.len() as u32).to_le_bytes()); __b.extend_from_slice(__item.string_value.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item.red as u32).to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item.green as u32).to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item.blue as u32).to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item.alpha as u32).to_le_bytes());} __b };
            crate::generated::dynamic_input::rml_ui::data_model_set_rows(rows_handle as i64, &__blob0)
        }

        #[inline]
        pub fn data_model_set_string(variable_handle: u64, value: &str) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(value, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(value)?),
            };
            crate::generated::borrowed::rml_ui::data_model_set_string(variable_handle, __core_string_1_buf.as_cstr())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_data_model_unbind_event {
            #[link(wasm_import_module = "spring:rml-ui")]
            unsafe extern "C" {
                #[link_name = "data-model-unbind-event"]
                pub safe fn call(p0: i64) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.data-model-unbind-event."]
        #[doc(hidden)]
        #[inline]
        pub fn data_model_unbind_event(p0: i64) -> i64 {
            __core_owned_data_model_unbind_event::call(p0)
        }

        #[inline]
        pub fn document_append_to_style_sheet(document_handle: u64, value: &str) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(value, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(value)?),
            };
            crate::generated::borrowed::rml_ui::document_append_to_style_sheet(document_handle, __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn document_close(document_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::document_close(document_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn document_create_element(document_handle: u64, tag_name: &str) -> Result<DocumentCreateElementValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + tag_name.len()); __b.extend_from_slice(&(tag_name.len() as u32).to_le_bytes()); __b.extend_from_slice(tag_name.as_bytes()); __b };
            let mut __output = [0u8; 16];
            crate::generated::dynamic_input::rml_ui::document_create_element(document_handle as i64, &__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(DocumentCreateElementValue {
                element_ptr_handle: crate::generated::__core_wire::u64(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                success: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[inline]
        pub fn document_create_text_node(document_handle: u64, value: &str) -> Result<DocumentCreateTextNodeValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + value.len()); __b.extend_from_slice(&(value.len() as u32).to_le_bytes()); __b.extend_from_slice(value.as_bytes()); __b };
            let mut __output = [0u8; 16];
            crate::generated::dynamic_input::rml_ui::document_create_text_node(document_handle as i64, &__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(DocumentCreateTextNodeValue {
                element_ptr_handle: crate::generated::__core_wire::u64(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                success: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
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
                    let descriptor_ptr = crate::wasm_output_ptr(&mut descriptor)?;
                    let (output_ptr, output_capacity) = crate::wasm_mut_slice_parts(&mut output)?;
                    descriptor[0] = output_ptr as u32;
                    descriptor[1] = output_capacity as u32;
                    let status = __core_variable_output_document_get_title::call(document_handle as i64, descriptor_ptr);
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(super::decode_core_string(output));
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, 0);
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
                    let descriptor_ptr = crate::wasm_output_ptr(&mut descriptor)?;
                    let (output_ptr, output_capacity) = crate::wasm_mut_slice_parts(&mut output)?;
                    descriptor[0] = output_ptr as u32;
                    descriptor[1] = output_capacity as u32;
                    let status = __core_variable_output_document_get_url::call(document_handle as i64, descriptor_ptr);
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(super::decode_core_string(output));
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, 0);
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
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(value, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(value)?),
            };
            crate::generated::borrowed::rml_ui::document_load_external_script(document_handle, __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn document_load_inline_script(document_handle: u64, content: &str, source_path: &str, source_line: i32) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(content, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(content)?),
            };
            let mut __core_string_2_scratch = [0u8; 256];
            let __core_string_2_buf = match super::write_cstr(source_path, &mut __core_string_2_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(source_path)?),
            };
            crate::generated::borrowed::rml_ui::document_load_inline_script(document_handle, __core_string_1_buf.as_cstr(), __core_string_2_buf.as_cstr(), source_line)
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
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(title, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(title)?),
            };
            crate::generated::borrowed::rml_ui::document_set_title(document_handle, __core_string_1_buf.as_cstr())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_document_show {
            #[link(wasm_import_module = "spring:rml-ui")]
            unsafe extern "C" {
                #[link_name = "document-show"]
                pub safe fn call(p0: i64, p1: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.document-show."]
        #[doc(hidden)]
        #[inline]
        pub fn document_show(p0: i64, p1: i32) -> i64 {
            __core_owned_document_show::call(p0, p1)
        }

        #[inline]
        pub fn document_update_document(document_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::document_update_document(document_handle)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_element_add_event_listener {
            #[link(wasm_import_module = "spring:rml-ui")]
            unsafe extern "C" {
                #[link_name = "element-add-event-listener"]
                pub safe fn call(p0: i64, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.element-add-event-listener."]
        #[doc(hidden)]
        #[inline]
        pub fn element_add_event_listener(p0: i64, p1: i32, p2: i32, p3: i32) -> i32 {
            __core_owned_element_add_event_listener::call(p0, p1, p2, p3)
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
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(value, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(value)?),
            };
            crate::generated::borrowed::rml_ui::element_are_pseudo_classes_set(element_handle, __core_string_1_buf.as_cstr())
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

        #[inline]
        pub fn element_closest(element_handle: u64, value: &str) -> Result<ElementClosestValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + value.len()); __b.extend_from_slice(&(value.len() as u32).to_le_bytes()); __b.extend_from_slice(value.as_bytes()); __b };
            let mut __output = [0u8; 16];
            crate::generated::dynamic_input::rml_ui::element_closest(element_handle as i64, &__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(ElementClosestValue {
                element_handle: crate::generated::__core_wire::u64(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                exists: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[inline]
        pub fn element_dispatch_event(element_handle: u64, event: &str) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(event, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(event)?),
            };
            crate::generated::borrowed::rml_ui::element_dispatch_event(element_handle, __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn element_focus(element_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::element_focus(element_handle)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_element_form_control_input_get_selection {
            #[link(wasm_import_module = "spring:rml-ui")]
            unsafe extern "C" {
                #[link_name = "element-form-control-input-get-selection"]
                pub safe fn call(p0: i64, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.element-form-control-input-get-selection."]
        #[doc(hidden)]
        #[inline]
        pub fn element_form_control_input_get_selection(p0: i64, p1: i32) -> i32 {
            __core_owned_element_form_control_input_get_selection::call(p0, p1)
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
            unsafe extern "C" {
                #[link_name = "element-form-control-text-area-get-selection"]
                pub safe fn call(p0: i64, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.element-form-control-text-area-get-selection."]
        #[doc(hidden)]
        #[inline]
        pub fn element_form_control_text_area_get_selection(p0: i64, p1: i32) -> i32 {
            __core_owned_element_form_control_text_area_get_selection::call(p0, p1)
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
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(name, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(name)?),
            };
            let mut __core_string_2_scratch = [0u8; 256];
            let __core_string_2_buf = match super::write_cstr(value, &mut __core_string_2_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(value)?),
            };
            crate::generated::borrowed::rml_ui::element_form_submit(element_handle, __core_string_1_buf.as_cstr(), __core_string_2_buf.as_cstr())
        }

        #[inline]
        pub fn element_get_active_pseudo_classes(element_handle: u64) -> Result<Vec<String>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::rml_ui::element_get_active_pseudo_classes(element_handle as i64, &mut __output) {
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

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_element_get_attribute {
            #[link(wasm_import_module = "spring:rml-ui")]
            unsafe extern "C" {
                #[link_name = "element-get-attribute"]
                pub safe fn call(p0: i64, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.element-get-attribute."]
        #[doc(hidden)]
        #[inline]
        pub fn element_get_attribute(p0: i64, p1: i32, p2: i32) -> i32 {
            __core_owned_element_get_attribute::call(p0, p1, p2)
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
                    let descriptor_ptr = crate::wasm_output_ptr(&mut descriptor)?;
                    let (output_ptr, output_capacity) = crate::wasm_mut_slice_parts(&mut output)?;
                    descriptor[0] = output_ptr as u32;
                    descriptor[1] = output_capacity as u32;
                    let status = __core_variable_output_element_get_class_name::call(element_handle as i64, descriptor_ptr);
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(super::decode_core_string(output));
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, 0);
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
        pub fn element_get_element_by_id(element_handle: u64, value: &str) -> Result<ElementGetElementByIdValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + value.len()); __b.extend_from_slice(&(value.len() as u32).to_le_bytes()); __b.extend_from_slice(value.as_bytes()); __b };
            let mut __output = [0u8; 16];
            crate::generated::dynamic_input::rml_ui::element_get_element_by_id(element_handle as i64, &__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(ElementGetElementByIdValue {
                element_handle: crate::generated::__core_wire::u64(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                exists: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[inline]
        pub fn element_get_elements_by_class_name(element_handle: u64, value: &str) -> Result<Vec<u64>> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + value.len()); __b.extend_from_slice(&(value.len() as u32).to_le_bytes()); __b.extend_from_slice(value.as_bytes()); __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::rml_ui::element_get_elements_by_class_name(element_handle as i64, &__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 8);
                        let mut __result = Vec::<u64>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(crate::generated::__core_wire::u64(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?);
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 8, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn element_get_elements_by_class_name_count(element_handle: u64, value: &str) -> Result<i32> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(value, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(value)?),
            };
            crate::generated::borrowed::rml_ui::element_get_elements_by_class_name_count(element_handle, __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn element_get_elements_by_tag_name(element_handle: u64, value: &str) -> Result<Vec<u64>> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + value.len()); __b.extend_from_slice(&(value.len() as u32).to_le_bytes()); __b.extend_from_slice(value.as_bytes()); __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::rml_ui::element_get_elements_by_tag_name(element_handle as i64, &__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 8);
                        let mut __result = Vec::<u64>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(crate::generated::__core_wire::u64(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?);
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 8, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn element_get_elements_by_tag_name_count(element_handle: u64, value: &str) -> Result<i32> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(value, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(value)?),
            };
            crate::generated::borrowed::rml_ui::element_get_elements_by_tag_name_count(element_handle, __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn element_get_id(element_handle: u64) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let descriptor_ptr = crate::wasm_output_ptr(&mut descriptor)?;
                    let (output_ptr, output_capacity) = crate::wasm_mut_slice_parts(&mut output)?;
                    descriptor[0] = output_ptr as u32;
                    descriptor[1] = output_capacity as u32;
                    let status = __core_variable_output_element_get_id::call(element_handle as i64, descriptor_ptr);
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(super::decode_core_string(output));
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, 0);
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
                    let descriptor_ptr = crate::wasm_output_ptr(&mut descriptor)?;
                    let (output_ptr, output_capacity) = crate::wasm_mut_slice_parts(&mut output)?;
                    descriptor[0] = output_ptr as u32;
                    descriptor[1] = output_capacity as u32;
                    let status = __core_variable_output_element_get_inner_rml::call(element_handle as i64, descriptor_ptr);
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(super::decode_core_string(output));
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, 0);
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
                    let descriptor_ptr = crate::wasm_output_ptr(&mut descriptor)?;
                    let (output_ptr, output_capacity) = crate::wasm_mut_slice_parts(&mut output)?;
                    descriptor[0] = output_ptr as u32;
                    descriptor[1] = output_capacity as u32;
                    let status = __core_variable_output_element_get_tag_name::call(element_handle as i64, descriptor_ptr);
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(super::decode_core_string(output));
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, 0);
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
                    let descriptor_ptr = crate::wasm_output_ptr(&mut descriptor)?;
                    let (output_ptr, output_capacity) = crate::wasm_mut_slice_parts(&mut output)?;
                    descriptor[0] = output_ptr as u32;
                    descriptor[1] = output_capacity as u32;
                    let status = __core_variable_output_element_get_value::call(element_handle as i64, descriptor_ptr);
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(super::decode_core_string(output));
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, 0);
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
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(value, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(value)?),
            };
            crate::generated::borrowed::rml_ui::element_has_attribute(element_handle, __core_string_1_buf.as_cstr())
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
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(value, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(value)?),
            };
            crate::generated::borrowed::rml_ui::element_is_class_set(element_handle, __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn element_is_point_within_element(element_handle: u64, x: f32, y: f32) -> Result<bool> {
            let value = crate::generated::rml_ui::element_is_point_within_element(element_handle, x, y)?;
            Ok(value)
        }

        #[inline]
        pub fn element_is_pseudo_class_set(element_handle: u64, value: &str) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(value, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(value)?),
            };
            crate::generated::borrowed::rml_ui::element_is_pseudo_class_set(element_handle, __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn element_is_visible(element_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::element_is_visible(element_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn element_matches(element_handle: u64, value: &str) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(value, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(value)?),
            };
            crate::generated::borrowed::rml_ui::element_matches(element_handle, __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn element_process_default_action(element_handle: u64, event_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::element_process_default_action(element_handle, event_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn element_query_selector(element_handle: u64, value: &str) -> Result<ElementQuerySelectorValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + value.len()); __b.extend_from_slice(&(value.len() as u32).to_le_bytes()); __b.extend_from_slice(value.as_bytes()); __b };
            let mut __output = [0u8; 16];
            crate::generated::dynamic_input::rml_ui::element_query_selector(element_handle as i64, &__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(ElementQuerySelectorValue {
                element_handle: crate::generated::__core_wire::u64(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                exists: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[inline]
        pub fn element_query_selector_all(element_handle: u64, value: &str) -> Result<Vec<u64>> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + value.len()); __b.extend_from_slice(&(value.len() as u32).to_le_bytes()); __b.extend_from_slice(value.as_bytes()); __b };
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_input::rml_ui::element_query_selector_all(element_handle as i64, &__blob0, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required * 8);
                        let mut __result = Vec::<u64>::with_capacity(required);
                        let mut __cursor = 0usize;
                        for _ in 0..required {
                            __result.push(crate::generated::__core_wire::u64(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?);
                        }
                        return Ok(__result);
                    }
                    Err(error) if error.error.code == crate::ErrorCode::BufferOverflow as i32 => {
                        __output.resize(error.required * 8, 0);
                    }
                    Err(error) => return Err(error.error),
                }
            }
        }

        #[inline]
        pub fn element_query_selector_all_count(element_handle: u64, value: &str) -> Result<i32> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(value, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(value)?),
            };
            crate::generated::borrowed::rml_ui::element_query_selector_all_count(element_handle, __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn element_remove_attribute(element_handle: u64, value: &str) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(value, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(value)?),
            };
            crate::generated::borrowed::rml_ui::element_remove_attribute(element_handle, __core_string_1_buf.as_cstr())
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
            let mut __core_string_2_scratch = [0u8; 256];
            let __core_string_2_buf = match super::write_cstr(event, &mut __core_string_2_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(event)?),
            };
            crate::generated::borrowed::rml_ui::element_remove_event_listener(element_handle, event_listener_handle, __core_string_2_buf.as_cstr(), in_capture_phase)
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
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(name, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(name)?),
            };
            let mut __core_string_2_scratch = [0u8; 256];
            let __core_string_2_buf = match super::write_cstr(value, &mut __core_string_2_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(value)?),
            };
            crate::generated::borrowed::rml_ui::element_set_attribute(element_handle, __core_string_1_buf.as_cstr(), __core_string_2_buf.as_cstr())
        }

        #[inline]
        pub fn element_set_class(element_handle: u64, name: &str, value: bool) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(name, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(name)?),
            };
            crate::generated::borrowed::rml_ui::element_set_class(element_handle, __core_string_1_buf.as_cstr(), value)
        }

        #[inline]
        pub fn element_set_class_name(element_handle: u64, value: &str) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(value, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(value)?),
            };
            crate::generated::borrowed::rml_ui::element_set_class_name(element_handle, __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn element_set_id(element_handle: u64, value: &str) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(value, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(value)?),
            };
            crate::generated::borrowed::rml_ui::element_set_id(element_handle, __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn element_set_inner_rml(element_handle: u64, value: &str) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(value, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(value)?),
            };
            crate::generated::borrowed::rml_ui::element_set_inner_rml(element_handle, __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn element_set_pseudo_class(element_handle: u64, name: &str, value: bool) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(name, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(name)?),
            };
            crate::generated::borrowed::rml_ui::element_set_pseudo_class(element_handle, __core_string_1_buf.as_cstr(), value)
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
            let mut __core_string_2_scratch = [0u8; 256];
            let __core_string_2_buf = match super::write_cstr(rml, &mut __core_string_2_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(rml)?),
            };
            crate::generated::borrowed::rml_ui::element_tab_set_set_panel(element_handle, index, __core_string_2_buf.as_cstr())
        }

        #[inline]
        pub fn element_tab_set_set_tab(element_handle: u64, index: i32, rml: &str) -> Result<bool> {
            let mut __core_string_2_scratch = [0u8; 256];
            let __core_string_2_buf = match super::write_cstr(rml, &mut __core_string_2_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(rml)?),
            };
            crate::generated::borrowed::rml_ui::element_tab_set_set_tab(element_handle, index, __core_string_2_buf.as_cstr())
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

        #[inline]
        pub fn event_get_parameter_bool(event_handle: u64, name: &str) -> Result<EventGetParameterBoolValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + name.len()); __b.extend_from_slice(&(name.len() as u32).to_le_bytes()); __b.extend_from_slice(name.as_bytes()); __b };
            let mut __output = [0u8; 8];
            crate::generated::dynamic_input::rml_ui::event_get_parameter_bool(event_handle as i64, &__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(EventGetParameterBoolValue {
                value: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                exists: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[inline]
        pub fn event_get_parameter_float(event_handle: u64, name: &str) -> Result<EventGetParameterFloatValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + name.len()); __b.extend_from_slice(&(name.len() as u32).to_le_bytes()); __b.extend_from_slice(name.as_bytes()); __b };
            let mut __output = [0u8; 8];
            crate::generated::dynamic_input::rml_ui::event_get_parameter_float(event_handle as i64, &__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(EventGetParameterFloatValue {
                value: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                exists: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[inline]
        pub fn event_get_parameter_int(event_handle: u64, name: &str) -> Result<EventGetParameterIntValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + name.len()); __b.extend_from_slice(&(name.len() as u32).to_le_bytes()); __b.extend_from_slice(name.as_bytes()); __b };
            let mut __output = [0u8; 8];
            crate::generated::dynamic_input::rml_ui::event_get_parameter_int(event_handle as i64, &__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(EventGetParameterIntValue {
                value: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                exists: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_event_get_parameter_string {
            #[link(wasm_import_module = "spring:rml-ui")]
            unsafe extern "C" {
                #[link_name = "event-get-parameter-string"]
                pub safe fn call(p0: i64, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.event-get-parameter-string."]
        #[doc(hidden)]
        #[inline]
        pub fn event_get_parameter_string(p0: i64, p1: i32, p2: i32) -> i32 {
            __core_owned_event_get_parameter_string::call(p0, p1, p2)
        }

        #[inline]
        pub fn event_get_parameter_type(event_handle: u64, name: &str) -> Result<EventGetParameterTypeValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + name.len()); __b.extend_from_slice(&(name.len() as u32).to_le_bytes()); __b.extend_from_slice(name.as_bytes()); __b };
            let mut __output = [0u8; 8];
            crate::generated::dynamic_input::rml_ui::event_get_parameter_type(event_handle as i64, &__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(EventGetParameterTypeValue {
                value: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                exists: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
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
            unsafe extern "C" {
                #[link_name = "event-get-type"]
                pub safe fn call(p0: i64, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.event-get-type."]
        #[doc(hidden)]
        #[inline]
        pub fn event_get_type(p0: i64, p1: i32) -> i32 {
            __core_owned_event_get_type::call(p0, p1)
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
            unsafe extern "C" {
                #[link_name = "event-listener-on-attach"]
                pub safe fn call(p0: i64, p1: i64) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.event-listener-on-attach."]
        #[doc(hidden)]
        #[inline]
        pub fn event_listener_on_attach(p0: i64, p1: i64) -> i64 {
            __core_owned_event_listener_on_attach::call(p0, p1)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_event_listener_on_detach {
            #[link(wasm_import_module = "spring:rml-ui")]
            unsafe extern "C" {
                #[link_name = "event-listener-on-detach"]
                pub safe fn call(p0: i64, p1: i64) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.event-listener-on-detach."]
        #[doc(hidden)]
        #[inline]
        pub fn event_listener_on_detach(p0: i64, p1: i64) -> i64 {
            __core_owned_event_listener_on_detach::call(p0, p1)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_event_listener_process_event {
            #[link(wasm_import_module = "spring:rml-ui")]
            unsafe extern "C" {
                #[link_name = "event-listener-process-event"]
                pub safe fn call(p0: i64, p1: i64) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.event-listener-process-event."]
        #[doc(hidden)]
        #[inline]
        pub fn event_listener_process_event(p0: i64, p1: i64) -> i64 {
            __core_owned_event_listener_process_event::call(p0, p1)
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

        #[inline]
        pub fn get_context(name: &str) -> Result<GetContextValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + name.len()); __b.extend_from_slice(&(name.len() as u32).to_le_bytes()); __b.extend_from_slice(name.as_bytes()); __b };
            let mut __output = [0u8; 16];
            crate::generated::dynamic_input::rml_ui::get_context(&__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(GetContextValue {
                context_handle: crate::generated::__core_wire::u64(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                exists: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_document_path_requests {
            #[link(wasm_import_module = "spring:rml-ui")]
            unsafe extern "C" {
                #[link_name = "get-document-path-requests"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.get-document-path-requests."]
        #[doc(hidden)]
        #[inline]
        pub fn get_document_path_requests(p0: i32, p1: i32) -> i32 {
            __core_owned_get_document_path_requests::call(p0, p1)
        }

        #[inline]
        pub fn get_version(unused: u8) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let descriptor_ptr = crate::wasm_output_ptr(&mut descriptor)?;
                    let (output_ptr, output_capacity) = crate::wasm_mut_slice_parts(&mut output)?;
                    descriptor[0] = output_ptr as u32;
                    descriptor[1] = output_capacity as u32;
                    let status = __core_variable_output_get_version::call(unused as i32, descriptor_ptr);
                    let required = descriptor[2] as usize;
                    if status == 0 {
                        output.truncate(required);
                        return Ok(super::decode_core_string(output));
                    }
                    if status != crate::ErrorCode::BufferOverflow as i32 {
                        return Err(crate::ApiError::new(status));
                    }
                    output.resize(required, 0);
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
            unsafe extern "C" {
                #[link_name = "load-font-face"]
                pub safe fn call(p0: i32, p1: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.load-font-face."]
        #[doc(hidden)]
        #[inline]
        pub fn load_font_face(p0: i32, p1: i32) -> i64 {
            __core_owned_load_font_face::call(p0, p1)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_regiser_event_type {
            #[link(wasm_import_module = "spring:rml-ui")]
            unsafe extern "C" {
                #[link_name = "regiser-event-type"]
                pub safe fn call(p0: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.regiser-event-type."]
        #[doc(hidden)]
        #[inline]
        pub fn regiser_event_type(p0: i32) -> i64 {
            __core_owned_regiser_event_type::call(p0)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_register_event_type {
            #[link(wasm_import_module = "spring:rml-ui")]
            unsafe extern "C" {
                #[link_name = "register-event-type"]
                pub safe fn call(p0: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:rml-ui.register-event-type."]
        #[doc(hidden)]
        #[inline]
        pub fn register_event_type(p0: i32) -> i64 {
            __core_owned_register_event_type::call(p0)
        }

        #[inline]
        pub fn remove_context(context_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::remove_context(context_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn remove_context_by_name(name: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(name, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(name)?),
            };
            crate::generated::borrowed::rml_ui::remove_context_by_name(__core_string_0_buf.as_cstr())
        }

        #[inline]
        pub fn set_debug_context(context_handle: u64) -> Result<bool> {
            let value = crate::generated::rml_ui::set_debug_context(context_handle)?;
            Ok(value)
        }

        #[inline]
        pub fn set_debug_context_by_name(name: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(name, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(name)?),
            };
            crate::generated::borrowed::rml_ui::set_debug_context_by_name(__core_string_0_buf.as_cstr())
        }

        #[inline]
        pub fn set_mouse_cursor_alias(rml_name: &str, recoil_name: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(rml_name, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(rml_name)?),
            };
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(recoil_name, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(recoil_name)?),
            };
            crate::generated::borrowed::rml_ui::set_mouse_cursor_alias(__core_string_0_buf.as_cstr(), __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn sol_lua_data_model_set_dirty(data_model_handle: u64, property: &str) -> Result<bool> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(property, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(property)?),
            };
            crate::generated::borrowed::rml_ui::sol_lua_data_model_set_dirty(data_model_handle, __core_string_1_buf.as_cstr())
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

