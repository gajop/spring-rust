use anyhow::{anyhow, Context, Result};
use clang::{Clang, Entity, EntityKind, Index, Type, TypeKind};
use heck::ToSnakeCase;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub struct ApiConfig<'a> {
    pub api_struct: &'a str,
    pub wrapper_struct: &'a str,
}

pub fn generate_units_query(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "UnitsQueryApi",
            wrapper_struct: "UnitsQuery",
        },
    )
}

pub fn generate_units_info(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "UnitsInfoApi",
            wrapper_struct: "UnitsInfo",
        },
    )
}

pub fn generate_teams(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "TeamsApi",
            wrapper_struct: "Teams",
        },
    )
}

pub fn generate_units_weapons(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "UnitsWeaponsApi",
            wrapper_struct: "UnitsWeapons",
        },
    )
}

pub fn generate_units_commands(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "UnitsCommandsApi",
            wrapper_struct: "UnitsCommands",
        },
    )
}

pub fn generate_units_pieces(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "UnitsPiecesApi",
            wrapper_struct: "UnitsPieces",
        },
    )
}

pub fn generate_features(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "FeaturesApi",
            wrapper_struct: "Features",
        },
    )
}

pub fn generate_projectiles(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "ProjectilesApi",
            wrapper_struct: "Projectiles",
        },
    )
}

pub fn generate_los(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "LOSApi",
            wrapper_struct: "Los",
        },
    )
}

pub fn generate_unit_defs(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "UnitDefsApi",
            wrapper_struct: "UnitDefs",
        },
    )
}

pub fn generate_feature_defs(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "FeatureDefsApi",
            wrapper_struct: "FeatureDefs",
        },
    )
}

pub fn generate_weapon_defs(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "WeaponDefsApi",
            wrapper_struct: "WeaponDefs",
        },
    )
}

pub fn generate_game(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "GameApi",
            wrapper_struct: "Game",
        },
    )
}

pub fn generate_terrain(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "TerrainApi",
            wrapper_struct: "Terrain",
        },
    )
}

pub fn generate_player(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "PlayerApi",
            wrapper_struct: "Player",
        },
    )
}

pub fn generate_math_extra(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "MathExtraApi",
            wrapper_struct: "MathExtra",
        },
    )
}

pub fn generate_metal_map(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "MetalMapApi",
            wrapper_struct: "MetalMap",
        },
    )
}

pub fn generate_path_finder(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "PathFinderApi",
            wrapper_struct: "PathFinder",
        },
    )
}

pub fn generate_rules_params(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "RulesParamsApi",
            wrapper_struct: "RulesParams",
        },
    )
}

pub fn generate_move_ctrl(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "MoveCtrlApi",
            wrapper_struct: "MoveCtrl",
        },
    )
}

pub fn generate_synced_ctrl(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "SyncedCtrlApi",
            wrapper_struct: "SyncedCtrl",
        },
    )
}

pub fn generate_camera(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "CameraApi",
            wrapper_struct: "Camera",
        },
    )
}

pub fn generate_input(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "InputApi",
            wrapper_struct: "Input",
        },
    )
}

pub fn generate_display(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "DisplayApi",
            wrapper_struct: "Display",
        },
    )
}

pub fn generate_selection(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "SelectionApi",
            wrapper_struct: "Selection",
        },
    )
}

pub fn generate_vfs(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "VFSApi",
            wrapper_struct: "Vfs",
        },
    )
}

pub fn generate_sound(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "SoundApi",
            wrapper_struct: "Sound",
        },
    )
}

pub fn generate_messages(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "MessagesApi",
            wrapper_struct: "Messages",
        },
    )
}

pub fn generate_config(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "ConfigApi",
            wrapper_struct: "Config",
        },
    )
}

pub fn generate_tracing(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "TracingApi",
            wrapper_struct: "Tracing",
        },
    )
}

pub fn generate_utils(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "UtilsApi",
            wrapper_struct: "Utils",
        },
    )
}

pub fn generate_memory(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "MemoryApi",
            wrapper_struct: "Memory",
        },
    )
}

// SyncedCtrl sub-APIs
pub fn generate_team_control(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "TeamControlApi",
            wrapper_struct: "TeamControl",
        },
    )
}

