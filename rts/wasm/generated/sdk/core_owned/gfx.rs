    pub mod gfx {
        use super::{Result, String, Vec};

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum GfxCullFace {
            GfxCullFaceBack,
            GfxCullFaceFront,
            GfxCullFaceFrontAndBack,
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxBlendEquationQuery {
            pub mode: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxBlendEquationSeparateQuery {
            pub mode_rgb: u32,
            pub mode_alpha: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxBlendFuncQuery {
            pub src: u32,
            pub dst: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxBlendFuncSeparateQuery {
            pub src_rgb: u32,
            pub dst_rgb: u32,
            pub src_alpha: u32,
            pub dst_alpha: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxBoolQuery {
            pub value: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxClipDistanceQuery {
            pub index: u32,
            pub enable: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxClipPlaneQuery {
            pub plane: u32,
            pub equation: Vec<f32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxColorMaskOptions {
            pub red: bool,
            pub green: bool,
            pub blue: bool,
            pub alpha: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxColorMaskQuery {
            pub options: GfxColorMaskOptions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxCreateShaderResult {
            pub shader_id: u32,
            pub gl_program_id: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxCreateTextureAtlasQuery {
            pub xsize: i32,
            pub ysize: i32,
            pub alloc_type: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxCreateTextureQuery {
            pub xsize: i32,
            pub ysize: i32,
            pub zsize: i32,
            pub params: GfxTextureParams,
        }

        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct GfxCullFaceQuery {
            pub face: GfxCullFace,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxDepthTestOptions {
            pub enable: bool,
            pub set_func: bool,
            pub func: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxDepthTestQuery {
            pub options: GfxDepthTestOptions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxDrawListAtUnitQuery {
            pub unit_id: i32,
            pub list_id: u32,
            pub use_mid_pos: bool,
            pub scale: Float3,
            pub degrees: f32,
            pub rot: Float3,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxEmptyQuery {
            pub unused: u8,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxEmptyResult {
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxEngineModelUniformDataSizeResult {
            pub size_in_elements: u32,
            pub size_in_bytes_on_cpu: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxEngineTextureNamesResult {
            pub names: Vec<String>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxFBOQuery {
            pub fbo_id: u32,
            pub target: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxFBOReadBufferQuery {
            pub fbo_id: u32,
            pub buffer: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxFBOResult {
            pub fbo_id: u32,
            pub raw_id: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxFBOStatusResult {
            pub valid: bool,
            pub status: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxFeatureDrawOptions {
            pub apply_transform: bool,
            pub do_raw_draw: bool,
            pub no_lua_call: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxFloatQuery {
            pub value: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxFloatResult {
            pub value: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxFontAutoOutlineColorQuery {
            pub font_id: u32,
            pub enable: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxFontBeginQuery {
            pub font_id: u32,
            pub user_defined_blending: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxFontQuery {
            pub font_id: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxFontResult {
            pub font_id: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxFontSubmitBufferedOptions {
            pub no_billboarding: bool,
            pub user_defined_blending: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxFrustumQuery {
            pub left: f32,
            pub right: f32,
            pub bottom: f32,
            pub top: f32,
            pub near_val: f32,
            pub far_val: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxGeometryShaderParameterQuery {
            pub shader_id: u32,
            pub param: u32,
            pub value: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxGetMatrixDataQuery {
            pub mode: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxGetMatrixDataResult {
            pub values: Vec<f32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxGetNumberQuery {
            pub pname: u32,
            pub max_values: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxGetNumberResult {
            pub values: Vec<f32>,
            pub count: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxGetStringQuery {
            pub pname: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxGroundCircleQuery {
            pub pos: Float3,
            pub radius: f32,
            pub resolution: i32,
            pub ballistic: bool,
            pub slope: f32,
            pub gravity: f32,
            pub weapon_def_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxIntQuery {
            pub value: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxIntResult {
            pub value: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxMatrixModeQuery {
            pub mode: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxMatrixQuery {
            pub values: Vec<f32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxMemoryBarrierQuery {
            pub barriers: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxMiniMapConfigQuery {
            pub px: i32,
            pub py: i32,
            pub sx: i32,
            pub sy: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxObjectBufferUniformsResult {
            pub count: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxObjectLabelQuery {
            pub identifier: u32,
            pub object_id: u32,
            pub label: String,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxObjectPieceQuery {
            pub object_id: i32,
            pub piece_id: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxObjectShapeOptions {
            pub raw_state: bool,
            pub to_screen: bool,
            pub opaque: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxObjectShapeQuery {
            pub def_id: i32,
            pub team_id: i32,
            pub options: GfxObjectShapeOptions,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxObjectTextureStateQuery {
            pub object_id: i32,
            pub push: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxPolygonModeQuery {
            pub face: u32,
            pub mode: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxRBOCreateQuery {
            pub xsize: i32,
            pub ysize: i32,
            pub target: u32,
            pub format: u32,
            pub samples: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxRBOInfoQuery {
            pub rbo_id: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxRBOInfoResult {
            pub valid: bool,
            pub target: u32,
            pub format: u32,
            pub xsize: i32,
            pub ysize: i32,
            pub samples: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxRawBindFBOQuery {
            pub bind_default: bool,
            pub fbo_id: u32,
            pub target: u32,
            pub raw_fbo_id: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxRawBindFBOResult {
            pub previously_bound_raw_fbo_id: u32,
            pub has_previous: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxScaleQuery {
            pub x: f32,
            pub y: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxScissorQuery {
            pub x: i32,
            pub y: i32,
            pub width: i32,
            pub height: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxShadeModelQuery {
            pub mode: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxShaderQuery {
            pub shader_id: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxShadowMapParamsResult {
            pub params: Float4,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxShapeQuery {
            pub primitive: u32,
            pub vertices: Vec<GfxVertexData>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxStencilFuncQuery {
            pub func: u32,
            pub ref_: i32,
            pub mask: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxStencilFuncSeparateQuery {
            pub face: u32,
            pub func: u32,
            pub ref_: i32,
            pub mask: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxStencilMaskQuery {
            pub mask: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxStencilMaskSeparateQuery {
            pub face: u32,
            pub mask: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxStencilOpQuery {
            pub fail: u32,
            pub zfail: u32,
            pub zpass: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxStencilOpSeparateQuery {
            pub face: u32,
            pub fail: u32,
            pub zfail: u32,
            pub zpass: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxTranslateQuery {
            pub x: f32,
            pub y: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxTranslateResult {
            pub x: f32,
            pub y: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxUIntQuery {
            pub value: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxUniformLocationResult {
            pub location: i32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxUniformMatrixQuery {
            pub location: i32,
            pub values: Vec<f32>,
            pub transpose: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxUniformSubroutineQuery {
            pub shader_type: u32,
            pub index: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxUnitDrawOptions {
            pub apply_transform: bool,
            pub do_raw_draw: bool,
            pub no_lua_call: bool,
            pub full_model: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxUseShaderResult {
            pub linked: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxVAOBufferQuery {
            pub vao_id: u32,
            pub vbo_id: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxVAODrawArraysQuery {
            pub vao_id: u32,
            pub mode: u32,
            pub vertex_count: i32,
            pub vertex_first: i32,
            pub instance_count: i32,
            pub instance_first: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxVAODrawElementsQuery {
            pub vao_id: u32,
            pub mode: u32,
            pub draw_count: i32,
            pub base_index: i32,
            pub instance_count: i32,
            pub base_vertex: i32,
            pub base_instance: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxVAORemoveSubmissionQuery {
            pub vao_id: u32,
            pub index: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxVAOResult {
            pub vao_id: u32,
            pub raw_id: u32,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct GfxVAOSubmissionQuery {
            pub vao_id: u32,
            pub ids: Vec<u32>,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxVBOAttributeOptions {
            pub id: i32,
            pub type_: u32,
            pub size: i32,
            pub normalized: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxVBOBindRangeQuery {
            pub vbo_id: u32,
            pub binding_index: u32,
            pub element_offset: i32,
            pub element_count: i32,
            pub target: u32,
            pub bind: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxVBOInfoQuery {
            pub vbo_id: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxVBOQuery {
            pub target: u32,
            pub freq_updated: bool,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxVertexQuery {
            pub x: f32,
            pub y: f32,
            pub z: f32,
            pub w: f32,
            pub count: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxViewRangeQuery {
            pub camera_type: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxViewRangeResult {
            pub near_plane_dist: f32,
            pub far_plane_dist: f32,
            pub min_view_range: f32,
            pub max_view_range: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxViewSizesResult {
            pub view_size_x: i32,
            pub view_size_y: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GfxViewportQuery {
            pub x: i32,
            pub y: i32,
            pub width: i32,
            pub height: i32,
        }

        pub use super::types::{AtmosphereParams, BoolResult, CollisionVolumeData, CommonErrorCode, DefRef, Error, Float2, Float2Result, Float3, Float3Array, Float3Result, Float4, Float4Result, FloatArray, FloatResult, Int2, Int3, Int32Array, Int32Result, MapRenderingParams, NativeExplosionParams, NativeProjectileParams, NumberOrBool, ProjectileTargetRef, ResourcePack, RgbColor, SoundEffectParams, StringArray, StringResult, SunLightingParams, UInt32Array, UInt32Result, UnitCostOverrides, UnitHealthValue, UnitTargetRef, WaterParams};

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_download_vbo {
            #[link(wasm_import_module = "spring:gfx")]
            unsafe extern "C" {
                #[link_name = "download-vbo"]
                pub fn call(pvbo_id: i32, pattribute_index: i32, pelement_offset: i32, pelement_count: i32, pforce_gpu_read: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_engine_model_uniform_data_def {
            #[link(wasm_import_module = "spring:gfx")]
            unsafe extern "C" {
                #[link_name = "get-engine-model-uniform-data-def"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_engine_uniform_buffer_def {
            #[link(wasm_import_module = "spring:gfx")]
            unsafe extern "C" {
                #[link_name = "get-engine-uniform-buffer-def"]
                pub fn call(pindex: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_shader_log {
            #[link(wasm_import_module = "spring:gfx")]
            unsafe extern "C" {
                #[link_name = "get-shader-log"]
                pub fn call(punused: i32, output: i32) -> i32;
            }
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_variable_output_get_string {
            #[link(wasm_import_module = "spring:gfx")]
            unsafe extern "C" {
                #[link_name = "get-string"]
                pub fn call(ppname: i32, output: i32) -> i32;
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct CreateFBOValue {
            pub fbo_id: u32,
            pub raw_id: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct CreateShaderValue {
            pub shader_id: u32,
            pub gl_program_id: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetRBOInfoValue {
            pub valid: bool,
            pub target: u32,
            pub format: u32,
            pub xsize: i32,
            pub ysize: i32,
            pub samples: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetScreenViewTransValue {
            pub x: f32,
            pub y: f32,
            pub z: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetTextHeightValue {
            pub height: f32,
            pub descender: f32,
            pub lines: i32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetVAOValue {
            pub vao_id: u32,
            pub raw_id: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetVBOValue {
            pub vbo_id: u32,
            pub raw_id: u32,
            pub target: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetVBOInfoValue {
            pub elements_count: u32,
            pub buffer_size_in_bytes: u32,
            pub gpu_buffer_size_in_bytes: u32,
            pub elem_size_in_bytes: u32,
            pub attributes_count: u32,
            pub primitive_restart_index: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct GetViewRangeValue {
            pub near_plane_dist: f32,
            pub far_plane_dist: f32,
            pub min_view_range: f32,
            pub max_view_range: f32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct IsValidFBOValue {
            pub valid: bool,
            pub status: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
        pub struct RawBindFBOValue {
            pub previously_bound_raw_fbo_id: u32,
            pub has_previous: bool,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ReadPixelsValue {
            pub values: Vec<f32>,
            pub components: u32,
        }

        #[derive(Debug, Clone, Copy, PartialEq, Default)]
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
            unsafe extern "C" {
                #[link_name = "active-fbo"]
                pub safe fn call(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.active-fbo."]
        #[doc(hidden)]
        #[inline]
        pub fn active_fbo(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32) -> i32 {
            __core_owned_active_fbo::call(p0, p1, p2, p3, p4)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_active_shader {
            #[link(wasm_import_module = "spring:gfx")]
            unsafe extern "C" {
                #[link_name = "active-shader"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.active-shader."]
        #[doc(hidden)]
        #[inline]
        pub fn active_shader(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_active_shader::call(p0, p1, p2)
        }

        #[inline]
        pub fn active_texture(tex_num: i32) -> Result<()> {
            crate::generated::gfx::active_texture(tex_num)?;
            Ok(())
        }

        #[inline]
        pub fn add_atlas_texture(atlas_name: &str, texture_name: &str) -> Result<()> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(atlas_name, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(atlas_name)?),
            };
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(texture_name, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(texture_name)?),
            };
            crate::generated::borrowed::gfx::add_atlas_texture(__core_string_0_buf.as_cstr(), __core_string_1_buf.as_cstr())
        }

        #[inline]
        pub fn add_fallback_font(value: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(value, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(value)?),
            };
            crate::generated::borrowed::gfx::add_fallback_font(__core_string_0_buf.as_cstr())
        }

        #[inline]
        pub fn add_feature_defs_to_submission_vao(vao_id: u32, ids: &[u32]) -> Result<u32> {
            crate::generated::borrowed::gfx::add_feature_defs_to_submission_vao(vao_id, ids)
        }

        #[inline]
        pub fn add_features_to_submission_vao(vao_id: u32, ids: &[u32]) -> Result<u32> {
            crate::generated::borrowed::gfx::add_features_to_submission_vao(vao_id, ids)
        }

        #[inline]
        pub fn add_unit_defs_to_submission_vao(vao_id: u32, ids: &[u32]) -> Result<u32> {
            crate::generated::borrowed::gfx::add_unit_defs_to_submission_vao(vao_id, ids)
        }

        #[inline]
        pub fn add_units_to_submission_vao(vao_id: u32, ids: &[u32]) -> Result<u32> {
            crate::generated::borrowed::gfx::add_units_to_submission_vao(vao_id, ids)
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
            unsafe extern "C" {
                #[link_name = "begin-end"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.begin-end."]
        #[doc(hidden)]
        #[inline]
        pub fn begin_end(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_begin_end::call(p0, p1, p2)
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

        #[inline]
        pub fn bind_image_texture(unit: u32, name: &str, level: i32, layer: i32, layered: bool, access: u32, format: u32) -> Result<()> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(name, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(name)?),
            };
            crate::generated::borrowed::gfx::bind_image_texture(unit, __core_string_1_buf.as_cstr(), level, layer, layered, access, format)
        }

        #[inline]
        pub fn bind_texture(name: &str, tex_num: i32, enable: bool) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(name, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(name)?),
            };
            crate::generated::borrowed::gfx::bind_texture(__core_string_0_buf.as_cstr(), tex_num, enable)
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
        #[expect(clippy::too_many_arguments, reason = "Core function preserves the corresponding Lua API arity")]
        pub fn blit_fbo(src_fboid: u32, dst_fboid: u32, x0_src: i32, y0_src: i32, x1_src: i32, y1_src: i32, x0_dst: i32, y0_dst: i32, x1_dst: i32, y1_dst: i32, mask: u32, filter: u32) -> Result<()> {
            crate::generated::gfx::blit_fbo(src_fboid, dst_fboid, x0_src, y0_src, x1_src, y1_src, x0_dst, y0_dst, x1_dst, y1_dst, mask, filter)?;
            Ok(())
        }

        #[inline]
        pub fn call_list(value: u32) -> Result<()> {
            crate::generated::gfx::call_list(value)?;
            Ok(())
        }

        #[inline]
        pub fn change_texture_params(name: &str, params: GfxTextureParams) -> Result<()> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + name.len()); __b.extend_from_slice(&(name.len() as u32).to_le_bytes()); __b.extend_from_slice(name.as_bytes()); __b };
            let __blob1 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&params.target.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&params.format.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&params.border.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&params.min_filter.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&params.mag_filter.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&params.wrap_s.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&params.wrap_t.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&params.wrap_r.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&params.compare_func.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&params.lod_bias.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&params.aniso.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&params.samples.to_le_bytes()); __b.extend_from_slice(&(if params.fbo { 1u32 } else { 0u32 }).to_le_bytes()); __b.extend_from_slice(&(if params.fbo_depth { 1u32 } else { 0u32 }).to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            crate::generated::dynamic_input::gfx::change_texture_params(&__blob0, &__blob1)
        }

        #[inline]
        pub fn clear(bits: u32, values: &[f32], count: u32) -> Result<()> {
            crate::generated::gfx::clear(bits, values.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, count)?;
            Ok(())
        }

        #[inline]
        pub fn clear_attachment_fbo(target: u32, attachment: u32, values: &[f32], count: u32) -> Result<bool> {
            let value = crate::generated::gfx::clear_attachment_fbo(target, attachment, values.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, count)?;
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
        pub fn clip_plane(plane: u32, equation: &[f32]) -> Result<()> {
            crate::generated::gfx::clip_plane(plane, equation.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?)?;
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

        #[inline]
        #[expect(clippy::too_many_arguments, reason = "Core function preserves the corresponding Lua API arity")]
        pub fn copy_to_texture(name: &str, xoff: i32, yoff: i32, x: i32, y: i32, width: i32, height: i32, target: u32, level: u32) -> Result<()> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(name, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(name)?),
            };
            crate::generated::borrowed::gfx::copy_to_texture(__core_string_0_buf.as_cstr(), xoff, yoff, x, y, width, height, target, level)
        }

        #[inline]
        pub fn copy_to_vbo(source_vboid: u32, destination_vboid: u32, copy_size_in_bytes: i32) -> Result<bool> {
            let value = crate::generated::gfx::copy_to_vbo(source_vboid, destination_vboid, copy_size_in_bytes)?;
            Ok(value)
        }

        #[inline]
        pub fn create_fbo(target: u32, attachments: &[GfxFBOAttachment], draw_buffers: &[u32], read_buffer: u32) -> Result<CreateFBOValue> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(attachments.len() as u32).to_le_bytes()); for __item in attachments.iter() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.attachment.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&(__item.texture_name.len() as u32).to_le_bytes()); __b.extend_from_slice(__item.texture_name.as_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.texture_target.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.mip_level.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.rbo_id.to_le_bytes()); __b.extend_from_slice(&(if __item.use_rbo { 1u32 } else { 0u32 }).to_le_bytes());} __b };
            let __blob1 = { let mut __b = Vec::new(); __b.extend_from_slice(&(draw_buffers.len() as u32).to_le_bytes()); for __item in draw_buffers.iter().copied() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.to_le_bytes());} __b };
            let mut __output = [0u8; 8];
            crate::generated::dynamic_input::gfx::create_fbo(target as i32, read_buffer as i32, &__blob0, &__blob1, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(CreateFBOValue {
                fbo_id: crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                raw_id: crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_create_list {
            #[link(wasm_import_module = "spring:gfx")]
            unsafe extern "C" {
                #[link_name = "create-list"]
                pub safe fn call(p0: i32, p1: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.create-list."]
        #[doc(hidden)]
        #[inline]
        pub fn create_list(p0: i32, p1: i32) -> i64 {
            __core_owned_create_list::call(p0, p1)
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

        #[inline]
        #[expect(clippy::too_many_arguments, reason = "Core function preserves the corresponding Lua API arity")]
        pub fn create_shader(definitions: &str, vertex: &str, tcs: &str, tes: &str, geometry: &str, fragment: &str, compute: &str, options: GfxCreateShaderOptions) -> Result<CreateShaderValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + definitions.len()); __b.extend_from_slice(&(definitions.len() as u32).to_le_bytes()); __b.extend_from_slice(definitions.as_bytes()); __b };
            let __blob1 = { let mut __b = Vec::with_capacity(4 + vertex.len()); __b.extend_from_slice(&(vertex.len() as u32).to_le_bytes()); __b.extend_from_slice(vertex.as_bytes()); __b };
            let __blob2 = { let mut __b = Vec::with_capacity(4 + tcs.len()); __b.extend_from_slice(&(tcs.len() as u32).to_le_bytes()); __b.extend_from_slice(tcs.as_bytes()); __b };
            let __blob3 = { let mut __b = Vec::with_capacity(4 + tes.len()); __b.extend_from_slice(&(tes.len() as u32).to_le_bytes()); __b.extend_from_slice(tes.as_bytes()); __b };
            let __blob4 = { let mut __b = Vec::with_capacity(4 + geometry.len()); __b.extend_from_slice(&(geometry.len() as u32).to_le_bytes()); __b.extend_from_slice(geometry.as_bytes()); __b };
            let __blob5 = { let mut __b = Vec::with_capacity(4 + fragment.len()); __b.extend_from_slice(&(fragment.len() as u32).to_le_bytes()); __b.extend_from_slice(fragment.as_bytes()); __b };
            let __blob6 = { let mut __b = Vec::with_capacity(4 + compute.len()); __b.extend_from_slice(&(compute.len() as u32).to_le_bytes()); __b.extend_from_slice(compute.as_bytes()); __b };
            let __blob7 = { let mut __b = Vec::new(); __b.extend_from_slice(&(if options.has_geo_input_type { 1u32 } else { 0u32 }).to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&options.geo_input_type.to_le_bytes()); __b.extend_from_slice(&(if options.has_geo_output_type { 1u32 } else { 0u32 }).to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&options.geo_output_type.to_le_bytes()); __b.extend_from_slice(&(if options.has_geo_output_verts { 1u32 } else { 0u32 }).to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&options.geo_output_verts.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            let mut __output = [0u8; 8];
            crate::generated::dynamic_input::gfx::create_shader(&__blob0, &__blob1, &__blob2, &__blob3, &__blob4, &__blob5, &__blob6, &__blob7, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(CreateShaderValue {
                shader_id: crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                gl_program_id: crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_create_texture {
            #[link(wasm_import_module = "spring:gfx")]
            unsafe extern "C" {
                #[link_name = "create-texture"]
                pub safe fn call(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.create-texture."]
        #[doc(hidden)]
        #[inline]
        pub fn create_texture(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32) -> i64 {
            __core_owned_create_texture::call(p0, p1, p2, p3, p4, p5)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_create_texture_atlas {
            #[link(wasm_import_module = "spring:gfx")]
            unsafe extern "C" {
                #[link_name = "create-texture-atlas"]
                pub safe fn call(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32) -> i64;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.create-texture-atlas."]
        #[doc(hidden)]
        #[inline]
        pub fn create_texture_atlas(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32) -> i64 {
            __core_owned_create_texture_atlas::call(p0, p1, p2, p3, p4)
        }

        #[inline]
        pub fn cull_face(face: GfxCullFace) -> Result<()> {
            crate::generated::gfx::cull_face(match face { GfxCullFace::GfxCullFaceBack => 1029i32, GfxCullFace::GfxCullFaceFront => 1028i32, GfxCullFace::GfxCullFaceFrontAndBack => 1032i32 })?;
            Ok(())
        }

        #[inline]
        pub fn culling(value: bool) -> Result<()> {
            crate::generated::gfx::culling(value)?;
            Ok(())
        }

        #[inline]
        pub fn define_vbo(vbo_id: u32, elements_count: i32, element_array: bool, index_type: u32, use_default_attributes: bool, default_attribute_count: u32, attributes: &[GfxVBOAttributeOptions]) -> Result<()> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(attributes.len() as u32).to_le_bytes()); for __item in attributes.iter() { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.id.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.type_.to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.size.to_le_bytes()); __b.extend_from_slice(&(if __item.normalized { 1u32 } else { 0u32 }).to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); }} __b };
            crate::generated::dynamic_input::gfx::define_vbo(vbo_id as i32, elements_count, element_array as i32, index_type as i32, use_default_attributes as i32, default_attribute_count as i32, &__blob0)
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
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(name, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(name)?),
            };
            crate::generated::borrowed::gfx::delete_texture(__core_string_0_buf.as_cstr())
        }

        #[inline]
        pub fn delete_texture_atlas(name: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(name, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(name)?),
            };
            crate::generated::borrowed::gfx::delete_texture_atlas(__core_string_0_buf.as_cstr())
        }

        #[inline]
        pub fn delete_texture_fbo(name: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(name, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(name)?),
            };
            crate::generated::borrowed::gfx::delete_texture_fbo(__core_string_0_buf.as_cstr())
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
                    let status = unsafe { __core_variable_output_download_vbo::call(vbo_id as i32, attribute_index, element_offset, element_count, u32::from(force_gpu_read) as i32, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (vbo_id as i32, attribute_index, element_offset, element_count, u32::from(force_gpu_read) as i32);
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
            unsafe extern "C" {
                #[link_name = "draw-func-at-unit"]
                pub safe fn call(p0: i32, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.draw-func-at-unit."]
        #[doc(hidden)]
        #[inline]
        pub fn draw_func_at_unit(p0: i32, p1: i32, p2: i32, p3: i32) -> i32 {
            __core_owned_draw_func_at_unit::call(p0, p1, p2, p3)
        }

        #[inline]
        pub fn draw_ground_circle(pos: Float3, radius: f32, resolution: i32, ballistic: bool, slope: f32, gravity: f32, weapon_def_id: i32) -> Result<()> {
            crate::generated::gfx::draw_ground_circle(crate::generated::gfx::Float3 { x: pos.x, y: pos.y, z: pos.z }, radius, resolution, ballistic, slope, gravity, weapon_def_id)?;
            Ok(())
        }

        #[inline]
        #[expect(clippy::too_many_arguments, reason = "Core function preserves the corresponding Lua API arity")]
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
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(name, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(name)?),
            };
            crate::generated::borrowed::gfx::finalize_texture_atlas(__core_string_0_buf.as_cstr())
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

        #[inline]
        pub fn font_get_text_height(font_id: u32, text: &str, x: f32, y: f32, size: f32, options: &str) -> Result<FontGetTextHeightValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + text.len()); __b.extend_from_slice(&(text.len() as u32).to_le_bytes()); __b.extend_from_slice(text.as_bytes()); __b };
            let __blob1 = { let mut __b = Vec::with_capacity(4 + options.len()); __b.extend_from_slice(&(options.len() as u32).to_le_bytes()); __b.extend_from_slice(options.as_bytes()); __b };
            let mut __output = [0u8; 12];
            crate::generated::dynamic_input::gfx::font_get_text_height(font_id as i32, x, y, size, &__blob0, &__blob1, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(FontGetTextHeightValue {
                height: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                descender: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                lines: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[inline]
        pub fn font_get_text_width(font_id: u32, text: &str, x: f32, y: f32, size: f32, options: &str) -> Result<f32> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(text, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(text)?),
            };
            let mut __core_string_5_scratch = [0u8; 256];
            let __core_string_5_buf = match super::write_cstr(options, &mut __core_string_5_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(options)?),
            };
            crate::generated::borrowed::gfx::font_get_text_width(font_id, __core_string_1_buf.as_cstr(), x, y, size, __core_string_5_buf.as_cstr())
        }

        #[inline]
        pub fn font_print(font_id: u32, text: &str, x: f32, y: f32, size: f32, options: &str) -> Result<()> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(text, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(text)?),
            };
            let mut __core_string_5_scratch = [0u8; 256];
            let __core_string_5_buf = match super::write_cstr(options, &mut __core_string_5_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(options)?),
            };
            crate::generated::borrowed::gfx::font_print(font_id, __core_string_1_buf.as_cstr(), x, y, size, __core_string_5_buf.as_cstr())
        }

        #[inline]
        pub fn font_print_world(font_id: u32, text: &str, pos: Float3, size: f32, options: &str) -> Result<()> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + text.len()); __b.extend_from_slice(&(text.len() as u32).to_le_bytes()); __b.extend_from_slice(text.as_bytes()); __b };
            let __blob1 = { let mut __b = Vec::new(); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&pos.x.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&pos.y.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&pos.z.to_bits().to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            let __blob2 = { let mut __b = Vec::with_capacity(4 + options.len()); __b.extend_from_slice(&(options.len() as u32).to_le_bytes()); __b.extend_from_slice(options.as_bytes()); __b };
            crate::generated::dynamic_input::gfx::font_print_world(font_id as i32, size, &__blob0, &__blob1, &__blob2)
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
            unsafe extern "C" {
                #[link_name = "font-wrap-text"]
                pub safe fn call(p0: i32, p1: f32, p2: f32, p3: f32, p4: i32, p5: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.font-wrap-text."]
        #[doc(hidden)]
        #[inline]
        pub fn font_wrap_text(p0: i32, p1: f32, p2: f32, p3: f32, p4: i32, p5: i32) -> i32 {
            __core_owned_font_wrap_text::call(p0, p1, p2, p3, p4, p5)
        }

        #[inline]
        pub fn frustum(left: f32, right: f32, bottom: f32, top: f32, near_val: f32, far_val: f32) -> Result<()> {
            crate::generated::gfx::frustum(left, right, bottom, top, near_val, far_val)?;
            Ok(())
        }

        #[inline]
        pub fn generate_mipmap(name: &str) -> Result<()> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(name, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(name)?),
            };
            crate::generated::borrowed::gfx::generate_mipmap(__core_string_0_buf.as_cstr())
        }

        #[inline]
        pub fn get_active_uniforms(shader_id: u32) -> Result<Vec<GfxActiveUniformEntry>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::gfx::get_active_uniforms(shader_id as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(GfxActiveUniformEntry { name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, type_: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, gl_type: crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, length: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, size: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, location: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? }); } __items };
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
        pub fn get_atlas_texture(atlas_name: &str, texture_name: &str) -> Result<GetAtlasTextureValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + atlas_name.len()); __b.extend_from_slice(&(atlas_name.len() as u32).to_le_bytes()); __b.extend_from_slice(atlas_name.as_bytes()); __b };
            let __blob1 = { let mut __b = Vec::with_capacity(4 + texture_name.len()); __b.extend_from_slice(&(texture_name.len() as u32).to_le_bytes()); __b.extend_from_slice(texture_name.as_bytes()); __b };
            let mut __output = [0u8; 20];
            crate::generated::dynamic_input::gfx::get_atlas_texture(&__blob0, &__blob1, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(GetAtlasTextureValue {
                x1: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                x2: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                y1: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                y2: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                page_num: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_atmosphere {
            #[link(wasm_import_module = "spring:gfx")]
            unsafe extern "C" {
                #[link_name = "get-atmosphere"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.get-atmosphere."]
        #[doc(hidden)]
        #[inline]
        pub fn get_atmosphere(p0: i32, p1: i32) -> i32 {
            __core_owned_get_atmosphere::call(p0, p1)
        }

        #[inline]
        pub fn get_console_commands(unused: u8) -> Result<Vec<GfxConsoleCommandEntry>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::gfx::get_console_commands(unused as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(GfxConsoleCommandEntry { command: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, description: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, synced: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, cheat: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? }); } __items };
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
        mod __core_owned_get_engine_atlas_textures {
            #[link(wasm_import_module = "spring:gfx")]
            unsafe extern "C" {
                #[link_name = "get-engine-atlas-textures"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.get-engine-atlas-textures."]
        #[doc(hidden)]
        #[inline]
        pub fn get_engine_atlas_textures(p0: i32, p1: i32) -> i32 {
            __core_owned_get_engine_atlas_textures::call(p0, p1)
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

        #[inline]
        pub fn get_engine_texture_names(unused: u8) -> Result<Vec<String>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::gfx::get_engine_texture_names(unused as i32, &mut __output) {
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
        pub fn get_engine_uniform_buffer_def(index: i32) -> Result<String> {
            #[cfg(target_arch = "wasm32")]
            {
                let mut descriptor = [0u32; 3];
                let mut output = Vec::<u8>::new();
                loop {
                    let status = unsafe { __core_variable_output_get_engine_uniform_buffer_def::call(index, descriptor.as_mut_ptr() as usize as u32 as i32) };
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
                let _ = (index);
                Err(unreachable!())
            }
        }

        #[inline]
        pub fn get_fixed_state(param: &str) -> Result<GetFixedStateValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + param.len()); __b.extend_from_slice(&(param.len() as u32).to_le_bytes()); __b.extend_from_slice(param.as_bytes()); __b };
            let mut __output = [0u8; 172];
            crate::generated::dynamic_input::gfx::get_fixed_state(&__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(GetFixedStateValue {
                bools: { let mut __arr = Vec::with_capacity(8); for _ in 0..8usize { __arr.push(crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __arr },
                bool_count: crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                ints: { let mut __arr = Vec::with_capacity(16); for _ in 0..16usize { __arr.push(crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __arr },
                int_count: crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                floats: { let mut __arr = Vec::with_capacity(16); for _ in 0..16usize { __arr.push(crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?); } __arr },
                float_count: crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_font_info {
            #[link(wasm_import_module = "spring:gfx")]
            unsafe extern "C" {
                #[link_name = "get-font-info"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.get-font-info."]
        #[doc(hidden)]
        #[inline]
        pub fn get_font_info(p0: i32, p1: i32) -> i32 {
            __core_owned_get_font_info::call(p0, p1)
        }

        #[inline]
        pub fn get_global_tex_coords(value: &str) -> Result<GetGlobalTexCoordsValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + value.len()); __b.extend_from_slice(&(value.len() as u32).to_le_bytes()); __b.extend_from_slice(value.as_bytes()); __b };
            let mut __output = [0u8; 20];
            crate::generated::dynamic_input::gfx::get_global_tex_coords(&__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(GetGlobalTexCoordsValue {
                x1: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                x2: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                y1: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                y2: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                page_num: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[inline]
        pub fn get_global_tex_names(unused: u8) -> Result<Vec<GfxAtlasTextureEntry>> {
            let mut __output = Vec::<u8>::new();
            loop {
                match crate::generated::dynamic_output::gfx::get_global_tex_names(unused as i32, &mut __output) {
                    Ok(required) => {
                        __output.truncate(required);
                        let mut __cursor = 0usize;
                        let __result = { let __count = crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? as usize; let mut __items = Vec::with_capacity(__count); for _ in 0..__count { __items.push(GfxAtlasTextureEntry { name: crate::generated::__core_wire::string(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, x1: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, x2: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, y1: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, y2: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?, page_num: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))? }); } __items };
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
        pub fn get_idvbo(value: u32) -> Result<u32> {
            let value = crate::generated::gfx::get_idvbo(value)?;
            Ok(value)
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_map_rendering {
            #[link(wasm_import_module = "spring:gfx")]
            unsafe extern "C" {
                #[link_name = "get-map-rendering"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.get-map-rendering."]
        #[doc(hidden)]
        #[inline]
        pub fn get_map_rendering(p0: i32, p1: i32) -> i32 {
            __core_owned_get_map_rendering::call(p0, p1)
        }

        #[inline]
        pub fn get_matrix_data(mode: u32) -> Result<Vec<f32>> {
            let value = crate::generated::gfx::get_matrix_data(mode)?;
            Ok(value.into_iter().collect::<Vec<_>>())
        }

        #[inline]
        pub fn get_number(pname: u32, max_values: u32) -> Result<GetNumberValue> {
            let value = crate::generated::gfx::get_number(pname, max_values)?;
            Ok(GetNumberValue {
                values: value.0.into_iter().collect::<Vec<_>>(),
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

        #[inline]
        pub fn get_subroutine_index(shader_id: u32, shader_type: u32, name: &str) -> Result<GetSubroutineIndexValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + name.len()); __b.extend_from_slice(&(name.len() as u32).to_le_bytes()); __b.extend_from_slice(name.as_bytes()); __b };
            let mut __output = [0u8; 8];
            crate::generated::dynamic_input::gfx::get_subroutine_index(shader_id as i32, shader_type as i32, &__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(GetSubroutineIndexValue {
                index: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                success: crate::generated::__core_wire::boolean(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_get_sun {
            #[link(wasm_import_module = "spring:gfx")]
            unsafe extern "C" {
                #[link_name = "get-sun"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.get-sun."]
        #[doc(hidden)]
        #[inline]
        pub fn get_sun(p0: i32, p1: i32) -> i32 {
            __core_owned_get_sun::call(p0, p1)
        }

        #[inline]
        pub fn get_text_height(value: &str) -> Result<GetTextHeightValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + value.len()); __b.extend_from_slice(&(value.len() as u32).to_le_bytes()); __b.extend_from_slice(value.as_bytes()); __b };
            let mut __output = [0u8; 12];
            crate::generated::dynamic_input::gfx::get_text_height(&__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(GetTextHeightValue {
                height: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                descender: crate::generated::__core_wire::f32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                lines: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
        }

        #[inline]
        pub fn get_text_width(value: &str) -> Result<f32> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(value, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(value)?),
            };
            crate::generated::borrowed::gfx::get_text_width(__core_string_0_buf.as_cstr())
        }

        #[inline]
        pub fn get_uniform_location(shader_id: u32, name: &str) -> Result<i32> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(name, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(name)?),
            };
            crate::generated::borrowed::gfx::get_uniform_location(shader_id, __core_string_1_buf.as_cstr())
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
            unsafe extern "C" {
                #[link_name = "get-water-rendering"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.get-water-rendering."]
        #[doc(hidden)]
        #[inline]
        pub fn get_water_rendering(p0: i32, p1: i32) -> i32 {
            __core_owned_get_water_rendering::call(p0, p1)
        }

        #[inline]
        pub fn has_extension(value: &str) -> Result<bool> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(value, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(value)?),
            };
            crate::generated::borrowed::gfx::has_extension(__core_string_0_buf.as_cstr())
        }

        #[inline]
        pub fn instance_data_from_feature_defs_vbo(vbo_id: u32, ids: &[u32], attribute_index: i32, team_id: i32, element_offset: i32) -> Result<u32> {
            crate::generated::borrowed::gfx::instance_data_from_feature_defs_vbo(vbo_id, ids, attribute_index, team_id, element_offset)
        }

        #[inline]
        pub fn instance_data_from_features_vbo(vbo_id: u32, ids: &[u32], attribute_index: i32, team_id: i32, element_offset: i32) -> Result<u32> {
            crate::generated::borrowed::gfx::instance_data_from_features_vbo(vbo_id, ids, attribute_index, team_id, element_offset)
        }

        #[inline]
        pub fn instance_data_from_unit_defs_vbo(vbo_id: u32, ids: &[u32], attribute_index: i32, team_id: i32, element_offset: i32) -> Result<u32> {
            crate::generated::borrowed::gfx::instance_data_from_unit_defs_vbo(vbo_id, ids, attribute_index, team_id, element_offset)
        }

        #[inline]
        pub fn instance_data_from_units_vbo(vbo_id: u32, ids: &[u32], attribute_index: i32, team_id: i32, element_offset: i32) -> Result<u32> {
            crate::generated::borrowed::gfx::instance_data_from_units_vbo(vbo_id, ids, attribute_index, team_id, element_offset)
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
        pub fn light(light: i32, options: GfxLightOptions, pname: u32, values: &[f32], count: u32) -> Result<()> {
            crate::generated::gfx::light(light, crate::generated::gfx::GfxLightOptions { set_state: options.set_state, state: options.state }, pname, values.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, count)?;
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
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(path, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(path)?),
            };
            crate::generated::borrowed::gfx::load_font(__core_string_0_buf.as_cstr(), size, outline_width, outline_weight)
        }

        #[inline]
        pub fn load_identity(unused: u8) -> Result<()> {
            crate::generated::gfx::load_identity(unused)?;
            Ok(())
        }

        #[inline]
        pub fn load_matrix(values: &[f32]) -> Result<()> {
            crate::generated::gfx::load_matrix(values.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?)?;
            Ok(())
        }

        #[inline]
        pub fn logic_op(enable: bool, opcode: u32) -> Result<()> {
            crate::generated::gfx::logic_op(enable, opcode)?;
            Ok(())
        }

        #[inline]
        pub fn material(pname: u32, values: &[f32], count: u32) -> Result<()> {
            crate::generated::gfx::material(pname, values.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, count)?;
            Ok(())
        }

        #[inline]
        pub fn matrix_data_from_projectiles_vbo(vbo_id: u32, ids: &[u32], attribute_index: i32, team_id: i32, element_offset: i32) -> Result<u32> {
            crate::generated::borrowed::gfx::matrix_data_from_projectiles_vbo(vbo_id, ids, attribute_index, team_id, element_offset)
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
        pub fn mult_matrix(values: &[f32]) -> Result<()> {
            crate::generated::gfx::mult_matrix(values.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?)?;
            Ok(())
        }

        #[inline]
        pub fn multi_tex_coord(tex_num: i32, s: f32, t: f32, r: f32, q: f32, count: u32) -> Result<()> {
            crate::generated::gfx::multi_tex_coord(tex_num, s, t, r, q, count)?;
            Ok(())
        }

        #[inline]
        pub fn multi_tex_env(tex_num: i32, target: u32, pname: u32, values: &[f32], count: u32) -> Result<()> {
            crate::generated::gfx::multi_tex_env(tex_num, target, pname, values.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, count)?;
            Ok(())
        }

        #[inline]
        pub fn multi_tex_gen(tex_num: i32, target: u32, options: GfxMultiTexGenOptions, pname: u32, values: &[f32], count: u32) -> Result<()> {
            crate::generated::gfx::multi_tex_gen(tex_num, target, crate::generated::gfx::GfxMultiTexGenOptions { set_state: options.set_state, state: options.state }, pname, values.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, count)?;
            Ok(())
        }

        #[inline]
        pub fn normal(x: f32, y: f32, z: f32) -> Result<()> {
            crate::generated::gfx::normal(x, y, z)?;
            Ok(())
        }

        #[inline]
        pub fn object_label(identifier: u32, object_id: u32, label: &str) -> Result<()> {
            let mut __core_string_2_scratch = [0u8; 256];
            let __core_string_2_buf = match super::write_cstr(label, &mut __core_string_2_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(label)?),
            };
            crate::generated::borrowed::gfx::object_label(identifier, object_id, __core_string_2_buf.as_cstr())
        }

        #[inline]
        pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near_val: f32, far_val: f32) -> Result<()> {
            crate::generated::gfx::ortho(left, right, bottom, top, near_val, far_val)?;
            Ok(())
        }

        #[inline]
        pub fn point_parameter(pname: u32, value: f32, values: &[f32], count: u32) -> Result<()> {
            crate::generated::gfx::point_parameter(pname, value, values.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, count)?;
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

        #[inline]
        pub fn push_debug_group(id: u32, message: &str, source_is_third_party: bool) -> Result<()> {
            let mut __core_string_1_scratch = [0u8; 256];
            let __core_string_1_buf = match super::write_cstr(message, &mut __core_string_1_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(message)?),
            };
            crate::generated::borrowed::gfx::push_debug_group(id, __core_string_1_buf.as_cstr(), source_is_third_party)
        }

        #[inline]
        pub fn push_matrix(unused: u8) -> Result<()> {
            crate::generated::gfx::push_matrix(unused)?;
            Ok(())
        }

        #[cfg(target_arch = "wasm32")]
        mod __core_owned_push_pop_matrix {
            #[link(wasm_import_module = "spring:gfx")]
            unsafe extern "C" {
                #[link_name = "push-pop-matrix"]
                pub safe fn call(p0: i32, p1: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.push-pop-matrix."]
        #[doc(hidden)]
        #[inline]
        pub fn push_pop_matrix(p0: i32, p1: i32) -> i32 {
            __core_owned_push_pop_matrix::call(p0, p1)
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
            unsafe extern "C" {
                #[link_name = "read-pixels"]
                pub safe fn call(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.read-pixels."]
        #[doc(hidden)]
        #[inline]
        pub fn read_pixels(p0: i32, p1: i32, p2: i32, p3: i32, p4: i32, p5: i32) -> i32 {
            __core_owned_read_pixels::call(p0, p1, p2, p3, p4, p5)
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
            unsafe extern "C" {
                #[link_name = "render-to-texture"]
                pub safe fn call(p0: i32, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.render-to-texture."]
        #[doc(hidden)]
        #[inline]
        pub fn render_to_texture(p0: i32, p1: i32, p2: i32, p3: i32) -> i32 {
            __core_owned_render_to_texture::call(p0, p1, p2, p3)
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
            unsafe extern "C" {
                #[link_name = "run-query"]
                pub safe fn call(p0: i32, p1: i32, p2: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.run-query."]
        #[doc(hidden)]
        #[inline]
        pub fn run_query(p0: i32, p1: i32, p2: i32) -> i32 {
            __core_owned_run_query::call(p0, p1, p2)
        }

        #[inline]
        pub fn save_image(x: i32, y: i32, width: i32, height: i32, filename: &str, options: GfxSaveImageOptions, read_buffer: u32) -> Result<bool> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + filename.len()); __b.extend_from_slice(&(filename.len() as u32).to_le_bytes()); __b.extend_from_slice(filename.as_bytes()); __b };
            let __blob1 = { let mut __b = Vec::new(); __b.extend_from_slice(&(if options.alpha { 1u32 } else { 0u32 }).to_le_bytes()); __b.extend_from_slice(&(if options.yflip { 1u32 } else { 0u32 }).to_le_bytes()); __b.extend_from_slice(&(if options.grayscale16bit { 1u32 } else { 0u32 }).to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); } __b };
            crate::generated::dynamic_input::gfx::save_image(x, y, width, height, read_buffer as i32, &__blob0, &__blob1)
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

        #[inline]
        pub fn set_fbo_attachment(fbo_id: u32, attachment: u32, texture_name: &str, texture_target: u32, mip_level: i32, rbo_id: u32, use_rbo: bool) -> Result<()> {
            let mut __core_string_2_scratch = [0u8; 256];
            let __core_string_2_buf = match super::write_cstr(texture_name, &mut __core_string_2_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(texture_name)?),
            };
            crate::generated::borrowed::gfx::set_fbo_attachment(fbo_id, attachment, __core_string_2_buf.as_cstr(), texture_target, mip_level, rbo_id, use_rbo)
        }

        #[inline]
        pub fn set_fbo_draw_buffers(fbo_id: u32, buffers: &[u32]) -> Result<()> {
            crate::generated::borrowed::gfx::set_fbo_draw_buffers(fbo_id, buffers)
        }

        #[inline]
        pub fn set_fbo_read_buffer(fbo_id: u32, buffer: u32) -> Result<()> {
            crate::generated::gfx::set_fbo_read_buffer(fbo_id, buffer)?;
            Ok(())
        }

        #[inline]
        pub fn set_feature_buffer_uniforms(object_id: i32, values: &[f32], offset: u32) -> Result<u32> {
            crate::generated::borrowed::gfx::set_feature_buffer_uniforms(object_id, values, offset)
        }

        #[inline]
        pub fn set_geometry_shader_parameter(shader_id: u32, param: u32, value: i32) -> Result<()> {
            crate::generated::gfx::set_geometry_shader_parameter(shader_id, param, value)?;
            Ok(())
        }

        #[inline]
        pub fn set_tesselation_shader_parameter(param: u32, value: i32, values: &[f32], value_count: u32, use_float_array: bool) -> Result<()> {
            crate::generated::gfx::set_tesselation_shader_parameter(param, value, values.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, value_count, use_float_array)?;
            Ok(())
        }

        #[inline]
        pub fn set_unit_buffer_uniforms(object_id: i32, values: &[f32], offset: u32) -> Result<u32> {
            crate::generated::borrowed::gfx::set_unit_buffer_uniforms(object_id, values, offset)
        }

        #[inline]
        pub fn shade_model(mode: u32) -> Result<()> {
            crate::generated::gfx::shade_model(mode)?;
            Ok(())
        }

        #[inline]
        pub fn shape(primitive: u32, vertices: &[GfxVertexData]) -> Result<()> {
            let __blob0 = { let mut __b = Vec::new(); __b.extend_from_slice(&(vertices.len() as u32).to_le_bytes()); for __item in vertices.iter() { for __i0 in 0..3usize { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.vertex[__i0].to_bits().to_le_bytes()); } for __i147 in 0..3usize { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.normal[__i147].to_bits().to_le_bytes()); } for __i298 in 0..2usize { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.tex_coord[__i298].to_bits().to_le_bytes()); } for __i452 in 0..4usize { while !__b.len().is_multiple_of(4) { __b.push(0); } __b.extend_from_slice(&__item.color[__i452].to_bits().to_le_bytes()); } __b.extend_from_slice(&(if __item.has_vertex { 1u32 } else { 0u32 }).to_le_bytes()); __b.extend_from_slice(&(if __item.has_normal { 1u32 } else { 0u32 }).to_le_bytes()); __b.extend_from_slice(&(if __item.has_tex_coord { 1u32 } else { 0u32 }).to_le_bytes()); __b.extend_from_slice(&(if __item.has_color { 1u32 } else { 0u32 }).to_le_bytes()); while !__b.len().is_multiple_of(4) { __b.push(0); }} __b };
            crate::generated::dynamic_input::gfx::shape(primitive as i32, &__blob0)
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
        pub fn tex_env(target: u32, pname: u32, values: &[f32], count: u32) -> Result<()> {
            crate::generated::gfx::tex_env(target, pname, values.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, count)?;
            Ok(())
        }

        #[inline]
        pub fn tex_gen(target: u32, options: GfxTexGenOptions, pname: u32, values: &[f32], count: u32) -> Result<()> {
            crate::generated::gfx::tex_gen(target, crate::generated::gfx::GfxTexGenOptions { set_state: options.set_state, state: options.state }, pname, values.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, count)?;
            Ok(())
        }

        #[inline]
        #[expect(clippy::too_many_arguments, reason = "Core function preserves the corresponding Lua API arity")]
        pub fn tex_rect(x1: f32, y1: f32, x2: f32, y2: f32, s1: f32, t1: f32, s2: f32, t2: f32) -> Result<()> {
            crate::generated::gfx::tex_rect(x1, y1, x2, y2, s1, t1, s2, t2)?;
            Ok(())
        }

        #[inline]
        pub fn text(text: &str, x: f32, y: f32, size: f32, options: &str) -> Result<()> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(text, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(text)?),
            };
            let mut __core_string_4_scratch = [0u8; 256];
            let __core_string_4_buf = match super::write_cstr(options, &mut __core_string_4_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(options)?),
            };
            crate::generated::borrowed::gfx::text(__core_string_0_buf.as_cstr(), x, y, size, __core_string_4_buf.as_cstr())
        }

        #[inline]
        pub fn text_env(target: u32, pname: u32, values: &[f32], count: u32) -> Result<()> {
            crate::generated::gfx::text_env(target, pname, values.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, count)?;
            Ok(())
        }

        #[inline]
        pub fn texture_info(name: &str) -> Result<TextureInfoValue> {
            let __blob0 = { let mut __b = Vec::with_capacity(4 + name.len()); __b.extend_from_slice(&(name.len() as u32).to_le_bytes()); __b.extend_from_slice(name.as_bytes()); __b };
            let mut __output = [0u8; 24];
            crate::generated::dynamic_input::gfx::texture_info(&__blob0, &mut __output)?;
            let mut __cursor = 0usize;
            Ok(TextureInfoValue {
                xsize: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                ysize: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                zsize: crate::generated::__core_wire::i32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                id: crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                target: crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?,
                fbo: crate::generated::__core_wire::u32(&__output, &mut __cursor).ok_or(crate::ApiError::new(crate::ErrorCode::Internal as i32))?
            })
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
        pub fn uniform(location: i32, values: &[f32], count: u32) -> Result<()> {
            crate::generated::gfx::uniform(location, values.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, count)?;
            Ok(())
        }

        #[inline]
        pub fn uniform_array_float(location: i32, values: &[f32]) -> Result<()> {
            crate::generated::borrowed::gfx::uniform_array_float(location, values)
        }

        #[inline]
        pub fn uniform_array_int(location: i32, values: &[i32]) -> Result<()> {
            crate::generated::borrowed::gfx::uniform_array_int(location, values)
        }

        #[inline]
        pub fn uniform_int(location: i32, values: &[i32], count: u32) -> Result<()> {
            crate::generated::gfx::uniform_int(location, values.to_vec().try_into().map_err(|_| crate::ApiError::new(crate::ErrorCode::InvalidArgument as i32))?, count)?;
            Ok(())
        }

        #[inline]
        pub fn uniform_matrix(location: i32, values: &[f32], transpose: bool) -> Result<()> {
            crate::generated::borrowed::gfx::uniform_matrix(location, values, transpose)
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
            unsafe extern "C" {
                #[link_name = "unsafe-state"]
                pub safe fn call(p0: i32, p1: i32, p2: i32, p3: i32) -> i32;
            }
        }

        #[doc = "Exact Core ABI forwarding entry for spring:gfx.unsafe-state."]
        #[doc(hidden)]
        #[inline]
        pub fn unsafe_state(p0: i32, p1: i32, p2: i32, p3: i32) -> i32 {
            __core_owned_unsafe_state::call(p0, p1, p2, p3)
        }

        #[inline]
        #[expect(clippy::too_many_arguments, reason = "Core function preserves the corresponding Lua API arity")]
        pub fn upload_texture(name: &str, target: u32, level: i32, xoff: i32, yoff: i32, zoff: i32, width: i32, height: i32, depth: i32, format: u32, pixel_type: u32, data: &[u8]) -> Result<()> {
            let mut __core_string_0_scratch = [0u8; 256];
            let __core_string_0_buf = match super::write_cstr(name, &mut __core_string_0_scratch) {
                Some(s) => super::CStrBuf::Stack(s),
                None => super::CStrBuf::Heap(super::str_to_cstr_heap(name)?),
            };
            crate::generated::borrowed::gfx::upload_texture(__core_string_0_buf.as_cstr(), target, level, xoff, yoff, zoff, width, height, depth, format, pixel_type, data)
        }

        #[inline]
        pub fn upload_vbo(vbo_id: u32, data: &[f32], attribute_index: i32, element_offset: i32, data_start_index: i32, data_finish_index: i32) -> Result<u32> {
            crate::generated::borrowed::gfx::upload_vbo(vbo_id, data, attribute_index, element_offset, data_start_index, data_finish_index)
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

