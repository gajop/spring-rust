use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_unit_storage_fixed(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_storage(unit_id)
            .map_err(|err| format!("get_unit_storage({unit_id}) failed: {err:?}"))?;
        self.same_if_present(label, message, "metalStorage", native.metalStorage)?;
        self.same_if_present(label, message, "energyStorage", native.energyStorage)
    }

    pub(crate) fn check_unit_stockpile_fixed(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let (stockpile, has_stockpile) = self
            .interface
            .units_info()
            .get_unit_stockpile(unit_id)
            .map_err(|err| format!("get_unit_stockpile({unit_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "hasStockpile", has_stockpile)?;
        self.same_i32_if_present(label, message, "stockpile", stockpile.stockpile as i32)?;
        self.same_i32_if_present(
            label,
            message,
            "stockpileQueueSize",
            stockpile.stockpileQueueSize as i32,
        )?;
        self.same_if_present(label, message, "buildPercent", stockpile.buildPercent)
    }

    pub(crate) fn check_unit_is_transporting_fixed(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let (unit_ids, is_transporting) = self
            .interface
            .units_info()
            .get_unit_is_transporting(unit_id)
            .map_err(|err| format!("get_unit_is_transporting({unit_id}) failed: {err:?}"))?;
        self.same_bool_if_present(label, message, "isTransporting", is_transporting)?;
        self.same_i32_set_if_present(label, message, "unitIDs", &unit_ids)
    }

    pub(crate) fn check_unit_pos_error_params_fixed(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let ally_team_id = i32_field(message, "allyTeamID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_pos_error_params(unit_id, ally_team_id)
            .map_err(|err| {
                format!("get_unit_pos_error_params({unit_id}, {ally_team_id}) failed: {err:?}")
            })?;
        self.same_if_present(label, message, "posErrorVectorX", native.posErrorVector.x)?;
        self.same_if_present(label, message, "posErrorVectorY", native.posErrorVector.y)?;
        self.same_if_present(label, message, "posErrorVectorZ", native.posErrorVector.z)?;
        self.same_if_present(label, message, "posErrorDeltaX", native.posErrorDelta.x)?;
        self.same_if_present(label, message, "posErrorDeltaY", native.posErrorDelta.y)?;
        self.same_if_present(label, message, "posErrorDeltaZ", native.posErrorDelta.z)?;
        self.same_i32_if_present(
            label,
            message,
            "nextPosErrorUpdate",
            native.nextPosErrorUpdate,
        )?;
        self.same_bool_if_present(label, message, "posErrorBit", native.posErrorBit)
    }

    pub(crate) fn check_unit_flanking_fixed(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_flanking(unit_id)
            .map_err(|err| format!("get_unit_flanking({unit_id}) failed: {err:?}"))?;
        self.same_i32_if_present(label, message, "flankingMode", native.flankingMode as i32)?;
        self.same_if_present(label, message, "moveFactor", native.moveFactor)?;
        self.same_if_present(label, message, "minDamage", native.minDamage)?;
        self.same_if_present(label, message, "maxDamage", native.maxDamage)?;
        self.same_if_present(label, message, "directionX", native.direction.x)?;
        self.same_if_present(label, message, "directionY", native.direction.y)?;
        self.same_if_present(label, message, "directionZ", native.direction.z)?;
        self.same_if_present(label, message, "mobility", native.mobility)
    }

    pub(crate) fn check_unit_last_hit_piece_fixed(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let unit_id = i32_field(message, "unitID")?;
        let native = self
            .interface
            .units_info()
            .get_unit_last_attacked_piece(unit_id)
            .map_err(|err| format!("get_unit_last_attacked_piece({unit_id}) failed: {err:?}"))?;
        self.compare_last_hit_piece(
            label,
            message,
            native.name,
            native.pieceNum,
            native.frame,
            native.wasHit,
        )
    }

    pub(crate) fn check_feature_last_hit_piece_fixed(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let feature_id = i32_field(message, "featureID")?;
        let native = self
            .interface
            .features()
            .get_feature_last_attacked_piece(feature_id)
            .map_err(|err| {
                format!("get_feature_last_attacked_piece({feature_id}) failed: {err:?}")
            })?;
        self.compare_last_hit_piece(
            label,
            message,
            native.name,
            native.pieceNum,
            native.frame,
            native.wasHit,
        )
    }

    fn compare_last_hit_piece(
        &self,
        label: &str,
        message: &Value,
        native_name: *const i8,
        native_piece_num: i32,
        native_frame: i32,
        native_was_hit: bool,
    ) -> Result<(), String> {
        let native_name = unsafe {
            if native_name.is_null() {
                ""
            } else {
                CStr::from_ptr(native_name).to_str().unwrap_or("")
            }
        };
        self.same_bool_if_present(label, message, "wasHit", native_was_hit)?;
        if !native_was_hit && message.get("wasHit").and_then(Value::as_bool) == Some(false) {
            return Ok(());
        }

        self.same_string_if_present(label, message, "pieceName", native_name)?;
        self.same_i32_if_present(label, message, "pieceNum", native_piece_num)?;
        self.same_i32_if_present(label, message, "frame", native_frame)
    }
}
