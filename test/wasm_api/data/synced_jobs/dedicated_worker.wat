;; This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
;; A separate 64 KiB source module for the Rust guest's submit_to path.
(module
  (memory (export "memory") 1 1)
  (global $calls (mut i32) (i32.const 0))
  (func (export "SPRING_ENV_MASK") (result i32) i32.const 1)
  (func (export "SPRING_JOB_INPUT_BUFFER") (result i32) i32.const 1024)
  (func (export "SPRING_JOB_INPUT_CAPACITY") (result i32) i32.const 16)
  (func (export "work") (param $ptr i32) (param $len i32) (result i64)
    (local $value i32) (local $remaining i32)
    local.get $len i32.const 8 i32.ne if unreachable end
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
    global.get $calls i32.const 1 i32.add global.set $calls
    i32.const 32 local.get $value i32.store
    i32.const 36 global.get $calls i32.store
    i32.const 40 local.get $ptr i32.load i32.store
    i64.const 51539607584)
)
