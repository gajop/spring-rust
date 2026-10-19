;; This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
;; Negative control: identical module bytes and unchanged linear memory.
;; With explicitly enabled synced timers, frame 7 changes a hidden global and
;; selects a private function-table entry. Memory-only comparison misses it.
(module
  (import "spring:desync" "get-timer" (func $timer (result i64)))
  (memory (export "memory") 1 1)
  (global $hidden (mut i64) (i64.const 0))
  (table 1 1 funcref)
  (func $a)
  (func $b)
  (elem declare func $a $b)
  (func (export "SPRING_ENV_MASK") (result i32) i32.const 1)
  (func (export "spring:callin/game-frame") (param $frame i32)
    local.get $frame i32.const 7 i32.eq
    if
      call $timer global.set $hidden
      i32.const 0
      global.get $hidden i64.const 1 i64.and i64.eqz
      if (result funcref) ref.func $a else ref.func $b end
      table.set 0
    end)
)
