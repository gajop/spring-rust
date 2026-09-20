use std::ptr::NonNull;

use crate::{
    camera::Camera, config::Config, debug_input::DebugInput, display::Display, encoding::Encoding,
    feature_defs::FeatureDefs, features::Features, game::Game, gfx::Gfx,
    ground_decals::GroundDecals, icons::Icons, input::Input, lights::Lights, los::Los,
    markers::Markers, math_extra::MathExtra, memory::Memory, messages::Messages,
    metal_map::MetalMap, move_ctrl::MoveCtrl, path_finder::PathFinder, platform::Platform,
    player::Player, profiling::Profiling, projectiles::Projectiles, rml_ui::RmlUi,
    rules_params::RulesParams, selection::Selection, sound::Sound, synced_ctrl::SyncedCtrl, sys,
    system_control::SystemControl, teams::Teams, terrain::Terrain, tracing::Tracing,
    unit_defs::UnitDefs, units_commands::UnitsCommands, units_info::UnitsInfo,
    units_pieces::UnitsPieces, units_query::UnitsQuery, units_weapons::UnitsWeapons,
    unsynced_ctrl::UnsyncedCtrl, unsynced_read::UnsyncedRead, utils::Utils, vfs::Vfs,
    weapon_defs::WeaponDefs,
};

#[derive(Clone, Copy)]
pub struct NativeInterfaceRef {
    raw: NonNull<sys::NativeInterface>,
    // Standalone menu modules receive only the APIs valid for that environment.
    // Keep absence local to each accessor so a menu module can construct a
    // handle and use RmlUi without pretending that game APIs exist.
    units_query_api: Option<&'static sys::UnitsQueryApi>,
    units_info_api: Option<&'static sys::UnitsInfoApi>,
    teams_api: Option<&'static sys::TeamsApi>,
    units_weapons_api: Option<&'static sys::UnitsWeaponsApi>,
    units_commands_api: Option<&'static sys::UnitsCommandsApi>,
    units_pieces_api: Option<&'static sys::UnitsPiecesApi>,
    features_api: Option<&'static sys::FeaturesApi>,
    projectiles_api: Option<&'static sys::ProjectilesApi>,
    los_api: Option<&'static sys::LOSApi>,
    unit_defs_api: Option<&'static sys::UnitDefsApi>,
    feature_defs_api: Option<&'static sys::FeatureDefsApi>,
    weapon_defs_api: Option<&'static sys::WeaponDefsApi>,
    game_api: Option<&'static sys::GameApi>,
    terrain_api: Option<&'static sys::TerrainApi>,
    player_api: Option<&'static sys::PlayerApi>,
    math_extra_api: Option<&'static sys::MathExtraApi>,
    encoding_api: Option<&'static sys::EncodingApi>,
    metal_map_api: Option<&'static sys::MetalMapApi>,
    path_finder_api: Option<&'static sys::PathFinderApi>,
    platform_api: Option<&'static sys::PlatformApi>,
    rules_params_api: Option<&'static sys::RulesParamsApi>,
    rml_ui_api: Option<&'static sys::RmlUiApi>,
    move_ctrl_api: Option<&'static sys::MoveCtrlApi>,
    synced_ctrl_api: Option<&'static sys::SyncedCtrlApi>,
    camera_api: Option<&'static sys::CameraApi>,
    input_api: Option<&'static sys::InputApi>,
    debug_input_api: Option<&'static sys::DebugInputApi>,
    display_api: Option<&'static sys::DisplayApi>,
    selection_api: Option<&'static sys::SelectionApi>,
    vfs_api: Option<&'static sys::VFSApi>,
    sound_api: Option<&'static sys::SoundApi>,
    messages_api: Option<&'static sys::MessagesApi>,
    config_api: Option<&'static sys::ConfigApi>,
    tracing_api: Option<&'static sys::TracingApi>,
    utils_api: Option<&'static sys::UtilsApi>,
    memory_api: Option<&'static sys::MemoryApi>,
    unsynced_ctrl_api: Option<&'static sys::UnsyncedCtrlApi>,
    unsynced_read_api: Option<&'static sys::UnsyncedReadApi>,
    lights_api: Option<&'static sys::LightsApi>,
    icons_api: Option<&'static sys::IconsApi>,
    markers_api: Option<&'static sys::MarkersApi>,
    ground_decals_api: Option<&'static sys::GroundDecalsApi>,
    system_control_api: Option<&'static sys::SystemControlApi>,
    profiling_api: Option<&'static sys::ProfilingApi>,
    gfx_api: Option<&'static sys::GfxApi>,
}

// Safety: The NativeInterface is managed by the Spring engine, which handles
// synchronization. The plugin API is designed to be called from a single game thread.
unsafe impl Send for NativeInterfaceRef {}
unsafe impl Sync for NativeInterfaceRef {}

