/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use spring::cus::{host::CusHost, *};
use spring_addons::reexports::{self as spring, All, Spring, UnitId};
use spring_addons::{AddonContext, Gadget, GadgetHandler};
use std::sync::atomic::{AtomicU32, Ordering};

#[cfg(not(target_arch = "wasm32"))]
static FRAME: AtomicU32 = AtomicU32::new(42);
static CREATED: AtomicU32 = AtomicU32::new(0);
static DROPPED: AtomicU32 = AtomicU32::new(0);
static QUEUED: AtomicU32 = AtomicU32::new(0);
#[cfg(not(target_arch = "wasm32"))]
static OPERATIONS: AtomicU32 = AtomicU32::new(0);
#[cfg(not(target_arch = "wasm32"))]
static ATTACHED: AtomicU32 = AtomicU32::new(0);
#[cfg(not(target_arch = "wasm32"))]
static SAVED_CONTEXT: std::sync::Mutex<Option<UnitCtx>> = std::sync::Mutex::new(None);

pub type Scripts = CusHost<Script, (), All>;
#[derive(Default)]
pub struct Global;
pub struct Script;
impl Drop for Script {
    fn drop(&mut self) {
        DROPPED.fetch_add(1, Ordering::Relaxed);
    }
}
async fn wake(ctx: UnitCtx) {
    ctx.sleep_frames(1).await;
    ctx.show(Piece(4));
}
impl UnitScript for Script {
    fn create(&mut self, ctx: &UnitCtx) {
        ctx.with_spring(|spring| {
            assert_eq!(spring.game().get_game_frame()?.low16 as u64, ctx.frame());
            Ok(())
        })
        .unwrap();
        #[cfg(not(target_arch = "wasm32"))]
        {
            let worker = ctx.clone();
            assert_eq!(
                std::thread::spawn(move || worker
                    .with_spring(|spring| spring.game().get_game_frame())
                    .unwrap_err()
                    .code)
                .join()
                .unwrap(),
                spring::ErrorCode::InvalidState as i32
            );
            *SAVED_CONTEXT.lock().unwrap() = Some(ctx.clone());
        }
        CREATED.fetch_add(1, Ordering::Relaxed);
        ctx.move_now(Piece(3), Axis::X, 12.0);
        ctx.spawn(TaskDefinition::new("wake", wake));
    }
    fn query_weapon(&mut self, _: &UnitCtx, _: WeaponId) -> Piece {
        Piece(3)
    }
    fn call_named(
        &mut self,
        _: &UnitCtx,
        name: &str,
        args: &[f32],
        returns: &mut [f32],
    ) -> Option<usize> {
        if name != "sum" {
            return None;
        }
        let queued_before = QUEUED.load(Ordering::Relaxed);
        with_addon_context(|ctx| {
            ctx.commands().run(|_| {
                QUEUED.fetch_add(1, Ordering::Relaxed);
            })
        });
        assert_eq!(QUEUED.load(Ordering::Relaxed), queued_before);
        returns[0] = args.iter().sum();
        Some(1)
    }
}
struct Probe;
impl Gadget<Global, All> for Probe {
    fn name(&self) -> &'static str {
        "CUS probe"
    }
    fn game_frame(&self, _: &AddonContext<'_, Global, All>, frame: i32) {
        with_cus_module(|host, spring| {
            if !host.is_attached(UnitId(7)) {
                host.attach(
                    spring,
                    UnitId(7),
                    ScriptCapabilities(
                        ScriptCapabilities::CREATE | ScriptCapabilities::QUERY_WEAPON,
                    ),
                    |_| Script,
                )?;
            }
            host.game_frame(frame);
            Ok(())
        })
        .unwrap();
    }
}
fn setup(handler: &mut GadgetHandler<Global, All>, _: &Spring<'_, All>) {
    handler.add(Box::new(Probe));
}
spring_addons::export_rules_gadgets! {
    state: Global, setup: setup, scratch: 4096, environment: All, cus: Scripts,
}

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use super::spring::__sys as sys;
    use super::*;
    std::thread_local! { static HOST: std::cell::Cell<(usize,usize)> = const { std::cell::Cell::new((0,0)) }; }
    unsafe extern "C" fn frame(
        _: *const sys::GetGameFrameQuery,
        out: *mut sys::GetGameFrameResult,
    ) {
        unsafe {
            (*out).low16 = FRAME.load(Ordering::Relaxed);
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
        assert_eq!(query.unitID, 7);
        match query.operation {
            5 => {
                assert_eq!((query.piece, query.axis, query.first), (3, 0, 12.0));
            }
            8 => {
                assert_eq!(query.piece, 4);
            }
            other => panic!("unexpected CUS operation {other}"),
        }
        OPERATIONS.fetch_add(1, Ordering::Relaxed);
        unsafe {
            (*out).completed = 1;
        }
    }
    unsafe extern "C" fn attach(query: *const sys::CusAttachQuery, out: *mut sys::CusAttachResult) {
        let query = unsafe { &*query };
        ATTACHED.store(query.instanceID, Ordering::Relaxed);
        HOST.with(|host| {
            let (interface, data) = host.get();
            let mut values = [0; 8];
            let mut result = sys::CusInvokeResult {
                intValues: values.as_mut_ptr(),
                intCapacity: 8,
                ..Default::default()
            };
            let mut call = sys::CusInvokeQuery {
                instanceID: query.instanceID,
                call: UnitScriptCall::Create as u32,
                ..Default::default()
            };
            unsafe {
                RustCUSInvoke(interface as *const _, data as *mut _, &call, &mut result);
            }
            assert!(result.error.is_null());
            assert_eq!(result.handled, 1);
            if CREATED.load(Ordering::Relaxed) > 1 {
                let weapon = [0];
                call.call = UnitScriptCall::QueryWeapon as u32;
                call.integerArguments = weapon.as_ptr();
                call.integerCount = 1;
                unsafe {
                    RustCUSInvoke(interface as *const _, data as *mut _, &call, &mut result);
                }
                assert!(result.error.is_null());
                assert_eq!((result.handled, result.intValue), (1, 3));
            }
        });
        unsafe {
            (*out).attached = 1;
        }
    }
    #[test]
    fn native_cus_reenters_during_attach_and_ticks_after_loading_thread_handoff() {
        let game = sys::GameApi {
            GetGameFrame: Some(frame),
            ..Default::default()
        };
        let pieces = sys::UnitsPiecesApi {
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
            unitsPieces: &pieces,
            cus: &cus,
            ..Default::default()
        });
        let pointer = (&*interface as *const sys::NativeInterface) as usize;
        let data = std::thread::spawn(move || {
            let [major, minor, patch] = spring::NATIVE_API_VERSION;
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
        HOST.with(|host| host.set((pointer, data)));
        for frame in [42, 43] {
            FRAME.store(frame, Ordering::Relaxed);
            let mut result = sys::GameFrameResult::default();
            unsafe {
                GameFrame(
                    &*interface,
                    data as *mut _,
                    &sys::GameFrameQuery {
                        gameFrame: frame as i32,
                    },
                    &mut result,
                );
            }
            assert!(result.error.is_null());
        }
        assert_eq!(CREATED.load(Ordering::Relaxed), 1);
        assert_eq!(OPERATIONS.load(Ordering::Relaxed), 2);
        let instance = ATTACHED.load(Ordering::Relaxed);
        let arguments = [2.0, 4.0];
        let mut values = [0.0; 2];
        let query = sys::CusNamedQuery {
            instanceID: instance,
            functionName: c"sum".as_ptr(),
            arguments: arguments.as_ptr(),
            argumentCount: 2,
            returnValues: values.as_mut_ptr(),
            returnCapacity: 2,
        };
        let mut result = sys::CusNamedResult::default();
        unsafe {
            RustCUSCallNamed(&*interface, data as *mut _, &query, &mut result);
        }
        assert!(result.error.is_null());
        assert_eq!(
            (
                result.handled,
                result.functionFound,
                result.returnCount,
                values[0]
            ),
            (1, 1, 1, 6.0)
        );
        assert_eq!(QUEUED.load(Ordering::Relaxed), 1);
        with_cus_module(|host, spring| {
            assert_eq!(
                host.call(spring, UnitId(7), "sum", &[2.0, 4.0], &mut [0.0; 1])?,
                Some(1)
            );
            Ok(())
        })
        .unwrap();
        let mut detached = sys::CusDetachResult::default();
        let previous_context = SAVED_CONTEXT.lock().unwrap().take().unwrap();
        unsafe {
            RustCUSDetach(
                &*interface,
                data as *mut _,
                &sys::CusDetachQuery {
                    instanceID: instance,
                },
                &mut detached,
            );
        }
        assert!(detached.error.is_null());
        assert_eq!(DROPPED.load(Ordering::Relaxed), 1);
        assert!(!previous_context.is_alive());
        with_cus_module(|_, _| {
            assert_eq!(
                previous_context
                    .with_spring(|spring| spring.game().get_game_frame())
                    .unwrap_err()
                    .code,
                spring::ErrorCode::InvalidState as i32
            );
            Ok(())
        })
        .unwrap();
        FRAME.store(44, Ordering::Relaxed);
        let mut frame = sys::GameFrameResult::default();
        unsafe {
            GameFrame(
                &*interface,
                data as *mut _,
                &sys::GameFrameQuery { gameFrame: 44 },
                &mut frame,
            );
        }
        assert!(frame.error.is_null());
        assert_ne!(ATTACHED.load(Ordering::Relaxed), instance);
        let mut stale = sys::CusNamedResult::default();
        unsafe {
            RustCUSCallNamed(&*interface, data as *mut _, &query, &mut stale);
        }
        assert!(stale.error.is_null());
        assert_eq!(
            (stale.handled, stale.functionFound, stale.returnCount),
            (0, 0, 0)
        );
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
        assert_eq!(DROPPED.load(Ordering::Relaxed), 2);
    }
}
