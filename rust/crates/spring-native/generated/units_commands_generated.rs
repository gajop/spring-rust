/// The complete result tuple returned by [`get_factory_bugger_off`].
pub type GetFactoryBuggerOffValue = (bool, f32, f32, i32, bool, bool);

impl<'a> UnitsCommands<'a> {
    pub fn get_unit_command_count(&self, unit_id: i32) -> Result<u32, Error> {
        unsafe {
            let query = sys::GetUnitCommandCountQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitCommandCountResult>::zeroed();
            let func = self.api.GetUnitCommandCount.expect("GetUnitCommandCount function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.count
            })
        }
    }

    pub fn get_unit_commands(&self, unit_id: i32, max_commands: u32) -> Result<Vec<sys::CommandFFI>, Error> {
        unsafe {
            let query = sys::GetUnitCommandsQuery {
                unitID: unit_id,
                maxCommands: max_commands,
            };
            let mut result = MaybeUninit::<sys::GetUnitCommandsResult>::zeroed();
            let func = self.api.GetUnitCommands.expect("GetUnitCommands function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.commands.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.commands as *const sys::CommandFFI, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_unit_current_command(&self, unit_id: i32, cmd_index: i32) -> Result<(sys::CommandFFI, bool), Error> {
        unsafe {
            let query = sys::GetUnitCurrentCommandQuery {
                unitID: unit_id,
                cmdIndex: cmd_index,
            };
            let mut result = MaybeUninit::<sys::GetUnitCurrentCommandResult>::zeroed();
            let func = self.api.GetUnitCurrentCommand.expect("GetUnitCurrentCommand function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.command,
                result.hasCommand,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_factory_counts(&self, unit_id: i32, count: i32, add_cmds: bool) -> Result<sys::FactoryQueueInfo, Error> {
        unsafe {
            let query = sys::GetFactoryCountsQuery {
                unitID: unit_id,
                count,
                addCmds: add_cmds,
            };
            let mut result = MaybeUninit::<sys::GetFactoryCountsResult>::zeroed();
            let func = self.api.GetFactoryCounts.expect("GetFactoryCounts function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.info
            })
        }
    }

    pub fn get_factory_command_count(&self, unit_id: i32) -> Result<u32, Error> {
        unsafe {
            let query = sys::GetFactoryCommandCountQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetFactoryCommandCountResult>::zeroed();
            let func = self.api.GetFactoryCommandCount.expect("GetFactoryCommandCount function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.count
            })
        }
    }

    pub fn get_factory_commands(&self, unit_id: i32, max_commands: u32) -> Result<Vec<sys::CommandFFI>, Error> {
        unsafe {
            let query = sys::GetFactoryCommandsQuery {
                unitID: unit_id,
                maxCommands: max_commands,
            };
            let mut result = MaybeUninit::<sys::GetFactoryCommandsResult>::zeroed();
            let func = self.api.GetFactoryCommands.expect("GetFactoryCommands function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.commands.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.commands as *const sys::CommandFFI, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_factory_bugger_off(&self, unit_id: i32) -> Result<GetFactoryBuggerOffValue, Error> {
        unsafe {
            let query = sys::GetFactoryBuggerOffQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetFactoryBuggerOffResult>::zeroed();
            let func = self.api.GetFactoryBuggerOff.expect("GetFactoryBuggerOff function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.perform,
                result.offset,
                result.radius,
                result.relHeading,
                result.spherical,
                result.forced,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_command_queue(&self, unit_id: i32, max_commands: u32) -> Result<Vec<sys::CommandFFI>, Error> {
        unsafe {
            let query = sys::GetCommandQueueQuery {
                unitID: unit_id,
                maxCommands: max_commands,
            };
            let mut result = MaybeUninit::<sys::GetCommandQueueResult>::zeroed();
            let func = self.api.GetCommandQueue.expect("GetCommandQueue function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.commands.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.commands as *const sys::CommandFFI, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_full_build_queue(&self, unit_id: i32) -> Result<Vec<sys::BuildQueueEntry>, Error> {
        unsafe {
            let query = sys::GetFullBuildQueueQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetFullBuildQueueResult>::zeroed();
            let func = self.api.GetFullBuildQueue.expect("GetFullBuildQueue function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.entries.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.entries as *const sys::BuildQueueEntry, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_real_build_queue(&self, unit_id: i32) -> Result<Vec<i32>, Error> {
        unsafe {
            let query = sys::GetRealBuildQueueQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetRealBuildQueueResult>::zeroed();
            let func = self.api.GetRealBuildQueue.expect("GetRealBuildQueue function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.unitDefIDs.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.unitDefIDs as *const i32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_unit_cmd_descs(&self, unit_id: i32) -> Result<Vec<sys::CommandDescription>, Error> {
        unsafe {
            let query = sys::GetUnitCmdDescsQuery {
                unitID: unit_id,
            };
            let mut result = MaybeUninit::<sys::GetUnitCmdDescsResult>::zeroed();
            let func = self.api.GetUnitCmdDescs.expect("GetUnitCmdDescs function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.cmdDescs.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.cmdDescs as *const sys::CommandDescription, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn find_unit_cmd_desc(&self, unit_id: i32, cmd_id: i32) -> Result<(i32, bool), Error> {
        unsafe {
            let query = sys::FindUnitCmdDescQuery {
                unitID: unit_id,
                cmdID: cmd_id,
            };
            let mut result = MaybeUninit::<sys::FindUnitCmdDescResult>::zeroed();
            let func = self.api.FindUnitCmdDesc.expect("FindUnitCmdDesc function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            let value = (
                result.cmdIndex,
                result.found,
            );
            Error::result_or(result.error, value)
        }
    }

    pub fn get_command_params(&self, command: &sys::CommandFFI) -> Result<Vec<f32>, Error> {
        unsafe {
            let query = sys::GetCommandParamsQuery {
                command: command as *const _,
            };
            let mut result = MaybeUninit::<sys::GetCommandParamsResult>::zeroed();
            let func = self.api.GetCommandParams.expect("GetCommandParams function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.params.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.params as *const f32, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn give_order(&self, cmd_id: i32, params: &[f32], options: u32, timeout: i32) -> Result<bool, Error> {
        unsafe {
            let query = sys::GiveOrderQuery {
                cmdID: cmd_id,
                params: params.as_ptr(),
                paramCount: params.len() as u32,
                options,
                timeout,
            };
            let mut result = MaybeUninit::<sys::GiveOrderResult>::zeroed();
            let func = self.api.GiveOrder.expect("GiveOrder function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn give_order_to_unit_map(&self, unit_ids: &[i32], cmd_id: i32, params: &[f32], options: u32, timeout: i32) -> Result<i32, Error> {
        unsafe {
            let query = sys::GiveOrderToUnitMapQuery {
                unitIDs: unit_ids.as_ptr(),
                count: unit_ids.len() as u32,
                cmdID: cmd_id,
                params: params.as_ptr(),
                paramCount: params.len() as u32,
                options,
                timeout,
            };
            let mut result = MaybeUninit::<sys::GiveOrderToUnitMapResult>::zeroed();
            let func = self.api.GiveOrderToUnitMap.expect("GiveOrderToUnitMap function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.unitsOrdered
            })
        }
    }

    pub fn give_order_array_to_unit_map(&self, unit_ids: &[i32], commands: &[sys::CommandFFI]) -> Result<i32, Error> {
        unsafe {
            let query = sys::GiveOrderArrayToUnitMapQuery {
                unitIDs: unit_ids.as_ptr(),
                unitCount: unit_ids.len() as u32,
                commands: commands.as_ptr(),
                commandCount: commands.len() as u32,
            };
            let mut result = MaybeUninit::<sys::GiveOrderArrayToUnitMapResult>::zeroed();
            let func = self.api.GiveOrderArrayToUnitMap.expect("GiveOrderArrayToUnitMap function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.unitsOrdered
            })
        }
    }

}