impl NativeInterfaceRef {
    /// # Safety
    /// Caller must ensure the pointer is valid for the lifetime of the wrapper.
    pub unsafe fn from_ptr(ptr: *const sys::NativeInterface) -> Option<Self> {
        unsafe {
            let raw = NonNull::new(ptr as *mut sys::NativeInterface)?;
            let iface = raw.as_ref();

            // Keep the engine's optional API pointers optional. Menu modules
            // intentionally receive a smaller interface than game modules.
            Some(Self {
                raw,
                units_query_api: iface.unitsQuery.as_ref(),
                units_info_api: iface.unitsInfo.as_ref(),
                teams_api: iface.teams.as_ref(),
                units_weapons_api: iface.unitsWeapons.as_ref(),
                units_commands_api: iface.unitsCommands.as_ref(),
                units_pieces_api: iface.unitsPieces.as_ref(),
                features_api: iface.features.as_ref(),
                projectiles_api: iface.projectiles.as_ref(),
                los_api: iface.los.as_ref(),
                unit_defs_api: iface.unitDefs.as_ref(),
                feature_defs_api: iface.featureDefs.as_ref(),
                weapon_defs_api: iface.weaponDefs.as_ref(),
                game_api: iface.game.as_ref(),
                terrain_api: iface.terrain.as_ref(),
                player_api: iface.player.as_ref(),
                math_extra_api: iface.mathExtra.as_ref(),
                encoding_api: iface.encoding.as_ref(),
                metal_map_api: iface.metalMap.as_ref(),
                path_finder_api: iface.pathFinder.as_ref(),
                platform_api: iface.platform.as_ref(),
                rules_params_api: iface.rulesParams.as_ref(),
                rml_ui_api: iface.rmlUi.as_ref(),
                move_ctrl_api: iface.moveCtrl.as_ref(),
                synced_ctrl_api: iface.syncedCtrl.as_ref(),
                camera_api: iface.cameraApi.as_ref(),
                input_api: iface.input.as_ref(),
                debug_input_api: iface.debugInput.as_ref(),
                display_api: iface.display.as_ref(),
                selection_api: iface.selection.as_ref(),
                vfs_api: iface.vfs.as_ref(),
                sound_api: iface.soundApi.as_ref(),
                messages_api: iface.messages.as_ref(),
                config_api: iface.config.as_ref(),
                tracing_api: iface.tracing.as_ref(),
                utils_api: iface.utils.as_ref(),
                memory_api: iface.memory.as_ref(),
                unsynced_ctrl_api: iface.unsyncedCtrl.as_ref(),
                unsynced_read_api: iface.unsyncedRead.as_ref(),
                lights_api: iface.lights.as_ref(),
                icons_api: iface.icons.as_ref(),
                markers_api: iface.markers.as_ref(),
                ground_decals_api: iface.groundDecals.as_ref(),
                system_control_api: iface.systemControl.as_ref(),
                profiling_api: iface.profiling.as_ref(),
                gfx_api: iface.gfx.as_ref(),
            })
        }
    }

    /// Return the borrowed C ABI pointer for a backend-specific extension.
    /// The engine owns the pointed-to interface for the module lifetime.
    pub fn as_ptr(&self) -> *const sys::NativeInterface {
        self.raw.as_ptr()
    }

    pub fn cus(&self) -> crate::cus::NativeCus {
        crate::cus::NativeCus::new(*self)
    }

