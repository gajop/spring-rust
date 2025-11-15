use std::ptr::NonNull;

use crate::{
    camera::Camera, config::Config, display::Display, feature_defs::FeatureDefs,
    features::Features, game::Game, input::Input, los::Los, math_extra::MathExtra,
    memory::Memory, messages::Messages, metal_map::MetalMap, move_ctrl::MoveCtrl,
    path_finder::PathFinder, player::Player, projectiles::Projectiles, rules_params::RulesParams,
    selection::Selection, sound::Sound, synced_ctrl::SyncedCtrl, sys, teams::Teams,
    terrain::Terrain, tracing::Tracing, unit_defs::UnitDefs, units_commands::UnitsCommands,
    units_info::UnitsInfo, units_pieces::UnitsPieces, units_query::UnitsQuery,
    units_weapons::UnitsWeapons, utils::Utils, vfs::Vfs, weapon_defs::WeaponDefs,
};

#[derive(Clone, Copy)]
pub struct NativeInterfaceRef {
    raw: NonNull<sys::NativeInterface>,
}

// Safety: The NativeInterface is managed by the Spring engine, which handles
// synchronization. The plugin API is designed to be called from a single game thread.
unsafe impl Send for NativeInterfaceRef {}
unsafe impl Sync for NativeInterfaceRef {}

impl NativeInterfaceRef {
    /// # Safety
    /// Caller must ensure the pointer is valid for the lifetime of the wrapper.
    pub unsafe fn from_ptr(ptr: *const sys::NativeInterface) -> Option<Self> {
        NonNull::new(ptr as *mut sys::NativeInterface).map(|raw| Self { raw })
    }

    pub fn as_ptr(&self) -> *const sys::NativeInterface {
        self.raw.as_ptr()
    }

    pub fn units_query(&self) -> Option<UnitsQuery<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.unitsQuery.as_ref().map(UnitsQuery::new)
        }
    }

    pub fn units_info(&self) -> Option<UnitsInfo<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.unitsInfo.as_ref().map(UnitsInfo::new)
        }
    }

    pub fn teams(&self) -> Option<Teams<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.teams.as_ref().map(Teams::new)
        }
    }

    pub fn units_weapons(&self) -> Option<UnitsWeapons<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.unitsWeapons.as_ref().map(UnitsWeapons::new)
        }
    }

    pub fn units_commands(&self) -> Option<UnitsCommands<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.unitsCommands.as_ref().map(UnitsCommands::new)
        }
    }

    pub fn units_pieces(&self) -> Option<UnitsPieces<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.unitsPieces.as_ref().map(UnitsPieces::new)
        }
    }

    pub fn features(&self) -> Option<Features<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.features.as_ref().map(Features::new)
        }
    }

    pub fn projectiles(&self) -> Option<Projectiles<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.projectiles.as_ref().map(Projectiles::new)
        }
    }

    pub fn los(&self) -> Option<Los<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.los.as_ref().map(Los::new)
        }
    }

    pub fn unit_defs(&self) -> Option<UnitDefs<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.unitDefs.as_ref().map(UnitDefs::new)
        }
    }

    pub fn feature_defs(&self) -> Option<FeatureDefs<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.featureDefs.as_ref().map(FeatureDefs::new)
        }
    }

    pub fn weapon_defs(&self) -> Option<WeaponDefs<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.weaponDefs.as_ref().map(WeaponDefs::new)
        }
    }

    pub fn game(&self) -> Option<Game<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.game.as_ref().map(Game::new)
        }
    }

    pub fn terrain(&self) -> Option<Terrain<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.terrain.as_ref().map(Terrain::new)
        }
    }

    pub fn player(&self) -> Option<Player<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.player.as_ref().map(Player::new)
        }
    }

    pub fn math_extra(&self) -> Option<MathExtra<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.mathExtra.as_ref().map(MathExtra::new)
        }
    }

    pub fn metal_map(&self) -> Option<MetalMap<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.metalMap.as_ref().map(MetalMap::new)
        }
    }

    pub fn path_finder(&self) -> Option<PathFinder<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.pathFinder.as_ref().map(PathFinder::new)
        }
    }

    pub fn rules_params(&self) -> Option<RulesParams<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.rulesParams.as_ref().map(RulesParams::new)
        }
    }

    pub fn move_ctrl(&self) -> Option<MoveCtrl<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.moveCtrl.as_ref().map(MoveCtrl::new)
        }
    }

    pub fn synced_ctrl(&self) -> Option<SyncedCtrl<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.syncedCtrl.as_ref().map(SyncedCtrl::new)
        }
    }

    pub fn camera(&self) -> Option<Camera<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.cameraApi.as_ref().map(Camera::new)
        }
    }

    pub fn input(&self) -> Option<Input<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.input.as_ref().map(Input::new)
        }
    }

    pub fn display(&self) -> Option<Display<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.display.as_ref().map(Display::new)
        }
    }

    pub fn selection(&self) -> Option<Selection<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.selection.as_ref().map(Selection::new)
        }
    }

    pub fn vfs(&self) -> Option<Vfs<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.vfs.as_ref().map(Vfs::new)
        }
    }

    pub fn sound(&self) -> Option<Sound<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.soundApi.as_ref().map(Sound::new)
        }
    }

    pub fn messages(&self) -> Option<Messages<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.messages.as_ref().map(Messages::new)
        }
    }

    pub fn config(&self) -> Option<Config<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.config.as_ref().map(Config::new)
        }
    }

    pub fn tracing(&self) -> Option<Tracing<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.tracing.as_ref().map(Tracing::new)
        }
    }

    pub fn utils(&self) -> Option<Utils<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.utils.as_ref().map(Utils::new)
        }
    }

    pub fn memory(&self) -> Option<Memory<'_>> {
        unsafe {
            let iface = self.raw.as_ref();
            iface.memory.as_ref().map(Memory::new)
        }
    }
}
