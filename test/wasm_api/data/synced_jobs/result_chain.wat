;; This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
;; No GameFrame export: results must get the new budget independently, may call
;; ordinary synced imports, and may enqueue another ordered next-frame batch.
(module
  (import "spring:jobs" "submit" (func $submit (param i32 i32 i32 i32) (result i64)))
  (import "spring:game" "get-game-frame" (func $get_frame (param i32 i32) (result i32)))
  (memory (export "memory") 2 2)
  (global $burn_value (mut i32) (i32.const 0))
  (global $last (mut i32) (i32.const 0))
  (global $delivered (mut i32) (i32.const 0))
  (data (i32.const 256) "work")
  (func (export "SPRING_ENV_MASK") (result i32) i32.const 1)
  (func (export "SPRING_JOB_INPUT_BUFFER") (result i32) i32.const 1024)
  (func (export "SPRING_JOB_INPUT_CAPACITY") (result i32) i32.const 8)
  (func (export "SPRING_JOB_RESULT_BUFFER") (result i32) i32.const 8192)
  (func (export "SPRING_JOB_RESULT_CAPACITY") (result i32) i32.const 8)
  (func $assert (param $ok i32) local.get $ok i32.eqz if unreachable end)
  (func $frame (result i32)
    i32.const 0 i32.const 16 call $get_frame i32.eqz call $assert
    i32.const 16 i32.load i32.const 20 i32.load i32.const 16 i32.shl i32.or)
  (func (export "work") (param $ptr i32) (param $len i32) (result i64)
    local.get $ptr i32.const 1024 i32.eq call $assert
    local.get $len i32.const 8 i32.eq call $assert
    i64.const 34359739392)
  ;; Over 600 frames this exceeds one billion guest instructions. Without
  ;; the new-frame reset for a module lacking GameFrame, this chain must fault.
  (func $burn (param $left i32)
    loop $again
      global.get $burn_value i32.const 1664525 i32.mul i32.const 1013904223 i32.add global.set $burn_value
      local.get $left i32.const 1 i32.sub local.tee $left br_if $again
    end
    i32.const 32 global.get $burn_value i32.store)
  (func $enqueue (param $frame i32) (param $slot i32) (local $packed i64)
    i32.const 1024 local.get $frame i32.store
    i32.const 1028 local.get $slot i32.store
    i32.const 256 i32.const 4 i32.const 1024 i32.const 8 call $submit local.set $packed
    local.get $packed i64.const 32 i64.shr_u i64.eqz call $assert
    local.get $packed i32.wrap_i64
    local.get $frame i32.const 2 i32.mul local.get $slot i32.add i32.const 1 i32.add i32.eq call $assert)
  (func (export "spring:callin/game-frame-post") (param $frame i32)
    call $frame local.get $frame i32.eq call $assert
    local.get $frame i32.eqz
    if
      i32.const 0 i32.const 0 call $enqueue
      i32.const 0 i32.const 1 call $enqueue
    else
      global.get $delivered i32.const 2 i32.eq call $assert
      i32.const 0 global.set $delivered
    end)
  (func (export "spring:jobs/result") (param $id i32) (param $status i32) (param $ptr i32) (param $len i32)
    (local $now i32) (local $slot i32)
    local.get $status i32.eqz call $assert
    local.get $len i32.const 8 i32.eq call $assert
    local.get $id global.get $last i32.const 1 i32.add i32.eq call $assert
    call $frame local.set $now
    local.get $ptr i32.load local.get $now i32.const 1 i32.sub i32.eq call $assert
    local.get $id i32.const 1 i32.sub i32.const 1 i32.and local.set $slot
    local.get $ptr i32.load offset=4 local.get $slot i32.eq call $assert
    i32.const 500000 call $burn
    local.get $id global.set $last
    global.get $delivered i32.const 1 i32.add global.set $delivered
    local.get $now local.get $slot call $enqueue))
