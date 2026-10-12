;; This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
;; Same module bytes in both runs: first guest-state divergence is frame 17.
(module
  (import "spring:desync" "get-timer" (func $timer (result i64)))
  (memory (export "memory") 1 1)
  (func (export "SPRING_ENV_MASK") (result i32) i32.const 1)
  (func (export "spring:callin/game-frame") (param $frame i32)
    i32.const 8 local.get $frame i32.store
    local.get $frame i32.const 17 i32.eq
    if i32.const 16 call $timer i64.store end)
)
