//! Production authorization policy for generated Core-Wasm callouts.
//!
//! Keep this separate from ABI lowering. `spring-native-codegen` may know how
//! to marshal an API safely without that API being appropriate authority for an
//! untrusted production guest. Security capability, process safety,
//! synced-determinism, and information visibility are intentionally separate
//! decisions here.

pub(crate) const CORE_SYNCED_ENVIRONMENT_MASK: u32 = (1u32 << 0) | (1u32 << 2);
pub(crate) const CORE_UNSYNCED_ENVIRONMENT_MASK: u32 = (1u32 << 1) | (1u32 << 3) | (1u32 << 4);

/// Whether an executable generated binding is authorized for production Core
/// guests at all. This is a security/capability decision, not a safety, sync,
/// or visibility decision.
pub(crate) fn production_import_allowed(module: &str, function: &str) -> bool {
    match module {
        // Process lifecycle/watchdog/restart/share controls are engine/process
        // authority. CallAsTeam is different: it scopes simulation team context
        // around one synchronous guest callback and carries no ambient OS/process
        // capability.
        "system_control" => {
            function == "CallAsTeam"
                || matches!(
                    function,
                    "GetGameName"
                        | "GetGameState"
                        | "GetMenuName"
                        | "GetReplayLength"
                        | "GetVideoCapturingMode"
                        | "IsReplay"
                )
        }

        // Synthetic host input can control the local application.
        "debug_input" => false,

        // Architecture/headless state is local machine identity. Keep it out of
        // production Core until an explicit reviewed local-info capability exists.
        "platform" => matches!(function, "GetArchitecture" | "IsHeadless"),

        // Generic config includes persistent setters. Individually reviewed
        // read-only helpers can be exposed through a dedicated Core namespace.
        "config" => false,

        // Generic profiling contains local clocks/memory/renderer state. The
        // handwritten profiling/desync namespaces carry the reviewed policy.
        "profiling" => false,

        // Generic VFS mixes content/archive helpers with raw path lookup,
        // directory creation, extraction/compression, data-dir rescans, and
        // downloader control. Keep it out of the ambient production namespace.
        "vfs" => false,

        // Most message APIs are game/UI communication. SendCommands feeds
        // guest-controlled strings into guihandler->RunCustomCommands and is
        // engine command authority rather than ordinary messaging.
        "messages" => function != "SendCommands",

        // RmlUi's global file interface uses SPRING_VFS_RAW_FIRST. Markup and
        // resource references can therefore read arbitrary host paths indirectly.
        "rml_ui" => false,

        // Most sound APIs operate on engine/VFS-owned sound items. Streaming is
        // different: MusicStream opens the guest-supplied path through the
        // RAW_FIRST CFileHandler default.
        "sound" => function != "PlaySoundStream",

        // Gfx is mostly GPU-local state. Deny only calls proven to cross the
        // filesystem boundary: SaveImage writes a guest-supplied host path;
        // font loading uses RAW_FIRST/system font discovery; and the listed
        // texture resolvers fall through to CNamedTextures lazy file loading.
        "gfx" => !matches!(
            function,
            "SaveImage"
                | "LoadFont"
                | "AddFallbackFont"
                | "BindTexture"
                | "TextureInfo"
                | "BindImageTexture"
                | "SetFBOAttachment"
        ),

        // Most unsynced control is renderer/UI state, but these calls cross into
        // OS/process-global input, clipboard, window, video-capture, or host
        // config/model-loading behavior.
        "unsynced_ctrl" => !matches!(
            function,
            "LoadCmdColorsConfig"
                | "LoadCtrlPanelConfig"
                | "LoadModelTextures"
                | "SetClipboard"
                | "SDLSetTextInputRect"
                | "SDLStartTextInput"
                | "SDLStopTextInput"
                | "SetVideoCapturingMode"
                | "SetVideoCapturingTimeOffset"
                | "SetWMCaption"
                | "SetWMIcon"
                | "SetWindowGeometry"
                | "SetWindowMaximized"
                | "SetWindowMinimized"
                | "WarpMouse"
        ),
        _ => true,
    }
}

