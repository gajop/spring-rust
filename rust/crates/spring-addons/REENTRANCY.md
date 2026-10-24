# Addons and the scoped SDK

Gadgets, widgets and unsynced addons use the same SDK on native and Wasm.
An addon receives game state and the current engine handle through `ctx`:

```rust,ignore
fn game_frame(&self, ctx: &AddonContext<'_, Global>, frame: i32) {
    let units = ctx.spring().units().get_all_units()?;
    // Borrow only the game state needed for this operation.
}
```

`AddonContext<'_, G>` defaults to `RulesSynced`. Widgets use
`AddonContext<'_, G, Ui>`; unsynced rules use `RulesUnsynced`. The environment
can also be `All`, including native games and tools. Environment restrictions
still come from the SDK's compile-time bounds.

An engine call can immediately run another callin. The handler keeps shared
access to the global, and `Resource<T>` / `Resources` let unrelated state be
borrowed independently. A conflicting borrow panics with the active callin
names. An immediate-result callin runs immediately; its result is never queued
or replaced with an invented answer.

## Explicitly delayed work

`ctx.delay` runs owned work after the outermost addon callin returns. Its
argument is a fresh context, with the same global and a valid engine handle:

```rust,ignore
ctx.delay(move |ctx| {
    ctx.global().units.access_mut().remove(unit_id);
    // Engine calls use ctx.spring(), after the state borrow ends.
});
```

Delayed work is FIFO. Work added by nested callins joins that queue. It can
still cause immediate reentry; finish incompatible state borrows before making
an engine call.

`ctx.commands()` returns the module's SDK command queue. These commands run
after delayed addon work. A continuation runs immediately after its command,
before the next command; commands appended during dispatch join the end:

```rust,ignore
ctx.commands().queue(|spring| spring.units().get_all_units())
    .then(|spring, result| {
        // The owned result and a fresh engine handle are available here.
    });
```

Neither delayed work nor queued commands can capture a borrowed `Spring`.
Native captures and addon objects must be `Send` so the engine can move owned
module state from loading to game execution. Scoped engine handles remain
neither `Send` nor `Sync`. Handler entry is serialized by the SDK; nested
callins on the current thread remain immediate.

## Exports

Setup receives `(&mut Handler, &Spring)`. The existing three export macros
create SDK modules and export every callin represented by their traits.
`with_global` gives module-local state access; `with_addon_context` obtains a
fresh checked context for helpers and callbacks. SDK Rml data models own their
retained callbacks; there is no separate addon callback registry.

Native exports select `environment: All`. For example:

```rust,ignore
spring_addons::export_rules_gadgets! {
    state: Global,
    setup: setup,
    scratch: 65536,
    environment: spring::All,
}
```

The default environments of the Wasm macros remain `RulesSynced`, `Ui` and
`RulesUnsynced`. An initialized handler receives shutdown before its state is
dropped. Addon initialization (every addon's `init`) runs from `Module::start`,
when the engine loads the module and before its first callin, as Lua runs a
gadget's `Initialize`. It runs after the module has been installed, so an
initialization callout can immediately reenter its callins.
