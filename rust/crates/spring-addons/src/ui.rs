use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::{Cell, RefCell};

use crate::event::{EventResult, KeyEvent, UnitDestroyedEvent, ViewGeometry};
use crate::runtime::{AddonContext, AddonRuntime};

pub trait Widget<G = ()> {
    fn name(&self) -> &'static str;

    fn is_enabled(&self) -> bool {
        true
    }

    fn init(&self, _ctx: &AddonContext<'_, G>) {}
    fn shutdown(&self, _ctx: &AddonContext<'_, G>) {}

    fn update(&self, _ctx: &AddonContext<'_, G>, _dt: f32) {}
    fn unit_created(
        &self,
        _ctx: &AddonContext<'_, G>,
        _unit_id: i32,
        _unit_def_id: i32,
        _unit_team: i32,
        _builder_id: i32,
    ) {
    }
    fn feature_created(&self, _ctx: &AddonContext<'_, G>, _feature_id: i32, _ally_team_id: i32) {}
    fn feature_destroyed(&self, _ctx: &AddonContext<'_, G>, _feature_id: i32, _ally_team_id: i32) {}
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
    fn unit_entered_los(
        &self,
        _ctx: &AddonContext<'_, G>,
        _unit_id: i32,
        _unit_def_id: i32,
        _unit_team: i32,
        _ally_team: i32,
    ) {
    }
    fn unit_left_los(
        &self,
        _ctx: &AddonContext<'_, G>,
        _unit_id: i32,
        _unit_def_id: i32,
        _unit_team: i32,
        _ally_team: i32,
    ) {
    }
    fn unit_entered_radar(
        &self,
        _ctx: &AddonContext<'_, G>,
        _unit_id: i32,
        _unit_def_id: i32,
        _unit_team: i32,
        _ally_team: i32,
    ) {
    }
    fn unit_left_radar(
        &self,
        _ctx: &AddonContext<'_, G>,
        _unit_id: i32,
        _unit_def_id: i32,
        _unit_team: i32,
        _ally_team: i32,
    ) {
    }
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
    fn draw_screen(&self, _ctx: &AddonContext<'_, G>, _width: i32, _height: i32) {}
    fn draw_genesis(&self, _ctx: &AddonContext<'_, G>) {}
    fn draw_world(&self, _ctx: &AddonContext<'_, G>) {}

    fn key_press(&self, _ctx: &AddonContext<'_, G>, _event: &KeyEvent<'_>) -> EventResult {
        EventResult::Ignored
    }
    fn key_release(&self, _ctx: &AddonContext<'_, G>, _event: &KeyEvent<'_>) -> EventResult {
        EventResult::Ignored
    }
    fn mouse_move(
        &self,
        _ctx: &AddonContext<'_, G>,
        _x: i32,
        _y: i32,
        _dx: i32,
        _dy: i32,
        _button: i32,
    ) -> EventResult {
        EventResult::Ignored
    }
    fn mouse_press(
        &self,
        _ctx: &AddonContext<'_, G>,
        _x: i32,
        _y: i32,
        _button: i32,
    ) -> EventResult {
        EventResult::Ignored
    }
    fn mouse_release(
        &self,
        _ctx: &AddonContext<'_, G>,
        _x: i32,
        _y: i32,
        _button: i32,
    ) -> EventResult {
        EventResult::Ignored
    }
    fn mouse_wheel(&self, _ctx: &AddonContext<'_, G>, _up: bool, _value: f32) -> EventResult {
        EventResult::Ignored
    }

    fn draw_world_pre_unit(&self, _ctx: &AddonContext<'_, G>) {}
    fn draw_world_refraction(&self, _ctx: &AddonContext<'_, G>) {}
    fn draw_screen_effects(&self, _ctx: &AddonContext<'_, G>, _view_width: i32, _view_height: i32) {
    }
    fn draw_unit(&self, _ctx: &AddonContext<'_, G>, _unit_id: i32, _draw_mode: i32) -> EventResult {
        EventResult::Ignored
    }
    fn view_resize(&self, _ctx: &AddonContext<'_, G>, _geometry: &ViewGeometry) {}
    fn game_over(&self, _ctx: &AddonContext<'_, G>, _winning_ally_teams: &[u8]) {}
    fn recv_from_synced(&self, _ctx: &AddonContext<'_, G>, _message: &[u8]) {}
}

pub struct WidgetHandler<G> {
    pub global: G,
    widgets: Vec<Box<dyn Widget<G>>>,
    enabled: Vec<Cell<bool>>,
    runtime: AddonRuntime<G>,
}

impl<G> WidgetHandler<G> {
    pub fn new(global: G) -> Self {
        Self {
            global,
            widgets: Vec::new(),
            enabled: Vec::new(),
            runtime: AddonRuntime::new(),
        }
    }

    pub fn add(&mut self, widget: Box<dyn Widget<G>>) {
        let is_enabled = widget.is_enabled();
        self.widgets.push(widget);
        self.enabled.push(Cell::new(is_enabled));
    }

    #[inline]
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
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.init(ctx);
                }
            }
        });
    }

    pub fn set_enabled(&self, name: &str, enabled: bool) {
        self.dispatch("SetEnabled", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if widget.name() == name {
                    if self.enabled[i].replace(enabled) != enabled {
                        if enabled {
                            widget.init(ctx);
                        } else {
                            widget.shutdown(ctx);
                        }
                    }
                    break;
                }
            }
        });
    }

    pub fn is_widget_enabled(&self, name: &str) -> bool {
        self.widgets
            .iter()
            .enumerate()
            .find_map(|(i, widget)| (widget.name() == name).then(|| self.enabled[i].get()))
            .unwrap_or(false)
    }

    pub fn update(&self, dt: f32) {
        self.dispatch("Update", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.update(ctx, dt);
                }
            }
        });
    }

    pub fn unit_created(&self, unit_id: i32, unit_def_id: i32, unit_team: i32, builder_id: i32) {
        self.dispatch("UnitCreated", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.unit_created(ctx, unit_id, unit_def_id, unit_team, builder_id);
                }
            }
        });
    }

