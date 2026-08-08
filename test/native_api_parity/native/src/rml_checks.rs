use super::*;

use std::{
    fmt::Debug,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

use serde_json::{Map, Value};
use spring_native::{RmlRegisterEventTypeOptions, RmlUi};

impl NativeApiParity {
    pub(crate) fn check_rml_global_context_document(&self, message: &Value) -> Result<(), String> {
        let expected = message
            .get("result")
            .ok_or_else(|| "Rml surface payload is missing `result`".to_owned())?;
        let rml = self.interface.rml_ui();
        ensure(
            rml.is_ready().map_err(format_error)?,
            "RmlUi should be ready in rendering tests",
        )?;

        // The Lua surface records all calls and invokes the native module only
        // after its cleanup calls. Replaying on that same context would compare
        // against already-unloaded state, so use an independent context while
        // keeping the operation order, arguments, and observable result shape
        // identical.
        let context_name = format!(
            "native_api_parity_surface_global_context_document_native_{}",
            std::process::id()
        );
        let (context, created) = rml.create_context(&context_name).map_err(format_error)?;
        ensure(
            created && context != 0,
            "Rml surface native replay should create a context",
        )?;

        let replay = (|| -> Result<Map<String, Value>, String> {
            let mut actual = Map::new();

            let record = |actual: &mut Map<String, Value>, name: &str, values: Vec<Value>| {
                actual.insert(
                    name.to_owned(),
                    serde_json::json!({
                        "n": values.len(),
                        "values": values,
                    }),
                );
            };
            let record_void = |actual: &mut Map<String, Value>, name: &str| {
                record(actual, name, Vec::new());
            };
            let record_bool = |actual: &mut Map<String, Value>, name: &str, value: bool| {
                record(actual, name, vec![serde_json::json!(value)]);
            };
            let record_userdata =
                |actual: &mut Map<String, Value>, name: &str, handle: u64, exists: bool| {
                    let value = if exists && handle != 0 {
                        serde_json::json!({ "type": "userdata" })
                    } else {
                        serde_json::json!({ "type": "nil" })
                    };
                    record(actual, name, vec![value]);
                };

            record_userdata(&mut actual, "RmlUi.CreateContext", context, true);
            let (got_context, context_exists) =
                rml.get_context(&context_name).map_err(format_error)?;
            record_userdata(&mut actual, "RmlUi.GetContext", got_context, context_exists);

            record_bool(
                &mut actual,
                "RmlUi.AddTranslationString",
                rml.add_translation_string("native_api_parity_surface_key", "surface translation")
                    .map_err(format_error)?,
            );
            ensure(
                rml.clear_translations().map_err(format_error)?,
                "RmlUi.ClearTranslations should succeed",
            )?;
            record_void(&mut actual, "RmlUi.ClearTranslations");

            record_bool(
                &mut actual,
                "RmlUi.LoadFontFace",
                rml.load_font_face("native_api_parity_missing_font.ttf", false, None)
                    .map_err(format_error)?,
            );

            let event_id = rml
                .regiser_event_type(
                    "native_api_parity_surface_event",
                    RmlRegisterEventTypeOptions {
                        interruptible: true,
                        bubbles: true,
                        default_phase: None,
                    },
                )
                .map_err(format_error)?;
            record(
                &mut actual,
                "RmlUi.RegiserEventType",
                vec![serde_json::json!(event_id)],
            );

            ensure(
                rml.set_mouse_cursor_alias("native-api-parity-surface", "Arrow")
                    .map_err(format_error)?,
                "RmlUi.SetMouseCursorAlias should succeed",
            )?;
            record_void(&mut actual, "RmlUi.SetMouseCursorAlias");
            ensure(
                rml.set_debug_context(context).map_err(format_error)?,
                "RmlUi.SetDebugContext should succeed",
            )?;
            record_void(&mut actual, "RmlUi.SetDebugContext");

            let paths = rml
                .get_document_path_requests("native-api-parity-surface.rml")
                .map_err(format_error)?;
            record(
                &mut actual,
                "RmlUi.GetDocumentPathRequests",
                vec![serde_json::json!({ "type": "table", "count": paths.len() })],
            );
            ensure(
                rml.clear_document_path_requests("native-api-parity-surface.rml")
                    .map_err(format_error)?,
                "RmlUi.ClearDocumentPathRequests should succeed",
            )?;
            record_void(&mut actual, "RmlUi.ClearDocumentPathRequests");

            let (x, y) = rml.vector2i_new(12, 34).map_err(format_error)?;
            record(
                &mut actual,
                "RmlUi.Vector2i.new",
                vec![serde_json::json!({ "type": "vector2i", "x": x, "y": y })],
            );
            let (x, y) = rml.vector2f_new(12.5, 34.5).map_err(format_error)?;
            record(
                &mut actual,
                "RmlUi.Vector2f.new",
                vec![serde_json::json!({ "type": "vector2f", "x": x, "y": y })],
            );

            let (_listener, attached) = rml
                .context_add_event_listener(context, "click", false, || {})
                .map_err(format_error)?;
            ensure(attached, "RmlUi.Context.AddEventListener should attach")?;
            record_void(&mut actual, "RmlUi.Context.AddEventListener");

            ensure(
                rml.context_enable_mouse_cursor(context, false)
                    .map_err(format_error)?,
                "RmlUi.Context.EnableMouseCursor should succeed",
            )?;
            record_void(&mut actual, "RmlUi.Context.EnableMouseCursor");
            ensure(
                rml.context_activate_theme(context, "native-api-parity-surface-theme", false)
                    .map_err(format_error)?,
                "RmlUi.Context.ActivateTheme should succeed",
            )?;
            record_void(&mut actual, "RmlUi.Context.ActivateTheme");
            record_bool(
                &mut actual,
                "RmlUi.Context.IsThemeActive",
                rml.context_is_theme_active(context, "native-api-parity-surface-theme")
                    .map_err(format_error)?,
            );
            record_bool(
                &mut actual,
                "RmlUi.Context.ProcessMouseMove",
                rml.context_process_mouse_move(context, 1.0, 1.0, 0)
                    .map_err(format_error)?,
            );
            record_bool(
                &mut actual,
                "RmlUi.Context.ProcessMouseButtonDown",
                rml.context_process_mouse_button_down(context, 0, 0)
                    .map_err(format_error)?,
            );
            record_bool(
                &mut actual,
                "RmlUi.Context.ProcessMouseButtonUp",
                rml.context_process_mouse_button_up(context, 0, 0)
                    .map_err(format_error)?,
            );
            record_bool(
                &mut actual,
                "RmlUi.Context.ProcessMouseWheel",
                rml.context_process_mouse_wheel(context, 0.0, 1.0, 0)
                    .map_err(format_error)?,
            );
            record_bool(
                &mut actual,
                "RmlUi.Context.ProcessMouseLeave",
                rml.context_process_mouse_leave(context)
                    .map_err(format_error)?,
            );
            record_bool(
                &mut actual,
                "RmlUi.Context.IsMouseInteracting",
                rml.context_is_mouse_interacting(context)
                    .map_err(format_error)?,
            );
            record_bool(
                &mut actual,
                "RmlUi.Context.ProcessKeyDown",
                rml.context_process_key_down(context, 65, 0)
                    .map_err(format_error)?,
            );
            record_bool(
                &mut actual,
                "RmlUi.Context.ProcessKeyUp",
                rml.context_process_key_up(context, 65, 0)
                    .map_err(format_error)?,
            );
            record_bool(
                &mut actual,
                "RmlUi.Context.ProcessTextInput",
                rml.context_process_text_input(context, "x")
                    .map_err(format_error)?,
            );

            let (document, document_created) = rml
                .context_create_document(context, "body")
                .map_err(format_error)?;
            ensure(
                document_created && document != 0,
                "RmlUi.Context.CreateDocument should create a document",
            )?;
            record_userdata(
                &mut actual,
                "RmlUi.Context.CreateDocument",
                document,
                document_created,
            );
            ensure(
                rml.element_set_id(document, "native-api-parity-surface-document")
                    .map_err(format_error)?,
                "surface document id should be set",
            )?;
            let (looked_up, document_exists) = rml
                .context_get_document(context, "native-api-parity-surface-document")
                .map_err(format_error)?;
            record_userdata(
                &mut actual,
                "RmlUi.Context.GetDocument",
                looked_up,
                document_exists,
            );
            let (missing_document, missing_document_exists) = rml
                .context_load_document(context, "native_api_parity_missing_surface_document.rml")
                .map_err(format_error)?;
            record_userdata(
                &mut actual,
                "RmlUi.Context.LoadDocument",
                missing_document,
                missing_document_exists,
            );
            let (data_model, data_model_opened) = rml
                .context_open_data_model(context, "native_api_parity_surface_model")
                .map_err(format_error)?;
            ensure(
                data_model_opened && data_model != 0,
                "RmlUi.Context.OpenDataModel should open a data model",
            )?;
            record(
                &mut actual,
                "RmlUi.Context.OpenDataModel",
                vec![serde_json::json!({ "type": "data_model", "fields": 1 })],
            );
            let (element, element_exists) = rml
                .context_get_element_at_point(context, 1.0, 1.0, 0)
                .map_err(format_error)?;
            record_userdata(
                &mut actual,
                "RmlUi.Context.GetElementAtPoint",
                element,
                element_exists,
            );
            ensure(
                rml.context_pull_document_to_front(context, document)
                    .map_err(format_error)?,
                "RmlUi.Context.PullDocumentToFront should succeed",
            )?;
            record_void(&mut actual, "RmlUi.Context.PullDocumentToFront");
            ensure(
                rml.context_push_document_to_back(context, document)
                    .map_err(format_error)?,
                "RmlUi.Context.PushDocumentToBack should succeed",
            )?;
            record_void(&mut actual, "RmlUi.Context.PushDocumentToBack");

            ensure(
                rml.document_append_to_style_sheet(document, "body { color: rgb(1, 2, 3); }")
                    .map_err(format_error)?,
                "RmlUi.Document.AppendToStyleSheet should succeed",
            )?;
            record_void(&mut actual, "RmlUi.Document.AppendToStyleSheet");
            let (element_ptr, element_created) = rml
                .document_create_element(document, "div")
                .map_err(format_error)?;
            record_userdata(
                &mut actual,
                "RmlUi.Document.CreateElement",
                element_ptr,
                element_created,
            );
            let (text_ptr, text_created) = rml
                .document_create_text_node(document, "surface")
                .map_err(format_error)?;
            record_userdata(
                &mut actual,
                "RmlUi.Document.CreateTextNode",
                text_ptr,
                text_created,
            );
            ensure(
                rml.document_load_inline_script(
                    document,
                    "return true",
                    "native-api-parity-surface",
                    1,
                )
                .map_err(format_error)?,
                "RmlUi.Document.LoadInlineScript should succeed",
            )?;
            record_void(&mut actual, "RmlUi.Document.LoadInlineScript");
            ensure(
                rml.document_load_external_script(document, "native_api_parity_missing_surface.js")
                    .map_err(format_error)?,
                "RmlUi.Document.LoadExternalScript should return after a missing file",
            )?;
            record_void(&mut actual, "RmlUi.Document.LoadExternalScript");
            ensure(
                rml.document_reload_style_sheet(document)
                    .map_err(format_error)?,
                "RmlUi.Document.ReloadStyleSheet should succeed",
            )?;
            record_void(&mut actual, "RmlUi.Document.ReloadStyleSheet");
            ensure(
                rml.document_show(document, spring_native::RmlDocumentShowOptions::default())
                    .map_err(format_error)?,
                "RmlUi.Document.Show should succeed",
            )?;
            record_void(&mut actual, "RmlUi.Document.Show");
            ensure(
                rml.document_hide(document).map_err(format_error)?,
                "RmlUi.Document.Hide should succeed",
            )?;
            record_void(&mut actual, "RmlUi.Document.Hide");
            ensure(
                rml.document_pull_to_front(document).map_err(format_error)?,
                "RmlUi.Document.PullToFront should succeed",
            )?;
            record_void(&mut actual, "RmlUi.Document.PullToFront");
            ensure(
                rml.document_push_to_back(document).map_err(format_error)?,
                "RmlUi.Document.PushToBack should succeed",
            )?;
            record_void(&mut actual, "RmlUi.Document.PushToBack");
            ensure(
                rml.document_update_document(document)
                    .map_err(format_error)?,
                "RmlUi.Document.UpdateDocument should succeed",
            )?;
            record_void(&mut actual, "RmlUi.Document.UpdateDocument");

            let (close_document, close_created) = rml
                .context_create_document(context, "body")
                .map_err(format_error)?;
            ensure(
                close_created && close_document != 0,
                "close document should be created",
            )?;
            ensure(
                rml.document_close(close_document).map_err(format_error)?,
                "RmlUi.Document.Close should succeed",
            )?;
            record_void(&mut actual, "RmlUi.Document.Close");

            let (unload_document, unload_created) = rml
                .context_create_document(context, "body")
                .map_err(format_error)?;
            ensure(
                unload_created && unload_document != 0,
                "unload document should be created",
            )?;
            ensure(
                rml.context_unload_document(context, unload_document)
                    .map_err(format_error)?,
                "RmlUi.Context.UnloadDocument should succeed",
            )?;
            record_void(&mut actual, "RmlUi.Context.UnloadDocument");
            record_bool(
                &mut actual,
                "RmlUi.Context.RemoveDataModel",
                rml.context_remove_data_model(context, "native_api_parity_surface_model")
                    .map_err(format_error)?,
            );
            record_bool(
                &mut actual,
                "RmlUi.Context.Update",
                rml.context_update(context).map_err(format_error)?,
            );
            record_bool(
                &mut actual,
                "RmlUi.Context.Render",
                rml.context_render(context).map_err(format_error)?,
            );
            ensure(
                rml.context_unload_all_documents(context)
                    .map_err(format_error)?,
                "RmlUi.Context.UnloadAllDocuments should succeed",
            )?;
            record_void(&mut actual, "RmlUi.Context.UnloadAllDocuments");
            ensure(
                rml.remove_context(context).map_err(format_error)?,
                "RmlUi.RemoveContext should succeed",
            )?;
            record_void(&mut actual, "RmlUi.RemoveContext");

            Ok(actual)
        })();

        if replay.is_err() {
            let _ = rml.context_unload_all_documents(context);
            let _ = rml.remove_context(context);
        }
        let actual = replay?;
        let actual = Value::Object(actual);
        if expected != &actual {
            return Err(format!(
                "Rml surface result mismatch: expected={expected}, actual={actual}"
            ));
        }
        Ok(())
    }

    pub(crate) fn check_rml_context_document_lifecycle(&self) -> Result<(), String> {
        let rml = self.interface.rml_ui();
        ensure(
            rml.is_ready().map_err(format_error)?,
            "RmlUi should be ready in rendering tests",
        )?;
        ensure(
            rml.get_version()
                .map_err(format_error)?
                .is_some_and(|version| !version.is_empty()),
            "RmlUi version should be present",
        )?;

        let missing_name = format!("native_api_parity_missing_{}", std::process::id());
        let (missing_context, exists) = rml.get_context(&missing_name).map_err(format_error)?;
        ensure_eq("missing context handle", missing_context, 0)?;
        ensure_eq("missing context exists", exists, false)?;

        self.with_rml_context("lifecycle", |rml, context| {
            ensure_eq(
                "context name",
                rml.context_get_name(context).map_err(format_error)?,
                Some(context_name("lifecycle")),
            )?;
            ensure(
                rml.context_set_dimensions(context, 640, 480)
                    .map_err(format_error)?,
                "context_set_dimensions should succeed",
            )?;
            ensure_eq(
                "context dimensions",
                rml.context_get_dimensions(context).map_err(format_error)?,
                (640, 480),
            )?;
            ensure(
                rml.context_set_density_independent_pixel_ratio(context, 1.25)
                    .map_err(format_error)?,
                "context_set_density_independent_pixel_ratio should succeed",
            )?;
            ensure_near(
                "context density independent pixel ratio",
                rml.context_get_density_independent_pixel_ratio(context)
                    .map_err(format_error)?,
                1.25,
            )?;

            let (root, root_exists) = rml
                .context_get_root_element(context)
                .map_err(format_error)?;
            ensure(
                root_exists && root != 0,
                "context root element should exist",
            )?;
            let (focus, focus_exists) = rml
                .context_get_focus_element(context)
                .map_err(format_error)?;
            ensure(
                !focus_exists || focus != 0,
                "focus element should be either absent or non-zero",
            )?;

            let document = create_document(rml, context, "lifecycle")?;
            let (document_context, document_context_exists) =
                rml.document_get_context(document).map_err(format_error)?;
            ensure_eq("document context", document_context, context)?;
            ensure(document_context_exists, "document context should exist")?;
            ensure_eq(
                "document tag name",
                rml.element_get_tag_name(document).map_err(format_error)?,
                Some("body".to_owned()),
            )?;
            ensure_eq(
                "document title before set",
                rml.document_get_title(document).map_err(format_error)?,
                Some(String::new()),
            )?;
            ensure(
                rml.document_set_title(document, "Native Rml Behavior")
                    .map_err(format_error)?,
                "document_set_title should succeed",
            )?;
            ensure_eq(
                "document title after set",
                rml.document_get_title(document).map_err(format_error)?,
                Some("Native Rml Behavior".to_owned()),
            )?;
            ensure(
                rml.document_show(document, spring_native::RmlDocumentShowOptions::default())
                    .map_err(format_error)?,
                "document_show should succeed",
            )?;
            ensure(
                rml.element_is_visible(document).map_err(format_error)?,
                "document should be visible after show",
            )?;
            ensure(
                rml.document_hide(document).map_err(format_error)?,
                "document_hide should succeed",
            )?;
            ensure(
                !rml.element_is_visible(document).map_err(format_error)?,
                "document should not be visible after hide",
            )?;
            ensure(
                rml.context_unload_document(context, document)
                    .map_err(format_error)?,
                "context_unload_document should unload the document",
            )
        })
    }

    pub(crate) fn check_rml_context_document_extra_behavior(&self) -> Result<(), String> {
        self.with_rml_context("context_document_extra", |rml, context| {
            ensure(
                rml.context_activate_theme(context, "native-api-parity-theme", true)
                    .map_err(format_error)?,
                "context_activate_theme(true) should succeed",
            )?;
            ensure(
                rml.context_is_theme_active(context, "native-api-parity-theme")
                    .map_err(format_error)?,
                "theme should be active after activation",
            )?;
            ensure(
                rml.context_activate_theme(context, "native-api-parity-theme", false)
                    .map_err(format_error)?,
                "context_activate_theme(false) should succeed",
            )?;
            ensure(
                !rml.context_is_theme_active(context, "native-api-parity-theme")
                    .map_err(format_error)?,
                "theme should be inactive after deactivation",
            )?;
            let document_path = "native-api-parity.rml";
            ensure(
                rml.get_document_path_requests(document_path)
                    .map_err(format_error)?
                    .is_empty(),
                "document path requests should initially be empty",
            )?;
            ensure(
                rml.clear_document_path_requests(document_path)
                    .map_err(format_error)?,
                "clear_document_path_requests should succeed",
            )?;
            ensure(
                rml.get_document_path_requests(document_path)
                    .map_err(format_error)?
                    .is_empty(),
                "document path requests should be empty after clear",
            )?;

            let document = create_document(rml, context, "context_document_extra")?;
            ensure(
                rml.element_set_id(document, "native-extra-document")
                    .map_err(format_error)?,
                "element_set_id(document) should succeed",
            )?;
            ensure(
                rml.document_show(
                    document,
                    spring_native::RmlDocumentShowOptions {
                        modal: Some(1),
                        focus: Some(1),
                    },
                )
                .map_err(format_error)?,
                "document_show(modal, document focus) should succeed",
            )?;
            ensure(
                rml.document_is_modal(document).map_err(format_error)?,
                "document should be modal after modal show",
            )?;
            ensure(
                rml.document_get_url(document)
                    .map_err(format_error)?
                    .is_some(),
                "document_get_url should return a string",
            )?;
            ensure_eq(
                "context_get_document(extra)",
                rml.context_get_document(context, "native-extra-document")
                    .map_err(format_error)?,
                (document, true),
            )?;
            ensure(
                rml.context_pull_document_to_front(context, document)
                    .map_err(format_error)?,
                "context_pull_document_to_front should succeed",
            )?;
            ensure(
                rml.context_push_document_to_back(context, document)
                    .map_err(format_error)?,
                "context_push_document_to_back should succeed",
            )?;
            ensure(
                rml.document_pull_to_front(document).map_err(format_error)?,
                "document_pull_to_front should succeed",
            )?;
            ensure(
                rml.document_push_to_back(document).map_err(format_error)?,
                "document_push_to_back should succeed",
            )?;

            let (text_ptr, text_created) = rml
                .document_create_text_node(document, "native text")
                .map_err(format_error)?;
            ensure(
                text_created && text_ptr != 0,
                "document_create_text_node should create an element ptr",
            )?;
            let text = expect_element(
                "appended text node",
                rml.element_append_child(document, text_ptr)
                    .map_err(format_error)?,
            )?;
            ensure(
                rml.element_get_inner_rml(document)
                    .map_err(format_error)?
                    .is_some_and(|rml| rml.contains("native text")),
                "document inner_rml should include appended text",
            )?;
            ensure(
                rml.element_set_scroll_left(text, 0).map_err(format_error)?,
                "element_set_scroll_left should succeed",
            )?;
            ensure_eq(
                "element scroll left",
                rml.element_get_scroll_left(text).map_err(format_error)?,
                0,
            )?;
            let _ = rml
                .context_get_element_at_point(context, 1.0, 1.0, 0)
                .map_err(format_error)?;

            let close_document = create_document(rml, context, "close")?;
            ensure(
                rml.element_set_id(close_document, "native-close-document")
                    .map_err(format_error)?,
                "element_set_id(close document) should succeed",
            )?;
            ensure(
                rml.document_show(
                    close_document,
                    spring_native::RmlDocumentShowOptions::default(),
                )
                .map_err(format_error)?,
                "close document show should succeed",
            )?;
            ensure_eq(
                "context_get_document(close) before close",
                rml.context_get_document(context, "native-close-document")
                    .map_err(format_error)?,
                (close_document, true),
            )?;
            ensure(
                rml.document_close(close_document).map_err(format_error)?,
                "document_close should succeed",
            )?;
            ensure(
                rml.context_update(context).map_err(format_error)?,
                "context_update after close should succeed",
            )?;
            ensure_eq(
                "context_get_document(close) after close",
                rml.context_get_document(context, "native-close-document")
                    .map_err(format_error)?,
                (0, false),
            )
        })
    }

    pub(crate) fn check_rml_global_input_behavior(&self) -> Result<(), String> {
        let rml = self.interface.rml_ui();
        ensure(
            rml.clear_translations().map_err(format_error)?,
            "clear_translations should succeed",
        )?;
        let _ = rml
            .add_translation_string("native_api_parity_key", "translated")
            .map_err(format_error)?;
        ensure(
            rml.set_mouse_cursor_alias("native-api-parity-cursor", "Move")
                .map_err(format_error)?,
            "set_mouse_cursor_alias should succeed",
        )?;

        let event_type = format!("native_api_parity_custom_{}", std::process::id());
        ensure(
            rml.register_event_type(
                &event_type,
                spring_native::RmlRegisterEventTypeOptions {
                    interruptible: true,
                    bubbles: true,
                    ..Default::default()
                },
            )
            .map_err(format_error)?
                > 0,
            "register_event_type should return a valid event id",
        )?;
        let legacy_event_type = format!("native_api_parity_legacy_{}", std::process::id());
        ensure(
            rml.regiser_event_type(
                &legacy_event_type,
                spring_native::RmlRegisterEventTypeOptions {
                    bubbles: true,
                    ..Default::default()
                },
            )
            .map_err(format_error)?
                > 0,
            "regiser_event_type alias should return a valid event id",
        )?;
        ensure_eq(
            "vector2f_new",
            rml.vector2f_new(1.5, -2.25).map_err(format_error)?,
            (1.5, -2.25),
        )?;
        ensure_eq(
            "vector2i_new",
            rml.vector2i_new(7, -8).map_err(format_error)?,
            (7, -8),
        )?;

        self.with_rml_context("input", |rml, context| {
            ensure(
                rml.set_debug_context(context).map_err(format_error)?,
                "set_debug_context should succeed",
            )?;
            ensure(
                rml.set_debug_context_by_name(&context_name("input"))
                    .map_err(format_error)?,
                "set_debug_context_by_name should succeed",
            )?;
            ensure(
                rml.context_enable_mouse_cursor(context, true)
                    .map_err(format_error)?,
                "context_enable_mouse_cursor should succeed",
            )?;

            let document = create_document(rml, context, "input")?;
            ensure(
                rml.document_show(document, spring_native::RmlDocumentShowOptions::default())
                    .map_err(format_error)?,
                "document_show should succeed before input processing",
            )?;
            let _ = rml
                .context_process_mouse_move(context, 4.0, 5.0, 0)
                .map_err(format_error)?;
            let _ = rml
                .context_process_mouse_button_down(context, 0, 0)
                .map_err(format_error)?;
            let _ = rml
                .context_process_mouse_button_up(context, 0, 0)
                .map_err(format_error)?;
            let _ = rml
                .context_process_mouse_wheel(context, 0.0, -1.0, 0)
                .map_err(format_error)?;
            let _ = rml
                .context_process_mouse_leave(context)
                .map_err(format_error)?;
            let _ = rml
                .context_is_mouse_interacting(context)
                .map_err(format_error)?;
            let _ = rml
                .context_process_key_down(context, 65, 0)
                .map_err(format_error)?;
            let _ = rml
                .context_process_key_up(context, 65, 0)
                .map_err(format_error)?;
            let _ = rml
                .context_process_text_input(context, "native")
                .map_err(format_error)?;

            let data_model_name = format!("native_api_parity_model_{}", std::process::id());
            let (data_model, opened) = rml
                .context_open_data_model(context, &data_model_name)
                .map_err(format_error)?;
            ensure(
                opened && data_model != 0,
                "context_open_data_model should return a handle",
            )?;
            ensure(
                rml.sol_lua_data_model___set_dirty(data_model, "missing_property")
                    .map_err(format_error)?,
                "sol_lua_data_model___set_dirty should accept native data model handle",
            )?;
            ensure(
                rml.context_remove_data_model(context, &data_model_name)
                    .map_err(format_error)?,
                "context_remove_data_model should succeed",
            )?;
            expect_invalid(
                "sol_lua_data_model___set_dirty removed data model",
                rml.sol_lua_data_model___set_dirty(data_model, "missing_property"),
            )
        })
    }

    pub(crate) fn check_rml_dom_query_behavior(&self) -> Result<(), String> {
        self.with_rml_document("dom", |rml, document| {
            let container = append_new_element(rml, document, document, "div", Some("container"))?;
            ensure(
                rml.element_set_class_name(container, "panel primary")
                    .map_err(format_error)?,
                "element_set_class_name should succeed",
            )?;
            ensure(
                rml.element_set_attribute(container, "data-owner", "native")
                    .map_err(format_error)?,
                "element_set_attribute should succeed",
            )?;
            ensure(
                rml.element_set_inner_rml(
                    container,
                    "<span id=\"alpha\" class=\"chip hot\" data-role=\"primary\">A</span><button id=\"beta\" class=\"chip\">B</button>",
                )
                .map_err(format_error)?,
                "element_set_inner_rml should succeed",
            )?;
            ensure(
                rml.document_update_document(document).map_err(format_error)?,
                "document_update_document should succeed",
            )?;

            let alpha = expect_element(
                "alpha by id",
                rml.element_get_element_by_id(document, "alpha")
                    .map_err(format_error)?,
            )?;
            let beta = expect_element(
                "beta by selector",
                rml.element_query_selector(document, "#beta")
                    .map_err(format_error)?,
            )?;
            ensure_eq(
                "alpha query selector should return same element",
                expect_element(
                    "alpha by selector",
                    rml.element_query_selector(document, "span.hot")
                        .map_err(format_error)?,
                )?,
                alpha,
            )?;
            ensure_eq(
                "chip class count",
                rml.element_get_elements_by_class_name_count(document, "chip")
                    .map_err(format_error)?,
                2,
            )?;
            ensure_eq(
                "span tag count",
                rml.element_get_elements_by_tag_name_count(document, "span")
                    .map_err(format_error)?,
                1,
            )?;
            let chips = rml
                .element_query_selector_all(document, ".chip")
                .map_err(format_error)?;
            ensure_eq("query selector all .chip count", chips.len(), 2)?;
            ensure(
                chips.contains(&alpha) && chips.contains(&beta),
                "query selector all .chip should contain alpha and beta",
            )?;

            ensure_eq(
                "alpha tag name",
                rml.element_get_tag_name(alpha).map_err(format_error)?,
                Some("span".to_owned()),
            )?;
            ensure_eq(
                "alpha id",
                rml.element_get_id(alpha).map_err(format_error)?,
                Some("alpha".to_owned()),
            )?;
            ensure_eq(
                "alpha data-role",
                rml.element_get_attribute(alpha, "data-role")
                    .map_err(format_error)?,
                (Some("primary".to_owned()), true),
            )?;
            ensure(
                rml.element_has_attribute(alpha, "data-role")
                    .map_err(format_error)?,
                "alpha should have data-role before removal",
            )?;
            ensure(
                rml.element_remove_attribute(alpha, "data-role")
                    .map_err(format_error)?,
                "element_remove_attribute should succeed",
            )?;
            ensure(
                !rml
                    .element_has_attribute(alpha, "data-role")
                    .map_err(format_error)?,
                "alpha should not have data-role after removal",
            )?;

            ensure(
                rml.element_is_class_set(alpha, "hot")
                    .map_err(format_error)?,
                "alpha should have hot class",
            )?;
            ensure(
                rml.element_set_class(alpha, "selected", true)
                    .map_err(format_error)?,
                "element_set_class should succeed",
            )?;
            ensure(
                rml.element_is_class_set(alpha, "selected")
                    .map_err(format_error)?,
                "alpha should have selected class",
            )?;
            ensure(
                rml.element_matches(alpha, "span.selected")
                    .map_err(format_error)?,
                "alpha should match span.selected",
            )?;
            ensure_eq(
                "alpha closest container",
                expect_element(
                    "closest #container",
                    rml.element_closest(alpha, "#container")
                        .map_err(format_error)?,
                )?,
                container,
            )?;

            ensure(
                rml.element_set_pseudo_class(alpha, "native-test", true)
                    .map_err(format_error)?,
                "element_set_pseudo_class should succeed",
            )?;
            ensure(
                rml.element_is_pseudo_class_set(alpha, "native-test")
                    .map_err(format_error)?,
                "native-test pseudo class should be set",
            )?;
            ensure(
                rml.element_get_active_pseudo_classes(alpha)
                    .map_err(format_error)?
                    .contains(&"native-test".to_owned()),
                "active pseudo classes should contain native-test",
            )
        })
    }

    pub(crate) fn check_rml_child_mutation_behavior(&self) -> Result<(), String> {
        self.with_rml_document("children", |rml, document| {
            let parent = append_new_element(rml, document, document, "div", Some("parent"))?;
            append_new_element(rml, document, parent, "p", Some("first"))?;
            let third = append_new_element(rml, document, parent, "p", Some("third"))?;

            let second_ptr = create_element_ptr(rml, document, "p", "second")?;
            let second = expect_element(
                "inserted second",
                rml.element_insert_before(parent, second_ptr, third)
                    .map_err(format_error)?,
            )?;
            ensure(
                rml.element_set_id(second, "second").map_err(format_error)?,
                "set second id should succeed",
            )?;
            ensure_child_id(rml, parent, 0, "first")?;
            ensure_child_id(rml, parent, 1, "second")?;
            ensure_child_id(rml, parent, 2, "third")?;

            let section_ptr = create_element_ptr(rml, document, "section", "replacement")?;
            let removed_second_ptr = rml
                .element_replace_child(parent, section_ptr, second)
                .map_err(format_error)?;
            ensure(
                removed_second_ptr.1,
                "element_replace_child should return removed child",
            )?;
            let replacement = expect_element(
                "replacement section",
                rml.element_query_selector(parent, "section")
                    .map_err(format_error)?,
            )?;
            ensure(
                rml.element_set_id(replacement, "replacement")
                    .map_err(format_error)?,
                "set replacement id should succeed",
            )?;
            ensure_child_id(rml, parent, 1, "replacement")?;
            ensure(
                !rml.element_query_selector(parent, "#second")
                    .map_err(format_error)?
                    .1,
                "second should be detached after replace",
            )?;

            let reattached_second = expect_element(
                "reattached second",
                rml.element_append_child(parent, removed_second_ptr.0)
                    .map_err(format_error)?,
            )?;
            ensure_eq("reattached second handle", reattached_second, second)?;
            ensure_child_id(rml, parent, 3, "second")?;

            let removed_third_ptr = rml
                .element_remove_child(parent, third)
                .map_err(format_error)?;
            ensure(
                removed_third_ptr.1,
                "element_remove_child should return removed child",
            )?;
            ensure(
                !rml.element_query_selector(parent, "#third")
                    .map_err(format_error)?
                    .1,
                "third should be detached after remove_child",
            )?;
            let reattached_third = expect_element(
                "reattached third",
                rml.element_append_child(parent, removed_third_ptr.0)
                    .map_err(format_error)?,
            )?;
            ensure_eq("reattached third handle", reattached_third, third)?;
            expect_invalid(
                "consumed element ptr append",
                rml.element_append_child(parent, removed_third_ptr.0),
            )?;
            ensure_child_id(rml, parent, 0, "first")
        })
    }

    pub(crate) fn check_rml_event_behavior(&self) -> Result<(), String> {
        self.with_rml_document("events", |rml, document| {
            let button =
                append_new_element(rml, document, document, "button", Some("event-button"))?;
            let calls = Arc::new(AtomicUsize::new(0));
            let context_calls = Arc::new(AtomicUsize::new(0));
            let event_log = Arc::new(Mutex::new(Vec::<String>::new()));
            let context_event_log = Arc::new(Mutex::new(Vec::<String>::new()));
            let interface = self.interface;
            let context = document_context(rml, document)?;
            let calls_for_callback = Arc::clone(&calls);
            let event_log_for_callback = Arc::clone(&event_log);
            let context_calls_for_callback = Arc::clone(&context_calls);
            let context_event_log_for_callback = Arc::clone(&context_event_log);

            let (listener, attached) = rml
                .element_add_event_listener(button, "click", false, move || {
                    calls_for_callback.fetch_add(1, Ordering::SeqCst);
                    let callback_rml = interface.rml_ui();
                    let mut log = event_log_for_callback.lock().expect("event log poisoned");
                    match callback_rml.event_get_current() {
                        Ok((event, element, callback_document, true)) => {
                            let (event_type, type_exists) = callback_rml
                                .event_get_type(event)
                                .unwrap_or((Some("<error>".to_owned()), false));
                            log.push(format!(
                                "{}:{}:{}:{}",
                                event_type.unwrap_or_default(),
                                type_exists,
                                element == button,
                                callback_document == document
                            ));
                        }
                        other => log.push(format!("missing current event: {other:?}")),
                    }
                })
                .map_err(format_error)?;
            ensure(attached && listener != 0, "event listener should attach")?;
            let (context_listener, context_attached) = rml
                .context_add_event_listener(context, "click", true, move || {
                    context_calls_for_callback.fetch_add(1, Ordering::SeqCst);
                    let callback_rml = interface.rml_ui();
                    let mut log = context_event_log_for_callback
                        .lock()
                        .expect("context event log poisoned");
                    match callback_rml.event_get_current() {
                        Ok((event, _, _, true)) => {
                            let (event_type, type_exists) = callback_rml
                                .event_get_type(event)
                                .unwrap_or((Some("<error>".to_owned()), false));
                            log.push(format!(
                                "{}:{}",
                                event_type.unwrap_or_default(),
                                type_exists
                            ));
                        }
                        other => log.push(format!("missing current event: {other:?}")),
                    }
                })
                .map_err(format_error)?;
            ensure(
                context_attached && context_listener != 0,
                "context event listener should attach",
            )?;
            ensure_eq(
                "current event before dispatch",
                rml.event_get_current().map_err(format_error)?.3,
                false,
            )?;
            ensure(
                rml.element_dispatch_event(button, "click")
                    .map_err(format_error)?,
                "element_dispatch_event should succeed",
            )?;
            ensure_eq("event callback count", calls.load(Ordering::SeqCst), 1)?;
            ensure_eq(
                "context event callback count",
                context_calls.load(Ordering::SeqCst),
                1,
            )?;
            ensure_eq(
                "event callback log",
                event_log.lock().map_err(|err| err.to_string())?.as_slice(),
                &["click:true:true:true".to_owned()],
            )?;
            ensure_eq(
                "context event callback log",
                context_event_log
                    .lock()
                    .map_err(|err| err.to_string())?
                    .as_slice(),
                &["click:true".to_owned()],
            )?;
            ensure_eq(
                "current event after dispatch",
                rml.event_get_current().map_err(format_error)?.3,
                false,
            )
        })
    }

    pub(crate) fn check_rml_form_control_behavior(&self) -> Result<(), String> {
        self.with_rml_document("forms", |rml, document| {
            ensure(
                rml.element_set_inner_rml(
                    document,
                    "<input id=\"input\" value=\"abcdef\" /><textarea id=\"text\">hello world</textarea>",
                )
                    .map_err(format_error)?,
                "setting form inner RML should succeed",
            )?;
            ensure(
                rml.document_update_document(document)
                    .map_err(format_error)?,
                "document_update_document should succeed",
            )?;
            let input = expect_element(
                "input by id",
                rml.element_get_element_by_id(document, "input")
                    .map_err(format_error)?,
            )?;
            ensure(
                rml.document_show(document, spring_native::RmlDocumentShowOptions::default())
                    .map_err(format_error)?,
                "document_show should succeed before form focus",
            )?;
            ensure(
                rml.element_focus(input).map_err(format_error)?,
                "element_focus(input) should succeed",
            )?;
            ensure_eq(
                "input value",
                rml.element_get_value(input).map_err(format_error)?,
                Some("abcdef".to_owned()),
            )?;
            ensure(
                rml.element_form_control_input_set_selection(input, 1, 4)
                    .map_err(format_error)?,
                "input set selection should succeed",
            )?;
            ensure_eq(
                "input selection",
                rml.element_form_control_input_get_selection(input)
                    .map_err(format_error)?,
                (1, 4, true),
            )?;

			let textarea = expect_element(
				"textarea by id",
				rml.element_get_element_by_id(document, "text")
					.map_err(format_error)?,
			)?;
			ensure(
				rml.element_focus(textarea).map_err(format_error)?,
				"element_focus(textarea) should succeed",
			)?;
			ensure_eq(
				"textarea value",
				rml.element_get_value(textarea).map_err(format_error)?,
				Some("hello world".to_owned()),
			)?;
            ensure(
                rml.element_form_control_text_area_set_selection(textarea, 0, 5)
                    .map_err(format_error)?,
                "textarea set selection should succeed",
            )?;
            ensure_eq(
                "textarea selection",
                rml.element_form_control_text_area_get_selection(textarea)
                    .map_err(format_error)?,
                (0, 5, true),
            )?;

			let select = append_new_element(rml, document, document, "select", Some("select"))?;
			let option_template =
				append_new_element(rml, document, document, "option", Some("option-template"))?;
			ensure(
				rml.element_set_attribute(option_template, "value", "one")
					.map_err(format_error)?,
				"setting option value should succeed",
			)?;
			ensure(
				rml.element_set_attribute(option_template, "selected", "")
					.map_err(format_error)?,
				"setting option selected attribute should succeed",
			)?;
			ensure(
				rml.element_set_inner_rml(option_template, "One")
					.map_err(format_error)?,
				"setting option inner RML should succeed",
			)?;
			let option_ptr = rml.element_clone(option_template).map_err(format_error)?;
			ensure(
				option_ptr.1 && option_ptr.0 != 0,
				"element_clone(option) should create an option ptr",
			)?;
			ensure(
				rml.element_form_control_select_add(select, option_ptr.0, -1)
					.map_err(format_error)?,
				"select add option should succeed",
			)?;
			expect_invalid(
				"consumed select option ptr",
				rml.element_form_control_select_add(select, option_ptr.0, -1),
			)?;
			ensure(
				rml.document_update_document(document)
					.map_err(format_error)?,
				"document_update_document should succeed after select add",
			)?;
			ensure_eq(
				"select value after add",
				rml.element_get_value(select).map_err(format_error)?,
				Some("one".to_owned()),
			)?;
			ensure(
				rml.element_form_control_select_remove(select, 0)
					.map_err(format_error)?,
				"select remove option should succeed",
			)?;
			ensure(
				rml.document_update_document(document)
					.map_err(format_error)?,
				"document_update_document should succeed after select remove",
			)?;
			ensure_eq(
				"select value after remove",
				rml.element_get_value(select).map_err(format_error)?,
				Some(String::new()),
			)?;
			ensure(
				rml.element_set_attribute(option_template, "value", "two")
					.map_err(format_error)?,
				"setting second option value should succeed",
			)?;
			ensure(
				rml.element_set_inner_rml(option_template, "Two")
					.map_err(format_error)?,
				"setting second option inner RML should succeed",
			)?;
			let second_option_ptr = rml.element_clone(option_template).map_err(format_error)?;
			ensure(
				second_option_ptr.1 && second_option_ptr.0 != 0,
				"element_clone(second option) should create an option ptr",
			)?;
			ensure(
				rml.element_form_control_select_add(select, second_option_ptr.0, -1)
					.map_err(format_error)?,
				"select add second option should succeed",
			)?;
			ensure_eq(
				"select value after second add",
				rml.element_get_value(select).map_err(format_error)?,
				Some("two".to_owned()),
			)?;
			ensure(
				rml.element_form_control_select_remove_all(select)
					.map_err(format_error)?,
				"select remove all should succeed",
			)?;
			ensure_eq(
				"select value after remove all",
				rml.element_get_value(select).map_err(format_error)?,
				Some(String::new()),
			)
		})
    }

    pub(crate) fn check_rml_stylesheet_append_behavior(&self) -> Result<(), String> {
        self.with_rml_document("stylesheet", |rml, document| {
            ensure(
                rml.document_append_to_style_sheet(document, "body { color: rgb(255, 0, 0); }")
                    .map_err(format_error)?,
                "first stylesheet append to empty native document should succeed",
            )?;
            ensure(
                rml.document_append_to_style_sheet(document, ".chip { display: block; }")
                    .map_err(format_error)?,
                "second stylesheet append should combine with existing stylesheet",
            )?;
            ensure(
                rml.document_update_document(document)
                    .map_err(format_error)?,
                "document_update_document should succeed after stylesheet append",
            )?;

            let malformed_document =
                create_document(rml, document_context(rml, document)?, "malformed")?;
            rml.document_append_to_style_sheet(malformed_document, "body { color: ")
                .map_err(format_error)?;
            Ok(())
        })
    }

    pub(crate) fn check_rml_invalid_zero_handle_behavior(&self) -> Result<(), String> {
        let rml = self.interface.rml_ui();
        expect_invalid("context_render zero handle", rml.context_render(0))?;
        expect_invalid(
            "context_unload_document zero document",
            rml.context_unload_document(0, 0),
        )?;
        expect_invalid("document_get_title zero handle", rml.document_get_title(0))?;
        expect_invalid(
            "document_set_title zero handle",
            rml.document_set_title(0, "bad"),
        )?;
        expect_invalid("element_get_id zero handle", rml.element_get_id(0))?;
        expect_invalid(
            "element_set_attribute zero handle",
            rml.element_set_attribute(0, "bad", "bad"),
        )?;
        expect_invalid(
            "element_append_child zero handles",
            rml.element_append_child(0, 0),
        )?;
        expect_invalid("event_get_type zero handle", rml.event_get_type(0))
    }
    fn with_rml_document<F>(&self, label: &str, test: F) -> Result<(), String>
    where
        F: FnOnce(&RmlUi<'_>, u64) -> Result<(), String>,
    {
        self.with_rml_context(label, |rml, context| {
            let document = create_document(rml, context, label)?;
            test(rml, document)
        })
    }

    fn with_rml_context<F>(&self, label: &str, test: F) -> Result<(), String>
    where
        F: FnOnce(&RmlUi<'_>, u64) -> Result<(), String>,
    {
        let rml = self.interface.rml_ui();
        ensure(
            rml.is_ready().map_err(format_error)?,
            "RmlUi should be ready in rendering tests",
        )?;

        let name = context_name(label);
        let (context, created) = rml.create_context(&name).map_err(format_error)?;
        ensure(
            created && context != 0,
            &format!("create_context({name}) should return a context"),
        )?;

        let body_result = test(&rml, context);
        let unload_result = rml
            .context_unload_all_documents(context)
            .map_err(format_error);
        let remove_result = rml.remove_context(context).map_err(format_error);

        body_result?;
        ensure(
            unload_result?,
            &format!("context_unload_all_documents({name}) should succeed"),
        )?;
        ensure(
            remove_result?,
            &format!("remove_context({name}) should succeed"),
        )
    }
}

fn context_name(label: &str) -> String {
    format!("native_api_parity_rml_{}_{}", label, std::process::id())
}

fn create_document(rml: &RmlUi<'_>, context: u64, label: &str) -> Result<u64, String> {
    let (document, created) = rml
        .context_create_document(context, "body")
        .map_err(format_error)?;
    ensure(
        created && document != 0,
        &format!("context_create_document({label}) should return a document"),
    )?;
    Ok(document)
}

fn create_element_ptr(
    rml: &RmlUi<'_>,
    document: u64,
    tag: &str,
    label: &str,
) -> Result<u64, String> {
    let (element_ptr, created) = rml
        .document_create_element(document, tag)
        .map_err(format_error)?;
    ensure(
        created && element_ptr != 0,
        &format!("document_create_element({label}:{tag}) should return an element ptr"),
    )?;
    Ok(element_ptr)
}

fn append_new_element(
    rml: &RmlUi<'_>,
    document: u64,
    parent: u64,
    tag: &str,
    id: Option<&str>,
) -> Result<u64, String> {
    let element_ptr = create_element_ptr(rml, document, tag, id.unwrap_or(tag))?;
    let element = expect_element(
        &format!("append {tag}"),
        rml.element_append_child(parent, element_ptr)
            .map_err(format_error)?,
    )?;
    if let Some(id) = id {
        ensure(
            rml.element_set_id(element, id).map_err(format_error)?,
            &format!("element_set_id({id}) should succeed"),
        )?;
    }
    Ok(element)
}

fn document_context(rml: &RmlUi<'_>, document: u64) -> Result<u64, String> {
    let (context, exists) = rml.document_get_context(document).map_err(format_error)?;
    ensure(exists, "document_get_context should exist")?;
    Ok(context)
}

fn ensure_child_id(
    rml: &RmlUi<'_>,
    parent: u64,
    index: i32,
    expected_id: &str,
) -> Result<(), String> {
    let child = expect_element(
        &format!("child at index {index}"),
        rml.element_get_child(parent, index).map_err(format_error)?,
    )?;
    ensure_eq(
        &format!("child {index} id"),
        rml.element_get_id(child).map_err(format_error)?,
        Some(expected_id.to_owned()),
    )
}

fn expect_element(label: &str, value: (u64, bool)) -> Result<u64, String> {
    ensure(
        value.1 && value.0 != 0,
        &format!("{label} should return an element"),
    )?;
    Ok(value.0)
}

fn ensure(condition: bool, message: &str) -> Result<(), String> {
    if condition {
        Ok(())
    } else {
        Err(message.to_owned())
    }
}

fn ensure_eq<T>(label: &str, actual: T, expected: T) -> Result<(), String>
where
    T: PartialEq + Debug,
{
    if actual == expected {
        Ok(())
    } else {
        Err(format!("{label}: actual={actual:?}, expected={expected:?}"))
    }
}

fn ensure_near(label: &str, actual: f32, expected: f32) -> Result<(), String> {
    if (actual - expected).abs() <= 0.001 {
        Ok(())
    } else {
        Err(format!("{label}: actual={actual}, expected={expected}"))
    }
}

fn expect_invalid<T>(label: &str, result: Result<T, spring_native::Error>) -> Result<(), String> {
    match result {
        Err(error) if error.code() == 1 => Ok(()),
        Err(error) => Err(format!("{label}: expected invalid argument, got {error:?}")),
        Ok(_) => Err(format!("{label}: expected invalid argument, got success")),
    }
}

fn format_error(error: spring_native::Error) -> String {
    format!("{error:?}")
}
