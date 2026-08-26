#![allow(clippy::all)]

include!("probe_bindings.rs");
mod probe_context {
    include!("probe_context.rs");
}

use std::sync::atomic::{AtomicBool, Ordering};

use bindings::recoil::spring_api::feature_defs;
use bindings::recoil::spring_api::features;
use bindings::recoil::spring_api::game;
use bindings::recoil::spring_api::messages;
use bindings::recoil::spring_api::projectiles;
use bindings::recoil::spring_api::projectiles::GetAllProjectilesOptions;
use bindings::recoil::spring_api::teams;
use bindings::recoil::spring_api::unit_defs;
use bindings::recoil::spring_api::units_info;
use bindings::recoil::spring_api::units_query;
use bindings::recoil::spring_api::weapon_defs;
use callin::{GameFrameQuery, GameFrameResult, Guest, SpringError, UpdateQuery, UpdateResult};
#[cfg(parity_has_synced_message)]
use callin::RecvFromSyncedQuery;

pub struct Fixture {
    pub unit_id: i32,
    pub extractor_unit_id: i32,
    pub feature_id: i32,
    pub projectile_id: i32,
    pub unit_def_id: i32,
    pub feature_def_id: i32,
    pub weapon_def_id: i32,
    pub weapon_def_name: String,
    pub team_id: i32,
    pub ally_team_id: i32,
    // The UI guest discovers the LOS and radar-only enemy IDs from the
    // visibility-filtered unit inventory and their typed LOS state. A hidden
    // unit is intentionally not enumerable from UI, so hidden-state coverage
    // remains in the Lua/native visibility fixture rather than inventing an
    // ID that would make a Wasm probe vacuous.
    pub enemy_los_unit_id: i32,
    pub enemy_radar_unit_id: i32,
    pub player_id: i32,
    pub piece_projectile_id: i32,
    pub ground_x: f32,
    pub ground_z: f32,
}

