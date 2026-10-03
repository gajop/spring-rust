use super::*;
use std::sync::{
    Mutex,
    atomic::{AtomicUsize, Ordering},
};
static HANDLED: Mutex<Vec<(i32, i32)>> = Mutex::new(Vec::new());
static CHEATS: AtomicUsize = AtomicUsize::new(0);
unsafe extern "system" fn team(ai: i32) -> i32 {
    ai
}
unsafe extern "system" fn maximum(_: i32) -> i32 {
    4
}
unsafe extern "system" fn friendly(_: i32, out: *mut i32, _: i32) -> i32 {
    unsafe {
        out.write(10);
        out.add(1).write(20)
    };
    2
}
unsafe extern "system" fn unit_team(_: i32, unit: i32) -> i32 {
    if unit == 10 { 1 } else { 2 }
}
unsafe extern "system" fn command(
    ai: i32,
    to: i32,
    id: i32,
    topic: i32,
    data: *mut std::ffi::c_void,
) -> i32 {
    assert_eq!((to, id, topic), (-1, -1, sys::COMMAND_UNIT_CUSTOM as i32));
    let order = unsafe { &*data.cast::<sys::SCustomUnitCommand>() };
    assert_eq!(order.unitId, 10);
    assert_eq!(order.cmdId, 10);
    assert_eq!(
        unsafe { std::slice::from_raw_parts(order.params, order.params_size as usize) },
        [10., 20., 30.]
    );
    assert_eq!(ai, 1);
    0
}
unsafe extern "system" fn cheat(_: i32, _: bool) -> bool {
    CHEATS.fetch_add(1, Ordering::SeqCst);
    true
}
fn table() -> sys::SSkirmishAICallback {
    sys::SSkirmishAICallback {
        SkirmishAI_getTeamId: Some(team),
        Unit_getMax: Some(maximum),
        getFriendlyUnits: Some(friendly),
        Unit_getTeam: Some(unit_team),
        Engine_handleCommand: Some(command),
        Cheats_setEnabled: Some(cheat),
        ..Default::default()
    }
}
struct Mock {
    count: i32,
}
impl SkirmishAi for Mock {
    fn new(c: &Context<'_>) -> Result<Self> {
        if c.ai_id().0 == 99 {
            panic!("constructor panic")
        };
        Ok(Self { count: 0 })
    }
    fn handle(&mut self, c: &Context<'_>, e: Event<'_>) -> Result<()> {
        if let Event::Update { frame } = e {
            if frame == 999 {
                panic!("planner panic")
            };
            self.count += 1;
            HANDLED.lock().unwrap().push((c.ai_id().0, self.count));
        }
        Ok(())
    }
}
#[test]
fn callbacks_filter_allies_guard_orders_and_never_enable_cheats() {
    let t = table();
    let c = Context::new(AiId(1), &t);
    assert_eq!(c.owned_units().unwrap(), [UnitId(10)]);
    assert!(
        c.move_to(UnitId(20), [10., 20., 30.], OrderOptions::default())
            .is_err()
    );
    assert!(
        c.move_to(UnitId(10), [f32::NAN, 0., 0.], OrderOptions::default())
            .is_err()
    );
    c.move_to(UnitId(10), [10., 20., 30.], OrderOptions::default())
        .unwrap();
    assert_eq!(CHEATS.load(Ordering::SeqCst), 0);
}
#[test]
fn instances_survive_thread_switch_and_isolate_panics() {
    let runtime = Runtime::<Mock>::new();
    let t = table();
    assert_ne!(unsafe { runtime.init(AiId(99), &t) }, 0);
    assert_eq!(unsafe { runtime.init(AiId(1), &t) }, 0);
    assert_eq!(unsafe { runtime.init(AiId(2), &t) }, 0);
    assert_ne!(unsafe { runtime.init(AiId(1), &t) }, 0);
    let update = |id, frame| {
        let e = sys::SUpdateEvent { frame };
        unsafe {
            runtime.handle(
                AiId(id),
                sys::EVENT_UPDATE as i32,
                (&e as *const sys::SUpdateEvent).cast(),
            )
        }
    };
    std::thread::scope(|scope| {
        scope
            .spawn(|| {
                assert_eq!(update(1, 1), 0);
                assert_eq!(update(2, 1), 0);
            })
            .join()
            .unwrap();
    });
    assert_ne!(update(1, 999), 0);
    assert_ne!(update(1, 2), 0);
    assert_eq!(update(2, 2), 0);
    assert_eq!(*HANDLED.lock().unwrap(), [(1, 1), (2, 1), (2, 2)]);
    assert_eq!(runtime.release(AiId(1)), 0);
    assert_eq!(runtime.release(AiId(1)), 0);
    assert_ne!(update(1, 3), 0);
    assert_eq!(runtime.release(AiId(2)), 0);
}
#[test]
fn decoding_rejects_invalid_nested_payloads_and_tolerates_future_topics() {
    assert!(unsafe { Event::decode(sys::EVENT_UPDATE as i32, std::ptr::null()) }.is_err());
    assert!(matches!(
        unsafe { Event::decode(999, std::ptr::null()) }.unwrap(),
        Event::Unknown { topic: 999 }
    ));
    let e = sys::SPlayerCommandEvent {
        unitIds: std::ptr::null_mut(),
        unitIds_size: -1,
        ..Default::default()
    };
    assert!(
        unsafe {
            Event::decode(
                sys::EVENT_PLAYER_COMMAND as i32,
                (&e as *const sys::SPlayerCommandEvent).cast(),
            )
        }
        .is_err()
    );
    let e = sys::SUnitDamagedEvent {
        unit: 10,
        attacker: -1,
        dir_posF3: std::ptr::null_mut(),
        ..Default::default()
    };
    assert!(
        unsafe {
            Event::decode(
                sys::EVENT_UNIT_DAMAGED as i32,
                (&e as *const sys::SUnitDamagedEvent).cast(),
            )
        }
        .is_err()
    );
}