    pub fn feature_created(&self, feature_id: i32, ally_team_id: i32) {
        self.dispatch("FeatureCreated", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.feature_created(ctx, feature_id, ally_team_id);
                }
            }
        });
    }

    pub fn feature_destroyed(&self, feature_id: i32, ally_team_id: i32) {
        self.dispatch("FeatureDestroyed", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.feature_destroyed(ctx, feature_id, ally_team_id);
                }
            }
        });
    }

    pub fn unit_destroyed(&self, event: &UnitDestroyedEvent) {
        self.dispatch("UnitDestroyed", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.unit_destroyed(ctx, event);
                }
            }
        });
    }

    pub fn unit_given(&self, unit_id: i32, unit_def_id: i32, old_team: i32, new_team: i32) {
        self.dispatch("UnitGiven", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.unit_given(ctx, unit_id, unit_def_id, old_team, new_team);
                }
            }
        });
    }

    pub fn unit_taken(&self, unit_id: i32, unit_def_id: i32, old_team: i32, new_team: i32) {
        self.dispatch("UnitTaken", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.unit_taken(ctx, unit_id, unit_def_id, old_team, new_team);
                }
            }
        });
    }

    pub fn unit_entered_los(&self, unit_id: i32, unit_def_id: i32, unit_team: i32, ally_team: i32) {
        self.dispatch("UnitEnteredLos", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.unit_entered_los(ctx, unit_id, unit_def_id, unit_team, ally_team);
                }
            }
        });
    }

    pub fn unit_left_los(&self, unit_id: i32, unit_def_id: i32, unit_team: i32, ally_team: i32) {
        self.dispatch("UnitLeftLos", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.unit_left_los(ctx, unit_id, unit_def_id, unit_team, ally_team);
                }
            }
        });
    }

    pub fn unit_entered_radar(
        &self,
        unit_id: i32,
        unit_def_id: i32,
        unit_team: i32,
        ally_team: i32,
    ) {
        self.dispatch("UnitEnteredRadar", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.unit_entered_radar(ctx, unit_id, unit_def_id, unit_team, ally_team);
                }
            }
        });
    }

    pub fn unit_left_radar(&self, unit_id: i32, unit_def_id: i32, unit_team: i32, ally_team: i32) {
        self.dispatch("UnitLeftRadar", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.unit_left_radar(ctx, unit_id, unit_def_id, unit_team, ally_team);
                }
            }
        });
    }

    pub fn projectile_created(&self, projectile_id: i32, owner_id: i32, weapon_def_id: i32) {
        self.dispatch("ProjectileCreated", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.projectile_created(ctx, projectile_id, owner_id, weapon_def_id);
                }
            }
        });
    }

    pub fn projectile_destroyed(&self, projectile_id: i32, owner_id: i32, weapon_def_id: i32) {
        self.dispatch("ProjectileDestroyed", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.projectile_destroyed(ctx, projectile_id, owner_id, weapon_def_id);
                }
            }
        });
    }

    pub fn draw_screen(&self, width: i32, height: i32) {
        self.dispatch("DrawScreen", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.draw_screen(ctx, width, height);
                }
            }
        });
    }

    pub fn draw_world(&self) {
        self.dispatch("DrawWorld", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.draw_world(ctx);
                }
            }
        });
    }

    pub fn draw_genesis(&self) {
        self.dispatch("DrawGenesis", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.draw_genesis(ctx);
                }
            }
        });
    }

    pub fn key_press(&self, event: &KeyEvent<'_>) -> bool {
        self.dispatch("KeyPress", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() && widget.key_press(ctx, event).is_handled() {
                    return true;
                }
            }
            false
        })
    }

    pub fn key_release(&self, event: &KeyEvent<'_>) -> bool {
        self.dispatch("KeyRelease", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() && widget.key_release(ctx, event).is_handled() {
                    return true;
                }
            }
            false
        })
    }

    pub fn mouse_move(&self, x: i32, y: i32, dx: i32, dy: i32, button: i32) -> bool {
        self.dispatch("MouseMove", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get()
                    && widget.mouse_move(ctx, x, y, dx, dy, button).is_handled()
                {
                    return true;
                }
            }
            false
        })
    }

    pub fn mouse_press(&self, x: i32, y: i32, button: i32) -> bool {
        self.dispatch("MousePress", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() && widget.mouse_press(ctx, x, y, button).is_handled() {
                    return true;
                }
            }
            false
        })
    }

    pub fn mouse_release(&self, x: i32, y: i32, button: i32) {
        self.dispatch("MouseRelease", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() && widget.mouse_release(ctx, x, y, button).is_handled() {
                    return;
                }
            }
        });
    }

    pub fn mouse_wheel(&self, up: bool, value: f32) -> bool {
        self.dispatch("MouseWheel", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() && widget.mouse_wheel(ctx, up, value).is_handled() {
                    return true;
                }
            }
            false
        })
    }

    pub fn draw_world_pre_unit(&self) {
        self.dispatch("DrawWorldPreUnit", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.draw_world_pre_unit(ctx);
                }
            }
        });
    }

    pub fn draw_world_refraction(&self) {
        self.dispatch("DrawWorldRefraction", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.draw_world_refraction(ctx);
                }
            }
        });
    }

    pub fn draw_screen_effects(&self, view_width: i32, view_height: i32) {
        self.dispatch("DrawScreenEffects", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.draw_screen_effects(ctx, view_width, view_height);
                }
            }
        });
    }

    pub fn draw_unit(&self, unit_id: i32, draw_mode: i32) -> bool {
        self.dispatch("DrawUnit", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() && widget.draw_unit(ctx, unit_id, draw_mode).is_handled() {
                    return true;
                }
            }
            false
        })
    }

    pub fn view_resize(&self, geometry: &ViewGeometry) {
        self.dispatch("ViewResize", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.view_resize(ctx, geometry);
                }
            }
        });
    }

    pub fn game_over(&self, winning_ally_teams: &[u8]) {
        self.dispatch("GameOver", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.game_over(ctx, winning_ally_teams);
                }
            }
        });
    }

    pub fn recv_from_synced(&self, message: &[u8]) {
        self.dispatch("RecvFromSynced", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.recv_from_synced(ctx, message);
                }
            }
        });
    }
}

