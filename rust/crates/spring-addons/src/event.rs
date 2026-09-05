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
        if handled { Self::Handled } else { Self::Ignored }
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ViewGeometry {
    pub screen_size: (i32, i32),
    pub screen_pos: (i32, i32),
    pub window_size: (i32, i32),
    pub window_pos: (i32, i32),
    pub window_border: (i32, i32, i32, i32),
    pub view_size: (i32, i32),
    pub view_pos: (i32, i32),
}

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
    pub player_num: Option<i32>,
    pub from_synced: bool,
    pub from_lua: bool,
}