fn discover_fixture() -> Result<Fixture, String> {
    // Resolve the fixture definitions independently, then match actual
    // instances against those IDs.  Looking up a name after every
    // GetUnitDefID is not reliable in unsynced contexts: the engine may
    // intentionally redact or omit the name there even though the unit is
    // visible and all typed ID APIs are valid.
    let fixture_unit_def_id = unit_defs::get_unit_def_id_by_name("native_api_test_unit")
        .map_err(|error| format!("get-unit-def-id-by-name:{}", error.code))?;
    let extractor_unit_def_id = unit_defs::get_unit_def_id_by_name("native_api_test_extractor")
        .map_err(|error| format!("get-extractor-unit-def-id-by-name:{}", error.code))?;
    let unit_ids = probe_context::fixture_unit_ids()?;
    let mut unit = None;
    let mut extractor_unit = None;
    let mut unit_candidates = Vec::new();
    for candidate_id in unit_ids.iter().copied() {
        // LuaUI can enumerate radar-visible units whose definition is still
        // intentionally redacted.  Match Lua's `Spring.GetUnitDefID`/nil
        // behaviour by skipping those candidates rather than turning one
        // opaque enemy handle into a terminal fixture-discovery error.
        let candidate_def_id = match units_info::get_unit_def_id(candidate_id) {
            Ok(def_id) => def_id,
            Err(_) => continue,
        };
        unit_candidates.push((candidate_id, candidate_def_id));
        if candidate_def_id == extractor_unit_def_id && extractor_unit.is_none() {
            extractor_unit = Some(candidate_id);
        }
        if candidate_def_id == fixture_unit_def_id
            && unit.is_none()
            && probe_context::unit_candidate_is_primary(candidate_id)
            && units_info::get_unit_team(candidate_id).ok() == Some(0)
        {
            unit = Some((candidate_id, candidate_def_id));
        }
    }
    let (unit_id, unit_def_id) =
        unit.ok_or_else(|| {
            format!(
                "native-api-test-unit-not-found:def={fixture_unit_def_id}:candidates={unit_candidates:?}"
            )
        })?;

    let fixture_feature_def_id =
        feature_defs::get_feature_def_id_by_name("native_api_test_feature")
            .map_err(|error| format!("get-feature-def-id-by-name:{}", error.code))?;
    let feature_ids = probe_context::fixture_feature_ids()?;
    let mut feature = None;
    for candidate_id in feature_ids {
        let candidate_def_id = match features::get_feature_def_id(candidate_id) {
            Ok(def_id) => def_id,
            Err(_) => continue,
        };
        if candidate_def_id == fixture_feature_def_id
            && probe_context::feature_candidate_is_primary(candidate_id)
        {
            feature = Some((candidate_id, candidate_def_id));
            break;
        }
    }
    let (feature_id, feature_def_id) =
        feature.ok_or_else(|| "native-api-test-feature-not-found".to_string())?;

    let team_id = units_info::get_unit_team(unit_id)
        .map_err(|error| format!("get-unit-team:{}", error.code))?;
    let ally_team_id = units_info::get_unit_ally_team(unit_id)
        .map_err(|error| format!("get-unit-ally-team:{}", error.code))?;
    let player_id = teams::get_player_list_in_team(team_id)
        .map_err(|error| format!("get-player-list-in-team:{}", error.code))?
        .into_iter()
        .next()
        .ok_or_else(|| "native-api-test-team-has-no-player".to_string())?;

    let projectile_ids = projectiles::get_all_projectiles(GetAllProjectilesOptions {
        exclude_weapon_projectiles: false,
        exclude_piece_projectiles: false,
    })
    .map_err(|error| format!("get-all-projectiles:{}", error.code))?;

    let mut weapon_projectile = None;
    let mut piece_projectile = None;
    let mut projectile_candidates = Vec::new();
    for candidate_id in projectile_ids {
        let owner_id = match projectiles::get_projectile_owner_id(candidate_id) {
            Ok(owner_id) => owner_id,
            Err(_) => continue,
        };
        projectile_candidates.push((candidate_id, owner_id));
        if owner_id != unit_id {
            continue;
        }
        let projectile_type = match projectiles::get_projectile_type(candidate_id) {
            Ok(projectile_type) => projectile_type,
            Err(_) => continue,
        };
        if projectile_type.weapon && weapon_projectile.is_none() {
            if let Ok(candidate_def_id) = projectiles::get_projectile_def_id(candidate_id) {
                weapon_projectile = Some((candidate_id, candidate_def_id));
            }
        }
        if projectile_type.piece && piece_projectile.is_none() {
            piece_projectile = Some(candidate_id);
        }
        if weapon_projectile.is_some() && piece_projectile.is_some() {
            break;
        }
    }
    let (projectile_id, weapon_def_id) = weapon_projectile.ok_or_else(|| {
        format!(
            "native-api-test-weapon-projectile-not-found:unit={unit_id}:candidates={projectile_candidates:?}"
        )
    })?;
    let extractor_unit_id = extractor_unit.ok_or_else(|| {
        format!(
            "native-api-test-extractor-unit-not-found:def={extractor_unit_def_id}:candidates={unit_candidates:?}"
        )
    })?;
    let (enemy_los_unit_id, enemy_radar_unit_id) =
        match probe_context::visibility_enemy_ids(&unit_ids, team_id, ally_team_id)? {
            Some(ids) => ids,
            None => (unit_id, unit_id),
        };
    let piece_projectile_id =
        piece_projectile.ok_or_else(|| {
            format!(
                "native-api-test-piece-projectile-not-found:unit={unit_id}:candidates={projectile_candidates:?}"
            )
        })?;

    let ground_position = units_info::get_unit_position(
        unit_id,
        units_info::GetUnitPositionOptions {
            mid_pos: false,
            aim_pos: false,
        },
    )
    .map_err(|error| {
        format!(
            "get-unit-position:{}:unit={}:candidates={:?}",
            error.code, unit_id, unit_candidates
        )
    })?;

    let weapon_def_name = weapon_defs::get_weapon_def_name(weapon_def_id)
        .map_err(|error| format!("get-weapon-def-name:{}", error.code))?;

    Ok(Fixture {
        unit_id,
        extractor_unit_id,
        feature_id,
        projectile_id,
        unit_def_id,
        feature_def_id,
        weapon_def_id,
        weapon_def_name,
        team_id,
        ally_team_id,
        enemy_los_unit_id,
        enemy_radar_unit_id,
        player_id,
        piece_projectile_id,
        ground_x: (ground_position.x * 1000.0 + 0.5).floor() / 1000.0,
        ground_z: (ground_position.z * 1000.0 + 0.5).floor() / 1000.0,
    })
}

#[allow(dead_code, unused_parens, unused_variables)]
mod generated {
    include!("probe_generated.rs");
}

struct ParityGuest;

static PROBE_RAN: AtomicBool = AtomicBool::new(false);
static DETERMINISM_SENT: AtomicBool = AtomicBool::new(false);
static LATEST_GAME_FRAME: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(-1);

fn deterministic_fp_signature() -> String {
    let signed_zero = (-0.0f32).to_bits();
    let denormal = f32::from_bits(1).to_bits();
    let nan = u8::from(f32::from_bits(0x7fc0_0001).is_nan());
    let square_root = 4.0f32.sqrt().to_bits();
    let negative_square_root_nan = u8::from((-1.0f32).sqrt().is_nan());
    let truncation = (-1.5f32) as i32;
    let high_boundary = 2147483648.0f32 as i32;
    let nan_cast = f32::NAN as i32;

    format!(
        "{signed_zero:08x}:{denormal:08x}:{nan}:{square_root:08x}:\
         {negative_square_root_nan}:{truncation}:{high_boundary}:{nan_cast}"
    )
}

