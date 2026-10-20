;; This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
;; Isolated real-import fixture: direct ordered return, mixed guest failures,
;; overlapping input/output, atomic descriptor rejection and deferred boundary.
(module
  (import "spring:jobs" "run-batch-to" (func $batch (param i32 i32 i32 i32 i32 i32 i32 i32) (result i64)))
  (import "spring:jobs" "submit" (func $submit (param i32 i32 i32 i32) (result i64)))
  (import "spring:messages" "echo" (func $echo (param i32 i32 i32 i32) (result i64)))
  (memory (export "memory") 2 2)
  (global $calls (mut i32) (i32.const 0))
  (global $delivered (mut i32) (i32.const 0))
  (data (i32.const 256) "work")
  (data (i32.const 320) "WASM_BATCH_IMPORT_PASSED")
  (data (i32.const 2048)
    "\01\00\00\00\06\00\00\00"
    "\08\00\00\00\0c\00\00\00\0c\00\00\00\40\0d\03\00"
    "\08\00\00\00\0c\00\00\00\ff\ff\ff\ff\00\00\00\00"
    "\08\00\00\00\0c\00\00\00\fe\ff\ff\ff\00\00\00\00"
    "\08\00\00\00\0c\00\00\00\fd\ff\ff\ff\00\00\00\00"
    "\08\00\00\00\0c\00\00\00\0d\00\00\00\03\00\00\00"
    "\08\00\00\00\0b\00\00\00\0e\00\00\00\00\00\00\00")
  (func (export "SPRING_ENV_MASK") (result i32) i32.const 1)
  (func (export "SPRING_JOB_INPUT_BUFFER") (result i32) i32.const 1024)
  (func (export "SPRING_JOB_INPUT_CAPACITY") (result i32) i32.const 16)
  (func (export "SPRING_JOB_RESULT_BUFFER") (result i32) i32.const 8192)
  (func (export "SPRING_JOB_RESULT_CAPACITY") (result i32) i32.const 16)
  (func $assert (param $ok i32) local.get $ok i32.eqz if unreachable end)
  (func $lcg (param $v i32) (param $n i32) (result i32)
    block $done loop $loop
      local.get $n i32.eqz br_if $done
      local.get $v i32.const 1664525 i32.mul i32.const 1013904223 i32.add local.set $v
      local.get $n i32.const 1 i32.sub local.set $n br $loop
    end end local.get $v)
  (func (export "work") (param $ptr i32) (param i32) (result i64)
    (local $v i32)
    local.get $ptr i32.load local.set $v
    local.get $v i32.const -1 i32.eq if unreachable end
    local.get $v i32.const -2 i32.eq if
      i32.const 320 i32.const 24 i32.const 0 i32.const 0 call $echo drop
    end
    local.get $v i32.const -3 i32.eq if i64.const -1 return end
    i32.const 32 local.get $v local.get $ptr i32.load offset=4 call $lcg i32.store
    global.get $calls i32.const 1 i32.add global.set $calls
    i32.const 36 global.get $calls i32.store
    i32.const 40 local.get $v i32.store
    i64.const 51539607584)
  (func $run (param $capacity i32) (result i64)
    i32.const 0 i32.const 0 i32.const 256 i32.const 4
    i32.const 2048 i32.const 104 i32.const 2048 local.get $capacity call $batch)
  (func (export "spring:callin/game-frame") (param $frame i32)
    (local $result i64)
    local.get $frame i32.eqz if
      ;; Bad capacity/version/ranges must not consume even one quota slot/ID.
      i32.const 126 call $run local.set $result
      local.get $result i64.const 32 i64.shr_u i64.const 9 i64.eq call $assert
      local.get $result i32.wrap_i64 i32.const 127 i32.eq call $assert
      i32.const 2048 i32.const 2 i32.store
      i32.const 127 call $run i64.const 32 i64.shr_u i64.const 1 i64.eq call $assert
      i32.const 2048 i32.const 1 i32.store
      i32.const 0 i32.const 0 i32.const 256 i32.const 4
      i32.const -4 i32.const 104 i32.const 2048 i32.const 127 call $batch
      i64.const 32 i64.shr_u i64.const 2 i64.eq call $assert
      ;; The output overlaps every copied input and is published after join.
      i32.const 127 call $run local.set $result
      local.get $result i64.const 32 i64.shr_u i64.eqz call $assert
      local.get $result i32.wrap_i64 i32.const 80 i32.eq call $assert
      i32.const 2048 i32.load i32.const 1 i32.eq call $assert
      i32.const 2052 i32.load i32.const 6 i32.eq call $assert
      i32.const 2056 i32.load i32.eqz call $assert
      i32.const 2060 i32.load i32.const 12 i32.eq call $assert
      i32.const 2064 i32.load i32.const 12 i32.const 200000 call $lcg i32.eq call $assert
      i32.const 2068 i32.load i32.const 1 i32.eq call $assert
      i32.const 2072 i32.load i32.const 12 i32.eq call $assert
      i32.const 2076 i32.load i32.const 2 i32.eq call $assert
      i32.const 2084 i32.load i32.const 2 i32.eq call $assert
      i32.const 2092 i32.load i32.const 4 i32.eq call $assert
      i32.const 2100 i32.load i32.eqz call $assert
      i32.const 2108 i32.load i32.const 13 i32.const 3 call $lcg i32.eq call $assert
      i32.const 2112 i32.load i32.const 1 i32.eq call $assert
      i32.const 2120 i32.load i32.const 4 i32.eq call $assert
      global.get $calls i32.eqz call $assert
      global.get $delivered i32.eqz call $assert
      ;; Ordinary imports and deferred submission still work in this callback.
      i32.const 1024 i32.const 17 i32.store
      i32.const 1028 i32.const 5 i32.store
      i32.const 256 i32.const 4 i32.const 1024 i32.const 8 call $submit
      i64.const 1 i64.eq call $assert
    end
    local.get $frame i32.const 1 i32.eq if
      global.get $delivered i32.const 1 i32.eq call $assert
      i32.const 320 i32.const 24 i32.const 0 i32.const 0 call $echo
      i64.const 32 i64.shr_u i64.eqz call $assert
    end)
  (func (export "spring:jobs/result") (param $id i32) (param $status i32) (param $ptr i32) (param $len i32)
    local.get $id i32.const 1 i32.eq call $assert
    local.get $status i32.eqz call $assert
    local.get $len i32.const 12 i32.eq call $assert
    local.get $ptr i32.load i32.const 17 i32.const 5 call $lcg i32.eq call $assert
    local.get $ptr i32.load offset=4 i32.const 1 i32.eq call $assert
    global.get $delivered i32.const 1 i32.add global.set $delivered)
)
