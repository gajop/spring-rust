# What guest authors see

The binding style is invisible to guests. The ABI choice is not. This is the
part of [abi_choice.md](abi_choice.md) that reaches users, and it is the main
non-performance argument in that decision.

## On the Component Model

A guest author's workflow:

1. Take the WIT from `rts/wasm/generated/wit/`
2. Run wit-bindgen, which generates the bindings
3. `cargo build --target wasm32-unknown-unknown`
4. Componentize with wit-component
5. Ship the `.wasm`

Steps 2 and 4 are Bytecode Alliance tools. We publish WIT and nothing else.
Rust, C, Go, C# and several other languages get bindings without any work from
us, and componentize-py and jco cover Python and JavaScript.

Every guest in the tree works this way today: `benchmark_guest`,
`parity_guest`, `aggregation_guest`, `allocator_guest` and `value_guest` all
call `wit_bindgen::generate!` on our WIT directly.

## On core wasm with our own layout

1. Take our layout specification
2. Use our bindings generator, or hand-write the import declarations
3. `cargo build --target wasm32-unknown-unknown`
4. No componentize step
5. Ship the `.wasm`

The componentize step disappearing is a genuine simplification, and core wasm
is the lowest common denominator, so languages wit-bindgen does not support can
still call us: Zig, AssemblyScript, plain clang without a component-capable
wasi-sdk, even hand-written `.wat`.

The cost is that step 2 becomes ours. Per language.

## Users still get types

The wire format is integers and pointers, but authors do not see them, because
hiding them is what the generator does. The same call, written by a guest
author, is unchanged:

```rust
let pos = units_info::get_unit_position(unit_id, units_info::GetUnitPositionOptions {
    mid_pos: false,
    aim_pos: false,
})?;
let sum = pos.x + pos.y + pos.z;
```

Records stay records, `Result` stays `Result`, `f32` stays `f32`. What changes
is the generated layer underneath, which nobody opens. Illustrative of a design
not yet written:

```rust
// generated, never hand-edited
#[link(wasm_import_module = "spring:units_info")]
extern "C" {
    fn get_unit_position(unit_id: i32, mid_pos: i32, aim_pos: i32, out: *mut Float3) -> i32;
}

pub fn get_unit_position(unit_id: i32, options: GetUnitPositionOptions)
    -> Result<Float3, SpringError>
{
    let mut out = Float3 { x: 0.0, y: 0.0, z: 0.0 };
    let code = unsafe {
        get_unit_position(unit_id, options.mid_pos as i32, options.aim_pos as i32, &mut out)
    };
    if code == 0 { Ok(out) } else { Err(SpringError { code }) }
}
```

Today the canonical ABI defines that lowering and wit-bindgen writes it. The
alternative is that we define it and we write it.

## Cost of a language backend

The structure would mirror wit-bindgen: one front end reading the interface
description, a backend per language. The front end exists already, since
`spring-native-codegen` reads `model.json` and emits WIT, C++ host adapters and
a Rust guest facade.

So the ABI is designed once, and each language costs roughly 600 to 1,200 lines
for its backend plus the runtime helpers for allocation and string encoding.
Recurring, and ours.

Hand-writing the import declarations remains a fallback for a language we have
not generated for. It is fine for scalars, and unpleasant for strings, lists
and variants, where the author has to allocate in the guest, pass a pointer and
a length, know who frees it, and read discriminants out of our specification.

## The question this comes down to

How many guest languages do we expect people to actually use?

If the honest answer is Rust plus maybe C, losing wit-bindgen costs little and
the specification is a one-time write. If the goal is that authors reach for
whatever language they know and it works, then the Component Model is carrying
weight that would transfer to us permanently.

There is also a direction-of-travel argument. The Component Model is where the
wasm ecosystem is heading, so staying on it means inheriting improvements,
while a private ABI means maintaining one against an ecosystem moving
elsewhere.

That is a product decision about who writes gadgets, not a technical one, and
it matters more than the difference between 4 ns and 37 ns.
