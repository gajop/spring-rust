;; This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
;; Run in a one-player recorded scenario. Playback adds a local observer;
;; neither that observer nor its ID may enter the synced player API.
(module
  (import "spring:teams" "get-player-list" (func $players (param i32 i32 i32) (result i32)))
  (import "spring:teams" "get-player-info" (func $info (param i32 i32 i32) (result i32)))
  (memory (export "memory") 1 1)
  (func (export "SPRING_ENV_MASK") (result i32) i32.const 1)
  (func (export "spring:callin/game-frame") (param $frame i32)
    local.get $frame i32.eqz
    if
      ;; i32 array output descriptor at 0; payload at 64, capacity 8 players.
      i32.const 0 i32.const 64 i32.store
      i32.const 4 i32.const 8 i32.store
      i32.const -1 i32.const 0 i32.const 0 call $players
      if unreachable end
      i32.const 8 i32.load i32.const 1 i32.ne
      if unreachable end
      i32.const 64 i32.load
      if unreachable end
      ;; Dynamic output descriptor. Player 1 is a local playback spectator.
      i32.const 16 i32.const 128 i32.store
      i32.const 20 i32.const 512 i32.store
      i32.const 1 i32.const 0 i32.const 16 call $info i32.const 1 i32.ne
      if unreachable end
    end))
