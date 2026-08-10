# COVERAGE — sqlite-experiment

> **GENERATED from `APP_MANIFEST.yaml` — never hand-edit.**
> Counts only. No completion percentages. Inventory progress, not migration progress.

- Generated: 2026-08-10T17:51:11Z
- App status: `in_progress` · completeness: `incomplete`
- Manifest last_updated: 2026-08-10T17:51:11Z by `estate-discovery-loop`

## Counts

| Metric | Count |
| --- | --- |
| Surfaces total | 15 |
| Behaviours known | 15 |
| Seeds scanned | 4 |
| Unscanned hints (residual) | 56 |
| legacy_green flags | 0 |
| parity_green flags | 0 |

## Surfaces by status

| Status | Count |
| --- | --- |
| candidate | 15 |

## Behaviours by status

| Status | Count |
| --- | --- |
| candidate | 15 |

## Surfaces per slice

| Slice | Surfaces |
| --- | --- |
| backup-api | 3 |
| connection-lifecycle-api | 4 |
| exec-convenience-api | 2 |
| prepare-statement-api | 6 |

## Scanned seeds

- `backup-api`
- `connection-lifecycle-api`
- `exec-convenience-api`
- `prepare-statement-api`

## Unscanned hints (residual register)

- src: blob-io-api — incremental blob I/O sqlite3_blob_* (src/vdbeblob.c)
- src: serialize-memdb-api — serialize/deserialize + in-memory VFS (src/memdb.c)
- src: loadext-api — runtime loadable extensions (src/loadext.c)
- src: unlock-notify-api — sqlite3_unlock_notify (src/notify.c)
- src: auth-callback-api — sqlite3_set_authorizer (src/auth.c)
- src: attach-detach — ATTACH/DETACH DATABASE (src/attach.c)
- src: error-status-api — errcode/errmsg/status/limits (src/util.c, src/status.c, src/sqliteLimit.h)
- src: builtin-scalar-agg-funcs — built-in SQL function registry (src/func.c)
- src: date-time-funcs — date/time SQL functions (src/date.c)
- src: json-funcs — JSON SQL functions + json_tree/json_each (src/json.c)
- src: printf-format — SQL printf/format + internal formatter (src/printf.c)
- src: pragma-surface — PRAGMA dispatcher + pragma vtabs (src/pragma.c)
- src: window-functions — SQL window functions (src/window.c)
- src: upsert — ON CONFLICT DO UPDATE/NOTHING (src/upsert.c)
- src: triggers — CREATE TRIGGER + firing (src/trigger.c)
- src: foreign-keys — FK constraint enforcement (src/fkey.c)
- src: ddl-schema — CREATE/DROP/ALTER schema objects (src/build.c, src/alter.c)
- src: analyze-stats — ANALYZE + sqlite_stat tables (src/analyze.c)
- src: vacuum — VACUUM / VACUUM INTO (src/vacuum.c)
- src: tokenizer — SQL tokenizer + complete() (src/tokenize.c, src/complete.c)
- src: parser-grammar — Lemon grammar (src/parse.y)
- src: name-resolution — identifier resolution (src/resolve.c, src/walker.c)
- src: expr-codegen — expression code generation (src/expr.c)
- src: select-codegen — SELECT compilation (src/select.c)
- src: dml-codegen — INSERT/UPDATE/DELETE compilation (src/insert.c, src/update.c, src/delete.c)
- src: where-optimizer — query planner (src/where.c, src/wherecode.c, src/whereexpr.c)
- src: vdbe-engine — bytecode VM (src/vdbe.c, src/vdbeaux.c, src/vdbemem.c, src/vdbesort.c)
- src: btree — B-tree layer (src/btree.c, src/btmutex.c)
- src: pager — page cache + transactions (src/pager.c)
- src: wal — write-ahead log (src/wal.c)
- src: pcache — pluggable page cache (src/pcache.c, src/pcache1.c)
- src: vfs-os-abstraction — VFS layer (src/os.c, src/os_unix.c, src/os_win.c, src/os_kv.c, src/memjournal.c)
- src: malloc-subsystem — allocators + memory status (src/malloc.c, src/mem0.c..mem5.c)
- src: mutex-subsystem — mutex implementations (src/mutex*.c)
- src: util-primitives — utf/random/hash/bitvec/rowset (src/utf.c, src/random.c, src/hash.c, src/bitvec.c, src/rowset.c)
- src: vtab-core — virtual table mechanism (src/vtab.c)
- src: introspection-vtabs — dbstat/dbpage/bytecode vtabs (src/dbstat.c, src/dbpage.c, src/vdbevtab.c)
- src: shell-cli — sqlite3 command-line shell (src/shell.c.in)
- src: tcl-binding — TCL language binding (src/tclsqlite.c)
- ext: fts5 — full-text search 5 (ext/fts5/)
- ext: fts3 — full-text search 3/4 legacy (ext/fts3/)
- ext: rtree — R-tree spatial index (ext/rtree/rtree.c)
- ext: geopoly — GeoJSON polygon vtab (ext/rtree/geopoly.c)
- ext: session — changesets/patchsets (ext/session/)
- ext: rbu — resumable bulk update (ext/rbu/)
- ext: recover — corrupt-db recovery API (ext/recover/)
- ext: intck — incremental integrity check (ext/intck/)
- ext: expert — index recommendation (ext/expert/)
- ext: icu — ICU collation/LIKE/upper-lower (ext/icu/)
- ext: qrf — query result formatter (ext/qrf/)
- ext: jni-binding — Java/JNI binding (ext/jni/)
- ext: wasm-binding — WASM/JS binding + fiddle (ext/wasm/)
- ext: misc-vfs-shims — VFS wrapper extensions (ext/misc: appendvfs, cksumvfs, vfsstat, vfstrace, vfslog, tmstmpvfs, mmapwarm, memtrace, pcachetrace)
- ext: misc-func-packs — SQL function extensions (ext/misc: base64, base85, basexx, decimal, ieee754, regexp, sha1, shathree, spellfix, totype, uint, uuid, fossildelta, compress, percentile, nextchar, rot13, urifuncs)
- ext: misc-vtab-packs — virtual-table extensions (ext/misc: series, csv, unionvtab, qpvtab, completion, closure, amatch, fuzzer, prefixes, wholenumber, zipfile, sqlar, stmt, templatevtab, vtablog, vtshim, btreeinfo, zorder)
- ext: misc-utilities — remaining misc utilities (ext/misc: fileio, dbdump, eval, explain, memstat, diskused, noop, normalize, randomjson, remember, stmtrand, strdup, showauth, anycollseq)

## Notes

Phase A radar only; candidates unbound. estate_scan=partial until METHOD_COVERAGE checklist addressed. src/test*.c harness adapters skipped-with-reason (see overnight/STRUCTURAL_INDEX.md).
