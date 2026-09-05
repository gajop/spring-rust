//! The unsynced-rules environment.
//!
//! `LuaRules` has an unsynced half that draws and observes but cannot affect
//! the simulation. It receives a much smaller set of call-ins than either a
//! widget or a gadget, so it gets its own trait rather than reusing
//! [`crate::ui::Widget`], whose input and screen call-ins are never delivered
//! here.

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::event::ViewGeometry;

pub trait UnsyncedAddon<G = ()> {
    fn name(&self) -> &'static str;

    fn is_enabled(&self) -> bool {
        true
    }

    fn init(&mut self, _global: &mut G) {}
    fn shutdown(&mut self, _global: &mut G) {}

    fn game_frame(&mut self, _global: &mut G, _frame: i32) {}
    fn draw_world_pre_unit(&mut self, _global: &mut G) {}
    fn view_resize(&mut self, _global: &mut G, _geometry: &ViewGeometry) {}
}

pub struct UnsyncedHandler<G> {
    pub global: G,
    addons: Vec<Box<dyn UnsyncedAddon<G>>>,
    enabled: Vec<bool>,
}

impl<G> UnsyncedHandler<G> {
    pub fn new(global: G) -> Self {
        Self {
            global,
            addons: Vec::new(),
            enabled: Vec::new(),
        }
    }

    pub fn add(&mut self, addon: Box<dyn UnsyncedAddon<G>>) {
        let is_enabled = addon.is_enabled();
        self.addons.push(addon);
        self.enabled.push(is_enabled);
    }

    pub fn init(&mut self) {
        for (i, addon) in self.addons.iter_mut().enumerate() {
            if self.enabled[i] {
                addon.init(&mut self.global);
            }
        }
    }

    pub fn set_enabled(&mut self, name: &str, enabled: bool) {
        for (i, addon) in self.addons.iter_mut().enumerate() {
            if addon.name() == name {
                if self.enabled[i] != enabled {
                    self.enabled[i] = enabled;
                    if enabled {
                        addon.init(&mut self.global);
                    } else {
                        addon.shutdown(&mut self.global);
                    }
                }
                break;
            }
        }
    }

    pub fn game_frame(&mut self, frame: i32) {
        for (i, addon) in self.addons.iter_mut().enumerate() {
            if self.enabled[i] {
                addon.game_frame(&mut self.global, frame);
            }
        }
    }

    pub fn draw_world_pre_unit(&mut self) {
        for (i, addon) in self.addons.iter_mut().enumerate() {
            if self.enabled[i] {
                addon.draw_world_pre_unit(&mut self.global);
            }
        }
    }

    pub fn view_resize(&mut self, geometry: &ViewGeometry) {
        for (i, addon) in self.addons.iter_mut().enumerate() {
            if self.enabled[i] {
                addon.view_resize(&mut self.global, geometry);
            }
        }
    }
}
