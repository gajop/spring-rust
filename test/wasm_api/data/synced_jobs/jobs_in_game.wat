;; This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
;; Added only to isolated replay snapshots. Eight jobs per frame are checked
;; against this same work export called inline, with a deliberately uneven cost.
(module
  (import "spring:jobs" "submit" (func $submit (param i32 i32 i32 i32) (result i64)))
  (import "spring:jobs" "submit-to" (func $submit_to (param i32 i32 i32 i32 i32 i32) (result i64)))
  (memory (export "memory") 2 2)
  (global $calls (mut i32) (i32.const 0))
  (data (i32.const 256) "work")
  (data (i32.const 512) "sync-jobs-test")
  (data (i32.const 544) "shipgame-look")
  (data (i32.const 0) "\ff\ff\ff\ff")
  (func (export "SPRING_ENV_MASK") (result i32) i32.const 1)
  (func (export "SPRING_JOB_INPUT_BUFFER") (result i32) i32.const 1024)
  (func (export "SPRING_JOB_INPUT_CAPACITY") (result i32) i32.const 65536)
  (func (export "SPRING_JOB_RESULT_BUFFER") (result i32) i32.const 8192)
  (func (export "SPRING_JOB_RESULT_CAPACITY") (result i32) i32.const 4096)
  (func $assert (param $ok i32) local.get $ok i32.eqz if unreachable end)
  (func $work (export "work") (param $ptr i32) (param $len i32) (result i64)
    (local $value i32) (local $remaining i32)
    local.get $ptr i32.load local.set $value
    local.get $ptr i32.load offset=4 local.set $remaining
    block $done
      loop $loop
        local.get $remaining i32.eqz br_if $done
        local.get $value i32.const 1664525 i32.mul i32.const 1013904223 i32.add local.set $value
        local.get $remaining i32.const 1 i32.sub local.set $remaining
        br $loop
      end
    end
    i32.const 32 local.get $value i32.store
    global.get $calls i32.const 1 i32.add global.set $calls
    i32.const 36 global.get $calls i32.store
    i32.const 40 local.get $ptr i32.load offset=8 i32.store
    i32.const 44 local.get $ptr i32.load f32.convert_i32_u f32.const 1.25 f32.mul f32.const 0.1 f32.add f32.store
    i64.const 68719476768)
  (func (export "spring:callin/game-frame") (param $frame i32)
    (local $i i32) (local $id i64)
    local.get $frame i32.const 0 i32.gt_u
    if
      i32.const 8 i32.load i32.const 8 i32.eq call $assert
      i32.const 16 i32.load local.get $frame i32.const 8 i32.mul i32.eq call $assert
    end
    local.get $frame i32.eqz
    if
      ;; Invalid admission must not consume IDs or enter a worker queue.
      i32.const 544 i32.const 13 i32.const 256 i32.const 4 i32.const 1024 i32.const 12 call $submit_to
      i64.const 32 i64.shr_u i64.const 3 i64.eq call $assert
      i32.const 256 i32.const 0 i32.const 1024 i32.const 12 call $submit
      i64.const 32 i64.shr_u i64.const 1 i64.eq call $assert
      i32.const -4 i32.const 8 i32.const 1024 i32.const 12 call $submit
      i64.const 32 i64.shr_u i64.const 2 i64.eq call $assert
    end
    i32.const 0 local.get $frame i32.store
    i32.const 8 i32.const 0 i32.store
    loop $submit_loop
      i32.const 1024 local.get $frame i32.const 8 i32.mul local.get $i i32.add i32.store
      i32.const 1028
      local.get $i i32.eqz
      if (result i32) i32.const 200000 else i32.const 5000 end
      i32.store
      i32.const 1032 local.get $frame i32.store
      i32.const 1024 i32.const 12 call $work drop
      i32.const 128 local.get $i i32.const 4 i32.mul i32.add i32.const 32 i32.load i32.store
      i32.const 160 local.get $i i32.const 4 i32.mul i32.add i32.const 44 i32.load i32.store
      local.get $i i32.const 4 i32.lt_u
      if (result i64)
        i32.const 256 i32.const 4 i32.const 1024 i32.const 12 call $submit
      else
        i32.const 512 i32.const 14 i32.const 256 i32.const 4 i32.const 1024 i32.const 12 call $submit_to
      end
      local.set $id
      local.get $id i64.const 32 i64.shr_u i64.eqz call $assert
      local.get $id i32.wrap_i64 local.get $frame i32.const 8 i32.mul local.get $i i32.add i32.const 1 i32.add i32.eq call $assert
      ;; Mutate scratch after submission: worker input must already be copied.
      i32.const 1024 i32.const -1 i32.store
      local.get $i i32.const 1 i32.add local.tee $i i32.const 8 i32.lt_u br_if $submit_loop
    end)
  (func (export "spring:jobs/result") (param $id i32) (param $status i32) (param $ptr i32) (param $len i32)
    local.get $status i32.eqz call $assert
    local.get $len i32.const 16 i32.eq call $assert
    local.get $id i32.const 16 i32.load i32.const 1 i32.add i32.eq call $assert
    local.get $ptr i32.load offset=4 i32.const 1 i32.eq call $assert
    local.get $ptr i32.load offset=8 i32.const 0 i32.load i32.eq call $assert
    local.get $ptr i32.load
    i32.const 128 local.get $id i32.const 1 i32.sub i32.const 8 i32.rem_u i32.const 4 i32.mul i32.add i32.load
    i32.eq call $assert
    local.get $ptr i32.load offset=12
    i32.const 160 local.get $id i32.const 1 i32.sub i32.const 8 i32.rem_u i32.const 4 i32.mul i32.add i32.load
    i32.eq call $assert
    i32.const 16 local.get $id i32.store
    i32.const 8 i32.const 8 i32.load i32.const 1 i32.add i32.store
    i32.const 20 i32.const 20 i32.load local.get $ptr i32.load i32.xor i32.store)
)
