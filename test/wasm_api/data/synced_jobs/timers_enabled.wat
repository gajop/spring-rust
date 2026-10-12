;; This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
;; Single-process diagnostic probe: enabled clocks intentionally are not synced.
(module
  (import "spring:desync" "try-get-timer" (func $timer (param i32) (result i32)))
  (import "spring:desync" "try-get-timer-micros" (func $micros (param i32) (result i32)))
  (import "spring:messages" "echo" (func $echo (param i32 i32 i32 i32) (result i64)))
  (memory (export "memory") 1 1)
  (data (i32.const 256) "ENGINE_NOTES_TIMERS_ENABLED_PASS")
  (func (export "SPRING_ENV_MASK") (result i32) i32.const 1)
  (func $assert (param $value i32) local.get $value i32.eqz if unreachable end)
  (func (export "spring:callin/game-frame") (param $frame i32)
    local.get $frame if return end
    i32.const 32 call $timer i32.eqz call $assert
    i32.const 32 i64.load i64.const 0 i64.gt_u call $assert
    i32.const 48 call $micros i32.eqz call $assert
    i32.const 48 i64.load i64.const 0 i64.gt_u call $assert
    i32.const 48 i64.load i32.const 32 i64.load i64.const 1000 i64.mul i64.ge_u call $assert
    i32.const 65533 call $timer i32.const 2 i32.eq call $assert
    i32.const 65533 call $micros i32.const 2 i32.eq call $assert
    i32.const 256 i32.const 32 i32.const 0 i32.const 0 call $echo drop)
)
