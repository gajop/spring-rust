use super::*;
use crate::support::*;
use std::ptr;

fn def_ref(id: i32) -> sys::DefRef {
    sys::DefRef {
        name: ptr::null(),
        id,
    }
}

fn position(message: &Value) -> Result<sys::Float3, String> {
    vec3_from_fields(message, "x", "y", "z")
}

impl NativeApiParity {
    fn create_temporary_unit(&self, message: &Value, label: &str) -> Result<i32, String> {
        self.interface
            .synced_ctrl()
            .unit()
            .create_unit(
                def_ref(i32_field(message, "unitDefID")?),
                position(message)?,
                0,
                i32_field(message, "teamID")?,
                spring_native::CreateUnitOptions {
                    build: false,
                    flatten_ground: false,
                    unit_id: -1,
                    builder_id: -1,
                },
            )
            .map_err(|err| format!("{label}: create_unit() failed: {err:?}"))
    }

    fn create_temporary_feature(&self, message: &Value, label: &str) -> Result<i32, String> {
        self.interface
            .synced_ctrl()
            .feature()
            .create_feature(
                def_ref(i32_field(message, "featureDefID")?),
                position(message)?,
                0,
                i32_field(message, "teamID")?,
                -1,
            )
            .map_err(|err| format!("{label}: create_feature() failed: {err:?}"))
    }

    fn destroy_temporary_unit(&self, unit_id: i32, label: &str) -> Result<(), String> {
        let destroyed = self
            .interface
            .synced_ctrl()
            .unit()
            .destroy_unit(
                unit_id,
                spring_native::DestroyUnitOptions {
                    selfd: false,
                    reclaimed: true,
                    attacker_id: -1,
                    // The Lua fixture passes DestroyUnit's cleanupImmediately
                    // option for test-owned temporary units.  Recycle the
                    // native ID as part of the matching cleanup.
                    recycle_id: true,
                },
            )
            .map_err(|err| format!("{label}: destroy_unit() failed: {err:?}"))?;
        if !destroyed {
            return Err(format!("{label}: destroy_unit() returned false"));
        }
        Ok(())
    }

    fn destroy_temporary_feature(&self, feature_id: i32, label: &str) -> Result<(), String> {
        let destroyed = self
            .interface
            .synced_ctrl()
            .feature()
            .destroy_feature(feature_id)
            .map_err(|err| format!("{label}: destroy_feature() failed: {err:?}"))?;
        if !destroyed {
            return Err(format!("{label}: destroy_feature() returned false"));
        }
        Ok(())
    }

