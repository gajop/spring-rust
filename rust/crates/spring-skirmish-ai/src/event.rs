use crate::{Error, Result, TeamId, UnitId, sys};
use std::ffi::{CStr, c_void};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Visibility {
    Los,
    Radar,
}

/// Borrowed engine event. Strings and slices expire when `handle` returns.
#[derive(Debug)]
#[non_exhaustive]
pub enum Event<'a> {
    Null,
    Init {
        saved_game: bool,
    },
    Release {
        reason: i32,
    },
    Update {
        frame: i32,
    },
    Message {
        player: i32,
        message: &'a CStr,
    },
    LuaMessage(&'a CStr),
    UnitCreated {
        unit: UnitId,
        builder: Option<UnitId>,
    },
    UnitFinished(UnitId),
    UnitIdle(UnitId),
    UnitMoveFailed(UnitId),
    Damaged {
        unit: UnitId,
        enemy: bool,
        attacker: Option<UnitId>,
        damage: f32,
        direction: [f32; 3],
        weapon: i32,
        paralyzer: bool,
    },
    Destroyed {
        unit: UnitId,
        enemy: bool,
        attacker: Option<UnitId>,
        weapon: Option<i32>,
    },
    Transferred {
        unit: UnitId,
        old_team: TeamId,
        new_team: TeamId,
        captured: bool,
    },
    EnemyEnter {
        unit: UnitId,
        visibility: Visibility,
    },
    EnemyLeave {
        unit: UnitId,
        visibility: Visibility,
    },
    EnemyCreated(UnitId),
    EnemyFinished(UnitId),
    WeaponFired {
        unit: UnitId,
        weapon: i32,
    },
    PlayerCommand {
        units: &'a [i32],
        topic: i32,
        player: i32,
    },
    CommandFinished {
        unit: UnitId,
        command: i32,
        topic: i32,
    },
    SeismicPing {
        position: [f32; 3],
        strength: f32,
    },
    Load(&'a CStr),
    Save(&'a CStr),
    Unknown {
        topic: i32,
    },
}
fn optional_unit(id: i32) -> Option<UnitId> {
    (id >= 0).then_some(UnitId(id))
}
unsafe fn payload<'a, T>(data: *const c_void) -> Result<&'a T> {
    if data.is_null() || (data as usize) % std::mem::align_of::<T>() != 0 {
        return Err(Error::InvalidInput("null or misaligned event"));
    }
    // SAFETY: caller supplies the matching, live engine event structure.
    Ok(unsafe { &*data.cast::<T>() })
}
unsafe fn string<'a>(p: *const std::ffi::c_char) -> Result<&'a CStr> {
    if p.is_null() {
        return Err(Error::InvalidInput("null event string"));
    }
    Ok(unsafe { CStr::from_ptr(p) })
}
unsafe fn slice<'a, T>(p: *const T, len: i32) -> Result<&'a [T]> {
    if !(0..=1_048_576).contains(&len) {
        return Err(Error::InvalidInput("invalid event slice length"));
    }
    if len == 0 {
        return Ok(&[]);
    }
    if p.is_null() || (p as usize) % std::mem::align_of::<T>() != 0 {
        return Err(Error::InvalidInput("null or misaligned event slice"));
    }
    Ok(unsafe { std::slice::from_raw_parts(p, len as usize) })
}
impl<'a> Event<'a> {
    /// # Safety
    /// `data` and its nested pointers must match `topic` and remain live for `'a`.
    pub unsafe fn decode(topic: i32, data: *const c_void) -> Result<Self> {
        macro_rules! read {
            ($ty:ident) => {
                unsafe { payload::<sys::$ty>(data)? }
            };
        }
        macro_rules! damage {
            ($ty:ident,$field:ident,$enemy:expr) => {{
                let e = read!($ty);
                Self::Damaged {
                    unit: UnitId(e.$field),
                    enemy: $enemy,
                    attacker: optional_unit(e.attacker),
                    damage: e.damage,
                    direction: unsafe { slice(e.dir_posF3, 3)? }.try_into().unwrap(),
                    weapon: e.weaponDefId,
                    paralyzer: e.paralyzer,
                }
            }};
        }
        Ok(match topic as u32 {
            sys::EVENT_NULL => Self::Null,
            sys::EVENT_INIT => Self::Init {
                saved_game: read!(SInitEvent).savedGame,
            },
            sys::EVENT_RELEASE => Self::Release {
                reason: read!(SReleaseEvent).reason,
            },
            sys::EVENT_UPDATE => Self::Update {
                frame: read!(SUpdateEvent).frame,
            },
            sys::EVENT_MESSAGE => {
                let e = read!(SMessageEvent);
                Self::Message {
                    player: e.player,
                    message: unsafe { string(e.message)? },
                }
            }
            sys::EVENT_LUA_MESSAGE => {
                let e = read!(SLuaMessageEvent);
                Self::LuaMessage(unsafe { string(e.inData)? })
            }
            sys::EVENT_UNIT_CREATED => {
                let e = read!(SUnitCreatedEvent);
                Self::UnitCreated {
                    unit: UnitId(e.unit),
                    builder: optional_unit(e.builder),
                }
            }
            sys::EVENT_UNIT_FINISHED => Self::UnitFinished(UnitId(read!(SUnitFinishedEvent).unit)),
            sys::EVENT_UNIT_IDLE => Self::UnitIdle(UnitId(read!(SUnitIdleEvent).unit)),
            sys::EVENT_UNIT_MOVE_FAILED => {
                Self::UnitMoveFailed(UnitId(read!(SUnitMoveFailedEvent).unit))
            }
            sys::EVENT_UNIT_DAMAGED => damage!(SUnitDamagedEvent, unit, false),
            sys::EVENT_ENEMY_DAMAGED => damage!(SEnemyDamagedEvent, enemy, true),
            sys::EVENT_UNIT_DESTROYED => {
                let e = read!(SUnitDestroyedEvent);
                Self::Destroyed {
                    unit: UnitId(e.unit),
                    enemy: false,
                    attacker: optional_unit(e.attacker),
                    weapon: Some(e.weaponDefID),
                }
            }
            sys::EVENT_ENEMY_DESTROYED => {
                let e = read!(SEnemyDestroyedEvent);
                Self::Destroyed {
                    unit: UnitId(e.enemy),
                    enemy: true,
                    attacker: optional_unit(e.attacker),
                    weapon: None,
                }
            }
            sys::EVENT_UNIT_GIVEN => {
                let e = read!(SUnitGivenEvent);
                Self::Transferred {
                    unit: UnitId(e.unitId),
                    old_team: TeamId(e.oldTeamId),
                    new_team: TeamId(e.newTeamId),
                    captured: false,
                }
            }
            sys::EVENT_UNIT_CAPTURED => {
                let e = read!(SUnitCapturedEvent);
                Self::Transferred {
                    unit: UnitId(e.unitId),
                    old_team: TeamId(e.oldTeamId),
                    new_team: TeamId(e.newTeamId),
                    captured: true,
                }
            }
            sys::EVENT_ENEMY_ENTER_LOS => Self::EnemyEnter {
                unit: UnitId(read!(SEnemyEnterLOSEvent).enemy),
                visibility: Visibility::Los,
            },
            sys::EVENT_ENEMY_LEAVE_LOS => Self::EnemyLeave {
                unit: UnitId(read!(SEnemyLeaveLOSEvent).enemy),
                visibility: Visibility::Los,
            },
            sys::EVENT_ENEMY_ENTER_RADAR => Self::EnemyEnter {
                unit: UnitId(read!(SEnemyEnterRadarEvent).enemy),
                visibility: Visibility::Radar,
            },
            sys::EVENT_ENEMY_LEAVE_RADAR => Self::EnemyLeave {
                unit: UnitId(read!(SEnemyLeaveRadarEvent).enemy),
                visibility: Visibility::Radar,
            },
            sys::EVENT_ENEMY_CREATED => Self::EnemyCreated(UnitId(read!(SEnemyCreatedEvent).enemy)),
            sys::EVENT_ENEMY_FINISHED => {
                Self::EnemyFinished(UnitId(read!(SEnemyFinishedEvent).enemy))
            }
            sys::EVENT_WEAPON_FIRED => {
                let e = read!(SWeaponFiredEvent);
                Self::WeaponFired {
                    unit: UnitId(e.unitId),
                    weapon: e.weaponDefId,
                }
            }
            sys::EVENT_PLAYER_COMMAND => {
                let e = read!(SPlayerCommandEvent);
                Self::PlayerCommand {
                    units: unsafe { slice(e.unitIds, e.unitIds_size)? },
                    topic: e.commandTopicId,
                    player: e.playerId,
                }
            }
            sys::EVENT_COMMAND_FINISHED => {
                let e = read!(SCommandFinishedEvent);
                Self::CommandFinished {
                    unit: UnitId(e.unitId),
                    command: e.commandId,
                    topic: e.commandTopicId,
                }
            }
            sys::EVENT_SEISMIC_PING => {
                let e = read!(SSeismicPingEvent);
                Self::SeismicPing {
                    position: unsafe { slice(e.pos_posF3, 3)? }.try_into().unwrap(),
                    strength: e.strength,
                }
            }
            sys::EVENT_LOAD => {
                let e = read!(SLoadEvent);
                Self::Load(unsafe { string(e.file)? })
            }
            sys::EVENT_SAVE => {
                let e = read!(SSaveEvent);
                Self::Save(unsafe { string(e.file)? })
            }
            _ => Self::Unknown { topic },
        })
    }
}
