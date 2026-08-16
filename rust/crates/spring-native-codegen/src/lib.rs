use anyhow::{anyhow, Context, Result};
use clang::{Clang, Entity, EntityKind, Index, Type, TypeKind};
use heck::ToSnakeCase;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub mod annotations;
pub mod callins;
pub mod lua_loader;
pub mod manifest;
pub mod model;
pub mod render_callins;
pub mod render_host;
pub mod render_signatures;
pub mod render_wasm_sdk;
pub mod render_wit;

pub use model::{ApiModel, ApiModule, Environment, FunctionModel, LoweringStatus, SemanticType};

pub struct ApiConfig<'a> {
    pub api_struct: &'a str,
    pub wrapper_struct: &'a str,
}

pub fn generate_units_query(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "UnitsQueryApi",
            wrapper_struct: "UnitsQuery",
        },
    )
}

pub fn generate_units_info(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "UnitsInfoApi",
            wrapper_struct: "UnitsInfo",
        },
    )
}

pub fn generate_teams(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "TeamsApi",
            wrapper_struct: "Teams",
        },
    )
}

pub fn generate_units_weapons(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "UnitsWeaponsApi",
            wrapper_struct: "UnitsWeapons",
        },
    )
}

pub fn generate_units_commands(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "UnitsCommandsApi",
            wrapper_struct: "UnitsCommands",
        },
    )
}

pub fn generate_units_pieces(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "UnitsPiecesApi",
            wrapper_struct: "UnitsPieces",
        },
    )
}

pub fn generate_features(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "FeaturesApi",
            wrapper_struct: "Features",
        },
    )
}

pub fn generate_projectiles(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "ProjectilesApi",
            wrapper_struct: "Projectiles",
        },
    )
}

pub fn generate_los(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "LOSApi",
            wrapper_struct: "Los",
        },
    )
}

pub fn generate_unit_defs(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "UnitDefsApi",
            wrapper_struct: "UnitDefs",
        },
    )
}

pub fn generate_feature_defs(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "FeatureDefsApi",
            wrapper_struct: "FeatureDefs",
        },
    )
}

pub fn generate_weapon_defs(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "WeaponDefsApi",
            wrapper_struct: "WeaponDefs",
        },
    )
}

pub fn generate_game(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "GameApi",
            wrapper_struct: "Game",
        },
    )
}

pub fn generate_terrain(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "TerrainApi",
            wrapper_struct: "Terrain",
        },
    )
}

pub fn generate_player(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "PlayerApi",
            wrapper_struct: "Player",
        },
    )
}

pub fn generate_math_extra(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "MathExtraApi",
            wrapper_struct: "MathExtra",
        },
    )
}

pub fn generate_encoding(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "EncodingApi",
            wrapper_struct: "Encoding",
        },
    )
}

pub fn generate_metal_map(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "MetalMapApi",
            wrapper_struct: "MetalMap",
        },
    )
}

pub fn generate_path_finder(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "PathFinderApi",
            wrapper_struct: "PathFinder",
        },
    )
}

pub fn generate_platform(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "PlatformApi",
            wrapper_struct: "Platform",
        },
    )
}

pub fn generate_rules_params(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "RulesParamsApi",
            wrapper_struct: "RulesParams",
        },
    )
}

pub fn generate_move_ctrl(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "MoveCtrlApi",
            wrapper_struct: "MoveCtrl",
        },
    )
}

pub fn generate_synced_ctrl(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "SyncedCtrlApi",
            wrapper_struct: "SyncedCtrl",
        },
    )
}

pub fn generate_camera(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "CameraApi",
            wrapper_struct: "Camera",
        },
    )
}

pub fn generate_input(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "InputApi",
            wrapper_struct: "Input",
        },
    )
}

pub fn generate_debug_input(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "DebugInputApi",
            wrapper_struct: "DebugInput",
        },
    )
}

pub fn generate_display(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "DisplayApi",
            wrapper_struct: "Display",
        },
    )
}

pub fn generate_selection(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "SelectionApi",
            wrapper_struct: "Selection",
        },
    )
}

pub fn generate_vfs(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "VFSApi",
            wrapper_struct: "Vfs",
        },
    )
}

pub fn generate_rml_ui(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "RmlUiApi",
            wrapper_struct: "RmlUi",
        },
    )
}

pub fn generate_sound(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "SoundApi",
            wrapper_struct: "Sound",
        },
    )
}

pub fn generate_messages(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "MessagesApi",
            wrapper_struct: "Messages",
        },
    )
}

pub fn generate_config(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "ConfigApi",
            wrapper_struct: "Config",
        },
    )
}

pub fn generate_tracing(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "TracingApi",
            wrapper_struct: "Tracing",
        },
    )
}

pub fn generate_lights(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "LightsApi",
            wrapper_struct: "Lights",
        },
    )
}

pub fn generate_gfx(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "GfxApi",
            wrapper_struct: "Gfx",
        },
    )
}

pub fn generate_utils(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "UtilsApi",
            wrapper_struct: "Utils",
        },
    )
}

pub fn generate_icons(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "IconsApi",
            wrapper_struct: "Icons",
        },
    )
}

pub fn generate_markers(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "MarkersApi",
            wrapper_struct: "Markers",
        },
    )
}

pub fn generate_ground_decals(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "GroundDecalsApi",
            wrapper_struct: "GroundDecals",
        },
    )
}

pub fn generate_system_control(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "SystemControlApi",
            wrapper_struct: "SystemControl",
        },
    )
}

pub fn generate_profiling(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "ProfilingApi",
            wrapper_struct: "Profiling",
        },
    )
}

pub fn generate_memory(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "MemoryApi",
            wrapper_struct: "Memory",
        },
    )
}

pub fn generate_unsynced_ctrl(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "UnsyncedCtrlApi",
            wrapper_struct: "UnsyncedCtrl",
        },
    )
}

// SyncedCtrl sub-APIs
pub fn generate_team_control(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "TeamControlApi",
            wrapper_struct: "TeamControl",
        },
    )
}

pub fn generate_unit_control(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "UnitControlApi",
            wrapper_struct: "UnitControl",
        },
    )
}

pub fn generate_feature_control(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "FeatureControlApi",
            wrapper_struct: "FeatureControl",
        },
    )
}

pub fn generate_terrain_control(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "TerrainControlApi",
            wrapper_struct: "TerrainControl",
        },
    )
}

pub fn generate_projectile_control(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "ProjectileControlApi",
            wrapper_struct: "ProjectileControl",
        },
    )
}

pub fn generate_effects_control(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "EffectsControlApi",
            wrapper_struct: "EffectsControl",
        },
    )
}

pub fn generate_game_config(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "GameConfigApi",
            wrapper_struct: "GameConfig",
        },
    )
}

pub fn generate_cob_script(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "COBScriptApi",
            wrapper_struct: "CobScript",
        },
    )
}

pub fn generate_unsynced_read(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "UnsyncedReadApi",
            wrapper_struct: "UnsyncedRead",
        },
    )
}

pub fn generate_unit_rendering(
    codegen: &CodeGenerator,
    header: &Path,
    include_dirs: &[PathBuf],
) -> Result<String> {
    codegen.generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "UnitRenderingApi",
            wrapper_struct: "UnitRendering",
        },
    )
}