    pub(crate) fn check_object_lifecycle(
        &mut self,
        message: &Value,
        label: &str,
    ) -> Result<(), String> {
        let name = base_test_name(label);
        match name {
            "create_unit_cleanup" => {
                let unit_id = self.create_temporary_unit(message, label)?;
                let created = unit_id >= 0;
                if created {
                    self.destroy_temporary_unit(unit_id, label)?;
                }
                self.same_bool_if_present(label, message, "created", created)
            }
            "destroy_unit" => {
                let unit_id = self.create_temporary_unit(message, label)?;
                if unit_id < 0 {
                    return Err(format!(
                        "{label}: temporary unit creation returned {unit_id}"
                    ));
                }
                self.destroy_temporary_unit(unit_id, label)?;
                let valid = self
                    .interface
                    .units_query()
                    .valid_unit_id(unit_id)
                    .map_err(|err| format!("{label}: valid_unit_id() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "destroyed", !valid)
            }
            "transfer_unit" => {
                let unit_id = self.create_temporary_unit(message, label)?;
                if unit_id < 0 {
                    return Err(format!(
                        "{label}: temporary unit creation returned {unit_id}"
                    ));
                }
                let success = self
                    .interface
                    .synced_ctrl()
                    .unit()
                    .transfer_unit(unit_id, i32_field(message, "teamID")?, true, false)
                    .map_err(|err| format!("{label}: transfer_unit() failed: {err:?}"))?;
                let team_after = self
                    .interface
                    .units_info()
                    .get_unit_team(unit_id)
                    .map_err(|err| format!("{label}: get_unit_team() failed: {err:?}"))?;
                self.destroy_temporary_unit(unit_id, label)?;
                self.same_bool_if_present(label, message, "success", success)?;
                self.same_i32_if_present(label, message, "teamAfter", team_after)
            }
            "create_feature_cleanup" => {
                let feature_id = self.create_temporary_feature(message, label)?;
                let created = feature_id >= 0;
                if created {
                    self.destroy_temporary_feature(feature_id, label)?;
                }
                self.same_bool_if_present(label, message, "created", created)
            }
            "destroy_feature" => {
                let feature_id = self.create_temporary_feature(message, label)?;
                if feature_id < 0 {
                    return Err(format!(
                        "{label}: temporary feature creation returned {feature_id}"
                    ));
                }
                self.destroy_temporary_feature(feature_id, label)?;
                let valid = self
                    .interface
                    .features()
                    .valid_feature_id(feature_id)
                    .map_err(|err| format!("{label}: valid_feature_id() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "destroyed", !valid)
            }
            "transfer_feature" => {
                let feature_id = self.create_temporary_feature(message, label)?;
                if feature_id < 0 {
                    return Err(format!(
                        "{label}: temporary feature creation returned {feature_id}"
                    ));
                }
                self.interface
                    .synced_ctrl()
                    .feature()
                    .transfer_feature(feature_id, i32_field(message, "teamID")?)
                    .map_err(|err| format!("{label}: transfer_feature() failed: {err:?}"))?;
                let team_after = self
                    .interface
                    .features()
                    .get_feature_team(feature_id)
                    .map_err(|err| format!("{label}: get_feature_team() failed: {err:?}"))?;
                self.destroy_temporary_feature(feature_id, label)?;
                self.same_i32_if_present(label, message, "teamAfter", team_after)
            }
            "create_unit_wreck_cleanup" => {
                let feature_id = self
                    .interface
                    .synced_ctrl()
                    .feature()
                    .create_unit_wreck(i32_field(message, "unitID")?, 1, false)
                    .map_err(|err| format!("{label}: create_unit_wreck() failed: {err:?}"))?;
                let created = feature_id >= 0;
                if created {
                    self.destroy_temporary_feature(feature_id, label)?;
                }
                self.same_bool_if_present(label, message, "created", created)
            }
            "create_feature_wreck_cleanup" => {
                let feature_id = self
                    .interface
                    .synced_ctrl()
                    .feature()
                    .create_feature_wreck(i32_field(message, "featureID")?, 1, false)
                    .map_err(|err| format!("{label}: create_feature_wreck() failed: {err:?}"))?;
                let created = feature_id >= 0;
                if created {
                    self.destroy_temporary_feature(feature_id, label)?;
                }
                self.same_bool_if_present(label, message, "created", created)
            }
            "unit_attach" | "unit_detach" | "unit_detach_from_air" => {
                let transporter_id = i32_field(message, "unitID")?;
                let passenger_id = self.create_temporary_unit(message, label)?;
                if passenger_id < 0 {
                    return Err(format!(
                        "{label}: temporary passenger creation returned {passenger_id}"
                    ));
                }

                let synced_ctrl = self.interface.synced_ctrl();
                let unit = synced_ctrl.unit();
                let attached = unit
                    .unit_attach(transporter_id, passenger_id, -1)
                    .map_err(|err| format!("{label}: unit_attach() failed: {err:?}"))?;
                let actual_attached = self
                    .interface
                    .units_info()
                    .get_unit_transporter(passenger_id)
                    .map_err(|err| format!("{label}: get_unit_transporter() failed: {err:?}"))?
                    == transporter_id;

                match name {
                    "unit_attach" => {
                        self.same_bool_if_present(label, message, "attached", actual_attached)?;
                        if !attached || !actual_attached {
                            return Err(format!(
                                "{label}: attach result={attached}, attached_state={actual_attached}"
                            ));
                        }
                        unit.unit_detach(passenger_id).map_err(|err| {
                            format!("{label}: cleanup unit_detach() failed: {err:?}")
                        })?;
                    }
                    "unit_detach" => {
                        unit.unit_detach(passenger_id)
                            .map_err(|err| format!("{label}: unit_detach() failed: {err:?}"))?;
                        let detached = self
                            .interface
                            .units_info()
                            .get_unit_transporter(passenger_id)
                            .map_err(|err| {
                                format!("{label}: post-detach transporter query failed: {err:?}")
                            })?
                            < 0;
                        self.same_bool_if_present(label, message, "detached", detached)?;
                    }
                    "unit_detach_from_air" => {
                        unit.unit_detach_from_air(passenger_id, position(message)?)
                            .map_err(|err| {
                                format!("{label}: unit_detach_from_air() failed: {err:?}")
                            })?;
                        let detached = self
                            .interface
                            .units_info()
                            .get_unit_transporter(passenger_id)
                            .map_err(|err| {
                                format!("{label}: post-detach transporter query failed: {err:?}")
                            })?
                            < 0;
                        self.same_bool_if_present(label, message, "detached", detached)?;
                    }
                    _ => unreachable!(),
                }

                let _ = unit.destroy_unit(
                    passenger_id,
                    spring_native::DestroyUnitOptions {
                        selfd: false,
                        reclaimed: true,
                        attacker_id: -1,
                        recycle_id: true,
                    },
                );
                Ok(())
            }
            "bugger_off" => {
                let success = self
                    .interface
                    .synced_ctrl()
                    .unit()
                    .bugger_off(
                        position(message)?,
                        128.0,
                        i32_field(message, "teamID")?,
                        spring_native::BuggerOffOptions {
                            spherical: true,
                            forced: true,
                            exclude_unit_id: -1,
                        },
                        &[],
                    )
                    .map_err(|err| format!("{label}: bugger_off() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "called", success)
            }
            _ => Err(format!("unsupported object lifecycle check `{name}`")),
        }
    }
}