fn deterministic_rng_signature(frame: i32, count: u32) -> String {
    let mut state = (frame as u32).wrapping_mul(31).wrapping_add(count);
    let mut values = [0u32; 3];
    for value in &mut values {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        *value = state;
    }

    format!("{:08x}:{:08x}:{:08x}", values[0], values[1], values[2])
}

fn send_determinism(frame: i32) -> Result<(), SpringError> {
    let count =
        units_query::get_team_unit_count(0).map_err(|error| SpringError { code: error.code })?;
    let message = format!(
        "WASM_PARITY|{}|{}|{}|{}",
        frame,
        count,
        deterministic_fp_signature(),
        deterministic_rng_signature(frame, count),
    );
    send_parity_message(&message)?;
    Ok(())
}

#[cfg(all(feature = "core", any(feature = "core_rules_synced", feature = "core_gaia_synced")))]
fn send_parity_message(message: &str) -> Result<(), SpringError> {
    messages::send_to_unsynced(message).map_err(|error| SpringError { code: error.code })?;
    Ok(())
}

#[cfg(all(feature = "core", any(feature = "core_rules_unsynced", feature = "core_gaia_unsynced", feature = "core_ui")))]
fn send_parity_message(message: &str) -> Result<(), SpringError> {
    messages::send_lua_rules_msg(message).map_err(|error| SpringError { code: error.code })?;
    Ok(())
}

#[cfg(all(feature = "core", not(any(
    feature = "core_rules_synced",
    feature = "core_rules_unsynced",
    feature = "core_gaia_synced",
    feature = "core_gaia_unsynced",
    feature = "core_ui",
))))]
fn send_parity_message(message: &str) -> Result<(), SpringError> {
    messages::send_to_unsynced(message).map_err(|error| SpringError { code: error.code })?;
    Ok(())
}

#[cfg(not(feature = "core"))]
fn send_parity_message(message: &str) -> Result<(), SpringError> {
    messages::send_lua_rules_msg(message).map_err(|error| SpringError { code: error.code })?;
    #[cfg(parity_is_synced)]
    messages::send_to_unsynced("native_api_wasm_direct_probe")
        .map_err(|error| SpringError { code: error.code })?;
    Ok(())
}

fn send_fixture_status(status: &str, detail: &str) {
    let safe_detail = detail.replace('|', "_");
    let message = format!("WASM_API_STATUS|fixture|{status}|{safe_detail}");
    let _ = send_parity_message(&message);
}

fn run_generated_probe(current_frame: i32) {
    if PROBE_RAN.load(Ordering::Acquire) {
        return;
    }
    let fixture = match discover_fixture() {
        Ok(fixture) => fixture,
        Err(reason) => {
            // Core callbacks can run before the synced fixture's
            // projectile has entered the query-visible list. Keep the
            // discovery boundary retryable for the first few simulation
            // frames; recording that scheduling race as a terminal result
            // would turn a valid fixture into a vacuous parity failure.
            if current_frame < 5 || probe_context::WAIT_FOR_UNSYNCED_FIXTURE {
                return;
            }
            send_fixture_status("error", &reason);
            PROBE_RAN.store(true, Ordering::Release);
            return;
        }
    };
    if probe_context::WAIT_FOR_UNSYNCED_FIXTURE
        && !probe_context::fixture_ready(fixture.unit_id, fixture.feature_id)
    {
        return;
    }
    if let Err(reason) = probe_context::prepare_probe() {
        if current_frame < 5 || probe_context::WAIT_FOR_UNSYNCED_FIXTURE {
            return;
        }
        send_fixture_status("error", &reason);
        PROBE_RAN.store(true, Ordering::Release);
        return;
    }
    let fixture_detail = format!(
        "unit={};extractor={};feature={};projectile={};piece={};team={};ally={};player={}",
        fixture.unit_id,
        fixture.extractor_unit_id,
        fixture.feature_id,
        fixture.projectile_id,
        fixture.piece_projectile_id,
        fixture.team_id,
        fixture.ally_team_id,
        fixture.player_id,
    );
    send_fixture_status("ready", &fixture_detail);
    generated::run(&fixture, |message| {
        let _ = send_parity_message(&message);
    });
    PROBE_RAN.store(true, Ordering::Release);
}

impl Guest for ParityGuest {
    #[cfg(parity_has_synced_message)]
    fn recv_from_synced(query: RecvFromSyncedQuery) -> Result<(), SpringError> {
        let message = query.message.replace('|', "_");
        let marker = format!("WASM_DIRECT_SYNCED|{}|{}", query.message_length, message);
        let _ = send_parity_message(&marker);
        Ok(())
    }

