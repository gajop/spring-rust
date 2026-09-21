//! The unsynced-rules environment.

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cell::Cell;

use crate::event::{EventResult, UnitDestroyedEvent, ViewGeometry};
use crate::runtime::{AddonContext, AddonRuntime};

pub trait UnsyncedAddon<G = ()> {
    fn name(&self) -> &'static str;

    fn is_enabled(&self) -> bool {
        true
    }

    fn init(&self, _ctx: &AddonContext<'_, G>) {}
    fn shutdown(&self, _ctx: &AddonContext<'_, G>) {}
    fn update(&self, _ctx: &AddonContext<'_, G>, _dt: f32) {}
    fn game_frame(&self, _ctx: &AddonContext<'_, G>, _frame: i32) {}
    fn unit_created(
        &self,
        _ctx: &AddonContext<'_, G>,
        _unit_id: i32,
        _unit_def_id: i32,
        _unit_team: i32,
        _builder_id: i32,
    ) {
    }
    fn unit_finished(
        &self,
        _ctx: &AddonContext<'_, G>,
        _unit_id: i32,
        _unit_def_id: i32,
        _unit_team: i32,
    ) {
    }
    fn unit_destroyed(&self, _ctx: &AddonContext<'_, G>, _event: &UnitDestroyedEvent) {}
    fn unit_given(
        &self,
        _ctx: &AddonContext<'_, G>,
        _unit_id: i32,
        _unit_def_id: i32,
        _old_team: i32,
        _new_team: i32,
    ) {
    }
    fn unit_taken(
        &self,
        _ctx: &AddonContext<'_, G>,
        _unit_id: i32,
        _unit_def_id: i32,
        _old_team: i32,
        _new_team: i32,
    ) {
    }
    fn feature_created(&self, _ctx: &AddonContext<'_, G>, _feature_id: i32, _ally_team_id: i32) {}
    fn feature_destroyed(&self, _ctx: &AddonContext<'_, G>, _feature_id: i32, _ally_team_id: i32) {}
    fn projectile_created(
        &self,
        _ctx: &AddonContext<'_, G>,
        _projectile_id: i32,
        _owner_id: i32,
        _weapon_def_id: i32,
    ) {
    }
    fn projectile_destroyed(
        &self,
        _ctx: &AddonContext<'_, G>,
        _projectile_id: i32,
        _owner_id: i32,
        _weapon_def_id: i32,
    ) {
    }
    fn recv_from_synced(&self, _ctx: &AddonContext<'_, G>, _message: &[u8]) {}
    fn draw_screen(&self, _ctx: &AddonContext<'_, G>, _width: i32, _height: i32) {}
    fn draw_genesis(&self, _ctx: &AddonContext<'_, G>) {}
    fn draw_world(&self, _ctx: &AddonContext<'_, G>) {}
    fn draw_world_pre_unit(&self, _ctx: &AddonContext<'_, G>) {}
    fn draw_world_pre_particles(
        &self,
        _ctx: &AddonContext<'_, G>,
        _draw_above_water: bool,
        _draw_below_water: bool,
        _draw_reflection: bool,
        _draw_refraction: bool,
    ) {
    }
    fn draw_screen_effects(&self, _ctx: &AddonContext<'_, G>, _width: i32, _height: i32) {}
    fn draw_unit(&self, _ctx: &AddonContext<'_, G>, _unit_id: i32, _draw_mode: i32) -> EventResult {
        EventResult::Ignored
    }
    fn draw_feature(
        &self,
        _ctx: &AddonContext<'_, G>,
        _feature_id: i32,
        _draw_mode: i32,
    ) -> EventResult {
        EventResult::Ignored
    }
    fn view_resize(&self, _ctx: &AddonContext<'_, G>, _geometry: &ViewGeometry) {}
}

pub struct UnsyncedHandler<G> {
    pub global: G,
    addons: Vec<Box<dyn UnsyncedAddon<G>>>,
    enabled: Vec<Cell<bool>>,
    runtime: AddonRuntime<G>,
}

impl<G> UnsyncedHandler<G> {
    pub fn new(global: G) -> Self {
        Self {
            global,
            addons: Vec::new(),
            enabled: Vec::new(),
            runtime: AddonRuntime::new(),
        }
    }

    pub fn add(&mut self, addon: Box<dyn UnsyncedAddon<G>>) {
        let is_enabled = addon.is_enabled();
        self.addons.push(addon);
        self.enabled.push(Cell::new(is_enabled));
    }

    pub fn global(&self) -> &G {
        &self.global
    }

    pub fn with_context<R>(&self, f: impl FnOnce(&AddonContext<'_, G>) -> R) -> R {
        self.runtime.callin("external", &self.global, f)
    }

    fn dispatch<R>(&self, callin: &'static str, f: impl FnOnce(&AddonContext<'_, G>) -> R) -> R {
        self.runtime.callin(callin, &self.global, f)
    }

    pub fn init(&self) {
        self.dispatch("Init", |ctx| {
            for (i, addon) in self.addons.iter().enumerate() {
                if self.enabled[i].get() {
                    addon.init(ctx);
                }
            }
        });
    }

    pub fn set_enabled(&self, name: &str, enabled: bool) {
        self.dispatch("SetEnabled", |ctx| {
            for (i, addon) in self.addons.iter().enumerate() {
                if addon.name() == name {
                    if self.enabled[i].replace(enabled) != enabled {
                        if enabled {
                            addon.init(ctx);
                        } else {
                            addon.shutdown(ctx);
                        }
                    }
                    break;
                }
            }
        });
    }

    pub fn game_frame(&self, frame: i32) {
        self.dispatch("GameFrame", |ctx| {
            for (i, addon) in self.addons.iter().enumerate() {
                if self.enabled[i].get() {
                    addon.game_frame(ctx, frame);
                }
            }
        });
    }

    pub fn update(&self, dt: f32) {
        self.dispatch("Update", |ctx| {
            for (i, addon) in self.addons.iter().enumerate() {
                if self.enabled[i].get() {
                    addon.update(ctx, dt);
                }
            }
        });
    }

    pub fn unit_created(&self, unit_id: i32, unit_def_id: i32, unit_team: i32, builder_id: i32) {
        self.dispatch("UnitCreated", |ctx| {
            for (i, addon) in self.addons.iter().enumerate() {
                if self.enabled[i].get() {
                    addon.unit_created(ctx, unit_id, unit_def_id, unit_team, builder_id);
                }
            }
        });
    }

    pub fn unit_finished(&self, unit_id: i32, unit_def_id: i32, unit_team: i32) {
        self.dispatch("UnitFinished", |ctx| {
            for (i, addon) in self.addons.iter().enumerate() {
                if self.enabled[i].get() {
                    addon.unit_finished(ctx, unit_id, unit_def_id, unit_team);
                }
            }
        });
    }

    pub fn unit_destroyed(&self, event: &UnitDestroyedEvent) {
        self.dispatch("UnitDestroyed", |ctx| {
            for (i, addon) in self.addons.iter().enumerate() {
                if self.enabled[i].get() {
                    addon.unit_destroyed(ctx, event);
                }
            }
        });
    }

    pub fn unit_given(&self, unit_id: i32, unit_def_id: i32, old_team: i32, new_team: i32) {
        self.dispatch("UnitGiven", |ctx| {
            for (i, addon) in self.addons.iter().enumerate() {
                if self.enabled[i].get() {
                    addon.unit_given(ctx, unit_id, unit_def_id, old_team, new_team);
                }
            }
        });
    }

    pub fn unit_taken(&self, unit_id: i32, unit_def_id: i32, old_team: i32, new_team: i32) {
        self.dispatch("UnitTaken", |ctx| {
            for (i, addon) in self.addons.iter().enumerate() {
                if self.enabled[i].get() {
                    addon.unit_taken(ctx, unit_id, unit_def_id, old_team, new_team);
                }
            }
        });
    }

    pub fn feature_created(&self, feature_id: i32, ally_team_id: i32) {
        self.dispatch("FeatureCreated", |ctx| {
            for (i, addon) in self.addons.iter().enumerate() {
                if self.enabled[i].get() {
                    addon.feature_created(ctx, feature_id, ally_team_id);
                }
            }
        });
    }

    pub fn feature_destroyed(&self, feature_id: i32, ally_team_id: i32) {
        self.dispatch("FeatureDestroyed", |ctx| {
            for (i, addon) in self.addons.iter().enumerate() {
                if self.enabled[i].get() {
                    addon.feature_destroyed(ctx, feature_id, ally_team_id);
                }
            }
        });
    }

    pub fn projectile_created(&self, projectile_id: i32, owner_id: i32, weapon_def_id: i32) {
        self.dispatch("ProjectileCreated", |ctx| {
            for (i, addon) in self.addons.iter().enumerate() {
                if self.enabled[i].get() {
                    addon.projectile_created(ctx, projectile_id, owner_id, weapon_def_id);
                }
            }
        });
    }

    pub fn projectile_destroyed(&self, projectile_id: i32, owner_id: i32, weapon_def_id: i32) {
        self.dispatch("ProjectileDestroyed", |ctx| {
            for (i, addon) in self.addons.iter().enumerate() {
                if self.enabled[i].get() {
                    addon.projectile_destroyed(ctx, projectile_id, owner_id, weapon_def_id);
                }
            }
        });
    }

    pub fn recv_from_synced(&self, message: &[u8]) {
        self.dispatch("RecvFromSynced", |ctx| {
            for (i, addon) in self.addons.iter().enumerate() {
                if self.enabled[i].get() {
                    addon.recv_from_synced(ctx, message);
                }
            }
        });
    }

    pub fn draw_screen(&self, width: i32, height: i32) {
        self.dispatch("DrawScreen", |ctx| {
            for (i, addon) in self.addons.iter().enumerate() {
                if self.enabled[i].get() {
                    addon.draw_screen(ctx, width, height);
                }
            }
        });
    }

    pub fn draw_world(&self) {
        self.dispatch("DrawWorld", |ctx| {
            for (i, addon) in self.addons.iter().enumerate() {
                if self.enabled[i].get() {
                    addon.draw_world(ctx);
                }
            }
        });
    }

    pub fn draw_world_pre_particles(
        &self,
        draw_above_water: bool,
        draw_below_water: bool,
        draw_reflection: bool,
        draw_refraction: bool,
    ) {
        self.dispatch("DrawWorldPreParticles", |ctx| {
            for (i, addon) in self.addons.iter().enumerate() {
                if self.enabled[i].get() {
                    addon.draw_world_pre_particles(
                        ctx,
                        draw_above_water,
                        draw_below_water,
                        draw_reflection,
                        draw_refraction,
                    );
                }
            }
        });
    }

    pub fn draw_screen_effects(&self, width: i32, height: i32) {
        self.dispatch("DrawScreenEffects", |ctx| {
            for (i, addon) in self.addons.iter().enumerate() {
                if self.enabled[i].get() {
                    addon.draw_screen_effects(ctx, width, height);
                }
            }
        });
    }

    pub fn draw_unit(&self, unit_id: i32, draw_mode: i32) -> bool {
        self.dispatch("DrawUnit", |ctx| {
            for (i, addon) in self.addons.iter().enumerate() {
                if self.enabled[i].get() && addon.draw_unit(ctx, unit_id, draw_mode).is_handled() {
                    return true;
                }
            }
            false
        })
    }

    pub fn draw_feature(&self, feature_id: i32, draw_mode: i32) -> bool {
        self.dispatch("DrawFeature", |ctx| {
            for (i, addon) in self.addons.iter().enumerate() {
                if self.enabled[i].get()
                    && addon.draw_feature(ctx, feature_id, draw_mode).is_handled()
                {
                    return true;
                }
            }
            false
        })
    }

    pub fn draw_world_pre_unit(&self) {
        self.dispatch("DrawWorldPreUnit", |ctx| {
            for (i, addon) in self.addons.iter().enumerate() {
                if self.enabled[i].get() {
                    addon.draw_world_pre_unit(ctx);
                }
            }
        });
    }

    pub fn draw_genesis(&self) {
        self.dispatch("DrawGenesis", |ctx| {
            for (i, addon) in self.addons.iter().enumerate() {
                if self.enabled[i].get() {
                    addon.draw_genesis(ctx);
                }
            }
        });
    }

    pub fn view_resize(&self, geometry: &ViewGeometry) {
        self.dispatch("ViewResize", |ctx| {
            for (i, addon) in self.addons.iter().enumerate() {
                if self.enabled[i].get() {
                    addon.view_resize(ctx, geometry);
                }
            }
        });
    }
}
