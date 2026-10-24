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
#[doc(hidden)]
macro_rules! __addon_state {
    ($state_type:ty, $spring:expr) => {
        Ok::<_, $crate::reexports::ApiError>(<$state_type>::default())
    };
    ($state_type:ty, $spring:expr, $create:path) => {
        $create($spring)
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __addon_module {
    ($state_type:ty, $setup_fn:path, $environment:ty, $handler:ident, $with_handler:ident $(, $cus:ty)? $(; $create:path)?) => {
        struct __SpringAddonModule {
            handler: $crate::$handler<$state_type, $environment>,
            initialized: ::core::cell::Cell<bool>,
            $(cus: $cus,)?
        }
        impl __SpringAddonModule {
            /// Every addon's `init` runs once, when the module loads (`Module::start`); later
            /// calls do nothing. Kept for modules that still call it from their own exports.
            #[allow(dead_code)]
            fn ensure_initialized(&self, spring: &$crate::reexports::Spring<'_, $environment>) {
                if !self.initialized.replace(true) {
                    self.handler.init(spring);
                }
            }
        }
        impl $crate::reexports::module::Module<$environment> for __SpringAddonModule {
            fn initialize(
                spring: &mut $crate::reexports::Spring<'_, $environment>,
            ) -> $crate::reexports::Result<Self> {
                $crate::install_panic_hook();
                let state = $crate::__addon_state!($state_type, spring $(, $create)?)?;
                let mut handler = $crate::$handler::new(state);
                $setup_fn(&mut handler, spring);
                Ok(Self {
                    handler,
                    initialized: ::core::cell::Cell::new(false),
                    $(cus: <$cus>::default(),)?
                })
            }
            fn start(
                &self,
                spring: &mut $crate::reexports::Spring<'_, $environment>,
            ) -> $crate::reexports::Result<()> {
                self.ensure_initialized(spring);
                Ok(())
            }
            fn shutdown(
                &self,
                spring: &mut $crate::reexports::Spring<'_, $environment>,
            ) -> $crate::reexports::Result<()> {
                if self.initialized.get() {
                    self.handler.shutdown(spring);
                }
                Ok(())
            }
            fn job_result(
                &self,
                spring: &mut $crate::reexports::Spring<'_, $environment>,
                id: $crate::reexports::jobs::JobId,
                result: $crate::reexports::jobs::JobResult<'_>,
            ) -> $crate::reexports::Result<()> {
                self.handler.job_result(spring, id, result);
                Ok(())
            }
        }
        $crate::reexports::export_module!(HANDLER, $environment, __SpringAddonModule);
        #[allow(dead_code)]
        pub fn $with_handler<R>(
            f: impl FnOnce(
                &$crate::$handler<$state_type, $environment>,
                &$crate::reexports::Spring<'_, $environment>,
            ) -> R,
        ) -> R {
            HANDLER
                .with(|slot| {
                    slot.invoke(|module, spring| {
                        Ok(f(&module.handler, spring))
                    })
                })
                .expect("addon handler is not available")
        }
        #[allow(dead_code)]
        pub fn with_addon_context<R>(
            f: impl FnOnce(&$crate::AddonContext<'_, $state_type, $environment>) -> R,
        ) -> R {
            $with_handler(|handler, spring| handler.with_context(spring, f))
        }
        #[allow(dead_code)]
        pub fn with_global<R>(f: impl FnOnce(&$state_type) -> R) -> R {
            with_addon_context(|ctx| f(ctx.global()))
        }
    };
}

/// Declare pure workers for an addon module. Addons receive results through
/// their `job_result` method, with the usual context and command queue.
#[macro_export]
macro_rules! export_jobs {
    ({$($workers:tt)*}) => {
        $crate::reexports::export_jobs!(HANDLER, {$($workers)*});
    };
}

/// Export UI addons. Optional `create: make_state` constructs state with Spring
/// and returns `Result<State>`. Without it, state uses `Default`.
#[macro_export]
macro_rules! export_ui_widgets {
    (state: $state_type:ty, $(create: $create:path,)? setup: $setup_fn:path, scratch: $scratch_size:expr $(,)?) => {
        $crate::__impl_ui_exports!($state_type, $setup_fn, $scratch_size, $crate::reexports::Ui $(; $create)?);
    };
    (state: $state_type:ty, $(create: $create:path,)? setup: $setup_fn:path, scratch: $scratch_size:expr, environment: $environment:ty $(,)?) => {
        $crate::__impl_ui_exports!($state_type, $setup_fn, $scratch_size, $environment $(; $create)?);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __impl_ui_exports {
    ($state_type:ty, $setup_fn:path, $scratch_size:expr, $environment:ty $(; $create:path)?) => {
        #[cfg(target_arch = "wasm32")]
        $crate::reexports::export_callin_scratch!($scratch_size);
        $crate::__addon_module!(
            $state_type,
            $setup_fn,
            $environment,
            WidgetHandler,
            with_widget_handler $(; $create)?
        );
        fn __spring_addon_update(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            dt: f32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.update(spring, dt);
            })
        }
        $crate::reexports::export_update!(HANDLER, __spring_addon_update);
        fn __spring_addon_unit_created(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            u: i32,
            d: i32,
            t: i32,
            b: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.unit_created(spring, u, d, t, b);
            })
        }
        $crate::reexports::export_unit_created!(HANDLER, __spring_addon_unit_created);
        fn __spring_addon_feature_created(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            feature_id: i32,
            ally_team_id: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.feature_created(spring, feature_id, ally_team_id);
            })
        }
        $crate::reexports::export_feature_created!(HANDLER, __spring_addon_feature_created);
        fn __spring_addon_feature_destroyed(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            feature_id: i32,
            ally_team_id: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.feature_destroyed(spring, feature_id, ally_team_id);
            })
        }
        $crate::reexports::export_feature_destroyed!(HANDLER, __spring_addon_feature_destroyed);
        fn __spring_addon_unit_destroyed(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            u: i32,
            d: i32,
            t: i32,
            a: i32,
            ad: i32,
            at: i32,
            w: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                let event = $crate::event::UnitDestroyedEvent {
                    unit_id: u,
                    unit_def_id: d,
                    unit_team: t,
                    attacker_id: a,
                    attacker_def_id: ad,
                    attacker_team: at,
                    weapon_def_id: w,
                };
                handler.unit_destroyed(spring, &event);
            })
        }
        $crate::reexports::export_unit_destroyed!(HANDLER, __spring_addon_unit_destroyed);
        fn __spring_addon_unit_given(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            u: i32,
            d: i32,
            old_team: i32,
            new_team: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.unit_given(spring, u, d, old_team, new_team);
            })
        }
        $crate::reexports::export_unit_given!(HANDLER, __spring_addon_unit_given);
        fn __spring_addon_unit_taken(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            u: i32,
            d: i32,
            old_team: i32,
            new_team: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.unit_taken(spring, u, d, old_team, new_team);
            })
        }
        $crate::reexports::export_unit_taken!(HANDLER, __spring_addon_unit_taken);
        fn __spring_addon_unit_entered_los(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            u: i32,
            d: i32,
            t: i32,
            a: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.unit_entered_los(spring, u, d, t, a);
            })
        }
        $crate::reexports::export_unit_entered_los!(HANDLER, __spring_addon_unit_entered_los);
        fn __spring_addon_unit_left_los(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            u: i32,
            d: i32,
            t: i32,
            a: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.unit_left_los(spring, u, d, t, a);
            })
        }
        $crate::reexports::export_unit_left_los!(HANDLER, __spring_addon_unit_left_los);
        fn __spring_addon_unit_entered_radar(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            u: i32,
            d: i32,
            t: i32,
            a: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.unit_entered_radar(spring, u, d, t, a);
            })
        }
        $crate::reexports::export_unit_entered_radar!(HANDLER, __spring_addon_unit_entered_radar);
        fn __spring_addon_unit_left_radar(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            u: i32,
            d: i32,
            t: i32,
            a: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.unit_left_radar(spring, u, d, t, a);
            })
        }
        $crate::reexports::export_unit_left_radar!(HANDLER, __spring_addon_unit_left_radar);
        fn __spring_addon_projectile_created(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            p: i32,
            o: i32,
            w: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.projectile_created(spring, p, o, w);
            })
        }
        $crate::reexports::export_projectile_created!(HANDLER, __spring_addon_projectile_created);
        fn __spring_addon_projectile_destroyed(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            p: i32,
            o: i32,
            w: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.projectile_destroyed(spring, p, o, w);
            })
        }
        $crate::reexports::export_projectile_destroyed!(
            HANDLER,
            __spring_addon_projectile_destroyed
        );
        fn __spring_addon_recv_from_synced(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            msg: &[u8],
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.recv_from_synced(spring, msg);
            })
        }
        $crate::reexports::export_recv_from_synced!(HANDLER, __spring_addon_recv_from_synced);
        fn __spring_addon_mouse_move(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            x: i32,
            y: i32,
            dx: i32,
            dy: i32,
            button: i32,
        ) -> $crate::reexports::Result<bool> {
            let handler = &module.handler;
            Ok({ handler.mouse_move(spring, x, y, dx, dy, button) })
        }
        $crate::reexports::export_mouse_move!(HANDLER, __spring_addon_mouse_move);
        fn __spring_addon_mouse_press(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            x: i32,
            y: i32,
            button: i32,
        ) -> $crate::reexports::Result<bool> {
            let handler = &module.handler;
            Ok({ handler.mouse_press(spring, x, y, button) })
        }
        $crate::reexports::export_mouse_press!(HANDLER, __spring_addon_mouse_press);
        fn __spring_addon_mouse_release(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            x: i32,
            y: i32,
            button: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.mouse_release(spring, x, y, button);
            })
        }
        $crate::reexports::export_mouse_release!(HANDLER, __spring_addon_mouse_release);
        fn __spring_addon_mouse_wheel(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            up: bool,
            value: f32,
        ) -> $crate::reexports::Result<bool> {
            let handler = &module.handler;
            Ok({ handler.mouse_wheel(spring, up, value) })
        }
        $crate::reexports::export_mouse_wheel!(HANDLER, __spring_addon_mouse_wheel);
        fn __spring_addon_key_press(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            key_code: i32,
            alt: bool,
            ctrl: bool,
            meta: bool,
            shift: bool,
            is_repeat: bool,
            label: &str,
            utf32_char: i32,
            scan_code: i32,
            _action_list: &[$crate::reexports::types::callins::KeyAction],
        ) -> $crate::reexports::Result<bool> {
            let handler = &module.handler;
            Ok({
                let event = $crate::event::KeyEvent {
                    key_code,
                    alt,
                    ctrl,
                    meta,
                    shift,
                    is_repeat,
                    label: label.as_bytes(),
                    utf32_char,
                    scan_code,
                };
                handler.key_press(spring, &event)
            })
        }
        $crate::reexports::export_key_press!(HANDLER, __spring_addon_key_press);
        fn __spring_addon_key_release(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            key_code: i32,
            alt: bool,
            ctrl: bool,
            meta: bool,
            shift: bool,
            label: &str,
            utf32_char: i32,
            scan_code: i32,
            _action_list: &[$crate::reexports::types::callins::KeyAction],
        ) -> $crate::reexports::Result<bool> {
            let handler = &module.handler;
            Ok({
                let event = $crate::event::KeyEvent {
                    key_code,
                    alt,
                    ctrl,
                    meta,
                    shift,
                    is_repeat: false,
                    label: label.as_bytes(),
                    utf32_char,
                    scan_code,
                };
                handler.key_release(spring, &event)
            })
        }
        $crate::reexports::export_key_release!(HANDLER, __spring_addon_key_release);
        fn __spring_addon_draw_world(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.draw_world(spring);
            })
        }
        $crate::reexports::export_draw_world!(HANDLER, __spring_addon_draw_world);
        fn __spring_addon_draw_genesis(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.draw_genesis(spring);
            })
        }
        $crate::reexports::export_draw_genesis!(HANDLER, __spring_addon_draw_genesis);
        fn __spring_addon_draw_screen(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            w: i32,
            h: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.draw_screen(spring, w, h);
            })
        }
        $crate::reexports::export_draw_screen!(HANDLER, __spring_addon_draw_screen);
        fn __spring_addon_draw_world_pre_unit(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.draw_world_pre_unit(spring);
            })
        }
        $crate::reexports::export_draw_world_pre_unit!(HANDLER, __spring_addon_draw_world_pre_unit);
        fn __spring_addon_draw_world_refraction(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.draw_world_refraction(spring);
            })
        }
        $crate::reexports::export_draw_world_refraction!(
            HANDLER,
            __spring_addon_draw_world_refraction
        );
        fn __spring_addon_draw_screen_effects(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            w: i32,
            h: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.draw_screen_effects(spring, w, h);
            })
        }
        $crate::reexports::export_draw_screen_effects!(HANDLER, __spring_addon_draw_screen_effects);
        fn __spring_addon_widget_game_over(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            winning_ally_teams: &[u8],
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.game_over(spring, winning_ally_teams);
            })
        }
        $crate::reexports::export_game_over!(HANDLER, __spring_addon_widget_game_over);
        fn __spring_addon_view_resize(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
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
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
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
                handler.view_resize(spring, &geometry);
            })
        }
        $crate::reexports::export_view_resize!(HANDLER, __spring_addon_view_resize);
    };
}