pub struct CodeGenerator {
    clang: Clang,
    lua_loaders: lua_loader::LuaLoaderMatrix,
}

impl CodeGenerator {
    pub fn new() -> Result<Self> {
        let clang = Clang::new().map_err(|error| anyhow!(error))?;
        Ok(Self {
            clang,
            lua_loaders: lua_loader::LuaLoaderMatrix::default(),
        })
    }

    pub fn with_repository_root(root: &Path) -> Result<Self> {
        let clang = Clang::new().map_err(|error| anyhow!(error))?;
        let lua_loaders = lua_loader::LuaLoaderMatrix::from_repository(root)?;
        Ok(Self { clang, lua_loaders })
    }

    fn generate_api(
        &self,
        header: &Path,
        include_dirs: &[PathBuf],
        config: ApiConfig<'_>,
    ) -> Result<String> {
        let spec = parse_api(&self.clang, header, include_dirs, config.api_struct)?;
        render_api(&spec, &config)
    }

    /// Build the transport-neutral semantic model for one NativeInterface
    /// header.  Native generation continues to use the legacy renderer above;
    /// this entry point is shared by the Wasm/WIT/signature generators.
    pub fn semantic_module(
        &self,
        header: &Path,
        include_dirs: &[PathBuf],
        api_struct: &str,
        module_name: &str,
    ) -> Result<ApiModule> {
        let spec = parse_api(&self.clang, header, include_dirs, api_struct)?;
        Ok(model::from_legacy_spec(
            &spec,
            module_name,
            header,
            &self.lua_loaders,
        ))
    }
}

#[derive(Debug, Clone)]
struct ApiSpec {
    structs: HashMap<String, StructDef>,
    all_structs: HashMap<String, StructDef>,
    enums: HashMap<String, EnumDef>,
    api: ApiDef,
}

#[derive(Debug, Clone)]
struct StructDef {
    name: String,
    fields: Vec<FieldDef>,
}

#[derive(Debug, Clone)]
struct EnumDef {
    name: String,
    variants: std::collections::BTreeMap<String, i64>,
}

#[derive(Debug, Clone)]
struct FieldDef {
    name: String,
    ty: CType,
    annotations: Vec<String>,
}