pub fn generate_unit_control(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "UnitControlApi",
            wrapper_struct: "UnitControl",
        },
    )
}

pub fn generate_feature_control(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "FeatureControlApi",
            wrapper_struct: "FeatureControl",
        },
    )
}

pub fn generate_terrain_control(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "TerrainControlApi",
            wrapper_struct: "TerrainControl",
        },
    )
}

pub fn generate_projectile_control(header: &Path, include_dirs: &[PathBuf]) -> Result<String> {
    generate_api(
        header,
        include_dirs,
        ApiConfig {
            api_struct: "ProjectileControlApi",
            wrapper_struct: "ProjectileControl",
        },
    )
}

fn generate_api(header: &Path, include_dirs: &[PathBuf], config: ApiConfig<'_>) -> Result<String> {
    let spec = parse_api(header, include_dirs, config.api_struct)?;
    render_api(&spec, &config)
}

#[derive(Debug, Clone)]
struct ApiSpec {
    structs: HashMap<String, StructDef>,
    api: ApiDef,
}

#[derive(Debug, Clone)]
struct StructDef {
    name: String,
    fields: Vec<FieldDef>,
}

#[derive(Debug, Clone)]
struct FieldDef {
    name: String,
    ty: CType,
}

#[derive(Debug, Clone)]
enum CType {
    Primitive(Primitive),
    Record(String),
    Pointer { pointee: Box<CType>, is_const: bool },
    #[allow(dead_code)]
    Array { element: Box<CType>, length: u64 },
    #[allow(dead_code)]
    Unknown(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Primitive {
    I32,
    U32,
    U8,
    Bool,
    F32,
    Char,
    Void,
}

impl Primitive {
    fn rust_type(self) -> &'static str {
        match self {
            Primitive::I32 => "i32",
            Primitive::U32 => "u32",
            Primitive::U8 => "u8",
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

fn parse_api(header: &Path, include_dirs: &[PathBuf], api_struct: &str) -> Result<ApiSpec> {
    let clang = Clang::new().map_err(|e| anyhow!(e))?;
    let index = Index::new(&clang, false, false);
    let mut parser = index.parser(header);
    let mut args = vec!["-xc++".to_string(), "-std=c++17".to_string()];
    for dir in include_dirs {
        args.push(format!("-I{}", dir.display()));
    }
    parser.arguments(&args);
    let tu = parser
        .parse()
        .with_context(|| format!("failed to parse {}", header.display()))?;

    let mut structs = HashMap::new();
    let mut api = None;
    visit_entity(tu.get_entity(), api_struct, &mut structs, &mut api);
    let api = api.context(format!("{} not found", api_struct))?;
    Ok(ApiSpec { structs, api })
}

fn visit_entity(
    entity: Entity,
    api_name: &str,
    structs: &mut HashMap<String, StructDef>,
    api: &mut Option<ApiDef>,
) {
    match entity.get_kind() {
        EntityKind::StructDecl => {
            if let Some(name) = entity.get_name() {
                if entity.is_definition() {
                    if name == api_name {
                        *api = Some(parse_api_struct(entity));
                    } else if is_query_struct(&name) {
                        structs.insert(name.clone(), parse_struct(entity));
                    }
                }
            }
        }
        _ => {}
    }

    entity.visit_children(|child, _| {
        visit_entity(child, api_name, structs, api);
        clang::EntityVisitResult::Continue
    });
}

fn is_query_struct(name: &str) -> bool {
    name.ends_with("Query")
        || name.ends_with("Result")
        || name.ends_with("Params")
        || name.ends_with("Count")
}

fn parse_struct(entity: Entity) -> StructDef {
    let name = entity.get_name().unwrap_or_default();
    let mut fields = Vec::new();
    entity.visit_children(|child, _| {
        if child.get_kind() == EntityKind::FieldDecl {
            if let (Some(field_name), Some(field_type)) = (child.get_name(), child.get_type()) {
                fields.push(FieldDef {
                    name: field_name,
                    ty: classify_type(field_type),
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
        TypeKind::UInt => CType::Primitive(Primitive::U32),
        TypeKind::UChar => CType::Primitive(Primitive::U8),
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
        TypeKind::ConstantArray => {
            let element = ty
                .get_element_type()
                .map(classify_type)
                .unwrap_or(CType::Unknown("unknown".into()));
            let len = ty.get_sizeof().unwrap_or(0) as u64;
            CType::Array {
                element: Box::new(element),
                length: len,
            }
        }
        _ => CType::Unknown(ty.get_display_name()),
    }
}

fn primitive_from_name(name: &str) -> Option<Primitive> {
    match name {
        "int32_t" | "int" => Some(Primitive::I32),
        "uint32_t" | "unsigned int" => Some(Primitive::U32),
        "uint8_t" | "unsigned char" => Some(Primitive::U8),
        "bool" => Some(Primitive::Bool),
        "float" => Some(Primitive::F32),
        "char" | "const char" | "signed char" => Some(Primitive::Char),
        "void" => Some(Primitive::Void),
        _ => None,
    }
}

fn render_api(spec: &ApiSpec, config: &ApiConfig<'_>) -> Result<String> {
    let mut out = String::new();
    out.push_str(&format!("impl<'a> {}<'a> {{\n", config.wrapper_struct));
    for func in &spec.api.functions {
        if let (Some(query), Some(result)) = (
            spec.structs.get(&func.query),
            spec.structs.get(&func.result),
        ) {
            let method = render_method(func, query, result)?;
            out.push_str(&method);
            out.push('\n');
        }
    }
    out.push_str("}\n");
    Ok(out)
}

#[derive(Debug)]
struct ParamSpec {
    name: String,
    ty: ParamType,
}

#[derive(Debug)]
enum ParamType {
    Primitive(&'static str),
    Struct(String),
    Slice { element: TypeRef },
    MutRef { element: TypeRef },
    CStr,
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
    SlicePtr { param: String, mutable_ptr: bool },
    SliceLen { param: String, cast: &'static str },
    MutRefPtr { param: String },
    CStrPtr { param: String },
    Zero,
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
}

impl ReturnFieldType {
    fn ty_string(&self) -> String {
        match self {
            ReturnFieldType::Plain(ty) => type_ref_to_string(ty),
            ReturnFieldType::CString => "Option<String>".into(),
        }
    }
}

#[derive(Debug)]
enum ReturnKind {
    Scalar(ReturnField),
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
    Tuple(Vec<ReturnField>),
    Unit,
}

fn render_method(func: &ApiFunction, query: &StructDef, result: &StructDef) -> Result<String> {
    let method_name = normalize_acronyms(&func.name).to_snake_case();
    let (params, inits) = build_params(query)?;
    let ret_kind = build_return(result)?;

    let mut sig = format!("    pub fn {}(&self", method_name);
    for param in &params {
        let ty = match &param.ty {
            ParamType::Primitive(name) => name.to_string(),
            ParamType::Struct(struct_name) => format!("sys::{}", struct_name),
            ParamType::Slice { element } => format!("&[{}]", type_ref_to_string(element)),
            ParamType::MutRef { element } => format!("&mut {}", type_ref_to_string(element)),
            ParamType::CStr => "&str".to_string(),
        };
        sig.push_str(&format!(", {}: {}", param.name, ty));
    }
    sig.push_str(") -> Result<");
    sig.push_str(
        match &ret_kind {
            ReturnKind::Scalar(field) => field.ty.ty_string(),
            ReturnKind::Vec { elem, .. } => format!("Vec<{}>", type_ref_to_string(elem)),
            ReturnKind::StringVec { .. } => "Vec<String>".to_string(),
            ReturnKind::Tuple(fields) => {
                let parts: Vec<_> = fields.iter().map(|f| f.ty.ty_string()).collect();
                format!("({})", parts.join(", "))
            }
            ReturnKind::Unit => "()".to_string(),
        }
        .as_str(),
    );
    sig.push_str(", Error> {\n");

    let mut body = String::new();
    body.push_str("        unsafe {\n");

    // Generate CStr conversions for string parameters
    let mut needs_cstr = false;
    for field in &inits {
        if matches!(field.expr, QueryExpr::CStrPtr { .. }) {
            needs_cstr = true;
            break;
        }
    }

    if needs_cstr {
        for field in &inits {
            if let QueryExpr::CStrPtr { param } = &field.expr {
                body.push_str(&format!("            let {}_cstr = std::ffi::CString::new({}).map_err(|_| Error::invalid_argument(\"{}\"))?;\n", param, param, param));
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
            QueryExpr::MutRefPtr { param } => format!("{} as *mut _", param),
            QueryExpr::CStrPtr { param } => format!("{}_cstr.as_ptr()", param),
            QueryExpr::Zero => "0".into(),
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
    body.push_str(&render_return(&ret_kind));
    body.push_str("        }\n");
    body.push_str("    }\n");

    Ok(format!("{}{}", sig, body))
}

fn render_return(kind: &ReturnKind) -> String {
    match kind {
        ReturnKind::Scalar(field) => {
            let expr = field_expr(field, "                ");
            format!(
                "            Error::result_or(result.error, {{\n{expr}\n            }})\n",
                expr = expr
            )
        }
        ReturnKind::Vec {
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
                "            let slice = if result.{len_field} == 0 || result.{ptr_field}.is_null() {{\n                &[]\n            }} else {{\n                slice::from_raw_parts({ptr_expr}, result.{len_field} as usize)\n            }};\n            Error::result_or(result.error, slice.to_vec())\n",
                len_field = len_field,
                ptr_field = ptr_field,
                ptr_expr = ptr_expr,
            )
        }
        ReturnKind::StringVec { ptr_field, len_field } => {
            format!(
                "            let vec = if result.{len_field} == 0 || result.{ptr_field}.is_null() {{\n                Vec::new()\n            }} else {{\n                let slice = slice::from_raw_parts(result.{ptr_field}, result.{len_field} as usize);\n                slice.iter().map(|&ptr| {{\n                    if ptr.is_null() {{\n                        String::new()\n                    }} else {{\n                        CStr::from_ptr(ptr).to_string_lossy().into_owned()\n                    }}\n                }}).collect()\n            }};\n            Error::result_or(result.error, vec)\n",
                ptr_field = ptr_field,
                len_field = len_field,
            )
        }
        ReturnKind::Tuple(fields) => {
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
        ReturnKind::Unit => "            Error::result_or(result.error, ())\n".into(),
    }
}

fn field_expr(field: &ReturnField, indent: &str) -> String {
    match &field.ty {
        ReturnFieldType::Plain(_) => format!("{indent}result.{}", field.name),
        ReturnFieldType::CString => format!(
            "{indent}{{\n{indent}    if result.{name}.is_null() {{\n{indent}        None\n{indent}    }} else {{\n{indent}        Some(CStr::from_ptr(result.{name}).to_string_lossy().into_owned())\n{indent}    }}\n{indent}}}",
            indent = indent,
            name = field.name
        ),
    }
}

fn build_params(query: &StructDef) -> Result<(Vec<ParamSpec>, Vec<QueryInitField>)> {
    let mut params = Vec::new();
    let mut inits = Vec::new();
    let mut i = 0;
    while i < query.fields.len() {
        let field = &query.fields[i];
        match &field.ty {
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
                        expr: QueryExpr::CStrPtr {
                            param: param_name,
                        },
                    });
                    i += 1;
                    continue;
                }

                // Check if the next field is a count
                if i + 1 < query.fields.len() {
                    if matches!(query.fields[i + 1].ty, CType::Primitive(Primitive::U32)) {
                        // Special case: const char** + length should be a raw pointer (for Memory::Free* functions)
                        if let CType::Pointer { pointee: inner_pointee, is_const: inner_const } = &**pointee {
                            if *inner_const && matches!(**inner_pointee, CType::Primitive(Primitive::Char)) {
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
                }

                // void* or unknown pointer types - use raw pointer
                if matches!(**pointee, CType::Primitive(Primitive::Void) | CType::Unknown(_)) {
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
                            expr: QueryExpr::MutRefPtr {
                                param: param_name,
                            },
                        });
                        i += 1;
                        continue;
                    }
                }

                return Err(anyhow!("pointer field {} missing length pair", field.name));
            }
            _ => {
                if let Some(ty) = rust_type_from_c(&field.ty) {
                    let param_name = make_param_name(&field.name);
                    params.push(ParamSpec {
                        name: param_name.clone(),
                        ty: match ty {
                            TypeRef::Primitive(name) => ParamType::Primitive(name),
                            TypeRef::Struct(ref name) => ParamType::Struct(name.clone()),
                        },
                    });
                    inits.push(QueryInitField {
                        field: make_field_name(&field.name),
                        expr: QueryExpr::Param(param_name),
                    });
                }
                i += 1;
            }
        }
    }
    Ok((params, inits))
}

fn build_return(result: &StructDef) -> Result<ReturnKind> {
    let data_fields: Vec<&FieldDef> = result.fields.iter().filter(|f| f.name != "error").collect();
    if data_fields.is_empty() {
        return Ok(ReturnKind::Unit);
    }
    let first = data_fields[0];
    if let CType::Pointer { pointee, is_const: outer_const } = &first.ty {
        if data_fields.len() >= 2 && matches!(data_fields[1].ty, CType::Primitive(Primitive::U32)) {
            // Check for const char** (array of strings)
            // Note: In C, "const char**" means the outer pointer is NOT const, but points to const char*
            if let CType::Pointer { pointee: inner_pointee, is_const: inner_const } = &**pointee {
                if *inner_const && matches!(**inner_pointee, CType::Primitive(Primitive::Char)) {
                    return Ok(ReturnKind::StringVec {
                        ptr_field: make_field_name(&first.name),
                        len_field: make_field_name(&data_fields[1].name),
                    });
                }
            }

            let elem = rust_type_from_pointer(pointee)
                .ok_or_else(|| anyhow!("unsupported result pointer {}", first.name))?;
            return Ok(ReturnKind::Vec {
                ptr_field: make_field_name(&first.name),
                len_field: make_field_name(&data_fields[1].name),
                elem,
                mutable_ptr: !outer_const,
            });
        }
    }
    let mut fields = Vec::new();
    for field in data_fields {
        fields.push(ReturnField {
            name: make_field_name(&field.name),
            ty: match return_field_type(&field.ty) {
                Some(ty) => ty,
                None => return Err(anyhow!("unsupported result field {}", field.name)),
            },
        });
    }
    if fields.len() == 1 {
        Ok(ReturnKind::Scalar(fields.remove(0)))
    } else {
        Ok(ReturnKind::Tuple(fields))
    }
}

fn return_field_type(ty: &CType) -> Option<ReturnFieldType> {
    match ty {
        CType::Primitive(p) => Some(ReturnFieldType::Plain(TypeRef::Primitive(p.rust_type()))),
        CType::Record(name) => Some(ReturnFieldType::Plain(TypeRef::Struct(name.clone()))),
        CType::Pointer { pointee, is_const } => {
            if *is_const {
                if let CType::Primitive(Primitive::Char) = **pointee {
                    return Some(ReturnFieldType::CString);
                }
            }
            None
        }
        _ => None,
    }
}

fn rust_type_from_c(ty: &CType) -> Option<TypeRef> {
    match ty {
        CType::Primitive(p) => Some(TypeRef::Primitive(p.rust_type())),
        CType::Record(name) => Some(TypeRef::Struct(name.clone())),
        _ => None,
    }
}

fn rust_type_from_pointer(ty: &CType) -> Option<TypeRef> {
    match ty {
        CType::Primitive(p) => Some(TypeRef::Primitive(p.rust_type())),
        CType::Record(name) => Some(TypeRef::Struct(name.clone())),
        _ => None,
    }
}

fn type_ref_to_string(ty: &TypeRef) -> String {
    match ty {
        TypeRef::Primitive(name) => name.to_string(),
        TypeRef::Struct(name) => format!("sys::{}", name),
    }
}

fn make_param_name(name: &str) -> String {
    let normalized = normalize_acronyms(name);
    let snake = normalized.to_snake_case();
    let collapsed = collapse_acronyms(&snake);
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
        Err(anyhow!("Could not find NATIVE_API_CURRENT_VERSION in {}", common_header.display()))
    }
}
