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
    // Cache unwrapped API pointers - guaranteed to be initialized by the engine
    units_query_api: &'static sys::UnitsQueryApi,
    units_info_api: &'static sys::UnitsInfoApi,
    teams_api: &'static sys::TeamsApi,
    units_weapons_api: &'static sys::UnitsWeaponsApi,
    units_commands_api: &'static sys::UnitsCommandsApi,
    units_pieces_api: &'static sys::UnitsPiecesApi,
    features_api: &'static sys::FeaturesApi,
    projectiles_api: &'static sys::ProjectilesApi,
    los_api: &'static sys::LOSApi,
    unit_defs_api: &'static sys::UnitDefsApi,
    feature_defs_api: &'static sys::FeatureDefsApi,
    weapon_defs_api: &'static sys::WeaponDefsApi,
    game_api: &'static sys::GameApi,
    terrain_api: &'static sys::TerrainApi,
    player_api: &'static sys::PlayerApi,
    math_extra_api: &'static sys::MathExtraApi,
    encoding_api: &'static sys::EncodingApi,
    metal_map_api: &'static sys::MetalMapApi,
    path_finder_api: &'static sys::PathFinderApi,
    platform_api: &'static sys::PlatformApi,
    rules_params_api: &'static sys::RulesParamsApi,
    rml_ui_api: &'static sys::RmlUiApi,
    move_ctrl_api: &'static sys::MoveCtrlApi,
    synced_ctrl_api: &'static sys::SyncedCtrlApi,
    camera_api: &'static sys::CameraApi,
    input_api: &'static sys::InputApi,
    debug_input_api: &'static sys::DebugInputApi,
    display_api: &'static sys::DisplayApi,
    selection_api: &'static sys::SelectionApi,
    vfs_api: &'static sys::VFSApi,
    sound_api: &'static sys::SoundApi,
    messages_api: &'static sys::MessagesApi,
    config_api: &'static sys::ConfigApi,
    tracing_api: &'static sys::TracingApi,
    utils_api: &'static sys::UtilsApi,
    memory_api: &'static sys::MemoryApi,
    unsynced_ctrl_api: &'static sys::UnsyncedCtrlApi,
    unsynced_read_api: &'static sys::UnsyncedReadApi,
    lights_api: &'static sys::LightsApi,
    icons_api: &'static sys::IconsApi,
    markers_api: &'static sys::MarkersApi,
    ground_decals_api: &'static sys::GroundDecalsApi,
    system_control_api: &'static sys::SystemControlApi,
    profiling_api: &'static sys::ProfilingApi,
    gfx_api: &'static sys::GfxApi,
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

            // Unwrap all API pointers at initialization - they are guaranteed to be set by the engine
            Some(Self {
                raw,
                units_query_api: iface
                    .unitsQuery
                    .as_ref()
                    .expect("unitsQuery API must be initialized"),
                units_info_api: iface
                    .unitsInfo
                    .as_ref()
                    .expect("unitsInfo API must be initialized"),
                teams_api: iface.teams.as_ref().expect("teams API must be initialized"),
                units_weapons_api: iface
                    .unitsWeapons
                    .as_ref()
                    .expect("unitsWeapons API must be initialized"),
                units_commands_api: iface
                    .unitsCommands
                    .as_ref()
                    .expect("unitsCommands API must be initialized"),
                units_pieces_api: iface
                    .unitsPieces
                    .as_ref()
                    .expect("unitsPieces API must be initialized"),
                features_api: iface
                    .features
                    .as_ref()
                    .expect("features API must be initialized"),
                projectiles_api: iface
                    .projectiles
                    .as_ref()
                    .expect("projectiles API must be initialized"),
                los_api: iface.los.as_ref().expect("los API must be initialized"),
                unit_defs_api: iface
                    .unitDefs
                    .as_ref()
                    .expect("unitDefs API must be initialized"),
                feature_defs_api: iface
                    .featureDefs
                    .as_ref()
                    .expect("featureDefs API must be initialized"),
                weapon_defs_api: iface
                    .weaponDefs
                    .as_ref()
                    .expect("weaponDefs API must be initialized"),
                game_api: iface.game.as_ref().expect("game API must be initialized"),
                terrain_api: iface
                    .terrain
                    .as_ref()
                    .expect("terrain API must be initialized"),
                player_api: iface
                    .player
                    .as_ref()
                    .expect("player API must be initialized"),
                math_extra_api: iface
                    .mathExtra
                    .as_ref()
                    .expect("mathExtra API must be initialized"),
                encoding_api: iface
                    .encoding
                    .as_ref()
                    .expect("encoding API must be initialized"),
                metal_map_api: iface
                    .metalMap
                    .as_ref()
                    .expect("metalMap API must be initialized"),
                path_finder_api: iface
                    .pathFinder
                    .as_ref()
                    .expect("pathFinder API must be initialized"),
                platform_api: iface
                    .platform
                    .as_ref()
                    .expect("platform API must be initialized"),
                rules_params_api: iface
                    .rulesParams
                    .as_ref()
                    .expect("rulesParams API must be initialized"),
                rml_ui_api: iface.rmlUi.as_ref().expect("rmlUi API must be initialized"),
                move_ctrl_api: iface
                    .moveCtrl
                    .as_ref()
                    .expect("moveCtrl API must be initialized"),
                synced_ctrl_api: iface
                    .syncedCtrl
                    .as_ref()
                    .expect("syncedCtrl API must be initialized"),
                camera_api: iface
                    .cameraApi
                    .as_ref()
                    .expect("cameraApi must be initialized"),
                input_api: iface.input.as_ref().expect("input API must be initialized"),
                debug_input_api: iface
                    .debugInput
                    .as_ref()
                    .expect("debugInput API must be initialized"),
                display_api: iface
                    .display
                    .as_ref()
                    .expect("display API must be initialized"),
                selection_api: iface
                    .selection
                    .as_ref()
                    .expect("selection API must be initialized"),
                vfs_api: iface.vfs.as_ref().expect("vfs API must be initialized"),
                sound_api: iface
                    .soundApi
                    .as_ref()
                    .expect("soundApi must be initialized"),
                messages_api: iface
                    .messages
                    .as_ref()
                    .expect("messages API must be initialized"),
                config_api: iface
                    .config
                    .as_ref()
                    .expect("config API must be initialized"),
                tracing_api: iface
                    .tracing
                    .as_ref()
                    .expect("tracing API must be initialized"),
                utils_api: iface.utils.as_ref().expect("utils API must be initialized"),
                memory_api: iface
                    .memory
                    .as_ref()
                    .expect("memory API must be initialized"),
                unsynced_ctrl_api: iface
                    .unsyncedCtrl
                    .as_ref()
                    .expect("unsyncedCtrl API must be initialized"),
                unsynced_read_api: iface
                    .unsyncedRead
                    .as_ref()
                    .expect("unsyncedRead API must be initialized"),
                lights_api: iface
                    .lights
                    .as_ref()
                    .expect("lights API must be initialized"),
                icons_api: iface.icons.as_ref().expect("icons API must be initialized"),
                markers_api: iface
                    .markers
                    .as_ref()
                    .expect("markers API must be initialized"),
                ground_decals_api: iface
                    .groundDecals
                    .as_ref()
                    .expect("groundDecals API must be initialized"),
                system_control_api: iface
                    .systemControl
                    .as_ref()
                    .expect("systemControl API must be initialized"),
                profiling_api: iface
                    .profiling
                    .as_ref()
                    .expect("profiling API must be initialized"),
                gfx_api: iface.gfx.as_ref().expect("gfx API must be initialized"),
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

    pub fn units_query(&self) -> UnitsQuery<'_> {
        UnitsQuery::new(self.units_query_api)
    }

    pub fn units_info(&self) -> UnitsInfo<'_> {
        UnitsInfo::new(self.units_info_api)
    }

    pub fn teams(&self) -> Teams<'_> {
        Teams::new(self.teams_api)
    }

    pub fn units_weapons(&self) -> UnitsWeapons<'_> {
        UnitsWeapons::new(self.units_weapons_api)
    }

    pub fn units_commands(&self) -> UnitsCommands<'_> {
        UnitsCommands::new(self.units_commands_api)
    }

    pub fn units_pieces(&self) -> UnitsPieces<'_> {
        UnitsPieces::new(self.units_pieces_api)
    }

    pub fn features(&self) -> Features<'_> {
        Features::new(self.features_api)
    }

    pub fn projectiles(&self) -> Projectiles<'_> {
        Projectiles::new(self.projectiles_api)
    }

    pub fn los(&self) -> Los<'_> {
        Los::new(self.los_api)
    }

    pub fn unit_defs(&self) -> UnitDefs<'_> {
        UnitDefs::new(self.unit_defs_api)
    }

    pub fn feature_defs(&self) -> FeatureDefs<'_> {
        FeatureDefs::new(self.feature_defs_api)
    }

    pub fn weapon_defs(&self) -> WeaponDefs<'_> {
        WeaponDefs::new(self.weapon_defs_api)
    }

    pub fn game(&self) -> Game<'_> {
        Game::new(self.game_api)
    }

    pub fn terrain(&self) -> Terrain<'_> {
        Terrain::new(self.terrain_api)
    }

    pub fn player(&self) -> Player<'_> {
        Player::new(self.player_api)
    }

    pub fn math_extra(&self) -> MathExtra<'_> {
        MathExtra::new(self.math_extra_api)
    }

    pub fn encoding(&self) -> Encoding<'_> {
        Encoding::new(self.encoding_api)
    }

    pub fn metal_map(&self) -> MetalMap<'_> {
        MetalMap::new(self.metal_map_api)
    }

    pub fn path_finder(&self) -> PathFinder<'_> {
        PathFinder::new(self.path_finder_api)
    }

    pub fn platform(&self) -> Platform<'_> {
        Platform::new(self.platform_api)
    }

    pub fn rules_params(&self) -> RulesParams<'_> {
        RulesParams::new(self.rules_params_api)
    }

    pub fn rml_ui(&self) -> RmlUi<'static> {
        RmlUi::new(self.rml_ui_api)
    }

    pub fn move_ctrl(&self) -> MoveCtrl<'_> {
        MoveCtrl::new(self.move_ctrl_api)
    }

    pub fn synced_ctrl(&self) -> SyncedCtrl<'_> {
        SyncedCtrl::new(self.synced_ctrl_api)
    }

    pub fn camera(&self) -> Camera<'_> {
        Camera::new(self.camera_api)
    }

    pub fn input(&self) -> Input<'_> {
        Input::new(self.input_api)
    }

    pub fn debug_input(&self) -> DebugInput<'_> {
        DebugInput::new(self.debug_input_api)
    }

    pub fn display(&self) -> Display<'_> {
        Display::new(self.display_api)
    }

    pub fn selection(&self) -> Selection<'_> {
        Selection::new(self.selection_api)
    }

    pub fn vfs(&self) -> Vfs<'_> {
        Vfs::new(self.vfs_api)
    }

    pub fn sound(&self) -> Sound<'_> {
        Sound::new(self.sound_api)
    }

    pub fn messages(&self) -> Messages<'_> {
        Messages::new(self.messages_api)
    }

    pub fn config(&self) -> Config<'_> {
        Config::new(self.config_api)
    }

    pub fn tracing(&self) -> Tracing<'_> {
        Tracing::new(self.tracing_api)
    }

    pub fn utils(&self) -> Utils<'_> {
        Utils::new(self.utils_api)
    }

    pub fn memory(&self) -> Memory<'_> {
        Memory::new(self.memory_api)
    }

    pub fn unsynced_ctrl(&self) -> UnsyncedCtrl<'_> {
        UnsyncedCtrl::new(self.unsynced_ctrl_api)
    }

    pub fn unsynced_read(&self) -> UnsyncedRead<'_> {
        UnsyncedRead::new(self.unsynced_read_api)
    }

    pub fn lights(&self) -> Lights<'_> {
        Lights::new(self.lights_api)
    }

    pub fn icons(&self) -> Icons<'_> {
        Icons::new(self.icons_api)
    }

    pub fn markers(&self) -> Markers<'_> {
        Markers::new(self.markers_api)
    }

    pub fn ground_decals(&self) -> GroundDecals<'_> {
        GroundDecals::new(self.ground_decals_api)
    }

    pub fn system_control(&self) -> SystemControl<'_> {
        SystemControl::new(self.system_control_api)
    }

    pub fn profiling(&self) -> Profiling<'_> {
        Profiling::new(self.profiling_api)
    }

    pub fn gfx(&self) -> Gfx<'_> {
        Gfx::new(self.gfx_api)
    }
}
