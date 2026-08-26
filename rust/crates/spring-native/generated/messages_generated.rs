impl<'a> Messages<'a> {
    pub fn echo(&self, message: &str, rest: &str) -> Result<bool, Error> {
        unsafe {
            let message_cstr = std::ffi::CString::new(message).map_err(|_| Error::invalid_argument("message"))?;
            let rest_cstr = std::ffi::CString::new(rest).map_err(|_| Error::invalid_argument("rest"))?;
            let query = sys::EchoQuery {
                message: message_cstr.as_ptr(),
                rest: rest_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::EchoResult>::zeroed();
            let func = self.api.Echo.expect("Echo function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn log(&self, section: &str, level: i32, message: &str) -> Result<bool, Error> {
        unsafe {
            let section_cstr = std::ffi::CString::new(section).map_err(|_| Error::invalid_argument("section"))?;
            let message_cstr = std::ffi::CString::new(message).map_err(|_| Error::invalid_argument("message"))?;
            let query = sys::LogQuery {
                section: section_cstr.as_ptr(),
                level,
                message: message_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::LogResult>::zeroed();
            let func = self.api.Log.expect("Log function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn send_message(&self, message: &str) -> Result<bool, Error> {
        unsafe {
            let message_cstr = std::ffi::CString::new(message).map_err(|_| Error::invalid_argument("message"))?;
            let query = sys::SendMessageQuery {
                message: message_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SendMessageResult>::zeroed();
            let func = self.api.SendMessage.expect("SendMessage function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn send_message_to_player(&self, player_id: i32, message: &str) -> Result<bool, Error> {
        unsafe {
            let message_cstr = std::ffi::CString::new(message).map_err(|_| Error::invalid_argument("message"))?;
            let query = sys::SendMessageToPlayerQuery {
                playerID: player_id,
                message: message_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SendMessageToPlayerResult>::zeroed();
            let func = self.api.SendMessageToPlayer.expect("SendMessageToPlayer function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn send_message_to_team(&self, team_id: i32, message: &str) -> Result<bool, Error> {
        unsafe {
            let message_cstr = std::ffi::CString::new(message).map_err(|_| Error::invalid_argument("message"))?;
            let query = sys::SendMessageToTeamQuery {
                teamID: team_id,
                message: message_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SendMessageToTeamResult>::zeroed();
            let func = self.api.SendMessageToTeam.expect("SendMessageToTeam function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn send_message_to_ally_team(&self, ally_team_id: i32, message: &str) -> Result<bool, Error> {
        unsafe {
            let message_cstr = std::ffi::CString::new(message).map_err(|_| Error::invalid_argument("message"))?;
            let query = sys::SendMessageToAllyTeamQuery {
                allyTeamID: ally_team_id,
                message: message_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SendMessageToAllyTeamResult>::zeroed();
            let func = self.api.SendMessageToAllyTeam.expect("SendMessageToAllyTeam function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn send_message_to_spectators(&self, message: &str) -> Result<bool, Error> {
        unsafe {
            let message_cstr = std::ffi::CString::new(message).map_err(|_| Error::invalid_argument("message"))?;
            let query = sys::SendMessageToSpectatorsQuery {
                message: message_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SendMessageToSpectatorsResult>::zeroed();
            let func = self.api.SendMessageToSpectators.expect("SendMessageToSpectators function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn send_public_chat(&self, message: &str) -> Result<bool, Error> {
        unsafe {
            let message_cstr = std::ffi::CString::new(message).map_err(|_| Error::invalid_argument("message"))?;
            let query = sys::SendPublicChatQuery {
                message: message_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SendPublicChatResult>::zeroed();
            let func = self.api.SendPublicChat.expect("SendPublicChat function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn send_ally_chat(&self, message: &str) -> Result<bool, Error> {
        unsafe {
            let message_cstr = std::ffi::CString::new(message).map_err(|_| Error::invalid_argument("message"))?;
            let query = sys::SendAllyChatQuery {
                message: message_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SendAllyChatResult>::zeroed();
            let func = self.api.SendAllyChat.expect("SendAllyChat function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn send_spectator_chat(&self, message: &str) -> Result<bool, Error> {
        unsafe {
            let message_cstr = std::ffi::CString::new(message).map_err(|_| Error::invalid_argument("message"))?;
            let query = sys::SendSpectatorChatQuery {
                message: message_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SendSpectatorChatResult>::zeroed();
            let func = self.api.SendSpectatorChat.expect("SendSpectatorChat function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn send_private_chat(&self, message: &str, player_id: i32) -> Result<bool, Error> {
        unsafe {
            let message_cstr = std::ffi::CString::new(message).map_err(|_| Error::invalid_argument("message"))?;
            let query = sys::SendPrivateChatQuery {
                message: message_cstr.as_ptr(),
                playerID: player_id,
            };
            let mut result = MaybeUninit::<sys::SendPrivateChatResult>::zeroed();
            let func = self.api.SendPrivateChat.expect("SendPrivateChat function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn send_commands(&self, command: &str, rest: &str) -> Result<bool, Error> {
        unsafe {
            let command_cstr = std::ffi::CString::new(command).map_err(|_| Error::invalid_argument("command"))?;
            let rest_cstr = std::ffi::CString::new(rest).map_err(|_| Error::invalid_argument("rest"))?;
            let query = sys::SendCommandsQuery {
                command: command_cstr.as_ptr(),
                rest: rest_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SendCommandsResult>::zeroed();
            let func = self.api.SendCommands.expect("SendCommands function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn send_lua_menu_msg(&self, message: &str) -> Result<bool, Error> {
        unsafe {
            let message_cstr = std::ffi::CString::new(message).map_err(|_| Error::invalid_argument("message"))?;
            let query = sys::SendLuaMenuMsgQuery {
                message: message_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SendLuaMenuMsgResult>::zeroed();
            let func = self.api.SendLuaMenuMsg.expect("SendLuaMenuMsg function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn send_skirmish_aimessage(&self, ai_id: i32, message: &str) -> Result<bool, Error> {
        unsafe {
            let message_cstr = std::ffi::CString::new(message).map_err(|_| Error::invalid_argument("message"))?;
            let query = sys::SendSkirmishAIMessageQuery {
                aiID: ai_id,
                message: message_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SendSkirmishAIMessageResult>::zeroed();
            let func = self.api.SendSkirmishAIMessage.expect("SendSkirmishAIMessage function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn send_lua_uimsg(&self, message: &str, mode: &str) -> Result<bool, Error> {
        unsafe {
            let message_cstr = std::ffi::CString::new(message).map_err(|_| Error::invalid_argument("message"))?;
            let mode_cstr = std::ffi::CString::new(mode).map_err(|_| Error::invalid_argument("mode"))?;
            let query = sys::SendLuaUIQuery {
                message: message_cstr.as_ptr(),
                mode: mode_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SendLuaUIResult>::zeroed();
            let func = self.api.SendLuaUIMsg.expect("SendLuaUIMsg function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn send_lua_gaia_msg(&self, message: &str) -> Result<bool, Error> {
        unsafe {
            let message_cstr = std::ffi::CString::new(message).map_err(|_| Error::invalid_argument("message"))?;
            let query = sys::SendLuaGaiaQuery {
                message: message_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SendLuaGaiaResult>::zeroed();
            let func = self.api.SendLuaGaiaMsg.expect("SendLuaGaiaMsg function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn send_lua_rules_msg(&self, message: &str) -> Result<bool, Error> {
        unsafe {
            let message_cstr = std::ffi::CString::new(message).map_err(|_| Error::invalid_argument("message"))?;
            let query = sys::SendLuaRulesQuery {
                message: message_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SendLuaRulesResult>::zeroed();
            let func = self.api.SendLuaRulesMsg.expect("SendLuaRulesMsg function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn send_to_unsynced(&self, message: &str) -> Result<bool, Error> {
        unsafe {
            let message_cstr = std::ffi::CString::new(message).map_err(|_| Error::invalid_argument("message"))?;
            let query = sys::SendToUnsyncedQuery {
                message: message_cstr.as_ptr(),
            };
            let mut result = MaybeUninit::<sys::SendToUnsyncedResult>::zeroed();
            let func = self.api.SendToUnsynced.expect("SendToUnsynced function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.success
            })
        }
    }

    pub fn get_console_buffer(&self, max_lines: u32) -> Result<Vec<sys::ConsoleEntry>, Error> {
        unsafe {
            let query = sys::GetConsoleBufferQuery {
                maxLines: max_lines,
            };
            let mut result = MaybeUninit::<sys::GetConsoleBufferResult>::zeroed();
            let func = self.api.GetConsoleBuffer.expect("GetConsoleBuffer function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    let slice = if result.count == 0 || result.entries.is_null() {
                        &[]
                    } else {
                        slice::from_raw_parts(result.entries as *const sys::ConsoleEntry, result.count as usize)
                    };
                    slice.to_vec()
                }
            })
        }
    }

    pub fn get_current_tooltip(&self) -> Result<Option<String>, Error> {
        unsafe {
            let query = sys::GetCurrentTooltipQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::GetCurrentTooltipResult>::zeroed();
            let func = self.api.GetCurrentTooltip.expect("GetCurrentTooltip function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                {
                    if result.tooltip.is_null() {
                        None
                    } else {
                        Some(CStr::from_ptr(result.tooltip).to_string_lossy().into_owned())
                    }
                }
            })
        }
    }

    pub fn is_user_writing(&self) -> Result<bool, Error> {
        unsafe {
            let query = sys::IsUserWritingQuery {
                _unused: 0,
            };
            let mut result = MaybeUninit::<sys::IsUserWritingResult>::zeroed();
            let func = self.api.IsUserWriting.expect("IsUserWriting function pointer must be initialized");
            func(&query, result.as_mut_ptr());
            let result = result.assume_init();
            Error::result_or(result.error, {
                result.writing
            })
        }
    }

}
