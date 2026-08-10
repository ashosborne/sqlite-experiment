# Structural index (bootstrap, iteration 0) — sqlite-experiment

Cheap read-only entrypoint sweep of the allowlist (`src/`, `ext/`). Every row: locator + evidence path.
Confidence: `observed-in-code` unless marked otherwise. This estate has **no** HTTP listeners, message
queues, or schedulers — the closest analogues are recorded honestly below.

## 1. Callable C API boundary (contract authority: `src/sqlite.h.in`)

| API family | Locator | Evidence |
| --- | --- | --- |
| Connection lifecycle | `sqlite3_open()` / `sqlite3_open_v2()` / `sqlite3_close()` | `src/sqlite.h.in:4019`, `src/sqlite.h.in:4027`, `src/sqlite.h.in:356` |
| Library init/config | `sqlite3_initialize()` / `sqlite3_config()` / `sqlite3_db_config()` | `src/sqlite.h.in:1689`, `src/sqlite.h.in:1729`, `src/sqlite.h.in:1748` |
| Prepared statements | `sqlite3_prepare_v2()` / `sqlite3_step()` / `sqlite3_finalize()` | `src/sqlite.h.in:4637`, `src/sqlite.h.in:5347`, `src/sqlite.h.in:5652` |
| One-shot exec | `sqlite3_exec()` | `src/sqlite.h.in:430` |
| UDF registration | `sqlite3_create_function()` | `src/sqlite.h.in:5817` |
| Runtime extension load | `sqlite3_load_extension()` | `src/sqlite.h.in:7609` |
| Virtual table registration | `sqlite3_create_module()` | `src/sqlite.h.in:8002` |
| Incremental blob I/O | `sqlite3_blob_open()` | `src/sqlite.h.in:8206` |
| VFS registration | `sqlite3_vfs_register()` | `src/sqlite.h.in:8381` |
| Online backup | `sqlite3_backup_init()` | `src/sqlite.h.in:9822` |
| Serialize/deserialize | `sqlite3_serialize()` / `sqlite3_deserialize()` | `src/sqlite.h.in:11248`, `src/sqlite.h.in:11326` |

## 2. SQL-language surfaces (registration tables)

| Surface | Locator | Evidence |
| --- | --- | --- |
| Built-in scalar/aggregate functions | `sqlite3RegisterBuiltinFunctions` table | `src/func.c` (~111 `FUNCTION(...)`-macro rows) |
| Date/time functions | date func table | `src/date.c` (6 `FUNCTION(...)` rows) |
| JSON functions + vtabs | json func table | `src/json.c` (33 `JFUNCTION(...)` rows) |
| Window functions | window func table | `src/window.c` (29 registration rows) |
| PRAGMA dispatcher | `sqlite3Pragma()` | `src/pragma.c:425` (pragma table generated from `tool/mkpragmatab.tcl` — out-of-scope path; cite `src/pragma.c` only) |
| PRAGMA-as-vtab | `sqlite3PragmaVtabRegister()` | `src/pragma.c:3092` |

## 3. Batch / CLI mains (closest analogue to batch entrypoints)

| CLI | Locator | Evidence |
| --- | --- | --- |
| sqlite3 shell | `src/shell.c.in` (main; built via tool/mkshellc.tcl) | `src/shell.c.in` |
| changeset tool | `main()` | `ext/session/changeset.c:186` |
| changesetfuzz tool | `main()` | `ext/session/changesetfuzz.c:1193` |
| sqlite3_expert CLI | `main()` | `ext/expert/expert.c:70` |
| rbu demo CLI | `main()` | `ext/rbu/rbu.c:76` |
| dbdump standalone | `main()` (behind `DBDUMP_STANDALONE`) | `ext/misc/dbdump.c:695` |
| wasm build helpers | `main()` | `ext/wasm/mkwasmbuilds.c:1086`, `ext/wasm/libcmpp.c:9432` (build tooling — likely defer) |

## 4. Language-binding adapters

| Binding | Locator | Evidence |
| --- | --- | --- |
| TCL | `Sqlite3_Init()` | `src/tclsqlite.c:4469` |
| Java/JNI | `ext/jni/` (4 top-level entries; src subtree) | `ext/jni/` |
| WASM/JS | `ext/wasm/` (40 entries) | `ext/wasm/` |

## 5. Loadable-extension entrypoints (`sqlite3_*_init`) — sweep of `ext/` (excl. wasm)

64 init symbols found, including:
`ext/fts3/fts3.c:6211`, `ext/fts5/fts5_main.c:3891`, `ext/icu/icu.c:582`, `ext/rtree/rtree.c:4485`,
`ext/rtree/geopoly.c:1795`, `ext/recover/dbdata.c:1014`, and ~55 under `ext/misc/*` (amatch, appendvfs,
base64/base85/basexx, btreeinfo, cksumvfs, closure, completion, compress, csv, decimal, diskused, eval,
explain, fileio, fossildelta, fuzzer, ieee754, memstat, nextchar, noop, percentile, prefixes, qpvtab,
randomjson, regexp, remember, rot13, series, sha1, shathree, showauth, spellfix, sqlar, stmt, stmtrand,
strdup, templatevtab, tmstmpvfs, totype, uint, unionvtab, urifuncs, uuid, vfsstat, vtablog, vtshim,
wholenumber, zipfile, zorder).

## 6. Explicitly skipped-with-reason (inside allowlist)

| Path | Reason |
| --- | --- |
| `src/test1.c`–`src/test9.c`, `src/test_*.c` (~44 files) | TCL test-harness adapters for the upstream test suite; charter marks `test/` out of scope — these are its in-src counterparts, not product surfaces |
| `ext/fts3/unicode/`, `ext/fts5/tool/`, `ext/session/*_fuzz*` where tooling-only | Generator/fuzz tooling; revisit only if a runtime entrypoint is claimed |
| Generated outputs (`parse.c`, `opcodes.*`, `keywordhash.h`, `pragma.h`, `sqlite3.c`) | Not present in tree / build artefacts; evidence must cite source inputs (`src/parse.y` etc.) |

## 7. UI entry

None (no UI layer). `ext/wasm` fiddle app noted as an adapter candidate — deprioritized per loop rules.