#[derive(Debug, Clone)]
enum CType {
    Primitive(Primitive),
    Enum(String),
    Record(String),
    Pointer {
        pointee: Box<CType>,
        is_const: bool,
    },
    /// A C function pointer (e.g. the NativeEditCallback typedef). Recognized so
    /// the generated wrapper can take a Rust closure instead of a raw pointer.
    FnPtr,
    #[allow(dead_code)]
    Array {
        element: Box<CType>,
        length: u64,
    },
    #[allow(dead_code)]
    Unknown(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Primitive {
    I8,
    I16,
    I32,
    I64,
    U32,
    U16,
    U8,
    U64,
    Bool,
    F32,
    Char,
    Void,
}

impl Primitive {
    fn rust_type(self) -> &'static str {
        match self {
            Primitive::I8 => "i8",
            Primitive::I16 => "i16",
            Primitive::I32 => "i32",
            Primitive::I64 => "i64",
            Primitive::U32 => "u32",
            Primitive::U16 => "u16",
            Primitive::U8 => "u8",
            Primitive::U64 => "u64",
            Primitive::Bool => "bool",
            Primitive::F32 => "f32",
            Primitive::Char => "i8",
            Primitive::Void => "std::ffi::c_void",
        }
    }
}

#[derive(Debug, Clone)]
struct ApiDef {
    functions: Vec<ApiFunction>,
}

#[derive(Debug, Clone)]
struct ApiFunction {
    name: String,
    query: String,
    result: String,
}

fn parse_api(
    clang: &Clang,
    header: &Path,
    include_dirs: &[PathBuf],
    api_struct: &str,
) -> Result<ApiSpec> {
    let index = Index::new(clang, false, false);
    let mut parser = index.parser(header);
    let mut args = vec![
        "-xc++".to_string(),
        "-std=c++17".to_string(),
        "-DRECOIL_WASM_CODEGEN".to_string(),
    ];
    for dir in include_dirs {
        args.push(format!("-I{}", dir.display()));
    }
    parser.arguments(&args);
    let tu = parser
        .parse()
        .with_context(|| format!("failed to parse {}", header.display()))?;

    let mut structs = HashMap::new();
    let mut all_structs = HashMap::new();
    let mut enums = HashMap::new();
    let mut api = None;
    visit_entity(
        tu.get_entity(),
        api_struct,
        &mut structs,
        &mut all_structs,
        &mut enums,
        &mut api,
    );
    let api = match api_struct {
        "" => ApiDef {
            functions: Vec::new(),
        },
        _ => api.context(format!("{} not found", api_struct))?,
    };
    Ok(ApiSpec {
        structs,
        all_structs,
        enums,
        api,
    })
}

fn visit_entity(
    entity: Entity,
    api_name: &str,
    structs: &mut HashMap<String, StructDef>,
    all_structs: &mut HashMap<String, StructDef>,
    enums: &mut HashMap<String, EnumDef>,
    api: &mut Option<ApiDef>,
) {
    if entity.get_kind() == EntityKind::StructDecl {
        if let Some(name) = entity.get_name() {
            if entity.is_definition() {
                if name == api_name {
                    *api = Some(parse_api_struct(entity));
                } else {
                    let definition = parse_struct(entity);
                    if is_query_struct(&name) {
                        structs.insert(name.clone(), definition.clone());
                    }
                    all_structs.insert(name.clone(), definition);
                }
            }
        }
    }

    if entity.get_kind() == EntityKind::EnumDecl {
        if let Some(name) = entity.get_name() {
            let mut variants = std::collections::BTreeMap::new();
            entity.visit_children(|child, _| {
                if child.get_kind() == EntityKind::EnumConstantDecl {
                    if let (Some(variant), Some((signed, _))) =
                        (child.get_name(), child.get_enum_constant_value())
                    {
                        variants.insert(variant, signed);
                    }
                }
                clang::EntityVisitResult::Continue
            });
            enums.insert(name.clone(), EnumDef { name, variants });
        }
    }

    entity.visit_children(|child, _| {
        visit_entity(child, api_name, structs, all_structs, enums, api);
        clang::EntityVisitResult::Continue
    });
}

fn is_query_struct(name: &str) -> bool {
    name.ends_with("Query")
        || name.ends_with("Result")
        || name.ends_with("Params")
        || name.ends_with("Options")
        || name.ends_with("Count")
}

fn parse_struct(entity: Entity) -> StructDef {
    let name = entity.get_name().unwrap_or_default();
    let mut fields = Vec::new();
    entity.visit_children(|child, _| {
        if child.get_kind() == EntityKind::FieldDecl {
            if let (Some(field_name), Some(field_type)) = (child.get_name(), child.get_type()) {
                let mut annotations = Vec::new();
                child.visit_children(|attribute, _| {
                    if attribute.get_kind() == EntityKind::AnnotateAttr {
                        if let Some(annotation) = attribute.get_display_name() {
                            annotations.push(annotation);
                        }
                    }
                    clang::EntityVisitResult::Continue
                });
                fields.push(FieldDef {
                    name: field_name,
                    ty: classify_type(field_type),
                    annotations,
                });
            }
        }
        clang::EntityVisitResult::Continue
    });
    StructDef { name, fields }
}

fn parse_api_struct(entity: Entity) -> ApiDef {
    let mut functions = Vec::new();
    entity.visit_children(|child, _| {
        if child.get_kind() == EntityKind::FieldDecl {
            if let (Some(name), Some(ty)) = (child.get_name(), child.get_type()) {
                if let Some(func) = parse_api_function(&name, ty) {
                    functions.push(func);
                }
            }
        }
        clang::EntityVisitResult::Continue
    });
    ApiDef { functions }
}

fn parse_api_function(name: &str, ty: Type) -> Option<ApiFunction> {
    let pointee = ty.get_pointee_type()?;
    if pointee.get_kind() != TypeKind::FunctionPrototype {
        return None;
    }
    let args = pointee.get_argument_types()?;
    if args.len() != 2 {
        return None;
    }
    let query = extract_struct_name(&args[0])?;
    let result = extract_struct_name(&args[1])?;
    Some(ApiFunction {
        name: name.to_string(),
        query,
        result,
    })
}

fn extract_struct_name(arg: &Type) -> Option<String> {
    let pointee = arg.get_pointee_type()?;
    let decl = pointee.get_declaration()?;
    decl.get_name()
}

fn classify_type(ty: Type) -> CType {
    match ty.get_kind() {
        TypeKind::Pointer => {
            let pointee = ty.get_pointee_type().unwrap();
            if pointee.get_kind() == TypeKind::FunctionPrototype {
                return CType::FnPtr;
            }
            CType::Pointer {
                pointee: Box::new(classify_type(pointee)),
                is_const: pointee.is_const_qualified(),
            }
        }
        TypeKind::Typedef | TypeKind::Elaborated | TypeKind::Unexposed => {
            if let Some(prim) = primitive_from_name(&ty.get_display_name()) {
                return CType::Primitive(prim);
            }
            let canonical = ty.get_canonical_type();
            if let Some(prim) = primitive_from_name(&canonical.get_display_name()) {
                return CType::Primitive(prim);
            }
            classify_type(canonical)
        }
        TypeKind::Int => CType::Primitive(Primitive::I32),
        TypeKind::Long => CType::Primitive(Primitive::I64),
        TypeKind::LongLong => CType::Primitive(Primitive::I64),
        TypeKind::Short => CType::Primitive(Primitive::I16),
        TypeKind::SChar => CType::Primitive(Primitive::I8),
        TypeKind::UInt => CType::Primitive(Primitive::U32),
        TypeKind::UShort => CType::Primitive(Primitive::U16),
        TypeKind::UChar => CType::Primitive(Primitive::U8),
        TypeKind::ULongLong => CType::Primitive(Primitive::U64),
        TypeKind::ULong => CType::Primitive(Primitive::U64),
        TypeKind::Bool => CType::Primitive(Primitive::Bool),
        TypeKind::Float => CType::Primitive(Primitive::F32),
        TypeKind::CharS => CType::Primitive(Primitive::Char),
        TypeKind::CharU => CType::Primitive(Primitive::Char),
        TypeKind::Record => {
            if let Some(decl) = ty.get_declaration() {
                if let Some(name) = decl.get_name() {
                    return CType::Record(name);
                }
            }
            CType::Unknown(ty.get_display_name())
        }
        TypeKind::Enum => {
            if let Some(decl) = ty.get_declaration() {
                if let Some(name) = decl.get_name() {
                    return CType::Enum(name);
                }
            }
            CType::Unknown(ty.get_display_name())
        }
        TypeKind::ConstantArray => {
            let element_ty = ty.get_element_type();
            let element = element_ty
                .as_ref()
                .map(|t| classify_type(*t))
                .unwrap_or(CType::Unknown("unknown".into()));
            // Calculate array length: total_size / element_size
            let total_size = ty.get_sizeof().unwrap_or(0);
            let elem_size = element_ty.and_then(|t| t.get_sizeof().ok()).unwrap_or(1);
            let len = total_size.checked_div(elem_size).unwrap_or(0);
            CType::Array {
                element: Box::new(element),
                length: len as u64,
            }
        }
        _ => CType::Unknown(ty.get_display_name()),
    }
}

fn primitive_from_name(name: &str) -> Option<Primitive> {
    match name {
        "int8_t" | "signed char" => Some(Primitive::I8),
        "int16_t" | "short" => Some(Primitive::I16),
        "int32_t" | "int" => Some(Primitive::I32),
        "int64_t" | "long" | "long long" | "signed long" | "signed long long" => {
            Some(Primitive::I64)
        }
        "uint32_t" | "unsigned int" => Some(Primitive::U32),
        "uint16_t" | "unsigned short" => Some(Primitive::U16),
        "uint8_t" | "unsigned char" => Some(Primitive::U8),
        "bool" => Some(Primitive::Bool),
        "float" => Some(Primitive::F32),
        "char" | "const char" => Some(Primitive::Char),
        "void" => Some(Primitive::Void),
        _ => None,
    }
}

fn render_api(spec: &ApiSpec, config: &ApiConfig<'_>) -> Result<String> {
    let mut out = String::new();

    // Query options are deliberately represented as public Rust descriptors,
    // rather than exposing bindgen's C-field names and presence flags.  Keep
    // this limited to option records that are actually accepted by a query;
    // result-only records such as UnitDefBuildOptions do not belong in the
    // API's call surface.
    let mut option_names = spec
        .api
        .functions
        .iter()
        .filter_map(|func| spec.structs.get(&func.query))
        .flat_map(|query| query.fields.iter())
        .filter_map(|field| match &field.ty {
            CType::Record(name) if name.ends_with("Options") => Some(name.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    option_names.sort();
    option_names.dedup();

    for name in option_names {
        let options = spec
            .structs
            .get(&name)
            .with_context(|| format!("missing option struct {name}"))?;
        out.push_str(&render_options(options)?);
        out.push('\n');
    }

    out.push_str(&format!("impl<'a> {}<'a> {{\n", config.wrapper_struct));
    for func in &spec.api.functions {
        // Data-model event callbacks have a typed two-argument C callback
        // signature. They are implemented by the hand-written RmlUi wrapper,
        // which can expose the callback arguments as owned Rust values; the
        // generic FnMut() callback generator is intentionally not applicable.
        if is_explicit_native_exclusion(config.wrapper_struct, &func.name) {
            continue;
        }
        let query = spec
            .structs
            .get(&func.query)
            .with_context(|| format!("missing query struct {}", func.query))?;
        let result = spec
            .structs
            .get(&func.result)
            .with_context(|| format!("missing result struct {}", func.result))?;
        let method = render_method(func, query, result)?;
        out.push_str(&method);
        out.push('\n');
    }
    out.push_str("}\n");
    Ok(out)
}

fn is_explicit_native_exclusion(wrapper_struct: &str, function: &str) -> bool {
    matches!(
        (wrapper_struct, function),
        ("RmlUi", "DataModelBindEvent") | ("RmlUi", "DataModelUnbindEvent")
    )
}

fn render_options(options: &StructDef) -> Result<String> {
    let mut fields = Vec::new();
    let mut conversions = Vec::new();
    let mut i = 0;

    while i < options.fields.len() {
        let field = &options.fields[i];
        let rust_name = make_param_name(&field.name);

        if let Some(next) = options.fields.get(i + 1) {
            if is_optional_presence_pair(field, next) {
                let CType::Primitive(primitive) = &field.ty else {
                    return Err(anyhow!(
                        "option presence pair {}.{} has unsupported value type",
                        options.name,
                        field.name
                    ));
                };
                let rust_type = primitive.rust_type();
                fields.push(format!("    pub {rust_name}: Option<{rust_type}>,"));
                conversions.push(format!(
                    "            {}: options.{}.unwrap_or({}),\n            {}: options.{}.is_some(),",
                    field.name,
                    rust_name,
                    primitive_default_expr(*primitive),
                    next.name,
                    rust_name,
                ));
                i += 2;
                continue;
            }
        }

        let rust_type = match rust_type_from_c(&field.ty) {
            Some(ty) => type_ref_to_string(&ty),
            None => {
                return Err(anyhow!(
                    "option struct {} has unsupported field {}",
                    options.name,
                    field.name
                ));
            }
        };
        fields.push(format!("    pub {rust_name}: {rust_type},"));
        conversions.push(format!(
            "            {}: options.{},",
            field.name, rust_name
        ));
        i += 1;
    }

    Ok(format!(
        "#[derive(Debug, Clone, Copy, Default)]\npub struct {name} {{\n{fields}}}\n\nimpl From<{name}> for sys::{name} {{\n    fn from(options: {name}) -> Self {{\n        sys::{name} {{\n{conversions}        }}\n    }}\n}}\n",
        name = options.name,
        fields = fields.join("\n") + "\n",
        conversions = conversions.join("\n") + "\n",
    ))
}

#[derive(Debug)]
struct ParamSpec {
    name: String,
    ty: ParamType,
}

#[derive(Debug)]
enum ParamType {
    Primitive(&'static str),
    OptionPrimitive(&'static str),
    OptionCStr,
    RulesParamValue,
    Struct(String),
    Options(String),
    Slice {
        element: TypeRef,
    },
    Ref {
        element: TypeRef,
    },
    MutRef {
        element: TypeRef,
    },
    Array {
        element: &'static str,
        length: u64,
    },
    CStr,
    /// A `{ NativeEditCallback callback; void* userData; }` pair surfaced as a
    /// boxed `FnMut()` closure; the wrapper builds an extern-C trampoline.
    Callback,
}

#[derive(Debug, Clone)]
enum TypeRef {
    Primitive(&'static str),
    Struct(String),
}

#[derive(Debug)]
struct QueryInitField {
    field: String,
    expr: QueryExpr,
}

#[derive(Debug)]
enum QueryExpr {
    Param(String),
    SlicePtr {
        param: String,
        mutable_ptr: bool,
    },
    SliceLen {
        param: String,
        cast: &'static str,
    },
    RefPtr {
        param: String,
    },
    MutRefPtr {
        param: String,
    },
    CStrPtr {
        param: String,
    },
    OptionParam {
        param: String,
        default_expr: &'static str,
    },
    OptionIsSome {
        param: String,
    },
    OptionCStrPtr {
        param: String,
    },
    RulesParamValue {
        param: String,
    },
    Options {
        param: String,
    },
    Zero,
    /// The callback function pointer field: emits `Some(trampoline::<F>)`.
    CallbackFn,
    /// The userData field paired with a callback: emits the boxed closure pointer.
    CallbackUserData {
        param: String,
    },
    /// The callback-data destructor paired with a persistent callback.
    CallbackDestroyFn,
}

#[derive(Debug)]
struct ReturnField {
    name: String,
    ty: ReturnFieldType,
}

#[derive(Debug)]
enum ReturnFieldType {
    Plain(TypeRef),
    CString,
    StringWithLen {
        ptr_field: String,
        len_field: String,
    },
    Array {
        element: String,
        length: u64,
    },
    Vec {
        ptr_field: String,
        len_field: String,
        elem: TypeRef,
        mutable_ptr: bool,
    },
    StringVec {
        ptr_field: String,
        len_field: String,
    },
}

impl ReturnFieldType {
    fn ty_string(&self) -> String {
        match self {
            ReturnFieldType::Plain(ty) => type_ref_to_string(ty),
            ReturnFieldType::CString => "Option<String>".into(),
            ReturnFieldType::StringWithLen { .. } => "Option<String>".into(),
            ReturnFieldType::Array { element, length } => format!("[{}; {}]", element, length),
            ReturnFieldType::Vec { elem, .. } => format!("Vec<{}>", type_ref_to_string(elem)),
            ReturnFieldType::StringVec { .. } => "Vec<String>".into(),
        }
    }
}

#[derive(Debug)]
enum ReturnKind {
    Fields(Vec<ReturnField>),
    Unit,
}

fn render_method(func: &ApiFunction, query: &StructDef, result: &StructDef) -> Result<String> {
    let method_name = make_param_name(&func.name);
    let (params, inits) = build_params(query)?;
    let ret_kind = build_return(result)?;

    let has_callback = params.iter().any(|p| matches!(p.ty, ParamType::Callback));
    let has_persistent_callback = has_callback && func.name.contains("EventListener");
    let generics = if has_persistent_callback {
        "<F: FnMut() + 'static>"
    } else if has_callback {
        "<F: FnMut()>"
    } else {
        ""
    };
    let mut sig = format!("    pub fn {}{}(&self", method_name, generics);
    for param in &params {
        let ty = match &param.ty {
            ParamType::Primitive(name) => name.to_string(),
            ParamType::OptionPrimitive(name) => format!("Option<{}>", name),
            ParamType::OptionCStr => "Option<&str>".to_string(),
            ParamType::RulesParamValue => "RulesParamValue".to_string(),
            ParamType::Struct(struct_name) => format!("sys::{}", struct_name),
            ParamType::Options(struct_name) => struct_name.clone(),
            ParamType::Slice { element } => format!("&[{}]", type_ref_to_string(element)),
            ParamType::Ref { element } => format!("&{}", type_ref_to_string(element)),
            ParamType::MutRef { element } => format!("&mut {}", type_ref_to_string(element)),
            ParamType::Array { element, length } => format!("[{}; {}]", element, length),
            ParamType::CStr => "&str".to_string(),
            ParamType::Callback => "F".to_string(),
        };
        let prefix = if matches!(param.ty, ParamType::Callback) && !has_persistent_callback {
            "mut "
        } else {
            ""
        };
        sig.push_str(&format!(", {}{}: {}", prefix, param.name, ty));
    }
    sig.push_str(") -> Result<");
    let return_ty = match &ret_kind {
        ReturnKind::Unit => "()".to_string(),
        ReturnKind::Fields(fields) => {
            if fields.len() == 1 {
                fields[0].ty.ty_string()
            } else {
                let parts: Vec<_> = fields.iter().map(|f| f.ty.ty_string()).collect();
                format!("({})", parts.join(", "))
            }
        }
    };
    sig.push_str(&return_ty);
    sig.push_str(", Error> {\n");

    let mut body = String::new();
    body.push_str("        unsafe {\n");

    // Generate CStr conversions for string parameters
    for field in &inits {
        match &field.expr {
            QueryExpr::CStrPtr { param } => {
                body.push_str(&format!("            let {}_cstr = std::ffi::CString::new({}).map_err(|_| Error::invalid_argument(\"{}\"))?;\n", param, param, param));
            }
            QueryExpr::OptionCStrPtr { param } => {
                body.push_str(&format!("            let {}_cstr = {}.as_ref().map(|value| std::ffi::CString::new(*value)).transpose().map_err(|_| Error::invalid_argument(\"{}\"))?;\n", param, param, param));
            }
            QueryExpr::RulesParamValue { param } => {
                body.push_str(&format!(
                    "            let {param}_sys = {param}.to_sys()?;\n"
                ));
            }
            _ => {}
        }
    }

    // For a callback param, define an extern-C trampoline that recovers the
    // closure from userData and calls it.
    if has_callback {
        body.push_str("            unsafe extern \"C\" fn trampoline<F: FnMut()>(user_data: *mut std::ffi::c_void) {\n");
        body.push_str("                let f = &mut *(user_data as *mut F);\n");
        body.push_str("                f();\n");
        body.push_str("            }\n");
        if has_persistent_callback {
            body.push_str("            unsafe extern \"C\" fn destroy_callback<F>(user_data: *mut std::ffi::c_void) {\n");
            body.push_str("                drop(Box::from_raw(user_data as *mut F));\n");
            body.push_str("            }\n");
        }
        if has_persistent_callback {
            for param in params
                .iter()
                .filter(|param| matches!(param.ty, ParamType::Callback))
            {
                body.push_str(&format!(
                    "            let {0}_user_data = Box::into_raw(Box::new({0}));\n",
                    param.name
                ));
            }
        }
    }

    body.push_str(&format!("            let query = sys::{} {{\n", query.name));
    for field in &inits {
        let expr = match &field.expr {
            QueryExpr::Param(param) => param.clone(),
            QueryExpr::SlicePtr { param, mutable_ptr } => {
                if *mutable_ptr {
                    format!("{}.as_ptr() as *mut _", param)
                } else {
                    format!("{}.as_ptr()", param)
                }
            }
            QueryExpr::SliceLen { param, cast } => format!("{}.len() as {}", param, cast),
            QueryExpr::RefPtr { param } => format!("{} as *const _", param),
            QueryExpr::MutRefPtr { param } => format!("{} as *mut _", param),
            QueryExpr::CStrPtr { param } => format!("{}_cstr.as_ptr()", param),
            QueryExpr::OptionParam {
                param,
                default_expr,
            } => {
                format!("{}.unwrap_or({})", param, default_expr)
            }
            QueryExpr::OptionIsSome { param } => format!("{}.is_some()", param),
            QueryExpr::OptionCStrPtr { param } => {
                format!(
                    "{}_cstr.as_ref().map_or(std::ptr::null(), |value| value.as_ptr())",
                    param
                )
            }
            QueryExpr::RulesParamValue { param } => format!("{param}_sys.value"),
            QueryExpr::Options { param } => format!("{}.into()", param),
            QueryExpr::Zero => "0".into(),
            QueryExpr::CallbackFn => "Some(trampoline::<F>)".to_string(),
            QueryExpr::CallbackUserData { param } => {
                if has_persistent_callback {
                    format!("{param}_user_data as *mut std::ffi::c_void")
                } else {
                    format!("&mut {param} as *mut F as *mut std::ffi::c_void")
                }
            }
            QueryExpr::CallbackDestroyFn => "Some(destroy_callback::<F>)".to_string(),
        };
        body.push_str(&format!("                {}: {},\n", field.field, expr));
    }
    body.push_str("            };\n");
    body.push_str(&format!(
        "            let mut result = MaybeUninit::<sys::{}>::zeroed();\n",
        result.name
    ));
    body.push_str(&format!(
        "            let func = self.api.{}.expect(\"{} function pointer must be initialized\");\n",
        func.name, func.name
    ));
    body.push_str("            func(&query, result.as_mut_ptr());\n");
    body.push_str("            let result = result.assume_init();\n");
    if has_persistent_callback {
        for param in params
            .iter()
            .filter(|param| matches!(param.ty, ParamType::Callback))
        {
            body.push_str(&format!(
                "            if !result.success || !result.error.is_null() {{ drop(Box::from_raw({0}_user_data)); }}\n",
                param.name
            ));
        }
    }
    body.push_str(&render_return(&ret_kind));
    body.push_str("        }\n");
    body.push_str("    }\n");

    Ok(format!("{}{}", sig, body))
}

fn render_return(kind: &ReturnKind) -> String {
    match kind {
        ReturnKind::Unit => "            Error::result_or(result.error, ())\n".into(),
        ReturnKind::Fields(fields) => {
            if fields.len() == 1 {
                let expr = field_expr(&fields[0], "                ");
                format!(
                    "            Error::result_or(result.error, {{\n{expr}\n            }})\n",
                    expr = expr
                )
            } else {
                let mut out = String::from("            let value = (\n");
                for field in fields {
                    let expr = field_expr(field, "                ");
                    out.push_str(&expr);
                    out.push_str(",\n");
                }
                out.push_str("            );\n");
                out.push_str("            Error::result_or(result.error, value)\n");
                out
            }
        }
    }
}

fn field_expr(field: &ReturnField, indent: &str) -> String {
    match &field.ty {
        ReturnFieldType::Plain(TypeRef::Struct(name)) if name == "RulesParamValue" => {
            format!("{indent}RulesParamValue::from_sys(result.{})", field.name)
        }
        ReturnFieldType::Plain(_) => format!("{indent}result.{}", field.name),
        ReturnFieldType::CString => format!(
            "{indent}{{\n{indent}    if result.{name}.is_null() {{\n{indent}        None\n{indent}    }} else {{\n{indent}        Some(CStr::from_ptr(result.{name}).to_string_lossy().into_owned())\n{indent}    }}\n{indent}}}",
            indent = indent,
            name = field.name
        ),
        ReturnFieldType::StringWithLen {
            ptr_field,
            len_field,
        } => format!(
            "{indent}{{\n{indent}    if result.{ptr_field}.is_null() {{\n{indent}        None\n{indent}    }} else {{\n{indent}        let slice = if result.{len_field} == 0 {{\n{indent}            &[]\n{indent}        }} else {{\n{indent}            slice::from_raw_parts(result.{ptr_field} as *const u8, result.{len_field} as usize)\n{indent}        }};\n{indent}        Some(String::from_utf8_lossy(slice).into_owned())\n{indent}    }}\n{indent}}}",
            indent = indent,
            ptr_field = ptr_field,
            len_field = len_field,
        ),
        ReturnFieldType::Array { .. } => format!("{indent}result.{}", field.name),
        ReturnFieldType::Vec {
            ptr_field,
            len_field,
            elem,
            mutable_ptr,
        } => {
            let ptr_expr = if *mutable_ptr {
                format!(
                    "result.{ptr_field} as *const {ty}",
                    ptr_field = ptr_field,
                    ty = type_ref_to_string(elem)
                )
            } else {
                format!("result.{}", ptr_field)
            };
            format!(
                "{indent}{{\n{indent}    let slice = if result.{len_field} == 0 || result.{ptr_field}.is_null() {{\n{indent}        &[]\n{indent}    }} else {{\n{indent}        slice::from_raw_parts({ptr_expr}, result.{len_field} as usize)\n{indent}    }};\n{indent}    slice.to_vec()\n{indent}}}",
                indent = indent,
                len_field = len_field,
                ptr_field = ptr_field,
                ptr_expr = ptr_expr,
            )
        }
        ReturnFieldType::StringVec {
            ptr_field,
            len_field,
        } => format!(
            "{indent}{{\n{indent}    if result.{len_field} == 0 || result.{ptr_field}.is_null() {{\n{indent}        Vec::new()\n{indent}    }} else {{\n{indent}        let slice = slice::from_raw_parts(result.{ptr_field}, result.{len_field} as usize);\n{indent}        slice.iter().map(|&ptr| {{\n{indent}            if ptr.is_null() {{\n{indent}                String::new()\n{indent}            }} else {{\n{indent}                CStr::from_ptr(ptr).to_string_lossy().into_owned()\n{indent}            }}\n{indent}        }}).collect()\n{indent}    }}\n{indent}}}",
            indent = indent,
            ptr_field = ptr_field,
            len_field = len_field,
        ),
    }
}

fn build_params(query: &StructDef) -> Result<(Vec<ParamSpec>, Vec<QueryInitField>)> {
    let mut params = Vec::new();
    let mut inits = Vec::new();
    let mut i = 0;
    while i < query.fields.len() {
        let field = &query.fields[i];
        // RmlUi persistent listeners are owned by the host. Their callback
        // payload needs an explicit host-side destructor when the listener is
        // detached, rather than another Rust closure parameter.
        if field.name == "destroyCallback" && matches!(field.ty, CType::FnPtr) {
            inits.push(QueryInitField {
                field: make_field_name(&field.name),
                expr: QueryExpr::CallbackDestroyFn,
            });
            i += 1;
            continue;
        }
        if let Some(next) = query.fields.get(i + 1) {
            if is_optional_presence_pair(field, next) {
                let param_name = make_param_name(&field.name);
                match &field.ty {
                    CType::Primitive(prim) => {
                        params.push(ParamSpec {
                            name: param_name.clone(),
                            ty: ParamType::OptionPrimitive(prim.rust_type()),
                        });
                        inits.push(QueryInitField {
                            field: make_field_name(&field.name),
                            expr: QueryExpr::OptionParam {
                                param: param_name.clone(),
                                default_expr: primitive_default_expr(*prim),
                            },
                        });
                        inits.push(QueryInitField {
                            field: make_field_name(&next.name),
                            expr: QueryExpr::OptionIsSome { param: param_name },
                        });
                        i += 2;
                        continue;
                    }
                    CType::Pointer { pointee, is_const }
                        if *is_const && matches!(**pointee, CType::Primitive(Primitive::Char)) =>
                    {
                        params.push(ParamSpec {
                            name: param_name.clone(),
                            ty: ParamType::OptionCStr,
                        });
                        inits.push(QueryInitField {
                            field: make_field_name(&field.name),
                            expr: QueryExpr::OptionCStrPtr {
                                param: param_name.clone(),
                            },
                        });
                        inits.push(QueryInitField {
                            field: make_field_name(&next.name),
                            expr: QueryExpr::OptionIsSome { param: param_name },
                        });
                        i += 2;
                        continue;
                    }
                    _ => {}
                }
            }
        }
        match &field.ty {
            // A function-pointer field followed by a void* userData field is a
            // native edit callback; surface it as one FnMut closure param.
            CType::FnPtr => {
                let next_is_userdata = i + 1 < query.fields.len()
                    && matches!(query.fields[i + 1].ty, CType::Pointer { .. });
                if !next_is_userdata {
                    return Err(anyhow!(
                        "callback field {} must be followed by a void* userData field",
                        field.name
                    ));
                }
                let param_name = make_param_name(&field.name);
                params.push(ParamSpec {
                    name: param_name.clone(),
                    ty: ParamType::Callback,
                });
                inits.push(QueryInitField {
                    field: make_field_name(&field.name),
                    expr: QueryExpr::CallbackFn,
                });
                inits.push(QueryInitField {
                    field: make_field_name(&query.fields[i + 1].name),
                    expr: QueryExpr::CallbackUserData { param: param_name },
                });
                i += 2;
            }
            CType::Primitive(Primitive::U8) if field.name.starts_with('_') => {
                inits.push(QueryInitField {
                    field: make_field_name(&field.name),
                    expr: QueryExpr::Zero,
                });
                i += 1;
            }
            CType::Pointer { pointee, is_const } => {
                // Special case: const char* is a C string parameter
                if *is_const && matches!(**pointee, CType::Primitive(Primitive::Char)) {
                    let param_name = make_param_name(&field.name);
                    params.push(ParamSpec {
                        name: param_name.clone(),
                        ty: ParamType::CStr,
                    });
                    inits.push(QueryInitField {
                        field: make_field_name(&field.name),
                        expr: QueryExpr::CStrPtr { param: param_name },
                    });
                    i += 1;
                    continue;
                }

                // Check if the next field is a count
                if i + 1 < query.fields.len()
                    && matches!(query.fields[i + 1].ty, CType::Primitive(Primitive::U32))
                {
                    // Special case: const char** + length should be a raw pointer (for Memory::Free* functions)
                    if let CType::Pointer {
                        pointee: inner_pointee,
                        is_const: inner_const,
                    } = &**pointee
                    {
                        if *inner_const
                            && matches!(**inner_pointee, CType::Primitive(Primitive::Char))
                        {
                            let param_name = make_param_name(&field.name);
                            params.push(ParamSpec {
                                name: param_name.clone(),
                                ty: ParamType::Primitive("*mut *const i8"),
                            });
                            inits.push(QueryInitField {
                                field: make_field_name(&field.name),
                                expr: QueryExpr::Param(param_name.clone()),
                            });
                            // Also add the length field
                            let len_param_name = make_param_name(&query.fields[i + 1].name);
                            params.push(ParamSpec {
                                name: len_param_name.clone(),
                                ty: ParamType::Primitive("u32"),
                            });
                            inits.push(QueryInitField {
                                field: make_field_name(&query.fields[i + 1].name),
                                expr: QueryExpr::Param(len_param_name),
                            });
                            i += 2;
                            continue;
                        }
                    }

                    let elem = rust_type_from_pointer(pointee)
                        .ok_or_else(|| anyhow!("unsupported pointer field {}", field.name))?;
                    let param_name = make_param_name(&field.name);
                    params.push(ParamSpec {
                        name: param_name.clone(),
                        ty: ParamType::Slice {
                            element: elem.clone(),
                        },
                    });
                    inits.push(QueryInitField {
                        field: make_field_name(&field.name),
                        expr: QueryExpr::SlicePtr {
                            param: param_name.clone(),
                            mutable_ptr: !is_const,
                        },
                    });
                    inits.push(QueryInitField {
                        field: make_field_name(&query.fields[i + 1].name),
                        expr: QueryExpr::SliceLen {
                            param: param_name,
                            cast: "u32",
                        },
                    });
                    i += 2;
                    continue;
                }

                // void* or unknown pointer types - use raw pointer
                if matches!(
                    **pointee,
                    CType::Primitive(Primitive::Void) | CType::Unknown(_)
                ) {
                    let param_name = make_param_name(&field.name);
                    let ptr_type = if *is_const {
                        "*const std::ffi::c_void"
                    } else {
                        "*mut std::ffi::c_void"
                    };
                    params.push(ParamSpec {
                        name: param_name.clone(),
                        ty: ParamType::Primitive(ptr_type),
                    });
                    inits.push(QueryInitField {
                        field: make_field_name(&field.name),
                        expr: QueryExpr::Param(param_name),
                    });
                    i += 1;
                    continue;
                }

                // Single mutable pointer without count - treat as &mut T
                if !is_const {
                    if let Some(elem) = rust_type_from_pointer(pointee) {
                        let param_name = make_param_name(&field.name);
                        params.push(ParamSpec {
                            name: param_name.clone(),
                            ty: ParamType::MutRef {
                                element: elem.clone(),
                            },
                        });
                        inits.push(QueryInitField {
                            field: make_field_name(&field.name),
                            expr: QueryExpr::MutRefPtr { param: param_name },
                        });
                        i += 1;
                        continue;
                    }
                }

                // Single const pointer without count - treat as &T
                if *is_const {
                    if let Some(elem) = rust_type_from_pointer(pointee) {
                        let param_name = make_param_name(&field.name);
                        params.push(ParamSpec {
                            name: param_name.clone(),
                            ty: ParamType::Ref {
                                element: elem.clone(),
                            },
                        });
                        inits.push(QueryInitField {
                            field: make_field_name(&field.name),
                            expr: QueryExpr::RefPtr { param: param_name },
                        });
                        i += 1;
                        continue;
                    }
                }

                return Err(anyhow!("pointer field {} missing length pair", field.name));
            }
            CType::Array { element, length } => {
                // Fixed-size arrays like float matrix[16]
                if let CType::Primitive(prim) = element.as_ref() {
                    let param_name = make_param_name(&field.name);
                    params.push(ParamSpec {
                        name: param_name.clone(),
                        ty: ParamType::Array {
                            element: prim.rust_type(),
                            length: *length,
                        },
                    });
                    inits.push(QueryInitField {
                        field: make_field_name(&field.name),
                        expr: QueryExpr::Param(param_name),
                    });
                    i += 1;
                    continue;
                }
                return Err(anyhow!(
                    "unsupported array element type in field {}",
                    field.name
                ));
            }
            _ => {
                if let Some(ty) = rust_type_from_c(&field.ty) {
                    let param_name = make_param_name(&field.name);
                    params.push(ParamSpec {
                        name: param_name.clone(),
                        ty: match &ty {
                            TypeRef::Primitive(name) => ParamType::Primitive(name),
                            TypeRef::Struct(name) if name == "RulesParamValue" => {
                                ParamType::RulesParamValue
                            }
                            TypeRef::Struct(name) if name.ends_with("Options") => {
                                ParamType::Options(name.clone())
                            }
                            TypeRef::Struct(name) => ParamType::Struct(name.clone()),
                        },
                    });
                    inits.push(QueryInitField {
                        field: make_field_name(&field.name),
                        expr: match &ty {
                            TypeRef::Struct(name) if name == "RulesParamValue" => {
                                QueryExpr::RulesParamValue { param: param_name }
                            }
                            TypeRef::Struct(name) if name.ends_with("Options") => {
                                QueryExpr::Options { param: param_name }
                            }
                            _ => QueryExpr::Param(param_name),
                        },
                    });
                }
                i += 1;
            }
        }
    }
    Ok((params, inits))
}

fn is_optional_presence_pair(value_field: &FieldDef, has_field: &FieldDef) -> bool {
    if !matches!(has_field.ty, CType::Primitive(Primitive::Bool)) {
        return false;
    }

    let Some(has_suffix) = has_field.name.strip_prefix("has") else {
        return false;
    };

    has_suffix.eq_ignore_ascii_case(&value_field.name)
        || value_field
            .name
            .strip_suffix("Value")
            .is_some_and(|prefix| has_suffix.eq_ignore_ascii_case(prefix))
}

fn primitive_default_expr(prim: Primitive) -> &'static str {
    match prim {
        Primitive::F32 => "0.0",
        Primitive::Bool => "false",
        _ => "0",
    }
}

fn build_return(result: &StructDef) -> Result<ReturnKind> {
    let data_fields: Vec<&FieldDef> = result.fields.iter().filter(|f| f.name != "error").collect();
    if data_fields.is_empty() {
        return Ok(ReturnKind::Unit);
    }
    let mut fields = Vec::new();
    let mut i = 0;
    while i < data_fields.len() {
        let field = data_fields[i];
        if let CType::Pointer {
            pointee,
            is_const: outer_const,
        } = &field.ty
        {
            if i + 1 < data_fields.len()
                && matches!(data_fields[i + 1].ty, CType::Primitive(Primitive::U32))
            {
                // Special case: const char** + length (array of strings)
                if let CType::Pointer {
                    pointee: inner_pointee,
                    is_const: inner_const,
                } = &**pointee
                {
                    if *inner_const && matches!(**inner_pointee, CType::Primitive(Primitive::Char))
                    {
                        fields.push(ReturnField {
                            name: make_field_name(&field.name),
                            ty: ReturnFieldType::StringVec {
                                ptr_field: make_field_name(&field.name),
                                len_field: make_field_name(&data_fields[i + 1].name),
                            },
                        });
                        i += 2;
                        continue;
                    }
                }

                // Special case: const char* + length (string that may contain embedded NUL bytes)
                if *outer_const && matches!(**pointee, CType::Primitive(Primitive::Char)) {
                    fields.push(ReturnField {
                        name: make_field_name(&field.name),
                        ty: ReturnFieldType::StringWithLen {
                            ptr_field: make_field_name(&field.name),
                            len_field: make_field_name(&data_fields[i + 1].name),
                        },
                    });
                    i += 2;
                    continue;
                }

                let elem = rust_type_from_pointer(pointee)
                    .ok_or_else(|| anyhow!("unsupported result pointer {}", field.name))?;
                fields.push(ReturnField {
                    name: make_field_name(&field.name),
                    ty: ReturnFieldType::Vec {
                        ptr_field: make_field_name(&field.name),
                        len_field: make_field_name(&data_fields[i + 1].name),
                        elem,
                        mutable_ptr: !outer_const,
                    },
                });
                i += 2;
                continue;
            }
        }

        let ty = match return_field_type(&field.ty) {
            Some(ty) => ty,
            None => return Err(anyhow!("unsupported result field {}", field.name)),
        };
        fields.push(ReturnField {
            name: make_field_name(&field.name),
            ty,
        });
        i += 1;
    }

    Ok(ReturnKind::Fields(fields))
}

fn return_field_type(ty: &CType) -> Option<ReturnFieldType> {
    match ty {
        CType::Primitive(p) => Some(ReturnFieldType::Plain(TypeRef::Primitive(p.rust_type()))),
        CType::Enum(name) => Some(ReturnFieldType::Plain(TypeRef::Struct(name.clone()))),
        CType::Record(name) => Some(ReturnFieldType::Plain(TypeRef::Struct(name.clone()))),
        CType::Pointer { pointee, is_const } => {
            if *is_const {
                if let CType::Primitive(Primitive::Char) = **pointee {
                    return Some(ReturnFieldType::CString);
                }
            }
            None
        }
        CType::Array { element, length } => {
            let elem_type = match &**element {
                CType::Primitive(p) => p.rust_type().to_string(),
                CType::Enum(name) => name.clone(),
                CType::Record(name) => name.clone(),
                _ => return None,
            };
            Some(ReturnFieldType::Array {
                element: elem_type,
                length: *length,
            })
        }
        _ => None,
    }
}

fn rust_type_from_c(ty: &CType) -> Option<TypeRef> {
    match ty {
        CType::Primitive(p) => Some(TypeRef::Primitive(p.rust_type())),
        CType::Enum(name) => Some(TypeRef::Struct(name.clone())),
        CType::Record(name) => Some(TypeRef::Struct(name.clone())),
        _ => None,
    }
}

fn rust_type_from_pointer(ty: &CType) -> Option<TypeRef> {
    match ty {
        CType::Primitive(p) => Some(TypeRef::Primitive(p.rust_type())),
        CType::Enum(name) => Some(TypeRef::Struct(name.clone())),
        CType::Record(name) => Some(TypeRef::Struct(name.clone())),
        _ => None,
    }
}

fn type_ref_to_string(ty: &TypeRef) -> String {
    match ty {
        TypeRef::Primitive(name) => name.to_string(),
        TypeRef::Struct(name) if name == "RulesParamValue" => "RulesParamValue".to_string(),
        TypeRef::Struct(name) => format!("sys::{}", name),
    }
}

fn make_param_name(name: &str) -> String {
    let normalized = normalize_acronyms(name);
    let snake = normalized.to_snake_case();
    let collapsed = collapse_acronyms(&snake).replace("_a_is", "_ais");
    let base = if collapsed.is_empty() {
        normalized
    } else {
        collapsed
    };
    sanitize_ident(&base)
}

fn make_field_name(name: &str) -> String {
    if is_rust_keyword(name) {
        format!("{}_", name)
    } else {
        name.to_string()
    }
}

fn sanitize_ident(name: &str) -> String {
    if is_rust_keyword(name) {
        format!("r#{}", name)
    } else {
        name.to_string()
    }
}

fn is_rust_keyword(name: &str) -> bool {
    matches!(
        name,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "dyn"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "macro"
            | "override"
            | "priv"
            | "try"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
    )
}

fn normalize_acronyms(raw: &str) -> String {
    let mut result = String::new();
    let mut chars = raw.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch.is_ascii_uppercase() {
            let mut run = vec![ch];
            while let Some(&next) = chars.peek() {
                if next.is_ascii_uppercase() {
                    run.push(next);
                    chars.next();
                } else {
                    break;
                }
            }
            if run.len() > 1 {
                result.push(run[0]);
                for c in run.iter().skip(1) {
                    result.push(c.to_ascii_lowercase());
                }
            } else {
                result.push(ch);
            }
        } else {
            result.push(ch);
        }
    }
    result
}

fn collapse_acronyms(input: &str) -> String {
    let mut parts = input.split('_').peekable();
    let mut result = Vec::new();
    while let Some(part) = parts.next() {
        if part.len() == 1 && part.chars().all(|c| c.is_ascii_lowercase()) {
            let mut run = vec![part.to_string()];
            while let Some(next) = parts.peek() {
                if next.len() == 1 && next.chars().all(|c| c.is_ascii_lowercase()) {
                    run.push(parts.next().unwrap().to_string());
                } else {
                    break;
                }
            }
            if run.len() == 1 {
                result.push(run.into_iter().next().unwrap());
            } else {
                let mut collapsed = String::new();
                for token in run {
                    collapsed.push_str(&token);
                }
                result.push(collapsed);
            }
        } else {
            result.push(part.to_string());
        }
    }
    result.join("_")
}

/// Extract the API version from Common.h
///
/// Parses the NATIVE_API_CURRENT_VERSION macro and returns (major, minor, patch).
/// Example: NATIVE_API_VERSION(1, 2, 3) -> (1, 2, 3)
pub fn extract_api_version(common_header: &Path) -> Result<(u32, u32, u32)> {
    use std::fs;

    let content = fs::read_to_string(common_header)
        .with_context(|| format!("Failed to read {}", common_header.display()))?;

    // Find: #define NATIVE_API_CURRENT_VERSION NATIVE_API_VERSION(major, minor, patch)
    let current_version_re = regex::Regex::new(
        r"#define\s+NATIVE_API_CURRENT_VERSION\s+NATIVE_API_VERSION\s*\(\s*(\d+)\s*,\s*(\d+)\s*,\s*(\d+)\s*\)"
    ).unwrap();

    if let Some(caps) = current_version_re.captures(&content) {
        let major: u32 = caps.get(1).unwrap().as_str().parse()?;
        let minor: u32 = caps.get(2).unwrap().as_str().parse()?;
        let patch: u32 = caps.get(3).unwrap().as_str().parse()?;

        Ok((major, minor, patch))
    } else {
        Err(anyhow!(
            "Could not find NATIVE_API_CURRENT_VERSION in {}",
            common_header.display()
        ))
    }
}

#[cfg(test)]
mod semantic_codegen_tests {
    use super::*;
    use crate::model::SemanticType;
    use std::fs;

    #[test]
    fn synthetic_header_covers_all_phase_zero_shapes() {
        let path = std::env::temp_dir().join(format!(
            "recoil-wasm-semantic-{}-{}.h",
            std::process::id(),
            std::thread::current().name().unwrap_or("test")
        ));
        fs::write(
            &path,
            r#"
#include <stdint.h>
struct NestedRecord { int32_t value; };
struct ResourceHandle { uint64_t value; };
enum SyntheticKind { SYNTHETIC_A = 1, SYNTHETIC_B = 2 };
typedef void (*SyntheticCallback)(int32_t);
struct SyntheticQuery {
    const char* label;
    const int32_t* values;
    uint32_t valueCount;
    int32_t optionalValue;
    bool hasOptionalValue;
    NestedRecord nested;
    SyntheticKind kind;
    float fixed[3];
    ResourceHandle resource;
    SyntheticCallback callback;
};
struct SyntheticResult { const char* message; };
struct SyntheticApi {
    void (*Sample)(const SyntheticQuery*, SyntheticResult*);
};
"#,
        )
        .unwrap();

        let generator = CodeGenerator::new().unwrap();
        let module = generator
            .semantic_module(&path, &[], "SyntheticApi", "synthetic")
            .unwrap();
        let function = &module.functions[0];

        assert!(matches!(function.inputs[0].ty, SemanticType::String));
        assert!(matches!(function.inputs[1].ty, SemanticType::List { .. }));
        assert!(matches!(function.inputs[2].ty, SemanticType::Option { .. }));
        assert!(matches!(function.inputs[3].ty, SemanticType::Record { .. }));
        assert!(matches!(function.inputs[4].ty, SemanticType::Enum { .. }));
        assert!(matches!(
            function.inputs[5].ty,
            SemanticType::FixedArray { .. }
        ));
        assert!(matches!(function.inputs[6].ty, SemanticType::Handle { .. }));
        assert!(matches!(
            function.inputs[7].ty,
            SemanticType::Callback { .. }
        ));
        assert!(matches!(function.outputs[0].ty, SemanticType::String));

        let _ = fs::remove_file(path);
    }
}
