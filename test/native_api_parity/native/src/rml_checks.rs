use super::*;

impl NativeApiParity {
    pub(crate) fn check_rml_append_stylesheet_smoke(&self) -> Result<(), String> {
        let rml = self.interface.rml_ui();
        if !rml.is_ready().map_err(|error| format!("{error:?}"))? {
            return Err("RmlUi is not ready".to_owned());
        }

        let context_name = format!("native_api_parity_rml_{}", std::process::id());
        let (context, created) = rml
            .create_context(&context_name)
            .map_err(|error| format!("create_context failed: {error:?}"))?;
        if !created || context == 0 {
            return Err(format!(
                "create_context({context_name}) returned context={context}, created={created}"
            ));
        }

        let result = self.run_rml_append_stylesheet_smoke(context);
        let unload_result = rml
            .context_unload_all_documents(context)
            .map_err(|error| format!("context_unload_all_documents failed: {error:?}"));
        let remove_result = rml
            .remove_context(context)
            .map_err(|error| format!("remove_context failed: {error:?}"));

        result?;
        if !unload_result? {
            return Err("context_unload_all_documents returned false".to_owned());
        }
        if !remove_result? {
            return Err("remove_context returned false".to_owned());
        }
        Ok(())
    }

    fn run_rml_append_stylesheet_smoke(&self, context: u64) -> Result<(), String> {
        let rml = self.interface.rml_ui();

        let document = create_document(&rml, context, "valid")?;
        let appended = rml
            .document_append_to_style_sheet(document, "body { color: rgb(255, 0, 0); }")
            .map_err(|error| format!("document_append_to_style_sheet(valid) failed: {error:?}"))?;
        if !appended {
            return Err("document_append_to_style_sheet(valid) returned false".to_owned());
        }

        let malformed_document = create_document(&rml, context, "malformed")?;
        rml.document_append_to_style_sheet(malformed_document, "body { color: ")
            .map_err(|error| {
                format!("document_append_to_style_sheet(malformed) failed: {error:?}")
            })?;

        Ok(())
    }
}

fn create_document(
    rml: &spring_native::RmlUi<'_>,
    context: u64,
    label: &str,
) -> Result<u64, String> {
    let (document, created) = rml
        .context_create_document(context, "body")
        .map_err(|error| format!("context_create_document({label}) failed: {error:?}"))?;
    if !created || document == 0 {
        return Err(format!(
            "context_create_document({label}) returned document={document}, created={created}"
        ));
    }
    Ok(document)
}