/// Export synced addons. Optional `create: make_state` receives Spring and
/// returns `Result<State>`; otherwise state uses `Default`. Optional `cus: Host`
/// adds the module's unit-script host; with a `CusHost`, register
/// `CusHostGadget::new(with_cus_host)` to attach its scripts.
#[macro_export]
macro_rules! export_rules_gadgets {
    (state: $state_type:ty, $(create: $create:path,)? setup: $setup_fn:path, scratch: $scratch_size:expr, cus: $cus:ty $(,)?) => {
        $crate::__impl_rules_exports!(
            $state_type,
            $setup_fn,
            $scratch_size,
            $crate::reexports::RulesSynced,
            $cus $(; $create)?
        );
    };
    (state: $state_type:ty, $(create: $create:path,)? setup: $setup_fn:path, scratch: $scratch_size:expr, environment: $environment:ty, cus: $cus:ty $(,)?) => {
        $crate::__impl_rules_exports!($state_type, $setup_fn, $scratch_size, $environment, $cus $(; $create)?);
    };
    (state: $state_type:ty, $(create: $create:path,)? setup: $setup_fn:path, scratch: $scratch_size:expr $(,)?) => {
        $crate::__impl_rules_exports!(
            $state_type,
            $setup_fn,
            $scratch_size,
            $crate::reexports::RulesSynced $(; $create)?
        );
    };
    (state: $state_type:ty, $(create: $create:path,)? setup: $setup_fn:path, scratch: $scratch_size:expr, environment: $environment:ty $(,)?) => {
        $crate::__impl_rules_exports!($state_type, $setup_fn, $scratch_size, $environment $(; $create)?);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __impl_rules_exports {
    ($state_type:ty, $setup_fn:path, $scratch_size:expr, $environment:ty $(, $cus:ty)? $(; $create:path)?) => {
        #[cfg(target_arch = "wasm32")]
        $crate::reexports::export_callin_scratch!($scratch_size);
        $crate::__addon_module!(
            $state_type,
            $setup_fn,
            $environment,
            GadgetHandler,
            with_gadget_handler
            $(, $cus)? $(; $create)?
        );
        $(
            fn __spring_addon_cus(module: &__SpringAddonModule) -> &$cus { &module.cus }
            fn __spring_addon_cus_entry<R>(module: &__SpringAddonModule, spring: &mut $crate::reexports::Spring<'_, $environment>, call: impl FnOnce(&$cus, &mut $crate::reexports::Spring<'_, $environment>) -> $crate::reexports::Result<R>) -> $crate::reexports::Result<R> {
                module.handler.with_context(spring, |ctx| call(&module.cus, &mut ctx.spring().reborrow()))
            }
            $crate::reexports::export_core_cus!(HANDLER, $cus, __spring_addon_cus, $environment, __spring_addon_cus_entry);
            /// Run `f` on the module's CUS host, from inside one of its callins; see
            /// [`CusHostGadget`]($crate::CusHostGadget).
            #[allow(dead_code)]
            pub fn with_cus_host(f: &mut dyn FnMut(&$cus)) {
                HANDLER
                    .with(|slot| {
                        slot.invoke(|module, _| {
                            f(&module.cus);
                            Ok(())
                        })
                    })
                    .expect("addon handler is not available")
            }
        )?
        fn __spring_addon_game_frame(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            frame: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.game_frame(spring, frame);
            })
        }
        $crate::reexports::export_game_frame!(HANDLER, __spring_addon_game_frame);
        fn __spring_addon_handle_lua_msg(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            p: i32,
            s: i32,
            m: i32,
            data: &[u8],
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.handle_lua_msg(spring, p, s, m, data);
            })
        }
        $crate::reexports::export_handle_lua_msg!(HANDLER, __spring_addon_handle_lua_msg);
        fn __spring_addon_unit_created(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            u: i32,
            d: i32,
            t: i32,
            b: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                {
                    handler.unit_created(spring, u, d, t, b);
                };
            })
        }
        $crate::reexports::export_unit_created!(HANDLER, __spring_addon_unit_created);
        fn __spring_addon_feature_created(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            feature_id: i32,
            ally_team_id: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.feature_created(spring, feature_id, ally_team_id);
            })
        }
        $crate::reexports::export_feature_created!(HANDLER, __spring_addon_feature_created);
        fn __spring_addon_feature_destroyed(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            feature_id: i32,
            ally_team_id: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.feature_destroyed(spring, feature_id, ally_team_id);
            })
        }
        $crate::reexports::export_feature_destroyed!(HANDLER, __spring_addon_feature_destroyed);
        fn __spring_addon_unit_destroyed(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            u: i32,
            d: i32,
            t: i32,
            a: i32,
            ad: i32,
            at: i32,
            w: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                let event = $crate::rules::UnitDestroyedEvent {
                    unit_id: u,
                    unit_def_id: d,
                    unit_team: t,
                    attacker_id: a,
                    attacker_def_id: ad,
                    attacker_team: at,
                    weapon_def_id: w,
                };
                handler.unit_destroyed(spring, &event);
            })
        }
        $crate::reexports::export_unit_destroyed!(HANDLER, __spring_addon_unit_destroyed);
        fn __spring_addon_unit_idle(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            u: i32,
            d: i32,
            t: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.unit_idle(spring, u, d, t);
            })
        }
        $crate::reexports::export_unit_idle!(HANDLER, __spring_addon_unit_idle);
        fn __spring_addon_projectile_created(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            p: i32,
            o: i32,
            w: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.projectile_created(spring, p, o, w);
            })
        }
        $crate::reexports::export_projectile_created!(HANDLER, __spring_addon_projectile_created);
        fn __spring_addon_unit_pre_damaged(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
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
        ) -> $crate::reexports::Result<(f32, f32)> {
            let handler = &module.handler;
            let result =
                { { handler.unit_pre_damaged(spring, u, ud, ut, d, p, w, pr, a, ad, at) } };
            Ok((result.new_damage, result.impulse_mult))
        }
        $crate::reexports::export_unit_pre_damaged!(HANDLER, __spring_addon_unit_pre_damaged);
        fn __spring_addon_explosion(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            w: i32,
            pos: $crate::reexports::Float3,
            o: i32,
            p: i32,
        ) -> $crate::reexports::Result<bool> {
            let handler = &module.handler;
            Ok({ handler.explosion(spring, w, (pos.x, pos.y, pos.z), o, p) })
        }
        $crate::reexports::export_explosion!(HANDLER, __spring_addon_explosion);
        fn __spring_addon_projectile_destroyed(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            p: i32,
            o: i32,
            w: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.projectile_destroyed(spring, p, o, w);
            })
        }
        $crate::reexports::export_projectile_destroyed!(
            HANDLER,
            __spring_addon_projectile_destroyed
        );
        fn __spring_addon_game_over(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            winning_ally_teams: &[u8],
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.game_over(spring, winning_ally_teams);
            })
        }
        $crate::reexports::export_game_over!(HANDLER, __spring_addon_game_over);
        fn __spring_addon_unit_cmd_done(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            unit_id: i32,
            unit_def_id: i32,
            unit_team: i32,
            command: &$crate::reexports::CallinCommand<'_>,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                let event = $crate::event::CommandEvent {
                    unit_id,
                    unit_def_id,
                    unit_team,
                    command_id: command.id,
                    command_time_out: command.time_out,
                    command_page_index: command.page_index,
                    command_tag: command.tag,
                    command_options: command.options,
                    command_params: command.params,
                    player_num: None,
                    from_synced: true,
                    from_lua: false,
                };
                handler.unit_cmd_done(spring, &event);
            })
        }
        $crate::reexports::export_unit_cmd_done!(HANDLER, __spring_addon_unit_cmd_done);
        fn __spring_addon_allow_command(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            unit_id: i32,
            unit_def_id: i32,
            unit_team: i32,
            command: &$crate::reexports::CallinCommand<'_>,
            player_num: i32,
            from_synced: bool,
            from_lua: bool,
        ) -> $crate::reexports::Result<bool> {
            let handler = &module.handler;
            Ok({
                let event = $crate::event::CommandEvent {
                    unit_id,
                    unit_def_id,
                    unit_team,
                    command_id: command.id,
                    command_time_out: command.time_out,
                    command_page_index: command.page_index,
                    command_tag: command.tag,
                    command_options: command.options,
                    command_params: command.params,
                    player_num: Some(player_num),
                    from_synced,
                    from_lua,
                };
                handler.allow_command(spring, &event)
            })
        }
        $crate::reexports::export_allow_command!(HANDLER, __spring_addon_allow_command);
    };
}

