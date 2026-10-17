/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use crate::{Result, Spring, capabilities::Game};

pub use spring::default_interface::{
    ALL, CLOCK, COMMAND_MENU, CONSOLE, END_GRAPH, FPS, MAP_BORDER, MINIMAP, PLAYER_INFO,
    RESOURCE_BAR, SPEED, TOOLTIP, UNIT_ICONS,
};

pub struct DefaultInterface<'call, E: Game> {
    spring: Spring<'call, E>,
}

impl<E: Game> Spring<'_, E> {
    pub fn default_interface(&self) -> DefaultInterface<'_, E> {
        DefaultInterface {
            spring: self.reborrow(),
        }
    }
}

impl<E: Game> DefaultInterface<'_, E> {
    pub fn hide(&self, parts: u32) -> Result<u32> {
        self.spring
            .unsynced_ctrl()
            .set_default_interface_visible(parts, false)
    }

    pub fn show(&self, parts: u32) -> Result<u32> {
        self.spring
            .unsynced_ctrl()
            .set_default_interface_visible(parts, true)
    }
}
