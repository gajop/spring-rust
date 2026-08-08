use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_encoding_value(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let text = str_field(message, "text")?;
        match base_test_name(label) {
            "encoding_decode_base64" => {
                let native = self
                    .interface
                    .encoding()
                    .decode_base64(text)
                    .map_err(|err| format!("decode_base64() failed: {err:?}"))?;
                let native = String::from_utf8(native)
                    .map_err(|err| format!("decode_base64() returned non-UTF-8 data: {err}"))?;
                self.same_string_if_present(label, message, "value", &native)
            }
            "encoding_encode_base64_default_padding" => {
                let native = self
                    .interface
                    .encoding()
                    .encode_base64(text.as_bytes(), true)
                    .map_err(|err| format!("encode_base64() failed: {err:?}"))?
                    .ok_or_else(|| "encode_base64() returned no string".to_string())?;
                self.same_string_if_present(label, message, "value", &native)
            }
            "encoding_encode_base64_padded" => {
                let strip_padding = bool_field(message, "stripPadding")?;
                let native = self
                    .interface
                    .encoding()
                    .encode_base64(text.as_bytes(), strip_padding)
                    .map_err(|err| format!("encode_base64() failed: {err:?}"))?
                    .ok_or_else(|| "encode_base64() returned no string".to_string())?;
                self.same_string_if_present(label, message, "value", &native)
            }
            "encoding_is_valid_base64" | "encoding_is_invalid_base64" => {
                let native = self
                    .interface
                    .encoding()
                    .is_valid_base64(text)
                    .map_err(|err| format!("is_valid_base64() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "value", native)
            }
            "encoding_decode_base64_url" => {
                let native = self
                    .interface
                    .encoding()
                    .decode_base64_url(text)
                    .map_err(|err| format!("decode_base64_url() failed: {err:?}"))?;
                let native = String::from_utf8(native)
                    .map_err(|err| format!("decode_base64_url() returned non-UTF-8 data: {err}"))?;
                self.same_string_if_present(label, message, "value", &native)
            }
            "encoding_encode_base64_url" => {
                let native = self
                    .interface
                    .encoding()
                    .encode_base64_url(text.as_bytes())
                    .map_err(|err| format!("encode_base64_url() failed: {err:?}"))?
                    .ok_or_else(|| "encode_base64_url() returned no string".to_string())?;
                self.same_string_if_present(label, message, "value", &native)
            }
            "encoding_is_valid_base64_url" => {
                let native = self
                    .interface
                    .encoding()
                    .is_valid_base64_url(text)
                    .map_err(|err| format!("is_valid_base64_url() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "value", native)
            }
            _ => Err(format!("unsupported encoding check `{label}`")),
        }
    }
}
