use super::*;
use crate::support::*;

impl NativeApiParity {
    pub(crate) fn check_player_value(&mut self, message: &Value, label: &str) -> Result<(), String> {
        match base_test_name(label) {
            "get_local_player_id" => {
                let native = self.interface.player().get_local_player_id()
                    .map_err(|err| format!("get_local_player_id() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "playerID", native)
            }
            "get_local_team_id" => {
                let native = self.interface.player().get_local_team_id()
                    .map_err(|err| format!("get_local_team_id() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "teamID", native)
            }
            "get_local_ally_team_id" => {
                let native = self.interface.player().get_local_ally_team_id()
                    .map_err(|err| format!("get_local_ally_team_id() failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "allyTeamID", native)
            }
            "get_spectating_state" => {
                let native = self.interface.player().get_spectating_state()
                    .map_err(|err| format!("get_spectating_state() failed: {err:?}"))?;
                self.same_bool_if_present(label, message, "spectating", native)
            }
            "get_player_roster_count" => {
                let sort_mode = i32_field(message, "sortMode")?;
                let show_pathing_players = bool_field(message, "showPathingPlayers")?;
                let native = self.interface.player().get_player_roster(sort_mode, show_pathing_players)
                    .map_err(|err| format!("get_player_roster({sort_mode}, {show_pathing_players}) failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "count", native.len() as i32)
            }
            "get_player_statistics" => {
                let player_id = i32_field(message, "playerID")?;
                let native = self.interface.player().get_player_statistics(player_id)
                    .map_err(|err| format!("get_player_statistics({player_id}) failed: {err:?}"))?;
                self.same_i32_if_present(label, message, "mousePixels", native.mousePixels)?;
                self.same_i32_if_present(label, message, "mouseClicks", native.mouseClicks)?;
                self.same_i32_if_present(label, message, "keyPresses", native.keyPresses)?;
                self.same_i32_if_present(label, message, "unitCommands", native.unitCommands as i32)
            }
            "get_player_traffic_missing_packet" => {
                let player_id = i32_field(message, "playerID")?;
                let packet_id = i32_field(message, "packetID")?;
                let native = self.interface.player().get_player_traffic(player_id, packet_id)
                    .map_err(|err| format!("get_player_traffic({player_id}, {packet_id}) failed: {err:?}"))?;
                let traffic = native
                    .first()
                    .map(|traffic| if traffic.bytesSent == u32::MAX { -1 } else { traffic.bytesSent as i32 })
                    .unwrap_or(-1);
                self.same_i32_if_present(label, message, "traffic", traffic)
            }
            _ => Err(format!("unsupported player check `{label}`")),
        }
    }
}
