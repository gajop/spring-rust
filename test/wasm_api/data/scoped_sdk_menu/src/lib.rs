/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use spring_sdk::{
    Menu, Result, Spring,
    module::Module,
    rml::{Color, DataModel, Percent, Pixels, Value, ValueType, Variable},
    types::{gfx::GfxSaveImageOptions, rml_ui::RmlDocumentShowOptions},
};
use std::{
    cell::Cell,
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
};

struct MenuFixture {
    context: u64,
    document: u64,
    button: u64,
    model: DataModel,
    count: Variable<i32>,
    status: Variable<String>,
    events: Arc<AtomicU32>,
    activations: Cell<u32>,
    frames: Cell<u32>,
    script: String,
    captured: Cell<bool>,
}
fn checkpoint(spring: &mut Spring<'_, Menu>, value: &str) -> Result<()> {
    assert!(
        spring
            .config()
            .set_config_string("ScopedSdkProof", value, false)?
    );
    Ok(())
}
impl Module<Menu> for MenuFixture {
    fn initialize(spring: &mut Spring<'_, Menu>) -> Result<Self> {
        assert!(
            spring
                .rml_ui()
                .load_font_face("fonts/FreeSansBold.otf", false, None)?
        );
        let created = spring.rml_ui().create_context("scoped-sdk-proof")?;
        assert!(created.success);
        let context = created.context_handle;
        let model = DataModel::create(spring, context, "scoped_fixture")?;
        let count = model.bind(spring, "count", 0i32)?;
        let status = model.bind(spring, "status", String::from("Ready"))?;
        let visible = model.bind(spring, "visible", true)?;
        assert!(visible.get(spring)?);
        visible.set(spring, false)?;
        assert!(!visible.get(spring)?);
        let scale = model.bind(spring, "scale", 1.0f32)?;
        scale.set(spring, 1.5)?;
        assert_eq!(scale.get(spring)?, 1.5);
        let accent = model.bind(spring, "accent", Color([84, 190, 220, 255]))?;
        assert_eq!(accent.get(spring)?, Color([84, 190, 220, 255]));
        let spacing = model.bind(spring, "spacing", Pixels(12.0))?;
        assert_eq!(spacing.get(spring)?, Pixels(12.0));
        let progress = model.bind(spring, "progress", Percent(75.0))?;
        assert_eq!(progress.get(spring)?, Percent(75.0));
        let rows = model.rows(
            spring,
            "players",
            &[("name", ValueType::String), ("score", ValueType::Int)],
        )?;
        rows.set(
            spring,
            vec![
                Value::String("Alice".into()),
                Value::Int(7),
                Value::String("Bob".into()),
                Value::Int(3),
            ],
        )?;
        let events = Arc::new(AtomicU32::new(0));
        let event_count = Arc::clone(&events);
        let status_copy = status.clone();
        let count_copy = count.clone();
        model.event(
            spring,
            "clicked",
            &[ValueType::String, ValueType::Float],
            move |spring, event| {
                assert_eq!(
                    event.values,
                    vec![Value::String("captured 日本語".into()), Value::Float(7.0)]
                );
                event_count.fetch_add(1, Ordering::Relaxed);
                count_copy.set(spring, 7)?;
                status_copy.set(spring, "Clicked #7".into())?;
                checkpoint(spring, "typed-event:7")
            },
        )?;
        let loaded = spring
            .rml_ui()
            .context_load_document(context, "WasmMenu/menu.rml")?;
        assert!(loaded.success);
        let document = loaded.document_handle;
        assert!(
            spring
                .rml_ui()
                .document_show(document, RmlDocumentShowOptions::default())?
        );
        let root = spring.rml_ui().context_get_root_element(context)?;
        assert!(root.exists);
        let button = spring
            .rml_ui()
            .element_get_element_by_id(root.element_handle, "action")?;
        assert!(button.exists);
        // Same retained handler reenters itself synchronously. Its Cell state
        // is shared, with no mutable handler or module borrow spanning dispatch.
        let depth = Arc::new(AtomicU32::new(0));
        let calls = Arc::clone(&depth);
        let element = button.element_handle;
        let listener = spring.rml_ui().element_add_event_listener(
            element,
            "change",
            false,
            move |spring, _| {
                let before = calls.load(Ordering::Relaxed);
                calls.store(before + 1, Ordering::Relaxed);
                if before == 0 {
                    assert!(spring.rml_ui().element_dispatch_event(element, "change")?);
                }
                Ok(())
            },
        )?;
        assert!(listener.success);
        assert!(spring.rml_ui().element_dispatch_event(element, "change")?);
        assert_eq!(depth.load(Ordering::Relaxed), 2);
        checkpoint(spring, "initialized:reentry=2")?;
        if cfg!(feature = "fail-init") {
            checkpoint(spring, "expected-init-failure")?;
            panic!(
                "intentional SDK fixture initialization failure after registering Rml resources"
            );
        }
        let script = String::from_utf8(spring.vfs().load_file("startscript.txt", "e")?)
            .expect("fixture script is UTF-8");
        assert!(
            script.starts_with("[GAME]"),
            "fixture start script must be loaded from the menu VFS"
        );
        Ok(Self {
            context,
            document,
            button: element,
            model,
            count,
            status,
            events,
            activations: Cell::new(
                spring
                    .config()
                    .get_config_string("ScopedSdkActivations", Some("0"))?
                    .value
                    .parse()
                    .expect("activation counter"),
            ),
            frames: Cell::new(0),
            script,
            captured: Cell::new(false),
        })
    }
    fn shutdown(&self, spring: &mut Spring<'_, Menu>) -> Result<()> {
        assert!(!spring.rml_ui().context_get_name(self.context)?.is_empty());
        assert!(spring.rml_ui().document_close(self.document)?);
        self.model.close(spring)?;
        assert!(self.count.get(spring).is_err());
        assert!(spring.rml_ui().remove_context(self.context)?);
        checkpoint(spring, "shutdown:bindings-invalidated")
    }
}
fn activate(module: &MenuFixture, spring: &mut Spring<'_, Menu>, _message: &str) -> Result<()> {
    module.activations.set(module.activations.get() + 1);
    assert!(spring.config().set_config_string(
        "ScopedSdkActivations",
        &module.activations.get().to_string(),
        false
    )?);
    module.frames.set(0);
    checkpoint(spring, &format!("activation:{}", module.activations.get()))?;
    Ok(())
}
fn game(_: &MenuFixture, spring: &mut Spring<'_, Menu>) -> Result<()> {
    checkpoint(spring, "game-transition")?;
    Ok(())
}
fn update(module: &MenuFixture, spring: &mut Spring<'_, Menu>, _delta_seconds: f32) -> Result<()> {
    let frame = module.frames.get() + 1;
    module.frames.set(frame);
    if frame == 30 {
        let before = module.events.load(Ordering::Relaxed);
        assert!(
            spring
                .rml_ui()
                .element_dispatch_event(module.button, "click")?
        );
        assert_eq!(module.events.load(Ordering::Relaxed), before + 1);
        assert_eq!(module.count.get(spring)?, 7);
        assert_eq!(module.status.get(spring)?, "Clicked #7");
    }
    if frame == 120 {
        if module.activations.get() < 3 {
            assert!(spring.system_control().reload(&module.script)?);
        } else {
            checkpoint(spring, "complete:2-cycles")?;
            assert!(spring.system_control().quit()?);
        }
    }
    Ok(())
}
spring_sdk::export_module!(MODULE, Menu, MenuFixture);
#[cfg(target_arch = "wasm32")]
spring_sdk::export_callin_scratch!(65536);
spring_sdk::export_activate_menu!(MODULE, activate);
spring_sdk::export_activate_game!(MODULE, game);
spring_sdk::export_update!(MODULE, update);

fn draw(
    module: &MenuFixture,
    spring: &mut Spring<'_, Menu>,
    view_size_x: i32,
    view_size_y: i32,
) -> Result<()> {
    if module.frames.get() >= 60 && !module.captured.replace(true) {
        assert!(spring.gfx().save_image(
            0,
            0,
            view_size_x,
            view_size_y,
            "scoped-sdk-menu.png",
            GfxSaveImageOptions {
                yflip: true,
                ..Default::default()
            },
            0x0405
        )?);
    }
    Ok(())
}
spring_sdk::export_draw_screen_post!(MODULE, draw);
