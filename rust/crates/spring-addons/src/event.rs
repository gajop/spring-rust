#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum EventResult {
    #[default]
    Ignored,
    Handled,
}

impl EventResult {
    #[inline]
    pub fn is_handled(self) -> bool {
        matches!(self, Self::Handled)
    }

    #[inline]
    pub fn is_ignored(self) -> bool {
        matches!(self, Self::Ignored)
    }
}

impl From<bool> for EventResult {
    #[inline]
    fn from(handled: bool) -> Self {
        if handled {
            Self::Handled
        } else {
            Self::Ignored
        }
    }
}

pub struct KeyEvent<'a> {
    pub key_code: i32,
    pub alt: bool,
    pub ctrl: bool,
    pub meta: bool,
    pub shift: bool,
    pub is_repeat: bool,
    pub label: &'a [u8],
    pub utf32_char: i32,
    pub scan_code: i32,
}

impl<'a> KeyEvent<'a> {
    pub fn matches(&self, symbol: &str) -> bool {
        let ascii = match symbol {
            "space" => Some(b' '),
            "enter" => return self.key_code == 13 || self.key_code == 271,
            "escape" => return self.key_code == 27,
            value if value.len() == 1 => value.as_bytes().first().copied(),
            _ => None,
        };
        if ascii.is_some_and(|value| {
            self.key_code == i32::from(value)
                || self.key_code == i32::from(value.to_ascii_uppercase())
        }) {
            return true;
        }
        self.label.len() == 1
            && ascii.is_some_and(|value| self.label[0].eq_ignore_ascii_case(&value))
    }
}

pub struct UnitPreDamagedEvent {
    pub unit_id: i32,
    pub unit_def_id: i32,
    pub unit_team: i32,
    pub damage: f32,
    pub paralyzer: bool,
    pub weapon_def_id: i32,
    pub projectile_id: i32,
    pub attacker_id: i32,
    pub attacker_def_id: i32,
    pub attacker_team: i32,
}

#[derive(Clone, Copy, Debug)]
pub struct UnitDestroyedEvent {
    pub unit_id: i32,
    pub unit_def_id: i32,
    pub unit_team: i32,
    pub attacker_id: i32,
    pub attacker_def_id: i32,
    pub attacker_team: i32,
    pub weapon_def_id: i32,
}

#[derive(Clone, Debug)]
pub enum PendingRulesEvent {
    UnitCreated {
        unit: i32,
        def: i32,
        team: i32,
        builder: i32,
    },
    UnitDestroyed(UnitDestroyedEvent),
    ProjectileCreated {
        projectile_id: i32,
        owner_id: i32,
        weapon_def_id: i32,
    },
    ProjectileDestroyed {
        projectile_id: i32,
        owner_id: i32,
        weapon_def_id: i32,
    },
    GameOver {
        winning_ally_teams: alloc::vec::Vec<u8>,
    },
    Explosion {
        weapon_def_id: i32,
        pos: (f32, f32, f32),
        owner_id: i32,
        projectile_id: i32,
    },
    LuaMsg {
        player_id: i32,
        script: i32,
        mode: i32,
        data: alloc::vec::Vec<u8>,
    },
}

/// Screen/window/view geometry delivered by `ViewResize`.
///
/// Bundled into a struct because the raw callin carries sixteen positional
/// `i32`s, which is unreadable at every call site.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ViewGeometry {
    pub screen_size: (i32, i32),
    pub screen_pos: (i32, i32),
    pub window_size: (i32, i32),
    pub window_pos: (i32, i32),
    /// Border insets, in `(top, left, bottom, right)` order.
    pub window_border: (i32, i32, i32, i32),
    pub view_size: (i32, i32),
    pub view_pos: (i32, i32),
}

/// A unit command, as delivered by `AllowCommand` and `UnitCmdDone`.
pub struct CommandEvent<'a> {
    pub unit_id: i32,
    pub unit_def_id: i32,
    pub unit_team: i32,
    pub command_id: i32,
    pub command_time_out: i32,
    pub command_page_index: u32,
    pub command_tag: u32,
    pub command_options: u8,
    pub command_params: &'a [f32],
    /// Player that issued the command. `None` for `UnitCmdDone`, which does not
    /// report an issuer.
    pub player_num: Option<i32>,
    pub from_synced: bool,
    pub from_lua: bool,
}
