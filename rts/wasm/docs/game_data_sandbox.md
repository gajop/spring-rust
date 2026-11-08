# Game data sandbox

`vfs.GameDataWrite`, `GameDataRead`, `GameDataDelete` and `GameDataList` give a module one directory
of its own for files it keeps between runs. The engine has no idea what they are for. Modules can
come from archives downloaded through a lobby or an autohost, so these calls treat the caller as
hostile.

## Where files live

`<base>/game-data/<id>/<path>`.

- `<base>` is the write directory, or `SPRING_GAME_DATA_BASE` when the launcher's environment sets it,
  for a launcher whose write directory is replaced by an update. A module cannot set either.
- `<id>` is the `gameDataId` field in the running archive's `modinfo.lua`: the game's in a match, the
  menu's in the menu. A game and its menu declare the same id to share a directory. The version and
  archive name have no part in it, so updating a game keeps its files. The rules for an id are those
  for a folder name below. With no valid id every call is refused: there is no shared fallback.
- Why not `gamedata/`: that is a path inside game archives, and raw-first lookups search the data
  directories for it.

## Rules, each with its test

Tests: `test/engine/System/FileSystem/TestGameDataFiles.cpp` (`GameData*` cases, table
`HOSTILE_FILE_PATHS`), `test/engine/WasmInterface/TestWasmModuleManifest.cpp` and the hostile guest
`test/wasm_api/data/game_data_fixture/run.py`, which drives a real engine.

| Rule | Proved by |
|---|---|
| A path is relative with `/` between components, parsed as text before any disk access; a bad one is refused whole, never repaired | `GameDataRefusesEveryHostilePathAndTouchesNothing`: nothing appears in the write directory |
| A folder: lowercase letters, digits, `_`, `-`; a letter or digit first and last. A file: the same stem, one `.`, an extension on the list | the same table |
| Extensions, data only: `json toml txt csv dat bin png jpg`; nothing a loader or a shell picks up (`lua so dll exe sdd sdz sd7 cfg` and the rest are refused because they are not listed) | the table's extension rows |
| No traversal, absolute paths, drive letters, `.`, `..`, empty components, backslashes, encoded separators (`%2f`, fullwidth and division slashes), leading dots, NUL, streams (`:`), wildcards, trailing dots or spaces | the table; NUL inside a path also through the guest, since paths are passed with a length |
| No Windows device names in any component, with or without an extension (`con`, `prn`, `aux`, `nul`, `com0-9`, `lpt0-9`) | the table |
| Lowercase only, so a case-insensitive disk sees the same rules and `CON` is not a second spelling | the table's case rows |
| A component is at most 64 bytes, a path at most 200, at most 8 components | `GameDataBoundsThePath` |
| A valid id or nothing | `GameDataNeedsAValidId`, guest archives `Game Data No Id` and `Bad Id` |
| No links: the root, the id directory, every folder and the file are opened one step at a time without following (`openat` with `O_NOFOLLOW`; on Windows a handle without delete sharing is held on each component, and a reparse point is refused), so there is no check-then-open to race | `GameDataRefusesLinks`, guest's planted link |
| A file where a folder is needed, or the reverse, is refused | `GameDataWrongTypes` |
| One game's tree: file size, files, folders and total bytes are bounded; replacing a file counts what it adds; refused writes create nothing. Links and foreign files count and are never followed | `GameDataQuotas` |
| Changes are rate limited (a burst, then a few a second), since each flushes the disk. A path refused by its text costs nothing | `GameDataChangeBudget` |
| Written to a dot-named temporary file in the same folder, flushed, renamed over the target (`MoveFileExW` with replace on Windows), so a reader or a crash sees the old file or the new one. A stale temporary file is removed by the next write | `GameDataRoundTrip` |
| Private permissions on POSIX (`0700`, `0600`). On Windows files take the ACL of the folder above | `GameDataRoundTrip` |
| No recursive delete; a list covers one folder, names only, bounded, links and foreign names left out | `GameDataListAndDelete` |
| Refusals return a code and a sentence, no absolute path; the log's line holds neither a path nor the caller's text, and stops after a while | guest's log check |
| No writes or deletes during replay playback; reads go on | `GameDataReplayIsReadOnly` (the host passes `hostDemo`) |
| Unsynced only: a synced module cannot load with these imports and the SDK does not offer them in a synced context | `Production Core ABI keeps game data out of synced environments` |
| Guest pointers and lengths are bounds checked against guest memory before use, by the same generated bindings as other calls: a status, no trap, no host read | guest's pointer cases |

## Codes

| Code | Meaning |
|---|---|
| `InvalidArgument` | The path's text breaks a rule, or a file is over the size limit |
| `InvalidId` | The archive declares no valid `gameDataId` |
| `PermissionDenied` | A link on the way, or a replay is playing |
| `NotFound` | No such file or folder |
| `InvalidState` | A file where a folder is needed or the reverse, or a folder that is not empty |
| `BufferOverflow` | Over a quota: files, folders or total bytes |
| `NotAvailable` | Too many changes in a short time |
| `OperationFailed` | The disk refused |

## Limits of the sandbox

- **An id is a claim, not an identity.** A game can declare another game's `gameDataId` and read or
  replace that game's files. The sandbox protects the player's other files, not one game from another
  that sets out to impersonate it. Only signed archives could close this.
- **Other calls are wider than this one.** It adds nothing to what a module already reaches through
  older calls, but it also does not take that away: `vfs.LoadFile` in raw mode, `vfs.CreateDir`,
  `gfx.SaveImage` and the archive helpers take raw paths in the data directories. Their production
  policy (`render_core_wasm_registry_policy.rs`) marks them withheld, yet the generated registry
  still links them.
- **Who may write.** A spectator and a client without local play may write, since it is the same code
  as for a player and the limits are what protect the machine. Only replay playback is refused,
  because watching must not change anything.
- **Native modules** run in the engine's process. They pass through the same checks, which keep an
  honest module inside the sandbox, but no check binds native code.
- **Windows.** The same rules and tests; the path rules, quotas, atomic replacement and errors run
  under Wine. Links do not: Wine shows a Unix symlink as a plain folder or file, with no reparse
  attribute, so `GameDataRefusesLinks` skips there and the reparse handling needs a run on real Windows
  (`test_GameDataFiles`, in a session allowed to make symlinks).
