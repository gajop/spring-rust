use alloc::boxed::Box;
use alloc::vec::Vec;

pub use crate::event::{CommandEvent, PendingRulesEvent, UnitDestroyedEvent, UnitPreDamagedEvent};

pub trait Gadget<G = ()> {
    fn name(&self) -> &'static str;

    fn is_enabled(&self) -> bool {
        true
    }

    fn init(&mut self, _global: &mut G) {}
    fn shutdown(&mut self, _global: &mut G) {}

    fn game_frame(&mut self, _global: &mut G, _frame: i32) {}
    fn handle_lua_msg(
        &mut self,
        _global: &mut G,
        _player_id: i32,
        _script: i32,
        _mode: i32,
        _data: &[u8],
    ) {
    }
    fn unit_created(&mut self, _global: &mut G, _unit: i32, _def: i32, _team: i32, _builder: i32) {}
    fn unit_destroyed(&mut self, _global: &mut G, _event: &UnitDestroyedEvent) {}
    fn unit_idle(&mut self, _global: &mut G, _unit: i32, _def: i32, _team: i32) {}
    fn projectile_created(
        &mut self,
        _global: &mut G,
        _projectile_id: i32,
        _owner_id: i32,
        _weapon_def_id: i32,
    ) {
    }
    fn unit_pre_damaged(
        &mut self,
        _global: &mut G,
        _event: &UnitPreDamagedEvent,
    ) -> Option<spring::DamageResult> {
        None
    }
    fn explosion(
        &mut self,
        _global: &mut G,
        _weapon_def_id: i32,
        _pos: (f32, f32, f32),
        _owner_id: i32,
        _projectile_id: i32,
    ) -> bool {
        false
    }

    fn projectile_destroyed(
        &mut self,
        _global: &mut G,
        _projectile_id: i32,
        _owner_id: i32,
        _weapon_def_id: i32,
    ) {
    }

    fn game_over(&mut self, _global: &mut G, _winning_ally_teams: &[u8]) {}

    fn unit_cmd_done(&mut self, _global: &mut G, _event: &CommandEvent<'_>) {}

    /// Return `false` to veto the command. The first gadget to veto wins and the
    /// remaining gadgets are not consulted, matching the Lua handler.
    fn allow_command(&mut self, _global: &mut G, _event: &CommandEvent<'_>) -> bool {
        true
    }
}

pub struct GadgetHandler<G> {
    pub global: G,
    gadgets: Vec<Box<dyn Gadget<G>>>,
    enabled: Vec<bool>,
}

impl<G> GadgetHandler<G> {
    pub fn new(global: G) -> Self {
        Self {
            global,
            gadgets: Vec::new(),
            enabled: Vec::new(),
        }
    }

    pub fn add(&mut self, gadget: Box<dyn Gadget<G>>) {
        let is_enabled = gadget.is_enabled();
        self.gadgets.push(gadget);
        self.enabled.push(is_enabled);
    }

    pub fn init(&mut self) {
        for (i, gadget) in self.gadgets.iter_mut().enumerate() {
            if self.enabled[i] {
                gadget.init(&mut self.global);
            }
        }
    }

    pub fn set_enabled(&mut self, name: &str, enabled: bool) {
        for (i, gadget) in self.gadgets.iter_mut().enumerate() {
            if gadget.name() == name {
                if self.enabled[i] != enabled {
                    self.enabled[i] = enabled;
                    if enabled {
                        gadget.init(&mut self.global);
                    } else {
                        gadget.shutdown(&mut self.global);
                    }
                }
                break;
            }
        }
    }

    pub fn is_gadget_enabled(&self, name: &str) -> bool {
        for (i, gadget) in self.gadgets.iter().enumerate() {
            if gadget.name() == name {
                return self.enabled[i];
            }
        }
        false
    }

    pub fn game_frame(&mut self, frame: i32) {
        for (i, gadget) in self.gadgets.iter_mut().enumerate() {
            if self.enabled[i] {
                gadget.game_frame(&mut self.global, frame);
            }
        }
    }

    pub fn handle_lua_msg(&mut self, player_id: i32, script: i32, mode: i32, data: &[u8]) {
        for (i, gadget) in self.gadgets.iter_mut().enumerate() {
            if self.enabled[i] {
                gadget.handle_lua_msg(&mut self.global, player_id, script, mode, data);
            }
        }
    }

    pub fn unit_created(&mut self, unit: i32, def: i32, team: i32, builder: i32) {
        for (i, gadget) in self.gadgets.iter_mut().enumerate() {
            if self.enabled[i] {
                gadget.unit_created(&mut self.global, unit, def, team, builder);
            }
        }
    }

    pub fn unit_destroyed(&mut self, event: &UnitDestroyedEvent) {
        for (i, gadget) in self.gadgets.iter_mut().enumerate() {
            if self.enabled[i] {
                gadget.unit_destroyed(&mut self.global, event);
            }
        }
    }

    pub fn unit_idle(&mut self, unit: i32, def: i32, team: i32) {
        for (i, gadget) in self.gadgets.iter_mut().enumerate() {
            if self.enabled[i] {
                gadget.unit_idle(&mut self.global, unit, def, team);
            }
        }
    }

