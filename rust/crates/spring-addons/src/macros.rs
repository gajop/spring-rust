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
macro_rules! register_unsynced_addons {
    ($handler:expr, [ $($addon:expr),* $(,)? ]) => {
        $(
            $handler.add(::alloc::boxed::Box::new($addon));
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

        fn __default_ui_callback(_global: &$state_type, _id: u32, _data: u32) -> bool {
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
            static HANDLER: ::core::cell::OnceCell<$crate::ui::WidgetHandler<$state_type>> = const {
                ::core::cell::OnceCell::new()
            };
            static CALLBACK_REGISTRY: $crate::ui::UiCallbackRegistry<$state_type> = const {
                $crate::ui::UiCallbackRegistry::new()
            };
        }

        fn __with_widget_handler<R>(
            f: impl FnOnce(&$crate::ui::WidgetHandler<$state_type>) -> R,
        ) -> R {
            HANDLER.with(|cell| {
                if cell.get().is_none() {
                    let mut handler = $crate::ui::WidgetHandler::new(<$state_type>::default());
                    $setup_fn(&mut handler);
                    assert!(cell.set(handler).is_ok(), "spring-addons widget handler initialized twice");
                    cell.get().unwrap().init();
                }
                f(cell.get().unwrap())
            })
        }

        #[allow(dead_code)]
        pub fn register_callback(
            f: impl FnMut(&$state_type) + 'static,
        ) -> $crate::reexports::callback::RetainedCallback {
            CALLBACK_REGISTRY.with(|registry| registry.register(f))
        }

        #[allow(dead_code)]
        pub fn unregister_callback(callback: $crate::reexports::callback::RetainedCallback) {
            CALLBACK_REGISTRY.with(|registry| registry.unregister(callback));
        }

        #[allow(dead_code)]
        fn __spring_addon_callback_dispatch(callback_id: u32, user_data: u32) {
            __with_widget_handler(|handler| {
                handler.with_context(|ctx| {
                    let handled = CALLBACK_REGISTRY.with(|registry| {
                        registry.dispatch(ctx.global(), callback_id, user_data)
                    });
                    if !handled && !$callbacks_fn(ctx.global(), callback_id, user_data) {
                        $crate::ui::warn_unhandled_callback(callback_id);
                    }
                });
            });
        }

        #[allow(dead_code)]
        pub fn with_widget_handler<R>(
            f: impl FnOnce(&$crate::ui::WidgetHandler<$state_type>) -> R,
        ) -> R {
            __with_widget_handler(f)
        }

        #[allow(dead_code)]
        pub fn with_global<R>(f: impl FnOnce(&$state_type) -> R) -> R {
            __with_widget_handler(|handler| handler.with_context(|ctx| f(ctx.global())))
        }

        #[allow(dead_code)]
        pub fn with_addon_context<R>(
            f: impl FnOnce(&$crate::AddonContext<'_, $state_type>) -> R,
        ) -> R {
            __with_widget_handler(|handler| handler.with_context(f))
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

        fn __spring_addon_draw_world_pre_unit(_unused: u8) {
            __with_widget_handler(|handler| handler.draw_world_pre_unit());
        }
        $crate::reexports::export_draw_world_pre_unit!(__spring_addon_draw_world_pre_unit);

        fn __spring_addon_draw_world_refraction(_unused: u8) {
            __with_widget_handler(|handler| handler.draw_world_refraction());
        }
        $crate::reexports::export_draw_world_refraction!(__spring_addon_draw_world_refraction);

        fn __spring_addon_draw_screen_effects(w: i32, h: i32) {
            __with_widget_handler(|handler| handler.draw_screen_effects(w, h));
        }
        $crate::reexports::export_draw_screen_effects!(__spring_addon_draw_screen_effects);

        fn __spring_addon_draw_unit(unit_id: i32, draw_mode: i32) -> bool {
            __with_widget_handler(|handler| handler.draw_unit(unit_id, draw_mode))
        }
        $crate::reexports::export_draw_unit!(__spring_addon_draw_unit);

        fn __spring_addon_widget_game_over(winning_ally_teams: &[u8]) {
            __with_widget_handler(|handler| handler.game_over(winning_ally_teams));
        }
        $crate::reexports::export_game_over!(__spring_addon_widget_game_over);

        fn __spring_addon_view_resize(
            screen_size_x: i32,
            screen_size_y: i32,
            screen_pos_x: i32,
            screen_pos_y: i32,
            window_size_x: i32,
            window_size_y: i32,
            window_pos_x: i32,
            window_pos_y: i32,
            window_border_top: i32,
            window_border_left: i32,
            window_border_bottom: i32,
            window_border_right: i32,
            view_size_x: i32,
            view_size_y: i32,
            view_pos_x: i32,
            view_pos_y: i32,
        ) {
            let geometry = $crate::event::ViewGeometry {
                screen_size: (screen_size_x, screen_size_y),
                screen_pos: (screen_pos_x, screen_pos_y),
                window_size: (window_size_x, window_size_y),
                window_pos: (window_pos_x, window_pos_y),
                window_border: (
                    window_border_top,
                    window_border_left,
                    window_border_bottom,
                    window_border_right,
                ),
                view_size: (view_size_x, view_size_y),
                view_pos: (view_pos_x, view_pos_y),
            };
            __with_widget_handler(|handler| handler.view_resize(&geometry));
        }
        $crate::reexports::export_view_resize!(__spring_addon_view_resize);
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
            static HANDLER: ::core::cell::OnceCell<$crate::rules::GadgetHandler<$state_type>> = const {
                ::core::cell::OnceCell::new()
            };
        }

        fn __with_gadget_handler<R>(
            f: impl FnOnce(&$crate::rules::GadgetHandler<$state_type>) -> R,
        ) -> R {
            HANDLER.with(|cell| {
                if cell.get().is_none() {
                    let mut handler = $crate::rules::GadgetHandler::new(<$state_type>::default());
                    $setup_fn(&mut handler);
                    assert!(cell.set(handler).is_ok(), "spring-addons gadget handler initialized twice");
                    cell.get().unwrap().init();
                }
                f(cell.get().unwrap())
            })
        }

        #[allow(dead_code)]
        pub fn with_gadget_handler<R>(
            f: impl FnOnce(&$crate::rules::GadgetHandler<$state_type>) -> R,
        ) -> R {
            __with_gadget_handler(f)
        }

        #[allow(dead_code)]
        pub fn with_global<R>(f: impl FnOnce(&$state_type) -> R) -> R {
            __with_gadget_handler(|handler| handler.with_context(|ctx| f(ctx.global())))
        }

        #[allow(dead_code)]
        pub fn with_addon_context<R>(
            f: impl FnOnce(&$crate::AddonContext<'_, $state_type>) -> R,
        ) -> R {
            __with_gadget_handler(|handler| handler.with_context(f))
        }

        fn __spring_addon_game_frame(frame: i32) {
            __with_gadget_handler(|handler| handler.game_frame(frame));
        }
        $crate::reexports::export_game_frame!(__spring_addon_game_frame);

        fn __spring_addon_handle_lua_msg(p: i32, s: i32, m: i32, data: &[u8]) {
            __with_gadget_handler(|handler| handler.handle_lua_msg(p, s, m, data));
        }
        $crate::reexports::export_handle_lua_msg!(__spring_addon_handle_lua_msg);

        fn __spring_addon_unit_created(
            u: $crate::reexports::UnitId,
            d: $crate::reexports::DefId,
            t: $crate::reexports::TeamId,
            b: $crate::reexports::UnitId,
        ) {
            __with_gadget_handler(|handler| {
                handler.unit_created(u.into(), d.into(), t.into(), b.into());
            });
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
            __with_gadget_handler(|handler| handler.unit_destroyed(&event));
        }
        $crate::reexports::export_unit_destroyed!(__spring_addon_unit_destroyed);

        fn __spring_addon_unit_idle(u: i32, d: i32, t: i32) {
            __with_gadget_handler(|handler| handler.unit_idle(u, d, t));
        }
        $crate::reexports::export_unit_idle!(__spring_addon_unit_idle);

        fn __spring_addon_projectile_created(p: i32, o: i32, w: i32) {
            __with_gadget_handler(|handler| handler.projectile_created(p, o, w));
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
            __with_gadget_handler(|handler| {
                handler.unit_pre_damaged(u, ud, ut, d, p, w, pr, a, ad, at)
            })
        }
        $crate::reexports::export_unit_pre_damaged!(__spring_addon_unit_pre_damaged);

        fn __spring_addon_explosion(w: i32, x: f32, y: f32, z: f32, o: i32, p: i32) -> bool {
            __with_gadget_handler(|handler| handler.explosion(w, (x, y, z), o, p))
        }
        $crate::reexports::export_explosion!(__spring_addon_explosion);

        fn __spring_addon_projectile_destroyed(p: i32, o: i32, w: i32) {
            __with_gadget_handler(|handler| handler.projectile_destroyed(p, o, w));
        }
        $crate::reexports::export_projectile_destroyed!(__spring_addon_projectile_destroyed);

        fn __spring_addon_game_over(winning_ally_teams: &[u8]) {
            __with_gadget_handler(|handler| handler.game_over(winning_ally_teams));
        }
        $crate::reexports::export_game_over!(__spring_addon_game_over);

        fn __spring_addon_unit_cmd_done(
            unit_id: i32,
            unit_def_id: i32,
            unit_team: i32,
            command_id: i32,
            command_time_out: i32,
            command_page_index: u32,
            command_tag: u32,
            command_options: u8,
            command_params: &[f32],
        ) {
            let event = $crate::event::CommandEvent {
                unit_id,
                unit_def_id,
                unit_team,
                command_id,
                command_time_out,
                command_page_index,
                command_tag,
                command_options,
                command_params,
                player_num: None,
                from_synced: true,
                from_lua: false,
            };
            __with_gadget_handler(|handler| handler.unit_cmd_done(&event));
        }
        $crate::reexports::export_unit_cmd_done!(__spring_addon_unit_cmd_done);

        fn __spring_addon_allow_command(
            unit_id: i32,
            unit_def_id: i32,
            unit_team: i32,
            command_id: i32,
            command_time_out: i32,
            command_page_index: u32,
            command_tag: u32,
            command_options: u8,
            command_params: &[f32],
            player_num: i32,
            from_synced: bool,
            from_lua: bool,
        ) -> bool {
            let event = $crate::event::CommandEvent {
                unit_id,
                unit_def_id,
                unit_team,
                command_id,
                command_time_out,
                command_page_index,
                command_tag,
                command_options,
                command_params,
                player_num: Some(player_num),
                from_synced,
                from_lua,
            };
            __with_gadget_handler(|handler| handler.allow_command(&event))
        }
        $crate::reexports::export_allow_command!(__spring_addon_allow_command);
    };
}

#[macro_export]
macro_rules! export_rules_unsynced_addons {
    (
        state: $state_type:ty,
        setup: $setup_fn:path,
        scratch: $scratch_size:expr $(,) ?
    ) => {
        $crate::export_rules_unsynced_addons! {
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
        callbacks: None $(,) ?
    ) => {
        fn __default_unsynced_callback(_id: u32, _data: u32) -> bool {
            false
        }
        $crate::__impl_unsynced_exports!(
            $state_type,
            $setup_fn,
            $scratch_size,
            __default_unsynced_callback
        );
    };
    (
        state: $state_type:ty,
        setup: $setup_fn:path,
        scratch: $scratch_size:expr,
        callbacks: $callbacks_fn:path $(,) ?
    ) => {
        $crate::__impl_unsynced_exports!($state_type, $setup_fn, $scratch_size, $callbacks_fn);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __impl_unsynced_exports {
    ($state_type:ty, $setup_fn:path, $scratch_size:expr, $callbacks_fn:path) => {
        $crate::reexports::export_callin_scratch!($scratch_size);
        $crate::reexports::export_environment_mask!(
            $crate::reexports::rules_unsynced::ENVIRONMENT_MASK
        );
        $crate::reexports::export_callback_dispatch!(__spring_addon_unsynced_callback_dispatch);

        thread_local! {
            static HANDLER: ::core::cell::OnceCell<$crate::unsynced::UnsyncedHandler<$state_type>> = const {
                ::core::cell::OnceCell::new()
            };
        }

        fn __with_unsynced_handler<R>(
            f: impl FnOnce(&$crate::unsynced::UnsyncedHandler<$state_type>) -> R,
        ) -> R {
            HANDLER.with(|cell| {
                if cell.get().is_none() {
                    let mut handler = $crate::unsynced::UnsyncedHandler::new(<$state_type>::default());
                    $setup_fn(&mut handler);
                    assert!(cell.set(handler).is_ok(), "spring-addons unsynced handler initialized twice");
                    cell.get().unwrap().init();
                }
                f(cell.get().unwrap())
            })
        }

        #[allow(dead_code)]
        pub fn with_unsynced_handler<R>(
            f: impl FnOnce(&$crate::unsynced::UnsyncedHandler<$state_type>) -> R,
        ) -> R {
            __with_unsynced_handler(f)
        }

        #[allow(dead_code)]
        pub fn with_global<R>(f: impl FnOnce(&$state_type) -> R) -> R {
            __with_unsynced_handler(|handler| handler.with_context(|ctx| f(ctx.global())))
        }

        #[allow(dead_code)]
        pub fn with_addon_context<R>(
            f: impl FnOnce(&$crate::AddonContext<'_, $state_type>) -> R,
        ) -> R {
            __with_unsynced_handler(|handler| handler.with_context(f))
        }

        #[allow(dead_code)]
        fn __spring_addon_unsynced_callback_dispatch(callback_id: u32, user_data: u32) {
            let _ = $callbacks_fn(callback_id, user_data);
        }

        fn __spring_addon_unsynced_game_frame(frame: i32) {
            __with_unsynced_handler(|handler| handler.game_frame(frame));
        }
        $crate::reexports::export_game_frame!(__spring_addon_unsynced_game_frame);

        fn __spring_addon_unsynced_draw_world_pre_unit(_unused: u8) {
            __with_unsynced_handler(|handler| handler.draw_world_pre_unit());
        }
        $crate::reexports::export_draw_world_pre_unit!(
            __spring_addon_unsynced_draw_world_pre_unit
        );

        fn __spring_addon_unsynced_view_resize(
            screen_size_x: i32,
            screen_size_y: i32,
            screen_pos_x: i32,
            screen_pos_y: i32,
            window_size_x: i32,
            window_size_y: i32,
            window_pos_x: i32,
            window_pos_y: i32,
            window_border_top: i32,
            window_border_left: i32,
            window_border_bottom: i32,
            window_border_right: i32,
            view_size_x: i32,
            view_size_y: i32,
            view_pos_x: i32,
            view_pos_y: i32,
        ) {
            let geometry = $crate::event::ViewGeometry {
                screen_size: (screen_size_x, screen_size_y),
                screen_pos: (screen_pos_x, screen_pos_y),
                window_size: (window_size_x, window_size_y),
                window_pos: (window_pos_x, window_pos_y),
                window_border: (
                    window_border_top,
                    window_border_left,
                    window_border_bottom,
                    window_border_right,
                ),
                view_size: (view_size_x, view_size_y),
                view_pos: (view_pos_x, view_pos_y),
            };
            __with_unsynced_handler(|handler| handler.view_resize(&geometry));
        }
        $crate::reexports::export_view_resize!(__spring_addon_unsynced_view_resize);
    };
}