unsafe extern "system" fn features(_: i32, ids: *mut i32, capacity: i32) -> i32 {
    if ids.is_null() {
        assert_eq!(capacity, i32::MAX);
        return 1;
    }
    assert_eq!(capacity, 1);
    unsafe { ids.write(7) };
    1
}
unsafe extern "system" fn feature_definition(_: i32, id: i32) -> i32 {
    assert_eq!(id, 7);
    3
}
unsafe extern "system" fn feature_position(_: i32, id: i32, position: *mut f32) {
    assert_eq!(id, 7);
    unsafe {
        position.write(10.);
        position.add(1).write(20.);
        position.add(2).write(30.)
    };
}
unsafe extern "system" fn feature_name(_: i32, id: i32) -> *const std::ffi::c_char {
    assert_eq!(id, 3);
    c"asteroid".as_ptr()
}
#[test]
fn features_use_normal_callbacks_and_copy_transient_data() {
    let t = sys::SSkirmishAICallback {
        getFeatures: Some(features),
        Feature_getDef: Some(feature_definition),
        Feature_getPosition: Some(feature_position),
        FeatureDef_getName: Some(feature_name),
        ..table()
    };
    let c = Context::new(AiId(1), &t);
    let result = c.visible_features().unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, FeatureId(7));
    assert_eq!(result[0].name, "asteroid");
    assert_eq!(result[0].position, [10., 20., 30.]);
    assert_eq!(CHEATS.load(Ordering::SeqCst), 0);
}
struct DropPanics;
impl Drop for DropPanics {
    fn drop(&mut self) {
        panic!("drop panic")
    }
}
impl SkirmishAi for DropPanics {
    fn new(_: &Context<'_>) -> Result<Self> {
        Ok(Self)
    }
    fn handle(&mut self, _: &Context<'_>, _: Event<'_>) -> Result<()> {
        Ok(())
    }
}
#[test]
fn destructor_panics_do_not_poison_the_registry() {
    let runtime = Runtime::<DropPanics>::new();
    let t = table();
    assert_eq!(unsafe { runtime.init(AiId(1), &t) }, 0);
    assert_ne!(runtime.release(AiId(1)), 0);
    assert_eq!(unsafe { runtime.init(AiId(2), &t) }, 0);
    assert_ne!(runtime.release(AiId(2)), 0);
}