    pub fn projectile_created(&mut self, projectile_id: i32, owner_id: i32, weapon_def_id: i32) {
        for (i, gadget) in self.gadgets.iter_mut().enumerate() {
            if self.enabled[i] {
                gadget.projectile_created(&mut self.global, projectile_id, owner_id, weapon_def_id);
            }
        }
    }

    pub fn unit_pre_damaged(
        &mut self,
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
        let mut impulse_mult = 1.0;
        for (i, gadget) in self.gadgets.iter_mut().enumerate() {
            if self.enabled[i] {
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
                if let Some(res) = gadget.unit_pre_damaged(&mut self.global, &event) {
                    damage = res.new_damage;
                    impulse_mult = res.impulse_mult;
                }
            }
        }
        spring::DamageResult {
            new_damage: damage,
            impulse_mult,
        }
    }

    pub fn explosion(
        &mut self,
        weapon_def_id: i32,
        pos: (f32, f32, f32),
        owner_id: i32,
        projectile_id: i32,
    ) -> bool {
        let mut handled = false;
        for (i, gadget) in self.gadgets.iter_mut().enumerate() {
            if self.enabled[i]
                && gadget.explosion(
                    &mut self.global,
                    weapon_def_id,
                    pos,
                    owner_id,
                    projectile_id,
                )
            {
                handled = true;
            }
        }
        handled
    }

    pub fn projectile_destroyed(&mut self, projectile_id: i32, owner_id: i32, weapon_def_id: i32) {
        for (i, gadget) in self.gadgets.iter_mut().enumerate() {
            if self.enabled[i] {
                gadget.projectile_destroyed(
                    &mut self.global,
                    projectile_id,
                    owner_id,
                    weapon_def_id,
                );
            }
        }
    }

    pub fn game_over(&mut self, winning_ally_teams: &[u8]) {
        for (i, gadget) in self.gadgets.iter_mut().enumerate() {
            if self.enabled[i] {
                gadget.game_over(&mut self.global, winning_ally_teams);
            }
        }
    }

    pub fn unit_cmd_done(&mut self, event: &CommandEvent<'_>) {
        for (i, gadget) in self.gadgets.iter_mut().enumerate() {
            if self.enabled[i] {
                gadget.unit_cmd_done(&mut self.global, event);
            }
        }
    }

    /// Stops at the first veto: a gadget that denies a command shadows the rest,
    /// so later gadgets must not observe a command that was already refused.
    pub fn allow_command(&mut self, event: &CommandEvent<'_>) -> bool {
        for (i, gadget) in self.gadgets.iter_mut().enumerate() {
            if self.enabled[i] && !gadget.allow_command(&mut self.global, event) {
                return false;
            }
        }
        true
    }

    pub fn dispatch_pending_event(&mut self, event: PendingRulesEvent) {
        match event {
            PendingRulesEvent::UnitCreated {
                unit,
                def,
                team,
                builder,
            } => {
                self.unit_created(unit, def, team, builder);
            }
            PendingRulesEvent::UnitDestroyed(event) => {
                self.unit_destroyed(&event);
            }
            PendingRulesEvent::ProjectileCreated {
                projectile_id,
                owner_id,
                weapon_def_id,
            } => {
                self.projectile_created(projectile_id, owner_id, weapon_def_id);
            }
            PendingRulesEvent::ProjectileDestroyed {
                projectile_id,
                owner_id,
                weapon_def_id,
            } => {
                self.projectile_destroyed(projectile_id, owner_id, weapon_def_id);
            }
            PendingRulesEvent::GameOver { winning_ally_teams } => {
                self.game_over(&winning_ally_teams);
            }
            PendingRulesEvent::Explosion {
                weapon_def_id,
                pos,
                owner_id,
                projectile_id,
            } => {
                self.explosion(weapon_def_id, pos, owner_id, projectile_id);
            }
            PendingRulesEvent::LuaMsg {
                player_id,
                script,
                mode,
                data,
            } => {
                self.handle_lua_msg(player_id, script, mode, &data);
            }
        }
    }
}

/// Report, once, that a re-entrant `UnitPreDamaged` could not reach the gadget
/// chain and the incoming damage was passed through unchanged.
///
/// A game whose gadgets modify damage *and* which triggers damage from inside
/// another call-in must pass `reentrant_unit_pre_damaged:` to
/// `export_rules_gadgets!`; otherwise its damage rules are silently skipped.
pub fn warn_reentrant_damage_dropped() {
    use core::sync::atomic::{AtomicBool, Ordering};
    static REPORTED: AtomicBool = AtomicBool::new(false);
    if REPORTED.swap(true, Ordering::Relaxed) {
        return;
    }
    let _ = spring::log(
        "spring-addons",
        40,
        "re-entrant UnitPreDamaged could not reach the gadgets; damage passed \
         through unchanged. Set `reentrant_unit_pre_damaged:` on \
         export_rules_gadgets! if this game modifies damage.",
    );
}
