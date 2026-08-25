pub use crate::{
    export_allow_unit_creation, export_draw_world, export_environment_mask, export_game_frame,
    export_game_frame_post, export_unit_created, export_unit_pre_damaged, export_update,
    AllowUnitCreationResult, ApiError, DamageResult, ErrorCode, Result, UnitHealth, ABI_VERSION,
};

pub use crate::{
    create_unit, set_camera_state, spawn_ceg, spawn_projectile, unit_piece_position_by_name,
    CameraState, CreateUnitOptions, Float3, ProjectileParams, SpawnCEGResult, UnitDefRef,
};

/// Environment-scoped generated surfaces. Keeping these names under the
/// prelude makes the safe entry point discoverable without flattening APIs
/// from incompatible execution environments into one namespace.
pub use crate::generated::{gaia_synced, gaia_unsynced, rules_synced, rules_unsynced, ui};

pub use crate::typed::{call_unit_script, UnitScriptCallResult};
