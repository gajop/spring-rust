;; This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
;; Timestamp markers for a fixed 1000-frame skirmish benchmark, with no sync log.
(module
  (import "spring:messages" "echo" (func $echo (param i32 i32 i32 i32) (result i64)))
  (memory (export "memory") 1 1)
  (data (i32.const 32) "WASM_HASH_COST_START")
  (data (i32.const 64) "WASM_HASH_COST_END")
  (func (export "SPRING_ENV_MASK") (result i32) i32.const 1)
  (func (export "spring:callin/game-frame") (param $frame i32)
    local.get $frame i32.eqz
    if i32.const 32 i32.const 20 i32.const 0 i32.const 0 call $echo drop end
    local.get $frame i32.const 1000 i32.eq
    if i32.const 64 i32.const 18 i32.const 0 i32.const 0 call $echo drop end)
)
