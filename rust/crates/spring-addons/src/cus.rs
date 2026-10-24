//! The gadget that gives units their scripts, as Lua's `unit_script.lua` does.

use alloc::format;
use spring::UnitId;
use spring::capabilities::Synced;
use spring::cus::host::{CusHost, UnitDefScript};

use crate::rules::Gadget;
use crate::runtime::AddonContext;

/// Attaches the scripts of a module's [`CusHost`] and wakes them each frame.
///
/// ```rust,ignore
/// spring_addons::export_rules_gadgets! { state: Global, setup: setup, scratch: 65536, cus: CusHost<MyScript> }
///
/// fn setup(handler: &mut GadgetHandler<Global>, _: &Spring<'_, RulesSynced>) {
///     register_gadgets!(handler, [Spawner, CusHostGadget::new(with_cus_host)]);
/// }
/// ```
///
/// `with_cus_host` is defined by `export_rules_gadgets!` when it has a `cus:` host.
///
/// - `init` attaches a script to every unit that already exists (a rules reload, or units
///   made before the rules loaded);
/// - `unit_created` attaches the new unit's script;
/// - `game_frame` wakes the sleeping script tasks ([`CusHost::game_frame`]), so they resume
///   where Lua's do: in `GameFrame`, before the units update. Register this gadget after the
///   gadgets that create or order units in a frame, as `unit_script.lua` runs late.
///
/// A destroyed unit's script is detached by the host when the engine detaches it.
pub struct CusHostGadget<H: 'static> {
    host: WithHost<H>,
}

/// `with_cus_host`, as `export_rules_gadgets!` defines it.
pub type WithHost<H> = fn(&mut dyn FnMut(&H));

impl<H: 'static> CusHostGadget<H> {
    pub const fn new(host: WithHost<H>) -> Self {
        Self { host }
    }
}

impl<G, E, S, Shared> Gadget<G, E> for CusHostGadget<CusHost<S, Shared, E>>
where
    E: Synced + 'static,
    S: UnitDefScript<Shared, E> + 'static,
    Shared: 'static,
{
    fn name(&self) -> &'static str {
        "CUS Host"
    }

    fn init(&self, ctx: &AddonContext<'_, G, E>) {
        (self.host)(&mut |host| {
            if let Err(error) = host.attach_existing(ctx.spring()) {
                crate::log::error(&format!(
                    "CUS Host: attaching existing units' scripts failed: error {}",
                    error.code
                ));
            }
        });
    }

    fn unit_created(
        &self,
        ctx: &AddonContext<'_, G, E>,
        unit: i32,
        def: i32,
        _team: i32,
        _builder: i32,
    ) {
        (self.host)(&mut |host| {
            if let Err(error) = host.unit_created(ctx.spring(), UnitId(unit), def) {
                crate::log::error(&format!(
                    "CUS Host: attaching unit {unit}'s script failed: error {}",
                    error.code
                ));
            }
        });
    }

    fn game_frame(&self, _ctx: &AddonContext<'_, G, E>, frame: i32) {
        (self.host)(&mut |host| host.game_frame(frame));
    }
}
