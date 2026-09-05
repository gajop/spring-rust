use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::event::{EventResult, KeyEvent, ViewGeometry};

pub trait Widget<G = ()> {
    fn name(&self) -> &'static str;

    fn is_enabled(&self) -> bool {
        true
    }

    fn init(&mut self, _global: &mut G) {}
    fn shutdown(&mut self, _global: &mut G) {}

    fn update(&mut self, _global: &mut G, _dt: f32) {}
    fn draw_screen(&mut self, _global: &mut G, _width: i32, _height: i32) {}
    fn draw_world(&mut self, _global: &mut G) {}

    fn key_press(&mut self, _global: &mut G, _event: &KeyEvent<'_>) -> EventResult {
        EventResult::Ignored
    }
    fn key_release(&mut self, _global: &mut G, _event: &KeyEvent<'_>) -> EventResult {
        EventResult::Ignored
    }

    fn mouse_move(
        &mut self,
        _global: &mut G,
        _x: i32,
        _y: i32,
        _dx: i32,
        _dy: i32,
        _button: i32,
    ) -> EventResult {
        EventResult::Ignored
    }
    fn mouse_press(&mut self, _global: &mut G, _x: i32, _y: i32, _button: i32) -> EventResult {
        EventResult::Ignored
    }
    fn mouse_release(&mut self, _global: &mut G, _x: i32, _y: i32, _button: i32) -> EventResult {
        EventResult::Ignored
    }

    fn draw_world_pre_unit(&mut self, _global: &mut G) {}
    fn draw_world_refraction(&mut self, _global: &mut G) {}
    fn draw_screen_effects(&mut self, _global: &mut G, _view_width: i32, _view_height: i32) {}

    /// Return `Handled` to take over drawing this unit and suppress the engine's
    /// own draw.
    fn draw_unit(&mut self, _global: &mut G, _unit_id: i32, _draw_mode: i32) -> EventResult {
        EventResult::Ignored
    }

    fn view_resize(&mut self, _global: &mut G, _geometry: &ViewGeometry) {}

    fn game_over(&mut self, _global: &mut G, _winning_ally_teams: &[u8]) {}

    fn recv_from_synced(&mut self, _global: &mut G, _message: &[u8]) {}
}

pub struct WidgetHandler<G> {
    pub global: G,
    widgets: Vec<Box<dyn Widget<G>>>,
    enabled: Vec<bool>,
}

impl<G> WidgetHandler<G> {
    pub fn new(global: G) -> Self {
        Self {
            global,
            widgets: Vec::new(),
            enabled: Vec::new(),
        }
    }

    pub fn add(&mut self, widget: Box<dyn Widget<G>>) {
        let is_enabled = widget.is_enabled();
        self.widgets.push(widget);
        self.enabled.push(is_enabled);
    }

    pub fn init(&mut self) {
        for (i, widget) in self.widgets.iter_mut().enumerate() {
            if self.enabled[i] {
                widget.init(&mut self.global);
            }
        }
    }

    pub fn set_enabled(&mut self, name: &str, enabled: bool) {
        for (i, widget) in self.widgets.iter_mut().enumerate() {
            if widget.name() == name {
                if self.enabled[i] != enabled {
                    self.enabled[i] = enabled;
                    if enabled {
                        widget.init(&mut self.global);
                    } else {
                        widget.shutdown(&mut self.global);
                    }
                }
                break;
            }
        }
    }

    pub fn is_widget_enabled(&self, name: &str) -> bool {
        for (i, widget) in self.widgets.iter().enumerate() {
            if widget.name() == name {
                return self.enabled[i];
            }
        }
        false
    }

    pub fn update(&mut self, dt: f32) {
        for (i, widget) in self.widgets.iter_mut().enumerate() {
            if self.enabled[i] {
                widget.update(&mut self.global, dt);
            }
        }
    }

    pub fn draw_screen(&mut self, width: i32, height: i32) {
        for (i, widget) in self.widgets.iter_mut().enumerate() {
            if self.enabled[i] {
                widget.draw_screen(&mut self.global, width, height);
            }
        }
    }

    pub fn draw_world(&mut self) {
        for (i, widget) in self.widgets.iter_mut().enumerate() {
            if self.enabled[i] {
                widget.draw_world(&mut self.global);
            }
        }
    }

    pub fn key_press(&mut self, event: &KeyEvent<'_>) -> bool {
        for (i, widget) in self.widgets.iter_mut().enumerate() {
            if self.enabled[i] && widget.key_press(&mut self.global, event).is_handled() {
                return true;
            }
        }
        false
    }

    pub fn key_release(&mut self, event: &KeyEvent<'_>) -> bool {
        for (i, widget) in self.widgets.iter_mut().enumerate() {
            if self.enabled[i] && widget.key_release(&mut self.global, event).is_handled() {
                return true;
            }
        }
        false
    }

    pub fn mouse_move(&mut self, x: i32, y: i32, dx: i32, dy: i32, button: i32) -> bool {
        for (i, widget) in self.widgets.iter_mut().enumerate() {
            if self.enabled[i]
                && widget
                    .mouse_move(&mut self.global, x, y, dx, dy, button)
                    .is_handled()
            {
                return true;
            }
        }
        false
    }

