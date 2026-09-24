/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! Unit defs by name: [`unit_kinds!`](crate::unit_kinds) and custom params.

use core::str::FromStr;

/// A unit def custom param parsed as `T`; `None` when it is missing, empty or
/// doesn't parse.
pub fn unit_def_custom_param<T: FromStr>(def: i32, key: &str) -> Option<T> {
    let value = crate::owned::unit_defs::get_unit_def_custom_param(def, key).ok()?;
    let value = value.trim();
    if value.is_empty() {
        return None;
    }
    value.parse().ok()
}

#[doc(hidden)]
pub fn resolve(slot: &core::sync::atomic::AtomicI32, name: &str) -> Option<i32> {
    use core::sync::atomic::Ordering;
    // UNRESOLVED until the first lookup; -1 remembers "no such def".
    const UNRESOLVED: i32 = i32::MIN;
    let mut id = slot.load(Ordering::Relaxed);
    if id == UNRESOLVED {
        id = crate::owned::unit_defs::get_unit_def_id_by_name(name).unwrap_or(-1);
        slot.store(id, Ordering::Relaxed);
    }
    (id >= 0).then_some(id)
}

/// Declare the unit defs a game refers to by name, resolved to def ids once:
///
/// ```rust,ignore
/// spring::unit_kinds! {
///     pub enum Kind {
///         Rabbit = "rabbit",
///         Carrot = "carrot",
///     }
/// }
///
/// let rabbit: Option<i32> = Kind::Rabbit.def();
/// let kind: Option<Kind> = Kind::from_def(def_id);
/// ```
#[macro_export]
macro_rules! unit_kinds {
    ($(#[$meta:meta])* $vis:vis enum $name:ident { $($kind:ident = $def:literal),+ $(,)? }) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        $vis enum $name { $($kind),+ }

        impl $name {
            pub const ALL: &'static [$name] = &[$($name::$kind),+];

            /// The unit def name.
            pub const fn name(self) -> &'static str {
                match self { $($name::$kind => $def),+ }
            }

            /// The def id, or `None` if the game has no def of this name.
            pub fn def(self) -> Option<i32> {
                static IDS: [core::sync::atomic::AtomicI32; [$($def),+].len()] =
                    [const { core::sync::atomic::AtomicI32::new(i32::MIN) }; [$($def),+].len()];
                $crate::kinds::resolve(&IDS[self as usize], self.name())
            }

            /// The kind of def id `def`, if it is one of these.
            pub fn from_def(def: i32) -> Option<Self> {
                Self::ALL.iter().copied().find(|kind| kind.def() == Some(def))
            }
        }
    };
}
