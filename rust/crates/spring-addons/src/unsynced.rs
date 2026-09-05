//! The unsynced-rules environment.

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cell::Cell;

use crate::event::ViewGeometry;
use crate::runtime::{AddonContext, AddonRuntime};

pub trait UnsyncedAddon<G = ()> {
    fn name(&self) -> &'static str;

    fn is_enabled(&self) -> bool {
        true
    }

    fn init(&self, _ctx: &AddonContext<'_, G>) {}
    fn shutdown(&self, _ctx: &AddonContext<'_, G>) {}
    fn game_frame(&self, _ctx: &AddonContext<'_, G>, _frame: i32) {}
    fn draw_world_pre_unit(&self, _ctx: &AddonContext<'_, G>) {}
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

    pub fn draw_world_pre_unit(&self) {
        self.dispatch("DrawWorldPreUnit", |ctx| {
            for (i, addon) in self.addons.iter().enumerate() {
                if self.enabled[i].get() {
                    addon.draw_world_pre_unit(ctx);
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
