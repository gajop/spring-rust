/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */
//! Engine-owned determinism fixture. No game assets or legacy SDK dependency.
#[cfg(feature = "gaia")]
use spring_sdk::GaiaSynced as Environment;
#[cfg(not(feature = "gaia"))]
use spring_sdk::RulesSynced as Environment;
use spring_sdk::{
    Result, Spring,
    jobs::{JobError, JobId},
    module::Module,
};
use std::cell::{Cell, RefCell};
use std::hint::black_box;

struct Probe {
    frame: Cell<i32>,
    seed: Cell<u32>,
    cross_world: Cell<bool>,
    callbacks: Cell<u32>,
    digest: Cell<u64>,
    pending: RefCell<Vec<(u32, i32, Vec<u8>)>>,
    allocations: RefCell<Vec<Vec<u8>>>,
    player_info: RefCell<String>,
}
impl Probe {
    fn retain(&self, bytes: &[u8]) {
        let mut hash = self.digest.get();
        for &byte in bytes {
            hash = (hash ^ u64::from(byte)).wrapping_mul(0x100000001b3);
        }
        self.digest.set(hash);
    }
}

#[allow(clippy::eq_op)] // Deliberately exercise 0/0 and NaN arithmetic.
fn work(input: &[u8]) -> Vec<u8> {
    let mut value = u32::from_le_bytes(input[..4].try_into().unwrap());
    // Unequal durations change completion order without changing delivery order.
    for _ in 0..(input[4] as u32 * 701) {
        value = value.wrapping_mul(1664525).wrapping_add(1013904223);
    }
    let mut out = value.to_le_bytes().to_vec();
    for bits in [
        0u32, 0x80000000, 0x00800000, 1, 0x3f800000, 0x7f800001, 0xffc12345,
    ] {
        let x = black_box(f32::from_bits(bits));
        for y in [
            x * black_box(0.5),
            x + black_box(f32::from_bits(0x33800000)),
            x / x,
        ] {
            out.extend_from_slice(&y.to_bits().to_le_bytes());
        }
    }
    let x = black_box(f64::from_bits(0x0010000000000000));
    out.extend_from_slice(&(x * black_box(0.5)).to_bits().to_le_bytes());
    out
}
impl Module<Environment> for Probe {
    fn initialize(spring: &mut Spring<'_, Environment>) -> Result<Self> {
        spring
            .messages()
            .echo("[SyncAdversarial] initialized", "")?;
        Ok(Self {
            frame: Cell::new(-1),
            seed: Cell::new(0),
            cross_world: Cell::new(false),
            callbacks: Cell::new(0),
            digest: Cell::new(0xcbf29ce484222325),
            pending: RefCell::new(Vec::new()),
            allocations: RefCell::new(Vec::new()),
            player_info: RefCell::new(String::new()),
        })
    }
    fn job_result(
        &self,
        spring: &mut Spring<'_, Environment>,
        id: JobId,
        result: core::result::Result<&[u8], JobError>,
    ) -> Result<()> {
        let (expected_id, submitted, bytes) = self.pending.borrow_mut().remove(0);
        assert_eq!(id.0, expected_id);
        assert_eq!(spring.game().current_frame()?, submitted + 1);
        assert_eq!(result, Ok(bytes.as_slice()));
        self.retain(result.unwrap());
        if submitted % 127 == 0 {
            // A result callback may submit next-frame work while the previous
            // batch is being drained. This must neither deliver early nor
            // disturb the globally ordered result IDs of other modules.
            let mut input = (id.0 ^ self.seed.get()).to_le_bytes().to_vec();
            input.push((id.0 % 8) as u8);
            let expected = work(&input);
            let next = spring.jobs().submit(MIX, &input)?;
            self.pending
                .borrow_mut()
                .push((next.0, submitted + 1, expected));
            input.fill(0xdd);
        }
        Ok(())
    }
}
fn frame(probe: &Probe, spring: &mut Spring<'_, Environment>, frame: i32) -> Result<()> {
    assert_eq!(frame, probe.frame.replace(frame) + 1);
    if frame == 0 {
        probe.seed.set(
            spring
                .rules_params()
                .get_game_rules_param_f32("sync_adversarial_seed")? as u32,
        );
        probe.cross_world.set(
            spring
                .rules_params()
                .get_game_rules_param_f32("sync_adversarial_cross")?
                != 0.0,
        );
    }
    assert!(
        probe
            .pending
            .borrow()
            .iter()
            .all(|(_, submitted, _)| *submitted == frame)
    );
    probe.retain(&frame.to_le_bytes());
    if frame == 7
        && spring
            .rules_params()
            .get_game_rules_param_f32("sync_adversarial_clock")?
            != 0.0
    {
        // Explicitly enabled test-only negative control. The runner must find
        // its first memory divergence at this exact frame, with identical code.
        probe.retain(&spring.profiling().get_timer_micros()?.to_le_bytes());
    }
    let def = spring
        .unit_defs()
        .get_unit_def_id_by_name("native_api_test_unit")?;
    assert_eq!(
        spring
            .unit_defs()
            .get_unit_def_custom_param(def, "missing-key")
            .unwrap_err()
            .code,
        spring_sdk::ErrorCode::NotFound as i32
    );
    // Retain the complete typed result, including player ping/CPU fields. The
    // test must expose nondeterministic API values instead of filtering them.
    let info = format!("{:?}", spring.teams().get_player_info(0, true)?);
    if *probe.player_info.borrow() != info {
        spring.messages().echo(
            &format!("[SyncAdversarial] player-frame={frame} {info}"),
            "",
        )?;
        *probe.player_info.borrow_mut() = info.clone();
    }
    probe.retain(info.as_bytes());
    probe.retain(format!("{:?}", spring.teams().get_team_resources(0, "metal")?).as_bytes());
    probe.retain(&spring.synced_random().next_float()?.to_bits().to_le_bytes());
    probe.retain(
        &spring
            .synced_random()
            .next_int(-12345, 67890)?
            .to_le_bytes(),
    );
    if frame % 31 == 0 {
        let mut called = false;
        assert!(spring.terrain_control().set_height_map_func(|spring| {
            // Engine -> guest -> engine while the original call is active.
            assert_eq!(spring.game().current_frame().unwrap(), frame);
            let height = 96.0 + ((frame as u32 ^ probe.seed.get()) % 17) as f32;
            assert!(
                spring
                    .terrain_control()
                    .set_height_map(8.0, 8.0, height, 1.0)
                    .unwrap()
            );
            probe.retain(&height.to_bits().to_le_bytes());
            probe.callbacks.set(probe.callbacks.get() + 1);
            called = true;
        })?);
        assert!(called, "terrain callback was not invoked");
        probe.retain(
            &spring
                .terrain()
                .get_ground_height(8.0, 8.0)?
                .to_bits()
                .to_le_bytes(),
        );
    }
    let units = spring.units().get_all_units()?;
    for unit in units {
        let pos = spring
            .units_info()
            .get_unit_position(unit, Default::default())?;
        probe.retain(
            format!(
                "{:?}",
                spring
                    .units_info()
                    .get_unit_states(unit, Default::default())?
            )
            .as_bytes(),
        );
        probe.retain(&unit.to_le_bytes());
        for value in [pos.x, pos.y, pos.z] {
            probe.retain(&value.to_bits().to_le_bytes());
        }
        probe.retain(format!("{:?}", spring.units_info().get_unit_health(unit)?).as_bytes());
    }
    if frame % 17 == 0 {
        let bytes = 65536 * (1 + ((frame / 17) % 9)) as usize;
        let mut allocations = probe.allocations.borrow_mut();
        allocations.push(vec![frame as u8; bytes]);
        if allocations.len() > 4 {
            allocations.remove(0);
        }
        for allocation in allocations.iter() {
            probe.retain(&[allocation[0], allocation[allocation.len() - 1]]);
        }
    }
    if frame % 7 == 0 {
        for i in 0..8u32 {
            let mut input = (i ^ frame as u32 ^ probe.seed.get()).to_le_bytes().to_vec();
            input.push((7 - i) as u8);
            let expected = work(&input);
            let worker = if i % 2 == 0 {
                MIX
            } else if probe.cross_world.get() {
                MIX.in_wasm_module(if cfg!(feature = "gaia") {
                    "rules-0"
                } else {
                    "gaia-0"
                })
            } else {
                MIX.in_wasm_module(if cfg!(feature = "gaia") {
                    "gaia-0"
                } else {
                    "rules-0"
                })
            };
            let id = spring.jobs().submit(worker, &input)?;
            probe.pending.borrow_mut().push((id.0, frame, expected));
            input.fill(0xee); // engine must own a copy, not borrow this input
        }
    }
    if frame % 60 == 0 {
        spring.messages().echo(
            &format!(
                "[SyncAdversarial] frame={frame} callbacks={} digest={:016x}",
                probe.callbacks.get(),
                probe.digest.get()
            ),
            "",
        )?;
    }
    Ok(())
}
spring_sdk::export_module!(PROBE, Environment, Probe);
spring_sdk::export_game_frame!(PROBE, frame);
spring_sdk::export_jobs!(PROBE, { MIX => ("mix", work) });