const DESTROY_BIT: u32 = 0x8000_0000;
type UiCallback<G> = Rc<RefCell<Box<dyn FnMut(&G)>>>;

/// Retained UI callbacks use per-callback runtime borrowing rather than one
/// registry-wide mutable borrow. Different callbacks may therefore re-enter
/// each other; recursively invoking the same `FnMut` still fails loudly.
pub struct UiCallbackRegistry<G> {
    next_id: Cell<u32>,
    callbacks: RefCell<BTreeMap<u32, UiCallback<G>>>,
}

impl<G> Default for UiCallbackRegistry<G> {
    fn default() -> Self {
        Self::new()
    }
}

impl<G> UiCallbackRegistry<G> {
    pub const fn new() -> Self {
        Self {
            next_id: Cell::new(0x1000),
            callbacks: RefCell::new(BTreeMap::new()),
        }
    }

    pub fn register(
        &self,
        callback: impl FnMut(&G) + 'static,
    ) -> spring::callback::RetainedCallback {
        let id = self.next_id.get();
        self.next_id.set(id + 1);
        debug_assert!(
            id & DESTROY_BIT == 0,
            "callback id space exhausted; ids must stay below the destroy bit"
        );
        let destroy_id = id | DESTROY_BIT;
        self.callbacks
            .borrow_mut()
            .insert(id, Rc::new(RefCell::new(Box::new(callback))));
        spring::callback::RetainedCallback::new(id, 0, destroy_id)
    }

