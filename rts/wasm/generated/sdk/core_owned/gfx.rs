    pub mod gfx {
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
        pub struct GfxActiveFBOQuery {
            pub fbo_id: u32,
            pub target: u32,
            pub identities: bool,
            pub callback: u32,
            pub user_data: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxActiveShaderQuery {
            pub shader_id: u32,
            pub callback: u32,
            pub user_data: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxActiveTextureQuery {
            pub tex_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxActiveUniformEntry {
            pub name: String,
            pub type_: String,
            pub gl_type: u32,
            pub length: i32,
            pub size: i32,
            pub location: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxActiveUniformsResult {
            pub entries: Vec<GfxActiveUniformEntry>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxAlphaTestQuery {
            pub enable: bool,
            pub func: u32,
            pub ref_: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxAtlasTextureEntry {
            pub name: String,
            pub x1: f32,
            pub x2: f32,
            pub y1: f32,
            pub y2: f32,
            pub page_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxAtlasTextureQuery {
            pub atlas_name: String,
            pub texture_name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxAtlasTextureResult {
            pub x1: f32,
            pub x2: f32,
            pub y1: f32,
            pub y2: f32,
            pub page_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxAtlasTexturesResult {
            pub entries: Vec<GfxAtlasTextureEntry>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxBeginEndQuery {
            pub primitive: u32,
            pub callback: u32,
            pub user_data: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxBindImageTextureQuery {
            pub unit: u32,
            pub name: String,
            pub level: i32,
            pub layer: i32,
            pub layered: bool,
            pub access: u32,
            pub format: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxBlendEquationQuery {
            pub mode: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxBlendEquationSeparateQuery {
            pub mode_rgb: u32,
            pub mode_alpha: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxBlendFuncQuery {
            pub src: u32,
            pub dst: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxBlendFuncSeparateQuery {
            pub src_rgb: u32,
            pub dst_rgb: u32,
            pub src_alpha: u32,
            pub dst_alpha: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxBlitFBOQuery {
            pub src_fboid: u32,
            pub dst_fboid: u32,
            pub x0_src: i32,
            pub y0_src: i32,
            pub x1_src: i32,
            pub y1_src: i32,
            pub x0_dst: i32,
            pub y0_dst: i32,
            pub x1_dst: i32,
            pub y1_dst: i32,
            pub mask: u32,
            pub filter: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxBoolQuery {
            pub value: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxBoolResult {
            pub value: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxCallbackQuery {
            pub callback: u32,
            pub user_data: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxChangeTextureParamsQuery {
            pub name: String,
            pub params: GfxTextureParams,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxClearAttachmentFBOQuery {
            pub target: u32,
            pub attachment: u32,
            pub values: Vec<f32>,
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxClearQuery {
            pub bits: u32,
            pub values: Vec<f32>,
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxClipDistanceQuery {
            pub index: u32,
            pub enable: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxClipPlaneQuery {
            pub plane: u32,
            pub equation: Vec<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxColorMaskOptions {
            pub red: bool,
            pub green: bool,
            pub blue: bool,
            pub alpha: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxColorMaskQuery {
            pub options: GfxColorMaskOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxColorQuery {
            pub r: f32,
            pub g: f32,
            pub b: f32,
            pub a: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxConsoleCommandEntry {
            pub command: String,
            pub description: String,
            pub synced: bool,
            pub cheat: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxConsoleCommandsResult {
            pub entries: Vec<GfxConsoleCommandEntry>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxCopyToTextureQuery {
            pub name: String,
            pub xoff: i32,
            pub yoff: i32,
            pub x: i32,
            pub y: i32,
            pub width: i32,
            pub height: i32,
            pub target: u32,
            pub level: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxCreateShaderOptions {
            pub has_geo_input_type: bool,
            pub geo_input_type: u32,
            pub has_geo_output_type: bool,
            pub geo_output_type: u32,
            pub has_geo_output_verts: bool,
            pub geo_output_verts: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxCreateShaderQuery {
            pub definitions: String,
            pub vertex: String,
            pub tcs: String,
            pub tes: String,
            pub geometry: String,
            pub fragment: String,
            pub compute: String,
            pub options: GfxCreateShaderOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxCreateShaderResult {
            pub shader_id: u32,
            pub gl_program_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxCreateTextureAtlasQuery {
            pub xsize: i32,
            pub ysize: i32,
            pub alloc_type: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxCreateTextureQuery {
            pub xsize: i32,
            pub ysize: i32,
            pub zsize: i32,
            pub params: GfxTextureParams,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxDepthTestOptions {
            pub enable: bool,
            pub set_func: bool,
            pub func: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxDepthTestQuery {
            pub options: GfxDepthTestOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxDispatchComputeQuery {
            pub num_group_x: u32,
            pub num_group_y: u32,
            pub num_group_z: u32,
            pub barriers: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxDrawFuncAtUnitQuery {
            pub unit_id: i32,
            pub use_mid_pos: bool,
            pub callback: u32,
            pub user_data: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxDrawListAtUnitQuery {
            pub unit_id: i32,
            pub list_id: u32,
            pub use_mid_pos: bool,
            pub scale: Float3,
            pub degrees: f32,
            pub rot: Float3,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxEmptyQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxEmptyResult {
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxEngineModelUniformDataSizeResult {
            pub size_in_elements: u32,
            pub size_in_bytes_on_cpu: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxEngineTextureNamesResult {
            pub names: Vec<String>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxEngineUniformBufferDefQuery {
            pub index: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFBOAttachment {
            pub attachment: u32,
            pub texture_name: String,
            pub texture_target: u32,
            pub mip_level: i32,
            pub rbo_id: u32,
            pub use_rbo: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFBOAttachmentQuery {
            pub fbo_id: u32,
            pub attachment: u32,
            pub texture_name: String,
            pub texture_target: u32,
            pub mip_level: i32,
            pub rbo_id: u32,
            pub use_rbo: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFBOCreateQuery {
            pub target: u32,
            pub attachments: Vec<GfxFBOAttachment>,
            pub draw_buffers: Vec<u32>,
            pub read_buffer: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFBODrawBuffersQuery {
            pub fbo_id: u32,
            pub buffers: Vec<u32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFBOQuery {
            pub fbo_id: u32,
            pub target: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFBOReadBufferQuery {
            pub fbo_id: u32,
            pub buffer: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFBOResult {
            pub fbo_id: u32,
            pub raw_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFBOStatusResult {
            pub valid: bool,
            pub status: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFeatureDrawOptions {
            pub apply_transform: bool,
            pub do_raw_draw: bool,
            pub no_lua_call: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFeatureDrawQuery {
            pub feature_id: i32,
            pub options: GfxFeatureDrawOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFixedStateQuery {
            pub param: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFixedStateResult {
            pub bools: Vec<bool>,
            pub bool_count: u32,
            pub ints: Vec<i32>,
            pub int_count: u32,
            pub floats: Vec<f32>,
            pub float_count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFloatQuery {
            pub value: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFloatResult {
            pub value: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFontAutoOutlineColorQuery {
            pub font_id: u32,
            pub enable: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFontBeginQuery {
            pub font_id: u32,
            pub user_defined_blending: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFontColorQuery {
            pub font_id: u32,
            pub r: f32,
            pub g: f32,
            pub b: f32,
            pub a: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFontInfoResult {
            pub path: String,
            pub family: String,
            pub style: String,
            pub size: f32,
            pub line_height: f32,
            pub descender: f32,
            pub outline_width: f32,
            pub outline_weight: f32,
            pub texture_width: i32,
            pub texture_height: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFontQuery {
            pub font_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFontResult {
            pub font_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFontSubmitBufferedOptions {
            pub no_billboarding: bool,
            pub user_defined_blending: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFontSubmitBufferedQuery {
            pub font_id: u32,
            pub options: GfxFontSubmitBufferedOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFontTextQuery {
            pub font_id: u32,
            pub text: String,
            pub x: f32,
            pub y: f32,
            pub size: f32,
            pub options: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFontWorldTextQuery {
            pub font_id: u32,
            pub text: String,
            pub pos: Float3,
            pub size: f32,
            pub options: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFontWrapTextQuery {
            pub font_id: u32,
            pub text: String,
            pub max_width: f32,
            pub max_height: f32,
            pub size: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFontWrapTextResult {
            pub text: String,
            pub lines: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxFrustumQuery {
            pub left: f32,
            pub right: f32,
            pub bottom: f32,
            pub top: f32,
            pub near_val: f32,
            pub far_val: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxGeometryShaderParameterQuery {
            pub shader_id: u32,
            pub param: u32,
            pub value: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxGetMatrixDataQuery {
            pub mode: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxGetMatrixDataResult {
            pub values: Vec<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxGetNumberQuery {
            pub pname: u32,
            pub max_values: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxGetNumberResult {
            pub values: Vec<f32>,
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxGetStringQuery {
            pub pname: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxGroundCircleQuery {
            pub pos: Float3,
            pub radius: f32,
            pub resolution: i32,
            pub ballistic: bool,
            pub slope: f32,
            pub gravity: f32,
            pub weapon_def_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxGroundQuadQuery {
            pub x0: f32,
            pub z0: f32,
            pub x1: f32,
            pub z1: f32,
            pub use_tex_coords: bool,
            pub tu0: f32,
            pub tv0: f32,
            pub tu1: f32,
            pub tv1: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxIntQuery {
            pub value: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxIntResult {
            pub value: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxLightOptions {
            pub set_state: bool,
            pub state: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxLightQuery {
            pub light: i32,
            pub options: GfxLightOptions,
            pub pname: u32,
            pub values: Vec<f32>,
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxLineStippleQuery {
            pub factor: i32,
            pub pattern: u16,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxLoadFontQuery {
            pub path: String,
            pub size: i32,
            pub outline_width: i32,
            pub outline_weight: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxLogicOpQuery {
            pub enable: bool,
            pub opcode: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxMaterialQuery {
            pub pname: u32,
            pub values: Vec<f32>,
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxMatrixModeQuery {
            pub mode: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxMatrixQuery {
            pub values: Vec<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxMemoryBarrierQuery {
            pub barriers: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxMiniMapConfigQuery {
            pub px: i32,
            pub py: i32,
            pub sx: i32,
            pub sy: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxMultiTexCoordQuery {
            pub tex_num: i32,
            pub s: f32,
            pub t: f32,
            pub r: f32,
            pub q: f32,
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxMultiTexEnvQuery {
            pub tex_num: i32,
            pub target: u32,
            pub pname: u32,
            pub values: Vec<f32>,
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxMultiTexGenOptions {
            pub set_state: bool,
            pub state: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxMultiTexGenQuery {
            pub tex_num: i32,
            pub target: u32,
            pub options: GfxMultiTexGenOptions,
            pub pname: u32,
            pub values: Vec<f32>,
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxObjectBufferUniformsQuery {
            pub object_id: i32,
            pub values: Vec<f32>,
            pub offset: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxObjectBufferUniformsResult {
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxObjectLabelQuery {
            pub identifier: u32,
            pub object_id: u32,
            pub label: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxObjectPieceQuery {
            pub object_id: i32,
            pub piece_id: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxObjectShapeOptions {
            pub raw_state: bool,
            pub to_screen: bool,
            pub opaque: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxObjectShapeQuery {
            pub def_id: i32,
            pub team_id: i32,
            pub options: GfxObjectShapeOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxObjectTextureStateQuery {
            pub object_id: i32,
            pub push: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxOrthoQuery {
            pub left: f32,
            pub right: f32,
            pub bottom: f32,
            pub top: f32,
            pub near_val: f32,
            pub far_val: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxPointParameterQuery {
            pub pname: u32,
            pub value: f32,
            pub values: Vec<f32>,
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxPolygonModeQuery {
            pub face: u32,
            pub mode: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxPolygonOffsetQuery {
            pub factor: f32,
            pub units: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxPushDebugGroupQuery {
            pub id: u32,
            pub message: String,
            pub source_is_third_party: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxRBOCreateQuery {
            pub xsize: i32,
            pub ysize: i32,
            pub target: u32,
            pub format: u32,
            pub samples: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxRBOInfoQuery {
            pub rbo_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxRBOInfoResult {
            pub valid: bool,
            pub target: u32,
            pub format: u32,
            pub xsize: i32,
            pub ysize: i32,
            pub samples: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxRawBindFBOQuery {
            pub bind_default: bool,
            pub fbo_id: u32,
            pub target: u32,
            pub raw_fbo_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxRawBindFBOResult {
            pub previously_bound_raw_fbo_id: u32,
            pub has_previous: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxReadPixelsQuery {
            pub x: i32,
            pub y: i32,
            pub width: i32,
            pub height: i32,
            pub format: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxReadPixelsResult {
            pub values: Vec<f32>,
            pub components: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxRectQuery {
            pub x1: f32,
            pub y1: f32,
            pub x2: f32,
            pub y2: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxRenderToTextureQuery {
            pub name: String,
            pub callback: u32,
            pub user_data: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxRotateQuery {
            pub degrees: f32,
            pub x: f32,
            pub y: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxRunQueryQuery {
            pub id: u32,
            pub callback: u32,
            pub user_data: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxSaveImageOptions {
            pub alpha: bool,
            pub yflip: bool,
            pub grayscale16bit: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxSaveImageQuery {
            pub x: i32,
            pub y: i32,
            pub width: i32,
            pub height: i32,
            pub filename: String,
            pub options: GfxSaveImageOptions,
            pub read_buffer: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxScaleQuery {
            pub x: f32,
            pub y: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxScissorQuery {
            pub x: i32,
            pub y: i32,
            pub width: i32,
            pub height: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxShadeModelQuery {
            pub mode: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxShaderQuery {
            pub shader_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxShadowMapParamsResult {
            pub params: Float4,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxShapeQuery {
            pub primitive: u32,
            pub vertices: Vec<GfxVertexData>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxStencilFuncQuery {
            pub func: u32,
            pub ref_: i32,
            pub mask: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxStencilFuncSeparateQuery {
            pub face: u32,
            pub func: u32,
            pub ref_: i32,
            pub mask: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxStencilMaskQuery {
            pub mask: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxStencilMaskSeparateQuery {
            pub face: u32,
            pub mask: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxStencilOpQuery {
            pub fail: u32,
            pub zfail: u32,
            pub zpass: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxStencilOpSeparateQuery {
            pub face: u32,
            pub fail: u32,
            pub zfail: u32,
            pub zpass: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxStencilTestQuery {
            pub enable: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxStringQuery {
            pub value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxStringResult {
            pub value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxSubroutineIndexQuery {
            pub shader_id: u32,
            pub shader_type: u32,
            pub name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxSubroutineIndexResult {
            pub index: i32,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxTesselationShaderParameterQuery {
            pub param: u32,
            pub value: i32,
            pub values: Vec<f32>,
            pub value_count: u32,
            pub use_float_array: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxTexEnvQuery {
            pub target: u32,
            pub pname: u32,
            pub values: Vec<f32>,
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxTexGenOptions {
            pub set_state: bool,
            pub state: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxTexGenQuery {
            pub target: u32,
            pub options: GfxTexGenOptions,
            pub pname: u32,
            pub values: Vec<f32>,
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxTexRectQuery {
            pub x1: f32,
            pub y1: f32,
            pub x2: f32,
            pub y2: f32,
            pub s1: f32,
            pub t1: f32,
            pub s2: f32,
            pub t2: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxTextHeightResult {
            pub height: f32,
            pub descender: f32,
            pub lines: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxTextQuery {
            pub text: String,
            pub x: f32,
            pub y: f32,
            pub size: f32,
            pub options: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxTextureBindQuery {
            pub name: String,
            pub tex_num: i32,
            pub enable: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxTextureInfoResult {
            pub xsize: i32,
            pub ysize: i32,
            pub zsize: i32,
            pub id: u32,
            pub target: u32,
            pub fbo: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxTextureNameQuery {
            pub name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxTextureParams {
            pub target: u32,
            pub format: u32,
            pub border: i32,
            pub min_filter: u32,
            pub mag_filter: u32,
            pub wrap_s: u32,
            pub wrap_t: u32,
            pub wrap_r: u32,
            pub compare_func: u32,
            pub lod_bias: f32,
            pub aniso: f32,
            pub samples: u32,
            pub fbo: bool,
            pub fbo_depth: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxTranslateQuery {
            pub x: f32,
            pub y: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxTranslateResult {
            pub x: f32,
            pub y: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxUIntQuery {
            pub value: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxUIntResult {
            pub value: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxUniformArrayFloatQuery {
            pub location: i32,
            pub values: Vec<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxUniformArrayIntQuery {
            pub location: i32,
            pub values: Vec<i32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxUniformFloatQuery {
            pub location: i32,
            pub values: Vec<f32>,
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxUniformIntQuery {
            pub location: i32,
            pub values: Vec<i32>,
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxUniformLocationQuery {
            pub shader_id: u32,
            pub name: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxUniformLocationResult {
            pub location: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxUniformMatrixQuery {
            pub location: i32,
            pub values: Vec<f32>,
            pub transpose: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxUniformSubroutineQuery {
            pub shader_type: u32,
            pub index: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxUnitDrawOptions {
            pub apply_transform: bool,
            pub do_raw_draw: bool,
            pub no_lua_call: bool,
            pub full_model: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxUnitDrawQuery {
            pub unit_id: i32,
            pub options: GfxUnitDrawOptions,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxUnsafeStateQuery {
            pub state: u32,
            pub reverse: bool,
            pub callback: u32,
            pub user_data: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxUploadTextureQuery {
            pub name: String,
            pub target: u32,
            pub level: i32,
            pub xoff: i32,
            pub yoff: i32,
            pub zoff: i32,
            pub width: i32,
            pub height: i32,
            pub depth: i32,
            pub format: u32,
            pub pixel_type: u32,
            pub data: Vec<u8>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxUseShaderResult {
            pub linked: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxVAOBufferQuery {
            pub vao_id: u32,
            pub vbo_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxVAODrawArraysQuery {
            pub vao_id: u32,
            pub mode: u32,
            pub vertex_count: i32,
            pub vertex_first: i32,
            pub instance_count: i32,
            pub instance_first: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxVAODrawElementsQuery {
            pub vao_id: u32,
            pub mode: u32,
            pub draw_count: i32,
            pub base_index: i32,
            pub instance_count: i32,
            pub base_vertex: i32,
            pub base_instance: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxVAORemoveSubmissionQuery {
            pub vao_id: u32,
            pub index: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxVAOResult {
            pub vao_id: u32,
            pub raw_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxVAOSubmissionQuery {
            pub vao_id: u32,
            pub ids: Vec<u32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxVBOAttributeOptions {
            pub id: i32,
            pub type_: u32,
            pub size: i32,
            pub normalized: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxVBOBindRangeQuery {
            pub vbo_id: u32,
            pub binding_index: u32,
            pub element_offset: i32,
            pub element_count: i32,
            pub target: u32,
            pub bind: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxVBOCopyQuery {
            pub source_vboid: u32,
            pub destination_vboid: u32,
            pub copy_size_in_bytes: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxVBODefineQuery {
            pub vbo_id: u32,
            pub elements_count: i32,
            pub element_array: bool,
            pub index_type: u32,
            pub use_default_attributes: bool,
            pub default_attribute_count: u32,
            pub attributes: Vec<GfxVBOAttributeOptions>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxVBODownloadQuery {
            pub vbo_id: u32,
            pub attribute_index: i32,
            pub element_offset: i32,
            pub element_count: i32,
            pub force_gpu_read: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxVBODownloadResult {
            pub values: Vec<f32>,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxVBOInfoQuery {
            pub vbo_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxVBOInfoResult {
            pub elements_count: u32,
            pub buffer_size_in_bytes: u32,
            pub gpu_buffer_size_in_bytes: u32,
            pub elem_size_in_bytes: u32,
            pub attributes_count: u32,
            pub primitive_restart_index: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxVBOInstanceDataQuery {
            pub vbo_id: u32,
            pub ids: Vec<u32>,
            pub attribute_index: i32,
            pub team_id: i32,
            pub element_offset: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxVBOQuery {
            pub target: u32,
            pub freq_updated: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxVBOResult {
            pub vbo_id: u32,
            pub raw_id: u32,
            pub target: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxVBOUploadQuery {
            pub vbo_id: u32,
            pub data: Vec<f32>,
            pub attribute_index: i32,
            pub element_offset: i32,
            pub data_start_index: i32,
            pub data_finish_index: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxVBOUploadResult {
            pub bytes_written: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxValueQuery {
            pub key: String,
            pub mode: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxValueResult {
            pub values: Vec<f32>,
            pub count: u32,
            pub bool_value: Option<bool>,
            pub string_value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxVertexData {
            pub vertex: Vec<f32>,
            pub normal: Vec<f32>,
            pub tex_coord: Vec<f32>,
            pub color: Vec<f32>,
            pub has_vertex: bool,
            pub has_normal: bool,
            pub has_tex_coord: bool,
            pub has_color: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxVertexQuery {
            pub x: f32,
            pub y: f32,
            pub z: f32,
            pub w: f32,
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxViewRangeQuery {
            pub camera_type: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxViewRangeResult {
            pub near_plane_dist: f32,
            pub far_plane_dist: f32,
            pub min_view_range: f32,
            pub max_view_range: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxViewSizesResult {
            pub view_size_x: i32,
            pub view_size_y: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxViewportQuery {
            pub x: i32,
            pub y: i32,
            pub width: i32,
            pub height: i32,
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
        mod __core_variable_output_download_vbo {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "download-vbo"]
                pub fn call(pvbo_id: i32, pattribute_index: i32, pelement_offset: i32, pelement_count: i32, pforce_gpu_read: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_engine_model_uniform_data_def {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "get-engine-model-uniform-data-def"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_engine_uniform_buffer_def {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "get-engine-uniform-buffer-def"]
                pub fn call(pindex: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_shader_log {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "get-shader-log"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_string {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "get-string"]
                pub fn call(ppname: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CreateFBOValue {
            pub fbo_id: u32,
            pub raw_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct CreateShaderValue {
            pub shader_id: u32,
            pub gl_program_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FontGetTextHeightValue {
            pub height: f32,
            pub descender: f32,
            pub lines: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct FontWrapTextValue {
            pub text: String,
            pub lines: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAtlasTextureValue {
            pub x1: f32,
            pub x2: f32,
            pub y1: f32,
            pub y2: f32,
            pub page_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetAtmosphereValue {
            pub values: Vec<f32>,
            pub count: u32,
            pub bool_value: Option<bool>,
            pub string_value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetEngineModelUniformDataSizeValue {
            pub size_in_elements: u32,
            pub size_in_bytes_on_cpu: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFixedStateValue {
            pub bools: Vec<bool>,
            pub bool_count: u32,
            pub ints: Vec<i32>,
            pub int_count: u32,
            pub floats: Vec<f32>,
            pub float_count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetFontInfoValue {
            pub path: String,
            pub family: String,
            pub style: String,
            pub size: f32,
            pub line_height: f32,
            pub descender: f32,
            pub outline_width: f32,
            pub outline_weight: f32,
            pub texture_width: i32,
            pub texture_height: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetGlobalTexCoordsValue {
            pub x1: f32,
            pub x2: f32,
            pub y1: f32,
            pub y2: f32,
            pub page_num: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetMapRenderingValue {
            pub values: Vec<f32>,
            pub count: u32,
            pub bool_value: Option<bool>,
            pub string_value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetNumberValue {
            pub values: Vec<f32>,
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetRBOInfoValue {
            pub valid: bool,
            pub target: u32,
            pub format: u32,
            pub xsize: i32,
            pub ysize: i32,
            pub samples: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetScreenViewTransValue {
            pub x: f32,
            pub y: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSubroutineIndexValue {
            pub index: i32,
            pub success: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetSunValue {
            pub values: Vec<f32>,
            pub count: u32,
            pub bool_value: Option<bool>,
            pub string_value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetTextHeightValue {
            pub height: f32,
            pub descender: f32,
            pub lines: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetVAOValue {
            pub vao_id: u32,
            pub raw_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetVBOValue {
            pub vbo_id: u32,
            pub raw_id: u32,
            pub target: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetVBOInfoValue {
            pub elements_count: u32,
            pub buffer_size_in_bytes: u32,
            pub gpu_buffer_size_in_bytes: u32,
            pub elem_size_in_bytes: u32,
            pub attributes_count: u32,
            pub primitive_restart_index: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetViewRangeValue {
            pub near_plane_dist: f32,
            pub far_plane_dist: f32,
            pub min_view_range: f32,
            pub max_view_range: f32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetViewSizesValue {
            pub view_size_x: i32,
            pub view_size_y: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GetWaterRenderingValue {
            pub values: Vec<f32>,
            pub count: u32,
            pub bool_value: Option<bool>,
            pub string_value: String,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct IsValidFBOValue {
            pub valid: bool,
            pub status: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct RawBindFBOValue {
            pub previously_bound_raw_fbo_id: u32,
            pub has_previous: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ReadPixelsValue {
            pub values: Vec<f32>,
            pub components: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct TextureInfoValue {
            pub xsize: i32,
            pub ysize: i32,
            pub zsize: i32,
            pub id: u32,
            pub target: u32,
            pub fbo: u32,
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_active_fbo {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "active-fbo"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.active-fbo."]
        #[inline]
        pub unsafe fn active_fbo(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32) -> i32 {
            unsafe { __core_owned_active_fbo::call(p0, p1, p2, p3, p4) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_active_shader {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "active-shader"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.active-shader."]
        #[inline]
        pub unsafe fn active_shader(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_active_shader::call(p0, p1, p2) }
        }

        #[inline]
        pub fn active_texture(tex_num: i32) -> Result<()> {
            crate::generated::gfx::active_texture(tex_num)?;
            Ok(())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_add_atlas_texture {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "add-atlas-texture"]
                pub fn call(p0: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.add-atlas-texture."]
        #[inline]
        pub unsafe fn add_atlas_texture(p0: i32) -> i32 {
            unsafe { __core_owned_add_atlas_texture::call(p0) }
        }

        #[inline]
        pub fn add_fallback_font(value: &str) -> Result<bool> {
            let mut value_bytes = value.as_bytes().to_vec();
            if value_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            value_bytes.push(0);
            let value_cstr = core::ffi::CStr::from_bytes_with_nul(&value_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::gfx::add_fallback_font(&value_cstr)
        }

        #[inline]
        pub fn add_feature_defs_to_submission_vao(vao_id: u32, ids: &Vec<u32>) -> Result<u32> {
            crate::generated::borrowed::gfx::add_feature_defs_to_submission_vao(vao_id, ids.as_slice())
        }

        #[inline]
        pub fn add_features_to_submission_vao(vao_id: u32, ids: &Vec<u32>) -> Result<u32> {
            crate::generated::borrowed::gfx::add_features_to_submission_vao(vao_id, ids.as_slice())
        }

        #[inline]
        pub fn add_unit_defs_to_submission_vao(vao_id: u32, ids: &Vec<u32>) -> Result<u32> {
            crate::generated::borrowed::gfx::add_unit_defs_to_submission_vao(vao_id, ids.as_slice())
        }

        #[inline]
        pub fn add_units_to_submission_vao(vao_id: u32, ids: &Vec<u32>) -> Result<u32> {
            crate::generated::borrowed::gfx::add_units_to_submission_vao(vao_id, ids.as_slice())
        }

        #[inline]
        pub fn alpha_test(enable: bool, func: u32, ref_: f32) -> Result<()> {
            crate::generated::gfx::alpha_test(enable, func, ref_)?;
            Ok(())
        }

        #[inline]
        pub fn alpha_to_coverage(value: bool) -> Result<()> {
            crate::generated::gfx::alpha_to_coverage(value)?;
            Ok(())
        }

        #[inline]
        pub fn attach_index_buffer_vao(vao_id: u32, vbo_id: u32) -> Result<()> {
            crate::generated::gfx::attach_index_buffer_vao(vao_id, vbo_id)?;
            Ok(())
        }

        #[inline]
        pub fn attach_instance_buffer_vao(vao_id: u32, vbo_id: u32) -> Result<()> {
            crate::generated::gfx::attach_instance_buffer_vao(vao_id, vbo_id)?;
            Ok(())
        }

        #[inline]
        pub fn attach_vertex_buffer_vao(vao_id: u32, vbo_id: u32) -> Result<()> {
            crate::generated::gfx::attach_vertex_buffer_vao(vao_id, vbo_id)?;
            Ok(())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_begin_end {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "begin-end"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.begin-end."]
        #[inline]
        pub unsafe fn begin_end(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_begin_end::call(p0, p1, p2) }
        }

        #[inline]
        pub fn begin_text(value: bool) -> Result<()> {
            crate::generated::gfx::begin_text(value)?;
            Ok(())
        }

        #[inline]
        pub fn billboard(unused: u8) -> Result<()> {
            crate::generated::gfx::billboard(unused)?;
            Ok(())
        }

        #[inline]
        pub fn bind_buffer_range_vbo(vbo_id: u32, binding_index: u32, element_offset: i32, element_count: i32, target: u32, bind: bool) -> Result<i32> {
            let value = crate::generated::gfx::bind_buffer_range_vbo(vbo_id, binding_index, element_offset, element_count, target, bind)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_bind_image_texture {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "bind-image-texture"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.bind-image-texture."]
        #[inline]
        pub unsafe fn bind_image_texture(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32) -> i32 {
            unsafe { __core_owned_bind_image_texture::call(p0, p1, p2, p3, p4, p5, p6) }
        }

        #[inline]
        pub fn bind_texture(name: &str, tex_num: i32, enable: bool) -> Result<bool> {
            let mut name_bytes = name.as_bytes().to_vec();
            if name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            name_bytes.push(0);
            let name_cstr = core::ffi::CStr::from_bytes_with_nul(&name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::gfx::bind_texture(&name_cstr, tex_num, enable)
        }

        #[inline]
        pub fn blend_equation(mode: u32) -> Result<()> {
            crate::generated::gfx::blend_equation(mode)?;
            Ok(())
        }

        #[inline]
        pub fn blend_equation_separate(mode_rgb: u32, mode_alpha: u32) -> Result<()> {
            crate::generated::gfx::blend_equation_separate(mode_rgb, mode_alpha)?;
            Ok(())
        }

        #[inline]
        pub fn blend_func(src: u32, dst: u32) -> Result<()> {
            crate::generated::gfx::blend_func(src, dst)?;
            Ok(())
        }

        #[inline]
        pub fn blend_func_separate(src_rgb: u32, dst_rgb: u32, src_alpha: u32, dst_alpha: u32) -> Result<()> {
            crate::generated::gfx::blend_func_separate(src_rgb, dst_rgb, src_alpha, dst_alpha)?;
            Ok(())
        }

        #[inline]
        pub fn blending(value: bool) -> Result<()> {
            crate::generated::gfx::blending(value)?;
            Ok(())
        }

        #[inline]
        pub fn blit_fbo(src_fboid: u32, dst_fboid: u32, x0_src: i32, y0_src: i32, x1_src: i32, y1_src: i32, x0_dst: i32, y0_dst: i32, x1_dst: i32, y1_dst: i32, mask: u32, filter: u32) -> Result<()> {
            crate::generated::gfx::blit_fbo(src_fboid, dst_fboid, x0_src, y0_src, x1_src, y1_src, x0_dst, y0_dst, x1_dst, y1_dst, mask, filter)?;
            Ok(())
        }

        #[inline]
        pub fn call_list(value: u32) -> Result<()> {
            crate::generated::gfx::call_list(value)?;
            Ok(())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_change_texture_params {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "change-texture-params"]
                pub fn call(p0: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.change-texture-params."]
        #[inline]
        pub unsafe fn change_texture_params(p0: i32) -> i32 {
            unsafe { __core_owned_change_texture_params::call(p0) }
        }

        #[inline]
        pub fn clear(bits: u32, values: &Vec<f32>, count: u32) -> Result<()> {
            crate::generated::gfx::clear(bits, values.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, count)?;
            Ok(())
        }

        #[inline]
        pub fn clear_attachment_fbo(target: u32, attachment: u32, values: &Vec<f32>, count: u32) -> Result<bool> {
            let value = crate::generated::gfx::clear_attachment_fbo(target, attachment, values.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, count)?;
            Ok(value)
        }

        #[inline]
        pub fn clear_fallback_fonts(unused: u8) -> Result<()> {
            crate::generated::gfx::clear_fallback_fonts(unused)?;
            Ok(())
        }

        #[inline]
        pub fn clear_submission_vao(value: u32) -> Result<()> {
            crate::generated::gfx::clear_submission_vao(value)?;
            Ok(())
        }

        #[inline]
        pub fn clear_vbo(value: u32) -> Result<()> {
            crate::generated::gfx::clear_vbo(value)?;
            Ok(())
        }

        #[inline]
        pub fn clip_distance(index: u32, enable: bool) -> Result<()> {
            crate::generated::gfx::clip_distance(index, enable)?;
            Ok(())
        }

        #[inline]
        pub fn clip_plane(plane: u32, equation: &Vec<f32>) -> Result<()> {
            crate::generated::gfx::clip_plane(plane, equation.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?)?;
            Ok(())
        }

        #[inline]
        pub fn color(r: f32, g: f32, b: f32, a: f32) -> Result<()> {
            crate::generated::gfx::color(r, g, b, a)?;
            Ok(())
        }

        #[inline]
        pub fn color_mask(options: GfxColorMaskOptions) -> Result<()> {
            crate::generated::gfx::color_mask(crate::generated::gfx::GfxColorMaskOptions { red: options.red, green: options.green, blue: options.blue, alpha: options.alpha })?;
            Ok(())
        }

        #[inline]
        pub fn config_mini_map(px: i32, py: i32, sx: i32, sy: i32) -> Result<()> {
            crate::generated::gfx::config_mini_map(px, py, sx, sy)?;
            Ok(())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_copy_to_texture {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "copy-to-texture"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32, p7: i32, p8: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.copy-to-texture."]
        #[inline]
        pub unsafe fn copy_to_texture(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32, p7: i32, p8: i32) -> i32 {
            unsafe { __core_owned_copy_to_texture::call(p0, p1, p2, p3, p4, p5, p6, p7, p8) }
        }

        #[inline]
        pub fn copy_to_vbo(source_vboid: u32, destination_vboid: u32, copy_size_in_bytes: i32) -> Result<bool> {
            let value = crate::generated::gfx::copy_to_vbo(source_vboid, destination_vboid, copy_size_in_bytes)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_create_fbo {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "create-fbo"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.create-fbo."]
        #[inline]
        pub unsafe fn create_fbo(p0: i32, p1: i32, p2: i32, p3: i32) -> i32 {
            unsafe { __core_owned_create_fbo::call(p0, p1, p2, p3) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_create_list {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "create-list"]
                pub fn call(p0: i32, p1: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.create-list."]
        #[inline]
        pub unsafe fn create_list(p0: i32, p1: i32) -> i64 {
            unsafe { __core_owned_create_list::call(p0, p1) }
        }

        #[inline]
        pub fn create_query(unused: u8) -> Result<u32> {
            let value = crate::generated::gfx::create_query(unused)?;
            Ok(value)
        }

        #[inline]
        pub fn create_rbo(xsize: i32, ysize: i32, target: u32, format: u32, samples: i32) -> Result<u32> {
            let value = crate::generated::gfx::create_rbo(xsize, ysize, target, format, samples)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_create_shader {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "create-shader"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.create-shader."]
        #[inline]
        pub unsafe fn create_shader(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_create_shader::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_create_texture {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "create-texture"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.create-texture."]
        #[inline]
        pub unsafe fn create_texture(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32) -> i64 {
            unsafe { __core_owned_create_texture::call(p0, p1, p2, p3, p4, p5) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_create_texture_atlas {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "create-texture-atlas"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.create-texture-atlas."]
        #[inline]
        pub unsafe fn create_texture_atlas(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32) -> i64 {
            unsafe { __core_owned_create_texture_atlas::call(p0, p1, p2, p3, p4) }
        }

        #[inline]
        pub fn culling(value: bool) -> Result<()> {
            crate::generated::gfx::culling(value)?;
            Ok(())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_define_vbo {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "define-vbo"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.define-vbo."]
        #[inline]
        pub unsafe fn define_vbo(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32) -> i32 {
            unsafe { __core_owned_define_vbo::call(p0, p1, p2, p3, p4, p5, p6) }
        }

        #[inline]
        pub fn delete_fbo(value: u32) -> Result<()> {
            crate::generated::gfx::delete_fbo(value)?;
            Ok(())
        }

        #[inline]
        pub fn delete_font(font_id: u32) -> Result<()> {
            crate::generated::gfx::delete_font(font_id)?;
            Ok(())
        }

        #[inline]
        pub fn delete_list(value: u32) -> Result<()> {
            crate::generated::gfx::delete_list(value)?;
            Ok(())
        }

        #[inline]
        pub fn delete_query(value: u32) -> Result<()> {
            crate::generated::gfx::delete_query(value)?;
            Ok(())
        }

        #[inline]
        pub fn delete_rbo(value: u32) -> Result<()> {
            crate::generated::gfx::delete_rbo(value)?;
            Ok(())
        }

        #[inline]
        pub fn delete_shader(shader_id: u32) -> Result<bool> {
            let value = crate::generated::gfx::delete_shader(shader_id)?;
            Ok(value)
        }

        #[inline]
        pub fn delete_texture(name: &str) -> Result<bool> {
            let mut name_bytes = name.as_bytes().to_vec();
            if name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            name_bytes.push(0);
            let name_cstr = core::ffi::CStr::from_bytes_with_nul(&name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::gfx::delete_texture(&name_cstr)
        }

        #[inline]
        pub fn delete_texture_atlas(name: &str) -> Result<bool> {
            let mut name_bytes = name.as_bytes().to_vec();
            if name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            name_bytes.push(0);
            let name_cstr = core::ffi::CStr::from_bytes_with_nul(&name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::gfx::delete_texture_atlas(&name_cstr)
        }

        #[inline]
        pub fn delete_texture_fbo(name: &str) -> Result<bool> {
            let mut name_bytes = name.as_bytes().to_vec();
            if name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            name_bytes.push(0);
            let name_cstr = core::ffi::CStr::from_bytes_with_nul(&name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::gfx::delete_texture_fbo(&name_cstr)
        }

        #[inline]
        pub fn delete_vao(value: u32) -> Result<()> {
            crate::generated::gfx::delete_vao(value)?;
            Ok(())
        }

        #[inline]
        pub fn delete_vbo(value: u32) -> Result<()> {
            crate::generated::gfx::delete_vbo(value)?;
            Ok(())
        }

        #[inline]
        pub fn depth_clamp(value: bool) -> Result<()> {
            crate::generated::gfx::depth_clamp(value)?;
            Ok(())
        }

        #[inline]
        pub fn depth_mask(value: bool) -> Result<()> {
            crate::generated::gfx::depth_mask(value)?;
            Ok(())
        }

        #[inline]
        pub fn depth_test(options: GfxDepthTestOptions) -> Result<()> {
            crate::generated::gfx::depth_test(crate::generated::gfx::GfxDepthTestOptions { enable: options.enable, set_func: options.set_func, func: options.func })?;
            Ok(())
        }

        #[inline]
        pub fn dispatch_compute(num_group_x: u32, num_group_y: u32, num_group_z: u32, barriers: u32) -> Result<()> {
            crate::generated::gfx::dispatch_compute(num_group_x, num_group_y, num_group_z, barriers)?;
            Ok(())
        }

        #[inline]
        pub fn download_vbo(vbo_id: u32, attribute_index: i32, element_offset: i32, element_count: i32, force_gpu_read: bool) -> Result<Vec<f32>> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<f32>::new();
                loop {
                    let status = unsafe { __core_variable_output_download_vbo::call(vbo_id as i32, attribute_index as i32, element_offset as i32, element_count as i32, u32::from(force_gpu_read) as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (vbo_id as i32, attribute_index as i32, element_offset as i32, element_count as i32, u32::from(force_gpu_read) as i32);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn draw_arrays_vao(vao_id: u32, mode: u32, vertex_count: i32, vertex_first: i32, instance_count: i32, instance_first: i32) -> Result<()> {
            crate::generated::gfx::draw_arrays_vao(vao_id, mode, vertex_count, vertex_first, instance_count, instance_first)?;
            Ok(())
        }

        #[inline]
        pub fn draw_elements_vao(vao_id: u32, mode: u32, draw_count: i32, base_index: i32, instance_count: i32, base_vertex: i32, base_instance: i32) -> Result<()> {
            crate::generated::gfx::draw_elements_vao(vao_id, mode, draw_count, base_index, instance_count, base_vertex, base_instance)?;
            Ok(())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_draw_func_at_unit {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "draw-func-at-unit"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.draw-func-at-unit."]
        #[inline]
        pub unsafe fn draw_func_at_unit(p0: i32, p1: i32, p2: i32, p3: i32) -> i32 {
            unsafe { __core_owned_draw_func_at_unit::call(p0, p1, p2, p3) }
        }

        #[inline]
        pub fn draw_ground_circle(pos: Float3, radius: f32, resolution: i32, ballistic: bool, slope: f32, gravity: f32, weapon_def_id: i32) -> Result<()> {
            crate::generated::gfx::draw_ground_circle(crate::generated::gfx::Float3 { x: pos.x, y: pos.y, z: pos.z }, radius, resolution, ballistic, slope, gravity, weapon_def_id)?;
            Ok(())
        }

        #[inline]
        pub fn draw_ground_quad(x0: f32, z0: f32, x1: f32, z1: f32, use_tex_coords: bool, tu0: f32, tv0: f32, tu1: f32, tv1: f32) -> Result<()> {
            crate::generated::gfx::draw_ground_quad(x0, z0, x1, z1, use_tex_coords, tu0, tv0, tu1, tv1)?;
            Ok(())
        }

        #[inline]
        pub fn draw_list_at_unit(unit_id: i32, list_id: u32, use_mid_pos: bool, scale: Float3, degrees: f32, rot: Float3) -> Result<()> {
            crate::generated::gfx::draw_list_at_unit(unit_id, list_id, use_mid_pos, crate::generated::gfx::Float3 { x: scale.x, y: scale.y, z: scale.z }, degrees, crate::generated::gfx::Float3 { x: rot.x, y: rot.y, z: rot.z })?;
            Ok(())
        }

        #[inline]
        pub fn draw_mini_map(value: bool) -> Result<()> {
            crate::generated::gfx::draw_mini_map(value)?;
            Ok(())
        }

        #[inline]
        pub fn dump_definition_vbo(value: u32) -> Result<()> {
            crate::generated::gfx::dump_definition_vbo(value)?;
            Ok(())
        }

        #[inline]
        pub fn edge_flag(value: bool) -> Result<()> {
            crate::generated::gfx::edge_flag(value)?;
            Ok(())
        }

        #[inline]
        pub fn end_text(unused: u8) -> Result<()> {
            crate::generated::gfx::end_text(unused)?;
            Ok(())
        }

        #[inline]
        pub fn feature(feature_id: i32, options: GfxFeatureDrawOptions) -> Result<()> {
            crate::generated::gfx::feature(feature_id, crate::generated::gfx::GfxFeatureDrawOptions { apply_transform: options.apply_transform, do_raw_draw: options.do_raw_draw, no_lua_call: options.no_lua_call })?;
            Ok(())
        }

        #[inline]
        pub fn feature_mult_matrix(value: i32) -> Result<()> {
            crate::generated::gfx::feature_mult_matrix(value)?;
            Ok(())
        }

        #[inline]
        pub fn feature_piece(object_id: i32, piece_id: i32) -> Result<()> {
            crate::generated::gfx::feature_piece(object_id, piece_id)?;
            Ok(())
        }

        #[inline]
        pub fn feature_piece_matrix(object_id: i32, piece_id: i32) -> Result<()> {
            crate::generated::gfx::feature_piece_matrix(object_id, piece_id)?;
            Ok(())
        }

        #[inline]
        pub fn feature_piece_mult_matrix(object_id: i32, piece_id: i32) -> Result<()> {
            crate::generated::gfx::feature_piece_mult_matrix(object_id, piece_id)?;
            Ok(())
        }

        #[inline]
        pub fn feature_raw(feature_id: i32, options: GfxFeatureDrawOptions) -> Result<()> {
            crate::generated::gfx::feature_raw(feature_id, crate::generated::gfx::GfxFeatureDrawOptions { apply_transform: options.apply_transform, do_raw_draw: options.do_raw_draw, no_lua_call: options.no_lua_call })?;
            Ok(())
        }

        #[inline]
        pub fn feature_shape(def_id: i32, team_id: i32, options: GfxObjectShapeOptions) -> Result<()> {
            crate::generated::gfx::feature_shape(def_id, team_id, crate::generated::gfx::GfxObjectShapeOptions { raw_state: options.raw_state, to_screen: options.to_screen, opaque: options.opaque })?;
            Ok(())
        }

        #[inline]
        pub fn feature_shape_textures(object_id: i32, push: bool) -> Result<()> {
            crate::generated::gfx::feature_shape_textures(object_id, push)?;
            Ok(())
        }

        #[inline]
        pub fn feature_textures(object_id: i32, push: bool) -> Result<()> {
            crate::generated::gfx::feature_textures(object_id, push)?;
            Ok(())
        }

        #[inline]
        pub fn finalize_texture_atlas(name: &str) -> Result<bool> {
            let mut name_bytes = name.as_bytes().to_vec();
            if name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            name_bytes.push(0);
            let name_cstr = core::ffi::CStr::from_bytes_with_nul(&name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::gfx::finalize_texture_atlas(&name_cstr)
        }

        #[inline]
        pub fn finish(unused: u8) -> Result<()> {
            crate::generated::gfx::finish(unused)?;
            Ok(())
        }

        #[inline]
        pub fn flush(unused: u8) -> Result<()> {
            crate::generated::gfx::flush(unused)?;
            Ok(())
        }

        #[inline]
        pub fn fog(value: bool) -> Result<()> {
            crate::generated::gfx::fog(value)?;
            Ok(())
        }

        #[inline]
        pub fn fog_coord(value: f32) -> Result<()> {
            crate::generated::gfx::fog_coord(value)?;
            Ok(())
        }

        #[inline]
        pub fn font_begin(font_id: u32, user_defined_blending: bool) -> Result<()> {
            crate::generated::gfx::font_begin(font_id, user_defined_blending)?;
            Ok(())
        }

        #[inline]
        pub fn font_bind_texture(font_id: u32) -> Result<()> {
            crate::generated::gfx::font_bind_texture(font_id)?;
            Ok(())
        }

        #[inline]
        pub fn font_end(font_id: u32) -> Result<()> {
            crate::generated::gfx::font_end(font_id)?;
            Ok(())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_font_get_text_height {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "font-get-text-height"]
                pub fn call(p0: i32, p1: f32, p2: f32, p3: f32, p4: i32, p5: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.font-get-text-height."]
        #[inline]
        pub unsafe fn font_get_text_height(p0: i32, p1: f32, p2: f32, p3: f32, p4: i32, p5: i32) -> i32 {
            unsafe { __core_owned_font_get_text_height::call(p0, p1, p2, p3, p4, p5) }
        }

        #[inline]
        pub fn font_get_text_width(font_id: u32, text: &str, x: f32, y: f32, size: f32, options: &str) -> Result<f32> {
            let mut text_bytes = text.as_bytes().to_vec();
            if text_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            text_bytes.push(0);
            let text_cstr = core::ffi::CStr::from_bytes_with_nul(&text_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            let mut options_bytes = options.as_bytes().to_vec();
            if options_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            options_bytes.push(0);
            let options_cstr = core::ffi::CStr::from_bytes_with_nul(&options_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::gfx::font_get_text_width(font_id, &text_cstr, x, y, size, &options_cstr)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_font_print {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "font-print"]
                pub fn call(p0: i32, p1: f32, p2: f32, p3: f32, p4: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.font-print."]
        #[inline]
        pub unsafe fn font_print(p0: i32, p1: f32, p2: f32, p3: f32, p4: i32) -> i32 {
            unsafe { __core_owned_font_print::call(p0, p1, p2, p3, p4) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_font_print_world {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "font-print-world"]
                pub fn call(p0: i32, p1: f32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.font-print-world."]
        #[inline]
        pub unsafe fn font_print_world(p0: i32, p1: f32, p2: i32) -> i32 {
            unsafe { __core_owned_font_print_world::call(p0, p1, p2) }
        }

        #[inline]
        pub fn font_set_auto_outline_color(font_id: u32, enable: bool) -> Result<()> {
            crate::generated::gfx::font_set_auto_outline_color(font_id, enable)?;
            Ok(())
        }

        #[inline]
        pub fn font_set_outline_color(font_id: u32, r: f32, g: f32, b: f32, a: f32) -> Result<()> {
            crate::generated::gfx::font_set_outline_color(font_id, r, g, b, a)?;
            Ok(())
        }

        #[inline]
        pub fn font_set_text_color(font_id: u32, r: f32, g: f32, b: f32, a: f32) -> Result<()> {
            crate::generated::gfx::font_set_text_color(font_id, r, g, b, a)?;
            Ok(())
        }

        #[inline]
        pub fn font_submit_buffered(font_id: u32, options: GfxFontSubmitBufferedOptions) -> Result<()> {
            crate::generated::gfx::font_submit_buffered(font_id, crate::generated::gfx::GfxFontSubmitBufferedOptions { no_billboarding: options.no_billboarding, user_defined_blending: options.user_defined_blending })?;
            Ok(())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_font_wrap_text {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "font-wrap-text"]
                pub fn call(p0: i32, p1: f32, p2: f32, p3: f32, p4: i32, p5: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.font-wrap-text."]
        #[inline]
        pub unsafe fn font_wrap_text(p0: i32, p1: f32, p2: f32, p3: f32, p4: i32, p5: i32) -> i32 {
            unsafe { __core_owned_font_wrap_text::call(p0, p1, p2, p3, p4, p5) }
        }

        #[inline]
        pub fn frustum(left: f32, right: f32, bottom: f32, top: f32, near_val: f32, far_val: f32) -> Result<()> {
            crate::generated::gfx::frustum(left, right, bottom, top, near_val, far_val)?;
            Ok(())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_generate_mipmap {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "generate-mipmap"]
                pub fn call(p0: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.generate-mipmap."]
        #[inline]
        pub unsafe fn generate_mipmap(p0: i32) -> i32 {
            unsafe { __core_owned_generate_mipmap::call(p0) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_active_uniforms {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "get-active-uniforms"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.get-active-uniforms."]
        #[inline]
        pub unsafe fn get_active_uniforms(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_active_uniforms::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_atlas_texture {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "get-atlas-texture"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.get-atlas-texture."]
        #[inline]
        pub unsafe fn get_atlas_texture(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_atlas_texture::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_atmosphere {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "get-atmosphere"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.get-atmosphere."]
        #[inline]
        pub unsafe fn get_atmosphere(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_atmosphere::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_console_commands {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "get-console-commands"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.get-console-commands."]
        #[inline]
        pub unsafe fn get_console_commands(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_console_commands::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_engine_atlas_textures {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "get-engine-atlas-textures"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.get-engine-atlas-textures."]
        #[inline]
        pub unsafe fn get_engine_atlas_textures(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_engine_atlas_textures::call(p0, p1) }
        }

        #[inline]
        pub fn get_engine_model_uniform_data_def(unused: u8) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_engine_model_uniform_data_def::call(unused as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
        pub fn get_engine_model_uniform_data_size(unused: u8) -> Result<GetEngineModelUniformDataSizeValue> {
            let value = crate::generated::gfx::get_engine_model_uniform_data_size(unused)?;
            Ok(GetEngineModelUniformDataSizeValue {
                size_in_elements: value.0,
                size_in_bytes_on_cpu: value.1
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_engine_texture_names {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "get-engine-texture-names"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.get-engine-texture-names."]
        #[inline]
        pub unsafe fn get_engine_texture_names(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_engine_texture_names::call(p0, p1) }
        }

        #[inline]
        pub fn get_engine_uniform_buffer_def(index: i32) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_engine_uniform_buffer_def::call(index as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (index as i32);
                Err(unreachable!())
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_fixed_state {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "get-fixed-state"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.get-fixed-state."]
        #[inline]
        pub unsafe fn get_fixed_state(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_fixed_state::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_font_info {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "get-font-info"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.get-font-info."]
        #[inline]
        pub unsafe fn get_font_info(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_font_info::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_global_tex_coords {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "get-global-tex-coords"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.get-global-tex-coords."]
        #[inline]
        pub unsafe fn get_global_tex_coords(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_global_tex_coords::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_global_tex_names {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "get-global-tex-names"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.get-global-tex-names."]
        #[inline]
        pub unsafe fn get_global_tex_names(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_global_tex_names::call(p0, p1) }
        }

        #[inline]
        pub fn get_idvbo(value: u32) -> Result<u32> {
            let value = crate::generated::gfx::get_idvbo(value)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_map_rendering {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "get-map-rendering"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.get-map-rendering."]
        #[inline]
        pub unsafe fn get_map_rendering(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_map_rendering::call(p0, p1) }
        }

        #[inline]
        pub fn get_matrix_data(mode: u32) -> Result<Vec<f32>> {
            let value = crate::generated::gfx::get_matrix_data(mode)?;
            Ok(value.into_iter().map(|value| Ok(value)).collect::<crate::Result<Vec<_>>>()?)
        }

        #[inline]
        pub fn get_number(pname: u32, max_values: u32) -> Result<GetNumberValue> {
            let value = crate::generated::gfx::get_number(pname, max_values)?;
            Ok(GetNumberValue {
                values: value.0.into_iter().map(|value| Ok(value)).collect::<crate::Result<Vec<_>>>()?,
                count: value.1
            })
        }

        #[inline]
        pub fn get_query(value: u32) -> Result<u32> {
            let value = crate::generated::gfx::get_query(value)?;
            Ok(value)
        }

        #[inline]
        pub fn get_rbo_info(rbo_id: u32) -> Result<GetRBOInfoValue> {
            let value = crate::generated::gfx::get_rbo_info(rbo_id)?;
            Ok(GetRBOInfoValue {
                valid: value.0,
                target: value.1,
                format: value.2,
                xsize: value.3,
                ysize: value.4,
                samples: value.5
            })
        }

        #[inline]
        pub fn get_screen_view_trans(unused: u8) -> Result<GetScreenViewTransValue> {
            let value = crate::generated::gfx::get_screen_view_trans(unused)?;
            Ok(GetScreenViewTransValue {
                x: value.0,
                y: value.1,
                z: value.2
            })
        }

        #[inline]
        pub fn get_shader_log(unused: u8) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_shader_log::call(unused as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
        pub fn get_shadow_map_params(unused: u8) -> Result<Float4> {
            let value = crate::generated::gfx::get_shadow_map_params(unused)?;
            Ok(Float4 { x: value.x, y: value.y, z: value.z, w: value.w })
        }

        #[inline]
        pub fn get_string(pname: u32) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_string::call(pname as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (pname as i32);
                Err(unreachable!())
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_subroutine_index {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "get-subroutine-index"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.get-subroutine-index."]
        #[inline]
        pub unsafe fn get_subroutine_index(p0: i32, p1: i32, p2: i32, p3: i32) -> i32 {
            unsafe { __core_owned_get_subroutine_index::call(p0, p1, p2, p3) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_sun {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "get-sun"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.get-sun."]
        #[inline]
        pub unsafe fn get_sun(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_sun::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_text_height {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "get-text-height"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.get-text-height."]
        #[inline]
        pub unsafe fn get_text_height(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_text_height::call(p0, p1) }
        }

        #[inline]
        pub fn get_text_width(value: &str) -> Result<f32> {
            let mut value_bytes = value.as_bytes().to_vec();
            if value_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            value_bytes.push(0);
            let value_cstr = core::ffi::CStr::from_bytes_with_nul(&value_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::gfx::get_text_width(&value_cstr)
        }

        #[inline]
        pub fn get_uniform_location(shader_id: u32, name: &str) -> Result<i32> {
            let mut name_bytes = name.as_bytes().to_vec();
            if name_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            name_bytes.push(0);
            let name_cstr = core::ffi::CStr::from_bytes_with_nul(&name_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::gfx::get_uniform_location(shader_id, &name_cstr)
        }

        #[inline]
        pub fn get_vao(unused: u8) -> Result<GetVAOValue> {
            let value = crate::generated::gfx::get_vao(unused)?;
            Ok(GetVAOValue {
                vao_id: value.0,
                raw_id: value.1
            })
        }

        #[inline]
        pub fn get_vbo(target: u32, freq_updated: bool) -> Result<GetVBOValue> {
            let value = crate::generated::gfx::get_vbo(target, freq_updated)?;
            Ok(GetVBOValue {
                vbo_id: value.0,
                raw_id: value.1,
                target: value.2
            })
        }

        #[inline]
        pub fn get_vbo_info(vbo_id: u32) -> Result<GetVBOInfoValue> {
            let value = crate::generated::gfx::get_vbo_info(vbo_id)?;
            Ok(GetVBOInfoValue {
                elements_count: value.0,
                buffer_size_in_bytes: value.1,
                gpu_buffer_size_in_bytes: value.2,
                elem_size_in_bytes: value.3,
                attributes_count: value.4,
                primitive_restart_index: value.5
            })
        }

        #[inline]
        pub fn get_view_range(camera_type: i32) -> Result<GetViewRangeValue> {
            let value = crate::generated::gfx::get_view_range(camera_type)?;
            Ok(GetViewRangeValue {
                near_plane_dist: value.0,
                far_plane_dist: value.1,
                min_view_range: value.2,
                max_view_range: value.3
            })
        }

        #[inline]
        pub fn get_view_sizes(unused: u8) -> Result<GetViewSizesValue> {
            let value = crate::generated::gfx::get_view_sizes(unused)?;
            Ok(GetViewSizesValue {
                view_size_x: value.0,
                view_size_y: value.1
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_water_rendering {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "get-water-rendering"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.get-water-rendering."]
        #[inline]
        pub unsafe fn get_water_rendering(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_get_water_rendering::call(p0, p1) }
        }

        #[inline]
        pub fn has_extension(value: &str) -> Result<bool> {
            let mut value_bytes = value.as_bytes().to_vec();
            if value_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            value_bytes.push(0);
            let value_cstr = core::ffi::CStr::from_bytes_with_nul(&value_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::gfx::has_extension(&value_cstr)
        }

        #[inline]
        pub fn instance_data_from_feature_defs_vbo(vbo_id: u32, ids: &Vec<u32>, attribute_index: i32, team_id: i32, element_offset: i32) -> Result<u32> {
            crate::generated::borrowed::gfx::instance_data_from_feature_defs_vbo(vbo_id, ids.as_slice(), attribute_index, team_id, element_offset)
        }

        #[inline]
        pub fn instance_data_from_features_vbo(vbo_id: u32, ids: &Vec<u32>, attribute_index: i32, team_id: i32, element_offset: i32) -> Result<u32> {
            crate::generated::borrowed::gfx::instance_data_from_features_vbo(vbo_id, ids.as_slice(), attribute_index, team_id, element_offset)
        }

        #[inline]
        pub fn instance_data_from_unit_defs_vbo(vbo_id: u32, ids: &Vec<u32>, attribute_index: i32, team_id: i32, element_offset: i32) -> Result<u32> {
            crate::generated::borrowed::gfx::instance_data_from_unit_defs_vbo(vbo_id, ids.as_slice(), attribute_index, team_id, element_offset)
        }

        #[inline]
        pub fn instance_data_from_units_vbo(vbo_id: u32, ids: &Vec<u32>, attribute_index: i32, team_id: i32, element_offset: i32) -> Result<u32> {
            crate::generated::borrowed::gfx::instance_data_from_units_vbo(vbo_id, ids.as_slice(), attribute_index, team_id, element_offset)
        }

        #[inline]
        pub fn is_valid_fbo(fbo_id: u32, target: u32) -> Result<IsValidFBOValue> {
            let value = crate::generated::gfx::is_valid_fbo(fbo_id, target)?;
            Ok(IsValidFBOValue {
                valid: value.0,
                status: value.1
            })
        }

        #[inline]
        pub fn light(light: i32, options: GfxLightOptions, pname: u32, values: &Vec<f32>, count: u32) -> Result<()> {
            crate::generated::gfx::light(light, crate::generated::gfx::GfxLightOptions { set_state: options.set_state, state: options.state }, pname, values.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, count)?;
            Ok(())
        }

        #[inline]
        pub fn lighting(value: bool) -> Result<()> {
            crate::generated::gfx::lighting(value)?;
            Ok(())
        }

        #[inline]
        pub fn line_stipple(factor: i32, pattern: u16) -> Result<()> {
            crate::generated::gfx::line_stipple(factor, pattern)?;
            Ok(())
        }

        #[inline]
        pub fn line_width(value: f32) -> Result<()> {
            crate::generated::gfx::line_width(value)?;
            Ok(())
        }

        #[inline]
        pub fn load_font(path: &str, size: i32, outline_width: i32, outline_weight: f32) -> Result<u32> {
            let mut path_bytes = path.as_bytes().to_vec();
            if path_bytes.contains(&0) { return Err(crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32)); }
            path_bytes.push(0);
            let path_cstr = core::ffi::CStr::from_bytes_with_nul(&path_bytes).map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?;
            crate::generated::borrowed::gfx::load_font(&path_cstr, size, outline_width, outline_weight)
        }

        #[inline]
        pub fn load_identity(unused: u8) -> Result<()> {
            crate::generated::gfx::load_identity(unused)?;
            Ok(())
        }

        #[inline]
        pub fn load_matrix(values: &Vec<f32>) -> Result<()> {
            crate::generated::gfx::load_matrix(values.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?)?;
            Ok(())
        }

        #[inline]
        pub fn logic_op(enable: bool, opcode: u32) -> Result<()> {
            crate::generated::gfx::logic_op(enable, opcode)?;
            Ok(())
        }

        #[inline]
        pub fn material(pname: u32, values: &Vec<f32>, count: u32) -> Result<()> {
            crate::generated::gfx::material(pname, values.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, count)?;
            Ok(())
        }

        #[inline]
        pub fn matrix_data_from_projectiles_vbo(vbo_id: u32, ids: &Vec<u32>, attribute_index: i32, team_id: i32, element_offset: i32) -> Result<u32> {
            crate::generated::borrowed::gfx::matrix_data_from_projectiles_vbo(vbo_id, ids.as_slice(), attribute_index, team_id, element_offset)
        }

        #[inline]
        pub fn matrix_mode(mode: u32) -> Result<()> {
            crate::generated::gfx::matrix_mode(mode)?;
            Ok(())
        }

        #[inline]
        pub fn memory_barrier(barriers: u32) -> Result<()> {
            crate::generated::gfx::memory_barrier(barriers)?;
            Ok(())
        }

        #[inline]
        pub fn models_vbo(value: u32) -> Result<u32> {
            let value = crate::generated::gfx::models_vbo(value)?;
            Ok(value)
        }

        #[inline]
        pub fn mult_matrix(values: &Vec<f32>) -> Result<()> {
            crate::generated::gfx::mult_matrix(values.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?)?;
            Ok(())
        }

        #[inline]
        pub fn multi_tex_coord(tex_num: i32, s: f32, t: f32, r: f32, q: f32, count: u32) -> Result<()> {
            crate::generated::gfx::multi_tex_coord(tex_num, s, t, r, q, count)?;
            Ok(())
        }

        #[inline]
        pub fn multi_tex_env(tex_num: i32, target: u32, pname: u32, values: &Vec<f32>, count: u32) -> Result<()> {
            crate::generated::gfx::multi_tex_env(tex_num, target, pname, values.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, count)?;
            Ok(())
        }

        #[inline]
        pub fn multi_tex_gen(tex_num: i32, target: u32, options: GfxMultiTexGenOptions, pname: u32, values: &Vec<f32>, count: u32) -> Result<()> {
            crate::generated::gfx::multi_tex_gen(tex_num, target, crate::generated::gfx::GfxMultiTexGenOptions { set_state: options.set_state, state: options.state }, pname, values.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, count)?;
            Ok(())
        }

        #[inline]
        pub fn normal(x: f32, y: f32, z: f32) -> Result<()> {
            crate::generated::gfx::normal(x, y, z)?;
            Ok(())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_object_label {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "object-label"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.object-label."]
        #[inline]
        pub unsafe fn object_label(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_object_label::call(p0, p1, p2) }
        }

        #[inline]
        pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near_val: f32, far_val: f32) -> Result<()> {
            crate::generated::gfx::ortho(left, right, bottom, top, near_val, far_val)?;
            Ok(())
        }

        #[inline]
        pub fn point_parameter(pname: u32, value: f32, values: &Vec<f32>, count: u32) -> Result<()> {
            crate::generated::gfx::point_parameter(pname, value, values.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, count)?;
            Ok(())
        }

        #[inline]
        pub fn point_size(value: f32) -> Result<()> {
            crate::generated::gfx::point_size(value)?;
            Ok(())
        }

        #[inline]
        pub fn point_sprite(value: bool) -> Result<()> {
            crate::generated::gfx::point_sprite(value)?;
            Ok(())
        }

        #[inline]
        pub fn polygon_mode(face: u32, mode: u32) -> Result<()> {
            crate::generated::gfx::polygon_mode(face, mode)?;
            Ok(())
        }

        #[inline]
        pub fn polygon_offset(factor: f32, units: f32) -> Result<()> {
            crate::generated::gfx::polygon_offset(factor, units)?;
            Ok(())
        }

        #[inline]
        pub fn pop_attrib(unused: u8) -> Result<()> {
            crate::generated::gfx::pop_attrib(unused)?;
            Ok(())
        }

        #[inline]
        pub fn pop_debug_group(unused: u8) -> Result<()> {
            crate::generated::gfx::pop_debug_group(unused)?;
            Ok(())
        }

        #[inline]
        pub fn pop_matrix(unused: u8) -> Result<()> {
            crate::generated::gfx::pop_matrix(unused)?;
            Ok(())
        }

        #[inline]
        pub fn push_attrib(value: u32) -> Result<()> {
            crate::generated::gfx::push_attrib(value)?;
            Ok(())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_push_debug_group {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "push-debug-group"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.push-debug-group."]
        #[inline]
        pub unsafe fn push_debug_group(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_push_debug_group::call(p0, p1, p2) }
        }

        #[inline]
        pub fn push_matrix(unused: u8) -> Result<()> {
            crate::generated::gfx::push_matrix(unused)?;
            Ok(())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_push_pop_matrix {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "push-pop-matrix"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.push-pop-matrix."]
        #[inline]
        pub unsafe fn push_pop_matrix(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_push_pop_matrix::call(p0, p1) }
        }

        #[inline]
        pub fn raw_bind_fbo(bind_default: bool, fbo_id: u32, target: u32, raw_fbo_id: u32) -> Result<RawBindFBOValue> {
            let value = crate::generated::gfx::raw_bind_fbo(bind_default, fbo_id, target, raw_fbo_id)?;
            Ok(RawBindFBOValue {
                previously_bound_raw_fbo_id: value.0,
                has_previous: value.1
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_read_pixels {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "read-pixels"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.read-pixels."]
        #[inline]
        pub unsafe fn read_pixels(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32) -> i32 {
            unsafe { __core_owned_read_pixels::call(p0, p1, p2, p3, p4, p5) }
        }

        #[inline]
        pub fn rect(x1: f32, y1: f32, x2: f32, y2: f32) -> Result<()> {
            crate::generated::gfx::rect(x1, y1, x2, y2)?;
            Ok(())
        }

        #[inline]
        pub fn remove_from_submission_vao(vao_id: u32, index: i32) -> Result<()> {
            crate::generated::gfx::remove_from_submission_vao(vao_id, index)?;
            Ok(())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_render_to_texture {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "render-to-texture"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.render-to-texture."]
        #[inline]
        pub unsafe fn render_to_texture(p0: i32, p1: i32, p2: i32, p3: i32) -> i32 {
            unsafe { __core_owned_render_to_texture::call(p0, p1, p2, p3) }
        }

        #[inline]
        pub fn reset_matrices(unused: u8) -> Result<()> {
            crate::generated::gfx::reset_matrices(unused)?;
            Ok(())
        }

        #[inline]
        pub fn reset_state(unused: u8) -> Result<()> {
            crate::generated::gfx::reset_state(unused)?;
            Ok(())
        }

        #[inline]
        pub fn rotate(degrees: f32, x: f32, y: f32, z: f32) -> Result<()> {
            crate::generated::gfx::rotate(degrees, x, y, z)?;
            Ok(())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_run_query {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "run-query"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.run-query."]
        #[inline]
        pub unsafe fn run_query(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_run_query::call(p0, p1, p2) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_save_image {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "save-image"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.save-image."]
        #[inline]
        pub unsafe fn save_image(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32) -> i64 {
            unsafe { __core_owned_save_image::call(p0, p1, p2, p3, p4, p5) }
        }

        #[inline]
        pub fn scale(x: f32, y: f32, z: f32) -> Result<()> {
            crate::generated::gfx::scale(x, y, z)?;
            Ok(())
        }

        #[inline]
        pub fn scissor(x: i32, y: i32, width: i32, height: i32) -> Result<()> {
            crate::generated::gfx::scissor(x, y, width, height)?;
            Ok(())
        }

        #[inline]
        pub fn secondary_color(x: f32, y: f32, z: f32) -> Result<()> {
            crate::generated::gfx::secondary_color(x, y, z)?;
            Ok(())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_set_fbo_attachment {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "set-fbo-attachment"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.set-fbo-attachment."]
        #[inline]
        pub unsafe fn set_fbo_attachment(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32) -> i32 {
            unsafe { __core_owned_set_fbo_attachment::call(p0, p1, p2, p3, p4, p5, p6) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_set_fbo_draw_buffers {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "set-fbo-draw-buffers"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.set-fbo-draw-buffers."]
        #[inline]
        pub unsafe fn set_fbo_draw_buffers(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_set_fbo_draw_buffers::call(p0, p1) }
        }

        #[inline]
        pub fn set_fbo_read_buffer(fbo_id: u32, buffer: u32) -> Result<()> {
            crate::generated::gfx::set_fbo_read_buffer(fbo_id, buffer)?;
            Ok(())
        }

        #[inline]
        pub fn set_feature_buffer_uniforms(object_id: i32, values: &Vec<f32>, offset: u32) -> Result<u32> {
            crate::generated::borrowed::gfx::set_feature_buffer_uniforms(object_id, values.as_slice(), offset)
        }

        #[inline]
        pub fn set_geometry_shader_parameter(shader_id: u32, param: u32, value: i32) -> Result<()> {
            crate::generated::gfx::set_geometry_shader_parameter(shader_id, param, value)?;
            Ok(())
        }

        #[inline]
        pub fn set_tesselation_shader_parameter(param: u32, value: i32, values: &Vec<f32>, value_count: u32, use_float_array: bool) -> Result<()> {
            crate::generated::gfx::set_tesselation_shader_parameter(param, value, values.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, value_count, use_float_array)?;
            Ok(())
        }

        #[inline]
        pub fn set_unit_buffer_uniforms(object_id: i32, values: &Vec<f32>, offset: u32) -> Result<u32> {
            crate::generated::borrowed::gfx::set_unit_buffer_uniforms(object_id, values.as_slice(), offset)
        }

        #[inline]
        pub fn shade_model(mode: u32) -> Result<()> {
            crate::generated::gfx::shade_model(mode)?;
            Ok(())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_shape {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "shape"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.shape."]
        #[inline]
        pub unsafe fn shape(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_shape::call(p0, p1) }
        }

        #[inline]
        pub fn slave_mini_map(value: bool) -> Result<()> {
            crate::generated::gfx::slave_mini_map(value)?;
            Ok(())
        }

        #[inline]
        pub fn stencil_func(func: u32, ref_: i32, mask: u32) -> Result<()> {
            crate::generated::gfx::stencil_func(func, ref_, mask)?;
            Ok(())
        }

        #[inline]
        pub fn stencil_func_separate(face: u32, func: u32, ref_: i32, mask: u32) -> Result<()> {
            crate::generated::gfx::stencil_func_separate(face, func, ref_, mask)?;
            Ok(())
        }

        #[inline]
        pub fn stencil_mask(mask: u32) -> Result<()> {
            crate::generated::gfx::stencil_mask(mask)?;
            Ok(())
        }

        #[inline]
        pub fn stencil_mask_separate(face: u32, mask: u32) -> Result<()> {
            crate::generated::gfx::stencil_mask_separate(face, mask)?;
            Ok(())
        }

        #[inline]
        pub fn stencil_op(fail: u32, zfail: u32, zpass: u32) -> Result<()> {
            crate::generated::gfx::stencil_op(fail, zfail, zpass)?;
            Ok(())
        }

        #[inline]
        pub fn stencil_op_separate(face: u32, fail: u32, zfail: u32, zpass: u32) -> Result<()> {
            crate::generated::gfx::stencil_op_separate(face, fail, zfail, zpass)?;
            Ok(())
        }

        #[inline]
        pub fn stencil_test(enable: bool) -> Result<()> {
            crate::generated::gfx::stencil_test(enable)?;
            Ok(())
        }

        #[inline]
        pub fn submit_vao(value: u32) -> Result<()> {
            crate::generated::gfx::submit_vao(value)?;
            Ok(())
        }

        #[inline]
        pub fn swap_buffers(unused: u8) -> Result<()> {
            crate::generated::gfx::swap_buffers(unused)?;
            Ok(())
        }

        #[inline]
        pub fn tex_coord(x: f32, y: f32, z: f32, w: f32, count: u32) -> Result<()> {
            crate::generated::gfx::tex_coord(x, y, z, w, count)?;
            Ok(())
        }

        #[inline]
        pub fn tex_env(target: u32, pname: u32, values: &Vec<f32>, count: u32) -> Result<()> {
            crate::generated::gfx::tex_env(target, pname, values.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, count)?;
            Ok(())
        }

        #[inline]
        pub fn tex_gen(target: u32, options: GfxTexGenOptions, pname: u32, values: &Vec<f32>, count: u32) -> Result<()> {
            crate::generated::gfx::tex_gen(target, crate::generated::gfx::GfxTexGenOptions { set_state: options.set_state, state: options.state }, pname, values.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, count)?;
            Ok(())
        }

        #[inline]
        pub fn tex_rect(x1: f32, y1: f32, x2: f32, y2: f32, s1: f32, t1: f32, s2: f32, t2: f32) -> Result<()> {
            crate::generated::gfx::tex_rect(x1, y1, x2, y2, s1, t1, s2, t2)?;
            Ok(())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_text {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "text"]
                pub fn call(p0: f32, p1: f32, p2: f32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.text."]
        #[inline]
        pub unsafe fn text(p0: f32, p1: f32, p2: f32, p3: i32) -> i32 {
            unsafe { __core_owned_text::call(p0, p1, p2, p3) }
        }

        #[inline]
        pub fn text_env(target: u32, pname: u32, values: &Vec<f32>, count: u32) -> Result<()> {
            crate::generated::gfx::text_env(target, pname, values.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, count)?;
            Ok(())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_texture_info {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "texture-info"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.texture-info."]
        #[inline]
        pub unsafe fn texture_info(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_texture_info::call(p0, p1) }
        }

        #[inline]
        pub fn translate(x: f32, y: f32, z: f32) -> Result<()> {
            crate::generated::gfx::translate(x, y, z)?;
            Ok(())
        }

        #[inline]
        pub fn unbind_buffer_range_vbo(vbo_id: u32, binding_index: u32, element_offset: i32, element_count: i32, target: u32, bind: bool) -> Result<i32> {
            let value = crate::generated::gfx::unbind_buffer_range_vbo(vbo_id, binding_index, element_offset, element_count, target, bind)?;
            Ok(value)
        }

        #[inline]
        pub fn uniform(location: i32, values: &Vec<f32>, count: u32) -> Result<()> {
            crate::generated::gfx::uniform(location, values.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, count)?;
            Ok(())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_uniform_array_float {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "uniform-array-float"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.uniform-array-float."]
        #[inline]
        pub unsafe fn uniform_array_float(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_uniform_array_float::call(p0, p1) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_uniform_array_int {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "uniform-array-int"]
                pub fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.uniform-array-int."]
        #[inline]
        pub unsafe fn uniform_array_int(p0: i32, p1: i32) -> i32 {
            unsafe { __core_owned_uniform_array_int::call(p0, p1) }
        }

        #[inline]
        pub fn uniform_int(location: i32, values: &Vec<i32>, count: u32) -> Result<()> {
            crate::generated::gfx::uniform_int(location, values.clone().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, count)?;
            Ok(())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_uniform_matrix {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "uniform-matrix"]
                pub fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.uniform-matrix."]
        #[inline]
        pub unsafe fn uniform_matrix(p0: i32, p1: i32, p2: i32) -> i32 {
            unsafe { __core_owned_uniform_matrix::call(p0, p1, p2) }
        }

        #[inline]
        pub fn uniform_subroutine(shader_type: u32, index: u32) -> Result<()> {
            crate::generated::gfx::uniform_subroutine(shader_type, index)?;
            Ok(())
        }

        #[inline]
        pub fn unit(unit_id: i32, options: GfxUnitDrawOptions) -> Result<()> {
            crate::generated::gfx::unit(unit_id, crate::generated::gfx::GfxUnitDrawOptions { apply_transform: options.apply_transform, do_raw_draw: options.do_raw_draw, no_lua_call: options.no_lua_call, full_model: options.full_model })?;
            Ok(())
        }

        #[inline]
        pub fn unit_mult_matrix(value: i32) -> Result<()> {
            crate::generated::gfx::unit_mult_matrix(value)?;
            Ok(())
        }

        #[inline]
        pub fn unit_piece(object_id: i32, piece_id: i32) -> Result<()> {
            crate::generated::gfx::unit_piece(object_id, piece_id)?;
            Ok(())
        }

        #[inline]
        pub fn unit_piece_matrix(object_id: i32, piece_id: i32) -> Result<()> {
            crate::generated::gfx::unit_piece_matrix(object_id, piece_id)?;
            Ok(())
        }

        #[inline]
        pub fn unit_piece_mult_matrix(object_id: i32, piece_id: i32) -> Result<()> {
            crate::generated::gfx::unit_piece_mult_matrix(object_id, piece_id)?;
            Ok(())
        }

        #[inline]
        pub fn unit_raw(unit_id: i32, options: GfxUnitDrawOptions) -> Result<()> {
            crate::generated::gfx::unit_raw(unit_id, crate::generated::gfx::GfxUnitDrawOptions { apply_transform: options.apply_transform, do_raw_draw: options.do_raw_draw, no_lua_call: options.no_lua_call, full_model: options.full_model })?;
            Ok(())
        }

        #[inline]
        pub fn unit_shape(def_id: i32, team_id: i32, options: GfxObjectShapeOptions) -> Result<()> {
            crate::generated::gfx::unit_shape(def_id, team_id, crate::generated::gfx::GfxObjectShapeOptions { raw_state: options.raw_state, to_screen: options.to_screen, opaque: options.opaque })?;
            Ok(())
        }

        #[inline]
        pub fn unit_shape_textures(object_id: i32, push: bool) -> Result<()> {
            crate::generated::gfx::unit_shape_textures(object_id, push)?;
            Ok(())
        }

        #[inline]
        pub fn unit_textures(object_id: i32, push: bool) -> Result<()> {
            crate::generated::gfx::unit_textures(object_id, push)?;
            Ok(())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_unsafe_state {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "unsafe-state"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.unsafe-state."]
        #[inline]
        pub unsafe fn unsafe_state(p0: i32, p1: i32, p2: i32, p3: i32) -> i32 {
            unsafe { __core_owned_unsafe_state::call(p0, p1, p2, p3) }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_upload_texture {
            #[link(wasm_import_module = "spring:gfx")]
            extern "C" {
                #[link_name = "upload-texture"]
                pub fn call(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32, p7: i32, p8: i32, p9: i32, p10: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.upload-texture."]
        #[inline]
        pub unsafe fn upload_texture(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32, p6: i32, p7: i32, p8: i32, p9: i32, p10: i32) -> i32 {
            unsafe { __core_owned_upload_texture::call(p0, p1, p2, p3, p4, p5, p6, p7, p8, p9, p10) }
        }

        #[inline]
        pub fn upload_vbo(vbo_id: u32, data: &Vec<f32>, attribute_index: i32, element_offset: i32, data_start_index: i32, data_finish_index: i32) -> Result<u32> {
            crate::generated::borrowed::gfx::upload_vbo(vbo_id, data.as_slice(), attribute_index, element_offset, data_start_index, data_finish_index)
        }

        #[inline]
        pub fn use_shader(shader_id: u32) -> Result<bool> {
            let value = crate::generated::gfx::use_shader(shader_id)?;
            Ok(value)
        }

        #[inline]
        pub fn vertex(x: f32, y: f32, z: f32, w: f32, count: u32) -> Result<()> {
            crate::generated::gfx::vertex(x, y, z, w, count)?;
            Ok(())
        }

        #[inline]
        pub fn viewport(x: i32, y: i32, width: i32, height: i32) -> Result<()> {
            crate::generated::gfx::viewport(x, y, width, height)?;
            Ok(())
        }

    }