    fn require_api<T>(api: Option<&'static T>, name: &'static str) -> &'static T {
        api.unwrap_or_else(|| panic!("{name} API is not available in this module environment"))
    }

    pub fn units_query(&self) -> UnitsQuery<'_> {
        UnitsQuery::new(Self::require_api(self.units_query_api, "unitsQuery"))
    }

    pub fn units_info(&self) -> UnitsInfo<'_> {
        UnitsInfo::new(Self::require_api(self.units_info_api, "unitsInfo"))
    }

    pub fn teams(&self) -> Teams<'_> {
        Teams::new(Self::require_api(self.teams_api, "teams"))
    }

    pub fn units_weapons(&self) -> UnitsWeapons<'_> {
        UnitsWeapons::new(Self::require_api(self.units_weapons_api, "unitsWeapons"))
    }

    pub fn units_commands(&self) -> UnitsCommands<'_> {
        UnitsCommands::new(Self::require_api(self.units_commands_api, "unitsCommands"))
    }

    pub fn units_pieces(&self) -> UnitsPieces<'_> {
        UnitsPieces::new(Self::require_api(self.units_pieces_api, "unitsPieces"))
    }

    pub fn features(&self) -> Features<'_> {
        Features::new(Self::require_api(self.features_api, "features"))
    }

    pub fn projectiles(&self) -> Projectiles<'_> {
        Projectiles::new(Self::require_api(self.projectiles_api, "projectiles"))
    }

    pub fn los(&self) -> Los<'_> {
        Los::new(Self::require_api(self.los_api, "los"))
    }

    pub fn unit_defs(&self) -> UnitDefs<'_> {
        UnitDefs::new(Self::require_api(self.unit_defs_api, "unitDefs"))
    }

    pub fn feature_defs(&self) -> FeatureDefs<'_> {
        FeatureDefs::new(Self::require_api(self.feature_defs_api, "featureDefs"))
    }

    pub fn weapon_defs(&self) -> WeaponDefs<'_> {
        WeaponDefs::new(Self::require_api(self.weapon_defs_api, "weaponDefs"))
    }

    pub fn game(&self) -> Game<'_> {
        Game::new(Self::require_api(self.game_api, "game"))
    }

    pub fn terrain(&self) -> Terrain<'_> {
        Terrain::new(Self::require_api(self.terrain_api, "terrain"))
    }

    pub fn player(&self) -> Player<'_> {
        Player::new(Self::require_api(self.player_api, "player"))
    }

    pub fn math_extra(&self) -> MathExtra<'_> {
        MathExtra::new(Self::require_api(self.math_extra_api, "mathExtra"))
    }

    pub fn encoding(&self) -> Encoding<'_> {
        Encoding::new(Self::require_api(self.encoding_api, "encoding"))
    }

    pub fn metal_map(&self) -> MetalMap<'_> {
        MetalMap::new(Self::require_api(self.metal_map_api, "metalMap"))
    }

    pub fn path_finder(&self) -> PathFinder<'_> {
        PathFinder::new(Self::require_api(self.path_finder_api, "pathFinder"))
    }

    pub fn platform(&self) -> Platform<'_> {
        Platform::new(Self::require_api(self.platform_api, "platform"))
    }

    pub fn rules_params(&self) -> RulesParams<'_> {
        RulesParams::new(Self::require_api(self.rules_params_api, "rulesParams"))
    }

    pub fn rml_ui(&self) -> RmlUi<'static> {
        RmlUi::new(Self::require_api(self.rml_ui_api, "rmlUi"))
    }

    pub fn move_ctrl(&self) -> MoveCtrl<'_> {
        MoveCtrl::new(Self::require_api(self.move_ctrl_api, "moveCtrl"))
    }

    pub fn synced_ctrl(&self) -> SyncedCtrl<'_> {
        SyncedCtrl::new(Self::require_api(self.synced_ctrl_api, "syncedCtrl"))
    }

    pub fn camera(&self) -> Camera<'_> {
        Camera::new(Self::require_api(self.camera_api, "camera"))
    }

    pub fn input(&self) -> Input<'_> {
        Input::new(Self::require_api(self.input_api, "input"))
    }

    pub fn debug_input(&self) -> DebugInput<'_> {
        DebugInput::new(Self::require_api(self.debug_input_api, "debugInput"))
    }

    pub fn display(&self) -> Display<'_> {
        Display::new(Self::require_api(self.display_api, "display"))
    }

    pub fn selection(&self) -> Selection<'_> {
        Selection::new(Self::require_api(self.selection_api, "selection"))
    }

    pub fn vfs(&self) -> Vfs<'_> {
        Vfs::new(Self::require_api(self.vfs_api, "vfs"))
    }

    pub fn sound(&self) -> Sound<'_> {
        Sound::new(Self::require_api(self.sound_api, "sound"))
    }

    pub fn messages(&self) -> Messages<'_> {
        Messages::new(Self::require_api(self.messages_api, "messages"))
    }

    pub fn config(&self) -> Config<'_> {
        Config::new(Self::require_api(self.config_api, "config"))
    }

    pub fn tracing(&self) -> Tracing<'_> {
        Tracing::new(Self::require_api(self.tracing_api, "tracing"))
    }

    pub fn utils(&self) -> Utils<'_> {
        Utils::new(Self::require_api(self.utils_api, "utils"))
    }

    pub fn memory(&self) -> Memory<'_> {
        Memory::new(Self::require_api(self.memory_api, "memory"))
    }

    pub fn unsynced_ctrl(&self) -> UnsyncedCtrl<'_> {
        UnsyncedCtrl::new(Self::require_api(self.unsynced_ctrl_api, "unsyncedCtrl"))
    }

    pub fn unsynced_read(&self) -> UnsyncedRead<'_> {
        UnsyncedRead::new(Self::require_api(self.unsynced_read_api, "unsyncedRead"))
    }

    pub fn lights(&self) -> Lights<'_> {
        Lights::new(Self::require_api(self.lights_api, "lights"))
    }

    pub fn icons(&self) -> Icons<'_> {
        Icons::new(Self::require_api(self.icons_api, "icons"))
    }

    pub fn markers(&self) -> Markers<'_> {
        Markers::new(Self::require_api(self.markers_api, "markers"))
    }

    pub fn ground_decals(&self) -> GroundDecals<'_> {
        GroundDecals::new(Self::require_api(self.ground_decals_api, "groundDecals"))
    }

    pub fn system_control(&self) -> SystemControl<'_> {
        SystemControl::new(Self::require_api(self.system_control_api, "systemControl"))
    }

    pub fn profiling(&self) -> Profiling<'_> {
        Profiling::new(Self::require_api(self.profiling_api, "profiling"))
    }

    pub fn gfx(&self) -> Gfx<'_> {
        Gfx::new(Self::require_api(self.gfx_api, "gfx"))
    }
}
