/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

//! `CusHostGadget` attaches scripts at init and on unit creation, and wakes them each frame.

use spring::cus::{host::CusHost, host::UnitDefScript, *};
use spring_addons::reexports::{self as spring, All, Spring};
use spring_addons::{CusHostGadget, GadgetHandler};
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Mutex;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::atomic::{AtomicU32, Ordering};

/// Units of this def have scripts; others do not.
const SCRIPTED: i32 = 1;

#[cfg(not(target_arch = "wasm32"))]
static FRAME: AtomicU32 = AtomicU32::new(0);
#[cfg(not(target_arch = "wasm32"))]
static SHOWN: AtomicU32 = AtomicU32::new(0);
#[cfg(not(target_arch = "wasm32"))]
static ATTACHED: Mutex<Vec<(u32, u32)>> = Mutex::new(Vec::new());

pub type Scripts = CusHost<Script, (), All>;
#[derive(Default)]
pub struct Global;
pub struct Script;
async fn wake(ctx: UnitCtx) {
    ctx.sleep_frames(1).await;
    ctx.show(Piece(4));
}
impl UnitScript for Script {
    fn create(&mut self, ctx: &UnitCtx) {
        ctx.spawn(TaskDefinition::new("wake", wake));
    }
}
impl UnitDefScript<(), All> for Script {
    fn capabilities(_: &Spring<'_, All>, def: i32) -> Option<ScriptCapabilities> {
        (def == SCRIPTED).then_some(ScriptCapabilities(ScriptCapabilities::CREATE))
    }
    fn for_def(_: &Spring<'_, All>, _: i32, _: &mut InitCtx<'_>, _: &mut ()) -> Self {
        Script
    }
}
fn setup(handler: &mut GadgetHandler<Global, All>, _: &Spring<'_, All>) {
    handler.add(Box::new(CusHostGadget::new(with_cus_host)));
}
spring_addons::export_rules_gadgets! {
    state: Global, setup: setup, scratch: 4096, environment: All, cus: Scripts,
}

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use super::spring::__sys as sys;
    use super::*;
    std::thread_local! {
        static HOST: std::cell::Cell<(usize, usize)> = const { std::cell::Cell::new((0, 0)) };
        static UNITS: std::cell::RefCell<Vec<i32>> = const { std::cell::RefCell::new(Vec::new()) };
    }
    unsafe extern "C" fn frame(
        _: *const sys::GetGameFrameQuery,
        out: *mut sys::GetGameFrameResult,
    ) {
        unsafe {
            (*out).low16 = FRAME.load(Ordering::Relaxed);
        }
    }
    /// Unit 3 exists before the rules load.
    unsafe extern "C" fn all_units(
        _: *const sys::GetAllUnitsQuery,
        out: *mut sys::GetAllUnitsResult,
    ) {
        UNITS.with(|units| {
            let mut units = units.borrow_mut();
            *units = vec![3];
            unsafe {
                (*out).units = units.as_mut_ptr();
                (*out).count = 1;
            }
        });
    }
    /// Unit 8 has an unscripted def.
    unsafe extern "C" fn def_id(
        query: *const sys::GetUnitDefIDQuery,
        out: *mut sys::GetUnitDefIDResult,
    ) {
        let unit = unsafe { (*query).unitID };
        unsafe {
            (*out).unitDefID = if unit == 8 { 2 } else { SCRIPTED };
        }
    }
    unsafe extern "C" fn root(
        _: *const sys::GetUnitRootPieceQuery,
        out: *mut sys::GetUnitRootPieceResult,
    ) {
        unsafe {
            (*out).rootPiece = 1;
        }
    }
    unsafe extern "C" fn pieces(
        _: *const sys::GetUnitPieceMapQuery,
        _: *mut sys::GetUnitPieceMapResult,
    ) {
    }
    unsafe extern "C" fn operation(
        query: *const sys::CusOperationQuery,
        out: *mut sys::CusOperationResult,
    ) {
        let query = unsafe { &*query };
        assert_eq!((query.unitID, query.operation, query.piece), (7, 8, 4));
        SHOWN.fetch_add(1, Ordering::Relaxed);
        unsafe {
            (*out).completed = 1;
        }
    }
    /// The engine sends `Create` once the module returns; here, at once, when it can.
    unsafe extern "C" fn attach(query: *const sys::CusAttachQuery, out: *mut sys::CusAttachResult) {
        let query = unsafe { &*query };
        ATTACHED
            .lock()
            .unwrap()
            .push((query.unitID as u32, query.instanceID));
        HOST.with(|host| {
            let (interface, data) = host.get();
            if data == 0 {
                return;
            }
            let mut values = [0; 8];
            let mut result = sys::CusInvokeResult {
                intValues: values.as_mut_ptr(),
                intCapacity: 8,
                ..Default::default()
            };
            let call = sys::CusInvokeQuery {
                instanceID: query.instanceID,
                call: UnitScriptCall::Create as u32,
                ..Default::default()
            };
            unsafe {
                RustCUSInvoke(interface as *const _, data as *mut _, &call, &mut result);
            }
            assert!(result.error.is_null());
        });
        unsafe {
            (*out).attached = 1;
        }
    }
    fn game_frame(interface: &sys::NativeInterface, data: usize, frame: u32) {
        FRAME.store(frame, Ordering::Relaxed);
        let mut result = sys::GameFrameResult::default();
        unsafe {
            GameFrame(
                interface,
                data as *mut _,
                &sys::GameFrameQuery {
                    gameFrame: frame as i32,
                },
                &mut result,
            );
        }
        assert!(result.error.is_null());
    }
    fn unit_created(interface: &sys::NativeInterface, data: usize, unit: i32, def: i32) {
        let mut result = sys::UnitCreatedResult::default();
        let query = sys::UnitCreatedQuery {
            unitID: unit,
            unitDefID: def,
            unitTeam: 0,
            builderID: -1,
        };
        unsafe {
            UnitCreated(interface, data as *mut _, &query, &mut result);
        }
        assert!(result.error.is_null());
    }
    #[test]
    fn gadget_attaches_existing_and_new_units_and_wakes_scripts_each_frame() {
        let game = sys::GameApi {
            GetGameFrame: Some(frame),
            ..Default::default()
        };
        let query = sys::UnitsQueryApi {
            GetAllUnits: Some(all_units),
            ..Default::default()
        };
        let info = sys::UnitsInfoApi {
            GetUnitDefID: Some(def_id),
            ..Default::default()
        };
        let unit_pieces = sys::UnitsPiecesApi {
            GetUnitRootPiece: Some(root),
            GetUnitPieceMap: Some(pieces),
            ..Default::default()
        };
        let cus = sys::CusApi {
            Attach: Some(attach),
            Operation: Some(operation),
            ..Default::default()
        };
        let interface = Box::new(sys::NativeInterface {
            game: &game,
            unitsQuery: &query,
            unitsInfo: &info,
            unitsPieces: &unit_pieces,
            cus: &cus,
            ..Default::default()
        });
        let [major, minor, patch] = spring::NATIVE_API_VERSION;
        let mut result = sys::InitializeNativeModuleResult::default();
        unsafe {
            InitializeNativeModule(
                &*interface,
                &sys::InitializeNativeModuleQuery {
                    hostVersionMajor: major,
                    hostVersionMinor: minor,
                    hostVersionPatch: patch,
                },
                &mut result,
            );
        }
        assert!(result.error.is_null());
        let data = result.moduleData as usize;
        // Init, when the module loaded, gave the existing unit its script.
        assert_eq!(
            ATTACHED
                .lock()
                .unwrap()
                .iter()
                .map(|a| a.0)
                .collect::<Vec<_>>(),
            [3]
        );
        HOST.with(|host| host.set((&*interface as *const _ as usize, data)));

        FRAME.store(42, Ordering::Relaxed);
        unit_created(&interface, data, 7, SCRIPTED);
        unit_created(&interface, data, 8, 2);
        assert_eq!(
            ATTACHED
                .lock()
                .unwrap()
                .iter()
                .map(|a| a.0)
                .collect::<Vec<_>>(),
            [3, 7]
        );
        game_frame(&interface, data, 42);
        assert_eq!(SHOWN.load(Ordering::Relaxed), 0);
        // The task slept one frame and wakes in the next GameFrame.
        game_frame(&interface, data, 43);
        assert_eq!(SHOWN.load(Ordering::Relaxed), 1);
        // Created twice is attached once.
        unit_created(&interface, data, 7, SCRIPTED);
        assert_eq!(ATTACHED.lock().unwrap().len(), 2);

        let mut shutdown = sys::ShutdownResult::default();
        unsafe {
            Shutdown(
                &*interface,
                data as *mut _,
                &sys::ShutdownQuery::default(),
                &mut shutdown,
            );
        }
        assert!(shutdown.error.is_null());
    }
}