    pub fn unregister(&self, callback: spring::callback::RetainedCallback) {
        self.callbacks
            .borrow_mut()
            .remove(&(callback.id & !DESTROY_BIT));
    }

    pub fn dispatch(&self, global: &G, callback_id: u32, _user_data: u32) -> bool {
        if callback_id & DESTROY_BIT != 0 {
            self.callbacks
                .borrow_mut()
                .remove(&(callback_id & !DESTROY_BIT));
            return true;
        }

        let callback = self.callbacks.borrow().get(&callback_id).cloned();
        if let Some(callback) = callback {
            callback.borrow_mut()(global);
            true
        } else {
            false
        }
    }
}

/// Widget that hides parts of the engine's built-in interface
/// (`spring::default_interface`) for as long as it is enabled.
///
/// ```ignore
/// use spring::default_interface::{ALL, MINIMAP};
/// handler.add(HideDefaultInterface::new(ALL & !MINIMAP));
/// ```
pub struct HideDefaultInterface {
    parts: u32,
}

impl HideDefaultInterface {
    pub fn new(parts: u32) -> Self {
        Self { parts }
    }
}

impl<G> Widget<G> for HideDefaultInterface {
    fn name(&self) -> &'static str {
        "HideDefaultInterface"
    }

    fn init(&self, _ctx: &AddonContext<'_, G>) {
        #[cfg(target_arch = "wasm32")]
        if let Err(error) = spring::default_interface::hide(self.parts) {
            crate::log::warning(&alloc::format!(
                "HideDefaultInterface: hiding failed: {error:?}"
            ));
        }
    }

    fn shutdown(&self, _ctx: &AddonContext<'_, G>) {
        #[cfg(target_arch = "wasm32")]
        let _ = spring::default_interface::show(self.parts);
        #[cfg(not(target_arch = "wasm32"))]
        let _ = self.parts;
    }
}
