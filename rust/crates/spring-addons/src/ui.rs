use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::event::{EventResult, KeyEvent};

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

    pub fn recv_from_synced(&mut self, message: &[u8]) {
        for (i, widget) in self.widgets.iter_mut().enumerate() {
            if self.enabled[i] {
                widget.recv_from_synced(&mut self.global, message);
            }
        }
    }
}