    pub fn mouse_press(&mut self, x: i32, y: i32, button: i32) -> bool {
        for (i, widget) in self.widgets.iter_mut().enumerate() {
            if self.enabled[i]
                && widget
                    .mouse_press(&mut self.global, x, y, button)
                    .is_handled()
            {
                return true;
            }
        }
        false
    }

    pub fn mouse_release(&mut self, x: i32, y: i32, button: i32) {
        for (i, widget) in self.widgets.iter_mut().enumerate() {
            if self.enabled[i]
                && widget
                    .mouse_release(&mut self.global, x, y, button)
                    .is_handled()
            {
                return;
            }
        }
    }

    pub fn draw_world_pre_unit(&mut self) {
        for (i, widget) in self.widgets.iter_mut().enumerate() {
            if self.enabled[i] {
                widget.draw_world_pre_unit(&mut self.global);
            }
        }
    }

    pub fn draw_world_refraction(&mut self) {
        for (i, widget) in self.widgets.iter_mut().enumerate() {
            if self.enabled[i] {
                widget.draw_world_refraction(&mut self.global);
            }
        }
    }

    pub fn draw_screen_effects(&mut self, view_width: i32, view_height: i32) {
        for (i, widget) in self.widgets.iter_mut().enumerate() {
            if self.enabled[i] {
                widget.draw_screen_effects(&mut self.global, view_width, view_height);
            }
        }
    }

    /// First widget to claim the unit wins, matching the Lua handler, which
    /// stops at the first `DrawUnit` returning true.
    pub fn draw_unit(&mut self, unit_id: i32, draw_mode: i32) -> bool {
        for (i, widget) in self.widgets.iter_mut().enumerate() {
            if self.enabled[i]
                && widget
                    .draw_unit(&mut self.global, unit_id, draw_mode)
                    .is_handled()
            {
                return true;
            }
        }
        false
    }

    pub fn view_resize(&mut self, geometry: &ViewGeometry) {
        for (i, widget) in self.widgets.iter_mut().enumerate() {
            if self.enabled[i] {
                widget.view_resize(&mut self.global, geometry);
            }
        }
    }

    pub fn game_over(&mut self, winning_ally_teams: &[u8]) {
        for (i, widget) in self.widgets.iter_mut().enumerate() {
            if self.enabled[i] {
                widget.game_over(&mut self.global, winning_ally_teams);
            }
        }
    }

    pub fn recv_from_synced(&mut self, message: &[u8]) {
        for (i, widget) in self.widgets.iter_mut().enumerate() {
            if self.enabled[i] {
                widget.recv_from_synced(&mut self.global, message);
            }
        }
    }
}

/// High bit of a callback ID, set by the engine to signal that the retained
/// callback has been destroyed and the guest-side closure can be dropped.
const DESTROY_BIT: u32 = 0x8000_0000;

pub struct UiCallbackRegistry<G> {
    next_id: u32,
    callbacks: alloc::collections::BTreeMap<u32, Box<dyn FnMut(&mut G)>>,
}

impl<G> Default for UiCallbackRegistry<G> {
    fn default() -> Self {
        Self::new()
    }
}

impl<G> UiCallbackRegistry<G> {
    pub const fn new() -> Self {
        Self {
            next_id: 0x1000,
            callbacks: alloc::collections::BTreeMap::new(),
        }
    }

    pub fn register(
        &mut self,
        callback: impl FnMut(&mut G) + 'static,
    ) -> spring::callback::RetainedCallback {
        let id = self.next_id;
        self.next_id += 1;
        debug_assert!(
            id & DESTROY_BIT == 0,
            "callback id space exhausted; ids must stay below the destroy bit"
        );
        let destroy_id = id | DESTROY_BIT;
        self.callbacks.insert(id, Box::new(callback));
        spring::callback::RetainedCallback::new(id, 0, destroy_id)
    }

    /// Drop a callback that was registered but never handed to the engine.
    ///
    /// Registration allocates an entry that is normally released when the engine
    /// fires `destroy_id`. If binding the callback fails, the engine never learns
    /// about it and never fires that destroy, so the caller must release it here
    /// or the entry leaks for the lifetime of the process.
    pub fn unregister(&mut self, callback: spring::callback::RetainedCallback) {
        self.callbacks.remove(&(callback.id & !DESTROY_BIT));
    }

    pub fn dispatch(&mut self, global: &mut G, callback_id: u32, _user_data: u32) -> bool {
        if callback_id & DESTROY_BIT != 0 {
            let id = callback_id & !DESTROY_BIT;
            self.callbacks.remove(&id);
            return true;
        }
        if let Some(cb) = self.callbacks.get_mut(&callback_id) {
            cb(global);
            true
        } else {
            false
        }
    }
}

/// Report a callback ID that no registered closure and no game-level handler
/// claimed. A mis-bound RmlUi event is otherwise completely silent.
pub fn warn_unhandled_callback(callback_id: u32) {
    let mut buffer = [0u8; 8];
    for (index, slot) in buffer.iter_mut().enumerate() {
        let nibble = (callback_id >> (28 - index * 4)) & 0xf;
        *slot = match nibble {
            0..=9 => b'0' + nibble as u8,
            value => b'a' + (value - 10) as u8,
        };
    }
    let hex = core::str::from_utf8(&buffer).unwrap_or("????????");
    let mut message = alloc::string::String::from("unhandled UI callback id 0x");
    message.push_str(hex);
    let _ = spring::log("spring-addons", 40, &message);
}