/// Export unsynced addons. Optional `create: make_state` receives Spring and
/// returns `Result<State>`; otherwise state uses `Default`.
#[macro_export]
macro_rules! export_rules_unsynced_addons {
    (state: $state_type:ty, $(create: $create:path,)? setup: $setup_fn:path, scratch: $scratch_size:expr $(,)?) => {
        $crate::__impl_unsynced_exports!(
            $state_type,
            $setup_fn,
            $scratch_size,
            $crate::reexports::RulesUnsynced $(; $create)?
        );
    };
    (state: $state_type:ty, $(create: $create:path,)? setup: $setup_fn:path, scratch: $scratch_size:expr, environment: $environment:ty $(,)?) => {
        $crate::__impl_unsynced_exports!($state_type, $setup_fn, $scratch_size, $environment $(; $create)?);
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! __impl_unsynced_exports {
    ($state_type:ty, $setup_fn:path, $scratch_size:expr, $environment:ty $(; $create:path)?) => {
        #[cfg(target_arch = "wasm32")]
        $crate::reexports::export_callin_scratch!($scratch_size);
        $crate::__addon_module!(
            $state_type,
            $setup_fn,
            $environment,
            UnsyncedHandler,
            with_unsynced_handler $(; $create)?
        );

        fn __spring_addon_unsynced_game_frame(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            frame: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.game_frame(spring, frame);
            })
        }
        $crate::reexports::export_game_frame!(HANDLER, __spring_addon_unsynced_game_frame);
        fn __spring_addon_unsynced_update(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            delta_seconds: f32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.update(spring, delta_seconds);
            })
        }
        $crate::reexports::export_update!(HANDLER, __spring_addon_unsynced_update);
        fn __spring_addon_unsynced_unit_created(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            unit_id: i32,
            unit_def_id: i32,
            unit_team: i32,
            builder_id: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                {
                    handler.unit_created(spring, unit_id, unit_def_id, unit_team, builder_id)
                };
            })
        }
        $crate::reexports::export_unit_created!(HANDLER, __spring_addon_unsynced_unit_created);
        fn __spring_addon_unsynced_unit_finished(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            unit_id: i32,
            unit_def_id: i32,
            unit_team: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.unit_finished(spring, unit_id, unit_def_id, unit_team);
            })
        }
        $crate::reexports::export_unit_finished!(HANDLER, __spring_addon_unsynced_unit_finished);
        fn __spring_addon_unsynced_unit_destroyed(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            unit_id: i32,
            unit_def_id: i32,
            unit_team: i32,
            attacker_id: i32,
            attacker_def_id: i32,
            attacker_team: i32,
            weapon_def_id: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                let event = $crate::event::UnitDestroyedEvent {
                    unit_id,
                    unit_def_id,
                    unit_team,
                    attacker_id,
                    attacker_def_id,
                    attacker_team,
                    weapon_def_id,
                };
                handler.unit_destroyed(spring, &event);
            })
        }
        $crate::reexports::export_unit_destroyed!(HANDLER, __spring_addon_unsynced_unit_destroyed);
        fn __spring_addon_unsynced_unit_given(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            unit_id: i32,
            unit_def_id: i32,
            old_team: i32,
            new_team: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.unit_given(spring, unit_id, unit_def_id, old_team, new_team);
            })
        }
        $crate::reexports::export_unit_given!(HANDLER, __spring_addon_unsynced_unit_given);
        fn __spring_addon_unsynced_unit_taken(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            unit_id: i32,
            unit_def_id: i32,
            old_team: i32,
            new_team: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.unit_taken(spring, unit_id, unit_def_id, old_team, new_team);
            })
        }
        $crate::reexports::export_unit_taken!(HANDLER, __spring_addon_unsynced_unit_taken);
        fn __spring_addon_unsynced_feature_created(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            feature_id: i32,
            ally_team_id: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.feature_created(spring, feature_id, ally_team_id);
            })
        }
        $crate::reexports::export_feature_created!(
            HANDLER,
            __spring_addon_unsynced_feature_created
        );
        fn __spring_addon_unsynced_feature_destroyed(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            feature_id: i32,
            ally_team_id: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.feature_destroyed(spring, feature_id, ally_team_id);
            })
        }
        $crate::reexports::export_feature_destroyed!(
            HANDLER,
            __spring_addon_unsynced_feature_destroyed
        );
        fn __spring_addon_unsynced_projectile_created(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            projectile_id: i32,
            owner_id: i32,
            weapon_def_id: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                {
                    handler.projectile_created(spring, projectile_id, owner_id, weapon_def_id)
                };
            })
        }
        $crate::reexports::export_projectile_created!(
            HANDLER,
            __spring_addon_unsynced_projectile_created
        );
        fn __spring_addon_unsynced_projectile_destroyed(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            projectile_id: i32,
            owner_id: i32,
            weapon_def_id: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                {
                    handler.projectile_destroyed(spring, projectile_id, owner_id, weapon_def_id)
                };
            })
        }
        $crate::reexports::export_projectile_destroyed!(
            HANDLER,
            __spring_addon_unsynced_projectile_destroyed
        );
        fn __spring_addon_unsynced_recv_from_synced(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            message: &[u8],
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.recv_from_synced(spring, message);
            })
        }
        $crate::reexports::export_recv_from_synced!(
            HANDLER,
            __spring_addon_unsynced_recv_from_synced
        );
        fn __spring_addon_unsynced_draw_screen(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            width: i32,
            height: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.draw_screen(spring, width, height);
            })
        }
        $crate::reexports::export_draw_screen!(HANDLER, __spring_addon_unsynced_draw_screen);
        fn __spring_addon_unsynced_draw_world(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.draw_world(spring);
            })
        }
        $crate::reexports::export_draw_world!(HANDLER, __spring_addon_unsynced_draw_world);
        fn __spring_addon_unsynced_draw_world_pre_particles(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            draw_above_water: bool,
            draw_below_water: bool,
            draw_reflection: bool,
            draw_refraction: bool,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                {
                    handler.draw_world_pre_particles(
                        spring,
                        draw_above_water,
                        draw_below_water,
                        draw_reflection,
                        draw_refraction,
                    )
                };
            })
        }
        $crate::reexports::export_draw_world_pre_particles!(
            HANDLER,
            __spring_addon_unsynced_draw_world_pre_particles
        );
        fn __spring_addon_unsynced_draw_screen_effects(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            width: i32,
            height: i32,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.draw_screen_effects(spring, width, height);
            })
        }
        $crate::reexports::export_draw_screen_effects!(
            HANDLER,
            __spring_addon_unsynced_draw_screen_effects
        );
        fn __spring_addon_unsynced_draw_unit(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            unit_id: i32,
            draw_mode: i32,
        ) -> $crate::reexports::Result<bool> {
            let handler = &module.handler;
            Ok({ handler.draw_unit(spring, unit_id, draw_mode) })
        }
        $crate::reexports::export_draw_unit!(HANDLER, __spring_addon_unsynced_draw_unit);
        fn __spring_addon_unsynced_draw_feature(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
            feature_id: i32,
            draw_mode: i32,
        ) -> $crate::reexports::Result<bool> {
            let handler = &module.handler;
            Ok({ handler.draw_feature(spring, feature_id, draw_mode) })
        }
        $crate::reexports::export_draw_feature!(HANDLER, __spring_addon_unsynced_draw_feature);
        fn __spring_addon_unsynced_draw_world_pre_unit(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.draw_world_pre_unit(spring);
            })
        }
        $crate::reexports::export_draw_world_pre_unit!(
            HANDLER,
            __spring_addon_unsynced_draw_world_pre_unit
        );
        fn __spring_addon_unsynced_draw_genesis(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
                handler.draw_genesis(spring);
            })
        }
        $crate::reexports::export_draw_genesis!(HANDLER, __spring_addon_unsynced_draw_genesis);
        fn __spring_addon_unsynced_view_resize(
            module: &__SpringAddonModule,
            spring: &mut $crate::reexports::Spring<'_, $environment>,
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
        ) -> $crate::reexports::Result<()> {
            let handler = &module.handler;
            Ok({
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
                handler.view_resize(spring, &geometry);
            })
        }
        $crate::reexports::export_view_resize!(HANDLER, __spring_addon_unsynced_view_resize);
    };
}
