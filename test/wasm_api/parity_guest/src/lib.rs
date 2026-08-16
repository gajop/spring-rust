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

pub struct Fixture {
    pub unit_id: i32,
    pub feature_id: i32,
    pub projectile_id: i32,
    pub unit_def_id: i32,
    pub feature_def_id: i32,
    pub weapon_def_id: i32,
    pub weapon_def_name: String,
    pub team_id: i32,
    pub ally_team_id: i32,
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
    let unit_ids = probe_context::fixture_unit_ids()?;
    let mut unit = None;
    let mut unit_candidates = Vec::new();
    for candidate_id in unit_ids {
        let candidate_def_id = units_info::get_unit_def_id(candidate_id)
            .map_err(|error| format!("get-unit-def-id:{}", error.code))?;
        unit_candidates.push((candidate_id, candidate_def_id));
        if candidate_def_id == fixture_unit_def_id
            && probe_context::unit_candidate_is_primary(candidate_id)
        {
            unit = Some((candidate_id, candidate_def_id));
            break;
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
        let candidate_def_id = features::get_feature_def_id(candidate_id)
            .map_err(|error| format!("get-feature-def-id:{}", error.code))?;
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
    for candidate_id in projectile_ids {
        let owner_id = projectiles::get_projectile_owner_id(candidate_id)
            .map_err(|error| format!("get-projectile-owner-id:{}", error.code))?;
        if owner_id != unit_id {
            continue;
        }
        let projectile_type = projectiles::get_projectile_type(candidate_id)
            .map_err(|error| format!("get-projectile-type:{}", error.code))?;
        if projectile_type.weapon && weapon_projectile.is_none() {
            let candidate_def_id = projectiles::get_projectile_def_id(candidate_id)
                .map_err(|error| format!("get-projectile-def-id:{}", error.code))?;
            weapon_projectile = Some((candidate_id, candidate_def_id));
        }
        if projectile_type.piece && piece_projectile.is_none() {
            piece_projectile = Some(candidate_id);
        }
        if weapon_projectile.is_some() && piece_projectile.is_some() {
            break;
        }
    }
    let (projectile_id, weapon_def_id) = weapon_projectile
        .ok_or_else(|| "native-api-test-weapon-projectile-not-found".to_string())?;
    let piece_projectile_id =
        piece_projectile.ok_or_else(|| "native-api-test-piece-projectile-not-found".to_string())?;

    let ground_position = units_info::get_unit_position(
        unit_id,
        units_info::GetUnitPositionOptions {
            mid_pos: false,
            aim_pos: false,
        },
    )
    .map_err(|error| format!("get-unit-position:{}", error.code))?;

    let weapon_def_name = weapon_defs::get_weapon_def_name(weapon_def_id)
        .map_err(|error| format!("get-weapon-def-name:{}", error.code))?;

    Ok(Fixture {
        unit_id,
        feature_id,
        projectile_id,
        unit_def_id,
        feature_def_id,
        weapon_def_id,
        weapon_def_name,
        team_id,
        ally_team_id,
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
    messages::send_lua_rules_msg(&message).map_err(|error| SpringError { code: error.code })?;
    Ok(())
}

fn send_fixture_status(status: &str, detail: &str) {
    let safe_detail = detail.replace('|', "_");
    let message = format!("WASM_API_STATUS|fixture|{status}|{safe_detail}");
    let _ = messages::send_lua_rules_msg(&message);
}

fn run_generated_probe() {
    if PROBE_RAN.load(Ordering::Acquire) {
        return;
    }
    let fixture = match discover_fixture() {
        Ok(fixture) => fixture,
        Err(reason) => {
            // Unsynced Update may be delivered before the synced gadget's
            // CreateUnit/CreateFeature hand-off.  Keep discovery retryable in
            // that environment; recording the first empty inventory as a
            // terminal result would turn a scheduling race into a vacuous
            // parity failure.
            if probe_context::WAIT_FOR_UNSYNCED_FIXTURE {
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
        if probe_context::WAIT_FOR_UNSYNCED_FIXTURE {
            return;
        }
        send_fixture_status("error", &reason);
        PROBE_RAN.store(true, Ordering::Release);
        return;
    }
    let fixture_detail = format!(
        "unit={};feature={};projectile={};piece={};team={};ally={};player={}",
        fixture.unit_id,
        fixture.feature_id,
        fixture.projectile_id,
        fixture.piece_projectile_id,
        fixture.team_id,
        fixture.ally_team_id,
        fixture.player_id,
    );
    send_fixture_status("ready", &fixture_detail);
    generated::run(&fixture, |message| {
        let _ = messages::send_lua_rules_msg(&message);
    });
    PROBE_RAN.store(true, Ordering::Release);
}

impl Guest for ParityGuest {
    fn game_frame(query: GameFrameQuery) -> Result<GameFrameResult, SpringError> {
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
        if !probe_context::WAIT_FOR_UNSYNCED_FIXTURE && query.game_frame == 2 {
            run_generated_probe();
        }
        Ok(GameFrameResult { unused: 0 })
    }

    fn update(_query: UpdateQuery) -> Result<UpdateResult, SpringError> {
        if probe_context::WAIT_FOR_UNSYNCED_FIXTURE
            // Update is render-rate driven and can run several times during
            // the first simulation frame. An update-count threshold is not a
            // fixture-readiness boundary; use the authoritative game frame
            // so render startup cannot observe objects before the Lua setup at
            // frame four has completed.
            && game::get_game_frame(0)
                .ok()
                .map(|frame| frame.low16 | (frame.high16 << 16))
                .is_some_and(|frame| frame >= 10)
        {
            run_generated_probe();
            if PROBE_RAN.load(Ordering::Acquire) && !DETERMINISM_SENT.swap(true, Ordering::AcqRel) {
                send_determinism(3)?;
            }
        }
        Ok(UpdateResult { unused: 0 })
    }
}

bindings::export!(ParityGuest with_types_in bindings);
