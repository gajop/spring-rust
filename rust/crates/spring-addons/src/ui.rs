use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cell::Cell;

use crate::event::{EventResult, KeyEvent, UnitDestroyedEvent, ViewGeometry};
use crate::runtime::{AddonContext, AddonRuntime};

pub trait Widget<G = (), E: spring::capabilities::Common = spring::Ui>:
    spring::RetainedCapture
{
    fn name(&self) -> &'static str;

    fn is_enabled(&self) -> bool {
        true
    }

    fn init(&self, _ctx: &AddonContext<'_, G, E>) {}
    fn shutdown(&self, _ctx: &AddonContext<'_, G, E>) {}

    fn update(&self, _ctx: &AddonContext<'_, G, E>, _dt: f32) {}
    fn job_result(
        &self,
        _ctx: &AddonContext<'_, G, E>,
        _id: spring::jobs::JobId,
        _result: spring::jobs::JobResult<'_>,
    ) {
    }
    fn unit_created(
        &self,
        _ctx: &AddonContext<'_, G, E>,
        _unit_id: i32,
        _unit_def_id: i32,
        _unit_team: i32,
        _builder_id: i32,
    ) {
    }
    fn feature_created(&self, _ctx: &AddonContext<'_, G, E>, _feature_id: i32, _ally_team_id: i32) {
    }
    fn feature_destroyed(
        &self,
        _ctx: &AddonContext<'_, G, E>,
        _feature_id: i32,
        _ally_team_id: i32,
    ) {
    }
    fn unit_destroyed(&self, _ctx: &AddonContext<'_, G, E>, _event: &UnitDestroyedEvent) {}
    fn unit_given(
        &self,
        _ctx: &AddonContext<'_, G, E>,
        _unit_id: i32,
        _unit_def_id: i32,
        _old_team: i32,
        _new_team: i32,
    ) {
    }
    fn unit_taken(
        &self,
        _ctx: &AddonContext<'_, G, E>,
        _unit_id: i32,
        _unit_def_id: i32,
        _old_team: i32,
        _new_team: i32,
    ) {
    }
    fn unit_entered_los(
        &self,
        _ctx: &AddonContext<'_, G, E>,
        _unit_id: i32,
        _unit_def_id: i32,
        _unit_team: i32,
        _ally_team: i32,
    ) {
    }
    fn unit_left_los(
        &self,
        _ctx: &AddonContext<'_, G, E>,
        _unit_id: i32,
        _unit_def_id: i32,
        _unit_team: i32,
        _ally_team: i32,
    ) {
    }
    fn unit_entered_radar(
        &self,
        _ctx: &AddonContext<'_, G, E>,
        _unit_id: i32,
        _unit_def_id: i32,
        _unit_team: i32,
        _ally_team: i32,
    ) {
    }
    fn unit_left_radar(
        &self,
        _ctx: &AddonContext<'_, G, E>,
        _unit_id: i32,
        _unit_def_id: i32,
        _unit_team: i32,
        _ally_team: i32,
    ) {
    }
    fn projectile_created(
        &self,
        _ctx: &AddonContext<'_, G, E>,
        _projectile_id: i32,
        _owner_id: i32,
        _weapon_def_id: i32,
    ) {
    }
    fn projectile_destroyed(
        &self,
        _ctx: &AddonContext<'_, G, E>,
        _projectile_id: i32,
        _owner_id: i32,
        _weapon_def_id: i32,
    ) {
    }
    fn draw_screen(&self, _ctx: &AddonContext<'_, G, E>, _width: i32, _height: i32) {}
    fn draw_genesis(&self, _ctx: &AddonContext<'_, G, E>) {}
    fn draw_world(&self, _ctx: &AddonContext<'_, G, E>) {}

    fn key_press(&self, _ctx: &AddonContext<'_, G, E>, _event: &KeyEvent<'_>) -> EventResult {
        EventResult::Ignored
    }
    fn key_release(&self, _ctx: &AddonContext<'_, G, E>, _event: &KeyEvent<'_>) -> EventResult {
        EventResult::Ignored
    }
    fn mouse_move(
        &self,
        _ctx: &AddonContext<'_, G, E>,
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
        _ctx: &AddonContext<'_, G, E>,
        _x: i32,
        _y: i32,
        _button: i32,
    ) -> EventResult {
        EventResult::Ignored
    }
    fn mouse_release(
        &self,
        _ctx: &AddonContext<'_, G, E>,
        _x: i32,
        _y: i32,
        _button: i32,
    ) -> EventResult {
        EventResult::Ignored
    }
    fn mouse_wheel(&self, _ctx: &AddonContext<'_, G, E>, _up: bool, _value: f32) -> EventResult {
        EventResult::Ignored
    }

    fn draw_world_pre_unit(&self, _ctx: &AddonContext<'_, G, E>) {}
    fn draw_world_refraction(&self, _ctx: &AddonContext<'_, G, E>) {}
    fn draw_screen_effects(
        &self,
        _ctx: &AddonContext<'_, G, E>,
        _view_width: i32,
        _view_height: i32,
    ) {
    }
    fn view_resize(&self, _ctx: &AddonContext<'_, G, E>, _geometry: &ViewGeometry) {}
    fn game_over(&self, _ctx: &AddonContext<'_, G, E>, _winning_ally_teams: &[u8]) {}
    fn recv_from_synced(&self, _ctx: &AddonContext<'_, G, E>, _message: &[u8]) {}
}

pub struct WidgetHandler<G, E: spring::capabilities::Common = spring::Ui> {
    pub global: G,
    widgets: Vec<Box<dyn Widget<G, E>>>,
    enabled: Vec<Cell<bool>>,
    runtime: AddonRuntime<G, E>,
}

impl<G, E: spring::capabilities::Common> WidgetHandler<G, E> {
    pub fn new(global: G) -> Self {
        Self {
            global,
            widgets: Vec::new(),
            enabled: Vec::new(),
            runtime: AddonRuntime::new(),
        }
    }

    pub fn add(&mut self, widget: Box<dyn Widget<G, E>>) {
        let is_enabled = widget.is_enabled();
        self.widgets.push(widget);
        self.enabled.push(Cell::new(is_enabled));
    }

    #[inline]
    pub fn global(&self) -> &G {
        &self.global
    }

    pub fn with_context<R>(
        &self,
        spring: &spring::Spring<'_, E>,
        f: impl FnOnce(&AddonContext<'_, G, E>) -> R,
    ) -> R {
        self.runtime.callin("external", &self.global, spring, f)
    }

    fn dispatch<R>(
        &self,
        spring: &spring::Spring<'_, E>,
        callin: &'static str,
        f: impl FnOnce(&AddonContext<'_, G, E>) -> R,
    ) -> R {
        self.runtime.callin(callin, &self.global, spring, f)
    }

    pub fn init(&self, spring: &spring::Spring<'_, E>) {
        self.dispatch(spring, "Init", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.init(ctx);
                }
            }
        });
    }

    pub fn shutdown(&self, spring: &spring::Spring<'_, E>) {
        self.dispatch(spring, "Shutdown", |ctx| {
            for (i, addon) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    addon.shutdown(ctx);
                }
            }
        });
    }

    pub fn set_enabled(&self, spring: &spring::Spring<'_, E>, name: &str, enabled: bool) {
        self.dispatch(spring, "SetEnabled", |ctx| {
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

    pub fn update(&self, spring: &spring::Spring<'_, E>, dt: f32) {
        self.dispatch(spring, "Update", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.update(ctx, dt);
                }
            }
        });
    }

    pub fn job_result(
        &self,
        spring: &spring::Spring<'_, E>,
        id: spring::jobs::JobId,
        result: spring::jobs::JobResult<'_>,
    ) {
        self.dispatch(spring, "JobResult", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.job_result(ctx, id, result);
                }
            }
        });
    }

    pub fn unit_created(
        &self,
        spring: &spring::Spring<'_, E>,
        unit_id: i32,
        unit_def_id: i32,
        unit_team: i32,
        builder_id: i32,
    ) {
        self.dispatch(spring, "UnitCreated", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.unit_created(ctx, unit_id, unit_def_id, unit_team, builder_id);
                }
            }
        });
    }

    pub fn feature_created(
        &self,
        spring: &spring::Spring<'_, E>,
        feature_id: i32,
        ally_team_id: i32,
    ) {
        self.dispatch(spring, "FeatureCreated", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.feature_created(ctx, feature_id, ally_team_id);
                }
            }
        });
    }

    pub fn feature_destroyed(
        &self,
        spring: &spring::Spring<'_, E>,
        feature_id: i32,
        ally_team_id: i32,
    ) {
        self.dispatch(spring, "FeatureDestroyed", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.feature_destroyed(ctx, feature_id, ally_team_id);
                }
            }
        });
    }

    pub fn unit_destroyed(&self, spring: &spring::Spring<'_, E>, event: &UnitDestroyedEvent) {
        self.dispatch(spring, "UnitDestroyed", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.unit_destroyed(ctx, event);
                }
            }
        });
    }

    pub fn unit_given(
        &self,
        spring: &spring::Spring<'_, E>,
        unit_id: i32,
        unit_def_id: i32,
        old_team: i32,
        new_team: i32,
    ) {
        self.dispatch(spring, "UnitGiven", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.unit_given(ctx, unit_id, unit_def_id, old_team, new_team);
                }
            }
        });
    }

    pub fn unit_taken(
        &self,
        spring: &spring::Spring<'_, E>,
        unit_id: i32,
        unit_def_id: i32,
        old_team: i32,
        new_team: i32,
    ) {
        self.dispatch(spring, "UnitTaken", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.unit_taken(ctx, unit_id, unit_def_id, old_team, new_team);
                }
            }
        });
    }

    pub fn unit_entered_los(
        &self,
        spring: &spring::Spring<'_, E>,
        unit_id: i32,
        unit_def_id: i32,
        unit_team: i32,
        ally_team: i32,
    ) {
        self.dispatch(spring, "UnitEnteredLos", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.unit_entered_los(ctx, unit_id, unit_def_id, unit_team, ally_team);
                }
            }
        });
    }

    pub fn unit_left_los(
        &self,
        spring: &spring::Spring<'_, E>,
        unit_id: i32,
        unit_def_id: i32,
        unit_team: i32,
        ally_team: i32,
    ) {
        self.dispatch(spring, "UnitLeftLos", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.unit_left_los(ctx, unit_id, unit_def_id, unit_team, ally_team);
                }
            }
        });
    }

    pub fn unit_entered_radar(
        &self,
        spring: &spring::Spring<'_, E>,
        unit_id: i32,
        unit_def_id: i32,
        unit_team: i32,
        ally_team: i32,
    ) {
        self.dispatch(spring, "UnitEnteredRadar", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.unit_entered_radar(ctx, unit_id, unit_def_id, unit_team, ally_team);
                }
            }
        });
    }

    pub fn unit_left_radar(
        &self,
        spring: &spring::Spring<'_, E>,
        unit_id: i32,
        unit_def_id: i32,
        unit_team: i32,
        ally_team: i32,
    ) {
        self.dispatch(spring, "UnitLeftRadar", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.unit_left_radar(ctx, unit_id, unit_def_id, unit_team, ally_team);
                }
            }
        });
    }

    pub fn projectile_created(
        &self,
        spring: &spring::Spring<'_, E>,
        projectile_id: i32,
        owner_id: i32,
        weapon_def_id: i32,
    ) {
        self.dispatch(spring, "ProjectileCreated", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.projectile_created(ctx, projectile_id, owner_id, weapon_def_id);
                }
            }
        });
    }

    pub fn projectile_destroyed(
        &self,
        spring: &spring::Spring<'_, E>,
        projectile_id: i32,
        owner_id: i32,
        weapon_def_id: i32,
    ) {
        self.dispatch(spring, "ProjectileDestroyed", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.projectile_destroyed(ctx, projectile_id, owner_id, weapon_def_id);
                }
            }
        });
    }

    pub fn draw_screen(&self, spring: &spring::Spring<'_, E>, width: i32, height: i32) {
        self.dispatch(spring, "DrawScreen", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.draw_screen(ctx, width, height);
                }
            }
        });
    }

    pub fn draw_world(&self, spring: &spring::Spring<'_, E>) {
        self.dispatch(spring, "DrawWorld", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.draw_world(ctx);
                }
            }
        });
    }

    pub fn draw_genesis(&self, spring: &spring::Spring<'_, E>) {
        self.dispatch(spring, "DrawGenesis", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.draw_genesis(ctx);
                }
            }
        });
    }

    pub fn key_press(&self, spring: &spring::Spring<'_, E>, event: &KeyEvent<'_>) -> bool {
        self.dispatch(spring, "KeyPress", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() && widget.key_press(ctx, event).is_handled() {
                    return true;
                }
            }
            false
        })
    }

    pub fn key_release(&self, spring: &spring::Spring<'_, E>, event: &KeyEvent<'_>) -> bool {
        self.dispatch(spring, "KeyRelease", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() && widget.key_release(ctx, event).is_handled() {
                    return true;
                }
            }
            false
        })
    }

    pub fn mouse_move(
        &self,
        spring: &spring::Spring<'_, E>,
        x: i32,
        y: i32,
        dx: i32,
        dy: i32,
        button: i32,
    ) -> bool {
        self.dispatch(spring, "MouseMove", |ctx| {
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

    pub fn mouse_press(&self, spring: &spring::Spring<'_, E>, x: i32, y: i32, button: i32) -> bool {
        self.dispatch(spring, "MousePress", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() && widget.mouse_press(ctx, x, y, button).is_handled() {
                    return true;
                }
            }
            false
        })
    }

    pub fn mouse_release(&self, spring: &spring::Spring<'_, E>, x: i32, y: i32, button: i32) {
        self.dispatch(spring, "MouseRelease", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() && widget.mouse_release(ctx, x, y, button).is_handled() {
                    return;
                }
            }
        });
    }

    pub fn mouse_wheel(&self, spring: &spring::Spring<'_, E>, up: bool, value: f32) -> bool {
        self.dispatch(spring, "MouseWheel", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() && widget.mouse_wheel(ctx, up, value).is_handled() {
                    return true;
                }
            }
            false
        })
    }

    pub fn draw_world_pre_unit(&self, spring: &spring::Spring<'_, E>) {
        self.dispatch(spring, "DrawWorldPreUnit", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.draw_world_pre_unit(ctx);
                }
            }
        });
    }

    pub fn draw_world_refraction(&self, spring: &spring::Spring<'_, E>) {
        self.dispatch(spring, "DrawWorldRefraction", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.draw_world_refraction(ctx);
                }
            }
        });
    }

    pub fn draw_screen_effects(
        &self,
        spring: &spring::Spring<'_, E>,
        view_width: i32,
        view_height: i32,
    ) {
        self.dispatch(spring, "DrawScreenEffects", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.draw_screen_effects(ctx, view_width, view_height);
                }
            }
        });
    }

    pub fn view_resize(&self, spring: &spring::Spring<'_, E>, geometry: &ViewGeometry) {
        self.dispatch(spring, "ViewResize", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.view_resize(ctx, geometry);
                }
            }
        });
    }

    pub fn game_over(&self, spring: &spring::Spring<'_, E>, winning_ally_teams: &[u8]) {
        self.dispatch(spring, "GameOver", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.game_over(ctx, winning_ally_teams);
                }
            }
        });
    }

    pub fn recv_from_synced(&self, spring: &spring::Spring<'_, E>, message: &[u8]) {
        self.dispatch(spring, "RecvFromSynced", |ctx| {
            for (i, widget) in self.widgets.iter().enumerate() {
                if self.enabled[i].get() {
                    widget.recv_from_synced(ctx, message);
                }
            }
        });
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

impl<G, E: spring::capabilities::Common + spring::capabilities::Game> Widget<G, E>
    for HideDefaultInterface
{
    fn name(&self) -> &'static str {
        "HideDefaultInterface"
    }
    fn init(&self, ctx: &AddonContext<'_, G, E>) {
        use spring::ResultExt;
        ctx.spring()
            .default_interface()
            .hide(self.parts)
            .log_err(ctx.spring(), "hide default interface");
    }
    fn shutdown(&self, ctx: &AddonContext<'_, G, E>) {
        use spring::ResultExt;
        ctx.spring()
            .default_interface()
            .show(self.parts)
            .log_err(ctx.spring(), "restore default interface");
    }
}
