use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cell::Cell;

pub use crate::event::{CommandEvent, UnitDestroyedEvent, UnitPreDamagedEvent};
use crate::runtime::{AddonContext, AddonRuntime};

/// A synced rules addon, analogous to a Lua gadget.
///
/// Addons are invoked through shared references so the engine may synchronously
/// re-enter another callin while an outer callin is still on the stack. Mutable
/// game state should therefore live behind runtime-checked resource access in
/// the shared global container rather than in an exclusive handler-wide borrow.
pub trait Gadget<G = (), E: spring::capabilities::Common = spring::RulesSynced>:
    spring::RetainedCapture
{
    fn name(&self) -> &'static str;

    fn is_enabled(&self) -> bool {
        true
    }

    fn init(&self, _ctx: &AddonContext<'_, G, E>) {}
    fn shutdown(&self, _ctx: &AddonContext<'_, G, E>) {}

    fn game_frame(&self, _ctx: &AddonContext<'_, G, E>, _frame: i32) {}
    fn job_result(
        &self,
        _ctx: &AddonContext<'_, G, E>,
        _id: spring::jobs::JobId,
        _result: spring::jobs::JobResult<'_>,
    ) {
    }
    fn handle_lua_msg(
        &self,
        _ctx: &AddonContext<'_, G, E>,
        _player_id: i32,
        _script: i32,
        _mode: i32,
        _data: &[u8],
    ) {
    }
    fn unit_created(
        &self,
        _ctx: &AddonContext<'_, G, E>,
        _unit: i32,
        _def: i32,
        _team: i32,
        _builder: i32,
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
    fn unit_idle(&self, _ctx: &AddonContext<'_, G, E>, _unit: i32, _def: i32, _team: i32) {}
    fn projectile_created(
        &self,
        _ctx: &AddonContext<'_, G, E>,
        _projectile_id: i32,
        _owner_id: i32,
        _weapon_def_id: i32,
    ) {
    }
    fn unit_pre_damaged(
        &self,
        _ctx: &AddonContext<'_, G, E>,
        _event: &UnitPreDamagedEvent,
    ) -> Option<spring::DamageResult> {
        None
    }
    fn explosion(
        &self,
        _ctx: &AddonContext<'_, G, E>,
        _weapon_def_id: i32,
        _pos: (f32, f32, f32),
        _owner_id: i32,
        _projectile_id: i32,
    ) -> bool {
        false
    }
    fn projectile_destroyed(
        &self,
        _ctx: &AddonContext<'_, G, E>,
        _projectile_id: i32,
        _owner_id: i32,
        _weapon_def_id: i32,
    ) {
    }
    fn game_over(&self, _ctx: &AddonContext<'_, G, E>, _winning_ally_teams: &[u8]) {}
    fn unit_cmd_done(&self, _ctx: &AddonContext<'_, G, E>, _event: &CommandEvent<'_>) {}

    /// Return `false` to veto the command. The first gadget to veto wins and the
    /// remaining gadgets are not consulted, matching the Lua handler.
    fn allow_command(&self, _ctx: &AddonContext<'_, G, E>, _event: &CommandEvent<'_>) -> bool {
        true
    }
}

pub struct GadgetHandler<G, E: spring::capabilities::Common = spring::RulesSynced> {
    pub global: G,
    gadgets: Vec<Box<dyn Gadget<G, E>>>,
    enabled: Vec<Cell<bool>>,
    runtime: AddonRuntime<G, E>,
}

impl<G, E: spring::capabilities::Common> GadgetHandler<G, E> {
    pub fn new(global: G) -> Self {
        Self {
            global,
            gadgets: Vec::new(),
            enabled: Vec::new(),
            runtime: AddonRuntime::new(),
        }
    }

    pub fn add(&mut self, gadget: Box<dyn Gadget<G, E>>) {
        let is_enabled = gadget.is_enabled();
        self.gadgets.push(gadget);
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
            for (i, gadget) in self.gadgets.iter().enumerate() {
                if self.enabled[i].get() {
                    gadget.init(ctx);
                }
            }
        });
    }

    pub fn shutdown(&self, spring: &spring::Spring<'_, E>) {
        self.dispatch(spring, "Shutdown", |ctx| {
            for (i, addon) in self.gadgets.iter().enumerate() {
                if self.enabled[i].get() {
                    addon.shutdown(ctx);
                }
            }
        });
    }

    pub fn set_enabled(&self, spring: &spring::Spring<'_, E>, name: &str, enabled: bool) {
        self.dispatch(spring, "SetEnabled", |ctx| {
            for (i, gadget) in self.gadgets.iter().enumerate() {
                if gadget.name() == name {
                    if self.enabled[i].replace(enabled) != enabled {
                        if enabled {
                            gadget.init(ctx);
                        } else {
                            gadget.shutdown(ctx);
                        }
                    }
                    break;
                }
            }
        });
    }

    pub fn is_gadget_enabled(&self, name: &str) -> bool {
        self.gadgets
            .iter()
            .enumerate()
            .find_map(|(i, gadget)| (gadget.name() == name).then_some(self.enabled[i].get()))
            .unwrap_or(false)
    }

    pub fn game_frame(&self, spring: &spring::Spring<'_, E>, frame: i32) {
        self.dispatch(spring, "GameFrame", |ctx| {
            for (i, gadget) in self.gadgets.iter().enumerate() {
                if self.enabled[i].get() {
                    gadget.game_frame(ctx, frame);
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
            for (i, gadget) in self.gadgets.iter().enumerate() {
                if self.enabled[i].get() {
                    gadget.job_result(ctx, id, result);
                }
            }
        });
    }

    pub fn handle_lua_msg(
        &self,
        spring: &spring::Spring<'_, E>,
        player_id: i32,
        script: i32,
        mode: i32,
        data: &[u8],
    ) {
        self.dispatch(spring, "HandleLuaMsg", |ctx| {
            for (i, gadget) in self.gadgets.iter().enumerate() {
                if self.enabled[i].get() {
                    gadget.handle_lua_msg(ctx, player_id, script, mode, data);
                }
            }
        });
    }

    pub fn unit_created(
        &self,
        spring: &spring::Spring<'_, E>,
        unit: i32,
        def: i32,
        team: i32,
        builder: i32,
    ) {
        self.dispatch(spring, "UnitCreated", |ctx| {
            for (i, gadget) in self.gadgets.iter().enumerate() {
                if self.enabled[i].get() {
                    gadget.unit_created(ctx, unit, def, team, builder);
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
            for (i, gadget) in self.gadgets.iter().enumerate() {
                if self.enabled[i].get() {
                    gadget.feature_created(ctx, feature_id, ally_team_id);
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
            for (i, gadget) in self.gadgets.iter().enumerate() {
                if self.enabled[i].get() {
                    gadget.feature_destroyed(ctx, feature_id, ally_team_id);
                }
            }
        });
    }

    pub fn unit_destroyed(&self, spring: &spring::Spring<'_, E>, event: &UnitDestroyedEvent) {
        self.dispatch(spring, "UnitDestroyed", |ctx| {
            for (i, gadget) in self.gadgets.iter().enumerate() {
                if self.enabled[i].get() {
                    gadget.unit_destroyed(ctx, event);
                }
            }
        });
    }

    pub fn unit_idle(&self, spring: &spring::Spring<'_, E>, unit: i32, def: i32, team: i32) {
        self.dispatch(spring, "UnitIdle", |ctx| {
            for (i, gadget) in self.gadgets.iter().enumerate() {
                if self.enabled[i].get() {
                    gadget.unit_idle(ctx, unit, def, team);
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
            for (i, gadget) in self.gadgets.iter().enumerate() {
                if self.enabled[i].get() {
                    gadget.projectile_created(ctx, projectile_id, owner_id, weapon_def_id);
                }
            }
        });
    }

    #[allow(clippy::too_many_arguments)]
    pub fn unit_pre_damaged(
        &self,
        spring: &spring::Spring<'_, E>,
        unit_id: i32,
        unit_def_id: i32,
        unit_team: i32,
        mut damage: f32,
        paralyzer: bool,
        weapon_def_id: i32,
        projectile_id: i32,
        attacker_id: i32,
        attacker_def_id: i32,
        attacker_team: i32,
    ) -> spring::DamageResult {
        self.dispatch(spring, "UnitPreDamaged", |ctx| {
            let mut impulse_mult = 1.0;
            for (i, gadget) in self.gadgets.iter().enumerate() {
                if self.enabled[i].get() {
                    let event = UnitPreDamagedEvent {
                        unit_id,
                        unit_def_id,
                        unit_team,
                        damage,
                        paralyzer,
                        weapon_def_id,
                        projectile_id,
                        attacker_id,
                        attacker_def_id,
                        attacker_team,
                    };
                    if let Some(result) = gadget.unit_pre_damaged(ctx, &event) {
                        damage = result.new_damage;
                        impulse_mult = result.impulse_mult;
                    }
                }
            }
            spring::DamageResult {
                new_damage: damage,
                impulse_mult,
            }
        })
    }

    pub fn explosion(
        &self,
        spring: &spring::Spring<'_, E>,
        weapon_def_id: i32,
        pos: (f32, f32, f32),
        owner_id: i32,
        projectile_id: i32,
    ) -> bool {
        self.dispatch(spring, "Explosion", |ctx| {
            let mut handled = false;
            for (i, gadget) in self.gadgets.iter().enumerate() {
                if self.enabled[i].get()
                    && gadget.explosion(ctx, weapon_def_id, pos, owner_id, projectile_id)
                {
                    handled = true;
                }
            }
            handled
        })
    }

    pub fn projectile_destroyed(
        &self,
        spring: &spring::Spring<'_, E>,
        projectile_id: i32,
        owner_id: i32,
        weapon_def_id: i32,
    ) {
        self.dispatch(spring, "ProjectileDestroyed", |ctx| {
            for (i, gadget) in self.gadgets.iter().enumerate() {
                if self.enabled[i].get() {
                    gadget.projectile_destroyed(ctx, projectile_id, owner_id, weapon_def_id);
                }
            }
        });
    }

    pub fn game_over(&self, spring: &spring::Spring<'_, E>, winning_ally_teams: &[u8]) {
        self.dispatch(spring, "GameOver", |ctx| {
            for (i, gadget) in self.gadgets.iter().enumerate() {
                if self.enabled[i].get() {
                    gadget.game_over(ctx, winning_ally_teams);
                }
            }
        });
    }

    pub fn unit_cmd_done(&self, spring: &spring::Spring<'_, E>, event: &CommandEvent<'_>) {
        self.dispatch(spring, "UnitCmdDone", |ctx| {
            for (i, gadget) in self.gadgets.iter().enumerate() {
                if self.enabled[i].get() {
                    gadget.unit_cmd_done(ctx, event);
                }
            }
        });
    }

    /// Stops at the first veto: a gadget that denies a command shadows the rest.
    pub fn allow_command(&self, spring: &spring::Spring<'_, E>, event: &CommandEvent<'_>) -> bool {
        self.dispatch(spring, "AllowCommand", |ctx| {
            for (i, gadget) in self.gadgets.iter().enumerate() {
                if self.enabled[i].get() && !gadget.allow_command(ctx, event) {
                    return false;
                }
            }
            true
        })
    }
}
