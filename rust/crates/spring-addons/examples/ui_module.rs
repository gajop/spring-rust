/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

use spring_addons::reexports::{Spring, Ui};
use spring_addons::{AddonContext, Widget, WidgetHandler};

#[derive(Default)]
pub struct State;
struct Panel;
impl Widget<State> for Panel {
    fn name(&self) -> &'static str {
        "ScopedPanel"
    }
    fn draw_screen(&self, ctx: &AddonContext<'_, State, Ui>, _: i32, _: i32) {
        let spring = ctx.spring();
        spring.gfx().color(0.1, 0.2, 0.3, 1.0).expect("panel color");
        spring
            .gfx()
            .rect(0.0, 0.0, 100.0, 50.0)
            .expect("panel rectangle");
    }
}
fn setup(handler: &mut WidgetHandler<State>, _: &Spring<'_, Ui>) {
    handler.add(Box::new(Panel));
}
spring_addons::export_ui_widgets! { state: State, setup: setup, scratch: 4096 }
