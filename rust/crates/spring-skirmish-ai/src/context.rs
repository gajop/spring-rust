use crate::{AiId, Error, FeatureDefId, FeatureId, Result, TeamId, UnitDefId, UnitId, sys};
use std::{
    ffi::{CStr, CString, c_void},
    marker::PhantomData,
    rc::Rc,
};

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UnitSnapshot {
    pub id: UnitId,
    pub definition: UnitDefId,
    pub team: TeamId,
    pub position: [f32; 3],
    pub velocity: [f32; 3],
    pub health: f32,
    pub max_health: f32,
    pub build_progress: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FeatureSnapshot {
    pub id: FeatureId,
    pub definition: FeatureDefId,
    pub name: String,
    pub position: [f32; 3],
}
impl Default for UnitId {
    fn default() -> Self {
        Self(-1)
    }
}
impl Default for UnitDefId {
    fn default() -> Self {
        Self(-1)
    }
}
impl Default for TeamId {
    fn default() -> Self {
        Self(-1)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct OrderOptions {
    pub flags: i16,
    pub timeout_frame: Option<i32>,
}

/// Engine access during a single dispatch. Not Send/Sync: callbacks must not
/// be queried from planner threads. Snapshot data can be sent to worker threads.
pub struct Context<'a> {
    id: AiId,
    callbacks: &'a sys::SSkirmishAICallback,
    _thread: PhantomData<Rc<()>>,
}
macro_rules! callback {
    ($self:ident,$name:ident $(,$arg:expr)*) => {{
        let call=$self.callbacks.$name.ok_or(Error::MissingCallback(stringify!($name)))?;
        // SAFETY: table lifetime is tied to this engine dispatch; typed arguments match ABI.
        unsafe { call($self.id.0 $(,$arg)*) }
    }};
}
impl<'a> Context<'a> {
    pub(crate) fn new(id: AiId, callbacks: &'a sys::SSkirmishAICallback) -> Self {
        Self {
            id,
            callbacks,
            _thread: PhantomData,
        }
    }
    /// Library metadata from AIInfo.lua (for example `version`). Copied from the engine.
    pub fn info(&self, key: &str) -> Result<Option<String>> {
        let key = CString::new(key).map_err(|_| Error::InvalidInput("info key contains NUL"))?;
        let pointer = callback!(self, SkirmishAI_Info_getValueByKey, key.as_ptr());
        if pointer.is_null() {
            return Ok(None);
        }
        Ok(Some(
            unsafe { CStr::from_ptr(pointer) }
                .to_string_lossy()
                .into_owned(),
        ))
    }
    /// Start-script AI option; availability depends on the game/lobby's AIOptions metadata.
    pub fn option(&self, key: &str) -> Result<Option<String>> {
        let key = CString::new(key).map_err(|_| Error::InvalidInput("option key contains NUL"))?;
        let pointer = callback!(self, SkirmishAI_OptionValues_getValueByKey, key.as_ptr());
        if pointer.is_null() {
            return Ok(None);
        }
        Ok(Some(
            unsafe { CStr::from_ptr(pointer) }
                .to_string_lossy()
                .into_owned(),
        ))
    }
    /// Read-only diagnostic; safe APIs never enable cheat access.
    pub fn cheats_enabled(&self) -> Result<bool> {
        Ok(callback!(self, Cheats_isEnabled))
    }
    pub fn ai_id(&self) -> AiId {
        self.id
    }
    pub fn team(&self) -> Result<TeamId> {
        Ok(TeamId(callback!(self, SkirmishAI_getTeamId)))
    }
    pub fn frame(&self) -> Result<i32> {
        Ok(callback!(self, Game_getCurrentFrame))
    }
    pub fn log(&self, message: &str) -> Result<()> {
        let text = CString::new(message).map_err(|_| Error::InvalidInput("log contains NUL"))?;
        callback!(self, Log_log, text.as_ptr());
        Ok(())
    }
    /// Escape hatch for APIs not yet wrapped. Invoking its function pointers is unsafe.
    /// Cheat callbacks are deliberately absent from the safe façade.
    pub fn raw(&self) -> &sys::SSkirmishAICallback {
        self.callbacks
    }
    fn list(&self, enemies: bool) -> Result<Vec<UnitId>> {
        let capacity = callback!(self, Unit_getMax);
        if !(1..=1_048_576).contains(&capacity) {
            return Err(Error::InvalidInput("invalid engine unit capacity"));
        }
        let mut ids = vec![0; capacity as usize];
        let size = if enemies {
            callback!(self, getEnemyUnits, ids.as_mut_ptr(), capacity)
        } else {
            callback!(self, getFriendlyUnits, ids.as_mut_ptr(), capacity)
        };
        if !(0..=capacity).contains(&size) {
            return Err(Error::InvalidInput("unit list overflow"));
        }
        ids.truncate(size as usize);
        Ok(ids.into_iter().map(UnitId).collect())
    }
    pub fn owned_units(&self) -> Result<Vec<UnitId>> {
        let team = self.team()?;
        let mut result = Vec::new();
        for id in self.list(false)? {
            if callback!(self, Unit_getTeam, id.0) == team.0 {
                result.push(id);
            }
        }
        Ok(result)
    }
    /// Direct LOS list from the normal callback, excluding radar-only contacts.
    pub fn visible_enemies(&self) -> Result<Vec<UnitId>> {
        self.list(true)
    }
    /// Query a unit exposed by normal callbacks. Unknown/hidden unit definitions return None.
    /// Exact enemy observations should only be taken from `visible_enemies`.
    pub fn unit(&self, id: UnitId) -> Result<Option<UnitSnapshot>> {
        if id.0 < 0 {
            return Err(Error::InvalidInput("invalid unit"));
        }
        let definition = callback!(self, Unit_getDef, id.0);
        if definition < 0 {
            return Ok(None);
        }
        let mut position = [0.; 3];
        let mut velocity = [0.; 3];
        callback!(self, Unit_getPos, id.0, position.as_mut_ptr());
        callback!(self, Unit_getVel, id.0, velocity.as_mut_ptr());
        Ok(Some(UnitSnapshot {
            id,
            definition: UnitDefId(definition),
            team: TeamId(callback!(self, Unit_getTeam, id.0)),
            position,
            velocity,
            health: callback!(self, Unit_getHealth, id.0),
            max_health: callback!(self, Unit_getMaxHealth, id.0),
            build_progress: callback!(self, Unit_getBuildProgress, id.0),
        }))
    }
    pub fn definition_name(&self, id: UnitDefId) -> Result<String> {
        if id.0 <= 0 {
            return Err(Error::InvalidInput("invalid unit definition"));
        }
        let pointer = callback!(self, UnitDef_getName, id.0);
        if pointer.is_null() {
            return Err(Error::InvalidInput("null definition name"));
        }
        // SAFETY: engine returns a terminated string; copy before the next host call.
        Ok(unsafe { CStr::from_ptr(pointer) }
            .to_string_lossy()
            .into_owned())
    }
    pub fn unit_rule(&self, id: UnitId, key: &str, default: f32) -> Result<f32> {
        let key = CString::new(key).map_err(|_| Error::InvalidInput("rule key contains NUL"))?;
        Ok(callback!(
            self,
            Unit_getRulesParamFloat,
            id.0,
            key.as_ptr(),
            default
        ))
    }
    pub fn own_team_rule(&self, key: &str, default: f32) -> Result<f32> {
        let key = CString::new(key).map_err(|_| Error::InvalidInput("rule key contains NUL"))?;
        Ok(callback!(
            self,
            Team_getRulesParamFloat,
            self.team()?.0,
            key.as_ptr(),
            default
        ))
    }
    pub fn game_rule(&self, key: &str, default: f32) -> Result<f32> {
        let key = CString::new(key).map_err(|_| Error::InvalidInput("rule key contains NUL"))?;
        Ok(callback!(
            self,
            Game_getRulesParamFloat,
            key.as_ptr(),
            default
        ))
    }
    /// Feature snapshots exposed by normal LOS callbacks. Never enables cheats.
    pub fn visible_features(&self) -> Result<Vec<FeatureSnapshot>> {
        let count = callback!(self, getFeatures, std::ptr::null_mut(), i32::MAX);
        if !(0..=1_048_576).contains(&count) {
            return Err(Error::InvalidInput("invalid feature count"));
        }
        if count == 0 {
            return Ok(Vec::new());
        }
        let mut ids = vec![0; count as usize];
        let size = callback!(self, getFeatures, ids.as_mut_ptr(), count);
        if !(0..=count).contains(&size) {
            return Err(Error::InvalidInput("feature list overflow"));
        }
        let mut result = Vec::with_capacity(size as usize);
        for id in ids.into_iter().take(size as usize) {
            let definition = callback!(self, Feature_getDef, id);
            if definition < 0 {
                continue;
            }
            let mut position = [0.; 3];
            callback!(self, Feature_getPosition, id, position.as_mut_ptr());
            let name = callback!(self, FeatureDef_getName, definition);
            if name.is_null() {
                return Err(Error::InvalidInput("null feature definition name"));
            }
            result.push(FeatureSnapshot {
                id: FeatureId(id),
                definition: FeatureDefId(definition),
                position,
                name: unsafe { CStr::from_ptr(name) }
                    .to_string_lossy()
                    .into_owned(),
            });
        }
        Ok(result)
    }
    pub fn map_size(&self) -> Result<[f32; 2]> {
        Ok([
            callback!(self, Map_getWidth) as f32 * 8.,
            callback!(self, Map_getHeight) as f32 * 8.,
        ])
    }
    fn require_owned(&self, id: UnitId) -> Result<()> {
        if id.0 < 0 || callback!(self, Unit_getTeam, id.0) != self.team()?.0 {
            Err(Error::InvalidInput("orders require own unit"))
        } else {
            Ok(())
        }
    }
    fn issue<T>(&self, topic: u32, data: &mut T) -> Result<()> {
        let status = callback!(
            self,
            Engine_handleCommand,
            sys::COMMAND_TO_ID_ENGINE,
            -1,
            topic as i32,
            (data as *mut T).cast::<c_void>()
        );
        if status == 0 {
            Ok(())
        } else {
            Err(Error::Engine(status))
        }
    }
    /// Ordinary AI unit order. The engine transmits it in the replay command stream.
    /// Custom command semantics and permission checks belong to the game.
    pub fn custom_order(
        &self,
        unit: UnitId,
        command: i32,
        params: &[f32],
        options: OrderOptions,
    ) -> Result<()> {
        self.require_owned(unit)?;
        if params.len() > 4096 || params.iter().any(|n| !n.is_finite()) {
            return Err(Error::InvalidInput("invalid command parameters"));
        }
        let mut params = params.to_vec();
        let mut data = sys::SCustomUnitCommand {
            unitId: unit.0,
            groupId: -1,
            options: options.flags,
            timeOut: options.timeout_frame.unwrap_or(i32::MAX),
            cmdId: command,
            params: params.as_mut_ptr(),
            params_size: params.len() as i32,
        };
        self.issue(sys::COMMAND_UNIT_CUSTOM, &mut data)
    }
    pub fn move_to(&self, unit: UnitId, position: [f32; 3], options: OrderOptions) -> Result<()> {
        self.custom_order(unit, 10, &position, options)
    }
    pub fn fight(&self, unit: UnitId, position: [f32; 3], options: OrderOptions) -> Result<()> {
        self.custom_order(unit, 16, &position, options)
    }
    pub fn attack(&self, unit: UnitId, target: UnitId, options: OrderOptions) -> Result<()> {
        if target.0 < 0 {
            return Err(Error::InvalidInput("invalid target"));
        }
        self.custom_order(unit, 20, &[target.0 as f32], options)
    }
    pub fn stop(&self, unit: UnitId) -> Result<()> {
        self.custom_order(unit, 0, &[], OrderOptions::default())
    }
}
