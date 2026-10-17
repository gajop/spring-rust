/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use super::{Pending, Queue};
use crate::{Float3, Result, capabilities::Synced, unit_control};

// Queue the same engine operations as the former commands helpers. Every
// borrowed input is copied now; execution and continuations receive Spring
// from the queue's owning callin rather than retaining a scope.
impl<E: Synced> Queue<E> {
    pub fn create_unit(
        &self,
        definition: &unit_control::DefRef,
        position: Float3,
        facing: i32,
        team: i32,
        options: unit_control::CreateUnitOptions,
    ) -> Pending<E, Result<i32>> {
        let definition = definition.clone();
        self.queue(move |spring| {
            spring
                .unit_control()
                .create_unit(&definition, position, facing, team, options)
        })
    }

    pub fn destroy_unit(
        &self,
        unit: i32,
        options: unit_control::DestroyUnitOptions,
    ) -> Pending<E, Result<bool>> {
        self.queue(move |spring| spring.unit_control().destroy_unit(unit, options))
    }

    pub fn transfer_unit(
        &self,
        unit: i32,
        team: i32,
        given: bool,
        adjust_unit_limit: bool,
    ) -> Pending<E, Result<bool>> {
        self.queue(move |spring| {
            spring
                .unit_control()
                .transfer_unit(unit, team, given, adjust_unit_limit)
        })
    }

    pub fn add_unit_damage(
        &self,
        unit: i32,
        damage: f32,
        paralyze_time: f32,
        weapon: i32,
        attacker: i32,
        impulse: Float3,
    ) -> Pending<E, Result<bool>> {
        self.queue(move |spring| {
            spring.unit_control().add_unit_damage(
                unit,
                damage,
                paralyze_time,
                weapon,
                attacker,
                impulse,
            )
        })
    }

    pub fn set_unit_health(
        &self,
        unit: i32,
        value: unit_control::UnitHealthValue,
    ) -> Pending<E, Result<bool>> {
        self.queue(move |spring| spring.unit_control().set_unit_health(unit, value))
    }

    pub fn set_unit_position(&self, unit: i32, position: Float3) -> Pending<E, Result<bool>> {
        self.queue(move |spring| spring.unit_control().set_unit_position(unit, position))
    }

    pub fn give_order_to_unit(
        &self,
        unit: i32,
        command: i32,
        params: &[f32],
        options: u32,
        timeout: i32,
    ) -> Pending<E, Result<bool>> {
        let params = params.to_vec();
        self.queue(move |spring| {
            spring
                .unit_control()
                .give_order_to_unit(unit, command, &params, options, timeout)
        })
    }

    pub fn give_order_to_unit_array(
        &self,
        units: &[i32],
        command: i32,
        params: &[f32],
        options: u32,
        timeout: i32,
    ) -> Pending<E, Result<bool>> {
        let units = units.to_vec();
        let params = params.to_vec();
        self.queue(move |spring| {
            spring
                .unit_control()
                .give_order_to_unit_array(&units, command, &params, options, timeout)
        })
    }

    pub fn unit_attach(
        &self,
        transporter: i32,
        transportee: i32,
        piece: i32,
    ) -> Pending<E, Result<bool>> {
        self.queue(move |spring| {
            spring
                .unit_control()
                .unit_attach(transporter, transportee, piece)
        })
    }

    pub fn unit_detach(&self, transportee: i32) -> Pending<E, Result<bool>> {
        self.queue(move |spring| spring.unit_control().unit_detach(transportee))
    }

    pub fn send_to_unsynced(&self, message: &str) -> Pending<E, Result<bool>> {
        let message = message.to_owned();
        self.queue(move |spring| spring.messages().send_to_unsynced(&message))
    }
}