    fn game_frame(query: GameFrameQuery) -> Result<GameFrameResult, SpringError> {
        LATEST_GAME_FRAME.store(query.game_frame, Ordering::Release);
        // Unsynced Update is the first callback that is guaranteed to run
        // after the synced fixture has crossed into the render-side handle.
        // Native GameFrame dispatch can precede that hand-off, so sampling
        // the unit inventory here would create a false zero-count parity row.
        if query.game_frame != 3 || probe_context::WAIT_FOR_UNSYNCED_FIXTURE {
            return Ok(GameFrameResult { unused: 0 });
        }
        send_determinism(query.game_frame)?;
        Ok(GameFrameResult { unused: 0 })
    }

    fn game_frame_post(query: GameFrameQuery) -> Result<GameFrameResult, SpringError> {
        // The Lua fixture creates its unit, feature, and projectile during
        // initialization and runs the matching reference probes at frame 2.
        // Waiting for that same post-frame gives the native queries a fully
        // populated world; frame 1 can still be before feature/projectile
        // registration has become visible to the native read surfaces.
        if !probe_context::WAIT_FOR_UNSYNCED_FIXTURE && (2..=5).contains(&query.game_frame) {
            run_generated_probe(query.game_frame);
        }
        Ok(GameFrameResult { unused: 0 })
    }

    fn update(_query: UpdateQuery) -> Result<UpdateResult, SpringError> {
        if probe_context::WAIT_FOR_UNSYNCED_FIXTURE {
            // Unsynced gadgets do not receive the synced GameFrame callin.
            // Read the same authoritative frame through the game callout so
            // the fixture gate does not wait forever or require a broader
            // callin environment mask.
            if let Ok(frame) = game::get_game_frame(0u8) {
                let current_frame = ((frame.high16 << 16) | frame.low16) as i32;
                LATEST_GAME_FRAME.store(current_frame, Ordering::Release);
            }
        }
        let probe_gate = probe_context::WAIT_FOR_UNSYNCED_FIXTURE
            && LATEST_GAME_FRAME.load(Ordering::Acquire) >= 10;
        // Update is render-rate driven and can run several times during the
        // first simulation frame. An update-count threshold is not a
        // fixture-readiness boundary; use the authoritative game frame so
        // render startup cannot observe objects before Lua setup completes.
        if probe_gate {
            let frame = LATEST_GAME_FRAME.load(Ordering::Acquire);
            run_generated_probe(frame as i32);
            if PROBE_RAN.load(Ordering::Acquire) && !DETERMINISM_SENT.swap(true, Ordering::AcqRel) {
                send_determinism(3)?;
            }
        }
        Ok(UpdateResult { unused: 0 })
    }
}

impl bindings::Guest for ParityGuest {
    fn callback_1(user_data: u32) {
        generated::callback_1(user_data);
    }
}

#[cfg(not(feature = "core"))]
bindings::export!(ParityGuest with_types_in bindings);

#[cfg(feature = "core")]
fn core_game_frame(frame: i32) {
    let _ = <ParityGuest as Guest>::game_frame(GameFrameQuery { game_frame: frame });
}

#[cfg(feature = "core")]
fn core_game_frame_post(frame: i32) {
    let _ = <ParityGuest as Guest>::game_frame_post(GameFrameQuery { game_frame: frame });
}

#[cfg(feature = "core")]
fn core_update(delta_seconds: f32) {
    let _ = <ParityGuest as Guest>::update(UpdateQuery { delta_seconds });
}

#[cfg(feature = "core")]
spring::export_game_frame!(core_game_frame);
#[cfg(feature = "core")]
spring::export_game_frame_post!(core_game_frame_post);
#[cfg(feature = "core")]
spring::export_update!(core_update);

#[cfg(all(feature = "core", feature = "core_rules_synced"))]
spring::export_environment_mask!(spring::rules_synced::ENVIRONMENT_MASK);
#[cfg(all(feature = "core", feature = "core_rules_unsynced"))]
spring::export_environment_mask!(spring::rules_unsynced::ENVIRONMENT_MASK);
#[cfg(all(feature = "core", feature = "core_gaia_synced"))]
spring::export_environment_mask!(spring::gaia_synced::ENVIRONMENT_MASK);
#[cfg(all(feature = "core", feature = "core_gaia_unsynced"))]
spring::export_environment_mask!(spring::gaia_unsynced::ENVIRONMENT_MASK);
#[cfg(all(feature = "core", feature = "core_ui"))]
spring::export_environment_mask!(spring::ui::ENVIRONMENT_MASK);
