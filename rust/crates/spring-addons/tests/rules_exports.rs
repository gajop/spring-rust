/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use spring_addons::reexports::{All, Spring};
use spring_addons::{AddonContext, Gadget, GadgetHandler};
use std::cell::RefCell;
use std::sync::atomic::{AtomicU32, Ordering};

static SHUTDOWNS: AtomicU32 = AtomicU32::new(0);
static INITS: AtomicU32 = AtomicU32::new(0);

#[derive(Default)]
pub struct State(RefCell<Vec<&'static str>>);
struct Probe;
impl Gadget<State, All> for Probe {
    fn name(&self) -> &'static str {
        "probe"
    }
    fn init(&self, ctx: &AddonContext<'_, State, All>) {
        ctx.global().0.borrow_mut().push("init");
        // Init runs once the module is installed, so it can re-enter it.
        assert_eq!(with_global(|global| global.0.borrow().len()), 1);
        INITS.fetch_add(1, Ordering::Relaxed);
    }
    fn game_frame(&self, ctx: &AddonContext<'_, State, All>, frame: i32) {
        assert_eq!(frame, 42);
        ctx.global().0.borrow_mut().push("frame");
        ctx.delay(|ctx| ctx.global().0.borrow_mut().push("delayed"));
        ctx.commands()
            .queue(|spring| spring.game().get_game_frame())
            .then(|_, frame| {
                assert_eq!(frame.unwrap().low16, 42);
                with_global(|global| global.0.borrow_mut().push("continuation"));
            });
        ctx.commands()
            .run(|_| with_global(|global| global.0.borrow_mut().push("command2")));
    }
    fn unit_created(
        &self,
        ctx: &AddonContext<'_, State, All>,
        unit: i32,
        def: i32,
        team: i32,
        builder: i32,
    ) {
        assert_eq!((unit, def, team, builder), (7, 2, 0, -1));
        ctx.global().0.borrow_mut().push("created");
        ctx.delay(|ctx| ctx.global().0.borrow_mut().push("created delayed"));
        ctx.commands()
            .run(|_| with_global(|global| global.0.borrow_mut().push("created command")));
    }
    fn shutdown(&self, ctx: &AddonContext<'_, State, All>) {
        ctx.global().0.borrow_mut().push("shutdown");
        SHUTDOWNS.fetch_add(1, Ordering::Relaxed);
    }
}

#[cfg(not(target_arch = "wasm32"))]
std::thread_local! { static HOST: std::cell::Cell<(usize, usize)> = const { std::cell::Cell::new((0, 0)) }; }

#[cfg(not(target_arch = "wasm32"))]
unsafe extern "C" fn frame_query(
    _: *const spring_addons::reexports::__sys::GetGameFrameQuery,
    output: *mut spring_addons::reexports::__sys::GetGameFrameResult,
) {
    use spring_addons::reexports::__sys as sys;
    HOST.with(|host| {
        let (interface, data) = host.get();
        let mut result = sys::UnitCreatedResult::default();
        let query = sys::UnitCreatedQuery {
            unitID: 7,
            unitDefID: 2,
            unitTeam: 0,
            builderID: -1,
        };
        unsafe {
            UnitCreated(interface as *const _, data as *mut _, &query, &mut result);
        }
        assert!(result.error.is_null());
    });
    unsafe {
        *output = sys::GetGameFrameResult {
            error: core::ptr::null(),
            low16: 42,
            high16: 0,
        };
    }
}
fn setup(handler: &mut GadgetHandler<State, All>, _: &Spring<'_, All>) {
    handler.add(Box::new(Probe));
}
spring_addons::export_rules_gadgets! {
    state: State, setup: setup, scratch: 4096, environment: All,
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn init_runs_at_load_and_loading_thread_handoff_and_shutdown_use_the_same_handler() {
    use spring_addons::reexports::{__sys as sys, NATIVE_API_VERSION};
    let game = sys::GameApi {
        GetGameFrame: Some(frame_query),
        ..Default::default()
    };
    let interface = Box::new(sys::NativeInterface {
        game: &game,
        ..Default::default()
    });
    let pointer = (&*interface as *const sys::NativeInterface) as usize;
    let data = std::thread::spawn(move || {
        let [major, minor, patch] = NATIVE_API_VERSION;
        let query = sys::InitializeNativeModuleQuery {
            hostVersionMajor: major,
            hostVersionMinor: minor,
            hostVersionPatch: patch,
        };
        let mut result = sys::InitializeNativeModuleResult::default();
        unsafe {
            InitializeNativeModule(pointer as *const _, &query, &mut result);
        }
        assert!(result.error.is_null());
        result.moduleData as usize
    })
    .join()
    .unwrap();
    assert_ne!(data, 0);
    // Init ran when the module loaded, before its first callin.
    assert_eq!(INITS.load(Ordering::Relaxed), 1);
    HOST.with(|host| host.set((pointer, data)));
    let mut result = sys::GameFrameResult::default();
    unsafe {
        GameFrame(
            &*interface,
            data as *mut _,
            &sys::GameFrameQuery { gameFrame: 42 },
            &mut result,
        );
    }
    assert!(result.error.is_null());
    assert_eq!(INITS.load(Ordering::Relaxed), 1);
    with_global(|global| {
        assert_eq!(
            &*global.0.borrow(),
            &[
                "init",
                "frame",
                "delayed",
                "created",
                "created delayed",
                "continuation",
                "command2",
                "created command",
            ]
        )
    });
    let mut result = sys::ShutdownResult::default();
    unsafe {
        Shutdown(
            &*interface,
            data as *mut _,
            &sys::ShutdownQuery::default(),
            &mut result,
        );
    }
    assert!(result.error.is_null());
    assert_eq!(SHUTDOWNS.load(Ordering::Relaxed), 1);
    assert!(HANDLER.with(|slot| slot.invoke(|_, _| Ok(()))).is_err());
}
