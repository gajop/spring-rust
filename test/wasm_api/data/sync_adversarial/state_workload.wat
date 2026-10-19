;; This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
;; Fixed-size Core state and bulk-memory/table workload, no host clock/imports.
(module
  (type $number (func (result i32)))
  (memory (export "memory") 2 2)
  (global $counter (mut i64) (i64.const 0))
  (global $float (mut f64) (f64.const 0x1p-1022))
  (table 16 16 funcref)
  (func $a (type $number) (result i32) i32.const 11)
  (func $b (type $number) (result i32) i32.const 22)
  (elem $functions func $a $b)
  (data $payload "bulk-memory: deterministic\00\ff\80")
  (func (export "SPRING_ENV_MASK") (result i32) i32.const 1)
  (func (export "spring:callin/game-frame") (param $frame i32)
    ;; Passive data/element contents are immutable. Their drop status is NOT
    ;; exposed by Wasmtime's C API: test future behavior as well as snapshots.
    local.get $frame i32.const 1 i32.eq
    if
      i32.const 0 i32.const 0 i32.const 28 memory.init $payload
      i32.const 0 i32.const 0 i32.const 2 table.init $functions
    end
    local.get $frame i32.const 2 i32.eq
    if data.drop $payload elem.drop $functions end
    local.get $frame i32.const 3 i32.ge_u
    if
      i32.const 0 local.get $frame i32.const 1 i32.and call_indirect (type $number) i32.store offset=64
      i32.const 32 i32.const 0 i32.const 28 memory.copy
      i32.const 128 local.get $frame i32.const 64 memory.fill
      ;; Fixed-size state must reject growth without changing its size.
      i32.const 256 i32.const 1 memory.grow i32.store
      i32.const 260 ref.null func i32.const 1 table.grow 0 i32.store
    end
    global.get $counter i64.const 1664525 i64.mul local.get $frame i64.extend_i32_u i64.add global.set $counter
    ;; Hidden FP state exercises subnormals under the guarded entry mode.
    f64.const 0x1p-1022 f64.const 0.5 f64.mul global.set $float
  )
)
