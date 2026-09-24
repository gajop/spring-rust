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

/// Engine key codes (SDL2 keycodes) for keys without a character.
pub mod keys {
    const fn scancode(code: i32) -> i32 {
        code | 0x4000_0000
    }

    pub const BACKSPACE: i32 = 8;
    pub const TAB: i32 = 9;
    pub const ENTER: i32 = 13;
    pub const ESCAPE: i32 = 27;
    pub const DELETE: i32 = 127;
    pub const RIGHT: i32 = scancode(79);
    pub const LEFT: i32 = scancode(80);
    pub const DOWN: i32 = scancode(81);
    pub const UP: i32 = scancode(82);
    pub const HOME: i32 = scancode(74);
    pub const PAGE_UP: i32 = scancode(75);
    pub const END: i32 = scancode(77);
    pub const PAGE_DOWN: i32 = scancode(78);

    /// `F1` .. `F12` as `f(1)` .. `f(12)`; other numbers give -1.
    pub const fn f(number: i32) -> i32 {
        if number >= 1 && number <= 12 {
            scancode(57 + number)
        } else {
            -1
        }
    }
}

impl<'a> KeyEvent<'a> {
    /// Whether this is `symbol`: a character (`"a"`, `"1"`), or one of `space`,
    /// `enter`, `escape`, `tab`, `backspace`, `delete`, `up`, `down`, `left`,
    /// `right`, `home`, `end`, `pageup`, `pagedown`, `f1` .. `f12`.
    pub fn matches(&self, symbol: &str) -> bool {
        let named = match symbol {
            "tab" => Some(keys::TAB),
            "backspace" => Some(keys::BACKSPACE),
            "delete" => Some(keys::DELETE),
            "up" => Some(keys::UP),
            "down" => Some(keys::DOWN),
            "left" => Some(keys::LEFT),
            "right" => Some(keys::RIGHT),
            "home" => Some(keys::HOME),
            "end" => Some(keys::END),
            "pageup" => Some(keys::PAGE_UP),
            "pagedown" => Some(keys::PAGE_DOWN),
            value if value.len() > 1 && value.starts_with('f') => {
                value[1..].parse().ok().map(keys::f).filter(|code| *code >= 0)
            }
            _ => None,
        };
        if let Some(code) = named {
            return self.key_code == code;
        }
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

#[cfg(test)]
mod tests {
    use super::{KeyEvent, keys};

    fn key(key_code: i32, label: &[u8]) -> KeyEvent<'_> {
        KeyEvent {
            key_code,
            alt: false,
            ctrl: false,
            meta: false,
            shift: false,
            is_repeat: false,
            label,
            utf32_char: 0,
            scan_code: 0,
        }
    }

    #[test]
    fn named_keys_match_their_codes() {
        assert!(key(keys::UP, b"").matches("up"));
        assert!(!key(keys::UP, b"").matches("down"));
        assert!(key(0x4000_003A, b"").matches("f1"));
        assert!(key(0x4000_0045, b"").matches("f12"));
        assert!(!key(0x4000_0045, b"").matches("f13"));
        assert!(key(i32::from(b'f'), b"f").matches("f"));
        assert!(key(i32::from(b'a'), b"a").matches("a"));
        assert!(key(27, b"").matches("escape"));
    }
}
