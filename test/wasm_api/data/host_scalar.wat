;; This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
;; Small core-Wasm host import fixture used by TestWasmInterface.
(module
  (import "spring" "add-i32" (func $add (param i32) (result i32)))
  (func (export "run") (param i32) (result i32)
    local.get 0
    call $add))
