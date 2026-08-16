;; This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
;; Core-Wasm smoke fixture for the pinned Wasmtime CLI. In-engine loading uses
;; the C API backend and the separate host_scalar.wat import fixture.
(module
  (func (export "scalar") (param i32) (result i32)
    local.get 0
    i32.const 1
    i32.add))
(invoke "scalar" (i32.const 41))
