;; Core-Wasm counterpart of the component bench guest: the same call loops
;; reached through a plain core import instead of the canonical ABI.
(module
  (import "spring" "add_i32" (func $add (param i32) (result i32)))

  (func (export "run_callout") (param $iters i32) (result i32)
    (local $acc i32)
    (local $i i32)
    (block $done
      (loop $loop
        (br_if $done (i32.ge_s (local.get $i) (local.get $iters)))
        (local.set $acc (call $add (local.get $acc)))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $loop)))
    (local.get $acc))

  (func (export "run_spin") (param $iters i32) (result i32)
    (local $acc i32)
    (local $i i32)
    (block $done
      (loop $loop
        (br_if $done (i32.ge_s (local.get $i) (local.get $iters)))
        (local.set $acc (i32.add (local.get $acc) (i32.const 1)))
        (local.set $i (i32.add (local.get $i) (i32.const 1)))
        (br $loop)))
    (local.get $acc))

  (func (export "noop") (param i32) (result i32)
    (local.get 0)))
