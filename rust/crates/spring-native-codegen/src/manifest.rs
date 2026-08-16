//! Shared list of NativeInterface API modules.

#[derive(Debug, Clone, Copy)]
pub struct ApiDefinition {
    pub module: &'static str,
    pub header: &'static str,
    pub api_struct: &'static str,
}

pub const API_DEFINITIONS: &[ApiDefinition] = &[
    ApiDefinition {
        module: "callins",
        header: "Callins.h",
        api_struct: "",
    },
    ApiDefinition {
        module: "units_query",
        header: "UnitsQuery.h",
        api_struct: "UnitsQueryApi",
    },
    ApiDefinition {
        module: "units_info",
        header: "UnitsInfo.h",
        api_struct: "UnitsInfoApi",
    },
    ApiDefinition {
        module: "units_weapons",
        header: "UnitsWeapons.h",
        api_struct: "UnitsWeaponsApi",
    },
    ApiDefinition {
        module: "units_commands",
        header: "UnitsCommands.h",
        api_struct: "UnitsCommandsApi",
    },
    ApiDefinition {
        module: "units_pieces",
        header: "UnitsPieces.h",
        api_struct: "UnitsPiecesApi",
    },
    ApiDefinition {
        module: "teams",
        header: "Teams.h",
        api_struct: "TeamsApi",
    },
    ApiDefinition {
        module: "features",
        header: "Features.h",
        api_struct: "FeaturesApi",
    },
    ApiDefinition {
        module: "projectiles",
        header: "Projectiles.h",
        api_struct: "ProjectilesApi",
    },
    ApiDefinition {
        module: "los",
        header: "LOS.h",
        api_struct: "LOSApi",
    },
    ApiDefinition {
        module: "unit_defs",
        header: "UnitDefs.h",
        api_struct: "UnitDefsApi",
    },
    ApiDefinition {
        module: "feature_defs",
        header: "FeatureDefs.h",
        api_struct: "FeatureDefsApi",
    },
    ApiDefinition {
        module: "weapon_defs",
        header: "WeaponDefs.h",
        api_struct: "WeaponDefsApi",
    },
    ApiDefinition {
        module: "game",
        header: "Game.h",
        api_struct: "GameApi",
    },
    ApiDefinition {
        module: "terrain",
        header: "Terrain.h",
        api_struct: "TerrainApi",
    },
    ApiDefinition {
        module: "player",
        header: "Player.h",
        api_struct: "PlayerApi",
    },
    ApiDefinition {
        module: "math_extra",
        header: "MathExtra.h",
        api_struct: "MathExtraApi",
    },
    ApiDefinition {
        module: "encoding",
        header: "Encoding.h",
        api_struct: "EncodingApi",
    },
    ApiDefinition {
        module: "metal_map",
        header: "MetalMap.h",
        api_struct: "MetalMapApi",
    },
    ApiDefinition {
        module: "path_finder",
        header: "PathFinder.h",
        api_struct: "PathFinderApi",
    },
    ApiDefinition {
        module: "platform",
        header: "Platform.h",
        api_struct: "PlatformApi",
    },
    ApiDefinition {
        module: "rules_params",
        header: "RulesParams.h",
        api_struct: "RulesParamsApi",
    },
    ApiDefinition {
        module: "move_ctrl",
        header: "MoveCtrl.h",
        api_struct: "MoveCtrlApi",
    },
    ApiDefinition {
        module: "synced_ctrl",
        header: "SyncedCtrl.h",
        api_struct: "SyncedCtrlApi",
    },
    ApiDefinition {
        module: "camera",
        header: "Camera.h",
        api_struct: "CameraApi",
    },
    ApiDefinition {
        module: "input",
        header: "Input.h",
        api_struct: "InputApi",
    },
    ApiDefinition {
        module: "debug_input",
        header: "DebugInput.h",
        api_struct: "DebugInputApi",
    },
    ApiDefinition {
        module: "display",
        header: "Display.h",
        api_struct: "DisplayApi",
    },
    ApiDefinition {
        module: "selection",
        header: "Selection.h",
        api_struct: "SelectionApi",
    },
    ApiDefinition {
        module: "sound",
        header: "Sound.h",
        api_struct: "SoundApi",
    },
    ApiDefinition {
        module: "messages",
        header: "Messages.h",
        api_struct: "MessagesApi",
    },
    ApiDefinition {
        module: "config",
        header: "Config.h",
        api_struct: "ConfigApi",
    },
    ApiDefinition {
        module: "tracing",
        header: "Tracing.h",
        api_struct: "TracingApi",
    },
    ApiDefinition {
        module: "utils",
        header: "Utils.h",
        api_struct: "UtilsApi",
    },
    ApiDefinition {
        module: "memory",
        header: "Memory.h",
        api_struct: "MemoryApi",
    },
    ApiDefinition {
        module: "unsynced_ctrl",
        header: "UnsyncedCtrl.h",
        api_struct: "UnsyncedCtrlApi",
    },
    ApiDefinition {
        module: "gfx",
        header: "Gfx.h",
        api_struct: "GfxApi",
    },
    ApiDefinition {
        module: "lights",
        header: "Lights.h",
        api_struct: "LightsApi",
    },
    ApiDefinition {
        module: "icons",
        header: "Icons.h",
        api_struct: "IconsApi",
    },
    ApiDefinition {
        module: "markers",
        header: "Markers.h",
        api_struct: "MarkersApi",
    },
    ApiDefinition {
        module: "ground_decals",
        header: "GroundDecals.h",
        api_struct: "GroundDecalsApi",
    },
    ApiDefinition {
        module: "system_control",
        header: "SystemControl.h",
        api_struct: "SystemControlApi",
    },
    ApiDefinition {
        module: "profiling",
        header: "Profiling.h",
        api_struct: "ProfilingApi",
    },
    ApiDefinition {
        module: "rml_ui",
        header: "RmlUi.h",
        api_struct: "RmlUiApi",
    },
    ApiDefinition {
        module: "vfs",
        header: "VFS.h",
        api_struct: "VFSApi",
    },
    ApiDefinition {
        module: "unsynced_read",
        header: "UnsyncedRead.h",
        api_struct: "UnsyncedReadApi",
    },
    ApiDefinition {
        module: "team_control",
        header: "SyncedCtrl.h",
        api_struct: "TeamControlApi",
    },
    ApiDefinition {
        module: "unit_control",
        header: "SyncedCtrl.h",
        api_struct: "UnitControlApi",
    },
    ApiDefinition {
        module: "feature_control",
        header: "SyncedCtrl.h",
        api_struct: "FeatureControlApi",
    },
    ApiDefinition {
        module: "terrain_control",
        header: "SyncedCtrl.h",
        api_struct: "TerrainControlApi",
    },
    ApiDefinition {
        module: "projectile_control",
        header: "SyncedCtrl.h",
        api_struct: "ProjectileControlApi",
    },
    ApiDefinition {
        module: "effects_control",
        header: "SyncedCtrl.h",
        api_struct: "EffectsControlApi",
    },
    ApiDefinition {
        module: "game_config",
        header: "SyncedCtrl.h",
        api_struct: "GameConfigApi",
    },
    ApiDefinition {
        module: "cob_script",
        header: "SyncedCtrl.h",
        api_struct: "COBScriptApi",
    },
    ApiDefinition {
        module: "unit_rendering",
        header: "UnsyncedRead.h",
        api_struct: "UnitRenderingApi",
    },
];
