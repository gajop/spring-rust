// This file is part of the Spring engine (GPL v2 or later), see LICENSE.html

use spring_native::RmlDocumentShowOptions;
use spring_native::prelude::*;

struct MenuModule {
    interface: NativeInterfaceRef,
    context: u64,
}

impl MenuModule {
    fn activate_menu(&mut self) {
        let _ = self.interface.rml_ui().context_get_name(self.context);
    }

    fn activate_game(&mut self) {
        let _ = self.interface.rml_ui().context_get_name(self.context);
    }
}

impl NativeModule for MenuModule {
    fn new(interface: NativeInterfaceRef) -> Self {
        let ui = interface.rml_ui();
        let (context, context_created) = ui
            .create_context("native-rml-menu")
            .expect("native fixture could not create its RmlUi context");
        assert!(context_created);

        let (document, document_loaded) = ui
            .context_load_document(context, "WasmMenu/menu.rml")
            .expect("native fixture could not load its packaged RML document");
        assert!(document_loaded);
        assert!(
            ui.document_show(document, RmlDocumentShowOptions::default())
                .expect("native fixture could not show its RML document")
        );

        let (root, root_exists) = ui
            .context_get_root_element(context)
            .expect("native fixture could not get its document root");
        assert!(root_exists);
        let (button, button_exists) = ui
            .element_get_element_by_id(root, "launch")
            .expect("native fixture could not find its button");
        assert!(button_exists);
        let (status, status_exists) = ui
            .element_get_element_by_id(root, "status")
            .expect("native fixture could not find its status element");
        assert!(status_exists);

        let callback_ui = ui;
        ui.element_add_event_listener(button, "click", false, move || {
            let _ = callback_ui.element_set_inner_rml(status, "Clicked!");
        })
        .expect("native fixture could not register its button callback");

        Self { interface, context }
    }

    fn shutdown(&mut self) -> Result<(), Error> {
        // The host deliberately calls Shutdown before owner cleanup. Querying
        // the context here proves that native modules receive that window.
        let _ = self.interface.rml_ui().context_get_name(self.context)?;
        let removed = self.interface.rml_ui().remove_context(self.context)?;
        if !removed {
            return Err(Error::new(
                1,
                "native fixture context was not removable".to_string(),
            ));
        }
        Ok(())
    }
}

spring_native::export_module!(MenuModule);

// Menu lifecycle hooks are intentionally separate from NativeModule's game
// event trait. They exercise the host's native menu phase forwarding.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn ActivateMenu(
    _interface: *const spring_native::sys::NativeInterface,
    module_data: *mut c_void,
    _query: *const spring_native::sys::ActivateMenuQuery,
    result: *mut spring_native::sys::ActivateMenuResult,
) {
    if module_data.is_null() || result.is_null() {
        return;
    }
    unsafe {
        (*result).error = std::ptr::null();
        (*(module_data as *mut spring_native::ModuleData<MenuModule>))
            .module()
            .activate_menu();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn ActivateGame(
    _interface: *const spring_native::sys::NativeInterface,
    module_data: *mut c_void,
    _query: *const spring_native::sys::SimpleCallinQuery,
    result: *mut spring_native::sys::SimpleCallinResult,
) {
    if module_data.is_null() || result.is_null() {
        return;
    }
    unsafe {
        (*result).error = std::ptr::null();
        (*(module_data as *mut spring_native::ModuleData<MenuModule>))
            .module()
            .activate_game();
    }
}
