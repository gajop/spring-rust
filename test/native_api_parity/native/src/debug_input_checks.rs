use super::*;
use crate::support::*;

// SDL1.2 SDLK_F1. This intentionally differs from SDL2's SDLK_F1 value so
// both APIs have to agree on the public keysym namespace.
const READBACK_KEY_CODE: i32 = 282;

impl NativeApiParity {
    pub(crate) fn check_debug_input_key_readback(&mut self, message: &Value) -> Result<(), String> {
        let key_code = i32_field(message, "keyCode")?;
        if key_code != READBACK_KEY_CODE {
            return Err(format!(
                "debug input key readback used unexpected keycode {key_code}"
            ));
        }

        self.interface
            .debug_input()
            .emulate_key(READBACK_KEY_CODE, true)
            .map_err(|err| format!("DebugInput.emulate_key(press) failed: {err:?}"))?;

        let pressed = self
            .interface
            .input()
            .get_key_state(READBACK_KEY_CODE)
            .map_err(|err| format!("Input.get_key_state failed: {err:?}"));

        let release = self
            .interface
            .debug_input()
            .emulate_key(READBACK_KEY_CODE, false)
            .map_err(|err| format!("DebugInput.emulate_key(release) failed: {err:?}"));

        let pressed = pressed?;
        release?;
        if !pressed {
            return Err(format!(
                "Input.get_key_state({READBACK_KEY_CODE}) did not observe the emulated press"
            ));
        }

        Ok(())
    }

    pub(crate) fn check_debug_input_mouse_readback(
        &mut self,
        message: &Value,
    ) -> Result<(), String> {
        let x = i32_field(message, "x")?;
        let y = i32_field(message, "y")?;

        self.interface
            .debug_input()
            .emulate_mouse_move(x, y)
            .map_err(|err| format!("DebugInput.emulate_mouse_move failed: {err:?}"))?;

        let state = self
            .interface
            .input()
            .get_mouse_state()
            .map_err(|err| format!("Input.get_mouse_state failed: {err:?}"))?;

        // Input.get_mouse_state follows Spring's existing screen API and
        // reports y from the bottom of the view. DebugInput deliberately
        // accepts the SDL top-left coordinate and passes it through without
        // flipping it, so convert only for this readback assertion.
        let view = self
            .interface
            .display()
            .get_view_geometry()
            .map_err(|err| format!("Display.get_view_geometry failed: {err:?}"))?;
        let expected_y = view.viewSizeY as f32 - y as f32 - 1.0;

        if state.x != x as f32 || state.y != expected_y {
            return Err(format!(
                "Input.get_mouse_state returned ({}, {}), expected ({x}, {expected_y})",
                state.x, state.y,
            ));
        }

        Ok(())
    }
}