/// Whether a generated import is safe for an untrusted guest to invoke without
/// risking the host process. This is deliberately independent of authority,
/// sync, and visibility. Unsafe entries remain codegen-covered so the
/// underlying adapter can be fixed without hiding transport support.
pub(crate) fn production_process_safe(module: &str, function: &str) -> bool {
    match module {
        // NativeInterface/api/Tracing.cpp passes owner=nullptr to the engine
        // TraceRay overload. Unit-scanning variants currently dereference
        // owner->allyteam. TraceRayFeatures sets NOUNITS and never reaches that
        // dereference, so it is process-safe even though its visibility policy
        // is still wrong for unsynced/UI callers.
        "tracing" => !matches!(function, "TraceRay" | "TraceRayUnits"),
        _ => true,
    }
}

/// Normalize generic NativeInterface availability into a deterministic Core
/// environment mask. This is a sync policy, not a security or visibility policy.
pub(crate) fn production_environment_mask(module: &str, source_mask: u32) -> u32 {
    match module {
        "unsynced_read" | "unsynced_ctrl" | "camera" | "input" | "display" | "selection"
        | "sound" | "gfx" | "lights" | "icons" | "markers" | "ground_decals" | "rml_ui"
        | "player" => source_mask & CORE_UNSYNCED_ENVIRONMENT_MASK,
        _ => source_mask,
    }
}

/// Function-level sync hardening for modules that intentionally mix synced and
/// unsynced operations. Keep this distinct from capability and visibility.
pub(crate) fn production_function_environment_mask(
    module: &str,
    function: &str,
    source_mask: u32,
) -> u32 {
    if module == "messages" {
        return if function == "SendToUnsynced" {
            source_mask & CORE_SYNCED_ENVIRONMENT_MASK
        } else {
            source_mask & CORE_UNSYNCED_ENVIRONMENT_MASK
        };
    }

    production_environment_mask(module, source_mask)
}

