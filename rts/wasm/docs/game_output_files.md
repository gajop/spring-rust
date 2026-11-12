# Game output files

`vfs.GameOutputOpen`, `GameOutputWrite` and `GameOutputClose` let an unsynced module append to files
it never reads back: recordings, exports, logs for a tool. They sit beside the
[game data sandbox](game_data_sandbox.md) and share its path rules and its threat model (the caller
is hostile), with three differences: the files are append-only streams, they may be written during
replay playback, and their bounds are the player's settings rather than fixed constants.

| | Game data | Game output |
|---|---|---|
| Directory | `<base>/game-data/<id>/` | `<base>/game-output/<id>/` |
| Calls | write whole file, read, delete, list | open, append, close; no read, list or delete |
| Extensions | `json toml txt csv dat bin png jpg` | `jsonl json txt csv tsv log dat bin` |
| Replay playback | read only | written |
| Bounds | constants in `GameDataFiles.h` | the player's settings, below |

`<base>` and `<id>` are the game data ones: the write directory or `SPRING_GAME_DATA_BASE`, and the
running archive's `gameDataId`. No valid id, no output.

## Calls

- `GameOutputOpen(path, append) -> handle`. Makes the folders on the way. `append = false` asks for a
  new file and refuses a taken name (`AlreadyExists`), so two runs never write into one file;
  `append = true` continues a file or makes it. A handle is never 0 and is never reused, so a
  stale one is `NotFound`, not someone else's stream.
- `GameOutputWrite(handle, bytes) -> bool`. Appends all of the bytes, or nothing when a bound refuses
  them. Each call goes to the operating system at once, so what was written survives the module
  or the engine crashing; a module that writes often should collect rows and write them together.
- `GameOutputClose(handle) -> bool`. Streams still open when the game ends are closed by the engine
  (`CGame::KillLua`, after the modules' `Shutdown`), with one log line counting them.

## Bounds

Settings (`springsettings.cfg`), read at every call, so a change applies to the next one:

| Setting | Bounds |
|---|---|
| `GameOutputMaxFileMB` | one file |
| `GameOutputMaxTotalMB` | the whole `game-output/<id>/` tree, earlier runs included: a player deletes old files to make room |
| `GameOutputMaxFiles` | files in that tree |
| `GameOutputMBPerSecond` | bytes written a second, after a burst of four seconds' worth |

Fixed in `GameDataFiles.h`: 16 streams open at once, opens limited to a burst of 16 then 2 a second,
at most 64 folders, and the game data path lengths and depth.

Why the player's settings and not the game's (`modrules`): a game may come from a download, and a
bound the game sets is no bound. A game that needs more says so and the player raises it; a tool
passes the settings on its command line.

The tree is measured when a game opens its first stream, walking it without following links, as
the game data quotas are. While any of its streams is open the engine counts the bytes it writes
instead of reading sizes back, which a still-open file on Windows may not report.

## Rules, each with its test

Tests: `test/engine/System/FileSystem/TestGameDataFiles.cpp` (`GameOutput*` cases) and
`test/engine/WasmInterface/TestWasmModuleManifest.cpp`.

| Rule | Proved by |
|---|---|
| Every path the game data rules refuse is refused before the disk is touched; nothing is created | `GameOutputPaths` (the same `HOSTILE_FILE_PATHS` table) |
| Only output extensions: no image, config, script or module a loader picks up | `GameOutputPaths` |
| A valid id or nothing | `GameOutputNeedsAValidId` |
| Appends land at once and in order; a new file refuses a taken name; reopening continues; a closed or unknown handle is refused; private permissions on POSIX; separate from game data | `GameOutputAppends` |
| No links: the root, the id directory, every folder and the file are opened without following (`openat` with `O_NOFOLLOW`; on Windows the reparse point itself is opened and refused). A link at a new file's name is a taken name | `GameOutputRefusesLinks` |
| A folder or a FIFO at the name is refused, without blocking | `GameOutputWrongTypes` |
| File size, tree total (earlier files counted), files, folders, open streams, opens and bytes a second; a refused write puts nothing down and a refused open creates nothing | `GameOutputLimits`, `GameOutputRateBudget` |
| Unsynced only: a synced module cannot load with these imports, so nothing written can be read by the simulation, and no call reads output back | `Production Core ABI keeps game data and output out of synced environments` |
| Refusals return a code and words without paths; the log line holds neither a path nor the caller's text and stops after a while | `GameOutputError` in `rts/NativeInterface/api/VFS.cpp` |

## Codes

| Code | Meaning |
|---|---|
| `InvalidArgument` | The path's text breaks a rule |
| `InvalidId` | The archive declares no valid `gameDataId` |
| `AlreadyExists` | A new file was asked for and the name is taken |
| `NotFound` | No open stream has this handle |
| `PermissionDenied` | A link on the way |
| `InvalidState` | Not a regular file at the name, or a file where a folder is needed |
| `BufferOverflow` | Over a bound: file size, tree total, files, folders or open streams |
| `NotAvailable` | Too many opens or bytes in a short time |
| `OperationFailed` | The disk refused |

## Limits

- **Handles are the process's, not a module's.** Two modules of one game can write to each other's
  streams if they share a handle. Streams a module leaves open when it is reloaded stay open until
  the game ends and count against the 16.
- **Replays write.** Watching a replay changes no game state, and exporting a replay is a main use.
  A replay's game declares the same id, so its output lands where the game's does.
- The game data sandbox's limits on ids and native modules apply here too.
