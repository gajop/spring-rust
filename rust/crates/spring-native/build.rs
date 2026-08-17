use spring_api_codegen as spring_native_codegen;
use std::{env, fs, path::PathBuf};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("manifest"));
    let mut project_root = manifest_dir.clone();
    for _ in 0..3 {
        if !project_root.pop() {
            panic!("failed to locate project root");
        }
    }

    let include_dir = project_root.join("rts/NativeInterface/api");
    let include_root = project_root.join("rts");
    let includes = vec![include_dir.clone(), include_root.clone()];

    // Normal module builds consume the committed deterministic snapshot. A
    // maintainer/CI regeneration explicitly opts into libclang with
    // SPRING_NATIVE_REGENERATE=1, keeping downstream builds independent of a
    // host libclang installation.
    let snapshot_dir = manifest_dir.join("generated");
    if env::var_os("SPRING_NATIVE_REGENERATE").is_none()
        && snapshot_dir.join("manifest.json").exists()
    {
        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
        for entry in fs::read_dir(&snapshot_dir).expect("read native snapshot") {
            let entry = entry.expect("native snapshot entry");
            if matches!(
                entry.file_name().to_str(),
                Some("manifest.json" | "README.md")
            ) {
                continue;
            }
            fs::copy(entry.path(), out_dir.join(entry.file_name()))
                .unwrap_or_else(|error| panic!("copy native snapshot: {error}"));
        }
        println!("cargo:rerun-if-changed={}", snapshot_dir.display());
        return;
    }

    // Define all API headers
    let api_headers = [
        ("units_query", "UnitsQuery.h"),
        ("units_info", "UnitsInfo.h"),
        ("units_weapons", "UnitsWeapons.h"),
        ("units_commands", "UnitsCommands.h"),
        ("units_pieces", "UnitsPieces.h"),
        ("teams", "Teams.h"),
        ("features", "Features.h"),
        ("projectiles", "Projectiles.h"),
        ("los", "LOS.h"),
        ("unit_defs", "UnitDefs.h"),
        ("feature_defs", "FeatureDefs.h"),
        ("weapon_defs", "WeaponDefs.h"),
        ("game", "Game.h"),
        ("terrain", "Terrain.h"),
        ("player", "Player.h"),
        ("math_extra", "MathExtra.h"),
        ("encoding", "Encoding.h"),
        ("metal_map", "MetalMap.h"),
        ("path_finder", "PathFinder.h"),
        ("platform", "Platform.h"),
        ("rules_params", "RulesParams.h"),
        ("move_ctrl", "MoveCtrl.h"),
        ("synced_ctrl", "SyncedCtrl.h"),
        ("camera", "Camera.h"),
        ("input", "Input.h"),
        ("debug_input", "DebugInput.h"),
        ("display", "Display.h"),
        ("selection", "Selection.h"),
        ("sound", "Sound.h"),
        ("messages", "Messages.h"),
        ("config", "Config.h"),
        ("tracing", "Tracing.h"),
        ("utils", "Utils.h"),
        ("memory", "Memory.h"),
        ("unsynced_ctrl", "UnsyncedCtrl.h"),
        ("gfx", "Gfx.h"),
        ("lights", "Lights.h"),
        ("icons", "Icons.h"),
        ("markers", "Markers.h"),
        ("ground_decals", "GroundDecals.h"),
        ("system_control", "SystemControl.h"),
        ("profiling", "Profiling.h"),
        ("rml_ui", "RmlUi.h"),
        ("vfs", "VFS.h"),
        ("unsynced_read", "UnsyncedRead.h"),
    ];

    let mut headers = Vec::new();
    for (_, filename) in &api_headers {
        headers.push(project_root.join(format!("rts/NativeInterface/api/{}", filename)));
    }

    for header in &headers {
        println!("cargo:rerun-if-changed={}", header.display());
    }
    println!("cargo:rerun-if-changed={}", include_dir.display());

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let codegen = spring_native_codegen::CodeGenerator::with_repository_root(&project_root)
        .unwrap_or_else(|error| panic!("initialize code generator: {error}"));

    // Generate code for all APIs
    for (i, (module_name, _)) in api_headers.iter().enumerate() {
        let generator = match *module_name {
            "units_query" => {
                spring_native_codegen::generate_units_query
                    as fn(&spring_native_codegen::CodeGenerator, &_, &_) -> _
            }
            "units_info" => spring_native_codegen::generate_units_info,
            "units_weapons" => spring_native_codegen::generate_units_weapons,
            "units_commands" => spring_native_codegen::generate_units_commands,
            "units_pieces" => spring_native_codegen::generate_units_pieces,
            "teams" => spring_native_codegen::generate_teams,
            "features" => spring_native_codegen::generate_features,
            "projectiles" => spring_native_codegen::generate_projectiles,
            "los" => spring_native_codegen::generate_los,
            "unit_defs" => spring_native_codegen::generate_unit_defs,
            "feature_defs" => spring_native_codegen::generate_feature_defs,
            "weapon_defs" => spring_native_codegen::generate_weapon_defs,
            "game" => spring_native_codegen::generate_game,
            "terrain" => spring_native_codegen::generate_terrain,
            "player" => spring_native_codegen::generate_player,
            "math_extra" => spring_native_codegen::generate_math_extra,
            "encoding" => spring_native_codegen::generate_encoding,
            "metal_map" => spring_native_codegen::generate_metal_map,
            "path_finder" => spring_native_codegen::generate_path_finder,
            "platform" => spring_native_codegen::generate_platform,
            "rules_params" => spring_native_codegen::generate_rules_params,
            "move_ctrl" => spring_native_codegen::generate_move_ctrl,
            "synced_ctrl" => spring_native_codegen::generate_synced_ctrl,
            "camera" => spring_native_codegen::generate_camera,
            "input" => spring_native_codegen::generate_input,
            "debug_input" => spring_native_codegen::generate_debug_input,
            "display" => spring_native_codegen::generate_display,
            "selection" => spring_native_codegen::generate_selection,
            "sound" => spring_native_codegen::generate_sound,
            "messages" => spring_native_codegen::generate_messages,
            "config" => spring_native_codegen::generate_config,
            "tracing" => spring_native_codegen::generate_tracing,
            "utils" => spring_native_codegen::generate_utils,
            "memory" => spring_native_codegen::generate_memory,
            "unsynced_ctrl" => spring_native_codegen::generate_unsynced_ctrl,
            "gfx" => spring_native_codegen::generate_gfx,
            "lights" => spring_native_codegen::generate_lights,
            "icons" => spring_native_codegen::generate_icons,
            "markers" => spring_native_codegen::generate_markers,
            "ground_decals" => spring_native_codegen::generate_ground_decals,
            "system_control" => spring_native_codegen::generate_system_control,
            "profiling" => spring_native_codegen::generate_profiling,
            "rml_ui" => spring_native_codegen::generate_rml_ui,
            "vfs" => spring_native_codegen::generate_vfs,
            "unsynced_read" => spring_native_codegen::generate_unsynced_read,
            _ => panic!("Unknown module: {}", module_name),
        };

        let code = generator(&codegen, &headers[i], &includes)
            .unwrap_or_else(|e| panic!("{} codegen: {}", module_name, e));
        fs::write(out_dir.join(format!("{}_generated.rs", module_name)), code)
            .unwrap_or_else(|e| panic!("write {}: {}", module_name, e));
    }

    // Generate SyncedCtrl sub-APIs (all use SyncedCtrl.h)
    let synced_ctrl_header = project_root.join("rts/NativeInterface/api/SyncedCtrl.h");
    let sub_apis = [
        (
            "team_control",
            spring_native_codegen::generate_team_control
                as fn(&spring_native_codegen::CodeGenerator, &_, &_) -> _,
        ),
        ("unit_control", spring_native_codegen::generate_unit_control),
        (
            "feature_control",
            spring_native_codegen::generate_feature_control,
        ),
        (
            "terrain_control",
            spring_native_codegen::generate_terrain_control,
        ),
        (
            "projectile_control",
            spring_native_codegen::generate_projectile_control,
        ),
        (
            "effects_control",
            spring_native_codegen::generate_effects_control,
        ),
        ("game_config", spring_native_codegen::generate_game_config),
        ("cob_script", spring_native_codegen::generate_cob_script),
    ];

    for (name, generator) in &sub_apis {
        let code = generator(&codegen, &synced_ctrl_header, &includes)
            .unwrap_or_else(|e| panic!("{} codegen: {}", name, e));
        fs::write(out_dir.join(format!("{}_generated.rs", name)), code)
            .unwrap_or_else(|e| panic!("write {}: {}", name, e));
    }

    // Generate UnitRendering sub-API (embedded within UnsyncedRead)
    let unsynced_read_header = project_root.join("rts/NativeInterface/api/UnsyncedRead.h");
    println!("cargo:rerun-if-changed={}", unsynced_read_header.display());
    let unit_rendering_code =
        spring_native_codegen::generate_unit_rendering(&codegen, &unsynced_read_header, &includes)
            .unwrap_or_else(|e| panic!("unit_rendering codegen: {}", e));
    fs::write(
        out_dir.join("unit_rendering_generated.rs"),
        unit_rendering_code,
    )
    .unwrap_or_else(|e| panic!("write unit_rendering: {}", e));

    // Extract and generate API version constants
    let common_header = project_root.join("rts/NativeInterface/api/Common.h");
    println!("cargo:rerun-if-changed={}", common_header.display());

    let (major, minor, patch) = spring_native_codegen::extract_api_version(&common_header)
        .unwrap_or_else(|e| panic!("Failed to extract API version: {}", e));

    let version_code = format!(
        r#"// API version constants extracted from Common.h at build time.

/// Major version of the Native API this module was built against.
///
/// Major version MUST match between host and module for compatibility.
pub const NATIVE_API_VERSION_MAJOR: u32 = {};

/// Minor version of the Native API this module was built against.
///
/// Module can require a minimum minor version from the host.
pub const NATIVE_API_VERSION_MINOR: u32 = {};

/// Patch version of the Native API this module was built against.
pub const NATIVE_API_VERSION_PATCH: u32 = {};
"#,
        major, minor, patch
    );

    fs::write(out_dir.join("version.rs"), version_code)
        .unwrap_or_else(|e| panic!("Failed to write version.rs: {}", e));
}