/// Information-visibility policy. This does not decide whether a call is
/// deterministic or privileged; it only prevents a guest perspective from
/// observing game information that the corresponding environment may not see.
pub(crate) fn production_visibility_environment_mask(
    module: &str,
    function: &str,
    source_mask: u32,
) -> u32 {
    match module {
        // The legacy single-hit tracing helpers do not call the LOS-filtering
        // predicates used by TraceRayInDirection/BetweenPositions. Their
        // guest-supplied allyTeamID can therefore reveal hidden units/features
        // from unsynced/UI environments. Synced simulation has full read access.
        "tracing" if matches!(function, "TraceRay" | "TraceRayUnits" | "TraceRayFeatures") => {
            source_mask & CORE_SYNCED_ENVIRONMENT_MASK
        }
        _ => source_mask,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        production_environment_mask, production_function_environment_mask,
        production_import_allowed, production_process_safe, production_visibility_environment_mask,
        CORE_SYNCED_ENVIRONMENT_MASK, CORE_UNSYNCED_ENVIRONMENT_MASK,
    };

    #[test]
    fn withholds_process_and_local_authority() {
        assert!(production_import_allowed("system_control", "CallAsTeam"));
        assert!(!production_import_allowed("system_control", "Quit"));
        assert!(production_import_allowed("system_control", "GetGameName"));
        assert!(!production_import_allowed("debug_input", "EmulateKey"));
        assert!(production_import_allowed("platform", "GetArchitecture"));
        assert!(!production_import_allowed("config", "GetConfigInt"));
        assert!(!production_import_allowed("config", "SetConfigString"));
        assert!(!production_import_allowed("profiling", "GetTimerMicros"));
        assert!(!production_import_allowed("vfs", "LoadFile"));
        assert!(!production_import_allowed("vfs", "ZlibCompress"));
        assert!(!production_import_allowed("messages", "SendCommands"));
        assert!(production_import_allowed("messages", "SendPublicChat"));
        assert!(!production_import_allowed("rml_ui", "ContextLoadDocument"));
        assert!(!production_import_allowed(
            "rml_ui",
            "ContextCreateDocument"
        ));
        assert!(!production_import_allowed("sound", "PlaySoundStream"));
        assert!(production_import_allowed("sound", "PlaySoundFile"));
    }

    #[test]
    fn process_safety_is_separate_from_visibility() {
        for function in ["TraceRay", "TraceRayUnits"] {
            assert!(production_import_allowed("tracing", function), "{function}");
            assert!(!production_process_safe("tracing", function), "{function}");
        }
        assert!(production_process_safe("tracing", "TraceRayFeatures"));
        assert!(production_process_safe(
            "tracing",
            "TraceRayBetweenPositions"
        ));
        assert!(production_process_safe("units_info", "GetUnitPosition"));
    }

    #[test]
    fn single_hit_tracing_is_synced_only_until_los_sanitized() {
        for function in ["TraceRay", "TraceRayUnits", "TraceRayFeatures"] {
            assert_eq!(
                production_visibility_environment_mask("tracing", function, 31),
                CORE_SYNCED_ENVIRONMENT_MASK,
                "{function}"
            );
        }
        assert_eq!(
            production_visibility_environment_mask("tracing", "TraceRayBetweenPositions", 31),
            31
        );
    }

    #[test]
    fn gfx_withholds_only_reviewed_filesystem_crossings() {
        for function in [
            "SaveImage",
            "LoadFont",
            "AddFallbackFont",
            "BindTexture",
            "TextureInfo",
            "BindImageTexture",
            "SetFBOAttachment",
        ] {
            assert!(!production_import_allowed("gfx", function), "{function}");
        }
        assert!(production_import_allowed("gfx", "CreateTexture"));
        assert!(production_import_allowed("gfx", "UploadTexture"));
        assert!(production_import_allowed("gfx", "DrawArraysVAO"));
    }

    #[test]
    fn withholds_os_facing_unsynced_controls_only() {
        assert!(!production_import_allowed("unsynced_ctrl", "SetClipboard"));
        assert!(!production_import_allowed(
            "unsynced_ctrl",
            "SetWindowGeometry"
        ));
        assert!(!production_import_allowed("unsynced_ctrl", "WarpMouse"));
        assert!(production_import_allowed("unsynced_ctrl", "SetUnitNoDraw"));
        assert!(production_import_allowed("units_info", "GetUnitPosition"));
    }

    #[test]
    fn keeps_client_local_modules_out_of_synced_environments() {
        assert_eq!(
            production_environment_mask("unsynced_ctrl", 31),
            CORE_UNSYNCED_ENVIRONMENT_MASK
        );
        assert_eq!(
            production_environment_mask("camera", 31),
            CORE_UNSYNCED_ENVIRONMENT_MASK
        );
        assert_eq!(
            production_environment_mask("unsynced_read", 31),
            CORE_UNSYNCED_ENVIRONMENT_MASK
        );
        assert_eq!(production_environment_mask("units_info", 31), 31);
    }

    #[test]
    fn messages_keep_only_the_reviewed_sync_direction() {
        assert_eq!(
            production_function_environment_mask("messages", "SendPublicChat", 31),
            CORE_UNSYNCED_ENVIRONMENT_MASK
        );
        assert_eq!(
            production_function_environment_mask("messages", "GetCurrentTooltip", 31),
            CORE_UNSYNCED_ENVIRONMENT_MASK
        );
        assert_eq!(
            production_function_environment_mask("messages", "SendToUnsynced", 31),
            CORE_SYNCED_ENVIRONMENT_MASK
        );
        assert_eq!(
            production_function_environment_mask("messages", "SendToUnsynced", 26),
            0
        );
    }
}
