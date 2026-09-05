#[macro_export]
macro_rules! register_widgets {
    ($handler:expr, [ $($widget:expr),* $(,)? ]) => {
        $(
            $handler.add(::alloc::boxed::Box::new($widget));
        )*
    };
}

#[macro_export]
macro_rules! register_gadgets {
    ($handler:expr, [ $($gadget:expr),* $(,)? ]) => {
        $(
            $handler.add(::alloc::boxed::Box::new($gadget));
        )*
    };
}

#[macro_export]
macro_rules! export_ui_widgets {
    (
        state: $state_type:ty,
        setup: $setup_fn:path,
        scratch: $scratch_size:expr $(,)?
    ) => {
        $crate::export_ui_widgets! {
            state: $state_type,
            setup: $setup_fn,
            scratch: $scratch_size,
            callbacks: None,
        }
    };
    (
        state: $state_type:ty,
        setup: $setup_fn:path,
        scratch: $scratch_size:expr,
        callbacks: None $(,)?
    ) => {
        $crate::reexports::export_callin_scratch!($scratch_size);
        $crate::reexports::export_environment_mask!($crate::reexports::ui::ENVIRONMENT_MASK);
        $crate::reexports::export_callback_dispatch!(__spring_addon_callback_dispatch);

        fn __default_ui_callback(_global: Option<&mut $state_type>, _id: u32, _data: u32) -> bool {
            false
        }

        $crate::__impl_ui_exports!($state_type, $setup_fn, __default_ui_callback);
    };
    (
        state: $state_type:ty,
        setup: $setup_fn:path,
        scratch: $scratch_size:expr,
        callbacks: $callbacks_fn:path $(,)?
    ) => {
        $crate::reexports::export_callin_scratch!($scratch_size);
        $crate::reexports::export_environment_mask!($crate::reexports::ui::ENVIRONMENT_MASK);
        $crate::reexports::export_callback_dispatch!(__spring_addon_callback_dispatch);

        $crate::__impl_ui_exports!($state_type, $setup_fn, $callbacks_fn);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __impl_ui_exports {
    ($state_type:ty, $setup_fn:path, $callbacks_fn:path) => {
        thread_local! {
            static HANDLER: ::core::cell::RefCell<Option<$crate::ui::WidgetHandler<$state_type>>> = const {
                ::core::cell::RefCell::new(None)
            };
            static PENDING_CALLBACKS: ::core::cell::RefCell<::alloc::vec::Vec<(u32, u32)>> = const {
                ::core::cell::RefCell::new(::alloc::vec::Vec::new())
            };
            static CALLBACK_REGISTRY: ::core::cell::RefCell<$crate::ui::UiCallbackRegistry<$state_type>> = const {
                ::core::cell::RefCell::new($crate::ui::UiCallbackRegistry::new())
            };
        }

        #[allow(dead_code)]
        pub fn register_callback(
            f: impl FnMut(&mut $state_type) + 'static,
        ) -> $crate::reexports::callback::RetainedCallback {
            CALLBACK_REGISTRY.with(|reg| reg.borrow_mut().register(f))
        }

        fn __with_widget_handler<R>(
            f: impl FnOnce(&mut $crate::ui::WidgetHandler<$state_type>) -> R,
        ) -> R {
            HANDLER.with(|cell| {
                let mut borrow = cell.borrow_mut();
                if borrow.is_none() {
                    let mut handler = $crate::ui::WidgetHandler::new(<$state_type>::default());
                    $setup_fn(&mut handler);
                    handler.init();
                    *borrow = Some(handler);
                }
                let handler = borrow.as_mut().unwrap();
                let result = f(handler);
                loop {
                    let pending = PENDING_CALLBACKS.with_borrow_mut(core::mem::take);
                    if pending.is_empty() {
                        break;
                    }
                    for (id, data) in pending {
                        let handled = CALLBACK_REGISTRY.with(|reg| {
                            reg.borrow_mut().dispatch(&mut handler.global, id, data)
                        });
                        if !handled {
                            $callbacks_fn(Some(&mut handler.global), id, data);
                        }
                    }
                }
                result
            })
        }

        fn __try_with_widget_handler<R>(
            f: impl FnOnce(&mut $crate::ui::WidgetHandler<$state_type>) -> R,
        ) -> Option<R> {
            HANDLER.with(|cell| {
                let mut borrow = cell.try_borrow_mut().ok()?;
                if borrow.is_none() {
                    let mut handler = $crate::ui::WidgetHandler::new(<$state_type>::default());
                    $setup_fn(&mut handler);
                    handler.init();
                    *borrow = Some(handler);
                }
                let handler = borrow.as_mut().unwrap();
                let result = f(handler);
                loop {
                    let pending = PENDING_CALLBACKS.with_borrow_mut(core::mem::take);
                    if pending.is_empty() {
                        break;
                    }
                    for (id, data) in pending {
                        let handled = CALLBACK_REGISTRY.with(|reg| {
                            reg.borrow_mut().dispatch(&mut handler.global, id, data)
                        });
                        if !handled {
                            $callbacks_fn(Some(&mut handler.global), id, data);
                        }
                    }
                }
                Some(result)
            })
        }

        #[allow(dead_code)]
        fn __spring_addon_callback_dispatch(callback_id: u32, user_data: u32) {
            let handled = __try_with_widget_handler(|handler| {
                let handled = CALLBACK_REGISTRY.with(|reg| {
                    reg.borrow_mut().dispatch(&mut handler.global, callback_id, user_data)
                });
                if handled {
                    true
                } else {
                    $callbacks_fn(Some(&mut handler.global), callback_id, user_data)
                }
            });
            if handled != Some(true) {
                if !$callbacks_fn(None, callback_id, user_data) {
                    PENDING_CALLBACKS.with_borrow_mut(|pending| {
                        pending.push((callback_id, user_data));
                    });
                }
            }
        }

        pub fn with_widget_handler<R>(
            f: impl FnOnce(&mut $crate::ui::WidgetHandler<$state_type>) -> R,
        ) -> R {
            __with_widget_handler(f)
        }

        pub fn with_global<R>(
            f: impl FnOnce(&mut $state_type) -> R,
        ) -> R {
            __with_widget_handler(|handler| f(&mut handler.global))
        }

        fn __spring_addon_update(dt: f32) {
            __with_widget_handler(|handler| handler.update(dt));
        }
        $crate::reexports::export_update!(__spring_addon_update);

        fn __spring_addon_recv_from_synced(msg: &[u8]) {
            __with_widget_handler(|handler| handler.recv_from_synced(msg));
        }
        $crate::reexports::export_recv_from_synced!(__spring_addon_recv_from_synced);

        fn __spring_addon_mouse_move(x: i32, y: i32, dx: i32, dy: i32, button: i32) -> bool {
            __with_widget_handler(|handler| handler.mouse_move(x, y, dx, dy, button))
        }
        $crate::reexports::export_mouse_move!(__spring_addon_mouse_move);

        fn __spring_addon_mouse_press(x: i32, y: i32, button: i32) -> bool {
            __with_widget_handler(|handler| handler.mouse_press(x, y, button))
        }
        $crate::reexports::export_mouse_press!(__spring_addon_mouse_press);

        fn __spring_addon_mouse_release(x: i32, y: i32, button: i32) {
            __with_widget_handler(|handler| handler.mouse_release(x, y, button));
        }
        $crate::reexports::export_mouse_release!(__spring_addon_mouse_release);

        fn __spring_addon_key_press<'a>(
            key_code: i32,
            alt: bool,
            ctrl: bool,
            meta: bool,
            shift: bool,
            is_repeat: bool,
            label: &[u8],
            utf32_char: i32,
            scan_code: i32,
            _action_list: impl Iterator<Item = (&'a [u8], &'a [u8], &'a [u8])>,
        ) -> bool {
            let event = $crate::event::KeyEvent {
                key_code,
                alt,
                ctrl,
                meta,
                shift,
                is_repeat,
                label,
                utf32_char,
                scan_code,
            };
            __with_widget_handler(|handler| handler.key_press(&event))
        }
        $crate::reexports::export_key_press!(__spring_addon_key_press);

        fn __spring_addon_key_release<'a>(
            key_code: i32,
            alt: bool,
            ctrl: bool,
            meta: bool,
            shift: bool,
            label: &[u8],
            utf32_char: i32,
            scan_code: i32,
            _action_list: impl Iterator<Item = (&'a [u8], &'a [u8], &'a [u8])>,
        ) -> bool {
            let event = $crate::event::KeyEvent {
                key_code,
                alt,
                ctrl,
                meta,
                shift,
                is_repeat: false,
                label,
                utf32_char,
                scan_code,
            };
            __with_widget_handler(|handler| handler.key_release(&event))
        }
        $crate::reexports::export_key_release!(__spring_addon_key_release);

        fn __spring_addon_draw_world() {
            __with_widget_handler(|handler| handler.draw_world());
        }
        $crate::reexports::export_draw_world!(__spring_addon_draw_world);

        fn __spring_addon_draw_screen(w: i32, h: i32) {
            __with_widget_handler(|handler| handler.draw_screen(w, h));
        }
        $crate::reexports::export_draw_screen!(__spring_addon_draw_screen);
    };
}

#[macro_export]
macro_rules! export_rules_gadgets {
    (
        state: $state_type:ty,
        setup: $setup_fn:path,
        scratch: $scratch_size:expr $(,)?
    ) => {
        $crate::reexports::export_callin_scratch!($scratch_size);
        $crate::reexports::export_environment_mask!(
            $crate::reexports::rules_synced::ENVIRONMENT_MASK
        );

        $crate::__impl_rules_exports!($state_type, $setup_fn);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __impl_rules_exports {
    ($state_type:ty, $setup_fn:path) => {
        thread_local! {
            static HANDLER: ::core::cell::RefCell<Option<$crate::rules::GadgetHandler<$state_type>>> = const {
                ::core::cell::RefCell::new(None)
            };
            static PENDING: ::core::cell::RefCell<::alloc::vec::Vec<$crate::rules::PendingRulesEvent>> = const {
                ::core::cell::RefCell::new(::alloc::vec::Vec::new())
            };
        }

        fn __with_gadget_handler<R>(
            f: impl FnOnce(&mut $crate::rules::GadgetHandler<$state_type>) -> R,
        ) -> R {
            HANDLER.with(|cell| {
                let mut borrow = cell.borrow_mut();
                if borrow.is_none() {
                    let mut handler = $crate::rules::GadgetHandler::new(<$state_type>::default());
                    $setup_fn(&mut handler);
                    handler.init();
                    *borrow = Some(handler);
                }
                let handler = borrow.as_mut().unwrap();
                let result = f(handler);
                loop {
                    let pending = PENDING.with_borrow_mut(core::mem::take);
                    if pending.is_empty() {
                        break;
                    }
                    for event in pending {
                        handler.dispatch_pending_event(event);
                    }
                }
                result
            })
        }

        fn __try_with_gadget_handler<R>(
            f: impl FnOnce(&mut $crate::rules::GadgetHandler<$state_type>) -> R,
        ) -> Option<R> {
            HANDLER.with(|cell| {
                let mut borrow = cell.try_borrow_mut().ok()?;
                if borrow.is_none() {
                    let mut handler = $crate::rules::GadgetHandler::new(<$state_type>::default());
                    $setup_fn(&mut handler);
                    handler.init();
                    *borrow = Some(handler);
                }
                let handler = borrow.as_mut().unwrap();
                let result = f(handler);
                loop {
                    let pending = PENDING.with_borrow_mut(core::mem::take);
                    if pending.is_empty() {
                        break;
                    }
                    for event in pending {
                        handler.dispatch_pending_event(event);
                    }
                }
                Some(result)
            })
        }

        pub fn with_gadget_handler<R>(
            f: impl FnOnce(&mut $crate::rules::GadgetHandler<$state_type>) -> R,
        ) -> R {
            __with_gadget_handler(f)
        }

        pub fn with_global<R>(
            f: impl FnOnce(&mut $state_type) -> R,
        ) -> R {
            __with_gadget_handler(|handler| f(&mut handler.global))
        }

        fn __spring_addon_game_frame(frame: i32) {
            __with_gadget_handler(|handler| handler.game_frame(frame));
        }
        $crate::reexports::export_game_frame!(__spring_addon_game_frame);

        fn __spring_addon_handle_lua_msg(p: i32, s: i32, m: i32, data: &[u8]) {
            if __try_with_gadget_handler(|handler| handler.handle_lua_msg(p, s, m, data)).is_none() {
                PENDING.with_borrow_mut(|pending| {
                    pending.push($crate::rules::PendingRulesEvent::LuaMsg {
                        player_id: p,
                        script: s,
                        mode: m,
                        data: data.to_vec(),
                    });
                });
            }
        }
        $crate::reexports::export_handle_lua_msg!(__spring_addon_handle_lua_msg);

        fn __spring_addon_unit_created(
            u: $crate::reexports::UnitId,
            d: $crate::reexports::DefId,
            t: $crate::reexports::TeamId,
            b: $crate::reexports::UnitId,
        ) {
            let unit = u.into();
            let def = d.into();
            let team = t.into();
            let builder = b.into();
            if __try_with_gadget_handler(|handler| {
                handler.unit_created(unit, def, team, builder);
            }).is_none() {
                PENDING.with_borrow_mut(|pending| {
                    pending.push($crate::rules::PendingRulesEvent::UnitCreated {
                        unit,
                        def,
                        team,
                        builder,
                    });
                });
            }
        }
        $crate::reexports::export_unit_created!(__spring_addon_unit_created);

        fn __spring_addon_unit_destroyed(
            u: i32,
            d: i32,
            t: i32,
            a: i32,
            ad: i32,
            at: i32,
            w: i32,
        ) {
            let event = $crate::rules::UnitDestroyedEvent {
                unit_id: u,
                unit_def_id: d,
                unit_team: t,
                attacker_id: a,
                attacker_def_id: ad,
                attacker_team: at,
                weapon_def_id: w,
            };
            if __try_with_gadget_handler(|handler| {
                handler.unit_destroyed(&event);
            }).is_none() {
                PENDING.with_borrow_mut(|pending| {
                    pending.push($crate::rules::PendingRulesEvent::UnitDestroyed(event));
                });
            }
        }
        $crate::reexports::export_unit_destroyed!(__spring_addon_unit_destroyed);

        fn __spring_addon_unit_idle(
            u: i32,
            d: i32,
            t: i32,
        ) {
            let _ = __try_with_gadget_handler(|handler| handler.unit_idle(u, d, t));
        }
        $crate::reexports::export_unit_idle!(__spring_addon_unit_idle);

        fn __spring_addon_projectile_created(p: i32, o: i32, w: i32) {
            if __try_with_gadget_handler(|handler| handler.projectile_created(p, o, w)).is_none() {
                PENDING.with_borrow_mut(|pending| {
                    pending.push($crate::rules::PendingRulesEvent::ProjectileCreated {
                        projectile_id: p,
                        owner_id: o,
                        weapon_def_id: w,
                    });
                });
            }
        }
        $crate::reexports::export_projectile_created!(__spring_addon_projectile_created);

        fn __spring_addon_unit_pre_damaged(
            u: i32,
            ud: i32,
            ut: i32,
            d: f32,
            p: bool,
            w: i32,
            pr: i32,
            a: i32,
            ad: i32,
            at: i32,
        ) -> $crate::reexports::DamageResult {
            let default_result = $crate::reexports::DamageResult {
                new_damage: d,
                impulse_mult: 1.0,
            };
            __try_with_gadget_handler(|handler| {
                handler.unit_pre_damaged(u, ud, ut, d, p, w, pr, a, ad, at)
            }).unwrap_or(default_result)
        }
        $crate::reexports::export_unit_pre_damaged!(__spring_addon_unit_pre_damaged);

        fn __spring_addon_explosion(w: i32, x: f32, y: f32, z: f32, o: i32, p: i32) -> bool {
            let handled = __try_with_gadget_handler(|handler| handler.explosion(w, (x, y, z), o, p));
            if let Some(handled) = handled {
                handled
            } else {
                PENDING.with_borrow_mut(|pending| {
                    pending.push($crate::rules::PendingRulesEvent::Explosion {
                        weapon_def_id: w,
                        pos: (x, y, z),
                        owner_id: o,
                        projectile_id: p,
                    });
                });
                false
            }
        }
        $crate::reexports::export_explosion!(__spring_addon_explosion);
    };
}
