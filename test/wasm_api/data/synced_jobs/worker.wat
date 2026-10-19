;; This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
(module
  ;; Permitted in a source rules module; workers must trap instead of calling it.
  (import "spring:desync" "get-timer" (func $timer (result i64)))
  (memory (export "memory") 2 2)
  (global $calls (mut i32) (i32.const 0))
  (func (export "SPRING_ENV_MASK") (result i32) i32.const 1)
  (func (export "SPRING_JOB_INPUT_BUFFER") (result i32) i32.const 1024)
  (func (export "SPRING_JOB_INPUT_CAPACITY") (result i32) i32.const 65536)
  ;; Input is LE u32 seed, iterations, followed by copied payload. Output is
  ;; LE u32 LCG state, private call count, payload's final byte.
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
    i32.const 40 local.get $ptr local.get $len i32.add i32.const 1 i32.sub i32.load8_u i32.store
    i64.const 51539607584) ;; 12-byte output at offset 32
  (func $pure (export "pure-work") (param $ptr i32) (param $len i32) (result i64)
    i32.const 0 global.set $calls
    local.get $ptr local.get $len call $work)
  (func (export "inline-batch") (param $ptr i32) (param $len i32) (result i64)
    (local $n i32) (local $result i64)
    local.get $ptr i32.load offset=8 local.set $n
    block $done
      loop $next
        local.get $n i32.eqz br_if $done
        local.get $ptr local.get $len call $pure local.set $result
        local.get $n i32.const 1 i32.sub local.set $n
        br $next
      end
    end
    local.get $result)
  (func (export "echo") (param $ptr i32) (param $len i32) (result i64)
    local.get $len i64.extend_i32_u i64.const 32 i64.shl
    local.get $ptr i64.extend_i32_u i64.or)
  (func (export "float-work") (param $ptr i32) (param i32) (result i64)
    i32.const 32 local.get $ptr f32.load local.get $ptr f32.load f32.div f32.store
    i32.const 36 local.get $ptr f32.load offset=4 f32.const 1.25 f32.mul f32.const 0.1 f32.add f32.store
    i64.const 34359738400)
  (func (export "trap") (param i32 i32) (result i64) unreachable)
  (func (export "host-call") (param i32 i32) (result i64) call $timer)
  (func (export "loop") (param i32 i32) (result i64) (loop $forever br $forever) i64.const 0)
  (func (export "bad-output") (param i32 i32) (result i64) i64.const -1)
  (func (export "grow") (param i32 i32) (result i64)
    i32.const 32 i32.const 1 memory.grow i32.store i64.const 17179869216)
  ;; Inputs are raw IEEE words: ties and subnormals expose inherited host FP state.
  (func (export "adversarial-float") (param $p i32) (param i32) (result i64)
    i32.const 32 local.get $p f32.load local.get $p f32.load offset=4 f32.add f32.store
    i32.const 36 local.get $p f32.load offset=8 local.get $p f32.load offset=12 f32.mul f32.store
    i32.const 40 local.get $p f64.load offset=16 local.get $p f64.load offset=24 f64.add f64.store
    i32.const 48 local.get $p f64.load offset=32 local.get $p f64.load offset=40 f64.mul f64.store
    i64.const 103079215136) ;; 24 bytes at offset 32
  (func (export "adversarial-simd") (param $p i32) (param i32) (result i64)
    i32.const 64 local.get $p v128.load f32.const 0.5 f32x4.splat f32x4.mul v128.store
    ;; f64->f32 demotion and i32->f32 conversion must also round nearest-even.
    i32.const 80 local.get $p f64.load offset=16 f64.const 0x1p-24 f64.add f32.demote_f64 f32.store
    i32.const 84 i32.const 16777217 f32.convert_i32_s f32.store
    i64.const 103079215168) ;; 24 bytes at offset 64
)
